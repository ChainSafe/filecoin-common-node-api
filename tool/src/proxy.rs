mod services;

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Display,
    fs::File,
    future::IntoFuture as _,
    hash::{BuildHasher, RandomState},
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
};

use crate::check::{Annotation, CheckAllMethods};
use crate::jsonrpc_types;
use anyhow::Context as _;
use clap::Parser;
use fluent_uri::UriRef;
use futures::{future::join, FutureExt as _, Stream, StreamExt as _};
use http::StatusCode;
use hyper::body::Bytes;
use jsonschema::CompilationOptions;
use serde::Deserialize;
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
pub struct Args {
    local: SocketAddr,
    remote_host: http::HeaderValue,
    remote_uri: UriRef<String>,
    spec: PathBuf,
    #[arg(long)]
    log: Option<PathBuf>,
    #[arg(long, default_value_t = 1024)]
    queue_depth: usize,
}

#[derive(Deserialize)]
struct Log {
    #[serde(flatten)]
    subscriber: tracing_configuration::Subscriber,
    #[serde(deserialize_with = "from_str")]
    filter: EnvFilter,
}

#[derive(Debug, Default)]
pub struct Report<S> {
    /// Methods which matched the specification.
    pub pass: HashMap<String, usize, S>,
    /// Methods which failed (and why).
    pub fail: Vec<(
        jsonrpc_types::Request<'static>,
        Option<jsonrpc_types::Response<'static>>,
        nunny::Vec<Annotation>,
    )>,
    /// Methods which were not covered in the specification
    pub skip: usize,
}

/// Construct a [`Report`] once the stream is finished.
async fn report<'a, S>(
    checker: &CheckAllMethods<impl BuildHasher>,
    s: impl Stream<
        Item = (
            jsonrpc_types::Request<'a>,
            Option<jsonrpc_types::Response<'a>>,
        ),
    >,
) -> Report<S>
where
    S: BuildHasher + Default,
{
    s.fold(Report::default(), |mut report, (req, resp)| async move {
        match checker.get(&req.method) {
            Some(check) => match nunny::Vec::new(check.check(&req, resp.as_ref())) {
                Ok(annot) => {
                    report
                        .fail
                        .push((req.into_owned(), resp.map(|it| it.into_owned()), annot))
                }
                Err(_) => {
                    report
                        .pass
                        .entry(req.method.into_owned())
                        .and_modify(|it| *it += 1)
                        .or_insert(1);
                }
            },
            None => report.skip += 1,
        }
        report
    })
    .await
}

pub async fn main(
    Args {
        local,
        remote_host,
        remote_uri,
        spec,
        log,
        queue_depth,
    }: Args,
) -> anyhow::Result<()> {
    let check = CheckAllMethods::new_with_hasher_and_compilation_options(
        serde_json::from_reader(File::open(spec).context("couldn't open file")?)
            .context("invalid spec file")?,
        RandomState::new(),
        &CompilationOptions::default(),
    )
    .context("invalid spec file")?;

    if let Some(log) = log {
        let Log { subscriber, filter } =
            serde_json::from_reader(File::open(log).context("couldn't open logging config file")?)
                .context("invalid logging config file")?;
        let (builder, _guard) = subscriber
            .try_builder()
            .context("couldn't set up logging")?;
        builder.with_env_filter(filter).init();
    }

    let (remote_host, remote_uri) = leak((remote_host, remote_uri));

    let (sink, stream) = futures::channel::mpsc::channel(queue_depth);

    info!(target: "app::serve", addr = %local, "listening");

    use services::*;
    let report = report::<RandomState>(
        &check,
        stream.filter_map(
            |(req, resp): (http::Request<Bytes>, http::Response<Bytes>)| async move {
                match (
                    serde_json::from_slice(req.body()),
                    match resp.body().is_empty() {
                        true => Ok(None),
                        false => serde_json::from_slice(resp.body()).map(Some),
                    },
                ) {
                    (Ok(req), Ok(resp)) => Some((req, resp)),
                    _ => {
                        warn!(target: "app", "ignoring HTTP exchange that isn't JSON-RPC");
                        None
                    }
                }
            },
        ),
    );
    let proxy = axum::serve(
        TcpListener::bind(local).await?,
        axum::Router::new().fallback_service(
            tower::ServiceBuilder::new()
                .layer_fn(|svc| {
                    UnwrapOrElse::new(svc, |e: anyhow::Error| -> axum::response::Response {
                        http::Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(format!("{:?}", e).into())
                            .unwrap()
                    })
                })
                .layer_fn(CollectRequest)
                .map_response(|resp: http::Response<Bytes>| -> axum::response::Response {
                    resp.map(Into::into)
                })
                .map_response(|(_req, resp)| resp)
                .layer_fn(move |svc| TeeOk::new(svc, sink.clone()))
                .layer_fn(Correlate)
                .layer_fn(CollectResponse)
                .map_response(|resp: reqwest::Response| -> http::Response<reqwest::Body> {
                    resp.into()
                })
                .layer_fn({
                    |svc| {
                        TryMapRequest::new(
                        svc,
                        |mut req: http::Request<Bytes>| -> Result<reqwest::Request, anyhow::Error> {
                            req.headers_mut()
                                .insert(http::header::HOST, remote_host.clone());
                            let new_uri = fluent_uri::UriRef::parse(req.uri().to_string())
                                .context("couldn't parse request URI")?
                                .resolve_against(remote_uri)
                                .context("couldn't rewrite URI")?;
                            let new_uri = new_uri
                                .as_str()
                                .parse()
                                .with_context(|| format!("rewritten URI {} is invalid", new_uri))?;

                            debug!(
                                target: "app::uri",
                                old = %req.uri(),
                                new = %new_uri,
                                "rewrote URI"
                            );

                            *req.uri_mut() = new_uri;
                            Ok(req.try_into()?)
                        },
                    )
                    }
                })
                .layer_fn(|svc| MapErr::new(svc, anyhow::Error::new))
                .service(reqwest::Client::new()),
        ),
    )
    .with_graceful_shutdown(tokio::signal::ctrl_c().map(|res| match res {
        Ok(()) => info!(target: "app::shutdown", "shutdown requested"),
        Err(error) => error!(target: "app::shutdown", %error, "error registering shutdown signal"),
    }));

    let (proxy, report) = join(proxy.into_future(), report).await;

    if let Err(error) = proxy {
        error!(target: "app::serve", %error, "couldn't run proxy server")
    }

    dbg!(report);

    Ok(())
}

fn from_str<'de, D: serde::Deserializer<'de>, T: FromStr<Err = E>, E: Display>(
    d: D,
) -> Result<T, D::Error> {
    Cow::<str>::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn leak<T>(it: T) -> &'static T {
    Box::leak(Box::new(it))
}

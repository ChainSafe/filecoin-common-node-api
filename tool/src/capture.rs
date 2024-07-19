use crate::{Dialogue, DialogueResponse};
use anyhow::Context as _;
use axum::response::IntoResponse as _;
use bstr::ByteSlice as _;
use bytes::Bytes;
use ez_jsonrpc_types as jsonrpc;
use fluent_uri::UriRef;
use futures::FutureExt as _;
use http_body_util::BodyExt as _;
use itertools::Itertools as _;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower::ServiceExt as _;
use tracing::{debug, error, info, warn};

struct State {
    remote: UriRef<String>,
    client: reqwest::Client,
}

pub async fn capture(local: SocketAddr, remote: UriRef<String>) -> anyhow::Result<()> {
    axum::serve(
        TcpListener::bind(local).await?,
        axum::Router::new()
            .fallback(handler)
            .with_state(Arc::new(State {
                remote,
                client: reqwest::Client::new(),
            })),
    )
    .with_graceful_shutdown(tokio::signal::ctrl_c().map(|res| match res {
        Ok(()) => info!(target: "app::shutdown", "shutdown requested"),
        Err(error) => error!(target: "app::shutdown", %error, "error registering shutdown signal"),
    }))
    .await
    .context("couldn't run server")
}

async fn handler(
    state: axum::extract::State<Arc<State>>,
    request: http::Request<axum::body::Body>,
) -> axum::response::Response {
    match proxy(&state, request).await {
        Ok((req, resp)) => {
            match (
                serde_json::from_slice::<jsonrpc::Request>(req.body()),
                match resp.body().trim().is_empty() {
                    true => Ok(None),
                    false => serde_json::from_slice::<jsonrpc::Response>(resp.body()).map(Some),
                },
            ) {
                (Ok(req), Ok(resp)) => {
                    let dialogue = Dialogue {
                        method: req.method,
                        params: req.params,
                        response: resp.map(|it| match it.result {
                            Ok(it) => DialogueResponse::Result(it),
                            Err(it) => DialogueResponse::Error(it),
                        }),
                    };
                    match serde_json::to_string(&dialogue) {
                        Ok(s) => println!("{}", s),
                        Err(error) => {
                            error!(target: "app::capture", %error, "couldn't (re)serialize dialogue")
                        }
                    }
                }
                (Err(error), _) | (_, Err(error)) => {
                    warn!(target: "app::capture", %error, "ignoring HTTP exchange that wasn't JSON-RPC")
                }
            }
            resp.map(axum::body::Body::from)
        }
        Err(e) => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            e.chain()
                .map(|e| e.to_string())
                .chain([String::new()])
                .join("\n"),
        )
            .into_response(),
    }
}

async fn proxy<'a>(
    state: &State,
    request: http::Request<axum::body::Body>,
) -> anyhow::Result<(http::Request<Bytes>, http::Response<Bytes>)> {
    let (mut parts, body) = request.into_parts();
    let body = body
        .collect()
        .await
        .context("couldn't gather request body")?
        .to_bytes();
    let new_uri = fluent_uri::UriRef::parse(parts.uri.to_string())
        .context("couldn't parse request URI")?
        .resolve_against(&state.remote)
        .context("couldn't rewrite URI")?;
    let new_uri = new_uri
        .as_str()
        .parse()
        .with_context(|| format!("rewritten URI {} is invalid", new_uri))?;

    debug!(target: "app::proxy", old = %parts.uri, new = %new_uri, "rewrote URI");

    parts.uri = new_uri;

    let request = http::Request::from_parts(parts, body);

    let (parts, body) = http::Response::from(
        (&state.client)
            .oneshot(
                request
                    .clone()
                    .try_into()
                    .context("couldn't reassemble request")?,
            )
            .await
            .context("couldn't proxy request")?,
    )
    .into_parts();
    let body = body
        .collect()
        .await
        .context("couldn't gather response body")?
        .to_bytes();
    let response = http::Response::from_parts(parts, body);

    Ok((request, response))
}

mod capture;
mod check;
mod gc;
mod generate;

use anyhow::{bail, Context as _};
use ascii::AsciiChar;
use check::CheckMethod;
use clap::Parser;
use ez_jsonrpc::types::{self as jsonrpc, Id, RequestParameters};
use fluent_uri::UriRef;
use futures::TryFutureExt as _;
use itertools::Itertools as _;
use openrpc_types::{
    resolve_within,
    resolved::{ExamplePairing, Method},
    ParamStructure,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt,
    fs::File,
    hash::BuildHasher,
    io::{self, IsTerminal as _},
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Utilities for creating, interacting with, and testing against the Filecoin
/// Common Node API.
#[derive(Parser)]
enum Args {
    #[command(subcommand)]
    Openrpc(Openrpc),
    /// Interpret stdin as a `delimiter`-separated series of lines, with a header,
    /// and print JSON.
    Csv2json {
        #[arg(short, long, default_value_t = Char(AsciiChar::Tab))]
        delimiter: Char,
    },
    #[command(subcommand)]
    JsonRpc(JsonRpc),
}

/// Subcommands related to processing OpenRPC documents.
#[derive(Parser)]
enum Openrpc {
    /// Performs validation of the spec, including FIP-specific validation.
    ///
    /// Errors are emitted to stderr.
    ///
    /// If stdin is received (and is not a terminal),
    /// it will be interpreted as concatenated JSON summaries of JSON-RPC
    /// dialogues (as output by the `json-rpc capture` command).
    ///
    /// Each dialogue will be validated against the spec.
    ///
    /// On EOF, a summary table of passing `count` and `method` dialogues is
    /// printed to stdout.
    ///
    /// If there is only a single dialogue, and it fails to validate,
    /// more detailed errors will be emitted.
    Validate { spec: PathBuf },
    /// Interpret `select` as a json document of methods to include in `openrpc`.
    ///
    /// A new schema with only the selected methods is printed to stdout.
    Select {
        openrpc: PathBuf,
        select: PathBuf,
        /// Specify a new title for the schema.
        #[arg(long)]
        overwrite_title: Option<String>,
        /// Specify a new version for the schema.
        #[arg(long)]
        overwrite_version: Option<String>,
    },
    /// Read an OpenRPC specification from stdin,
    /// and print Rust code for a client trait.
    Generate { trait_name: String },
}

/// Subcommands for interacting with JSON-RPC endpoints.
#[derive(Parser)]
enum JsonRpc {
    /// Start a HTTP server, forwarding all requests to a single URI.
    ///
    /// Ctrl+C will request a graceful shutdown.
    ///
    /// For HTTP dialogue whose bodies can be parsed as a single JSON-RPC v2
    /// method call, print a summary as a JSON line to stdout.
    ///
    /// The summary includes only the method name, params, and response.
    ///
    /// This does NOT validate adherence to the JSON-RPC protocol.
    ///
    /// This is NOT robust to malice,
    /// and should only be run in trusted environments.
    Capture {
        /// The local socket address to bind to.
        #[arg(long)]
        local: SocketAddr,
        /// The remote URI to forward requests to.
        #[arg(long)]
        remote: UriRef<String>,
    },
    /// Receive's stdin's concatenated JSON summaries of JSON-RPC
    /// dialogue (as output by the `json-rpc capture` command).
    ///
    /// Each request in the exchange is send via HTTP POST to `remote`,
    /// and the dialogue is printed to stdout.
    ///
    /// All requests are sent with an `id` (i.e not as a JSON-RPC Notification).
    ///
    /// This does NOT validate adherence to the JSON-RPC protocol.
    Play {
        /// Additional headers to append to every request.
        ///
        /// By default, `Content-Type` and `User-Agent` headers are set.
        #[arg(long)]
        header: Vec<Header>,
        /// The host to send JSON-RPC requests to.
        #[arg(long)]
        remote: String,
        /// Don't short-circuit on the first HTTP/serialization failure.
        #[arg(long)]
        keep_going: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<RequestParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<DialogueResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DialogueResponse {
    Result(Value),
    Error(ez_jsonrpc::types::Error),
}

fn main() -> anyhow::Result<()> {
    match Args::parse() {
        Args::Openrpc(Openrpc::Validate { spec }) => {
            let mut errs = ErrorEmitter::new(io::stderr());
            let document = resolve_within(load_json(spec)?)?;
            let compilation_options = jsonschema::CompilationOptions::default();

            let method2checker = validate_document(&mut errs, &document, compilation_options);

            if !io::stdin().is_terminal() {
                let passed =
                    validate_dialogues_from_reader(&mut errs, method2checker, io::stdin())?;
                if !passed.is_empty() {
                    for (method, count) in passed {
                        println!("{}\t{}", count, method)
                    }
                }
            }
            errs.finish()
        }
        Args::Openrpc(Openrpc::Select {
            openrpc,
            select,
            overwrite_title,
            overwrite_version,
        }) => {
            let mut openrpc = resolve_within(load_json(openrpc)?)?;
            let select = load_json::<Vec<Select>>(select)?
                .into_iter()
                .filter(|it| matches!(it.include, Some(InclusionDirective::Include)))
                // formatting the name like this is a hack
                .map(|it| (format!("Filecoin.{}", it.method), it.description))
                .collect::<BTreeMap<_, _>>();
            openrpc.methods.retain_mut(|it| match select.get(&it.name) {
                Some(new_description) => {
                    if new_description.is_some() && it.description.is_none() {
                        it.description.clone_from(new_description)
                    }
                    true
                }
                None => false,
            });
            gc::prune_schemas(&mut openrpc)?;
            if let Ok(missed) = nunny::Vec::new(
                select
                    .keys()
                    .collect::<BTreeSet<_>>()
                    .difference(&openrpc.methods.iter().map(|it| &it.name).collect())
                    .collect(),
            ) {
                eprintln!(
                    "the following selected methods were not present: {}",
                    missed.iter().join(", ")
                )
            }
            if let Some(title) = overwrite_title {
                openrpc.info.title = title
            }
            if let Some(version) = overwrite_version {
                openrpc.info.version = version
            }
            serde_json::to_writer_pretty(io::stdout(), &openrpc)?;
            Ok(())
        }
        Args::Openrpc(Openrpc::Generate { trait_name }) => {
            let tokens = generate::generate(
                resolve_within(serde_json::from_reader(io::stdin())?)?,
                syn::parse_str(&trait_name)?,
            )?;
            println!("{}", tokens);
            Ok(())
        }
        Args::Csv2json {
            delimiter: Char(delimiter),
        } => {
            let mut records = csv::ReaderBuilder::new()
                .delimiter(delimiter.as_byte())
                .from_reader(io::stdin())
                .deserialize::<BTreeMap<String, String>>()
                .collect::<Result<Vec<_>, _>>()?;
            for record in &mut records {
                record.retain(|_k, v| !v.is_empty())
            }
            serde_json::to_writer_pretty(io::stdout(), &records)?;
            Ok(())
        }
        Args::JsonRpc(sub) => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .context("couldn't start async runtime")?
            .block_on(async move {
                match sub {
                    JsonRpc::Capture { local, remote } => capture::capture(local, remote).await,
                    JsonRpc::Play {
                        header,
                        remote,
                        keep_going,
                    } => play(header, remote, keep_going).await,
                }
            }),
    }
}

async fn play(header: Vec<Header>, remote: String, keep_going: bool) -> anyhow::Result<()> {
    let mut errs = ErrorEmitter::new(io::stderr());
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(
            header
                .into_iter()
                .map(|Header { header, value }| (header, value))
                .collect(),
        )
        .build()
        .context("couldn't start HTTP client")?;
    for (ix, it) in serde_json::Deserializer::from_reader(io::stdin())
        .into_iter()
        .enumerate()
    {
        match it {
            Ok(Dialogue {
                method,
                params,
                response: _,
            }) => {
                let request = ez_jsonrpc::types::Request {
                    method,
                    params,
                    id: Some(ez_jsonrpc::types::Id::Number(ix.into())),
                };
                let res = client
                    .post(&remote)
                    .json(&request)
                    .send()
                    .map_err(|it| {
                        anyhow::Error::from(it).context("couldn't sent request to remote")
                    })
                    .and_then(|it| async move {
                        let body = it
                            .error_for_status()
                            .context("HTTP error from remote")?
                            .text()
                            .await
                            .context("couldn't collect HTTP response from remote")?;
                        match body.trim().is_empty() {
                            true => Ok(None),
                            false => {
                                let ez_jsonrpc::types::Response { result, id: _ } =
                                    serde_json::from_str(&body).context(
                                        "couldn't deserialize HTTP response as JSON-RPC",
                                    )?;
                                anyhow::Ok(Some(result))
                            }
                        }
                    })
                    .await;
                match (res, keep_going) {
                    (Ok(rpc), _) => {
                        serde_json::to_writer(
                            io::stdout(),
                            &Dialogue {
                                method: request.method,
                                params: request.params,
                                response: match rpc {
                                    Some(Ok(v)) => Some(DialogueResponse::Result(v)),
                                    Some(Err(e)) => Some(DialogueResponse::Error(e)),
                                    None => None,
                                },
                            },
                        )
                        .context("couldn't serialize dialogue")?;
                        println!()
                    }
                    (Err(e), true) => {
                        errs.push(format_args!("couldn't sent request at index {}: {}", ix, e))
                    }
                    (Err(e), false) => {
                        return Err(e.context(format!("couldn't execute request at index {}", ix)))
                    }
                };
            }
            Err(e) => {
                bail!("failed to deserialize dialogue at index {}: {}", ix, e)
            }
        }
    }
    errs.finish()
}

fn validate_dialogues_from_reader(
    errs: &mut ErrorEmitter<impl io::Write>,
    method2checker: HashMap<&str, CheckMethod, impl BuildHasher>,
    reader: impl io::Read,
) -> anyhow::Result<BTreeMap<String, usize>> {
    let mut passed = BTreeMap::new();
    for (pos, (ix, it)) in serde_json::Deserializer::from_reader(reader)
        .into_iter()
        .enumerate()
        .with_position()
    {
        match it {
            Ok(Dialogue {
                method,
                params,
                response,
            }) => match method2checker.get(&*method) {
                Some(check) => {
                    let in_depth = matches!(pos, itertools::Position::Only);
                    let annotations = check.check(
                        params.as_ref(),
                        response
                            .map(|it| match it {
                                DialogueResponse::Result(ok) => Ok(ok),
                                DialogueResponse::Error(err) => Err(err),
                            })
                            .as_ref(),
                        in_depth,
                    );
                    match annotations.is_empty() {
                        true => {
                            passed.entry(method).and_modify(|it| *it += 1).or_insert(1);
                        }
                        false => match in_depth {
                            true => {
                                errs.push(format_args!(
                                    "script[{}]: failed to validate method {} ({} errors)",
                                    ix,
                                    method,
                                    annotations.len()
                                ));
                                for annotation in annotations {
                                    use check::Annotation::*;
                                    match annotation {
                                        MissingRequiredParam(name) => errs.push(format_args!(
                                            "missing required parameter {}",
                                            name
                                        )),
                                        DeprecatedParam(name) => errs.push(format_args!(
                                            "use of deprecated parameter {}",
                                            name
                                        )),
                                        InvalidParam { name, errors } => {
                                            errs.push(format_args!("invalid parameter {}", name));
                                            for it in errors.into_iter().flatten() {
                                                errs.push(format_args!(
                                                    "\t{}\t{}\t{}",
                                                    it.schema_path, it.instance_path, it
                                                ))
                                            }
                                        }
                                        InvalidResult(errors) => {
                                            errs.push("invalid result:");
                                            for it in errors.into_iter().flatten() {
                                                errs.push(format_args!(
                                                    "\t{}\t{}\t{}",
                                                    it.schema_path, it.instance_path, it
                                                ))
                                            }
                                        }
                                        other => errs.push(other),
                                    }
                                }
                            }
                            false => errs.push(format!(
                                "script[{}]: failed to validate method {}: [{}]",
                                ix,
                                method,
                                annotations.iter().join(", ")
                            )),
                        },
                    }
                }
                None => errs.push(format!(
                    "script[{}]: method {} not found in spec",
                    ix, method
                )),
            },
            Err(e) => bail!("failed to deserialized dialogue at index {}: {}", ix, e),
        }
    }
    Ok(passed)
}

fn validate_document<'a>(
    errs: &mut ErrorEmitter<impl io::Write>,
    document: &'a openrpc_types::resolved::OpenRPC,
    compilation_options: jsonschema::CompilationOptions,
) -> HashMap<&'a str, CheckMethod> {
    let mut checkers = HashMap::<&str, CheckMethod>::new();
    for method @ Method {
        name,
        param_structure,
        examples,
        errors,
        params,
        ..
    } in &document.methods
    {
        if checkers.contains_key(&**name) {
            errs.push(format!(
                "spec: duplicate method {} (this will be excluded)",
                name
            ));
            continue;
        }
        errs.extend(
            params
                .iter()
                .map(|it| it.name.as_str())
                .duplicates()
                .map(|it| format!("spec: duplicate parameter name {} on method {}", it, name)),
        );
        errs.extend(
            params
                .iter()
                .filter(|it| !it.required.unwrap_or_default())
                .map(|it| {
                    format!(
                        "spec: non-required parameters are forbidden by the FIP, \
                                 but parameter {} on method {} is not required",
                        it.name, name
                    )
                }),
        );
        if param_structure.unwrap_or_default() != ParamStructure::ByPosition {
            errs.push(format!(
                "spec: param structure must be by-position according to the FIP, \
                         but is not on method {}",
                name
            ))
        }

        errs.extend(
            errors
                .iter()
                .flatten()
                .map(|it| it.code)
                .duplicates()
                .map(|it| format!("spec: duplicate error code {} on method {}", it, name)),
        );

        match CheckMethod::new(method, &compilation_options, document.components.as_ref()) {
            Ok(check_method) => {
                for (ix, ExamplePairing { params, result, .. }) in
                    examples.iter().flatten().enumerate()
                {
                    let Some(params) = params
                        .iter()
                        .map(|example| example.value.clone())
                        .collect::<Option<Vec<_>>>()
                    else {
                        errs.push(format!(
                            "spec: example at index {} for method {} must contain inline parameters (this will be excluded)",
                            ix,
                            name
                        ));
                        continue;
                    };
                    let request = jsonrpc::Request {
                        method: name.clone(),
                        params: Some(match param_structure.unwrap_or_default() {
                            ParamStructure::ByPosition | ParamStructure::Either => {
                                if params.len() > method.params.len() {
                                    // zip will drop the excess
                                    errs.push(format!(
                                        "spec: example at index {} for method {} contains too many parameters", 
                                        ix,
                                        name
                                    ));
                                }
                                RequestParameters::ByName(
                                    params
                                        .into_iter()
                                        .zip(&method.params)
                                        .map(|(p, m)| (m.name.clone(), p))
                                        .collect(),
                                )
                            }
                            ParamStructure::ByName => RequestParameters::ByPosition(params),
                        }),
                        id: Some(Id::Null),
                    };
                    let response = match result {
                        Some(it) => match it.value.clone() {
                            Some(it) => Some(jsonrpc::Response {
                                result: Ok::<_, jsonrpc::Error>(it),
                                id: Id::Null,
                            }),
                            None => {
                                errs.push(format!(
                                    "spec: example at index {} for method {} must contain an inline result (this will be excluded)",
                                    ix,
                                    name
                                ));
                                continue;
                            }
                        },
                        None => None,
                    };
                    if !check_method
                        .check(
                            request.params.as_ref(),
                            response.map(|it| it.result).as_ref(),
                            true,
                        )
                        .is_empty()
                    {
                        errs.push(format!(
                            "spec: example at index {} for method {} failed to validate",
                            ix, name
                        ))
                    };
                }
                checkers.insert(name, check_method);
            }
            Err(e) => errs.push(format!(
                "spec: error checking method {} (this will be excluded): {}",
                name, e
            )),
        }
    }
    checkers
}

struct ErrorEmitter<T> {
    inner: T,
    error: Option<io::Error>,
    count: usize,
}

impl<T> ErrorEmitter<T> {
    fn finish(self) -> anyhow::Result<()> {
        let Self {
            inner: _,
            error,
            count,
        } = self;
        if count != 0 {
            bail!("found {} errors", count)
        };
        if let Some(e) = error {
            bail!("reporting error: {}", e)
        }
        Ok(())
    }
}

impl<T, M> Extend<M> for ErrorEmitter<T>
where
    T: io::Write,
    M: fmt::Display,
{
    fn extend<I: IntoIterator<Item = M>>(&mut self, iter: I) {
        for msg in iter {
            self.push(msg)
        }
    }
}

impl<T> ErrorEmitter<T>
where
    T: io::Write,
{
    fn new(inner: T) -> Self {
        Self {
            inner,
            error: None,
            count: 0,
        }
    }
    fn push(&mut self, msg: impl fmt::Display) {
        self.count += 1;
        if let Err(e) = self.inner.write_fmt(format_args!("{}\n", msg)) {
            self.error = Some(e);
        }
    }
}

fn load_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    fn imp<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
        Ok(serde_path_to_error::deserialize(
            &mut serde_json::Deserializer::from_reader(File::open(path)?),
        )?)
    }
    imp(path.as_ref())
        .with_context(|| format!("couldn't load json from file {}", path.as_ref().display()))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Select {
    description: Option<String>,
    include: Option<InclusionDirective>,
    method: String,
}

#[derive(Serialize, Deserialize)]
enum InclusionDirective {
    Discussion,
    Include,
    Exclude,
}

#[derive(Clone)]
struct Char(AsciiChar);

impl std::str::FromStr for Char {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(AsciiChar::from_ascii(char::from_str(s)?)?))
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
struct Header {
    header: http::HeaderName,
    value: http::HeaderValue,
}

impl FromStr for Header {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, value) = s
            .split_once(':')
            .context("expected format `header-name: header-value`")?;
        Ok(Self {
            header: header.trim().parse()?,
            value: value.trim().parse()?,
        })
    }
}

#[test]
fn doc() {
    use stack_list::Node;

    fn write(buf: &mut String, tail: &Node<&str>, cmd: &clap::Command) {
        if !matches!(tail, Node::Root) {
            buf.push('\n');
        }
        let path = Node::Head {
            data: cmd.get_name(),
            tail,
        };
        for _ in 0..path.count() {
            buf.push('#')
        }
        path.for_each_rev(|component| {
            buf.push_str(" `");
            buf.push_str(component);
            buf.push('`');
        });
        buf.push('\n');
        let mut cmd = cmd
            .clone()
            .disable_help_subcommand(true)
            .disable_help_flag(true);
        std::fmt::write(buf, format_args!("\n```\n{}\n```", cmd.render_long_help())).unwrap();
        for sub in cmd.get_subcommands() {
            write(buf, &path, sub)
        }
    }

    let mut buf = String::new();
    write(
        &mut buf,
        &Node::Root,
        &<Args as clap::CommandFactory>::command(),
    );
    expect_test::expect_file!["../README.md"].assert_eq(&buf);
}

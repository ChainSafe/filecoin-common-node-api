mod capture;
mod check;
mod gc;
#[allow(unused)]
mod jsonrpc_types;

use anyhow::{bail, Context as _};
use ascii::AsciiChar;
use check::CheckMethod;
use clap::Parser;
use fluent_uri::UriRef;
use itertools::Itertools as _;
use jsonrpc_types::RequestParameters;
use openrpc_types::{
    resolve_within,
    resolved::{ExamplePairing, Method},
    ParamStructure,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt,
    fs::File,
    hash::BuildHasher,
    io::{self, IsTerminal as _},
    net::SocketAddr,
    path::{Path, PathBuf},
};

/// Utilities for creating, interacting with, and testing against the Filecoin
/// Common Node API
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

/// Subommands related to processing OpenRPC documents.
#[derive(Parser)]
enum Openrpc {
    /// Validates that:
    /// - method names are unique
    /// - parameter names are unique
    /// - there are no optional parameters
    ///
    /// Does not validate anything else, including:
    /// - that example pairings match schemas
    /// - that Example::value and Example::externalValue are mutually exclusive
    /// - dead $refs, or JSON Schema $refs
    /// - links, runtime expressions
    /// - component keys are idents
    /// - error codes are unique
    Validate { spec: PathBuf },
    /// Interpret `select` as a table of methods to include in `openrpc`, outputting
    /// a new schema with only the selected methods.
    Select {
        openrpc: PathBuf,
        select: PathBuf,
        /// Specify a new title for the schema
        #[arg(long)]
        overwrite_title: Option<String>,
        /// Specify a new version for the schema
        #[arg(long)]
        overwrite_version: Option<String>,
    },
}

/// Interact with JSON-RPC endpoints.
#[derive(Parser)]
enum JsonRpc {
    /// Start a HTTP server, forwarding all requests to a single URI.
    ///
    /// Ctrl+C will request a graceful shutdown.
    ///
    /// For HTTP exchanges whose bodies can be parsed as a singel JSON-RPC v2
    /// method call, print a summary as a JSON line to stdout.
    ///
    /// The summary includes only the method name, params, and response.
    /// This does NOT validate adherence to the JSON-RPC protocol.
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue<'a> {
    method: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<jsonrpc_types::RequestParameters<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<DialogueResponse<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DialogueResponse<'a> {
    Result(Value),
    Error(jsonrpc_types::Error<'a>),
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
        Args::JsonRpc(JsonRpc::Capture { local, remote }) => {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .context("couldn't start async runtime")?
                .block_on(capture::capture(local, remote))
        }
    }
}

fn validate_dialogues_from_reader(
    errs: &mut ErrorEmitter<impl io::Write>,
    method2checker: HashMap<&str, CheckMethod, impl BuildHasher>,
    reader: impl io::Read,
) -> anyhow::Result<BTreeMap<String, usize>> {
    let mut passed = BTreeMap::new();
    for (ix, it) in serde_json::Deserializer::from_reader(reader)
        .into_iter()
        .enumerate()
    {
        match it {
            Ok(Dialogue {
                method,
                params,
                response,
            }) => match method2checker.get(&*method) {
                Some(check) => match check
                    .check(
                        &jsonrpc_types::Request {
                            jsonrpc: jsonrpc_types::V2,
                            method: Cow::Borrowed(&*method),
                            params,
                            id: response.is_some().then_some(jsonrpc_types::Id::Null),
                        },
                        response
                            .map(|it| jsonrpc_types::Response {
                                jsonrpc: jsonrpc_types::V2,
                                result: match it {
                                    DialogueResponse::Result(it) => Ok(it),
                                    DialogueResponse::Error(e) => Err(e),
                                },
                                id: jsonrpc_types::Id::Null,
                            })
                            .as_ref(),
                    )
                    .is_empty()
                {
                    true => {
                        passed
                            .entry(method.into_owned())
                            .and_modify(|it| *it += 1)
                            .or_insert(1);
                    }
                    false => errs.push(format!(
                        "script[{}]: failed to validate method {}",
                        ix, method
                    )),
                },
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
                    let request = jsonrpc_types::Request {
                        jsonrpc: jsonrpc_types::V2,
                        method: Cow::Borrowed(name),
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
                                        .map(|(p, m)| (Cow::Borrowed(&*m.name), p))
                                        .collect(),
                                )
                            }
                            ParamStructure::ByName => RequestParameters::ByPosition(params),
                        }),
                        id: Some(jsonrpc_types::Id::Null),
                    };
                    let response = match result {
                        Some(it) => match it.value.clone() {
                            Some(it) => Some(jsonrpc_types::Response {
                                jsonrpc: jsonrpc_types::V2,
                                result: Ok::<_, jsonrpc_types::Error<Value>>(it),
                                id: jsonrpc_types::Id::Null,
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
                    if !check_method.check(&request, response.as_ref()).is_empty() {
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

//! Supporting code for a test suite against Filecoin RPC servers,
//! with the following goals:
//! - Filtering and grouping functionality.
//!   - By user tags.
//!   - By e.g authorization token requirements.
//! - Extensible to provide test resources
//!   (e.g loading and testing against Filecoin snapshots for deterministic nodes).
//! - Accessible to Rust beginners:
//!   - No `async` code.
//!   - Limited fancy traits.
//!   - Limited exposure to lifetimes.
//! - Difficult to misuse.
//!
//! # Current non-goals
//! - Custom setup/teardown - this could be handled _outside_ the binary,
//!   and managed with the filtering.
//! - Isolation between tests.
//! - Test timeouts (this is difficult without async).
//! - Performance.

use std::{
    any::type_name,
    collections::BTreeSet,
    convert::Infallible,
    fmt,
    io::{self, Read as _, Write as _},
    ops::ControlFlow,
    panic::{self, AssertUnwindSafe},
    process,
    sync::mpsc,
    time::Duration,
};

use ez_jsonrpc::{
    params::SerializePositional,
    types::{self as jsonrpc, RequestParameters},
};
use owo_colors::OwoColorize as _;
use serde::{de::DeserializeOwned, Deserialize};

pub use config::Harness as Config;

/// Create a test case that is provided with a [`V0Client`] with a `read` token.
///
/// `name` SHOULD:
/// - be unique.
/// - NOT contain tabs.
/// - fit on a single line, with no punctuation.
pub fn v0read<'a>(
    name: impl Into<String>,
    tags: impl IntoIterator<Item = Tag>,
    f: impl FnOnce(V0Client) -> Result<(), TestFailure> + 'a,
) -> Test<'a> {
    Test {
        name: name.into(),
        tags: tags.into_iter().collect(),
        inner: TestInner::V0Read(Box::new(f)),
    }
}

/// A dynamic test case, created by free functions in this module.
pub struct Test<'a> {
    name: String,
    tags: BTreeSet<Tag>,
    inner: TestInner<'a>,
}

impl Test<'_> {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn tags(&self) -> impl Iterator<Item = &Tag> + '_ {
        self.tags.iter()
    }
}

/// Closed set of tags for [`Test`]s, used for filtering.
#[derive(
    strum::Display, strum::EnumString, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Tag {
    /// Simple test that just tries to cover schemas.
    SchemaCoverage,
}

#[derive(Debug)]
pub struct V0Client(Client);

/// Represents that a single test has failed - the runner may stop running.
///
/// This SHOULD be propogated with the `?` operator.
// This is a shim of anyhow::Error to enforce semantics.
// We hide Send + Sync + 'static bounds for messages to hide complexity.
pub struct TestFailure(anyhow::Error);

impl TestFailure {
    /// Manually fail the test with the given message.
    pub fn msg(msg: impl fmt::Display) -> Self {
        Self(anyhow::Error::msg(msg.to_string()))
    }
    /// Attach additional context to the error.
    pub fn context(self, msg: impl fmt::Display) -> Self {
        Self(self.0.context(msg.to_string()))
    }
    /// By default, when an JSON-RPC method call returns an [Error Object](jsonrpc::Error),
    /// it is assumed to be a test failure.
    ///
    /// You may manually extract such an error if it exists using this method.
    pub fn into_rpc_error(self) -> Result<jsonrpc::Error, TestFailure> {
        self.0.downcast().map_err(TestFailure)
    }
    #[doc(hidden)]
    #[deprecated = "not public API"]
    pub fn __new(e: anyhow::Error) -> Self {
        Self(e)
    }
}

impl<T> From<T> for TestFailure
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// We support a small number of test scenarios.
enum TestInner<'a> {
    V0Read(Box<dyn FnOnce(V0Client) -> Result<(), TestFailure> + 'a>),
}

/// On-disk config for the harness.
mod config {
    use schemars::JsonSchema;
    use serde::Deserialize;
    use std::time::Duration;

    #[derive(Deserialize, Default, JsonSchema)]
    pub struct Harness {
        pub(super) v0: Option<Client>,
        pub(super) timeouts: Option<Timeout>,
        pub(super) exclude: Option<Vec<String>>,
    }

    #[derive(Deserialize, Default, JsonSchema)]
    pub struct Client {
        pub(super) url: String,
        pub(super) read_token: Option<String>,
        pub(super) write_token: Option<String>,
        pub(super) admin_token: Option<String>,
    }

    #[derive(Deserialize, Default, JsonSchema)]
    pub struct Timeout {
        #[serde(with = "humantime_serde")]
        #[schemars(with = "String")]
        pub(super) default: Option<Duration>,
        #[serde(with = "humantime_serde")]
        #[schemars(with = "String")]
        pub(super) long: Option<Duration>,
    }
}

struct RunnerClients {
    v0read: Option<Client>,
}

impl RunnerClients {
    fn from_config(config: &config::Harness, log: &LogSender) -> Self {
        let config::Harness { v0, timeouts, .. } = config;

        // Shared client for conneciton pooling
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .expect("couldn't initialize client");
        Self {
            v0read: v0.as_ref().and_then(|config| {
                Client::from_config(&client, log, config, timeouts.as_ref(), AuthMode::Read)
            }),
        }
    }
}

#[derive(Debug, Clone)]
struct Client {
    client: reqwest::blocking::Client,
    token: Option<String>,
    url: String,
    id: u64,
    log: LogSender,

    timeout_mode: TimeoutMode,
    timeout_default: Duration,
    timeout_long: Duration,
}

impl Client {
    fn from_config(
        client: &reqwest::blocking::Client,
        log: &LogSender,
        config: &config::Client,
        timeouts: Option<&config::Timeout>,
        auth_mode: AuthMode,
    ) -> Option<Self> {
        let config::Client {
            url,
            read_token,
            write_token,
            admin_token,
        } = config;
        Some(Self {
            client: client.clone(),
            token: match auth_mode {
                AuthMode::Read => Some(read_token.clone()?),
                AuthMode::Write => Some(write_token.clone()?),
                AuthMode::Admin => Some(admin_token.clone()?),
                AuthMode::None => None,
            },
            url: url.clone(),
            id: 0,
            log: log.clone(),
            timeout_mode: TimeoutMode::Default,
            timeout_default: timeouts
                .and_then(|it| it.default)
                .unwrap_or(Duration::from_secs(30)),
            timeout_long: timeouts
                .and_then(|it| it.default)
                .unwrap_or(Duration::from_secs(90)),
        })
    }
    fn call<T: DeserializeOwned>(
        &mut self,
        method: impl Into<String>,
        args: impl SerializePositional,
    ) -> Result<T, TestFailure> {
        let request_id = jsonrpc::Id::Number(self.id.into());
        self.id += 1;
        let request = jsonrpc::Request {
            method: method.into(),
            params: Some(RequestParameters::ByPosition(
                args.ser_positional(ez_jsonrpc::params::ser::ByPosition::new())
                    .context("couldn't serialize params")?,
            )),
            id: Some(request_id.clone()),
        };
        self.log.log(LogEvent::Request(request.clone()));
        let builder = self
            .client
            .post(&self.url)
            .json(&request)
            .timeout(match self.timeout_mode {
                TimeoutMode::Default => self.timeout_default,
                TimeoutMode::Long => self.timeout_long,
            });

        let mut resp = match &self.token {
            Some(tok) => builder.bearer_auth(tok),
            None => builder,
        }
        .send()
        .context("couldn't send HTTP request")?;
        let mut body = vec![];
        resp.read_to_end(&mut body)
            .context("couldn't collect HTTP response body from server")?;
        match (
            resp.error_for_status().context("HTTP error from server"),
            serde_path_to_error::deserialize::<_, jsonrpc::Response>(
                &mut serde_json::Deserializer::from_slice(&body),
            )
            .context("invalid JSON-RPC response from server"),
        ) {
            (Err(e), _) | (_, Err(e)) => {
                self.log.log(LogEvent::Body(body));
                Err(e)
            }
            (_, Ok(response)) => {
                self.log.log(LogEvent::Response(response.clone()));
                if request_id != response.id {
                    fail!("server violated the JSON-RPC protocol (member `id` does not match)")
                }
                let o = response.result.context("JSON-RPC call returned an error")?;
                let o = T::deserialize(o).context(format!(
                    "couldn't deserialize JSON-RPC response into a {}",
                    type_name::<T>()
                ))?;
                Ok(o)
            }
        }
    }
}

#[derive(Debug, Clone)]
enum TimeoutMode {
    Default,
    Long,
}

#[derive(Default)]
enum AuthMode {
    Read,
    Write,
    Admin,
    #[default]
    None,
}

/// Run the given test cases.
///
/// This SHOULD only be called once, unconditionally.
pub fn run<'a>(
    tests: impl IntoIterator<Item = Test<'a>>,
    config: config::Harness,
    mut filter: impl FnMut(&str, &BTreeSet<Tag>) -> ControlFlow<()>,
) -> anyhow::Result<()> {
    // TODO(aatifsyed): seeded randomization of tests
    let mut tests = tests.into_iter().collect::<Vec<_>>();
    tests.sort_by(|l, r| l.name.cmp(&r.name));

    let (log_tx, log_rx) = mpsc::channel();
    let log = LogSender(log_tx);

    let runner_clients = RunnerClients::from_config(&config, &log);

    let mut skipped = 0;
    let mut succeeded = 0;
    let mut failed = 0;

    let stdout = &mut anstream::AutoStream::auto(io::stdout());
    let stderr = &mut anstream::AutoStream::auto(io::stderr());
    let mut failed_logs = vec![];

    panic::set_hook(Box::new(|_| {})); // TODO(aatifsyed): siphon off a backtrace and log it
    for (ix, Test { name, tags, inner }) in tests.into_iter().enumerate() {
        write!(stdout, "{}\t{}\t", ix.dimmed(), name.white())?;

        if filter(&name, &tags).is_break() {
            writeln!(stdout, "{}", "skipped (filter)".yellow())?;
            skipped += 1;
            continue;
        }
        let res = match (inner, &runner_clients) {
            (
                TestInner::V0Read(runme),
                RunnerClients {
                    v0read: Some(client),
                    ..
                },
            ) => panic::catch_unwind(AssertUnwindSafe(|| runme(V0Client(client.clone())))),
            _ => {
                writeln!(stdout, "{}", "skipped (harness)".yellow())?;
                skipped += 1;
                continue;
            }
        };

        let mut logs = log_rx.try_iter().collect::<Vec<_>>();
        match res {
            Ok(Ok(())) => {
                writeln!(stdout, "{}", "passed".green())?;
                succeeded += 1;
            }
            Ok(Err(TestFailure(e))) => {
                writeln!(stdout, "{}", "failed".red())?;
                failed += 1;

                logs.push(LogEvent::Errors(e));
                write_logs(&mut *stderr, &logs)?;
                failed_logs.push((name, logs));
            }
            Err(panic) => {
                writeln!(stdout, "{}", "failed".red())?;
                failed += 1;

                let panic_msg = match (panic.downcast_ref::<String>(), panic.downcast_ref::<&str>())
                {
                    (Some(s), _) => s.as_str(),
                    (_, Some(s)) => s,
                    _ => "Box<dyn Any>",
                };
                logs.push(LogEvent::Errors(anyhow::Error::msg(panic_msg.to_string())));
                write_logs(&mut *stderr, &logs)?;
                failed_logs.push((name, logs));
            }
        }
    }

    if skipped + succeeded + failed > 1 {
        for (ix, (name, logs)) in failed_logs.into_iter().enumerate() {
            writeln!(stdout, "failure {} ({})", ix, name)?;
            write_logs(&mut *stderr, &logs)?;
        }
    }

    writeln!(
        stderr,
        "{} skipped, {} succeeded, {} failed",
        skipped, succeeded, failed
    )?;

    match failed {
        0 => process::exit(0),
        _ => process::exit(1),
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T, E> Sealed for Result<T, E> {}
    impl<T> Sealed for Option<T> {}
}

/// Utility trait for failing a test on a [`Result::Err`] or [`Option::None`].
pub trait Context<T, E>: sealed::Sealed {
    fn context<C: fmt::Display>(self, context: C) -> Result<T, TestFailure>;
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C: fmt::Display>(self, context: C) -> Result<T, TestFailure> {
        anyhow::Context::context(self, context.to_string()).map_err(TestFailure)
    }
}

impl<T> Context<T, Infallible> for Option<T> {
    fn context<C: fmt::Display>(self, context: C) -> Result<T, TestFailure> {
        anyhow::Context::context(self, context.to_string()).map_err(TestFailure)
    }
}

macro_rules! fail {
    ($msg:literal $(,)?) => {
        return ::core::result::Result::Err(
            #[allow(deprecated)]
            $crate::test_harness::TestFailure::__new(::anyhow::anyhow!($msg))
        )
    };
    ($err:expr $(,)?) => {
        return ::core::result::Result::Err(
            #[allow(deprecated)]
            $crate::test_harness::TestFailure::__new(::anyhow::anyhow!($err))
        )
    };
    ($fmt:expr, $($arg:tt)*) => {
        return ::core::result::Result::Err(
            #[allow(deprecated)]
            $crate::test_harness::TestFailure::__new(::anyhow::anyhow!($fmt, $($arg)*))
        )
    };
}
pub(crate) use fail;

#[derive(Debug, Clone)]
struct LogSender(std::sync::mpsc::Sender<LogEvent>);

impl LogSender {
    fn log(&self, event: LogEvent) {
        let _listener_dropped = self.0.send(event);
    }
}

#[derive(Debug)]
enum LogEvent {
    User(String),
    Request(jsonrpc::Request),
    Response(jsonrpc::Response),
    Body(Vec<u8>),
    Errors(anyhow::Error),
}

/// `writer` will receive (ANSI) colored reports
fn write_logs<'a>(
    mut writer: impl io::Write,
    logs: impl IntoIterator<Item = &'a LogEvent>,
) -> anyhow::Result<()> {
    let w = &mut writer;
    for (ix, log) in logs.into_iter().enumerate() {
        match log {
            LogEvent::User(msg) => writeln!(w, "\t{ix}\t{}\t{msg}", msg.blue())?,
            LogEvent::Request(it) => {
                let it = serde_json::to_string_pretty(it)?;
                writeln!(w, "\t{ix}\t{}", "req".blue())?;
                for line in it.lines() {
                    writeln!(w, "\t{}", line)?
                }
            }
            LogEvent::Response(it) => {
                let it = serde_json::to_string_pretty(it)?;
                writeln!(w, "\t{ix}\t{}", "resp".blue())?;
                for line in it.lines() {
                    writeln!(w, "\t{}", line)?
                }
            }
            LogEvent::Body(it) => {
                writeln!(w, "\t{ix}\t{}", "body".blue())?;
                for line in String::from_utf8_lossy(it).lines() {
                    writeln!(w, "\t{}", line)?
                }
            }
            LogEvent::Errors(e) => {
                writeln!(w, "\t{ix}\t{}", "err".blue())?;
                for err in e.chain() {
                    writeln!(w, "\t{}", err)?
                }
            }
        }
    }
    Ok(())
}

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

/// This code is generated by the `openrpc generate` subcommand.
// TODO(aatifsyed): this could be put in a separate crate to improve compile times
#[path = "test/generated.rs"]
pub mod types;

use std::{
    any::type_name,
    collections::BTreeSet,
    convert::Infallible,
    fmt,
    io::{self, Read as _, Write as _},
    panic::{self, AssertUnwindSafe},
    process,
    sync::{
        atomic::{AtomicU32, Ordering::SeqCst},
        mpsc::{self, Sender},
        Arc,
    },
    time::Duration,
};

use ez_jsonrpc::{
    params::SerializePositional,
    types::{self as jsonrpc, RequestParameters},
};
use owo_colors::OwoColorize as _;
use serde::de::DeserializeOwned;

/// Create a test case.
///
/// `name` SHOULD:
/// - be unique.
/// - fit on a single line, with no punctuation.
pub fn test<'a>(
    name: impl Into<String>,
    f: impl FnOnce(ConfigTest) -> Result<(), TestFailure> + 'a,
) -> Test<'a> {
    Test {
        name: name.into(),
        runner: Box::new(f),
    }
}

/// Dynamic test case, created with the [`test`] function.
pub struct Test<'a> {
    name: String,
    #[allow(clippy::type_complexity)]
    runner: Box<dyn FnOnce(ConfigTest) -> Result<(), TestFailure> + 'a>,
}

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
    /// See the documentation on [`Ctx::call`].
    pub fn as_rpc_error(&self) -> Option<&jsonrpc::Error> {
        self.0.downcast_ref()
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

/// Closed set of tags for the test suite, used to filter multiple tests.
#[derive(strum::Display, strum::EnumString, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Tag {
    /// Simple test that just tries to cover schemas.
    SchemaCoverage,
}

/// Configure your test.
///
/// This should be used to request any of the following from the test harness:
/// - Authorization scopes.
/// - Resource bundles.
/// - Categorization.
///
/// After configuration, you may call [`Self::begin_test`] to obtain a handle
/// with which RPC calls may be made.
pub struct ConfigTest {
    per_run: Arc<PerRun>,
    log: Sender<Log>,
    requested: BTreeSet<ConfigRequest>,
    tags: BTreeSet<Tag>,
}

/// We keep track of all the configuration that the user has requested to catch
/// over or under provision.
///
/// This is important for maintaining test quality.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum ConfigRequest {
    Read,
    Write,
    Admin,
    Wrong,
}

impl ConfigTest {
    /// This test requires a token with `read` permissions.
    pub fn read_auth(mut self) -> Self {
        self.requested.insert(ConfigRequest::Read);
        self
    }
    /// This test requires a token with `write` permissions.
    pub fn write_auth(mut self) -> Self {
        self.requested.insert(ConfigRequest::Write);
        self
    }
    /// This test requires a token with `admin` permissions.
    pub fn admin_auth(mut self) -> Self {
        self.requested.insert(ConfigRequest::Admin);
        self
    }
    /// This test requires an invalid token.
    pub fn wrong_auth(mut self) -> Self {
        self.requested.insert(ConfigRequest::Wrong);
        self
    }
    /// Apply a single [`Tag`] to this test.
    pub fn tag(mut self, tag: Tag) -> Self {
        self.tags.insert(tag);
        self
    }
    /// Apply a multiple [`Tag`]s to this test.
    pub fn tags(mut self, tags: impl IntoIterator<Item = Tag>) -> Self {
        self.tags.extend(tags);
        self
    }
    /// End configuration, possibly [cancelling](Cancelled) this test
    /// (if required resources were not loaded, or the test was filtered out).
    pub fn begin_test(self) -> Result<RunTest, Cancelled> {
        let Self {
            per_run,
            requested,
            tags,
            log,
        } = self;
        for requested in &requested {
            match (
                requested,
                &per_run.token_read,
                &per_run.token_write,
                &per_run.token_admin,
                &per_run.token_wrong,
            ) {
                (ConfigRequest::Read, None, _, _, _) => return Err(Cancelled(())),
                (ConfigRequest::Write, _, None, _, _) => return Err(Cancelled(())),
                (ConfigRequest::Admin, _, _, None, _) => return Err(Cancelled(())),
                (ConfigRequest::Wrong, _, _, _, None) => return Err(Cancelled(())),
                _ => {}
            }
        }
        if !tags.is_subset(&per_run.tag_filter) {
            return Err(Cancelled(()));
        }
        Ok(RunTest {
            per_run,
            used: requested.clone(),
            requested,
            timeout_mode: TimeoutMode::Default,
            auth_mode: AuthMode::None,
            log,
        })
    }
}

/// The core handle for running a test, used to make RPC calls.
pub struct RunTest {
    per_run: Arc<PerRun>,

    timeout_mode: TimeoutMode,
    auth_mode: AuthMode,
    log: Sender<Log>,

    requested: BTreeSet<ConfigRequest>,
    used: BTreeSet<ConfigRequest>,
}

impl RunTest {
    /// [`Self::call`]s after this will use a longer timeout.
    pub fn long_timeout(&mut self) {
        self.timeout_mode = TimeoutMode::Long
    }

    /// [`Self::call`]s after this will use the default timeout.
    pub fn normal_timeout(&mut self) {
        self.timeout_mode = TimeoutMode::Default
    }

    /// [`Self::call`]s after this will be unauthorised.
    /// (This is the default).
    pub fn no_auth(&mut self) {
        self.auth_mode = AuthMode::None
    }
    /// [`Self::call`]s after this will use a token with `read` permissions.
    pub fn read_auth(&mut self) {
        self.auth_mode = AuthMode::Read
    }
    /// [`Self::call`]s after this will use a token with `write` permissions.
    pub fn write_auth(&mut self) {
        self.auth_mode = AuthMode::Write
    }
    /// [`Self::call`]s after this will use a token with `admin` permissions.
    pub fn admin_auth(&mut self) {
        self.auth_mode = AuthMode::Admin
    }
    /// [`Self::call`]s after this will have malformed authorisation.
    pub fn wrong_auth(&mut self) {
        self.auth_mode = AuthMode::Wrong
    }
    /// Add a custom message to the test log.
    ///
    /// By default, this is printed on failure.
    pub fn log(&mut self, msg: impl fmt::Display) {
        let _listener_dropped = self.log.send(Log::User(msg.to_string()));
    }

    /// Call a JSON-RPC method.
    ///
    /// JSON-RPC [specifies an error object](https://www.jsonrpc.org/specification#error_object)
    /// that all methods may return.
    ///
    /// Such errors are assumed to _also_ be [`TestFailure`]s -
    /// you may manually check to see if a method returned an error by calling
    /// [`TestFailure::as_rpc_error`].
    pub fn call<T: DeserializeOwned>(
        &mut self,
        method: impl Into<String>,
        args: impl SerializePositional,
    ) -> Result<T, TestFailure> {
        let request_id = jsonrpc::Id::Number(self.per_run.id.fetch_add(1, SeqCst).into());
        let request = jsonrpc::Request {
            method: method.into(),
            params: Some(RequestParameters::ByPosition(
                args.ser_positional(ez_jsonrpc::params::ser::ByPosition::new())
                    .context("couldn't serialize params")?,
            )),
            id: Some(request_id.clone()),
        };
        let _listener_dropped = self.log.send(Log::Request(request.clone()));
        let builder = self
            .per_run
            .client
            .post(&self.per_run.url)
            .json(&request)
            .timeout(match self.timeout_mode {
                TimeoutMode::Default => self.per_run.timeout_default,
                TimeoutMode::Long => self.per_run.timeout_long,
            });

        let (src, used) = match self.auth_mode {
            AuthMode::None => (None, None),
            AuthMode::Wrong => (Some(&self.per_run.token_wrong), Some(ConfigRequest::Wrong)),
            AuthMode::Read => (Some(&self.per_run.token_read), Some(ConfigRequest::Read)),
            AuthMode::Write => (Some(&self.per_run.token_write), Some(ConfigRequest::Write)),
            AuthMode::Admin => (Some(&self.per_run.token_admin), Some(ConfigRequest::Admin)),
        };

        if let Some(used) = used {
            self.used.remove(&used);
        }

        let mut resp = match src {
            Some(Some(tok)) => builder.bearer_auth(tok),
            Some(None) => fail!("incorrectly configured test: required token not present"),
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
                let _listener_dropped = self.log.send(Log::Body(body));
                Err(e)
            }
            (_, Ok(response)) => {
                let _listener_dropped = self.log.send(Log::Response(response.clone()));
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

/// Represents that a test should not be run.
/// This is NOT a failure state - see [`TestFailure`].
#[derive(Debug, thiserror::Error)]
#[error("test cancelled after configuration")]
pub struct Cancelled(());

/// State that is shared across many [`Test`] executions.
#[derive(Debug)]
struct PerRun {
    id: AtomicU32,
    tag_filter: BTreeSet<Tag>,

    client: reqwest::blocking::Client,
    url: String,

    timeout_default: Duration,
    timeout_long: Duration,

    token_read: Option<String>,
    token_write: Option<String>,
    token_admin: Option<String>,
    token_wrong: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum TimeoutMode {
    Default,
    Long,
}

#[derive(Debug, Clone, Copy)]
enum AuthMode {
    None,
    Wrong,
    Read,
    Write,
    Admin,
}

#[derive(clap::Parser)]
pub struct Args {
    /// Only run the test with the given name.
    #[arg(long)]
    pub name: Option<String>,

    /// If provided, only run [`Test`]s which contain ALL of the given [`Tag`]s.
    #[arg(long)]
    pub tags: Vec<Tag>,

    /// The URL to make RPC requests to.
    #[arg(long)]
    pub url: String,

    /// A default RPC call timeout, in seconds.
    #[arg(long, default_value = "60")]
    pub default_timeout: f64,
    /// The `long` RPC call timeout, in seconds.
    #[arg(long, default_value = "120")]
    pub long_timeout: f64,

    /// An authorisation token with read permissions.
    #[arg(long)]
    pub read: Option<String>,
    /// An authorisation token with write permissions.
    #[arg(long)]
    pub write: Option<String>,
    /// An authorisation token with admin permissions.
    #[arg(long)]
    pub admin: Option<String>,
    /// A malformed authorisation token.
    #[arg(long)]
    pub wrong: Option<String>,
}

/// Run the given test cases.
///
/// This SHOULD only be called once, unconditionally.
pub fn run<'a>(tests: impl IntoIterator<Item = Test<'a>>, args: Args) -> anyhow::Result<()> {
    let mut tests = tests.into_iter().collect::<Vec<_>>();
    tests.sort_by_cached_key(|it| it.name.clone());

    let Args {
        name: filter_name,
        tags,
        url,
        default_timeout,
        long_timeout,
        read,
        write,
        admin,
        wrong,
    } = args;

    let mut skipped = 0;
    let mut succeeded = 0;
    let mut failed = 0;

    let per_run = Arc::new(PerRun {
        id: AtomicU32::new(0),
        tag_filter: tags.into_iter().collect(),
        client: reqwest::blocking::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .expect("couldn't initialize client"),
        url,
        timeout_default: Duration::from_secs_f64(default_timeout),
        timeout_long: Duration::from_secs_f64(long_timeout),
        token_read: read,
        token_write: write,
        token_admin: admin,
        token_wrong: wrong,
    });

    let stdout = &mut anstream::AutoStream::auto(io::stdout());
    let stderr = &mut anstream::AutoStream::auto(io::stderr());
    let mut failed_logs = vec![];

    let (log_tx, log_rx) = mpsc::channel();

    panic::set_hook(Box::new(|_| {})); // TODO(aatifsyed): siphon off a backtrace and log it
    for (ix, Test { name, runner }) in tests.into_iter().enumerate() {
        let handle = ConfigTest {
            per_run: per_run.clone(),
            log: log_tx.clone(),
            requested: BTreeSet::new(),
            tags: BTreeSet::new(),
        };
        write!(stdout, "{}\t{}\t", ix.dimmed(), name.white())?;
        let res = match &filter_name {
            Some(filter_name) => match name == *filter_name {
                true => panic::catch_unwind(AssertUnwindSafe(|| runner(handle))),
                false => {
                    writeln!(stdout, "{}", "skipped (name)".yellow())?;
                    skipped += 1;
                    continue;
                }
            },
            None => panic::catch_unwind(AssertUnwindSafe(|| runner(handle))),
        };
        let mut logs = log_rx.try_iter().collect::<Vec<_>>();
        match res {
            Ok(Ok(())) => {
                writeln!(stdout, "{}", "passed".green())?;
                succeeded += 1;
            }
            Ok(Err(TestFailure(e))) => match e.downcast_ref::<Cancelled>() {
                Some(_) => {
                    writeln!(stdout, "{}", "skipped".yellow())?;
                    skipped += 1
                }
                None => {
                    writeln!(stdout, "{}", "failed".red())?;
                    failed += 1;
                    logs.push(Log::Errors(e));
                    write_logs(&mut *stderr, &logs)?;
                    failed_logs.push((name, logs));
                }
            },
            Err(panic) => {
                let panic_msg = match (panic.downcast_ref::<String>(), panic.downcast_ref::<&str>())
                {
                    (Some(s), _) => s.as_str(),
                    (_, Some(s)) => s,
                    _ => "Box<dyn Any>",
                };
                writeln!(stdout, "{}", "failed".red())?;
                failed += 1;
                logs.push(Log::Errors(anyhow::Error::msg(panic_msg.to_string())));
                write_logs(&mut *stderr, &logs)?;
                failed_logs.push((name, logs));
            }
        }
    }
    let _unregister = panic::take_hook();

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
        return ::core::result::Result::Err(#[allow(deprecated)] $crate::test::TestFailure::__new(::anyhow::anyhow!($msg)))
    };
    ($err:expr $(,)?) => {
        #[allow(deprecated)]
        return ::core::result::Result::Err(#[allow(deprecated)] $crate::test::TestFailure::__new(::anyhow::anyhow!($err)))
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[allow(deprecated)]
        return ::core::result::Result::Err(#[allow(deprecated)] $crate::test::TestFailure::__new(::anyhow::anyhow!($fmt, $($arg)*)))
    };
}
pub(crate) use fail;

#[derive(Debug)]
enum Log {
    User(String),
    Request(jsonrpc::Request),
    Response(jsonrpc::Response),
    Body(Vec<u8>),
    Errors(anyhow::Error),
}

/// `writer` will receive (ANSI) colored reports
fn write_logs<'a>(
    mut writer: impl io::Write,
    logs: impl IntoIterator<Item = &'a Log>,
) -> anyhow::Result<()> {
    let w = &mut writer;
    for (ix, log) in logs.into_iter().enumerate() {
        match log {
            Log::User(msg) => writeln!(w, "\t{ix}\t{}\t{msg}", msg.blue())?,
            Log::Request(it) => {
                let it = serde_json::to_string_pretty(it)?;
                writeln!(w, "\t{ix}\t{}", "req".blue())?;
                for line in it.lines() {
                    writeln!(w, "\t{}", line)?
                }
            }
            Log::Response(it) => {
                let it = serde_json::to_string_pretty(it)?;
                writeln!(w, "\t{ix}\t{}", "resp".blue())?;
                for line in it.lines() {
                    writeln!(w, "\t{}", line)?
                }
            }
            Log::Body(it) => {
                writeln!(w, "\t{ix}\t{}", "body".blue())?;
                for line in String::from_utf8_lossy(it).lines() {
                    writeln!(w, "\t{}", line)?
                }
            }
            Log::Errors(e) => {
                writeln!(w, "\t{ix}\t{}", "err".blue())?;
                for err in e.chain() {
                    writeln!(w, "\t{}", err)?
                }
            }
        }
    }
    Ok(())
}

use std::{
    any::type_name,
    convert::Infallible,
    fmt,
    io::Read as _,
    sync::atomic::{AtomicU64, Ordering},
};

use bstr::BString;
use ez_jsonrpc::{
    params::SerializePositional,
    types::{self as jsonrpc, RequestParameters},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::debug;

/// Represents that a single test has failed - the runner may stop running.
///
/// This SHOULD be propogated with the `?` operator.
// This is a shim of anyhow::Error to enforce semantics.
// We hide Send + Sync + 'static bounds for messages to hide complexity.
pub struct TestFailure(anyhow::Error);

impl TestFailure {
    pub fn msg(msg: impl fmt::Display) -> Self {
        Self(anyhow::Error::msg(msg.to_string()))
    }
    pub fn context(self, msg: impl fmt::Display) -> Self {
        Self(self.0.context(msg.to_string()))
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

/// Create a test case.
///
/// `name` SHOULD be unique.
pub fn test<'a>(
    name: impl Into<String>,
    f: impl FnOnce(&mut Ctx) -> Result<(), TestFailure> + 'a,
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
    runner: Box<dyn FnOnce(&mut Ctx) -> Result<(), TestFailure> + 'a>,
}

/// Closed set of tags for the test suite.
#[derive(strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Tag {}

/// Context for a [`Test`].
///
/// You may use this to:
/// - Send and receive JSON-RPC requests.
/// - Request resources like snapshots from the runner.
/// - Categorize your test.
pub struct Ctx {
    client: reqwest::blocking::Client,
    url: String,
    id: AtomicU64,
    tags: Vec<Tag>,
}

impl Ctx {
    /// Apply tags to this [`Test`].
    ///
    /// This SHOULD be called early, and SHOULD NOT be conditional.
    #[allow(private_bounds)]
    pub fn tag(&mut self, tags: impl TagCtx) -> &mut Self {
        tags.tag_ctx(&mut self.tags);
        self
    }
    /// `JSON-RPC` calls may return a JSON [`Error`](jsonrpc::Error) object,
    /// but this is coerced to a test-level error,
    /// which is probably what you want.
    ///
    /// To access the JSON-RPC error, see [`Self::call_result`].
    pub fn call<T: DeserializeOwned>(
        &self,
        method: impl Into<String>,
        args: impl SerializePositional,
    ) -> Result<T, TestFailure> {
        self.call_result(method, args)?.context("JSON-RPC error")
    }

    /// Access JSON-RPC level [`Error`](jsonrpc::Error)s returned by the remote.
    ///
    /// See [`Self::call`], if a JSON-RPC level error is also a test-level error.
    pub fn call_result<T: DeserializeOwned>(
        &self,
        method: impl Into<String>,
        args: impl SerializePositional,
    ) -> Result<Result<T, jsonrpc::Error>, TestFailure> {
        let request_id = jsonrpc::Id::Number(self.id.fetch_add(1, Ordering::SeqCst).into());
        let request = jsonrpc::Request {
            method: method.into(),
            params: Some(RequestParameters::ByPosition(
                args.ser_positional(ez_jsonrpc::params::ser::ByPosition::new())
                    .context("couldn't serialize params")?,
            )),
            id: Some(request_id.clone()),
        };
        debug!(?request);
        let mut resp = self
            .client
            .post(&self.url)
            .json(&request)
            .send()
            .context("couldn't send HTTP request")?;
        let mut body = BString::default();
        resp.read_to_end(&mut body)
            .context("couldn't collect HTTP response body from server")?;
        match (
            resp.error_for_status().context("HTTP error from server"),
            serde_json::from_slice::<jsonrpc::Response>(&body)
                .context("invalid JSON-RPC response from server"),
        ) {
            (Err(e), _) | (_, Err(e)) => {
                debug!(?body);
                Err(e)
            }
            (_, Ok(response)) => {
                debug!(?response);
                if request_id != response.id {
                    fail!("server violated the JSON-RPC protocol (member `id` does not match)")
                }
                match response.result {
                    Ok(o) => {
                        let o = T::deserialize(o).context(format!(
                            "couldn't deserialize JSON-RPC response into a {}",
                            type_name::<T>()
                        ))?;
                        Ok(Ok(o))
                    }
                    Err(e) => Ok(Err(e)),
                }
            }
        }
    }
}

/// Utility trait to allow [`Ctx::tag`] to accept either a single [`Tag`] or a
/// collection of many.
trait TagCtx {
    fn tag_ctx(self, tags: &mut Vec<Tag>);
}

impl TagCtx for Tag {
    fn tag_ctx(self, tags: &mut Vec<Tag>) {
        tags.push(self)
    }
}

impl<T> TagCtx for T
where
    T: IntoIterator<Item = Tag>,
{
    fn tag_ctx(self, tags: &mut Vec<Tag>) {
        tags.extend(self)
    }
}

macro_rules! fail {
    ($msg:literal $(,)?) => {
        #[allow(deprecated)]
        return ::core::result::Result::Err($crate::test::TestFailure::__new(::anyhow::anyhow!($msg)))
    };
    ($err:expr $(,)?) => {
        #[allow(deprecated)]
        return ::core::result::Result::Err($crate::test::TestFailure::__new(::anyhow::anyhow!($err)))
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[allow(deprecated)]
        return ::core::result::Result::Err($crate::test::TestFailure::__new(::anyhow::anyhow!($fmt, $($arg)*)))
    };
}
pub(crate) use fail;

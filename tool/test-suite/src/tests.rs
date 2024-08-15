//! Collection of tests against an RPC node.
//!
//! # Overview
//! Tests are functions created using functions like [`v0read`].
//! The test function will receive, in the case of the above,
//! an RPC client with a `read` authorization token which can make calls against
//! a v0 RPC API.
//!
//! Tests are free to [`panic!`], [`assert!`], [`fail!`], or [propogate errors](crate::harness::Context).
//!
//!

pub use crate::harness::prelude::*;
pub use assert2::assert;

/// This should produce the definitive collection of tests,
/// with no conditional test inclusion - all filtering must be done by the harness.
pub fn all() -> Vec<Test<'static>> {
    [v0none(
        "ChainHead with no authorization token",
        [Tag::SchemaCoverage],
        |client| {
            let tipset = client.Filecoin_ChainHead()?;
            assert!(tipset.height >= 0);
            client.log("this is a log message");
            Ok(())
        },
    )]
    .into()
}

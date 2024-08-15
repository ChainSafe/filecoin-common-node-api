//! Collection of tests against an RPC node.
//!
//! # Overview
//! Tests are functions created using functions like [`v0read`].
//! The test function will receive, in the case of the above,
//! an RPC client with a `read` authorization token.
//!
//! It is free to [`panic!`], [`assert!`], [`fail!`], or [propogate errors](crate::test_harness::Context)
//! as desired by the test author.
//!
//!

pub use crate::test_harness::prelude::*;
pub use assert2::assert;

/// This should produce the definitive collection of tests,
/// with no conditional test inclusion - all filtering must be done by the harness.
pub fn test_suite() -> Vec<Test<'static>> {
    [v0none(
        "ChainHead with no authorization token",
        [Tag::SchemaCoverage],
        |client| {
            let tipset = client.Filecoin_ChainHead()?;
            assert!(tipset.height >= 0);
            Ok(())
        },
    )]
    .into()
}

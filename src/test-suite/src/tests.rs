//! Collection of tests against an RPC node.
//!
//! # Overview
//! Tests are functions created using functions like [`v0read`].
//! The test function will receive, in the case of the above,
//! an RPC client with a `read` authorization token which can make calls against
//! a v0 RPC API.
//!
//! Tests are free to [`panic!`], [`assert!`], [`fail!`], or [propogate errors](crate::harness::Context).

pub use crate::harness::prelude::*;
pub use assert2::assert;

use num_bigint::BigUint;
use num_traits::Num;

fn is_hex_string(s: &str) -> bool {
    let s = s.strip_prefix("0x").or(s.strip_prefix("0X")).unwrap_or(s);
    BigUint::from_str_radix(s, 16).is_ok()
}

/// This should produce the definitive collection of tests,
/// with no conditional test inclusion - all filtering must be done by the harness.
pub fn all() -> Vec<Test<'static>> {
    [
        v0::none(
            "ChainHead with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let tipset = client.Filecoin_ChainHead()?;
                assert!(tipset.height >= 0);
                client.log("this is a log message");
                Ok(())
            },
        ),
        v1::none(
            "EthBlockNumber with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let number = client.Filecoin_EthBlockNumber()?;
                assert!(
                    is_hex_string(&number),
                    "a block number should be an hexadecimal string"
                );
                Ok(())
            },
        ),
    ]
    .into()
}

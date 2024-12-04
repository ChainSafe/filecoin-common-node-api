//! Collection of tests against an RPC node.
//!
//! # Overview
//! Tests are functions created using functions like [`v0read`].
//! The test function will receive, in the case of the above,
//! an RPC client with a `read` authorization token which can make calls against
//! a v0 RPC API.
//!
//! Tests are free to [`panic!`], [`assert!`], [`fail!`], or [propogate errors](crate::harness::Context).

use std::str::FromStr;

pub use crate::harness::prelude::*;
pub use assert2::assert;

use bindings::v1::{
    BlockHash, BlockNumber, BlockNumberOrHash, EthAddress, EthBigInt, EthBytes, EthCallMessage,
    EthHash, EthInt64, EthUint64, FilterId,
};
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
                // TODO(elmattic): use pattern instead
                assert!(
                    is_hex_string(&number),
                    "a block number should be an hexadecimal string"
                );
                Ok(())
            },
        ),
        // v1::none(
        //     "EthBlockNumber with no authorization token",
        //     [Tag::SchemaCoverage],
        //     |client| {
        //         let block = client.Filecoin_EthGetBlockByNumber(
        //             &BlockNumberOrHash {
        //                 subtype_0: None,
        //                 subtype_1: None,
        //                 subtype_2: None,
        //                 subtype_3: Some(BlockNumber {
        //                     block_number: EthInt64("0x1FFBDF".into()),
        //                 }),
        //                 subtype_4: None,
        //             },
        //             true,
        //         )?;
        //         Ok(())
        //     },
        // ),
        v1::none(
            "EthAccounts with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _accounts = client.Filecoin_EthAccounts()?;
                Ok(())
            },
        ),
        v1::none(
            "EthChainId with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _id = client.Filecoin_EthChainId()?;
                Ok(())
            },
        ),
        v1::none(
            "EthGasPrice with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _price = client.Filecoin_EthGasPrice()?;
                Ok(())
            },
        ),
        v1::none(
            "EthProtocolVersion with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _version = client.Filecoin_EthProtocolVersion()?;
                Ok(())
            },
        ),
        v1::none(
            "EthNewPendingTransactionFilter with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _version = client.Filecoin_EthNewPendingTransactionFilter()?;
                Ok(())
            },
        ),
        v1::none(
            "EthNewBlockFilter with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _vfilter = client.Filecoin_EthNewBlockFilter()?;
                Ok(())
            },
        ),
        v1::none(
            "EthAddressToFilecoinAddress with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _address = client.Filecoin_EthAddressToFilecoinAddress(&EthAddress::from_str(
                    "0xff38c072f286e3b20b3954ca9f99c05fbecc64aa",
                )
                .unwrap())?;
                Ok(())
            },
        ),
        v1::none(
            "EthSyncing with no authorization token",
            [Tag::SchemaCoverage],
            |client| {
                let _result = client.Filecoin_EthSyncing()?;
                Ok(())
            },
        ),
        // v1::none(
        //     "EthGetBalance with no authorization token and subtype_1",
        //     [Tag::SchemaCoverage],
        //     |client| {
        //         let _balance = client.Filecoin_EthGetBalance(
        //             &EthAddress::from_str("0xff38c072f286e3b20b3954ca9f99c05fbecc64aa").unwrap(),
        //             &BlockNumberOrHash {
        //                 subtype_0: None,
        //                 subtype_1: Some(EthInt64("0x1FFBDF".into())),
        //                 subtype_2: None,
        //                 subtype_3: None,
        //                 subtype_4: None,
        //             },
        //         )?;
        //         Ok(())
        //     },
        // ),
        // v1::none(
        //     "EthGetBalance with no authorization token and subtype_2",
        //     [Tag::SchemaCoverage],
        //     |client| {
        //         let _balance = client.Filecoin_EthGetBalance(
        //             &EthAddress::from_str("0xff38c072f286e3b20b3954ca9f99c05fbecc64aa").unwrap(),
        //             &BlockNumberOrHash {
        //                 subtype_0: None,
        //                 subtype_1: None,
        //                 subtype_2: Some(EthHash("0xabe09cdb5df97dfe16510f1714282fb16d66147ba21378f8bb1e9a9a52d38c98".into())),
        //                 subtype_3: None,
        //                 subtype_4: None,
        //             },
        //         )?;
        //         Ok(())
        //     },
        // ),
        v1::none(
            "EthGetBalance with no authorization token and subtype_3",
            [Tag::SchemaCoverage],
            |client| {
                let _balance = client.Filecoin_EthGetBalance(
                    &EthAddress::from_str("0x6cb414224f0b91de5c3b616e700e34a5172c149f").unwrap(),
                    &BlockNumberOrHash {
                        subtype_0: None,
                        subtype_1: None,
                        subtype_2: None,
                        subtype_3: Some(BlockNumber {
                            block_number: EthInt64("0x219153".into()),
                        }),
                        subtype_4: None,
                    },
                )?;
                Ok(())
            },
        ),
        v1::none(
            "EthGetBalance with no authorization token and subtype_4",
            [Tag::SchemaCoverage],
            |client| {
                let _balance = client.Filecoin_EthGetBalance(
                    &EthAddress::from_str("0x6cb414224f0b91de5c3b616e700e34a5172c149f").unwrap(),
                    &BlockNumberOrHash {
                        subtype_0: None,
                        subtype_1: None,
                        subtype_2: None,
                        subtype_3: None,
                        subtype_4: Some(BlockHash {
                            block_hash: EthHash("0x711da3a3ebc8fdfea3427d0d7b14e55b94f354baf33a93adca658b28e333f133".into()),
                            require_canonical: false,
                        }),
                    },
                )?;
                Ok(())
            },
        ),
        // v1::none(
        //     "EthCall with no authorization token",
        //     [Tag::SchemaCoverage],
        //     |client| {
        //         let balance = client.Filecoin_EthCall(&EthCallMessage {
        //             data:
        //                 EthBytes { 0: "0xf8b2cb4f000000000000000000000000CbfF24DED1CE6B53712078759233Ac8f91ea71B6"
        //                     .into() } ,
        //             from: None,
        //             gas: EthUint64 { 0: "0x0".into() },
        //             gas_price: EthBigInt { 0: "0x0".into() },
        //         to: Some(
        //                 EthAddress::from_str("0x0c1d86d34e469770339b53613f3a2343accd62cb").unwrap(),
        //             ),
        //             value: EthBigInt { 0: "0x0".into() },
        //         }, & BlockNumberOrHash {
        //                 subtype_0: Some("latest".into()),
        //                 subtype_1: None,
        //                 subtype_2: None,
        //                 subtype_3: None,
        //                 subtype_4: None,
        //             },
        //         )?;
        //         dbg!(&balance);
        //         Ok(())
        //     },
        // ),
    ]
    .into()
}

# Filecoin Common Node API Specification

This repo is an appendix to the Filecoin Common Node API FIP.

The main document is the [`spec.json`](./spec.json), which is a description of a
set of [JSON-RPC](https://www.jsonrpc.org/) methods as an [OpenRPC](https://spec.open-rpc.org/)
document.
You may [browse the spec on the OpenRPC playground](https://playground.open-rpc.org/?schemaUrl=https://github.com/ChainSafe/filecoin-common-node-api/raw/main/spec.json).

The [`tool`](./tool/) directory contains tooling for creating and interacting 
with the schema.
See [its readme](./tool/README.md) for information on the available commands.

To get started, you should [install rust](https://www.rust-lang.org/tools/install),
and run the following from the root of the repository.
Subcommands must follow the `--`.
```console
$ cargo run --manifest-path ./tool/Cargo.toml -- --help
```

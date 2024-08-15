# Filecoin Common Node API Specification

This repo is an appendix to the [Filecoin Common Node API FIP](https://github.com/filecoin-project/FIPs/pull/1027).

The main document is the [`spec.json`](./spec.json), which is a description of a
set of [JSON-RPC](https://www.jsonrpc.org/) methods as an [OpenRPC](https://spec.open-rpc.org/)
document.
You may [browse the spec on the OpenRPC playground](https://playground.open-rpc.org/?schemaUrl=https://github.com/ChainSafe/filecoin-common-node-api/raw/main/spec.json).

The [`rust`](./rust/) directory contains tooling for creating, interacting with,
and testing against the schema.

To get started, you should [install rust](https://www.rust-lang.org/tools/install),
and run the any of the following commands from the root of the repository.
Note that subcommands must follow the `--` at the command line.

List the available tests
```console
$ cargo run --manifest-path ./rust/Cargo.toml --package test-suite -- list
```

Capture JSON-RPC calls
```console
$ cargo run --manifest-path ./rust/Cargo.toml --package tool -- json-rpc capture --help
```

Validate the spec, and perhaps captured calls
```console
$ cargo run --manifest-path ./rust/Cargo.toml --package tool -- openrpc validate --help
```

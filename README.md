# Filecoin Common Node API Specification

[![codecov](https://codecov.io/gh/ChainSafe/filecoin-common-node-api/graph/badge.svg?token=15C2O7Z4G6)](https://codecov.io/gh/ChainSafe/filecoin-common-node-api)

This repo is an appendix to the [Filecoin Common Node API FIP](https://github.com/filecoin-project/FIPs/pull/1027).

# Spec

The main document is the [`spec.json`](./spec.json), which is a description of a
set of [JSON-RPC](https://www.jsonrpc.org/) methods as an [OpenRPC](https://spec.open-rpc.org/)
document.
You may [browse the spec on the OpenRPC playground](https://playground.open-rpc.org/?schemaUrl=https://github.com/ChainSafe/filecoin-common-node-api/raw/main/spec.json).

# Tooling

This repo also contains tooling for creating, interacting with,
and testing against the schema.

- [`cna-tool`](src/tool/README.md)
- [`cna-test-suite`](src/test-suite/README.md)
- [`cna-util`](src/test-suite/README.md)

## Setup
To get started, you should [install rust](https://www.rust-lang.org/tools/install).

You can compile the utilities with:
```
make build
```

Or, install them with:
```
make install
```
## Example Commands

List the available tests
```console
$ cna-test-suite list
```

Capture JSON-RPC calls
```console
$ cna-tool json-rpc capture --help
```

Validate the spec, and perhaps captured calls
```console
$ cna-tool openrpc validate --help
```

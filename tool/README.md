# `filecoin-common-node-api-util`

```
Utilities for creating, interacting with, and testing against the Filecoin Common Node API

Usage: filecoin-common-node-api-util <COMMAND>

Commands:
  openrpc   Subommands related to processing OpenRPC documents
  csv2json  Interpret stdin as a `delimiter`-separated series of lines, with a header, and print JSON
  json-rpc  Interact with JSON-RPC endpoints

```
## `filecoin-common-node-api-util` `openrpc`

```
Subommands related to processing OpenRPC documents

Usage: openrpc <COMMAND>

Commands:
  validate  Performs validation of the spec, including FIP-specific validation
  select    Interpret `select` as a json document of methods to include in `openrpc`

```
### `filecoin-common-node-api-util` `openrpc` `validate`

```
Performs validation of the spec, including FIP-specific validation.

Errors are emitted to stderr.

If stdin is received (and is not a terminal), it will be interpreted as concatenated JSON summaries of JSON-RPC exchanges (as output by the `json-rpc capture` command).

Each exchange will be validated against the spec.

On EOF, a summary table of `count` and `method` exchanges is printed to stdout.

Usage: validate <SPEC>

Arguments:
  <SPEC>

```
### `filecoin-common-node-api-util` `openrpc` `select`

```
Interpret `select` as a json document of methods to include in `openrpc`.

A new schema with only the selected methods is printed to stdout.

Usage: select [OPTIONS] <OPENRPC> <SELECT>

Arguments:
  <OPENRPC>
          

  <SELECT>
          

Options:
      --overwrite-title <OVERWRITE_TITLE>
          Specify a new title for the schema

      --overwrite-version <OVERWRITE_VERSION>
          Specify a new version for the schema

```
## `filecoin-common-node-api-util` `csv2json`

```
Interpret stdin as a `delimiter`-separated series of lines, with a header, and print JSON

Usage: csv2json [OPTIONS]

Options:
  -d, --delimiter <DELIMITER>
          [default: "\t"]

```
## `filecoin-common-node-api-util` `json-rpc`

```
Interact with JSON-RPC endpoints

Usage: json-rpc <COMMAND>

Commands:
  capture  Start a HTTP server, forwarding all requests to a single URI

```
### `filecoin-common-node-api-util` `json-rpc` `capture`

```
Start a HTTP server, forwarding all requests to a single URI.

Ctrl+C will request a graceful shutdown.

For HTTP exchanges whose bodies can be parsed as a singel JSON-RPC v2 method call, print a summary as a JSON line to stdout.

The summary includes only the method name, params, and response.

This does NOT validate adherence to the JSON-RPC protocol.

This is NOT robust to malice, and should only be run in trusted environments.

Usage: capture --local <LOCAL> --remote <REMOTE>

Options:
      --local <LOCAL>
          The local socket address to bind to

      --remote <REMOTE>
          The remote URI to forward requests to

```
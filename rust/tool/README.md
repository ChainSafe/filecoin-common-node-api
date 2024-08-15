# `tool`

```
Utilities for creating, interacting with, and testing against the Filecoin Common Node API

Usage: tool <COMMAND>

Commands:
  openrpc   Subcommands related to processing OpenRPC documents
  csv2json  Interpret stdin as a `delimiter`-separated series of lines, with a header, and print JSON
  json-rpc  Subcommands for interacting with JSON-RPC endpoints

```
## `tool` `openrpc`

```
Subcommands related to processing OpenRPC documents

Usage: openrpc <COMMAND>

Commands:
  validate  Performs validation of the spec, including FIP-specific validation
  select    Interpret `select` as a json document of methods to include in `openrpc`
  generate  Read an OpenRPC specification from stdin, and print Rust code for a client trait

```
### `tool` `openrpc` `validate`

```
Performs validation of the spec, including FIP-specific validation.

Errors are emitted to stderr.

If stdin is received (and is not a terminal), it will be interpreted as concatenated JSON summaries of JSON-RPC dialogues (as output by the `json-rpc capture` command).

Each dialogue will be validated against the spec.

On EOF, a summary table of passing `count` and `method` dialogues is printed to stdout.

If there is only a single dialogue, and it fails to validate, more detailed errors will be emitted.

Usage: validate <SPEC>

Arguments:
  <SPEC>

```
### `tool` `openrpc` `select`

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
### `tool` `openrpc` `generate`

```
Read an OpenRPC specification from stdin, and print Rust code for a client trait

Usage: generate <TRAIT_NAME>

Arguments:
  <TRAIT_NAME>

```
## `tool` `csv2json`

```
Interpret stdin as a `delimiter`-separated series of lines, with a header, and print JSON

Usage: csv2json [OPTIONS]

Options:
  -d, --delimiter <DELIMITER>
          [default: "\t"]

```
## `tool` `json-rpc`

```
Subcommands for interacting with JSON-RPC endpoints

Usage: json-rpc <COMMAND>

Commands:
  capture  Start a HTTP server, forwarding all requests to a single URI
  play     Receive's stdin's concatenated JSON summaries of JSON-RPC dialogue (as output by the `json-rpc capture` command)

```
### `tool` `json-rpc` `capture`

```
Start a HTTP server, forwarding all requests to a single URI.

Ctrl+C will request a graceful shutdown.

For HTTP dialogue whose bodies can be parsed as a single JSON-RPC v2 method call, print a summary as a JSON line to stdout.

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
### `tool` `json-rpc` `play`

```
Receive's stdin's concatenated JSON summaries of JSON-RPC dialogue (as output by the `json-rpc capture` command).

Each request in the exchange is send via HTTP POST to `remote`, and the dialogue is printed to stdout.

All requests are sent with an `id` (i.e not as a JSON-RPC Notification).

This does NOT validate adherence to the JSON-RPC protocol.

Usage: play [OPTIONS] --remote <REMOTE>

Options:
      --header <HEADER>
          Additional headers to append to every request.
          
          By default, `Content-Type` and `User-Agent` headers are set.

      --remote <REMOTE>
          The host to send JSON-RPC requests to

      --keep-going
          Don't short-circuit on the first HTTP/serialization failure

```
# `test-suite`

```
Filecoin RPC test suite

Usage: test-suite <COMMAND>

Commands:
  list  Print each test as a line of JSON to stdout
  run   Run the tests, loading the given config file

```
## `test-suite` `list`

```
Print each test as a line of JSON to stdout

Usage: list

```
## `test-suite` `run`

```
Run the tests, loading the given config file

Usage: run <CONFIG> [INCLUDE]...

Arguments:
  <CONFIG>
          The config file should match the schema in the repository.
          
          Tests will only run if the required config is available.

  [INCLUDE]...
          If supplied, only run tests with this name

```
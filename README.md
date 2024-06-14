# rgen

Generate cryptographically secure strings, numbers and more right from your terminal.

## Installation

```sh
cargo install rgen-cli
```

To compile from source instead [please read this guide](BUILDING.md)

## Usage

```text
Usage: rgen [OPTIONS] <COMMAND>

Commands:
  string
    -l, --length <LENGTH>  [default: 20]
    -t, --type <type>      [default: ascii] [possible values: ascii, letters, numbers, extended, hex]

  number <MIN> <MAX>

  boolean
    -t, --type <type>  [default: true-false] [possible values: true-false, yes-no, numeric]

  help     Print help or the help of the given subcommand(s)

Options:
  -c, --count <COUNT>  [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

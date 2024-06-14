# Compiling from source

## Prerequisites

You need the Rust Toolchain installed:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Cloning

First, clone the git repository:

```sh
git clone https://github.com/sinjs/rgen.git
```

## Installing

Then, you can compile and install the binary into PATH:

```sh
cargo install --path .
rgen -V # rgen-cli 0.1.0
```

## Development

While developing, you can run the project:

```sh
$ cargo run -- -V # All command line arguments go after the `--`
rgen-cli 0.1.0
```

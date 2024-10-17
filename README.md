# cargo-brief

[![Crates.io](https://img.shields.io/crates/v/cargo-brief)](https://crates.io/crates/cargo-brief)
[![Github actions](https://github.com/sanpii/cargo-brief/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/cargo-brief/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Build Status](https://gitlab.com/sanpi/cargo-brief/badges/main/pipeline.svg)](https://gitlab.com/sanpi/cargo-brief/commits/main)

Display a brief summary of cargo dependencies.

> [!WARNING]
> This plugin is unmaintened, use `cargo info` instead

## Install

```
cargo install cargo-brief
```

## Usage

```
$ cargo brief --help
cargo-brief 0.1.0

USAGE:
    cargo brief [FLAGS] [OPTIONS] [package]

FLAGS:
    -h, --help         Prints help information
        --no-dev
    -r, --recursive
    -V, --version      Prints version information

OPTIONS:
        --manifest-path <manifest-path>     [default: ./Cargo.toml]

ARGS:
    <package>     [default: *]
```

Display direct depedencies short summary:

```
$ cargo brief
ansi_term       0.12.1  Library for ANSI terminal colours and styles (bold, underline)
cargo_metadata  0.12.1  structured access to the output of `cargo metadata`
structopt       0.3.21  Parse command line argument by defining a struct.
tabwriter       1.2.1   Elastic tabstops.
thiserror       1.0.22  derive(Error)
wildmatch       1.0.12  Simple string matching  with questionmark and star wildcard operator.
```

Display long summary for one depedency:

```
$ cargo brief structopt
name        : structopt
descrip.    : Parse command line argument by defining a struct.
keywords    : clap, cli, derive, docopt
categories  : command-line-interface
version     : 0.3.21
license     : Apache-2.0 OR MIT
homepage    :
repository  : https://github.com/TeXitoi/structopt
features    : no_cargo, yaml, debug, suggestions, doc, color, default, paw, lints, wrap_help
```

The `package` argument supports wirdcard:

```
$ cargo brief serde*
serde       1.0.117  A generic serialization/deserialization framework
serde_json  1.0.59   A JSON serialization file format
```

You can use the `--recursive` option to display depedencies of all crates in the
workspace:

```
$ cargo brief --recursive
# elephantry 1.1.1 (path+file:///home/sanpi/projects/elephantry/elephantry/core)

async-std          1.6.5   Async version of the Rust standard library
byteorder          1.3.4   Library for reading/writing numbers in big-endian and little-endian.
bytes              0.6.0   Types and traits for working with bytes
elephantry-derive  1.1.1   Macro implementation of #[derive(Entity)]
lazy_static        1.4.0   A macro for declaring lazily evaluated statics in Rust.
libpq              1.0.0   Safe binding for libpq
log                0.4.11  A lightweight logging facade for Rust
pretty_env_logger  0.4.0   a visually pretty env_logger
regex              1.4.1   An implementation of regular expressions for Rust. This implementation uses…
serde_json         1.0.59  A JSON serialization file format
tuple_len          1.0.0   macro to get the number of elements in a tuple
uuid               0.8.1   A library to generate and parse UUIDs.

# elephantry-derive 1.1.1 (path+file:///home/sanpi/projects/elephantry/elephantry/derive)

quote  1.0.7   Quasi-quoting macro quote!(...)
syn    1.0.45  Parser for Rust source code

# elephantry-cli 1.1.1 (path+file:///home/sanpi/projects/elephantry/elephantry/cli)

case        1.0.0   A set of letter case string helpers
dotenv      0.15.0  A `dotenv` implementation for Rust
elephantry  1.1.1   Object model manager for PostgreSQL
structopt   0.3.20  Parse command line argument by defining a struct.
term-table  1.3.0   Tables for CLI apps
```

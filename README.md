# cargo-info

[![Crates.io](https://img.shields.io/crates/v/cargo-info)](https://crates.io/crates/cargo-info)
[![Github actions](https://github.com/sanpii/cargo-info/workflows/.github/workflows/ci.yml/badge.svg)](https://github.com/sanpii/cargo-info/actions?query=workflow%3A.github%2Fworkflows%2Fci.yml)
[![Build Status](https://gitlab.com/sanpi/cargo-info/badges/master/pipeline.svg)](https://gitlab.com/sanpi/cargo-info/commits/master)

## Install

```
cargo install cargo-info
```

## Usage

```
$ cargo info --help
cargo-info 0.1.0

USAGE:
    cargo info [FLAGS] [OPTIONS] [package]

FLAGS:
    -h, --help       Prints help information
        --no-dev
    -V, --version    Prints version information

OPTIONS:
        --manifest-path <manifest-path>     [default: ./Cargo.toml]

ARGS:
    <package>     [default: *]
```

Display direct depedencies short summary:

```
$ cargo info
ansi_term       0.12.1  Library for ANSI terminal colours and styles (bold, underline)
cargo_metadata  0.12.1  structured access to the output of `cargo metadata`
structopt       0.3.21  Parse command line argument by defining a struct.
tabwriter       1.2.1   Elastic tabstops.
thiserror       1.0.22  derive(Error)
wildmatch       1.0.12  Simple string matching  with questionmark and star wildcard operator.
```

Display long summary for one depedency:

```
$ cargo info structopt
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
$ cg info serde*
serde       1.0.117  A generic serialization/deserialization framework
serde_json  1.0.59   A JSON serialization file format
```

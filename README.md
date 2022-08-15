# brainfuck-rs

Very fast optimizing brainfuck intepreter written in Rust

## Features

- constant folding
- run-length encoding
- optimizing away loops with balanced `<`s and `>`s
- optimizing unnested loops
- optimizing common idioms - eg. clear loops (`[-]`)

## Install

### Manual

Compile code with cargo (get from [rustup](https://rustup.rs)):

#### *Note: requires nightly toolchain: `rustup toolchain install nightly`*

```sh
cargo +nightly build --release
```

The binary is then in `target/release/`

### From release

Grab a binary from the latest GitHub release

## Usage

```txt
bfrs [OPTIONS] [FILE]

    <FILE>
            Brainfuck file to interpret. Leave empty to read from stdin.

OPTIONS:
    -h, --help
            Print help information

    -m, --memory-size <MEMORY_SIZE>
            Starting memory size in bytes
            Accepts suffixes B, k, M, G. Default is 256B.

    -V, --version
            Print version information
```

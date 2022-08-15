# brainfuck-rs

Very fast optimizing brainfuck intepreter written in Rust

## Features

- constant folding
- running multiple simple instructions at once
- optimizing away loops with balanced `<`s and `>`s
- optimizing unnested loops

## Install

### Manual

Compile code with cargo (get from [rustup](https://rustup.rs)):

```sh
cargo build --release
```

The binary is in `target/release/`

### From release

Grab a binary from the latest release

## Usage

Read from file:

```sh
bfrs example/mandel.bf
```

or from stdin:

```sh
cat example/mandel.bf | bfrs
# or
bfrs < example/mandel.bf
```

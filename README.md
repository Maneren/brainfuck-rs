# brainfuck-rs

Very fast optimizing brainfuck intepreter written in Rust

## features

- constant folding
- running multiple simple instructions at once
- optimizing away loops with balanced `<`s and `>`s
- optimizing unnested loops

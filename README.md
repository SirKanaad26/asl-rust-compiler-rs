# The ASL to Rust Compiler rewritten in Rust

## Setup

```bash
# Clone and enter
git clone <repo-url>
cd asl-rust-compiler-rs

# Build
cargo build
```

## Usage
```
cargo run -- <input.asl> <output.rs>
```

## Examples
```
# Basic register
cargo run -- examples/minimal.asl output/minimal.rs

# Register with bitfields
cargo run -- examples/register_bitfield.asl output/registers.rs
```

## Regenerate Parser (after grammar changes)
```
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -visitor -o src/parser grammar/asl.g4
mv src/parser/grammar/*.rs src/parser/
rm -rf src/parser/grammar/
```


# Reference Documentation
- https://github.com/rrevenantt/antlr4rust/
- https://docs.rs/antlr4rust/latest/antlr4rust/
- ANTLR Release Used: https://github.com/rrevenantt/antlr4rust/releases/tag/antlr4-4.8-2-Rust0.3.0-beta
# The ASL to Rust Compiler rewritten in Rust

## Setup

```bash
# Clone and enter
git clone https://github.com/SirKanaad26/asl-rust-compiler-rs.git
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

## Testing

```bash
# Run all integration tests
cargo test

# Run bitvec unit tests under nightly (requires rustup nightly toolchain)
# Tests BV-1 through BV-8: slice, concat, AslValue, mixed-type arithmetic, etc.
rustc +nightly --edition 2021 --crate-type lib --test src/runtime/test_bitvec.rs -o /tmp/bitvec_test
/tmp/bitvec_test
```

## Example: Tock AND
```bash
cargo run -- instructions examples/tock/tock_and.asl output/tock/tock_and.rs
rustup run nightly rustc --edition 2021 --out-dir /tmp output/tock/tock_and.rs && /tmp/tock_and
```

#### Sample:
```rust
fn main() {
    let bits: u64 = 0x40110000; // ANDS R1, R2 (Rdn=1, Rm=2)
    let mut cpu = CpuState::new();
    cpu.R[1] = 0xFF00FF00;
    cpu.R[2] = 0x0F0F0F0F;
    let enc = aarch32_AND_r_T1_A::decode(bits).unwrap();
    execute_aarch32_AND_r_A(&enc, &mut cpu);
    println!("--- Decoded fields ---");
    println!("  Rdn      = {}", enc.Rdn);
    println!("  Rm       = {}", enc.Rm);
    println!("  d        = {}", enc.d);
    println!("  n        = {}", enc.n);
    println!("  m        = {}", enc.m);
    println!("  setflags = {}", enc.setflags);
    println!("  shift_t  = {}", enc.shift_t);
    println!("  shift_n  = {}", enc.shift_n);
    println!("--- CPU state after execute ---");
    println!("  R[1] = {:#010x}  (expected 0x0f000f00)", cpu.R[1]);
    println!("  R[2] = {:#010x}", cpu.R[2]);
    println!("  N    = {}  (expected false)", cpu.N);
    println!("  Z    = {}  (expected false)", cpu.Z);
    println!("  C    = {}  (expected false)", cpu.C);
    println!("  V    = {}", cpu.V);
    assert_eq!(cpu.R[1], 0x0F000F00);
    assert_eq!(cpu.N, false);
    assert_eq!(cpu.Z, false);
    assert_eq!(cpu.C, false);
    println!("--- All assertions passed! ---");
}
```

#### Silence Warnings:
```rust
#![allow(incomplete_features, non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_mut, unused_imports, unused_assignments, non_camel_case_types, unused_parens)]
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
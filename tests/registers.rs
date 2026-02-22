mod common;
use common::run_compiler;

#[test]
fn register_bitfield() {
    let out = run_compiler("registers", "examples/defs/register_bitfield.asl");
    assert!(out.contains("pub struct"), "expected register struct:\n{out}");
}

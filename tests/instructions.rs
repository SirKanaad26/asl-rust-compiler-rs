mod common;
use common::run_compiler;

#[test]
fn instruction_simple() {
    let out = run_compiler("instructions", "examples/instruction_simple.asl");
    assert!(out.contains("pub struct TestEncoding"), "expected encoding struct:\n{out}");
    assert!(out.contains("pub fn decode(bits: u64) -> Option<Self>"), "expected decode fn:\n{out}");
    assert!(out.contains("pub fn execute_TestInstruction"), "expected execute fn:\n{out}");
    assert!(out.contains("fixed_mask"), "expected opcode mask check:\n{out}");
    // Decode block body is emitted
    assert!(out.contains("let mut d: i64 = UInt(Rd)"), "expected decode block stmt:\n{out}");
}

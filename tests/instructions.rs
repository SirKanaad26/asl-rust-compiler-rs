mod common;
use common::run_compiler;

#[test]
fn instruction_execute() {
    let out = run_compiler("instructions", "examples/instruction_execute.asl");
    assert!(out.contains("pub fn postdecode_AddInstruction()"), "expected postdecode fn:\n{out}");
    assert!(out.contains("assert!(d != 31)"), "expected postdecode body:\n{out}");
    assert!(out.contains("let mut result: i64 = d + n"), "expected execute body:\n{out}");
}

#[test]
fn instruction_guard() {
    let out = run_compiler("instructions", "examples/instruction_guard.asl");
    assert!(out.contains("if !(cond != 0b1111) { return None; }"), "expected guard check:\n{out}");
}

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

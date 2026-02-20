mod common;
use common::run_compiler;

#[test]
fn instruction_runtime_stubs() {
    let out = run_compiler("instructions", "examples/instruction_execute.asl");
    assert!(out.contains("pub fn UInt(x: u64) -> i64"), "expected UInt stub:\n{out}");
    assert!(out.contains("pub fn HaveFP16Ext() -> bool"), "expected HaveFP16Ext stub:\n{out}");
    assert!(out.contains("#![allow(non_snake_case"), "expected allow attr:\n{out}");
}

#[test]
fn instruction_execute() {
    let out = run_compiler("instructions", "examples/instruction_execute.asl");
    // Struct includes decode-computed vars
    assert!(out.contains("pub d: i64"), "expected decode var in struct:\n{out}");
    assert!(out.contains("Some(Self { Rd, Rn, d, n })"), "expected decode vars in Self:\n{out}");
    // Postdecode takes enc param and shadows fields
    assert!(out.contains("pub fn postdecode_AddInstruction(enc: &AddEncoding)"), "expected postdecode sig:\n{out}");
    assert!(out.contains("let d = enc.d"), "expected field shadow:\n{out}");
    assert!(out.contains("assert!(d != 31)"), "expected postdecode body:\n{out}");
    // Execute takes enc param; body uses bare names
    assert!(out.contains("pub fn execute_AddInstruction(enc: &AddEncoding)"), "expected execute sig:\n{out}");
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
    assert!(out.contains("pub fn execute_TestInstruction(enc: &TestEncoding)"), "expected execute fn:\n{out}");
    assert!(out.contains("fixed_mask"), "expected opcode mask check:\n{out}");
    // Decode block body is emitted
    assert!(out.contains("let mut d: i64 = UInt(Rd)"), "expected decode block stmt:\n{out}");
}

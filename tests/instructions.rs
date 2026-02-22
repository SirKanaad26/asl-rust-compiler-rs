mod common;
use common::run_compiler;

#[test]
fn instruction_runtime_stubs() {
    let out = run_compiler("instructions", "examples/instr/instruction_execute.asl");
    // Runtime is now in a separate module — check for module declaration and import
    assert!(out.contains("mod runtime;"), "expected mod runtime:\n{out}");
    assert!(out.contains("use runtime::*;"), "expected use runtime::*:\n{out}");
    assert!(out.contains("use bitvec::{BitVec, AslValue}"), "expected bitvec import:\n{out}");
    assert!(out.contains("non_snake_case") && out.contains("#![allow("), "expected allow attr:\n{out}");
    // Inline stub definitions must NOT appear in the generated file (they live in runtime.rs)
    assert!(!out.contains("pub fn UInt(x: impl AslValue)"), "UInt stub should be in runtime.rs, not inlined:\n{out}");
    assert!(!out.contains("pub struct CpuState"), "CpuState should be in runtime.rs, not inlined:\n{out}");
    // cpu-threading: helper functions must take cpu — old no-op stubs must not appear
    assert!(!out.contains("fn BXWritePC(_addr"), "BXWritePC stub should take cpu as first arg:\n{out}");
}

#[test]
fn instruction_execute() {
    let out = run_compiler("instructions", "examples/instr/instruction_execute.asl");
    // Struct includes decode-computed vars
    assert!(out.contains("pub d: i128"), "expected decode var in struct:\n{out}");
    assert!(out.contains("Some(Self { Rd, Rn, d, n })"), "expected decode vars in Self:\n{out}");
    // Postdecode takes enc param and shadows fields
    assert!(out.contains("pub fn postdecode_AddInstruction(enc: &AddEncoding)"), "expected postdecode sig:\n{out}");
    assert!(out.contains("let d = enc.d"), "expected field shadow:\n{out}");
    assert!(out.contains("assert!(d != 31)"), "expected postdecode body:\n{out}");
    // Execute takes enc + cpu params; body uses bare names
    assert!(out.contains("pub fn execute_AddInstruction(enc: &AddEncoding, cpu: &mut CpuState)"), "expected execute sig:\n{out}");
    assert!(out.contains("let mut result: i128 = d + n"), "expected execute body:\n{out}");
}

#[test]
fn instruction_guard() {
    let out = run_compiler("instructions", "examples/instr/instruction_guard.asl");
    assert!(out.contains("if !(cond != 0b1111) { return None; }"), "expected guard check:\n{out}");
}

#[test]
fn instruction_unpredictable_unless() {
    let out = run_compiler("instructions", "examples/instr/instruction_unpredictable_unless.asl");
    assert!(out.contains("assert!((bits >> 6) & 1 == 0, \"UNPREDICTABLE\");"), "expected bit-6 check:\n{out}");
    assert!(out.contains("assert!((bits >> 11) & 1 == 1, \"UNPREDICTABLE\");"), "expected bit-11 check:\n{out}");
}

#[test]
fn instruction_implicit_decl() {
    let out = run_compiler("instructions", "examples/instr/instruction_implicit_decl.asl");
    // `result` is assigned but never declared — must be hoisted
    assert!(out.contains("let mut result = Default::default();"), "expected implicit decl:\n{out}");
    // Field shadows (d, n) must NOT be double-declared as implicit
    assert!(!out.contains("let mut d = Default::default();"), "d is a field shadow, should not be implicit:\n{out}");
    assert!(!out.contains("let mut n = Default::default();"), "n is a field shadow, should not be implicit:\n{out}");
}

#[test]
fn instruction_conditional() {
    let out = run_compiler("instructions", "examples/instr/instruction_conditional.asl");
    // check_condition lives in runtime.rs now — generated file uses it via `use runtime::*`
    assert!(out.contains("use runtime::*;"), "expected use runtime::*:\n{out}");
    // execute body must still be wrapped with check_condition
    assert!(out.contains("if check_condition(cpu) {"), "expected conditional wrapper in execute:\n{out}");
}

#[test]
fn instruction_simple() {
    let out = run_compiler("instructions", "examples/instr/instruction_simple.asl");
    assert!(out.contains("pub struct TestEncoding"), "expected encoding struct:\n{out}");
    assert!(out.contains("pub fn decode(bits: u64) -> Option<Self>"), "expected decode fn:\n{out}");
    assert!(out.contains("pub fn execute_TestInstruction(enc: &TestEncoding, cpu: &mut CpuState)"), "expected execute fn:\n{out}");
    assert!(out.contains("fixed_mask"), "expected opcode mask check:\n{out}");
    // Decode block body is emitted
    assert!(out.contains("let mut d: i128 = UInt(Rd)"), "expected decode block stmt:\n{out}");
}

#[test]
fn bv6_coercions() {
    let out = run_compiler("instructions", "examples/expr/bv6_coercions.asl");
    // AslValue trait imported so stubs and generated code can use it
    assert!(out.contains("use bitvec::{BitVec, AslValue}"), "expected AslValue import:\n{out}");
    // bits(64) decode var wrapped with BitVec::from_asl — NOT a direct cast
    assert!(out.contains("BitVec::from_asl(Xreg(cpu, n))"), "expected from_asl coercion for Xreg read:\n{out}");
    // set_Xreg called with BitVec<64> value (impl AslValue accepts it)
    assert!(out.contains("set_Xreg(cpu, d, result)"), "expected set_Xreg call:\n{out}");
    // Struct uses integer (i128) fields for UInt-computed vars
    assert!(out.contains("pub d: i128"), "expected d: i128 in struct:\n{out}");
}

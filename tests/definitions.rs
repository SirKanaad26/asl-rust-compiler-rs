mod common;
use common::run_compiler;

// ── Types ──

#[test]
fn type_alias() {
    let out = run_compiler("definitions", "examples/type_alias.asl");
    assert!(out.contains("pub type"), "expected type alias, got:\n{out}");
}

#[test]
fn enum_def() {
    let out = run_compiler("definitions", "examples/enum.asl");
    assert!(out.contains("pub enum"), "expected enum, got:\n{out}");
}

#[test]
fn struct_def() {
    let out = run_compiler("definitions", "examples/struct.asl");
    assert!(out.contains("pub struct"), "expected struct, got:\n{out}");
}

#[test]
fn builtin_type() {
    let out = run_compiler("definitions", "examples/builtin_type.asl");
    assert!(out.contains("// builtin:"), "expected builtin comment, got:\n{out}");
}

#[test]
fn abstract_type() {
    let out = run_compiler("definitions", "examples/abstract_type.asl");
    assert!(out.contains("pub struct"), "expected struct decl, got:\n{out}");
}

// ── Type Mapping ──

#[test]
fn type_array() {
    let out = run_compiler("definitions", "examples/type_array.asl");
    assert!(out.contains("[i64; 7 - 0 + 1]"), "missing array type mapping:\n{out}");
}

// ── Variables ──

#[test]
fn variable() {
    let out = run_compiler("definitions", "examples/variable.asl");
    assert!(out.contains("pub static myVar: std::sync::Mutex<i64>"), "missing myVar:\n{out}");
    assert!(out.contains("pub static flag: std::sync::Mutex<bool>"), "missing flag:\n{out}");
    assert!(out.contains("pub static data: std::sync::Mutex<u32>"), "missing data:\n{out}");
}

#[test]
fn constant() {
    let out = run_compiler("definitions", "examples/constant.asl");
    assert!(out.contains("pub const MAX: i64 = 100;"), "missing MAX:\n{out}");
}

#[test]
fn array_def() {
    let out = run_compiler("definitions", "examples/array_def.asl");
    assert!(out.contains("Mutex<[i64; 11]>"), "missing arr[0..10]:\n{out}");
    assert!(out.contains("Mutex<[u32; 4]>"), "missing regs[0..3]:\n{out}");
    assert!(out.contains("Mutex<[bool; 8]>"), "missing flags[0..7]:\n{out}");
}

// ── Getters and Setters ──

#[test]
fn getter_setter() {
    let out = run_compiler("definitions", "examples/getter_setter.asl");
    // Simple getter
    assert!(out.contains("pub fn GetValue(&self) -> i64"), "missing simple getter:\n{out}");
    // Simple setter
    assert!(out.contains("pub fn set_SetValue(&mut self, val: i64)"), "missing simple setter:\n{out}");
    // Indexed getter
    assert!(out.contains("pub fn GetElement(&self, i: i64) -> i64"), "missing indexed getter:\n{out}");
    // Indexed setter
    assert!(out.contains("pub fn set_SetElement(&mut self, idx: i64, val: i64)"), "missing indexed setter:\n{out}");
}

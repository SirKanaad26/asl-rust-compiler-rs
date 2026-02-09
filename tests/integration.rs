use std::process::Command;

/// Run the compiler with given mode/input, return stdout as a string
fn run_compiler(mode: &str, input: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", mode, input, "/dev/stdout"])
        .output()
        .expect("failed to run compiler");
    String::from_utf8(output.stdout).unwrap()
}

// ── Definitions: Types ──

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

// ── Definitions: Variables ──

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

// ── Functions ──

#[test]
fn function_return_expr() {
    let out = run_compiler("definitions", "examples/function.asl");
    assert!(out.contains("pub fn Add(a: i64, b: i64) -> i64 {"), "missing Add signature:\n{out}");
    assert!(out.contains("return a + b;"), "missing return statement:\n{out}");
}

#[test]
fn procedure_with_assignments() {
    let out = run_compiler("definitions", "examples/function.asl");
    assert!(out.contains("pub fn Swap(x: i64, y: i64) {"), "missing Swap signature:\n{out}");
    assert!(out.contains("x = x + y;"), "missing assignment:\n{out}");
}

#[test]
fn declaration_only_function() {
    let out = run_compiler("definitions", "examples/function.asl");
    assert!(out.contains("pub fn Identity(x: i64) -> i64 {"), "missing Identity signature:\n{out}");
    assert!(out.contains("todo!()"), "missing todo!() placeholder:\n{out}");
}

// ── Registers ──

#[test]
fn register_bitfield() {
    let out = run_compiler("registers", "examples/register_bitfield.asl");
    assert!(out.contains("pub struct"), "expected register struct:\n{out}");
}

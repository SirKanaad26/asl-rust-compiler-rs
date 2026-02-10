use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Run the compiler with given mode/input, return generated Rust output as a string
fn run_compiler(mode: &str, input: &str) -> String {
    let binary = env!("CARGO_BIN_EXE_asl-rust-compiler-rs");
    let unique_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    let out_path = std::env::temp_dir().join(format!("asl_codegen_test_{nanos}_{unique_id}.rs"));
    let out_file = out_path.to_string_lossy().into_owned();

    let output = Command::new(binary)
        .args([mode, input, out_file.as_str()])
        .output()
        .expect("failed to run compiler binary");

    assert!(
        output.status.success(),
        "compiler failed for mode={mode}, input={input}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let generated = fs::read_to_string(&out_path)
        .unwrap_or_else(|e| panic!("failed to read generated file {}: {e}", out_path.display()));
    let _ = fs::remove_file(&out_path);
    generated
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
fn if_statement() {
    let out = run_compiler("definitions", "examples/stmt_if.asl");
    assert!(out.contains("pub fn TestIf(x: i64) -> i64 {"), "missing TestIf signature:\n{out}");
    assert!(out.contains("if x > 0 {"), "missing if branch:\n{out}");
    assert!(out.contains("} else if x == 0 {"), "missing elsif branch:\n{out}");
    assert!(out.contains("} else {"), "missing else branch:\n{out}");
    assert!(out.contains("return -1;"), "missing else body:\n{out}");
}

#[test]
fn assert_statement() {
    let out = run_compiler("definitions", "examples/stmt_assert.asl");
    assert!(out.contains("pub fn EnsurePositive(n: i64) -> i64 {"), "missing EnsurePositive signature:\n{out}");
    assert!(out.contains("assert!(n > 0);"), "missing assert statement:\n{out}");
    assert!(out.contains("return n;"), "missing return statement:\n{out}");
}

#[test]
fn for_statement() {
    let out = run_compiler("definitions", "examples/stmt_for.asl");
    assert!(out.contains("for i in 0..=n {"), "missing for loop:\n{out}");
    assert!(out.contains("n = n + i;"), "missing for body:\n{out}");
    assert!(out.contains("    return n;"), "return should be outside for loop:\n{out}");
}

#[test]
fn while_statement() {
    let out = run_compiler("definitions", "examples/stmt_while.asl");
    assert!(out.contains("while n > 0 {"), "missing while condition:\n{out}");
    assert!(out.contains("n = n - 1;"), "missing loop body:\n{out}");
}

#[test]
fn declaration_only_function() {
    let out = run_compiler("definitions", "examples/function.asl");
    assert!(out.contains("pub fn Identity(x: i64) -> i64 {"), "missing Identity signature:\n{out}");
    assert!(out.contains("todo!()"), "missing todo!() placeholder:\n{out}");
}

// ── Expressions ──

#[test]
fn expr_member_access() {
    let out = run_compiler("definitions", "examples/expr_member.asl");
    assert!(out.contains("a.x + b.x"), "missing member access expr:\n{out}");
}

// ── Registers ──

#[test]
fn register_bitfield() {
    let out = run_compiler("registers", "examples/register_bitfield.asl");
    assert!(out.contains("pub struct"), "expected register struct:\n{out}");
}

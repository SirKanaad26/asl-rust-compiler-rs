mod common;
use common::run_compiler;

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
fn repeat_until_statement() {
    let out = run_compiler("definitions", "examples/stmt_repeat.asl");
    assert!(out.contains("loop {"), "missing loop:\n{out}");
    assert!(out.contains("n = n - 1;"), "missing loop body:\n{out}");
    assert!(out.contains("if n == 0 { break; }"), "missing until break:\n{out}");
    assert!(out.contains("    return n;"), "return should be outside loop:\n{out}");
}

#[test]
fn case_statement() {
    let out = run_compiler("definitions", "examples/stmt_case.asl");
    assert!(out.contains("match opcode {"), "missing match:\n{out}");
    assert!(out.contains("0 => {"), "missing when 0 arm:\n{out}");
    assert!(out.contains("1 | 2 => {"), "missing when 1,2 arm:\n{out}");
    assert!(out.contains("_ => {"), "missing otherwise arm:\n{out}");
    assert!(out.contains("result = 100;"), "missing first arm body:\n{out}");
    assert!(out.contains("result = 999;"), "missing otherwise body:\n{out}");
    assert!(out.contains("    return result;"), "return should be outside match:\n{out}");
}

#[test]
fn stmt_stubs() {
    let out = run_compiler("definitions", "examples/stubs.asl");
    assert!(out.contains("panic!(\"UNPREDICTABLE\");"), "missing UNPREDICTABLE:\n{out}");
    assert!(out.contains("panic!(\"UNDEFINED\");"), "missing UNDEFINED:\n{out}");
    assert!(out.contains("panic!(\"IMPLEMENTATION_DEFINED\");"), "missing IMPLEMENTATION_DEFINED:\n{out}");
    assert!(out.contains("panic!(\"MyError\");"), "missing throw:\n{out}");
}

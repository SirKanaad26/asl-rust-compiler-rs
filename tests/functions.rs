mod common;
use common::run_compiler;

#[test]
fn function_return_expr() {
    let out = run_compiler("definitions", "examples/function.asl");
    assert!(out.contains("pub fn Add(a: i128, b: i128) -> i128 {"), "missing Add signature:\n{out}");
    assert!(out.contains("return a + b;"), "missing return statement:\n{out}");
}

#[test]
fn procedure_with_assignments() {
    let out = run_compiler("definitions", "examples/function.asl");
    assert!(out.contains("pub fn Swap(x: i128, y: i128) {"), "missing Swap signature:\n{out}");
    assert!(out.contains("x = x + y;"), "missing assignment:\n{out}");
}

#[test]
fn declaration_only_function() {
    let out = run_compiler("definitions", "examples/function.asl");
    assert!(out.contains("pub fn Identity(x: i128) -> i128 {"), "missing Identity signature:\n{out}");
    assert!(out.contains("todo!()"), "missing todo!() placeholder:\n{out}");
}

#[test]
fn inline_var_decl_and_bool_literals() {
    let out = run_compiler("definitions", "examples/leetcode_palindrome.asl");
    assert!(out.contains("let mut original: i128 = x;"), "missing var decl with init:\n{out}");
    assert!(out.contains("let mut reversed: i128 = 0;"), "missing var decl with zero init:\n{out}");
    assert!(out.contains("return false;"), "missing FALSE -> false mapping:\n{out}");
}

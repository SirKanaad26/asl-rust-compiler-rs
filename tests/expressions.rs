mod common;
use common::run_compiler;

#[test]
fn expr_member_access() {
    let out = run_compiler("definitions", "examples/expr_member.asl");
    assert!(out.contains("a.x + b.x"), "missing member access expr:\n{out}");
}

#[test]
fn expr_array_index() {
    let out = run_compiler("definitions", "examples/expr_index.asl");
    assert!(out.contains("arr[i]"), "missing array index expr:\n{out}");
}

#[test]
fn expr_bit_slice() {
    let out = run_compiler("definitions", "examples/expr_slice.asl");
    assert!(out.contains("((x >> 0) & ((1 << (7 - 0 + 1)) - 1))"), "missing range bit slice:\n{out}");
    assert!(out.contains("((x >> n) & 1)"), "missing single bit slice:\n{out}");
}

#[test]
fn expr_tuple() {
    let out = run_compiler("definitions", "examples/expr_tuple.asl");
    assert!(out.contains("(b, a)"), "missing tuple expr:\n{out}");
}

#[test]
fn expr_if() {
    let out = run_compiler("definitions", "examples/expr_if.asl");
    assert!(out.contains("if x > 0 { x } else { -x }"), "missing if expression:\n{out}");
}

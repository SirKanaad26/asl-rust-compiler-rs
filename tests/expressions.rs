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
    assert!(out.contains("x.slice::<7, 0>()"), "missing range bit slice:\n{out}");
    assert!(out.contains("x.bit(n as usize)"), "missing single bit slice:\n{out}");
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

#[test]
fn expr_concat() {
    let out = run_compiler("definitions", "examples/expr_concat.asl");
    assert!(out.contains("hi.concat(lo)"), "missing concat expression:\n{out}");
}

#[test]
fn expr_in_set() {
    let out = run_compiler("definitions", "examples/expr_in.asl");
    assert!(out.contains("[0, 1, 5].contains(&opcode)"), "missing IN set expr:\n{out}");
}

#[test]
fn expr_bin_and_mask_literals() {
    let out = run_compiler("definitions", "examples/expr_literals.asl");
    assert!(out.contains("0b10110011"), "missing binary literal:\n{out}");
    assert!(out.contains("todo!(/* mask: '1x1x0011' */)"), "missing mask literal:\n{out}");
}

#[test]
fn lval_tuple() {
    let out = run_compiler("definitions", "examples/lval_tuple.asl");
    assert!(out.contains("(a, b) = (b, a);"), "missing tuple lvalue:\n{out}");
}

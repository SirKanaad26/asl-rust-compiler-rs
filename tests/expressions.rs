mod common;
use common::run_compiler;

#[test]
fn expr_member_access() {
    let out = run_compiler("definitions", "examples/expr/expr_member.asl");
    assert!(out.contains("a.x + b.x"), "missing member access expr:\n{out}");
}

#[test]
fn expr_array_index() {
    let out = run_compiler("definitions", "examples/expr/expr_index.asl");
    assert!(out.contains("arr[i]"), "missing array index expr:\n{out}");
}

#[test]
fn expr_bit_slice() {
    let out = run_compiler("definitions", "examples/expr/expr_slice.asl");
    assert!(out.contains("x.slice::<7, 0>()"), "missing range bit slice:\n{out}");
    assert!(out.contains("asl_bit(x, n as usize)"), "missing single bit slice:\n{out}");
}

#[test]
fn expr_tuple() {
    let out = run_compiler("definitions", "examples/expr/expr_tuple.asl");
    assert!(out.contains("(b, a)"), "missing tuple expr:\n{out}");
}

#[test]
fn expr_if() {
    let out = run_compiler("definitions", "examples/expr/expr_if.asl");
    assert!(out.contains("if x > 0 { x } else { -x }"), "missing if expression:\n{out}");
}

#[test]
fn expr_concat() {
    let out = run_compiler("definitions", "examples/expr/expr_concat.asl");
    assert!(out.contains("hi.concat(lo)"), "missing concat expression:\n{out}");
}

#[test]
fn expr_in_set() {
    let out = run_compiler("definitions", "examples/expr/expr_in.asl");
    assert!(out.contains("[0, 1, 5].contains(&opcode)"), "missing IN set expr:\n{out}");
}

#[test]
fn expr_bin_and_mask_literals() {
    let out = run_compiler("definitions", "examples/expr/expr_literals.asl");
    assert!(out.contains("0b10110011"), "missing binary literal:\n{out}");
    assert!(out.contains("todo!(/* mask: '1x1x0011' */)"), "missing mask literal:\n{out}");
}

#[test]
fn lval_tuple() {
    let out = run_compiler("definitions", "examples/expr/lval_tuple.asl");
    assert!(out.contains("(a, b) = (b, a);"), "missing tuple lvalue:\n{out}");
}

// BV-9: IN mask → inline bitmask comparison
#[test]
fn expr_in_mask() {
    let out = run_compiler("definitions", "examples/expr/expr_in_mask.asl");
    // '0xx1': bit 3 fixed to 0, bit 0 fixed to 1 → mask=0x9, expected=0x1
    assert!(
        out.contains("AslValue::to_u128(x) & 0x9u128) == 0x1u128"),
        "expected inline mask comparison:\n{out}"
    );
}

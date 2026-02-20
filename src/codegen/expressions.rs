use std::rc::Rc;
use antlr_rust::tree::ParseTree;
use antlr_rust::token::Token;

use crate::parser::aslparser::*;

/// Generate a Rust expression string from an ASL expr node
pub fn generate_expr(expr: &Rc<ExprContextAll<'_>>) -> String {
    match expr.as_ref() {
        ExprContextAll::ExprVarRefContext(ctx) => {
            let name = ctx.qualId().unwrap().get_text().replace('.', "_");
            match name.as_str() {
                "TRUE" => "true".to_string(),
                "FALSE" => "false".to_string(),
                _ => name,
            }
        }
        ExprContextAll::ExprLitNatContext(ctx) => {
            ctx.NAT_LIT().unwrap().get_text()
        }
        ExprContextAll::ExprLitHexContext(ctx) => {
            ctx.HEX_LIT().unwrap().get_text()
        }
        ExprContextAll::ExprLitRealContext(ctx) => {
            ctx.REAL_LIT().unwrap().get_text()
        }
        ExprContextAll::ExprLitBinContext(ctx) => {
            let raw = ctx.BIN_LIT().unwrap().get_text();
            let bits = raw.trim_matches('\'').replace(' ', "");
            format!("0b{}", bits)
        }
        ExprContextAll::ExprLitMaskContext(ctx) => {
            let raw = ctx.MASK_LIT().unwrap().get_text();
            let mask = raw.trim_matches('\'').replace(' ', "");
            format!("todo!(/* mask: '{}' */)", mask)
        }
        ExprContextAll::ExprLitStringContext(ctx) => {
            ctx.STRING_LIT().unwrap().get_text()
        }
        ExprContextAll::ExprBinOpContext(ctx) => {
            let lhs = generate_expr(ctx.operand1.as_ref().unwrap());
            let rhs = generate_expr(ctx.operand2.as_ref().unwrap());
            let op_text = ctx.operator.as_ref().unwrap().get_text();
            match &*op_text {
                "++" | ":" => format!("{}.concat({})", lhs, rhs),
                "MOD" => format!("asl_mod({}, {})", lhs, rhs),
                _ => {
                    let op = map_binop(&op_text);
                    format!("{} {} {}", lhs, op, rhs)
                }
            }
        }
        ExprContextAll::ExprUnOpContext(ctx) => {
            let operand = generate_expr(&ctx.expr().unwrap());
            let op_token = ctx.operator.as_ref().unwrap();
            let op = map_unop(op_token.get_text());
            format!("{}{}", op, operand)
        }
        ExprContextAll::ExprParenContext(ctx) => {
            let inner = generate_expr(&ctx.expr().unwrap());
            format!("({})", inner)
        }
        ExprContextAll::ExprCallContext(ctx) => {
            let name = ctx.qualId().unwrap().get_text().replace('.', "_");
            let args: Vec<String> = ctx.exprCommaList0().unwrap()
                .expr_all()
                .iter()
                .map(|e| generate_expr(e))
                .collect();
            format!("{}({})", name, args.join(", "))
        }
        ExprContextAll::ExprIndexContext(ctx) => {
            let obj = generate_expr(&ctx.expr().unwrap());
            let slices: Vec<String> = ctx.sliceCommaList0().unwrap()
                .slice_all()
                .iter()
                .map(|s| generate_slice(s))
                .collect();
            // Map register array reads to CpuState accessor calls.
            // In ASL, X[n]/W[n] are always register accesses, not plain arrays.
            if slices.len() == 1 {
                match obj.as_str() {
                    "X" => return format!("Xreg(cpu, {})", slices[0]),
                    "W" => return format!("Wreg(cpu, {})", slices[0]),
                    "R" => return format!("Rreg(cpu, {})", slices[0]),
                    "S" => return format!("Sreg(cpu, {})", slices[0]),
                    "D" => return format!("Dreg(cpu, {})", slices[0]),
                    _ => {}
                }
            }
            format!("{}[{}]", obj, slices.join(", "))
        }
        ExprContextAll::ExprSliceContext(ctx) => {
            let obj = generate_expr(&ctx.expr().unwrap());
            let slices = ctx.sliceCommaList1().unwrap().slice_all();
            if slices.len() == 1 {
                generate_bit_slice(&obj, &slices[0])
            } else {
                format!("todo!(/* bit slice: {}<{}> */)", obj,
                    slices.iter().map(|s| s.get_text()).collect::<Vec<_>>().join(", "))
            }
        }
        ExprContextAll::ExprMemberContext(ctx) => {
            let obj = generate_expr(&ctx.expr().unwrap());
            let field = ctx.id().unwrap().get_text();
            if obj == "PSTATE" {
                format!("cpu.{}", field)
            } else {
                format!("{}.{}", obj, field)
            }
        }
        ExprContextAll::ExprTupleContext(ctx) => {
            let elems: Vec<String> = ctx.exprCommaList1().unwrap()
                .expr_all()
                .iter()
                .map(|e| generate_expr(e))
                .collect();
            format!("({})", elems.join(", "))
        }
        ExprContextAll::ExprUnknownContext(_) => {
            "Default::default()".to_string()
        }
        ExprContextAll::ExprImpDefContext(_) => {
            "panic!(\"IMPLEMENTATION_DEFINED\")".to_string()
        }
        ExprContextAll::ExprInSetContext(ctx) => {
            let val = generate_expr(&ctx.expr().unwrap());
            let elements = ctx.set().unwrap().setElement_all();
            let all_single = elements.iter().all(|el| {
                matches!(el.as_ref(), SetElementContextAll::SetElementSingleContext(_))
            });
            if all_single {
                let vals: Vec<String> = elements.iter().map(|el| {
                    if let SetElementContextAll::SetElementSingleContext(single) = el.as_ref() {
                        generate_expr(&single.expr().unwrap())
                    } else {
                        unreachable!()
                    }
                }).collect();
                format!("[{}].contains(&{})", vals.join(", "), val)
            } else {
                let checks: Vec<String> = elements.iter().map(|el| {
                    match el.as_ref() {
                        SetElementContextAll::SetElementSingleContext(single) => {
                            let elem = generate_expr(&single.expr().unwrap());
                            format!("{} == {}", val, elem)
                        }
                        SetElementContextAll::SetElementRangeContext(range) => {
                            let begin = generate_expr(range.begin.as_ref().unwrap());
                            let end = generate_expr(range.end.as_ref().unwrap());
                            format!("({}..={}).contains(&{})", begin, end, val)
                        }
                        _ => format!("todo!(/* set element */)")
                    }
                }).collect();
                format!("({})", checks.join(" || "))
            }
        }
        ExprContextAll::ExprInMaskContext(ctx) => {
            let val = generate_expr(&ctx.expr().unwrap());
            let mask = ctx.MASK_LIT().unwrap().get_text();
            format!("in_mask({}, {})", val, mask)
        }
        ExprContextAll::ExprIfContext(ctx) => {
            let cond = generate_expr(ctx.test.as_ref().unwrap());
            let then_expr = generate_expr(ctx.thenExpr.as_ref().unwrap());
            let else_expr = generate_expr(ctx.elseExpr.as_ref().unwrap());

            let mut result = format!("if {} {{ {} }}", cond, then_expr);

            for elsif in ctx.exprElsIf_all() {
                let elsif_cond = generate_expr(elsif.test.as_ref().unwrap());
                let elsif_result = generate_expr(elsif.result.as_ref().unwrap());
                result.push_str(&format!(" else if {} {{ {} }}", elsif_cond, elsif_result));
            }

            result.push_str(&format!(" else {{ {} }}", else_expr));
            result
        }
        _ => {
            format!("todo!(/* {} */)", expr.get_text())
        }
    }
}

/// Generate a Rust lvalue string from an ASL lValExpr node
pub fn generate_lval(lval: &Rc<LValExprContextAll<'_>>) -> String {
    match lval.as_ref() {
        LValExprContextAll::LValVarRefContext(ctx) => {
            ctx.qualId().unwrap().get_text().replace('.', "_")
        }
        LValExprContextAll::LValMemberContext(ctx) => {
            let obj = generate_lval(&ctx.lValExpr().unwrap());
            let field = ctx.id().unwrap().get_text();
            if obj == "PSTATE" {
                format!("cpu.{}", field)
            } else {
                format!("{}.{}", obj, field)
            }
        }
        LValExprContextAll::LValArrayIndexContext(ctx) => {
            let obj = generate_lval(&ctx.lValExpr().unwrap());
            let slices: Vec<String> = ctx.slice_all()
                .iter()
                .map(|s| s.get_text())
                .collect();
            format!("{}[{}]", obj, slices.join(", "))
        }
        LValExprContextAll::LValIgnoreContext(_) => {
            "_".to_string()
        }
        LValExprContextAll::LValTupleContext(ctx) => {
            let elems: Vec<String> = ctx.lValExpr_all()
                .iter()
                .map(|e| generate_lval(e))
                .collect();
            format!("({})", elems.join(", "))
        }
        LValExprContextAll::LValSliceOfContext(ctx) => {
            let obj = generate_lval(&ctx.lValExpr().unwrap());
            let slices = ctx.sliceCommaList1().unwrap().slice_all();
            if slices.len() == 1 {
                generate_lval_bit_slice(&obj, &slices[0])
            } else {
                format!("todo!(/* lval slice: {}<{}> */)", obj,
                    slices.iter().map(|s| s.get_text()).collect::<Vec<_>>().join(", "))
            }
        }
        LValExprContextAll::LValSliceContext(ctx) => {
            let elems: Vec<String> = ctx.lValExpr_all()
                .iter()
                .map(|e| generate_lval(e))
                .collect();
            format!("<{}>", elems.join(", "))
        }
        LValExprContextAll::LValMemberBitsContext(ctx) => {
            let obj = generate_lval(&ctx.lValExpr().unwrap());
            let fields: Vec<String> = ctx.identifierCommaList1().unwrap()
                .id_all()
                .iter()
                .map(|id| id.get_text())
                .collect();
            format!("{}.<{}>", obj, fields.join(", "))
        }
        _ => {
            format!("todo!(/* lval: {} */)", lval.get_text())
        }
    }
}

/// Map ASL binary operators to Rust equivalents
fn map_binop(op: &str) -> &str {
    match op {
        "+" => "+",
        "-" => "-",
        "*" => "*",
        "/" => "/",
        "^" => "^",
        "==" => "==",
        "!=" => "!=",
        ">" => ">",
        ">=" => ">=",
        "<" => "<",
        "<=" => "<=",
        "&&" => "&&",
        "||" => "||",
        "AND" => "&",
        "OR" => "|",
        "EOR" => "^",
        ">>" => ">>",
        "<<" => "<<",
        "DIV" => "/",
        other => other,
    }
}

/// Generate a Rust bit-extraction expression from an ASL bit slice.
///
/// Single bit `x<i>`    → `x.bit(i as usize)`  (returns bool)
/// Range `x<HI:LO>`     → `x.slice::<HI, LO>()` when HI/LO are integer literals
///                      → `x.slice_rt(hi, lo)`   when HI/LO are runtime expressions
/// Offset `x<base+:len>`→ `x.slice_rt(base, base + len - 1)` (always runtime)
fn generate_bit_slice(obj: &str, slice: &Rc<SliceContextAll<'_>>) -> String {
    match slice.as_ref() {
        SliceContextAll::SliceSingleContext(ctx) => {
            let bit = generate_expr(&ctx.expr().unwrap());
            format!("{}.bit({} as usize)", obj, bit)
        }
        SliceContextAll::SliceRangeContext(ctx) => {
            let hi_text = ctx.begin.as_ref().unwrap().get_text();
            let lo_text = ctx.end.as_ref().unwrap().get_text();
            // Use const-generic slice when both bounds are integer literals —
            // this preserves the width in the return type (BitVec<{HI-LO+1}>).
            if let (Ok(hi), Ok(lo)) = (hi_text.parse::<usize>(), lo_text.parse::<usize>()) {
                format!("{}.slice::<{}, {}>()", obj, hi, lo)
            } else {
                format!("{}.slice_rt({}, {})", obj, hi_text, lo_text)
            }
        }
        SliceContextAll::SliceOffsetContext(ctx) => {
            // x<base+:len>  →  bits [base, base+len-1]
            let base = ctx.sliceBase.as_ref().unwrap().get_text();
            let count = ctx.count.as_ref().unwrap().get_text();
            format!("{}.slice_rt({}, {} + {} - 1)", obj, base, base, count)
        }
        _ => {
            format!("todo!(/* bit slice: {}<{}> */)", obj, slice.get_text())
        }
    }
}

/// Generate a Rust expression from an ASL slice node
fn generate_slice(slice: &Rc<SliceContextAll<'_>>) -> String {
    match slice.as_ref() {
        SliceContextAll::SliceSingleContext(ctx) => {
            generate_expr(&ctx.expr().unwrap())
        }
        _ => {
            slice.get_text()
        }
    }
}

/// Generate a Rust lvalue bit-slice assignment target string.
/// NOTE: This is only reached when `generate_lval` is called directly on a
/// `LValSliceOfContext` node (e.g. nested lvalues). The primary assignment path
/// in `generate_inline_stmt` intercepts `LValSliceOfContext` and emits the full
/// `obj.set_slice(lo, hi, val as u128)` statement directly.
fn generate_lval_bit_slice(obj: &str, slice: &Rc<SliceContextAll<'_>>) -> String {
    match slice.as_ref() {
        SliceContextAll::SliceSingleContext(ctx) => {
            let bit = generate_expr(&ctx.expr().unwrap());
            // Placeholder: real write uses obj.set_slice(bit, bit, val as u128)
            format!("todo!(/* set bit {}<{}> */)", obj, bit)
        }
        SliceContextAll::SliceRangeContext(ctx) => {
            let hi = ctx.begin.as_ref().unwrap().get_text();
            let lo = ctx.end.as_ref().unwrap().get_text();
            // Placeholder: real write uses obj.set_slice(lo, hi, val as u128)
            format!("todo!(/* set slice {}<{}:{}> */)", obj, hi, lo)
        }
        _ => {
            format!("todo!(/* lval bit slice: {}<{}> */)", obj, slice.get_text())
        }
    }
}

/// Map ASL unary operators to Rust equivalents
fn map_unop(op: &str) -> &str {
    match op {
        "-" => "-",
        "!" => "!",
        "NOT" => "!",
        other => other,
    }
}

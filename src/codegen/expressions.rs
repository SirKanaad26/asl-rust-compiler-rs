use std::rc::Rc;
use antlr_rust::tree::ParseTree;
use antlr_rust::token::Token;

use crate::parser::aslparser::*;

/// Generate a Rust expression string from an ASL expr node
pub fn generate_expr(expr: &Rc<ExprContextAll<'_>>) -> String {
    match expr.as_ref() {
        ExprContextAll::ExprVarRefContext(ctx) => {
            ctx.qualId().unwrap().get_text()
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
        ExprContextAll::ExprLitStringContext(ctx) => {
            ctx.STRING_LIT().unwrap().get_text()
        }
        ExprContextAll::ExprBinOpContext(ctx) => {
            let lhs = generate_expr(ctx.operand1.as_ref().unwrap());
            let rhs = generate_expr(ctx.operand2.as_ref().unwrap());
            let op_token = ctx.operator.as_ref().unwrap();
            let op = map_binop(op_token.get_text());
            format!("{} {} {}", lhs, op, rhs)
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
            let name = ctx.qualId().unwrap().get_text();
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
            format!("{}.{}", obj, field)
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
            ctx.qualId().unwrap().get_text()
        }
        LValExprContextAll::LValMemberContext(ctx) => {
            let obj = generate_lval(&ctx.lValExpr().unwrap());
            let field = ctx.id().unwrap().get_text();
            format!("{}.{}", obj, field)
        }
        LValExprContextAll::LValArrayIndexContext(ctx) => {
            let obj = generate_lval(&ctx.lValExpr().unwrap());
            let slices: Vec<String> = ctx.slice_all()
                .iter()
                .map(|s| s.get_text())
                .collect();
            format!("{}[{}]", obj, slices.join(", "))
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
        "MOD" => "%",
        other => other,
    }
}

/// Generate a Rust bit-extraction expression from an ASL bit slice
fn generate_bit_slice(obj: &str, slice: &Rc<SliceContextAll<'_>>) -> String {
    match slice.as_ref() {
        SliceContextAll::SliceSingleContext(ctx) => {
            let bit = generate_expr(&ctx.expr().unwrap());
            format!("(({} >> {}) & 1)", obj, bit)
        }
        SliceContextAll::SliceRangeContext(ctx) => {
            let hi = ctx.begin.as_ref().unwrap().get_text();
            let lo = ctx.end.as_ref().unwrap().get_text();
            format!("(({} >> {}) & ((1 << ({} - {} + 1)) - 1))", obj, lo, hi, lo)
        }
        SliceContextAll::SliceOffsetContext(ctx) => {
            let base = ctx.sliceBase.as_ref().unwrap().get_text();
            let count = ctx.count.as_ref().unwrap().get_text();
            format!("(({} >> {}) & ((1 << {}) - 1))", obj, base, count)
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

/// Map ASL unary operators to Rust equivalents
fn map_unop(op: &str) -> &str {
    match op {
        "-" => "-",
        "!" => "!",
        "NOT" => "!",
        other => other,
    }
}

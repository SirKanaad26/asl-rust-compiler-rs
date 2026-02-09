use std::rc::Rc;
use antlr_rust::tree::ParseTree;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::expressions::{generate_expr, generate_lval};
use crate::parser::aslparser::*;

/// Generate Rust code for a single stmt node
pub fn generate_stmt(emitter: &mut CodeEmitter, stmt: &Rc<StmtContextAll<'_>>) {
    match stmt.as_ref() {
        StmtContextAll::StmtsInlineContext(ctx) => {
            if let Some(inline) = ctx.inlineStmt() {
                generate_inline_stmt(emitter, &inline);
            }
        }
        _ => {
            emitter.emit(&format!("// TODO stmt: {}", stmt.get_text()));
        }
    }
}

/// Generate Rust code for an inline statement
fn generate_inline_stmt(emitter: &mut CodeEmitter, stmt: &Rc<InlineStmtContextAll<'_>>) {
    match stmt.as_ref() {
        InlineStmtContextAll::StmtReturnContext(ctx) => {
            match ctx.expr() {
                Some(expr) => {
                    let val = generate_expr(&expr);
                    emitter.emit(&format!("return {};", val));
                }
                None => {
                    emitter.emit("return;");
                }
            }
        }
        InlineStmtContextAll::StmtAssignContext(ctx) => {
            let lhs = generate_lval(&ctx.lValExpr().unwrap());
            let rhs = generate_expr(&ctx.expr().unwrap());
            emitter.emit(&format!("{} = {};", lhs, rhs));
        }
        _ => {
            emitter.emit(&format!("// TODO: {}", stmt.get_text()));
        }
    }
}

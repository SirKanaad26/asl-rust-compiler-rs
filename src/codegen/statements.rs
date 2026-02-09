use antlr_rust::tree::ParseTree;
use std::rc::Rc;

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
        StmtContextAll::StmtIfContext(ctx) => {
            generate_if_stmt(emitter, ctx);
        }
        _ => {
            emitter.emit(&format!("// TODO stmt: {}", stmt.get_text()));
        }
    }
}

fn generate_if_stmt(emitter: &mut CodeEmitter, ctx: &StmtIfContext<'_>) {
    let test = generate_expr(&ctx.expr().unwrap());
    emitter.emit(&format!("if {} {{", test));
    emitter.indent();
    if let Some(then_block) = ctx.blockOrEmbed1(0) {
        generate_block_or_embed1(emitter, &then_block);
    }
    emitter.dedent();

    for elsif in ctx.stmtElsIf_all() {
        let elsif_test = generate_expr(&elsif.expr().unwrap());
        emitter.emit(&format!("}} else if {} {{", elsif_test));
        emitter.indent();
        if let Some(elsif_block) = elsif.blockOrEmbed1() {
            generate_block_or_embed1(emitter, &elsif_block);
        }
        emitter.dedent();
    }

    if let Some(else_block) = ctx.blockOrEmbed1(1) {
        emitter.emit("} else {");
        emitter.indent();
        generate_block_or_embed1(emitter, &else_block);
        emitter.dedent();
    }

    emitter.emit("}");
}

fn generate_block_or_embed1(emitter: &mut CodeEmitter, block: &Rc<BlockOrEmbed1ContextAll<'_>>) {
    match block.as_ref() {
        BlockOrEmbed1ContextAll::BlockInlineContext(ctx) => {
            for inline in ctx.inlineStmt_all() {
                generate_inline_stmt(emitter, &inline);
            }
            if let Some(stmt) = ctx.stmt() {
                generate_stmt(emitter, &stmt);
            }
        }
        BlockOrEmbed1ContextAll::BlockIndentContext(ctx) => {
            if let Some(indented) = ctx.indentedBlock() {
                for stmt in indented.stmt_all() {
                    generate_stmt(emitter, &stmt);
                }
            }
        }
        _ => {
            emitter.emit(&format!("// TODO block: {}", block.get_text()));
        }
    }
}

/// Generate Rust code for an inline statement
fn generate_inline_stmt(emitter: &mut CodeEmitter, stmt: &Rc<InlineStmtContextAll<'_>>) {
    match stmt.as_ref() {
        InlineStmtContextAll::StmtReturnContext(ctx) => match ctx.expr() {
            Some(expr) => {
                let val = generate_expr(&expr);
                emitter.emit(&format!("return {};", val));
            }
            None => {
                emitter.emit("return;");
            }
        },
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

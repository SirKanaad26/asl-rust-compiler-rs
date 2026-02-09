use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTree;
use std::ops::Deref;
use std::rc::Rc;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::expressions::{generate_expr, generate_lval};
use crate::parser::aslparser::*;

/// Get the source column of a statement's first token.
fn stmt_col(stmt: &Rc<StmtContextAll<'_>>) -> isize {
    stmt.deref().start().get_column()
}

/// Generate stmts from an indentedBlock, splitting absorbed stmts by column.
/// Stmts deeper than `parent_col` are emitted as body; the rest are returned
/// as deferred for the caller to handle at a higher scope.
fn generate_indented_block_split<'a>(
    emitter: &mut CodeEmitter,
    block: &Rc<IndentedBlockContextAll<'a>>,
    parent_col: isize,
) -> Vec<Rc<StmtContextAll<'a>>> {
    let stmts: Vec<_> = block.stmt_all();
    let split = stmts.iter().position(|s| {
        stmt_col(s) <= parent_col
    }).unwrap_or(stmts.len());

    let mut deferred: Vec<Rc<StmtContextAll<'a>>> = Vec::new();

    // Generate body stmts; any deferred from inner blocks get filtered here
    for stmt in &stmts[..split] {
        let inner_deferred = generate_stmt(emitter, stmt);
        for d in inner_deferred {
            if stmt_col(&d) > parent_col {
                let further = generate_stmt(emitter, &d);
                deferred.extend(further);
            } else {
                deferred.push(d);
            }
        }
    }

    // Absorbed stmts from this block are all deferred
    deferred.extend(stmts[split..].iter().cloned());

    deferred
}

/// Generate Rust code for a single stmt node.
/// Returns any deferred stmts that were absorbed by the parser's INDENT/DEDENT
/// handling and belong at a higher scope.
pub fn generate_stmt<'a>(
    emitter: &mut CodeEmitter,
    stmt: &Rc<StmtContextAll<'a>>,
) -> Vec<Rc<StmtContextAll<'a>>> {
    match stmt.as_ref() {
        StmtContextAll::StmtsInlineContext(ctx) => {
            if let Some(inline) = ctx.inlineStmt() {
                generate_inline_stmt(emitter, &inline);
            }
            vec![]
        }
        StmtContextAll::StmtIfContext(ctx) => {
            generate_if_stmt(emitter, ctx);
            vec![]
        }
        StmtContextAll::StmtWhileContext(ctx) => {
            generate_while_stmt(emitter, ctx)
        }
        StmtContextAll::StmtForContext(ctx) => {
            generate_for_stmt(emitter, ctx)
        }
        _ => {
            emitter.emit(&format!("// TODO stmt: {}", stmt.get_text()));
            vec![]
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

fn generate_while_stmt<'a>(
    emitter: &mut CodeEmitter,
    ctx: &StmtWhileContext<'a>,
) -> Vec<Rc<StmtContextAll<'a>>> {
    let cond = generate_expr(&ctx.expr().unwrap());
    let while_col = ctx.start().get_column();

    emitter.emit(&format!("while {} {{", cond));
    emitter.indent();

    let deferred = match ctx.indentedBlock() {
        Some(block) => generate_indented_block_split(emitter, &block, while_col),
        None => vec![],
    };

    emitter.dedent();
    emitter.emit("}");

    deferred
}

fn generate_for_stmt<'a>(
    emitter: &mut CodeEmitter,
    ctx: &StmtForContext<'a>,
) -> Vec<Rc<StmtContextAll<'a>>> {
    let var = ctx.id().unwrap().get_text();
    let for_col = ctx.start().get_column();

    let begin = ctx.begin.as_ref().map(|e| generate_expr(e)).unwrap_or_default();
    let end = ctx.end.as_ref().map(|e| generate_expr(e)).unwrap_or_default();

    let is_downto = ctx.direction.as_ref()
        .map(|t| t.get_text() == "downto")
        .unwrap_or(false);

    if is_downto {
        emitter.emit(&format!("for {} in ({}..={}).rev() {{", var, end, begin));
    } else {
        emitter.emit(&format!("for {} in {}..={} {{", var, begin, end));
    }

    emitter.indent();

    let deferred = match ctx.indentedBlock() {
        Some(block) => generate_indented_block_split(emitter, &block, for_col),
        None => vec![],
    };

    emitter.dedent();
    emitter.emit("}");

    deferred
}

fn generate_block_or_embed1(emitter: &mut CodeEmitter, block: &Rc<BlockOrEmbed1ContextAll<'_>>) {
    match block.as_ref() {
        BlockOrEmbed1ContextAll::BlockInlineContext(ctx) => {
            for inline in ctx.inlineStmt_all() {
                generate_inline_stmt(emitter, &inline);
            }
            if let Some(stmt) = ctx.stmt() {
                let deferred = generate_stmt(emitter, &stmt);
                for d in deferred {
                    generate_stmt(emitter, &d);
                }
            }
        }
        BlockOrEmbed1ContextAll::BlockIndentContext(ctx) => {
            if let Some(indented) = ctx.indentedBlock() {
                for stmt in indented.stmt_all() {
                    let deferred = generate_stmt(emitter, &stmt);
                    for d in deferred {
                        generate_stmt(emitter, &d);
                    }
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
        InlineStmtContextAll::StmtAssertContext(ctx) => {
            let cond = generate_expr(&ctx.expr().unwrap());
            emitter.emit(&format!("assert!({});", cond));
        }
        _ => {
            emitter.emit(&format!("// TODO: {}", stmt.get_text()));
        }
    }
}

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTree;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::ops::Deref;
use std::rc::Rc;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::expressions::{generate_expr, generate_lval};
use crate::codegen::types::map_type;
use crate::parser::aslparser::*;

// Tracks whether the currently-generating statement is inside a __decode block.
// UNDEFINED in decode means the instruction doesn't exist → return None, not panic.
thread_local! {
    static IN_DECODE_CTX: Cell<bool> = Cell::new(false);
    // Names of variables that are implicitly declared via first assignment in __decode.
    // These need `let mut name = rhs;` instead of bare `name = rhs;`.
    static IMPLICIT_DECODE_VARS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Register the set of implicitly-declared decode vars before generating a decode block.
/// Call with an empty set after the block to clear state.
pub fn set_implicit_decode_vars(names: HashSet<String>) {
    IMPLICIT_DECODE_VARS.with(|v| *v.borrow_mut() = names);
}

fn is_implicit_decode_var(name: &str) -> bool {
    IMPLICIT_DECODE_VARS.with(|v| v.borrow().contains(name))
}

fn in_decode_ctx() -> bool {
    IN_DECODE_CTX.with(|b| b.get())
}

/// Like `generate_stmt`, but sets the decode context flag so that UNDEFINED
/// emits `return None` instead of `panic!`.  Use this for __decode block stmts.
pub fn generate_stmt_in_decode<'a>(
    emitter: &mut CodeEmitter,
    stmt: &Rc<StmtContextAll<'a>>,
) -> Vec<Rc<StmtContextAll<'a>>> {
    IN_DECODE_CTX.with(|b| b.set(true));
    let result = generate_stmt(emitter, stmt);
    IN_DECODE_CTX.with(|b| b.set(false));
    result
}

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
        StmtContextAll::StmtCaseContext(ctx) => {
            generate_case_stmt(emitter, ctx)
        }
        StmtContextAll::StmtTryContext(ctx) => {
            generate_try_stmt(emitter, ctx)
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
            let lval = ctx.lValExpr().unwrap();
            let rhs = generate_expr(&ctx.expr().unwrap());
            // Special-case register array writes: X[n] = v → set_Xreg(cpu, n, v)
            if let Some((setter, idx)) = reg_write_setter(&lval) {
                emitter.emit(&format!("{}(cpu, {}, {});", setter, idx, rhs));
            } else if let LValExprContextAll::LValSliceOfContext(slice_ctx) = lval.as_ref() {
                // Bit-slice write: x<hi:lo> = val → obj.set_slice(lo, hi, val as u128)
                //                  x<i>     = val → obj.set_slice(i, i, val as u128)
                let obj = generate_lval(&slice_ctx.lValExpr().unwrap());
                let slices = slice_ctx.sliceCommaList1().unwrap().slice_all();
                if slices.len() == 1 {
                    // BV-6: use AslValue::to_u128() instead of `as u128` so
                    // BitVec<N> RHS values coerce cleanly (Rust's `as` cast
                    // does not work on non-primitive types).
                    let stmt = match slices[0].as_ref() {
                        SliceContextAll::SliceSingleContext(sctx) => {
                            let bit = generate_expr(&sctx.expr().unwrap());
                            format!("{}.set_slice({} as usize, {} as usize, AslValue::to_u128({}));",
                                obj, bit, bit, rhs)
                        }
                        SliceContextAll::SliceRangeContext(rctx) => {
                            let hi = rctx.begin.as_ref().unwrap().get_text();
                            let lo = rctx.end.as_ref().unwrap().get_text();
                            format!("{}.set_slice({} as usize, {} as usize, AslValue::to_u128({}));",
                                obj, lo, hi, rhs)
                        }
                        _ => format!("// TODO: bit slice write: {}<{}>",
                            obj, slices[0].get_text()),
                    };
                    emitter.emit(&stmt);
                } else {
                    emitter.emit(&format!("// TODO: multi-slice write: {}", lval.get_text()));
                }
            } else {
                // In __decode blocks, an assignment to a bare name that was collected as
                // an implicit decode var needs `let mut` (no explicit type annotation in ASL source).
                if in_decode_ctx() {
                    if let LValExprContextAll::LValVarRefContext(lref) = lval.as_ref() {
                        let var_name = lref.qualId().unwrap().get_text();
                        if is_implicit_decode_var(&var_name) {
                            emitter.emit(&format!("let mut {} = {};", var_name, rhs));
                            return;
                        }
                    }
                }
                let lhs = generate_lval(&lval);
                emitter.emit(&format!("{} = {};", lhs, rhs));
            }
        }
        InlineStmtContextAll::StmtAssertContext(ctx) => {
            let cond = generate_expr(&ctx.expr().unwrap());
            emitter.emit(&format!("assert!({});", cond));
        }
        InlineStmtContextAll::StmtVarDeclInitContext(ctx) => {
            let sym = ctx.symDecl().unwrap();
            let ty = map_type(&sym.typeSpec().unwrap());
            let name = sym.id().unwrap().get_text();
            let val = generate_expr(&ctx.expr().unwrap());
            // BV-6: use `BitVec::from_asl(rhs)` — NOT `BitVec<N>::from_asl`.
            // The let-binding's explicit `BitVec<N>` annotation drives const-
            // generic N inference; writing `BitVec<N>::` is not valid Rust.
            if ty.starts_with("BitVec<") {
                emitter.emit(&format!("let mut {}: {} = BitVec::from_asl({});", name, ty, val));
            } else {
                emitter.emit(&format!("let mut {}: {} = {};", name, ty, val));
            }
        }
        InlineStmtContextAll::StmtConstDeclContext(ctx) => {
            let sym = ctx.symDecl().unwrap();
            let ty = map_type(&sym.typeSpec().unwrap());
            let name = sym.id().unwrap().get_text();
            let val = generate_expr(&ctx.expr().unwrap());
            if ty.starts_with("BitVec<") {
                emitter.emit(&format!("let {}: {} = BitVec::from_asl({});", name, ty, val));
            } else {
                emitter.emit(&format!("let {}: {} = {};", name, ty, val));
            }
        }
        InlineStmtContextAll::StmtUnpredictableContext(_) => {
            emitter.emit("panic!(\"UNPREDICTABLE\");");
        }
        InlineStmtContextAll::StmtUndefinedContext(_) => {
            if in_decode_ctx() {
                emitter.emit("return None; // UNDEFINED");
            } else {
                emitter.emit("panic!(\"UNDEFINED\");");
            }
        }
        InlineStmtContextAll::StmtImpDefContext(_) => {
            emitter.emit("panic!(\"IMPLEMENTATION_DEFINED\");");
        }
        InlineStmtContextAll::StmtSeeContext(ctx) => {
            let see_text = ctx.SEE_TOK().map(|t| t.get_text()).unwrap_or_default();
            emitter.emit(&format!("panic!(\"SEE {}\");", see_text));
        }
        InlineStmtContextAll::StmtThrowContext(ctx) => {
            let id = ctx.id().unwrap().get_text();
            emitter.emit(&format!("panic!(\"{}\");", id));
        }
        InlineStmtContextAll::StmtRepeatContext(ctx) => {
            emitter.emit("loop {");
            emitter.indent();
            if let Some(block) = ctx.indentedBlock() {
                for stmt in block.stmt_all() {
                    let deferred = generate_stmt(emitter, &stmt);
                    for d in deferred {
                        generate_stmt(emitter, &d);
                    }
                }
            }
            let cond = generate_expr(&ctx.expr().unwrap());
            emitter.emit(&format!("if {} {{ break; }}", cond));
            emitter.dedent();
            emitter.emit("}");
        }
        InlineStmtContextAll::StmtCallContext(ctx) => {
            // Procedure call as a statement: `CheckVFPEnabled(TRUE);`
            let name = ctx.qualId().unwrap().get_text().replace('.', "_");
            let args: Vec<String> = ctx.exprCommaList0().unwrap()
                .expr_all()
                .iter()
                .map(|e| generate_expr(e))
                .collect();
            emitter.emit(&format!("{}({});", name, args.join(", ")));
        }
        InlineStmtContextAll::StmtDefEnumContext(ctx) => {
            let name = ctx.id().unwrap().get_text();
            let variants = ctx.identifierCommaList0().unwrap();
            let ids: Vec<String> = variants.id_all().iter().map(|id| id.get_text()).collect();
            emitter.emit("#[derive(Debug, Clone, Copy, PartialEq)]");
            emitter.emit(&format!("enum {} {{ {} }}", name, ids.join(", ")));
            emitter.emit(&format!("use {}::*;", name));
        }
        _ => {
            emitter.emit(&format!("// TODO: {}", stmt.get_text()));
        }
    }
}

fn generate_case_stmt<'a>(
    emitter: &mut CodeEmitter,
    ctx: &StmtCaseContext<'a>,
) -> Vec<Rc<StmtContextAll<'a>>> {
    let case_col = ctx.start().get_column();
    let test = generate_expr(&ctx.expr().unwrap());
    emitter.emit(&format!("match {} {{", test));
    emitter.indent();

    let mut deferred: Vec<Rc<StmtContextAll<'a>>> = Vec::new();

    for alt in ctx.caseAlt_all() {
        match alt.as_ref() {
            CaseAltContextAll::CaseAltWhenContext(when_ctx) => {
                let patterns: Vec<String> = when_ctx
                    .casePattern_all()
                    .iter()
                    .map(|p| generate_case_pattern(p))
                    .collect();
                let combined = patterns.join(" | ");

                let arm = match when_ctx.expr() {
                    Some(guard) => format!("{} if {} => {{", combined, generate_expr(&guard)),
                    None => format!("{} => {{", combined),
                };
                emitter.emit(&arm);
                emitter.indent();
                if let Some(embed0) = when_ctx.blockOrEmbed0() {
                    deferred.extend(generate_case_alt_body(emitter, &embed0, case_col));
                }
                emitter.dedent();
                emitter.emit("}");
            }
            CaseAltContextAll::CaseAltOtherwiseContext(ow_ctx) => {
                emitter.emit("_ => {");
                emitter.indent();
                if let Some(embed0) = ow_ctx.blockOrEmbed0() {
                    deferred.extend(generate_case_alt_body(emitter, &embed0, case_col));
                }
                emitter.dedent();
                emitter.emit("}");
            }
            _ => {
                emitter.emit("// TODO: unknown case alt");
            }
        }
    }

    emitter.dedent();
    emitter.emit("}");

    deferred
}

fn generate_case_alt_body<'a>(
    emitter: &mut CodeEmitter,
    embed0: &Rc<BlockOrEmbed0ContextAll<'a>>,
    case_col: isize,
) -> Vec<Rc<StmtContextAll<'a>>> {
    match embed0.blockOrEmbed1() {
        Some(embed1) => match embed1.as_ref() {
            BlockOrEmbed1ContextAll::BlockIndentContext(ctx) => {
                match ctx.indentedBlock() {
                    Some(block) => generate_indented_block_split(emitter, &block, case_col),
                    None => vec![],
                }
            }
            _ => {
                generate_block_or_embed1(emitter, &embed1);
                vec![]
            }
        },
        None => vec![],
    }
}

fn generate_case_pattern(pat: &Rc<CasePatternContextAll<'_>>) -> String {
    match pat.as_ref() {
        CasePatternContextAll::CasePatternNatContext(ctx) => {
            ctx.NAT_LIT().unwrap().get_text()
        }
        CasePatternContextAll::CasePatternHexContext(ctx) => {
            ctx.HEX_LIT().unwrap().get_text()
        }
        CasePatternContextAll::CasePatternBinContext(ctx) => {
            let raw = ctx.BIN_LIT().unwrap().get_text();
            let bits = raw.trim_matches('\'').replace(' ', "");
            format!("0b{}", bits)
        }
        CasePatternContextAll::CasePatternMaskContext(ctx) => {
            format!("todo!(\"mask pattern: {}\")", ctx.MASK_LIT().unwrap().get_text())
        }
        CasePatternContextAll::CasePatternBindContext(ctx) => {
            ctx.id().unwrap().get_text()
        }
        CasePatternContextAll::CasePatternIgnoreContext(_) => {
            "_".to_string()
        }
        CasePatternContextAll::CasePatternTupleContext(ctx) => {
            let inner: Vec<String> = ctx
                .casePattern_all()
                .iter()
                .map(|p| generate_case_pattern(p))
                .collect();
            format!("({})", inner.join(", "))
        }
        _ => "todo!(\"unknown pattern\")".to_string(),
    }
}

fn generate_try_stmt<'a>(
    emitter: &mut CodeEmitter,
    ctx: &StmtTryContext<'a>,
) -> Vec<Rc<StmtContextAll<'a>>> {
    let try_col = ctx.start().get_column();
    let catch_var = ctx.id().map(|id| id.get_text()).unwrap_or_else(|| "e".to_string());

    // Emit try body wrapped in catch_unwind
    emitter.emit("let __try_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {");
    emitter.indent();

    let mut deferred = match ctx.indentedBlock() {
        Some(block) => generate_indented_block_split(emitter, &block, try_col),
        None => vec![],
    };

    emitter.dedent();
    emitter.emit("}));");

    // Emit catch handlers as if/else-if/else chain
    emitter.emit("if let Err(__e) = __try_result {");
    emitter.indent();
    emitter.emit(&format!(
        "let {v} = if let Some(s) = __e.downcast_ref::<&str>() {{ s }} else if let Some(s) = __e.downcast_ref::<String>() {{ s.as_str() }} else {{ \"unknown\" }};",
        v = catch_var
    ));

    let alts = ctx.catchAlt_all();
    let mut first_when = true;

    for alt in &alts {
        match alt.as_ref() {
            CatchAltContextAll::CatchAltWhenContext(when_ctx) => {
                let cond = generate_expr(&when_ctx.expr().unwrap());
                if first_when {
                    emitter.emit(&format!("if {} {{", cond));
                    first_when = false;
                } else {
                    emitter.emit(&format!("}} else if {} {{", cond));
                }
                emitter.indent();
                if let Some(block) = when_ctx.blockOrEmbed1() {
                    deferred.extend(generate_catch_alt_body(emitter, &block, try_col));
                }
                emitter.dedent();
            }
            CatchAltContextAll::CatchAltOtherwiseContext(ow_ctx) => {
                if first_when {
                    // otherwise with no preceding when — just emit the body directly
                    if let Some(block) = ow_ctx.blockOrEmbed1() {
                        deferred.extend(generate_catch_alt_body(emitter, &block, try_col));
                    }
                } else {
                    emitter.emit("} else {");
                    emitter.indent();
                    if let Some(block) = ow_ctx.blockOrEmbed1() {
                        deferred.extend(generate_catch_alt_body(emitter, &block, try_col));
                    }
                    emitter.dedent();
                }
            }
            _ => {
                emitter.emit("// TODO: unknown catch alt");
            }
        }
    }

    if !first_when {
        emitter.emit("}");
    }

    emitter.dedent();
    emitter.emit("}");

    deferred
}

/// If `lval` is a register array write (e.g. `X[n]`, `W[n]`), returns
/// `(setter_fn_name, index_string)` so the caller can emit `setter(cpu, idx, rhs)`.
/// Returns `None` for all other lvalues.
fn reg_write_setter(lval: &Rc<LValExprContextAll<'_>>) -> Option<(String, String)> {
    if let LValExprContextAll::LValArrayIndexContext(ctx) = lval.as_ref() {
        let inner = ctx.lValExpr()?;
        if let LValExprContextAll::LValVarRefContext(vctx) = inner.as_ref() {
            let name = vctx.qualId()?.get_text();
            let slices = ctx.slice_all();
            if slices.len() == 1 {
                let idx = slices[0].get_text().to_string();
                let setter = match name.as_str() {
                    "X" => "set_Xreg",
                    "W" => "set_Wreg",
                    "R" => "set_Rreg",
                    "S" => "set_Sreg",
                    "D" => "set_Dreg",
                    _ => return None,
                };
                return Some((setter.to_string(), idx));
            }
        }
    }
    None
}

fn generate_catch_alt_body<'a>(
    emitter: &mut CodeEmitter,
    embed1: &Rc<BlockOrEmbed1ContextAll<'a>>,
    try_col: isize,
) -> Vec<Rc<StmtContextAll<'a>>> {
    match embed1.as_ref() {
        BlockOrEmbed1ContextAll::BlockIndentContext(ctx) => {
            match ctx.indentedBlock() {
                Some(block) => generate_indented_block_split(emitter, &block, try_col),
                None => vec![],
            }
        }
        _ => {
            generate_block_or_embed1(emitter, embed1);
            vec![]
        }
    }
}

/// Pre-scan a flat list of top-level stmts to find variable names that are
/// assigned but never explicitly declared.  Returns names in first-seen order,
/// each needing a `let mut x = Default::default();` hoisted before the body.
///
/// `known` contains names already in scope: field shadows, function params, etc.
pub fn collect_implicit_decls(
    stmts: &[Rc<StmtContextAll<'_>>],
    known: &HashSet<String>,
) -> Vec<String> {
    let mut declared: HashSet<String> = HashSet::new();
    let mut assigned: Vec<String> = Vec::new();

    for stmt in stmts {
        if let StmtContextAll::StmtsInlineContext(ctx) = stmt.as_ref() {
            if let Some(inline) = ctx.inlineStmt() {
                match inline.as_ref() {
                    InlineStmtContextAll::StmtVarDeclInitContext(vctx) => {
                        if let Some(sym) = vctx.symDecl() {
                            if let Some(id) = sym.id() {
                                declared.insert(id.get_text());
                            }
                        }
                    }
                    InlineStmtContextAll::StmtConstDeclContext(vctx) => {
                        if let Some(sym) = vctx.symDecl() {
                            if let Some(id) = sym.id() {
                                declared.insert(id.get_text());
                            }
                        }
                    }
                    InlineStmtContextAll::StmtAssignContext(actx) => {
                        if let Some(lval) = actx.lValExpr() {
                            collect_lval_simple_names(&lval, &mut assigned);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut seen = HashSet::new();
    assigned
        .into_iter()
        .filter(|name| {
            !declared.contains(name) && !known.contains(name) && seen.insert(name.clone())
        })
        .collect()
}

/// Collect bare variable names from an lvalue (simple var refs and tuples only).
/// Skips member access, array index, PSTATE, and register family names.
fn collect_lval_simple_names(lval: &Rc<LValExprContextAll<'_>>, names: &mut Vec<String>) {
    match lval.as_ref() {
        LValExprContextAll::LValVarRefContext(ctx) => {
            if let Some(qid) = ctx.qualId() {
                let name = qid.get_text().replace('.', "_");
                // Skip PSTATE (mapped to cpu.*) and register families (X, W, R, S, D)
                match name.as_str() {
                    "PSTATE" | "X" | "W" | "R" | "S" | "D" => {}
                    _ => names.push(name),
                }
            }
        }
        LValExprContextAll::LValTupleContext(ctx) => {
            for elem in ctx.lValExpr_all() {
                collect_lval_simple_names(&elem, names);
            }
        }
        _ => {} // member access (PSTATE.N), array index (X[n]), slice, etc.
    }
}

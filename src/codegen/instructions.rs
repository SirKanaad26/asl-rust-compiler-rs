use std::collections::HashSet;
use std::rc::Rc;
use antlr_rust::tree::ParseTree;
use antlr_rust::token::Token;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::expressions::generate_expr;
use crate::codegen::statements::{collect_implicit_decls, generate_stmt, generate_stmt_in_decode, generate_stmts_with_else_fixup, set_implicit_decode_vars};
use crate::codegen::types::map_type;
use crate::parser::aslparser::{
    InstructionContextAll,
    InstructionContextAttrs,
    EncodingContextAttrs,
    InstructionFieldContextAttrs,
    IndentedBlockContextAll,
    IndentedBlockContextAttrs,
    StmtContextAll,
    StmtsInlineContextAttrs,
    InlineStmtContextAll,
    StmtVarDeclInitContextAttrs,
    StmtAssignContextAttrs,
    SymDeclContextAttrs,
    LValExprContextAll,
    LValVarRefContextAttrs,
};

/// Emit the crate-root header for a generated instructions file.
/// The full runtime (stubs, CpuState, register accessors) lives in
/// `src/runtime/runtime.rs` and is copied to the output directory as
/// `runtime.rs` by the caller in `main.rs`.
pub fn generate_asl_runtime(emitter: &mut CodeEmitter) {
    emitter.emit("#![feature(generic_const_exprs)]");
    emitter.emit("#![allow(incomplete_features, non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]");
    emitter.emit("mod bitvec;");
    emitter.emit("mod runtime;");
    emitter.emit("use bitvec::{BitVec, AslValue};");
    emitter.emit("use runtime::*;");
    emitter.emit("");
}

/// Scan a decode block for top-level variable declarations, returning:
/// - `Vec<(name, rust_type)>` — all vars to add to the encoding struct
/// - `HashSet<String>` — names of *implicitly* declared vars (bare assignment, no type annotation)
///
/// Typed declarations (`integer d = UInt(Rd)`) are handled by `generate_inline_stmt` which
/// already emits `let mut d: i128 = ...;`.  Implicit assignments (`m = UInt(Rm)`) also need
/// `let mut` but have no type annotation — we infer `bool` for comparisons, `i128` otherwise.
fn collect_decode_vars(decode: &Option<Rc<IndentedBlockContextAll<'_>>>) -> (Vec<(String, String)>, HashSet<String>) {
    let mut vars: Vec<(String, String)> = Vec::new();
    let mut implicit_names: HashSet<String> = HashSet::new();
    let mut seen: HashSet<String> = HashSet::new();
    let block = match decode {
        Some(b) => b,
        None => return (vars, implicit_names),
    };
    for stmt in block.stmt_all() {
        if let StmtContextAll::StmtsInlineContext(ctx) = stmt.as_ref() {
            if let Some(inline) = ctx.inlineStmt() {
                match inline.as_ref() {
                    InlineStmtContextAll::StmtVarDeclInitContext(vctx) => {
                        let sym = vctx.symDecl().unwrap();
                        let ty = map_type(&sym.typeSpec().unwrap());
                        let name = sym.id().unwrap().get_text();
                        if seen.insert(name.clone()) {
                            vars.push((name, ty));
                        }
                    }
                    InlineStmtContextAll::StmtAssignContext(actx) => {
                        if let Some(lval) = actx.lValExpr() {
                            if let LValExprContextAll::LValVarRefContext(lref) = lval.as_ref() {
                                let name = lref.qualId().unwrap().get_text();
                                if seen.insert(name.clone()) {
                                    // Infer type: bool for comparisons, i128 otherwise
                                    let rhs_text = actx.expr()
                                        .map(|e| e.get_text())
                                        .unwrap_or_default();
                                    let ty = if rhs_text.contains("==") || rhs_text.contains("!=")
                                        || rhs_text.starts_with('!')
                                        || rhs_text == "TRUE" || rhs_text == "FALSE"
                                    {
                                        "bool".to_string()
                                    } else {
                                        "i128".to_string()
                                    };
                                    vars.push((name.clone(), ty));
                                    implicit_names.insert(name);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    (vars, implicit_names)
}

/// Emit shadowing lets at the top of execute/postdecode so the body can use bare names.
fn emit_field_shadows(
    emitter: &mut CodeEmitter,
    raw_fields: &[(String, u64, u64)],
    decode_vars: &[(String, String)],
) {
    for (name, _, _) in raw_fields {
        emitter.emit(&format!("let {} = enc.{};", name, name));
    }
    for (name, _) in decode_vars {
        emitter.emit(&format!("let {} = enc.{};", name, name));
    }
}

pub fn generate_instruction(emitter: &mut CodeEmitter, instr: &Rc<InstructionContextAll<'_>>) {
    let instr_name = instr.idWithDots().unwrap().get_text();
    let instr_name_safe = instr_name.replace('.', "_");

    emitter.emit(&format!("// Instruction: {}", instr_name));

    // Track the first encoding's name and field sets for postdecode/execute signatures.
    // All encodings of an instruction compute the same variable names (ARM spec convention),
    // so using the first encoding's struct for postdecode/execute is correct in practice.
    let mut first_enc_name_safe: Option<String> = None;
    let mut first_raw_fields: Vec<(String, u64, u64)> = Vec::new();
    let mut first_decode_vars: Vec<(String, String)> = Vec::new();
    // (enc_name_safe, fixed_bits) — used for the test harness
    let mut test_cases: Vec<(String, u64)> = Vec::new();

    for enc in instr.encoding_all() {
        let enc_name = enc.idWithDots().unwrap().get_text();
        let enc_name_safe = enc_name.replace('.', "_");
        let inst_set = enc.instructionSet.as_ref().map(|t| t.get_text()).unwrap_or("unknown");

        emitter.emit("");
        emitter.emit(&format!("// Encoding: {} ({})", enc_name, inst_set));

        // Collect raw bit fields: (name, start_bit, bit_length)
        let fields: Vec<(String, u64, u64)> = enc
            .instructionField_all()
            .iter()
            .map(|f| {
                let name = f.id().unwrap().get_text();
                let begin: u64 = f.begin.as_ref().unwrap().get_text().parse().unwrap();
                let len: u64 = f.len.as_ref().unwrap().get_text().parse().unwrap();
                (name, begin, len)
            })
            .collect();

        // Collect variables declared in __decode (e.g. `integer d = UInt(Rd)`)
        // Also collect implicitly-declared vars (bare assignment, e.g. `m = UInt(Rm)`)
        let (decode_vars, implicit_names) = collect_decode_vars(&enc.decode);

        if first_enc_name_safe.is_none() {
            first_enc_name_safe = Some(enc_name_safe.clone());
            first_raw_fields = fields.clone();
            first_decode_vars = decode_vars.clone();
        }

        // Generate struct: raw bit fields + decode-computed vars
        emitter.emit(&format!("pub struct {} {{", enc_name_safe));
        emitter.indent();
        for (name, _, _) in &fields {
            emitter.emit(&format!("pub {}: u64,", name));
        }
        for (name, ty) in &decode_vars {
            emitter.emit(&format!("pub {}: {},", name, ty));
        }
        emitter.dedent();
        emitter.emit("}");
        emitter.emit("");

        // Generate impl with decode function
        emitter.emit(&format!("impl {} {{", enc_name_safe));
        emitter.indent();
        emitter.emit("pub fn decode(bits: u64) -> Option<Self> {");
        emitter.indent();

        // Opcode mask check
        let opcode_text = enc.opcode.as_ref().map(|t| t.get_text()).unwrap_or("''");
        let (fixed_mask, fixed_bits) = parse_opcode(&opcode_text);
        test_cases.push((enc_name_safe.clone(), fixed_bits));
        emitter.emit(&format!("let fixed_mask: u64 = 0x{:X};", fixed_mask));
        emitter.emit(&format!("let fixed_bits: u64 = 0x{:X};", fixed_bits));
        emitter.emit("if bits & fixed_mask != fixed_bits {");
        emitter.indent();
        emitter.emit("return None;");
        emitter.dedent();
        emitter.emit("}");

        // Field extraction
        for (name, begin, len) in &fields {
            let mask = (1u64 << len) - 1;
            emitter.emit(&format!(
                "let {}: u64 = (bits >> {}) & 0x{:X};",
                name, begin, mask
            ));
        }

        // Guard check — skip trivially-true guards
        if let Some(guard) = enc.expr() {
            let guard_str = generate_expr(&guard);
            if guard_str != "true" {
                emitter.emit(&format!("if !({}) {{ return None; }}", guard_str));
            }
        }

        // __unpredictable_unless checks: runtime asserts that certain bits hold expected values.
        // UNPREDICTABLE behaviour is architecturally undefined, so we panic if violated.
        for unpred in enc.instrUnpredictableUnless_all() {
            if let (Some(idx_tok), Some(bin_tok)) = (&unpred.idx, &unpred.bin) {
                let idx: u64 = idx_tok.get_text().parse().unwrap_or(0);
                // BIN_LIT is like '0' or '1' — strip the surrounding quotes
                let bit_val: u64 = bin_tok.get_text().trim_matches('\'').parse().unwrap_or(0);
                emitter.emit(&format!(
                    "assert!((bits >> {}) & 1 == {}, \"UNPREDICTABLE\");",
                    idx, bit_val
                ));
            }
        }

        // Decode block statements (declare and compute the decode vars).
        // UNDEFINED here means the instruction doesn't exist → return None.
        // Register implicit var names so generate_stmt_in_decode emits `let mut` for them.
        set_implicit_decode_vars(implicit_names);
        if let Some(block) = &enc.decode {
            for stmt in block.stmt_all() {
                let deferred = generate_stmt_in_decode(emitter, &stmt);
                for d in deferred {
                    generate_stmt_in_decode(emitter, &d);
                }
            }
        }
        set_implicit_decode_vars(HashSet::new());

        // Return struct populated with raw fields + decode-computed vars
        let all_names: Vec<String> = fields
            .iter()
            .map(|(n, _, _)| n.clone())
            .chain(decode_vars.iter().map(|(n, _)| n.clone()))
            .collect();
        if all_names.is_empty() {
            emitter.emit("Some(Self {})");
        } else {
            emitter.emit(&format!("Some(Self {{ {} }})", all_names.join(", ")));
        }

        emitter.dedent();
        emitter.emit("}");
        emitter.dedent();
        emitter.emit("}");
    }

    let enc_type = first_enc_name_safe.as_deref().unwrap_or("/* unknown */");

    // Postdecode function — receives the decoded struct; shadows fields so body uses bare names
    if let Some(block) = &instr.postDecodeBlock {
        emitter.emit("");
        emitter.emit(&format!("// Postdecode: {}", instr_name));
        emitter.emit(&format!("pub fn postdecode_{}(enc: &{}) {{", instr_name_safe, enc_type));
        emitter.indent();
        emit_field_shadows(emitter, &first_raw_fields, &first_decode_vars);
        let shadow_names: HashSet<String> = first_raw_fields.iter()
            .map(|(n, _, _)| n.clone())
            .chain(first_decode_vars.iter().map(|(n, _)| n.clone()))
            .collect();
        let postdecode_stmts = block.stmt_all();
        for name in collect_implicit_decls(&postdecode_stmts, &shadow_names) {
            emitter.emit(&format!("let mut {} = Default::default();", name));
        }
        let deferred = generate_stmts_with_else_fixup(emitter, &postdecode_stmts);
        for d in deferred {
            generate_stmt(emitter, &d);
        }
        emitter.dedent();
        emitter.emit("}");
    }

    // Execute function — receives the decoded struct; shadows fields so body uses bare names
    emitter.emit("");
    emitter.emit(&format!("// Execute: {}", instr_name));
    emitter.emit(&format!(
        "pub fn execute_{}(enc: &{}, cpu: &mut CpuState) {{",
        instr_name_safe, enc_type
    ));
    emitter.indent();
    emit_field_shadows(emitter, &first_raw_fields, &first_decode_vars);
    let execute_shadow_names: HashSet<String> = first_raw_fields.iter()
        .map(|(n, _, _)| n.clone())
        .chain(first_decode_vars.iter().map(|(n, _)| n.clone()))
        .collect();
    if let Some(block) = &instr.executeBlock {
        for name in collect_implicit_decls(&block.stmt_all(), &execute_shadow_names) {
            emitter.emit(&format!("let mut {} = Default::default();", name));
        }
    }
    let is_conditional = instr.conditional.is_some();
    if is_conditional {
        emitter.emit("if check_condition(cpu) {");
        emitter.indent();
    }
    if let Some(block) = &instr.executeBlock {
        let stmts = block.stmt_all();
        if stmts.is_empty() {
            emitter.emit("// empty execute block");
        } else {
            let deferred = generate_stmts_with_else_fixup(emitter, &stmts);
            for d in deferred {
                generate_stmt(emitter, &d);
            }
        }
    } else {
        emitter.emit(&format!("todo!(\"execute {}\");", instr_name));
    }
    if is_conditional {
        emitter.dedent();
        emitter.emit("}");
    }
    emitter.dedent();
    emitter.emit("}");

}

/// Parse an ASL opcode pattern (e.g. `'11xx 00xx'`) into (fixed_mask, fixed_bits).
/// `'1'` → both mask and bit set; `'0'` → mask set, bit clear; `'x'` → ignored.
fn parse_opcode(opcode: &str) -> (u64, u64) {
    let bits: String = opcode
        .trim_matches('\'')
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let n = bits.len();
    let mut fixed_mask: u64 = 0;
    let mut fixed_bits: u64 = 0;
    for (i, ch) in bits.chars().enumerate() {
        let bit_pos = (n - 1 - i) as u64;
        match ch {
            '1' => {
                fixed_mask |= 1u64 << bit_pos;
                fixed_bits |= 1u64 << bit_pos;
            }
            '0' => {
                fixed_mask |= 1u64 << bit_pos;
            }
            _ => {} // 'x' or other: don't-care bit
        }
    }
    (fixed_mask, fixed_bits)
}

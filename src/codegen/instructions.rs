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

/// Emit stub implementations of common ASL built-in functions so the generated
/// file compiles without an external runtime crate.  These are approximations —
/// callers should replace them with correct implementations as needed.
pub fn generate_asl_runtime(emitter: &mut CodeEmitter) {
    // Nightly features required for BitVec<N> const-generic slice/concat.
    emitter.emit("#![feature(generic_const_exprs)]");
    emitter.emit("#![allow(incomplete_features, non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]");
    emitter.emit("mod bitvec;");
    emitter.emit("use bitvec::BitVec;");
    emitter.emit("use bitvec::AslValue;");
    emitter.emit("");
    emitter.emit("// ── ASL built-in runtime stubs ──────────────────────────────────────────────");
    for (sig, body) in &[
        // Numeric conversion stubs — accept any AslValue so u64, i128, and
        // BitVec<N> all flow through without explicit casts in generated code.
        ("fn UInt(x: impl AslValue) -> i128",              "x.to_u128() as i128"),
        ("fn SInt(x: impl AslValue) -> i128",              "{ let w = x.asl_bit_width(); let v = x.to_u128(); if w == 0 || (v >> (w - 1)) & 1 == 0 { v as i128 } else { (v | !((1u128 << w).wrapping_sub(1))) as i128 } }"),
        ("fn IsZero(x: impl AslValue) -> bool",            "x.to_u128() == 0"),
        ("fn IsOnes(x: impl AslValue) -> bool",            "x.to_u64() == u64::MAX"),
        ("fn Zeros(_n: impl AslValue) -> i128",            "0"),
        ("fn Ones(n: impl AslValue) -> i128",              "{ let n = n.to_u64(); if n >= 64 { u64::MAX as i128 } else { ((1u64 << n) - 1) as i128 } }"),
        ("fn ZeroExtend(x: impl AslValue, _n: impl AslValue) -> i128", "x.to_u128() as i128"),
        ("fn HaveFP16Ext() -> bool",                       "false"),
        ("fn HaveBF16Ext() -> bool",                       "false"),
        ("fn HaveSVE() -> bool",                           "false"),
        ("fn HaveSVE2() -> bool",                          "false"),
        ("fn HaveMTE() -> bool",                           "false"),
        ("fn asl_mod(a: i128, b: i128) -> i128",           "((a % b) + b) % b"),
    ] {
        emitter.emit("#[allow(non_snake_case, dead_code)]");
        emitter.emit(&format!("pub {} {{ {} }}", sig, body));
    }
    emitter.emit("");

    // CPU state struct
    emitter.emit("// ── CPU state ───────────────────────────────────────────────────────────────");
    emitter.emit("pub struct CpuState {");
    emitter.indent();
    emitter.emit("pub X: [u64; 32],");   // AArch64 64-bit GPRs
    emitter.emit("pub R: [u64; 16],");   // AArch32 32-bit GPRs
    emitter.emit("pub S: [u64; 32],");   // VFP single-precision
    emitter.emit("pub VD: [u64; 32],");  // VFP double-precision (D renamed to avoid clash with PSTATE.D)
    emitter.emit("pub SP: u64,");
    emitter.emit("pub PC: u64,");
    // PSTATE flags
    emitter.emit("pub N: bool,");
    emitter.emit("pub Z: bool,");
    emitter.emit("pub C: bool,");
    emitter.emit("pub V: bool,");
    emitter.emit("pub EL: u8,");
    emitter.emit("pub M: u8,");
    emitter.emit("pub T: bool,");
    emitter.emit("pub nRW: bool,");
    emitter.emit("pub SS: bool,");
    emitter.emit("pub IL: bool,");
    emitter.emit("pub D: bool,");
    emitter.emit("pub A: bool,");
    emitter.emit("pub I: bool,");
    emitter.emit("pub F: bool,");
    emitter.dedent();
    emitter.emit("}");
    emitter.emit("impl CpuState {");
    emitter.indent();
    emitter.emit("pub fn new() -> Self {");
    emitter.indent();
    emitter.emit("CpuState { X: [0u64; 32], R: [0u64; 16], S: [0u64; 32], VD: [0u64; 32], SP: 0, PC: 0,");
    emitter.emit("    N: false, Z: false, C: false, V: false,");
    emitter.emit("    EL: 0, M: 0, T: false, nRW: false,");
    emitter.emit("    SS: false, IL: false, D: false, A: false, I: false, F: false }");
    emitter.dedent();
    emitter.emit("}");
    emitter.dedent();
    emitter.emit("}");
    for (sig, body) in &[
        // Register accessors: index and value both accept any AslValue so
        // integer (i128) decode vars and BitVec<N> fields flow without casts.
        // Read accessors return i128 (ASL integer) rather than u64 so the
        // result can be stored directly in integer or bits(N) decoded vars.
        ("fn Xreg(cpu: &CpuState, n: impl AslValue) -> i128",              "cpu.X[n.to_u64() as usize] as i128"),
        ("fn Wreg(cpu: &CpuState, n: impl AslValue) -> i128",              "(cpu.X[n.to_u64() as usize] & 0xFFFF_FFFF) as i128"),
        ("fn set_Xreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue)", "cpu.X[n.to_u64() as usize] = val.to_u64()"),
        ("fn set_Wreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue)", "cpu.X[n.to_u64() as usize] = val.to_u64() & 0xFFFF_FFFF"),
        ("fn Rreg(cpu: &CpuState, n: impl AslValue) -> i128",              "cpu.R[n.to_u64() as usize] as i128"),
        ("fn set_Rreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue)", "cpu.R[n.to_u64() as usize] = val.to_u64() & 0xFFFF_FFFF"),
        ("fn Sreg(cpu: &CpuState, n: impl AslValue) -> i128",              "cpu.S[n.to_u64() as usize] as i128"),
        ("fn set_Sreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue)", "cpu.S[n.to_u64() as usize] = val.to_u64()"),
        ("fn Dreg(cpu: &CpuState, n: impl AslValue) -> i128",              "cpu.VD[n.to_u64() as usize] as i128"),
        ("fn set_Dreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue)", "cpu.VD[n.to_u64() as usize] = val.to_u64()"),
        ("fn check_condition(cpu: &CpuState) -> bool",
         "true /* TODO: evaluate CPSR/PSTATE condition codes against N/Z/C/V */"),
    ] {
        emitter.emit("#[allow(non_snake_case, dead_code)]");
        emitter.emit(&format!("pub {} {{ {} }}", sig, body));
    }
    emitter.emit("");

    // Branch type constants (ASL enum BranchType → named i128 constants).
    // Generated call sites use these as bare names (dotted `BranchType.INDIR` → `BranchType_INDIR`).
    emitter.emit("// ── Branch type constants ───────────────────────────────────────────────────");
    for (name, val) in &[
        ("BranchType_INDIR",    0i32),
        ("BranchType_DIR",      1),
        ("BranchType_DIRCALL",  2),
        ("BranchType_INDIRCALL",3),
        ("BranchType_ERET",     4),
        ("BranchType_DBGEXIT",  5),
    ] {
        emitter.emit(&format!("#[allow(non_upper_case_globals, dead_code)]"));
        emitter.emit(&format!("pub const {}: i128 = {};", name, val));
    }
    emitter.emit("");

    // High-level ARM operation stubs.
    // These would normally update CpuState but call sites in generated execute bodies
    // don't pass `cpu` (generated as standalone calls).  Stubs are no-ops so the file
    // compiles; real implementations can be wired in when register access (Step D) lands.
    emitter.emit("// ── ARM operation stubs ─────────────────────────────────────────────────────");
    for (sig, body) in &[
        // PC-write stubs (branch instructions)
        ("fn BXWritePC(_addr: impl AslValue, _btype: i128)",     "/* TODO: cpu.PC = addr */"),
        ("fn ALUWritePC(_result: impl AslValue)",                "/* TODO: cpu.PC = result */"),
        ("fn BranchWritePC(_addr: impl AslValue)",               "/* TODO: cpu.PC = addr */"),
        ("fn BranchTo(_addr: impl AslValue, _btype: i128)",      "/* TODO: cpu.PC = addr */"),
        ("fn LoadWritePC(_addr: impl AslValue)",                  "/* TODO: cpu.PC = addr */"),
        // PSR access stubs (MRS / MSR instructions)
        ("fn GetPSRFromPSTATE() -> i128",                        "0 /* TODO: pack PSTATE fields into PSR */"),
        ("fn SPSRWriteByInstr(_val: impl AslValue, _mask: impl AslValue)", "/* TODO: write SPSR */"),
        ("fn CPSRWriteByInstr(_val: impl AslValue, _mask: impl AslValue)", "/* TODO: write CPSR/PSTATE */"),
        // Enabled / validity checks (decode guards)
        ("fn CheckVFPEnabled(_exc_on_failure: impl AslValue)",   "/* TODO: check VFP enabled */"),
        ("fn CheckAdvSIMDOrFPEnabled(_exc: impl AslValue)",       "/* TODO */"),
        ("fn AArch64_BranchTo(_addr: impl AslValue, _btype: i128)", "/* TODO: cpu.PC = addr */"),
    ] {
        emitter.emit("#[allow(non_snake_case, dead_code)]");
        emitter.emit(&format!("pub {} {{ {} }}", sig, body));
    }
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
                                    let ty = if rhs_text.contains("==") || rhs_text.contains("!=") {
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

        // Field extraction (mut so decode block can reassign them)
        for (name, begin, len) in &fields {
            let mask = (1u64 << len) - 1;
            emitter.emit(&format!(
                "let mut {}: u64 = (bits >> {}) & 0x{:X};",
                name, begin, mask
            ));
        }

        // Guard check — evaluated after field extraction since guard may reference fields
        if let Some(guard) = enc.expr() {
            let guard_str = generate_expr(&guard);
            emitter.emit(&format!("if !({}) {{ return None; }}", guard_str));
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

    // Emit one #[test] per encoding that exercises decode → execute end-to-end.
    // Uses fixed_bits (all don't-care bits = 0) as the test opcode.
    // Wrapped in `if let Some` because the guard expression may reject all-zero don't-care fields.
    if !test_cases.is_empty() {
        emitter.emit("");
        emitter.emit("#[cfg(test)]");
        emitter.emit(&format!("mod tests_{} {{", instr_name_safe));
        emitter.indent();
        emitter.emit("use super::*;");
        for (enc_name, fixed_bits) in &test_cases {
            emitter.emit("");
            emitter.emit("#[test]");
            emitter.emit(&format!("fn decode_execute_{}() {{", enc_name));
            emitter.indent();
            emitter.emit(&format!("let bits: u64 = 0x{:016X};", fixed_bits));
            emitter.emit(&format!("if let Some(enc) = {}::decode(bits) {{", enc_name));
            emitter.indent();
            emitter.emit("let mut cpu = CpuState::new();");
            emitter.emit(&format!("execute_{}(&enc, &mut cpu);", instr_name_safe));
            emitter.dedent();
            emitter.emit("}");
            emitter.dedent();
            emitter.emit("}");
        }
        emitter.dedent();
        emitter.emit("}");
    }
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

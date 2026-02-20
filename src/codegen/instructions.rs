use std::rc::Rc;
use antlr_rust::tree::ParseTree;
use antlr_rust::token::Token;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::expressions::generate_expr;
use crate::codegen::statements::generate_stmt;
use crate::parser::aslparser::{
    InstructionContextAll,
    InstructionContextAttrs,
    EncodingContextAttrs,
    InstructionFieldContextAttrs,
    IndentedBlockContextAttrs,
};

pub fn generate_instruction(emitter: &mut CodeEmitter, instr: &Rc<InstructionContextAll<'_>>) {
    let instr_name = instr.idWithDots().unwrap().get_text();
    let instr_name_safe = instr_name.replace('.', "_");

    emitter.emit(&format!("// Instruction: {}", instr_name));

    for enc in instr.encoding_all() {
        let enc_name = enc.idWithDots().unwrap().get_text();
        let enc_name_safe = enc_name.replace('.', "_");
        let inst_set = enc.instructionSet.as_ref().map(|t| t.get_text()).unwrap_or("unknown");

        emitter.emit("");
        emitter.emit(&format!("// Encoding: {} ({})", enc_name, inst_set));

        // Collect fields: (name, start_bit, bit_length)
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

        // Generate struct
        emitter.emit(&format!("pub struct {} {{", enc_name_safe));
        emitter.indent();
        for (name, _, _) in &fields {
            emitter.emit(&format!("pub {}: u64,", name));
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

        // Decode block statements
        if let Some(block) = &enc.decode {
            for stmt in block.stmt_all() {
                let deferred = generate_stmt(emitter, &stmt);
                for d in deferred {
                    generate_stmt(emitter, &d);
                }
            }
        }

        // Return constructed value
        if fields.is_empty() {
            emitter.emit("Some(Self {})");
        } else {
            let field_names: Vec<String> = fields.iter().map(|(n, _, _)| n.clone()).collect();
            emitter.emit(&format!("Some(Self {{ {} }})", field_names.join(", ")));
        }

        emitter.dedent();
        emitter.emit("}");
        emitter.dedent();
        emitter.emit("}");
    }

    // Postdecode function (shared logic run after any encoding's decode)
    if let Some(block) = &instr.postDecodeBlock {
        emitter.emit("");
        emitter.emit(&format!("// Postdecode: {}", instr_name));
        emitter.emit(&format!("pub fn postdecode_{}() {{", instr_name_safe));
        emitter.indent();
        for stmt in block.stmt_all() {
            let deferred = generate_stmt(emitter, &stmt);
            for d in deferred {
                generate_stmt(emitter, &d);
            }
        }
        emitter.dedent();
        emitter.emit("}");
    }

    // Execute function
    emitter.emit("");
    emitter.emit(&format!("// Execute: {}", instr_name));
    if instr.conditional.is_some() {
        emitter.emit("// __conditional: instruction is conditionally executed");
    }
    emitter.emit(&format!(
        "pub fn execute_{}(/* TODO: cpu state */) {{",
        instr_name_safe
    ));
    emitter.indent();
    if let Some(block) = &instr.executeBlock {
        let stmts = block.stmt_all();
        if stmts.is_empty() {
            emitter.emit("// empty execute block");
        } else {
            for stmt in &stmts {
                let deferred = generate_stmt(emitter, &stmt);
                for d in deferred {
                    generate_stmt(emitter, &d);
                }
            }
        }
    } else {
        emitter.emit(&format!("todo!(\"execute {}\");", instr_name));
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

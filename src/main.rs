mod parser;
mod codegen;

use std::{collections::HashSet, env, fs, path::Path};

/// The bitvec runtime library source, embedded at compile time.
/// Written to the output directory alongside any generated instructions file.
const BITVEC_RS: &str = include_str!("runtime/bitvec.rs");

/// The ASL runtime (stubs, CpuState, register accessors), embedded at compile time.
/// Written to the output directory alongside any generated instructions file.
const RUNTIME_RS: &str = include_str!("runtime/runtime.rs");
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

use parser::{aslLexer, aslParser};
use parser::aslparser::*;
use codegen::CodeEmitter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <mode> <input.asl> <output.rs>", args[0]);
        eprintln!("Modes: registers, definitions, instructions");
        std::process::exit(1);
    }

    let mode = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let input = fs::read_to_string(input_file)
        .expect(&format!("Failed to read {}", input_file));

    let input_stream = InputStream::new(input.as_str());
    let lexer = aslLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = aslParser::new(token_stream);

    let mut emitter = CodeEmitter::new();

    match mode.as_str() {
        "registers" => {
            let tree = parser.registers().expect("Parse failed");
            for reg_def in tree.registerDefinition_all() {
                match reg_def.as_ref() {
                    RegisterDefinitionContextAll::RegDefBasicContext(ctx) => {
                        if let Some(reg) = ctx.register() {
                            codegen::registers::generate_register(&mut emitter, &reg);
                        }
                    }
                    RegisterDefinitionContextAll::RegDefArrayContext(ctx) => {
                        if let Some(arr) = ctx.arrayRegister() {
                            codegen::registers::generate_array_register(&mut emitter, &arr);
                        }
                    }
                    _ => {}
                }
            }
        }
        "definitions" => {
            let tree = parser.definitions().expect("Parse failed");
            let mut seen: HashSet<String> = HashSet::new();
            for def in tree.definition_all() {
                // Skip duplicate definitions (same kind + name).
                // The real ARM spec redefines the same enum/type multiple times across files.
                if let Some(key) = def_key(&def) {
                    if !seen.insert(key.clone()) {
                        emitter.emit(&format!("// skipped duplicate: {}", key));
                        continue;
                    }
                }
                match def.as_ref() {
                    DefinitionContextAll::DefTypeBuiltinContext(ctx) => {
                        codegen::types::generate_builtin_type(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefConstantContext(ctx) => {
                        codegen::variables::generate_constant(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefTypeAbstractContext(ctx) => {
                        codegen::types::generate_abstract_type(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefTypeAliasContext(ctx) => {
                        codegen::types::generate_type_alias(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefTypeEnumContext(ctx) => {
                        codegen::types::generate_enum(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefTypeStructContext(ctx) => {
                        codegen::types::generate_struct(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefVariableContext(ctx) => {
                        codegen::variables::generate_variable(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefArrayContext(ctx) => {
                        codegen::variables::generate_array(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefCallableContext(ctx) => {
                        codegen::functions::generate_callable(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefGetterContext(ctx) => {
                        codegen::functions::generate_getter(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefSetterContext(ctx) => {
                        codegen::functions::generate_setter(&mut emitter, ctx);
                    }
                    _ => {
                        emitter.emit(&format!("// TODO: {:?}", def.get_text()));
                    }
                }
            }
        }
        "instructions" => {
            codegen::instructions::generate_asl_runtime(&mut emitter);
            let tree = parser.instructions().expect("Parse failed");
            for instr in tree.instruction_all() {
                codegen::instructions::generate_instruction(&mut emitter, &instr);
            }
        }
        _ => {
            eprintln!("Unknown mode: {}", mode);
            std::process::exit(1);
        }
    }

    if let Some(parent) = Path::new(output_file).parent() {
        fs::create_dir_all(parent).ok();
    }

    fs::write(output_file, emitter.output())
        .expect(&format!("Failed to write {}", output_file));

    println!("Generated: {}", output_file);

    // In instructions mode the generated file references `mod bitvec` and `mod runtime`,
    // so write both alongside the output file.
    if mode == "instructions" {
        let out_dir = Path::new(output_file).parent().unwrap_or(Path::new("."));
        let bitvec_path = out_dir.join("bitvec.rs");
        fs::write(&bitvec_path, BITVEC_RS)
            .expect("Failed to write bitvec.rs");
        println!("Generated: {}", bitvec_path.display());
        let runtime_path = out_dir.join("runtime.rs");
        fs::write(&runtime_path, RUNTIME_RS)
            .expect("Failed to write runtime.rs");
        println!("Generated: {}", runtime_path.display());
    }
}

/// Returns a `"kind:name"` key for a definition, used to detect duplicates.
/// Returns `None` for definitions that have no meaningful unique name.
fn def_key(def: &DefinitionContextAll) -> Option<String> {
    use antlr_rust::tree::ParseTree;
    match def {
        DefinitionContextAll::DefTypeBuiltinContext(ctx) =>
            ctx.id().map(|id| format!("builtin:{}", id.get_text())),
        DefinitionContextAll::DefTypeAbstractContext(ctx) =>
            ctx.id().map(|id| format!("type:{}", id.get_text())),
        DefinitionContextAll::DefTypeAliasContext(ctx) =>
            ctx.id().map(|id| format!("alias:{}", id.get_text())),
        DefinitionContextAll::DefTypeEnumContext(ctx) =>
            ctx.id().map(|id| format!("enum:{}", id.get_text())),
        DefinitionContextAll::DefTypeStructContext(ctx) =>
            ctx.qualId().map(|q| format!("struct:{}", q.get_text())),
        DefinitionContextAll::DefConstantContext(ctx) =>
            ctx.id().map(|id| format!("const:{}", id.get_text())),
        DefinitionContextAll::DefArrayContext(ctx) =>
            ctx.id().map(|id| format!("array:{}", id.get_text())),
        DefinitionContextAll::DefVariableContext(ctx) =>
            ctx.qualId().map(|q| format!("var:{}", q.get_text())),
        // Functions/getters/setters are keyed by name — overloads are a separate issue (#24)
        DefinitionContextAll::DefCallableContext(ctx) =>
            ctx.qualId().map(|q| format!("fn:{}", q.get_text())),
        DefinitionContextAll::DefGetterContext(ctx) =>
            ctx.qualId().map(|q| format!("getter:{}", q.get_text())),
        DefinitionContextAll::DefSetterContext(ctx) =>
            ctx.qualId().map(|q| format!("setter:{}", q.get_text())),
        _ => None,
    }
}

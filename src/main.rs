mod parser;
mod codegen;

use std::{env, fs, path::Path};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

use parser::{aslLexer, aslParser};
use parser::aslparser::*;
use codegen::CodeEmitter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <mode> <input.asl> <output.rs>", args[0]);
        eprintln!("Modes: registers, definitions");
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
            for def in tree.definition_all() {
                match def.as_ref() {
                    DefinitionContextAll::DefTypeAliasContext(ctx) => {
                        codegen::types::generate_type_alias(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefTypeEnumContext(ctx) => {
                        codegen::types::generate_enum(&mut emitter, ctx);
                    }
                    DefinitionContextAll::DefTypeStructContext(ctx) => {
                        codegen::types::generate_struct(&mut emitter, ctx);
                    }
                    _ => {
                        emitter.emit(&format!("// TODO: {:?}", def.get_text()));
                    }
                }
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
}

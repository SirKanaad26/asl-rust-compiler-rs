mod parser;
mod codegen;

use std::{env, fs, path::Path};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

use parser::{aslLexer, aslParser};
use parser::aslparser::{
    RegistersContextAttrs,
    RegisterDefinitionContextAll,
    RegDefBasicContextAttrs,
    RegDefArrayContextAttrs,  // Add this
};

use codegen::CodeEmitter;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <input.asl> <output.rs>", args[0]);
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let output_file = &args[2];

    let input = fs::read_to_string(input_file)
        .expect(&format!("Failed to read {}", input_file));

    let input_stream = InputStream::new(input.as_str());
    let lexer = aslLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = aslParser::new(token_stream);

    let tree = parser.registers().expect("Parse failed");
    let reg_defs = tree.registerDefinition_all();

    let mut emitter = CodeEmitter::new();

    for reg_def in reg_defs {
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

    if let Some(parent) = Path::new(output_file).parent() {
        fs::create_dir_all(parent).ok();
    }

    fs::write(output_file, emitter.output())
        .expect(&format!("Failed to write {}", output_file));
    
    println!("Generated: {}", output_file);
}

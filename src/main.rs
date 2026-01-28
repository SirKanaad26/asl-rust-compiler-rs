mod parser;
mod codegen;

use std::fs;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

use parser::{aslLexer, aslParser};
use parser::aslparser::{
    RegistersContextAttrs,
    RegisterDefinitionContextAll,
    RegDefBasicContextAttrs,
};
use codegen::CodeEmitter;

fn main() {
    let input = fs::read_to_string("examples/register_bitfield.asl")
        .expect("Failed to read file");

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
            RegisterDefinitionContextAll::RegDefArrayContext(_) => {
                emitter.emit("// TODO: array register");
            }
            _ => {}
        }
    }

    println!("{}", emitter.output());
}

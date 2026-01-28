mod parser;

use std::fs;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::InputStream;

use parser::{aslLexer, aslParser};
use parser::aslparser::{
    RegistersContextAttrs,
    RegisterDefinitionContextAll,
    RegDefBasicContextAttrs,
    RegisterContextAttrs,
};

fn main() {
    let input = fs::read_to_string("examples/minimal.asl")
        .expect("Failed to read file");

    let input_stream = InputStream::new(input.as_str());
    let lexer = aslLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = aslParser::new(token_stream);

    let tree = parser.registers().expect("Parse failed");

    let reg_defs = tree.registerDefinition_all();
    
    for reg_def in reg_defs {
        match reg_def.as_ref() {
            RegisterDefinitionContextAll::RegDefBasicContext(ctx) => {
                // Get the register context
                if let Some(reg) = ctx.register() {
                    // Get size (NAT_LIT)
                    let size = reg.NAT_LIT().unwrap().get_text();
                    
                    // Get name (id)
                    let name = reg.id().unwrap().get_text();
                    
                    // Map to Rust type
                    let rust_type = match size.as_str() {
                        "8" => "u8",
                        "16" => "u16",
                        "32" => "u32",
                        "64" => "u64",
                        "128" => "u128",
                        _ => "u64",
                    };
                    
                    println!("pub struct {}({});", name, rust_type);
                }
            }
            RegisterDefinitionContextAll::RegDefArrayContext(_ctx) => {
                println!("// TODO: array register");
            }
            _ => {}
        }
    }
}

mod parser;

use std::fs;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;

use parser::aslLexer;
use parser::aslParser;

fn main() {
    // Read the input file
    let input = fs::read_to_string("examples/minimal.asl")
        .expect("Failed to read file");

    // Create lexer
    let input_stream = InputStream::new(input.as_str());
    let lexer = aslLexer::new(input_stream);
    
    // Create token stream and parser
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = aslParser::new(token_stream);

    // Parse using 'registers' rule
    let result = parser.registers();

    match result {
        Ok(_tree) => {
            println!("Parsed successfully!");
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
}

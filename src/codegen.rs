use crate::parser::*;
use antlr_rust::tree::ParseTreeListener;

pub struct RustCodeGenerator {
    pub output: String,
    indent: usize,
}

impl RustCodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    fn emit(&mut self, line: &str) {
        self.output.push_str(&"    ".repeat(self.indent));
        self.output.push_str(line);
        self.output.push('\n');
    }
}

impl<'input> ParseTreeListener<'input, aslParserContextType> for RustCodeGenerator {}

impl<'input> aslListener<'input> for RustCodeGenerator {
    fn exit_register(&mut self, ctx: &RegisterContext<'input>) {
        // Get register size (e.g., 32)
        let size = ctx.NAT_LIT().unwrap().get_text();
        
        // Get register name (e.g., EXAMPLE_REG)
        let name = ctx.id().unwrap().get_text();
        
        // Map bit size to Rust type
        let rust_type = match size.as_str() {
            "8" => "u8",
            "16" => "u16",
            "32" => "u32",
            "64" => "u64",
            "128" => "u128",
            _ => "u64",
        };

        self.emit(&format!("pub struct {}({});", name, rust_type));
    }
}

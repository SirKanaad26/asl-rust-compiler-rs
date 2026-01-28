pub struct CodeEmitter {
    output: String,
    indent: usize,
}

impl CodeEmitter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    pub fn emit(&mut self, line: &str) {
        self.output.push_str(&"    ".repeat(self.indent));
        self.output.push_str(line);
        self.output.push('\n');
    }

    pub fn emit_raw(&mut self, text: &str) {
        self.output.push_str(text);
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    pub fn output(&self) -> &str {
        &self.output
    }
}

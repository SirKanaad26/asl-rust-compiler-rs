pub struct CodeEmitter {
    lines: Vec<String>,
    indent: usize,
}

impl CodeEmitter {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            indent: 0,
        }
    }

    pub fn emit(&mut self, line: &str) {
        let indented = format!("{}{}", "    ".repeat(self.indent), line);
        self.lines.push(indented);
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    pub fn output(&self) -> String {
        let mut result = self.lines.join("\n");
        if !result.is_empty() {
            result.push('\n');
        }
        result
    }
}

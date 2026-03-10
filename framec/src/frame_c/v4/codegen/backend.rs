//! Language Backend Trait
//!
//! This module defines the LanguageBackend trait that all language-specific
//! code generators must implement.

use crate::frame_c::visitors::TargetLanguage;
use super::ast::{CodegenNode, Literal, BinaryOp, UnaryOp, Visibility};

/// Emit context tracks state during code generation
#[derive(Debug, Clone)]
pub struct EmitContext {
    /// Current indentation level
    pub indent: usize,
    /// Indentation string (e.g., "    " or "\t")
    pub indent_str: String,
    /// Current system name
    pub system_name: Option<String>,
    /// Whether we're in a method body
    pub in_method: bool,
    /// Whether we're in a class
    pub in_class: bool,
    /// Output buffer
    pub output: String,
}

impl Default for EmitContext {
    fn default() -> Self {
        Self {
            indent: 0,
            indent_str: "    ".to_string(),
            system_name: None,
            in_method: false,
            in_class: false,
            output: String::new(),
        }
    }
}

impl EmitContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_indent(mut self, indent_str: &str) -> Self {
        self.indent_str = indent_str.to_string();
        self
    }

    pub fn with_system(mut self, name: &str) -> Self {
        self.system_name = Some(name.to_string());
        self
    }

    /// Get current indentation string
    pub fn get_indent(&self) -> String {
        self.indent_str.repeat(self.indent)
    }

    /// Increase indentation
    pub fn push_indent(&mut self) {
        self.indent += 1;
    }

    /// Decrease indentation
    pub fn pop_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }

    /// Write indented line
    pub fn write_line(&mut self, line: &str) {
        self.output.push_str(&self.get_indent());
        self.output.push_str(line);
        self.output.push('\n');
    }

    /// Write without indent
    pub fn write(&mut self, text: &str) {
        self.output.push_str(text);
    }

    /// Write a blank line
    pub fn blank_line(&mut self) {
        self.output.push('\n');
    }
}

/// Class syntax configuration for different languages
#[derive(Debug, Clone)]
pub struct ClassSyntax {
    /// Target language
    pub language: TargetLanguage,
    /// Keyword for class (e.g., "class", "struct")
    pub class_keyword: String,
    /// How to specify inheritance (e.g., "extends", ":")
    pub extends_keyword: Option<String>,
    /// Self/this keyword (e.g., "self", "this")
    pub self_keyword: String,
    /// Constructor name (e.g., "__init__", "constructor", "new")
    pub constructor_name: String,
    /// Whether fields need explicit declaration
    pub explicit_fields: bool,
    /// Whether methods need `def` or `fn` keyword
    pub method_keyword: Option<String>,
    /// Block start (e.g., "{" or ":")
    pub block_start: String,
    /// Block end (e.g., "}" or empty for Python)
    pub block_end: String,
    /// Statement terminator (e.g., ";" or empty)
    pub statement_terminator: String,
    /// Type annotation separator (e.g., ":" or "->")
    pub type_sep: String,
    /// Return type separator
    pub return_type_sep: String,
}

impl ClassSyntax {
    pub fn python() -> Self {
        Self {
            language: TargetLanguage::Python3,
            class_keyword: "class".to_string(),
            extends_keyword: None,
            self_keyword: "self".to_string(),
            constructor_name: "__init__".to_string(),
            explicit_fields: false,
            method_keyword: Some("def".to_string()),
            block_start: ":".to_string(),
            block_end: String::new(),
            statement_terminator: String::new(),
            type_sep: ": ".to_string(),
            return_type_sep: " -> ".to_string(),
        }
    }

    pub fn typescript() -> Self {
        Self {
            language: TargetLanguage::TypeScript,
            class_keyword: "class".to_string(),
            extends_keyword: Some("extends".to_string()),
            self_keyword: "this".to_string(),
            constructor_name: "constructor".to_string(),
            explicit_fields: true,
            method_keyword: None,
            block_start: "{".to_string(),
            block_end: "}".to_string(),
            statement_terminator: ";".to_string(),
            type_sep: ": ".to_string(),
            return_type_sep: ": ".to_string(),
        }
    }

    pub fn rust() -> Self {
        Self {
            language: TargetLanguage::Rust,
            class_keyword: "struct".to_string(),
            extends_keyword: None,
            self_keyword: "self".to_string(),
            constructor_name: "new".to_string(),
            explicit_fields: true,
            method_keyword: Some("fn".to_string()),
            block_start: "{".to_string(),
            block_end: "}".to_string(),
            statement_terminator: ";".to_string(),
            type_sep: ": ".to_string(),
            return_type_sep: " -> ".to_string(),
        }
    }

    pub fn java() -> Self {
        Self {
            language: TargetLanguage::Java,
            class_keyword: "class".to_string(),
            extends_keyword: Some("extends".to_string()),
            self_keyword: "this".to_string(),
            constructor_name: String::new(), // Uses class name
            explicit_fields: true,
            method_keyword: None,
            block_start: "{".to_string(),
            block_end: "}".to_string(),
            statement_terminator: ";".to_string(),
            type_sep: " ".to_string(), // Java: "Type name" not "name: Type"
            return_type_sep: " ".to_string(),
        }
    }

    pub fn csharp() -> Self {
        Self {
            language: TargetLanguage::CSharp,
            class_keyword: "class".to_string(),
            extends_keyword: Some(":".to_string()),
            self_keyword: "this".to_string(),
            constructor_name: String::new(), // Uses class name
            explicit_fields: true,
            method_keyword: None,
            block_start: "{".to_string(),
            block_end: "}".to_string(),
            statement_terminator: ";".to_string(),
            type_sep: " ".to_string(),
            return_type_sep: " ".to_string(),
        }
    }

    pub fn c() -> Self {
        Self {
            language: TargetLanguage::C,
            class_keyword: "struct".to_string(),
            extends_keyword: None,
            self_keyword: "self".to_string(),
            constructor_name: "_init".to_string(),
            explicit_fields: true,
            method_keyword: None,
            block_start: "{".to_string(),
            block_end: "}".to_string(),
            statement_terminator: ";".to_string(),
            type_sep: " ".to_string(),
            return_type_sep: " ".to_string(),
        }
    }

    pub fn cpp() -> Self {
        Self {
            language: TargetLanguage::Cpp,
            class_keyword: "class".to_string(),
            extends_keyword: Some(":".to_string()),
            self_keyword: "this".to_string(),
            constructor_name: String::new(), // Uses class name
            explicit_fields: true,
            method_keyword: None,
            block_start: "{".to_string(),
            block_end: "}".to_string(),
            statement_terminator: ";".to_string(),
            type_sep: " ".to_string(),
            return_type_sep: " ".to_string(),
        }
    }
}

/// Trait that all language-specific code generators must implement
pub trait LanguageBackend: Send + Sync {
    /// Emit code for a CodegenNode
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String;

    /// Get runtime imports needed for Frame state machines
    fn runtime_imports(&self) -> Vec<String>;

    /// Get class/struct syntax for this language
    fn class_syntax(&self) -> ClassSyntax;

    /// Get the target language
    fn target_language(&self) -> TargetLanguage;

    /// Emit a literal value
    fn emit_literal(&self, lit: &Literal, _ctx: &mut EmitContext) -> String {
        match lit {
            Literal::Int(n) => n.to_string(),
            Literal::Float(f) => format!("{}", f),
            Literal::String(s) => format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\"")),
            Literal::Bool(b) => if *b { self.true_keyword() } else { self.false_keyword() }.to_string(),
            Literal::Null => self.null_keyword().to_string(),
        }
    }

    /// Emit binary operator
    fn emit_binary_op(&self, op: &BinaryOp, left: &CodegenNode, right: &CodegenNode, ctx: &mut EmitContext) -> String {
        let left_str = self.emit(left, ctx);
        let right_str = self.emit(right, ctx);
        let op_str = match op {
            BinaryOp::And => self.and_operator(),
            BinaryOp::Or => self.or_operator(),
            _ => op.as_str(),
        };
        format!("{} {} {}", left_str, op_str, right_str)
    }

    /// Emit unary operator
    fn emit_unary_op(&self, op: &UnaryOp, operand: &CodegenNode, ctx: &mut EmitContext) -> String {
        let operand_str = self.emit(operand, ctx);
        let op_str = match op {
            UnaryOp::Not => self.not_operator(),
            _ => op.as_str(),
        };
        format!("{}{}", op_str, operand_str)
    }

    /// Emit visibility modifier
    fn emit_visibility(&self, vis: Visibility) -> &'static str {
        match vis {
            Visibility::Public => "public",
            Visibility::Private => "private",
            Visibility::Protected => "protected",
        }
    }

    // Language-specific keywords (with defaults)

    fn true_keyword(&self) -> &'static str { "true" }
    fn false_keyword(&self) -> &'static str { "false" }
    fn null_keyword(&self) -> &'static str { "null" }
    fn and_operator(&self) -> &'static str { "&&" }
    fn or_operator(&self) -> &'static str { "||" }
    fn not_operator(&self) -> &'static str { "!" }
}

/// Get the appropriate backend for a target language
pub fn get_backend(lang: TargetLanguage) -> Box<dyn LanguageBackend> {
    use super::backends::*;

    match lang {
        TargetLanguage::Python3 => Box::new(python::PythonBackend),
        TargetLanguage::TypeScript => Box::new(typescript::TypeScriptBackend),
        TargetLanguage::Rust => Box::new(rust::RustBackend),
        TargetLanguage::CSharp => Box::new(csharp::CSharpBackend),
        TargetLanguage::C => Box::new(c::CBackend),
        TargetLanguage::Cpp => Box::new(cpp::CppBackend),
        TargetLanguage::Java => Box::new(java::JavaBackend),
        _ => panic!("No CodegenNode backend for {:?} — use target-specific pipeline instead", lang),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_context_indent() {
        let mut ctx = EmitContext::new();
        assert_eq!(ctx.get_indent(), "");

        ctx.push_indent();
        assert_eq!(ctx.get_indent(), "    ");

        ctx.push_indent();
        assert_eq!(ctx.get_indent(), "        ");

        ctx.pop_indent();
        assert_eq!(ctx.get_indent(), "    ");
    }

    #[test]
    fn test_class_syntax_python() {
        let syntax = ClassSyntax::python();
        assert_eq!(syntax.self_keyword, "self");
        assert_eq!(syntax.constructor_name, "__init__");
        assert_eq!(syntax.block_start, ":");
    }

    #[test]
    fn test_class_syntax_typescript() {
        let syntax = ClassSyntax::typescript();
        assert_eq!(syntax.self_keyword, "this");
        assert_eq!(syntax.constructor_name, "constructor");
        assert_eq!(syntax.block_start, "{");
    }
}

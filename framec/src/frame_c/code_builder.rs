// Frame Code Builder - Robust line-aware code generation with automatic source mapping
//
// This module provides a proper architecture for code generation that automatically
// tracks line numbers and maintains source mappings without manual offset adjustments.

// Reserved for future use with ordered fragment composition
// use std::collections::BTreeMap;

/// Position in the generated code
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }
}

/// A single source mapping entry
#[derive(Debug, Clone)]
pub struct SourceMapping {
    pub frame_line: usize,
    pub python_line: usize,
    pub python_column: Option<usize>,
    pub mapping_type: Option<crate::frame_c::source_map::MappingType>,
}

/// Code fragment with associated metadata
#[derive(Debug, Clone)]
pub struct CodeFragment {
    pub content: String,
    pub frame_line: Option<usize>,
    pub fragment_type: FragmentType,
    pub indent_level: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FragmentType {
    Function,
    Statement,
    Expression,
    Comment,
    Whitespace,
    Block,
}

/// Smart code builder that tracks every character and maintains perfect mappings
pub struct CodeBuilder {
    // Current output being built
    output: String,

    // Current position in the output (1-based line numbers)
    current_position: Position,

    // All source mappings collected
    mappings: Vec<SourceMapping>,

    // Current indentation level (number of indent units)
    indent_level: usize,

    // Indentation string (e.g., "    " for 4 spaces)
    indent_str: String,

    // Track if we're at the start of a line (for auto-indent)
    at_line_start: bool,

    // Deferred mapping - set this before writing code to map it
    pending_mapping: Option<(usize, Option<crate::frame_c::source_map::MappingType>)>,

    // Fragment stack for non-linear generation
    fragment_stack: Vec<CodeFragment>,

    // Enable debug output
    debug: bool,
}

impl CodeBuilder {
    /// Create a new code builder with specified indentation
    pub fn new(indent_str: &str) -> Self {
        CodeBuilder {
            output: String::new(),
            current_position: Position::new(1, 0),
            mappings: Vec::new(),
            indent_level: 0,
            indent_str: indent_str.to_string(),
            at_line_start: true,
            pending_mapping: None,
            fragment_stack: Vec::new(),
            debug: std::env::var("FRAME_CODEBUILDER_DEBUG").is_ok(),
        }
    }

    /// Set the source line for the next code to be written
    pub fn map_next(&mut self, frame_line: usize) -> &mut Self {
        self.pending_mapping = Some((frame_line, None));
        self
    }

    /// Set the source line and mapping type for the next code to be written
    pub fn map_next_with_type(
        &mut self,
        frame_line: usize,
        mapping_type: crate::frame_c::source_map::MappingType,
    ) -> &mut Self {
        self.pending_mapping = Some((frame_line, Some(mapping_type)));
        self
    }

    /// Write a string to the output, tracking every character
    pub fn write(&mut self, s: &str) -> &mut Self {
        if s.is_empty() {
            return self;
        }

        // Apply pending mapping if we have one
        if let Some((frame_line, mapping_type)) = self.pending_mapping.take() {
            self.add_mapping_with_type(frame_line, mapping_type);
        }

        // Process each character
        for ch in s.chars() {
            // Handle auto-indentation at line start
            if self.at_line_start && ch != '\n' {
                let indent = self.get_indent();
                for indent_ch in indent.chars() {
                    self.output.push(indent_ch);
                    self.current_position.column += 1;
                }
                self.at_line_start = false;
            }

            // Add the character
            self.output.push(ch);

            // Update position tracking
            if ch == '\n' {
                self.current_position.line += 1;
                self.current_position.column = 0;
                self.at_line_start = true;

                if self.debug {
                    eprintln!(
                        "CodeBuilder: Advanced to line {}",
                        self.current_position.line
                    );
                }
            } else {
                self.current_position.column += 1;
                self.at_line_start = false;
            }
        }

        self
    }

    /// Write a line of code (adds newline automatically)
    pub fn writeln(&mut self, s: &str) -> &mut Self {
        self.write(s);
        self.write("\n");
        self
    }

    /// Write code and map it to a Frame source line
    pub fn write_mapped(&mut self, s: &str, frame_line: usize) -> &mut Self {
        self.map_next(frame_line);
        self.write(s);
        self
    }

    /// Write a line and map it to a Frame source line
    pub fn writeln_mapped(&mut self, s: &str, frame_line: usize) -> &mut Self {
        self.map_next(frame_line);
        self.writeln(s);
        self
    }

    /// Write a line with source mapping and specific type
    pub fn writeln_mapped_with_type(
        &mut self,
        s: &str,
        frame_line: usize,
        mapping_type: crate::frame_c::source_map::MappingType,
    ) -> &mut Self {
        self.map_next_with_type(frame_line, mapping_type);
        self.writeln(s);
        self
    }

    /// Add just a newline
    pub fn newline(&mut self) -> &mut Self {
        self.write("\n");
        self
    }

    /// Increase indentation level
    pub fn indent(&mut self) -> &mut Self {
        self.indent_level += 1;
        if self.debug {
            eprintln!("CodeBuilder: Indent level now {}", self.indent_level);
        }
        self
    }

    /// Decrease indentation level
    pub fn dedent(&mut self) -> &mut Self {
        if self.indent_level > 0 {
            self.indent_level -= 1;
            if self.debug {
                eprintln!("CodeBuilder: Indent level now {}", self.indent_level);
            }
        }
        self
    }

    /// Get current indentation string
    fn get_indent(&self) -> String {
        self.indent_str.repeat(self.indent_level)
    }

    fn add_mapping_with_type(
        &mut self,
        frame_line: usize,
        mapping_type: Option<crate::frame_c::source_map::MappingType>,
    ) {
        // Check if this Frame line already has a mapping to prevent Bug #27 duplicates
        if self.mappings.iter().any(|m| m.frame_line == frame_line) {
            if self.debug {
                eprintln!(
                    "CodeBuilder: SKIPPING duplicate mapping for Frame line {} (already mapped)",
                    frame_line
                );
            }
            return;
        }

        // Clone for debug before moving into struct
        let type_str = if self.debug {
            mapping_type
                .as_ref()
                .map(|t| format!("{:?}", t))
                .unwrap_or("None".to_string())
        } else {
            String::new()
        };

        let mapping = SourceMapping {
            frame_line,
            python_line: self.current_position.line,
            python_column: Some(self.current_position.column),
            mapping_type,
        };

        if self.debug {
            eprintln!(
                "CodeBuilder: Mapping Frame line {} -> Python line {} col {} type: {}",
                frame_line, mapping.python_line, self.current_position.column, type_str
            );
        }

        self.mappings.push(mapping);
    }

    /// Push a code fragment for later composition
    pub fn push_fragment(&mut self, fragment: CodeFragment) {
        self.fragment_stack.push(fragment);
    }

    /// Create a new child builder with the same configuration
    pub fn child(&self) -> CodeBuilder {
        CodeBuilder::new(&self.indent_str)
    }

    /// Merge a child builder's output into this builder
    pub fn merge(&mut self, child: CodeBuilder, frame_line: Option<usize>) -> &mut Self {
        // Save the Frame line for the merged content
        if let Some(line) = frame_line {
            self.map_next(line);
        }

        // Merge the output
        self.write(&child.output);

        // Merge mappings, adjusting line numbers
        let line_offset =
            self.current_position.line - child.mappings.first().map(|m| m.python_line).unwrap_or(1);

        for mut mapping in child.mappings {
            mapping.python_line += line_offset;
            self.mappings.push(mapping);
        }

        self
    }

    /// Build the final output with source map
    pub fn build(self) -> (String, Vec<SourceMapping>) {
        (self.output, self.mappings)
    }

    /// Get current line number (1-based)
    pub fn current_line(&self) -> usize {
        self.current_position.line
    }

    /// Get current column number (0-based)
    pub fn current_column(&self) -> usize {
        self.current_position.column
    }

    /// Write a Python function definition with proper mapping
    pub fn write_function(
        &mut self,
        name: &str,
        params: &str,
        is_async: bool,
        frame_line: usize,
    ) -> &mut Self {
        // Only create mapping if frame_line is not 0 (0 indicates generated code)
        if frame_line > 0 {
            self.map_next(frame_line);
        }

        if is_async {
            self.write(&format!("async def {}({}):", name, params));
        } else {
            self.write(&format!("def {}({}):", name, params));
        }

        self.newline();
        self.indent();
        self
    }

    /// End a function definition
    pub fn end_function(&mut self) -> &mut Self {
        self.dedent();
        self.newline();
        self
    }

    /// Write a class definition
    pub fn write_class(
        &mut self,
        name: &str,
        base: Option<&str>,
        frame_line: Option<usize>,
    ) -> &mut Self {
        if let Some(line) = frame_line {
            self.map_next(line);
        }

        if let Some(base_class) = base {
            self.write(&format!("class {}({}):", name, base_class));
        } else {
            self.write(&format!("class {}:", name));
        }

        self.newline();
        self.indent();
        self
    }

    /// End a class definition
    pub fn end_class(&mut self) -> &mut Self {
        self.dedent();
        self.newline();
        self
    }

    /// Write a comment
    pub fn write_comment(&mut self, comment: &str) -> &mut Self {
        self.write(&format!("# {}", comment));
        self.newline();
        self
    }

    /// Write a block of code with automatic indent/dedent
    pub fn write_block<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut CodeBuilder),
    {
        self.indent();
        f(self);
        self.dedent();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_writing() {
        let mut builder = CodeBuilder::new("    ");
        builder.write("hello").write(" ").write("world").newline();

        let (output, _) = builder.build();
        assert_eq!(output, "hello world\n");
    }

    #[test]
    fn test_line_tracking() {
        let mut builder = CodeBuilder::new("    ");

        assert_eq!(builder.current_line(), 1);
        builder.writeln("line 1");
        assert_eq!(builder.current_line(), 2);
        builder.writeln("line 2");
        assert_eq!(builder.current_line(), 3);

        let (output, _) = builder.build();
        assert_eq!(output, "line 1\nline 2\n");
    }

    #[test]
    fn test_indentation() {
        let mut builder = CodeBuilder::new("    ");

        builder.writeln("no indent");
        builder.indent();
        builder.writeln("indented once");
        builder.indent();
        builder.writeln("indented twice");
        builder.dedent();
        builder.writeln("indented once again");
        builder.dedent();
        builder.writeln("no indent again");

        let (output, _) = builder.build();
        assert_eq!(
            output,
            "no indent\n    indented once\n        indented twice\n    indented once again\nno indent again\n"
        );
    }

    #[test]
    fn test_mapping() {
        let mut builder = CodeBuilder::new("    ");

        builder.writeln_mapped("def foo():", 10);
        builder.indent();
        builder.writeln_mapped("return 42", 11);
        builder.dedent();

        let (_, mappings) = builder.build();
        assert_eq!(mappings.len(), 2);
        assert_eq!(mappings[0].frame_line, 10);
        assert_eq!(mappings[0].python_line, 1);
        assert_eq!(mappings[1].frame_line, 11);
        assert_eq!(mappings[1].python_line, 2);
    }

    #[test]
    fn test_function_helper() {
        let mut builder = CodeBuilder::new("    ");

        builder
            .write_function("test_func", "x, y", false, 5)
            .writeln_mapped("return x + y", 6)
            .end_function();

        let (output, mappings) = builder.build();
        assert_eq!(output, "def test_func(x, y):\n    return x + y\n\n");
        assert_eq!(mappings[0].frame_line, 5);
        assert_eq!(mappings[0].python_line, 1);
    }
}

//! Frame Parser - Builds Frame AST from source code
//!
//! This parser is responsible for parsing Frame-specific constructs and building
//! the Frame AST. It works in conjunction with the native parsers to create a
//! complete picture of the source code.
//!
//! Enhanced with native_region_scanner integration for proper handler body parsing.

use super::frame_ast::*;
use super::native_region_scanner::{
    NativeRegionScanner, Region, RegionSpan, FrameSegmentKind,
    python::NativeRegionScannerPy,
    typescript::NativeRegionScannerTs,
    rust::NativeRegionScannerRust,
    csharp::NativeRegionScannerCs,
    c::NativeRegionScannerC,
    cpp::NativeRegionScannerCpp,
    java::NativeRegionScannerJava,
};

/// Main Frame parser
pub struct FrameParser {
    /// Source code as bytes
    source: Vec<u8>,
    /// Current position in source
    cursor: usize,
    /// Target language
    target: TargetLanguage,
    /// Whether @@persist was seen before current system
    persist_seen: bool,
    /// Optional library for persist (e.g., "serde" for Rust)
    persist_library: Option<String>,
}

impl FrameParser {
    /// Create a new Frame parser
    pub fn new(source: &[u8], target: TargetLanguage) -> Self {
        Self {
            source: source.to_vec(),
            cursor: 0,
            target,
            persist_seen: false,
            persist_library: None,
        }
    }
    
    /// Parse a complete Frame file (V4 syntax)
    ///
    /// V4 supports:
    /// - `@@target <lang>` pragma
    /// - `@@persist` annotation (before @@system)
    /// - `@@system Name { }` blocks (one or more)
    /// - Native code passes through (imports, functions, etc.)
    ///
    /// V4 does NOT support:
    /// - `module` keyword (legacy)
    /// - `@@module` (legacy)
    /// - Frame imports (native imports pass through)
    pub fn parse_module(&mut self) -> Result<FrameAst, ParseError> {
        // Skip any leading whitespace or comments
        self.skip_whitespace();

        // Skip all @@ pragmas until we hit @@system
        // This handles @@target, @@run-expect, @@skip-if, @@persist, etc.
        self.skip_pragmas();

        // Skip native preamble (imports, functions, etc.) until @@system
        self.skip_native_preamble();

        // Check for @@ pragmas again (@@persist may come after native imports)
        self.skip_pragmas();

        // Count @@system blocks to determine single vs multi-system
        let system_count = self.count_systems();

        if system_count == 0 {
            return Err(ParseError::Expected("@@system block".to_string()));
        } else if system_count == 1 {
            // Single system file
            let system = self.parse_system()?;
            Ok(FrameAst::System(system))
        } else {
            // Multiple systems - parse as module
            self.parse_multi_system_file()
        }
    }

    /// Count the number of @@system blocks in the source
    fn count_systems(&self) -> usize {
        let content = String::from_utf8_lossy(&self.source);
        content.matches("@@system ").count()
    }

    /// Parse a file with multiple @@system blocks
    fn parse_multi_system_file(&mut self) -> Result<FrameAst, ParseError> {
        let start = self.cursor;

        // Parse all systems
        let mut systems = Vec::new();
        loop {
            self.skip_whitespace();
            self.skip_pragmas();
            self.skip_native_preamble();

            if self.cursor >= self.source.len() {
                break;
            }

            if self.peek_keyword("@@system") {
                systems.push(self.parse_system()?);
            } else {
                break;
            }
        }

        Ok(FrameAst::Module(ModuleAst {
            name: "unnamed".to_string(),
            systems,
            imports: vec![], // V4: native imports pass through, not parsed
            span: Span::new(start, self.cursor),
        }))
    }

    /// Skip native code preamble until we reach @@system
    ///
    /// V4: Native code (imports, functions, etc.) passes through.
    /// This just advances the cursor to the first @@system block.
    fn skip_native_preamble(&mut self) {
        let debug = std::env::var("FRAME_PARSER_DEBUG").is_ok();

        while self.cursor < self.source.len() {
            self.skip_whitespace();

            if debug {
                let preview: String = self.source[self.cursor..].iter()
                    .take(40)
                    .map(|&b| b as char)
                    .collect();
                eprintln!("[skip_native_preamble] cursor={}, preview={:?}", self.cursor, preview);
            }

            // Stop if we're at any @@ pragma (@@system, @@persist, etc.)
            // These need to be processed by skip_pragmas, not skipped as native code
            if self.peek_string("@@") {
                if debug { eprintln!("[skip_native_preamble] Found @@ pragma, stopping"); }
                break;
            }

            // Stop if we're at EOF
            if self.cursor >= self.source.len() {
                break;
            }

            // Skip this line (native code, comment, etc.)
            while self.cursor < self.source.len() && self.source[self.cursor] != b'\n' {
                self.cursor += 1;
            }
            if self.cursor < self.source.len() {
                self.cursor += 1; // Skip newline
            }
        }
    }

    /// Skip @@ pragmas (@@target, @@run-expect, @@persist, etc.)
    ///
    /// V4 pragmas:
    /// - @@target <lang>     - Target language (skipped)
    /// - @@system <name>     - System definition (NOT skipped - parsed)
    /// - @@persist           - Persistence attribute (captured for next system)
    /// - @@run-expect, @@skip-if, @@timeout - Test metadata (skipped)
    fn skip_pragmas(&mut self) {
        loop {
            self.skip_whitespace();

            // Check for @@ pragma
            if !self.peek_string("@@") {
                break;
            }

            // Don't skip @@system - that's what we're looking for
            if self.peek_string("@@system") {
                break;
            }

            // Check for @@persist - set flag for next system
            // Supports: @@persist or @@persist(library) e.g., @@persist(serde)
            if self.peek_string("@@persist") {
                self.persist_seen = true;
                self.persist_library = None;

                // Check for @@persist(library)
                let persist_start = self.cursor;
                self.cursor += 9; // Skip "@@persist"
                self.skip_whitespace_no_newline();

                if self.cursor < self.source.len() && self.source[self.cursor] == b'(' {
                    self.cursor += 1; // Skip '('
                    self.skip_whitespace_no_newline();

                    // Parse library name
                    let lib_start = self.cursor;
                    while self.cursor < self.source.len()
                        && self.source[self.cursor] != b')'
                        && self.source[self.cursor] != b'\n'
                    {
                        self.cursor += 1;
                    }

                    if lib_start < self.cursor {
                        let lib_name = String::from_utf8_lossy(&self.source[lib_start..self.cursor])
                            .trim()
                            .to_string();
                        if !lib_name.is_empty() {
                            self.persist_library = Some(lib_name);
                        }
                    }

                    // Skip closing paren if present
                    if self.cursor < self.source.len() && self.source[self.cursor] == b')' {
                        self.cursor += 1;
                    }
                }

                // Reset cursor to skip the whole line
                self.cursor = persist_start;
            }

            // Skip this pragma line (@@target, @@persist, @@run-expect, etc.)
            while self.cursor < self.source.len() && self.source[self.cursor] != b'\n' {
                self.cursor += 1;
            }
            if self.cursor < self.source.len() {
                self.cursor += 1; // Skip the newline
            }
        }
    }

    // =========================================================================
    // LEGACY SYNTAX REMOVED
    // =========================================================================
    // The following legacy constructs are NOT supported in V4:
    // - `module` keyword
    // - `@@module`
    // - Frame `import` statements (native imports pass through)
    //
    // V4 only supports: @@target, @@system, @@persist
    // =========================================================================

    /// Parse a system definition
    pub fn parse_system(&mut self) -> Result<SystemAst, ParseError> {
        let start = self.cursor;
        
        // Skip "@@system"
        if self.peek_keyword("@@system") {
            self.cursor += 8;
        } else {
            return Err(ParseError::Expected("@@system keyword".to_string()));
        }
        
        self.skip_whitespace();
        
        // Parse system name
        let name = self.parse_identifier()?;
        
        // Parse optional parameters
        let params = if self.peek_char('(') {
            self.parse_system_params()?
        } else {
            vec![]
        };
        
        self.skip_whitespace();
        self.expect_char('{')?;
        
        // Parse system sections in fixed order: interface, machine, actions, operations, domain
        // Each section is optional, but if present they must appear in this order.
        let mut interface = vec![];
        let mut machine = None;
        let mut actions = vec![];
        let mut operations = vec![];
        let mut domain = vec![];

        self.skip_whitespace();

        // 1. interface: (optional)
        if self.peek_keyword("interface:") {
            interface = self.parse_interface()?;
            self.skip_whitespace();
        }

        // 2. machine: (optional)
        if self.peek_keyword("machine:") {
            machine = Some(self.parse_machine()?);
            self.skip_whitespace();
        }

        // 3. actions: (optional)
        if self.peek_keyword("actions:") {
            actions = self.parse_actions()?;
            self.skip_whitespace();
        }

        // 4. operations: (optional)
        if self.peek_keyword("operations:") {
            operations = self.parse_operations()?;
            self.skip_whitespace();
        }

        // 5. domain: (optional)
        if self.peek_keyword("domain:") {
            domain = self.parse_domain()?;
            self.skip_whitespace();
        }

        self.expect_char('}')?;
        
        // Build persist_attr if @@persist was seen before this system
        let persist_attr = if self.persist_seen {
            self.persist_seen = false;  // Reset for next system
            let library = self.persist_library.take();  // Take and reset
            Some(PersistAttr {
                save_name: None,
                restore_name: None,
                library,
                span: Span::new(start, start),  // Span is approximate
            })
        } else {
            None
        };

        Ok(SystemAst {
            name,
            params,
            interface,
            machine,
            actions,
            operations,
            domain,
            span: Span::new(start, self.cursor),
            section_spans: SystemSectionSpans::default(),
            persist_attr,
            section_order: vec![],
        })
    }
    
    /// Parse system parameters
    fn parse_system_params(&mut self) -> Result<Vec<SystemParam>, ParseError> {
        self.expect_char('(')?;
        let mut params = vec![];

        while !self.peek_char(')') {
            self.skip_whitespace();

            // Handle $(...) state parameter syntax - e.g., $(color)
            let (name, is_state_param) = if self.peek_string("$(") {
                self.cursor += 2; // Skip "$("
                let inner_name = self.parse_identifier()?;
                self.expect_char(')')?;
                (inner_name, true)
            } else {
                (self.parse_identifier()?, false)
            };
            self.skip_whitespace();
            
            // Parse optional type
            let param_type = if self.peek_char(':') {
                self.cursor += 1;
                self.skip_whitespace();
                self.parse_type()?
            } else {
                Type::Unknown
            };
            
            // Parse optional default
            let default = if self.peek_char('=') {
                self.cursor += 1;
                self.skip_whitespace();
                Some(self.parse_until_chars(&[',', ')'])?)
            } else {
                None
            };
            
            params.push(SystemParam {
                name,
                param_type,
                default,
                is_state_param,
                span: Span::new(self.cursor, self.cursor),
            });
            
            if self.peek_char(',') {
                self.cursor += 1;
            }
        }
        
        self.expect_char(')')?;
        Ok(params)
    }
    
    /// Parse interface section
    fn parse_interface(&mut self) -> Result<Vec<InterfaceMethod>, ParseError> {
        self.expect_keyword("interface:")?;
        self.skip_whitespace();

        let mut methods = vec![];

        // Parse methods until we hit a section keyword or closing brace
        while self.peek_method_start() && !self.is_section_keyword() && !self.peek_char('}') {
            methods.push(self.parse_interface_method()?);
            self.skip_whitespace();
        }

        Ok(methods)
    }
    
    /// Parse a single interface method
    fn parse_interface_method(&mut self) -> Result<InterfaceMethod, ParseError> {
        let start = self.cursor;
        let name = self.parse_identifier()?;

        // Parse parameters
        let params = if self.peek_char('(') {
            self.parse_method_params()?
        } else {
            vec![]
        };

        // Parse return type
        let return_type = if self.peek_char(':') {
            self.cursor += 1;
            self.skip_whitespace();
            Some(self.parse_type()?)
        } else {
            None
        };

        // Parse optional default return value (= expr)
        self.skip_whitespace();
        let return_init = if self.peek_char('=') {
            self.cursor += 1;
            self.skip_whitespace();
            let expr_start = self.cursor;
            // Read until newline
            while self.cursor < self.source.len() {
                let ch = self.source[self.cursor];
                if ch == b'\n' || ch == b'\r' {
                    break;
                }
                self.cursor += 1;
            }
            let expr = String::from_utf8_lossy(&self.source[expr_start..self.cursor]).trim().to_string();
            if expr.is_empty() { None } else { Some(expr) }
        } else {
            None
        };

        Ok(InterfaceMethod {
            name,
            params,
            return_type,
            return_init,
            span: Span::new(start, self.cursor),
        })
    }
    
    /// Parse method parameters
    fn parse_method_params(&mut self) -> Result<Vec<MethodParam>, ParseError> {
        self.expect_char('(')?;
        let mut params = vec![];
        
        while !self.peek_char(')') {
            self.skip_whitespace();
            
            let name = self.parse_identifier()?;
            self.skip_whitespace();
            
            // Parse type
            let param_type = if self.peek_char(':') {
                self.cursor += 1;
                self.skip_whitespace();
                self.parse_type()?
            } else {
                Type::Unknown
            };
            
            // Parse optional default
            let default = if self.peek_char('=') {
                self.cursor += 1;
                self.skip_whitespace();
                Some(self.parse_until_chars(&[',', ')'])?)
            } else {
                None
            };
            
            params.push(MethodParam {
                name,
                param_type,
                default,
                span: Span::new(self.cursor, self.cursor),
            });
            
            if self.peek_char(',') {
                self.cursor += 1;
            }
        }
        
        self.expect_char(')')?;
        Ok(params)
    }
    
    /// Parse machine section
    fn parse_machine(&mut self) -> Result<MachineAst, ParseError> {
        let start = self.cursor;
        
        self.expect_keyword("machine:")?;
        self.skip_whitespace();
        
        let mut states = vec![];
        
        while self.peek_state_start() {
            states.push(self.parse_state()?);
            self.skip_whitespace();
        }
        
        Ok(MachineAst {
            states,
            span: Span::new(start, self.cursor),
        })
    }
    
    /// Parse a state definition
    fn parse_state(&mut self) -> Result<StateAst, ParseError> {
        let start = self.cursor;

        // Parse state marker ($)
        self.expect_char('$')?;

        // Parse state name
        let name = self.parse_identifier()?;

        // Parse optional parameters
        let params = if self.peek_char('(') {
            self.parse_state_params()?
        } else {
            vec![]
        };

        self.skip_whitespace();

        // Parse optional parent (=> syntax for HSM)
        let parent = if self.peek_string("=>") {
            self.cursor += 2;
            self.skip_whitespace();
            self.expect_char('$')?;
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.skip_whitespace();
        self.expect_char('{')?;

        // Parse state contents
        let mut state_vars = vec![];
        let mut handlers = vec![];
        let mut enter = None;
        let mut exit = None;
        let mut default_forward = false;

        loop {
            self.skip_whitespace();

            if self.peek_char('}') {
                break;
            }

            if self.cursor >= self.source.len() {
                return Err(ParseError::Eof);
            }

            if self.peek_string("$>") {
                // Enter handler
                enter = Some(self.parse_enter_handler()?);
            } else if self.peek_string("<$") {
                // Exit handler
                exit = Some(self.parse_exit_handler()?);
            } else if self.peek_string("$.") {
                // State variable ($.varName: type = init)
                state_vars.push(self.parse_state_var()?);
            } else if self.peek_string("=>") {
                // State-level default forward (=> $^)
                self.cursor += 2;
                self.skip_whitespace();
                self.expect_string("$^")?;
                default_forward = true;
            } else if self.peek_identifier() {
                // Event handler
                handlers.push(self.parse_handler()?);
            } else {
                // Unknown content - skip one character and continue
                self.cursor += 1;
            }
        }

        let end = self.cursor;
        self.expect_char('}')?;

        Ok(StateAst {
            name,
            params,
            parent,
            state_vars,
            handlers,
            enter,
            exit,
            default_forward,
            span: Span::new(start, self.cursor),
            body_span: Span::new(start, end), // Body span before closing brace
        })
    }
    
    /// Parse state parameters
    fn parse_state_params(&mut self) -> Result<Vec<StateParam>, ParseError> {
        self.expect_char('(')?;
        let mut params = vec![];
        
        while !self.peek_char(')') {
            self.skip_whitespace();
            
            let name = self.parse_identifier()?;
            
            let param_type = if self.peek_char(':') {
                self.cursor += 1;
                self.skip_whitespace();
                self.parse_type()?
            } else {
                Type::Unknown
            };
            
            params.push(StateParam {
                name,
                param_type,
                span: Span::new(self.cursor, self.cursor),
            });
            
            if self.peek_char(',') {
                self.cursor += 1;
            }
        }
        
        self.expect_char(')')?;
        Ok(params)
    }

    /// Parse state variable declaration ($.varName: type = init)
    fn parse_state_var(&mut self) -> Result<StateVarAst, ParseError> {
        let start = self.cursor;

        // Skip "$."
        self.expect_string("$.")?;

        // Parse variable name
        let name = self.parse_identifier()?;

        self.skip_whitespace();

        // Parse type (required)
        self.expect_char(':')?;
        self.skip_whitespace();
        let var_type = self.parse_type()?;

        self.skip_whitespace();

        // Parse optional initializer
        let init = if self.peek_char('=') {
            self.cursor += 1;
            self.skip_whitespace();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(StateVarAst {
            name,
            var_type,
            init,
            span: Span::new(start, self.cursor),
        })
    }

    // ... (continued in next part due to length)

    // Helper methods
    
    /// Skip whitespace and Frame-level comments
    ///
    /// Note: We only handle Frame structural comments (// and /* */), NOT
    /// language-specific comments like Python's #. Language-specific syntax
    /// belongs in native code regions handled by native_region_scanner.
    ///
    /// Test metadata and other pragmas should use @@ syntax (e.g., @@run-expect)
    /// rather than language-specific comments.
    fn skip_whitespace(&mut self) {
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor] as char;
            if ch.is_whitespace() {
                self.cursor += 1;
            } else if self.peek_string("//") {
                // Skip Frame line comment
                while self.cursor < self.source.len() && self.source[self.cursor] != b'\n' {
                    self.cursor += 1;
                }
            } else if self.peek_string("/*") {
                // Skip Frame block comment
                self.cursor += 2;
                while self.cursor < self.source.len() - 1 {
                    if self.peek_string("*/") {
                        self.cursor += 2;
                        break;
                    }
                    self.cursor += 1;
                }
            } else {
                break;
            }
        }
    }

    /// Skip whitespace but not newlines (for same-line parsing)
    fn skip_whitespace_no_newline(&mut self) {
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor];
            if ch == b' ' || ch == b'\t' {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    /// Peek at next character
    fn peek_char(&self, ch: char) -> bool {
        self.cursor < self.source.len() && self.source[self.cursor] == ch as u8
    }
    
    /// Peek at string
    fn peek_string(&self, s: &str) -> bool {
        let bytes = s.as_bytes();
        if self.cursor + bytes.len() > self.source.len() {
            return false;
        }
        &self.source[self.cursor..self.cursor + bytes.len()] == bytes
    }
    
    /// Peek for keyword (with word boundary)
    fn peek_keyword(&self, keyword: &str) -> bool {
        if !self.peek_string(keyword) {
            return false;
        }
        
        // Check for word boundary
        let next_idx = self.cursor + keyword.len();
        if next_idx >= self.source.len() {
            return true;
        }
        
        let next_ch = self.source[next_idx] as char;
        !next_ch.is_alphanumeric() && next_ch != '_'
    }
    
    /// Check if next token looks like an identifier
    fn peek_identifier(&self) -> bool {
        if self.cursor >= self.source.len() {
            return false;
        }
        
        let ch = self.source[self.cursor] as char;
        ch.is_alphabetic() || ch == '_'
    }
    
    /// Check if next token looks like a method start
    fn peek_method_start(&self) -> bool {
        self.peek_identifier()
    }
    
    /// Check if next token looks like a state start
    fn peek_state_start(&self) -> bool {
        self.peek_char('$')
    }
    
    /// Parse an identifier
    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let start = self.cursor;

        if !self.peek_identifier() {
            return Err(ParseError::Expected("identifier".to_string()));
        }
        
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor] as char;
            if ch.is_alphanumeric() || ch == '_' {
                self.cursor += 1;
            } else {
                break;
            }
        }
        
        Ok(String::from_utf8_lossy(&self.source[start..self.cursor]).to_string())
    }
    
    /// Parse a type
    /// V4 uses native language types, so we preserve the type name as-is.
    /// This parser handles complex native types like:
    /// - string[] (TypeScript arrays)
    /// - Array<string> (TypeScript/Java generics)
    /// - Vec<String> (Rust generics)
    /// - Map<string, number> (nested generics)
    /// - int* (C pointers)
    /// - std::vector<int> (C++ namespaced types)
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let start = self.cursor;

        // Track bracket nesting to handle balanced expressions
        let mut angle_depth = 0;  // < >
        let mut bracket_depth = 0;  // [ ]
        let mut paren_depth = 0;  // ( ) - for function types

        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor] as char;

            match ch {
                // Opening brackets increase depth
                '<' => { angle_depth += 1; self.cursor += 1; }
                '[' => { bracket_depth += 1; self.cursor += 1; }
                '(' => { paren_depth += 1; self.cursor += 1; }

                // Closing brackets decrease depth
                '>' => {
                    if angle_depth > 0 {
                        angle_depth -= 1;
                        self.cursor += 1;
                    } else {
                        // Unbalanced > is a terminator
                        break;
                    }
                }
                ']' => {
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                        self.cursor += 1;
                    } else {
                        // Unbalanced ] is a terminator
                        break;
                    }
                }
                ')' => {
                    if paren_depth > 0 {
                        paren_depth -= 1;
                        self.cursor += 1;
                    } else {
                        // Unbalanced ) means end of parameter list
                        break;
                    }
                }

                // Terminators (only when not nested)
                ',' | '{' | '=' | '\n' | '\r' if angle_depth == 0 && bracket_depth == 0 && paren_depth == 0 => {
                    break;
                }

                // Any other character is part of the type
                _ => { self.cursor += 1; }
            }
        }

        let type_str = String::from_utf8_lossy(&self.source[start..self.cursor])
            .trim()
            .to_string();

        if type_str.is_empty() {
            return Err(ParseError::Expected("type".to_string()));
        }

        // For V4, always use Custom to preserve the native type name
        Ok(Type::Custom(type_str))
    }
    
    /// Parse until one of the given characters
    fn parse_until_chars(&mut self, chars: &[char]) -> Result<String, ParseError> {
        let start = self.cursor;
        
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor] as char;
            if chars.contains(&ch) {
                break;
            }
            self.cursor += 1;
        }
        
        Ok(String::from_utf8_lossy(&self.source[start..self.cursor]).to_string())
    }
    
    /// Expect a specific character
    fn expect_char(&mut self, ch: char) -> Result<(), ParseError> {
        if !self.peek_char(ch) {
            return Err(ParseError::Expected(format!("'{}'", ch)));
        }
        self.cursor += 1;
        Ok(())
    }

    /// Expect a specific string (like "$.")
    fn expect_string(&mut self, s: &str) -> Result<(), ParseError> {
        if !self.peek_string(s) {
            return Err(ParseError::Expected(format!("'{}'", s)));
        }
        self.cursor += s.len();
        Ok(())
    }

    /// Expect a specific keyword
    fn expect_keyword(&mut self, keyword: &str) -> Result<(), ParseError> {
        if !self.peek_keyword(keyword) {
            return Err(ParseError::Expected(keyword.to_string()));
        }
        self.cursor += keyword.len();
        Ok(())
    }
    
    // Stub methods - to be implemented
    
    fn parse_enter_handler(&mut self) -> Result<EnterHandler, ParseError> {
        let start = self.cursor;

        // Skip $>
        self.cursor += 2;

        // Parse optional parameters
        let params = if self.peek_char('(') {
            self.parse_event_params()?
        } else {
            vec![]
        };

        self.skip_whitespace();

        // Parse handler body using native region scanner for proper Frame detection
        let body = self.parse_handler_body_with_scanner()?;

        Ok(EnterHandler {
            params,
            body,
            span: Span::new(start, self.cursor),
        })
    }
    
    fn parse_exit_handler(&mut self) -> Result<ExitHandler, ParseError> {
        let start = self.cursor;

        // Skip <$ (exit handler token)
        self.cursor += 2;

        // Parse optional parameters
        let params = if self.peek_char('(') {
            self.parse_event_params()?
        } else {
            vec![]
        };

        self.skip_whitespace();

        // Parse handler body using native region scanner for proper Frame detection
        let body = self.parse_handler_body_with_scanner()?;

        Ok(ExitHandler {
            params,
            body,
            span: Span::new(start, self.cursor),
        })
    }
    
    fn parse_handler(&mut self) -> Result<HandlerAst, ParseError> {
        let start = self.cursor;

        // Parse event name
        let event = self.parse_identifier()?;

        // Parse parameters
        let params = if self.peek_char('(') {
            self.parse_event_params()?
        } else {
            vec![]
        };

        self.skip_whitespace();

        // Parse optional return type
        let return_type = if self.peek_char(':') {
            self.cursor += 1;
            self.skip_whitespace();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.skip_whitespace();

        // Parse handler body using native region scanner for proper Frame detection
        let body = self.parse_handler_body_with_scanner()?;

        Ok(HandlerAst {
            event,
            params,
            return_type,
            body,
            span: Span::new(start, self.cursor),
        })
    }
    
    /// Parse event parameters
    fn parse_event_params(&mut self) -> Result<Vec<EventParam>, ParseError> {
        self.expect_char('(')?;
        let mut params = vec![];
        
        while !self.peek_char(')') {
            self.skip_whitespace();
            
            if self.peek_char(')') {
                break;
            }
            
            let name = self.parse_identifier()?;
            
            let param_type = if self.peek_char(':') {
                self.cursor += 1;
                self.skip_whitespace();
                self.parse_type()?
            } else {
                Type::Unknown
            };
            
            params.push(EventParam {
                name,
                param_type,
                span: Span::new(self.cursor, self.cursor),
            });
            
            if self.peek_char(',') {
                self.cursor += 1;
            }
        }
        
        self.expect_char(')')?;
        Ok(params)
    }

    /// Parse expression
    /// Recognizes simple literals and identifiers, falls back to NativeExpr
    /// for any expression the parser doesn't understand (language-agnostic).
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.skip_whitespace();

        // Try to recognize common literal types first
        if self.peek_char('"') {
            // String literal
            self.cursor += 1;
            let start = self.cursor;
            while self.cursor < self.source.len() && self.source[self.cursor] != b'"' {
                if self.source[self.cursor] == b'\\' {
                    self.cursor += 2; // Skip escape sequence
                } else {
                    self.cursor += 1;
                }
            }
            let s = String::from_utf8_lossy(&self.source[start..self.cursor]).to_string();
            self.cursor += 1; // Skip closing quote
            Ok(Expression::Literal(Literal::String(s)))
        } else if self.peek_keyword("true") {
            self.cursor += 4;
            Ok(Expression::Literal(Literal::Bool(true)))
        } else if self.peek_keyword("false") {
            self.cursor += 5;
            Ok(Expression::Literal(Literal::Bool(false)))
        } else if self.peek_keyword("True") {
            // Python boolean
            self.cursor += 4;
            Ok(Expression::Literal(Literal::Bool(true)))
        } else if self.peek_keyword("False") {
            // Python boolean
            self.cursor += 5;
            Ok(Expression::Literal(Literal::Bool(false)))
        } else if self.cursor < self.source.len() && (self.source[self.cursor].is_ascii_digit() || self.source[self.cursor] == b'-') {
            // Number literal (possibly negative)
            let start = self.cursor;
            if self.source[self.cursor] == b'-' {
                self.cursor += 1;
            }
            while self.cursor < self.source.len() && self.source[self.cursor].is_ascii_digit() {
                self.cursor += 1;
            }
            // Check for float
            if self.cursor < self.source.len() && self.source[self.cursor] == b'.' {
                self.cursor += 1;
                while self.cursor < self.source.len() && self.source[self.cursor].is_ascii_digit() {
                    self.cursor += 1;
                }
                let num_str = String::from_utf8_lossy(&self.source[start..self.cursor]);
                let num = num_str.parse::<f64>().unwrap_or(0.0);
                Ok(Expression::Literal(Literal::Float(num)))
            } else {
                let num_str = String::from_utf8_lossy(&self.source[start..self.cursor]);
                let num = num_str.parse::<i64>().unwrap_or(0);
                Ok(Expression::Literal(Literal::Int(num)))
            }
        } else {
            // For anything else (arrays, dicts, complex expressions),
            // capture as native expression that passes through verbatim
            self.parse_native_expression()
        }
    }

    /// Parse a native expression as raw text
    /// Handles balanced brackets and stops at newline or end of expression context
    fn parse_native_expression(&mut self) -> Result<Expression, ParseError> {
        let start = self.cursor;
        let mut bracket_depth = 0;  // [ ]
        let mut paren_depth = 0;    // ( )
        let mut brace_depth = 0;    // { }

        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor] as char;

            match ch {
                '[' => { bracket_depth += 1; self.cursor += 1; }
                '(' => { paren_depth += 1; self.cursor += 1; }
                '{' => { brace_depth += 1; self.cursor += 1; }
                ']' => {
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                        self.cursor += 1;
                    } else {
                        break;
                    }
                }
                ')' => {
                    if paren_depth > 0 {
                        paren_depth -= 1;
                        self.cursor += 1;
                    } else {
                        break;
                    }
                }
                '}' => {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                        self.cursor += 1;
                    } else {
                        break;
                    }
                }
                // Stop at newline when not nested
                '\n' | '\r' if bracket_depth == 0 && paren_depth == 0 && brace_depth == 0 => {
                    break;
                }
                // Stop at comma when not nested (for parameter lists)
                ',' if bracket_depth == 0 && paren_depth == 0 && brace_depth == 0 => {
                    break;
                }
                _ => { self.cursor += 1; }
            }
        }

        let expr_str = String::from_utf8_lossy(&self.source[start..self.cursor])
            .trim()
            .to_string();

        if expr_str.is_empty() {
            // Empty expression defaults to null
            Ok(Expression::Literal(Literal::Null))
        } else {
            Ok(Expression::NativeExpr(expr_str))
        }
    }
    
    /// Parse actions section
    fn parse_actions(&mut self) -> Result<Vec<ActionAst>, ParseError> {
        self.expect_keyword("actions:")?;
        self.skip_whitespace();
        
        let mut actions = vec![];
        
        while self.peek_identifier() && !self.is_section_keyword() {
            actions.push(self.parse_action()?);
            self.skip_whitespace();
        }
        
        Ok(actions)
    }
    
    /// Parse a single action
    fn parse_action(&mut self) -> Result<ActionAst, ParseError> {
        let start = self.cursor;
        
        let name = self.parse_identifier()?;
        
        // Parse parameters
        let params = if self.peek_char('(') {
            self.parse_action_params()?
        } else {
            vec![]
        };
        
        self.skip_whitespace();
        
        // Parse action body (native code block)
        let body = self.parse_action_body()?;
        
        Ok(ActionAst {
            name,
            params,
            body,
            span: Span::new(start, self.cursor),
        })
    }
    
    /// Parse action parameters
    fn parse_action_params(&mut self) -> Result<Vec<ActionParam>, ParseError> {
        self.expect_char('(')?;
        let mut params = vec![];
        
        while !self.peek_char(')') {
            self.skip_whitespace();
            
            if self.peek_char(')') {
                break;
            }
            
            let name = self.parse_identifier()?;
            
            let param_type = if self.peek_char(':') {
                self.cursor += 1;
                self.skip_whitespace();
                self.parse_type()?
            } else {
                Type::Unknown
            };
            
            let default = if self.peek_char('=') {
                self.cursor += 1;
                self.skip_whitespace();
                Some(self.parse_until_chars(&[',', ')'])?)
            } else {
                None
            };
            
            params.push(ActionParam {
                name,
                param_type,
                default,
                span: Span::new(self.cursor, self.cursor),
            });
            
            if self.peek_char(',') {
                self.cursor += 1;
            }
        }
        
        self.expect_char(')')?;
        Ok(params)
    }
    
    /// Parse action body
    fn parse_action_body(&mut self) -> Result<ActionBody, ParseError> {
        let start = self.cursor;
        
        self.expect_char('{')?;
        
        // Collect native code until closing brace
        let _body_start = self.cursor;
        let mut depth = 1;
        
        while self.cursor < self.source.len() && depth > 0 {
            if self.source[self.cursor] == b'{' {
                depth += 1;
            } else if self.source[self.cursor] == b'}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            self.cursor += 1;
        }
        
        self.expect_char('}')?;

        let span = Span::new(start, self.cursor);
        let code = self.extract_body_content(&span);
        Ok(ActionBody { span, code: Some(code) })
    }

    /// Parse operations section
    fn parse_operations(&mut self) -> Result<Vec<OperationAst>, ParseError> {
        self.expect_keyword("operations:")?;
        self.skip_whitespace();
        
        let mut operations = vec![];
        
        while self.peek_identifier() && !self.is_section_keyword() {
            operations.push(self.parse_operation()?);
            self.skip_whitespace();
        }
        
        Ok(operations)
    }
    
    /// Parse a single operation
    fn parse_operation(&mut self) -> Result<OperationAst, ParseError> {
        let start = self.cursor;

        // Check for optional 'static' keyword
        let is_static = if self.peek_keyword("static") {
            self.cursor += 6; // Skip "static"
            self.skip_whitespace();
            true
        } else {
            false
        };

        let name = self.parse_identifier()?;

        // Parse parameters
        let params = if self.peek_char('(') {
            self.parse_operation_params()?
        } else {
            vec![]
        };

        // Parse return type
        let return_type = if self.peek_char(':') {
            self.cursor += 1;
            self.skip_whitespace();
            self.parse_type()?
        } else {
            Type::Unknown
        };

        self.skip_whitespace();

        // Parse operation body
        let body = self.parse_operation_body()?;

        Ok(OperationAst {
            name,
            params,
            return_type,
            body,
            is_static,
            span: Span::new(start, self.cursor),
        })
    }
    
    /// Parse operation parameters
    fn parse_operation_params(&mut self) -> Result<Vec<OperationParam>, ParseError> {
        self.expect_char('(')?;
        let mut params = vec![];
        
        while !self.peek_char(')') {
            self.skip_whitespace();
            
            if self.peek_char(')') {
                break;
            }
            
            let name = self.parse_identifier()?;
            
            let param_type = if self.peek_char(':') {
                self.cursor += 1;
                self.skip_whitespace();
                self.parse_type()?
            } else {
                Type::Unknown
            };
            
            let default = if self.peek_char('=') {
                self.cursor += 1;
                self.skip_whitespace();
                Some(self.parse_until_chars(&[',', ')'])?)
            } else {
                None
            };
            
            params.push(OperationParam {
                name,
                param_type,
                default,
                span: Span::new(self.cursor, self.cursor),
            });
            
            if self.peek_char(',') {
                self.cursor += 1;
            }
        }
        
        self.expect_char(')')?;
        Ok(params)
    }
    
    /// Parse operation body
    fn parse_operation_body(&mut self) -> Result<OperationBody, ParseError> {
        let start = self.cursor;
        
        self.expect_char('{')?;
        
        // Collect native code until closing brace
        let _body_start = self.cursor;
        let mut depth = 1;
        
        while self.cursor < self.source.len() && depth > 0 {
            if self.source[self.cursor] == b'{' {
                depth += 1;
            } else if self.source[self.cursor] == b'}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            self.cursor += 1;
        }
        
        self.expect_char('}')?;

        let span = Span::new(start, self.cursor);
        let code = self.extract_body_content(&span);
        Ok(OperationBody { span, code: Some(code) })
    }

    /// Parse domain section
    fn parse_domain(&mut self) -> Result<Vec<DomainVar>, ParseError> {
        self.expect_keyword("domain:")?;
        self.skip_whitespace();

        let mut vars = vec![];

        while self.cursor < self.source.len() {
            self.skip_whitespace();

            // Check termination conditions AFTER skip_whitespace
            if self.cursor >= self.source.len() {
                break;
            }
            if self.is_section_keyword() || self.peek_char('}') {
                break;
            }

            // V4: Domain is native code - capture entire line verbatim
            // Don't skip 'var' keyword - let it fail in the target language if used
            if !self.peek_identifier() && !self.peek_keyword("var") {
                // Skip unknown content to next line
                self.skip_to_next_line();
                continue;
            }

            if let Ok(var) = self.parse_domain_var(false) {
                vars.push(var);
            } else {
                // Skip to next line if parsing fails
                self.skip_to_next_line();
            }
        }

        Ok(vars)
    }
    
    /// Parse a single domain variable (V4: captures native code verbatim)
    fn parse_domain_var(&mut self, _is_frame: bool) -> Result<DomainVar, ParseError> {
        let start = self.cursor;

        // Capture the entire line as raw native code
        let line_start = self.cursor;
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor];
            if ch == b'\n' {
                break;
            }
            self.cursor += 1;
        }
        let line_end = self.cursor;

        // Extract the raw line (trimmed)
        let raw_line = String::from_utf8_lossy(&self.source[line_start..line_end])
            .trim()
            .to_string();

        // Extract variable name for identification (first identifier in the line)
        // This handles both "int x = 0" and "x: int = 0" styles
        let name = self.extract_var_name_from_native(&raw_line);

        Ok(DomainVar {
            name,
            var_type: Type::Unknown,  // Not parsed for native
            initializer: None,         // Not parsed for native
            is_frame: false,           // V4: domain is always native
            raw_code: Some(raw_line),  // Store native code for pass-through
            span: Span::new(start, self.cursor),
        })
    }

    /// Extract variable name from native declaration line
    /// Handles: "int x = 0", "x: int = 0", "let x = 0", "var x = 0", etc.
    fn extract_var_name_from_native(&self, line: &str) -> String {
        let line = line.trim();

        // Skip common keywords at start
        let keywords = ["var", "let", "const", "mut", "static", "int", "float", "double",
                        "char", "bool", "i32", "i64", "f32", "f64", "String", "str",
                        "number", "string", "boolean", "void", "auto"];

        let mut rest = line;

        // Skip leading type/keyword tokens until we find the identifier
        loop {
            rest = rest.trim_start();
            let mut found_keyword = false;
            for kw in &keywords {
                if rest.starts_with(kw) {
                    let after = &rest[kw.len()..];
                    if after.is_empty() || !after.chars().next().unwrap().is_alphanumeric() {
                        rest = after;
                        found_keyword = true;
                        break;
                    }
                }
            }
            // Handle pointer/reference markers
            rest = rest.trim_start_matches('*').trim_start_matches('&').trim_start();
            if !found_keyword {
                break;
            }
        }

        // Now extract the identifier
        let name: String = rest.chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();

        if name.is_empty() {
            // Fallback: just take the whole line as name (will likely cause issues)
            line.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
        } else {
            name
        }
    }
    
    /// Check if current position is a section keyword
    fn is_section_keyword(&self) -> bool {
        self.peek_keyword("interface:") ||
        self.peek_keyword("machine:") ||
        self.peek_keyword("actions:") ||
        self.peek_keyword("operations:") ||
        self.peek_keyword("domain:")
    }
    
    /// Skip to next line
    fn skip_to_next_line(&mut self) {
        while self.cursor < self.source.len() && self.source[self.cursor] != b'\n' {
            self.cursor += 1;
        }
        if self.cursor < self.source.len() {
            self.cursor += 1; // Skip the newline
        }
    }

    // ========================================================================
    // Body Content Extraction
    // ========================================================================

    /// Extract the inner content of a braced body from source bytes.
    /// Strips outer `{ }` and normalizes indentation.
    fn extract_body_content(&self, span: &Span) -> String {
        let bytes = &self.source[span.start..span.end];
        let text = std::str::from_utf8(bytes).unwrap_or("");
        // Strip outer braces
        let trimmed = text.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            let inner = &trimmed[1..trimmed.len() - 1];
            inner.trim_matches('\n').to_string()
        } else {
            text.to_string()
        }
    }

    // ========================================================================
    // Native Region Scanner Integration
    // ========================================================================

    /// Get the appropriate native region scanner for the target language
    fn get_native_scanner(&self) -> Box<dyn NativeRegionScanner> {
        match self.target {
            TargetLanguage::Python3 => Box::new(NativeRegionScannerPy),
            TargetLanguage::TypeScript => Box::new(NativeRegionScannerTs),
            TargetLanguage::Rust => Box::new(NativeRegionScannerRust),
            TargetLanguage::CSharp => Box::new(NativeRegionScannerCs),
            TargetLanguage::C => Box::new(NativeRegionScannerC),
            TargetLanguage::Cpp => Box::new(NativeRegionScannerCpp),
            TargetLanguage::Java => Box::new(NativeRegionScannerJava),
        }
    }

    /// Parse handler body using native region scanner for proper Frame detection
    ///
    /// This parser only extracts Frame statements - native code is NOT stored in AST.
    /// The splicer in codegen handles native code preservation using scanner regions.
    fn parse_handler_body_with_scanner(&mut self) -> Result<HandlerBody, ParseError> {
        let start = self.cursor;

        // Find the opening brace
        if !self.peek_char('{') {
            return Err(ParseError::Expected("'{' to start handler body".to_string()));
        }

        // Use native_region_scanner to identify Frame segments
        let mut scanner = self.get_native_scanner();
        let scan_result = scanner.scan(&self.source, self.cursor)
            .map_err(|e| ParseError::Unexpected(format!("Scanner error: {}", e.message)))?;

        // Parse only Frame segments - native code is handled by splicer in codegen
        let mut statements = Vec::new();

        for region in &scan_result.regions {
            match region {
                Region::NativeText { .. } => {
                    // Skip native text - it's preserved by the splicer, not stored in AST
                }
                Region::FrameSegment { span, kind, indent } => {
                    // StateVar, StateVarAssign, ReturnSugar, and Context segments
                    // are inline expressions handled by the splicer during code generation
                    if *kind == FrameSegmentKind::StateVar
                        || *kind == FrameSegmentKind::StateVarAssign
                        || *kind == FrameSegmentKind::ReturnSugar
                        || *kind == FrameSegmentKind::ContextParamShorthand
                        || *kind == FrameSegmentKind::ContextReturn
                        || *kind == FrameSegmentKind::ContextEvent
                        || *kind == FrameSegmentKind::ContextData
                        || *kind == FrameSegmentKind::ContextDataAssign
                        || *kind == FrameSegmentKind::ContextParams
                    {
                        continue;
                    }
                    let segment_bytes = &self.source[span.start..span.end];
                    let stmt = self.parse_frame_segment_from_bytes(segment_bytes, *kind, span, *indent)?;
                    statements.push(stmt);
                }
            }
        }

        // Update cursor to after the closing brace
        self.cursor = scan_result.close_byte + 1;

        Ok(HandlerBody {
            statements,
            span: Span::new(start, self.cursor),
        })
    }

    /// Parse a Frame segment from bytes based on its kind
    fn parse_frame_segment_from_bytes(
        &self,
        bytes: &[u8],
        kind: FrameSegmentKind,
        span: &RegionSpan,
        indent: usize,
    ) -> Result<Statement, ParseError> {
        let content = String::from_utf8_lossy(bytes);

        match kind {
            FrameSegmentKind::Transition => {
                // Parse transition: -> $State(args) or (exit_args) -> (enter_args) $State(state_args)
                let (target, args) = self.parse_transition_from_string(&content)?;
                Ok(Statement::Transition(TransitionAst {
                    target,
                    args,
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKind::Forward => {
                // Parse forward: => $^
                Ok(Statement::Forward(ForwardAst {
                    event: "^".to_string(), // Forward to parent
                    args: vec![],
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKind::TransitionForward => {
                // -> => $State (transition then forward event)
                let (target, _args) = self.parse_transition_forward_from_string(&content)?;
                Ok(Statement::TransitionForward(TransitionForwardAst {
                    target,
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKind::StackPush => {
                Ok(Statement::StackPush(StackPushAst {
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKind::StackPop => {
                Ok(Statement::StackPop(StackPopAst {
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKind::StateVar | FrameSegmentKind::StateVarAssign => {
                // State variables are expanded inline by the splicer
                // No separate statement needed - return an error that will be handled
                Err(ParseError::Expected("StateVar handled by splicer".to_string()))
            }
            FrameSegmentKind::ReturnSugar => {
                // return <expr> sugar
                // Handled by splicer expansion, similar to StateVar
                Err(ParseError::Expected("ReturnSugar handled by splicer".to_string()))
            }
            // Context syntax and tagged instantiation - handled by splicer expansion
            FrameSegmentKind::ContextParamShorthand |
            FrameSegmentKind::ContextReturn |
            FrameSegmentKind::ContextEvent |
            FrameSegmentKind::ContextData |
            FrameSegmentKind::ContextDataAssign |
            FrameSegmentKind::ContextParams |
            FrameSegmentKind::TaggedInstantiation => {
                Err(ParseError::Expected("Context/tagged syntax handled by splicer".to_string()))
            }
        }
    }

    /// Parse transition-forward string: "-> => $State"
    fn parse_transition_forward_from_string(&self, content: &str) -> Result<(String, Vec<Expression>), ParseError> {
        let content = content.trim();

        // Find the state name (after $)
        if let Some(dollar_pos) = content.rfind('$') {
            let after_dollar = &content[dollar_pos + 1..];

            // Extract state name
            let mut target = String::new();
            let mut chars = after_dollar.chars().peekable();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    target.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            Ok((target, vec![]))
        } else {
            Err(ParseError::Expected("state name after '$' in transition-forward".to_string()))
        }
    }

    /// Parse transition string: "-> $State" or "-> $State(args)" or "(exit) -> (enter) $State(args)"
    /// Also handles pop-transition: "-> pop$"
    fn parse_transition_from_string(&self, content: &str) -> Result<(String, Vec<Expression>), ParseError> {
        let content = content.trim();

        // Check for pop-transition: -> pop$
        if content.contains("pop$") {
            return Ok(("pop$".to_string(), vec![]));
        }

        // Find the state name (after $)
        if let Some(dollar_pos) = content.rfind('$') {
            let after_dollar = &content[dollar_pos + 1..];

            // Extract state name
            let mut target = String::new();
            let mut chars = after_dollar.chars().peekable();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    target.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            // Check for state arguments
            let mut args = Vec::new();
            let remaining: String = chars.collect();
            if remaining.starts_with('(') {
                // Parse arguments
                if let Some(end_paren) = remaining.find(')') {
                    let args_str = &remaining[1..end_paren];
                    for arg in args_str.split(',') {
                        let arg = arg.trim();
                        if !arg.is_empty() {
                            args.push(self.parse_arg_expression(arg));
                        }
                    }
                }
            }

            Ok((target, args))
        } else {
            Err(ParseError::Expected("state name after '$'".to_string()))
        }
    }

    /// Parse an argument expression from a string
    fn parse_arg_expression(&self, arg: &str) -> Expression {
        let arg = arg.trim();

        // Check for string literal
        if arg.starts_with('"') && arg.ends_with('"') && arg.len() >= 2 {
            let s = arg[1..arg.len()-1].to_string();
            return Expression::Literal(Literal::String(s));
        }

        // Check for integer literal
        if let Ok(n) = arg.parse::<i64>() {
            return Expression::Literal(Literal::Int(n));
        }

        // Check for float literal
        if let Ok(f) = arg.parse::<f64>() {
            return Expression::Literal(Literal::Float(f));
        }

        // Check for boolean literals
        if arg == "true" {
            return Expression::Literal(Literal::Bool(true));
        }
        if arg == "false" {
            return Expression::Literal(Literal::Bool(false));
        }

        // Check for null
        if arg == "null" || arg == "None" || arg == "nil" {
            return Expression::Literal(Literal::Null);
        }

        // Default to variable reference
        Expression::Var(arg.to_string())
    }
}

/// Parse error type
#[derive(Debug)]
pub enum ParseError {
    Expected(String),
    Unexpected(String),
    Eof,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::Expected(s) => write!(f, "Expected {}", s),
            ParseError::Unexpected(s) => write!(f, "Unexpected {}", s),
            ParseError::Eof => write!(f, "Unexpected end of file"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_system() {
        let source = r#"
@@system TrafficLight {
    machine:
        $Red {
            tick() { -> $Green() }
        }
        $Green {
            tick() { -> $Yellow() }
        }
        $Yellow {
            tick() { -> $Red() }
        }
}"#;
        
        let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
        let result = parser.parse_module();
        
        assert!(result.is_ok());
        
        if let Ok(FrameAst::System(system)) = result {
            assert_eq!(system.name, "TrafficLight");
            assert!(system.machine.is_some());
            
            let machine = system.machine.unwrap();
            assert_eq!(machine.states.len(), 3);
            assert_eq!(machine.states[0].name, "Red");
            assert_eq!(machine.states[1].name, "Green");
            assert_eq!(machine.states[2].name, "Yellow");
        } else {
            panic!("Expected System AST");
        }
    }
}
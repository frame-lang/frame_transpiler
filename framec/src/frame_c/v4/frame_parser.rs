//! Frame Parser - Builds Frame AST from source code
//!
//! This parser is responsible for parsing Frame-specific constructs and building
//! the Frame AST. It works in conjunction with the native parsers to create a
//! complete picture of the source code.
//!
//! Enhanced with native_region_scanner integration for proper handler body parsing.

use super::frame_ast::*;
use super::native_region_scanner::{
    NativeRegionScannerV3, RegionV3, RegionSpan, FrameSegmentKindV3,
    python::NativeRegionScannerPyV3,
    typescript::NativeRegionScannerTsV3,
    rust::NativeRegionScannerRustV3,
    csharp::NativeRegionScannerCsV3,
    c::NativeRegionScannerCV3,
    cpp::NativeRegionScannerCppV3,
    java::NativeRegionScannerJavaV3,
};

/// Main Frame parser
pub struct FrameParser {
    /// Source code as bytes
    source: Vec<u8>,
    /// Current position in source
    cursor: usize,
    /// Target language
    target: TargetLanguage,
}

impl FrameParser {
    /// Create a new Frame parser
    pub fn new(source: &[u8], target: TargetLanguage) -> Self {
        Self {
            source: source.to_vec(),
            cursor: 0,
            target,
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
    /// - `module` keyword (V3)
    /// - `@@module` (V3)
    /// - Frame imports (native imports pass through)
    pub fn parse_module(&mut self) -> Result<FrameAst, ParseError> {
        // Skip any leading whitespace or comments
        self.skip_whitespace();

        // Skip all @@ pragmas until we hit @@system
        // This handles @@target, @@run-expect, @@skip-if, @@persist, etc.
        self.skip_pragmas();

        // Skip native preamble (imports, functions, etc.) until @@system
        self.skip_native_preamble();

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

            // Stop if we're at @@system (V4 only recognizes @@system, not module)
            if self.peek_string("@@system") {
                if debug { eprintln!("[skip_native_preamble] Found @@system, stopping"); }
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
    /// - @@persist           - Persistence attribute (skipped, handled separately)
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
    // V3 SYNTAX REMOVED
    // =========================================================================
    // The following V3 constructs are NOT supported in V4:
    // - `module` keyword
    // - `@@module`
    // - Frame `import` statements (native imports pass through)
    //
    // V4 only supports: @@target, @@system, @@persist
    // =========================================================================

    /// Try to parse a system
    fn try_parse_system(&mut self) -> Result<Option<SystemAst>, ParseError> {
        self.skip_whitespace();
        
        if self.peek_keyword("@@system") {
            Ok(Some(self.parse_system()?))
        } else {
            Ok(None)
        }
    }
    
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
        
        // Parse system sections
        let mut interface = vec![];
        let mut machine = None;
        let mut actions = vec![];
        let mut operations = vec![];
        let mut domain = vec![];
        
        // Loop until we find the closing brace OR hit EOF
        while self.cursor < self.source.len() && !self.peek_char('}') {
            self.skip_whitespace();

            // Check for EOF after skip_whitespace
            if self.cursor >= self.source.len() {
                break;
            }

            if self.peek_keyword("interface:") {
                interface = self.parse_interface()?;
            } else if self.peek_keyword("machine:") {
                machine = Some(self.parse_machine()?);
            } else if self.peek_keyword("actions:") {
                actions = self.parse_actions()?;
            } else if self.peek_keyword("operations:") {
                operations = self.parse_operations()?;
            } else if self.peek_keyword("domain:") {
                domain = self.parse_domain()?;
            } else if self.peek_char('}') {
                // Found closing brace, exit loop
                break;
            } else {
                // Skip unknown content
                self.skip_to_next_section();
            }
        }
        
        self.expect_char('}')?;
        
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
            persist_attr: None,
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
        
        Ok(InterfaceMethod {
            name,
            params,
            return_type,
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
        let mut handlers = vec![];
        let mut enter = None;
        let mut exit = None;

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
            } else if self.peek_string("$<") {
                // Exit handler
                exit = Some(self.parse_exit_handler()?);
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
            handlers,
            enter,
            exit,
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
    /// V4 uses native language types, so we preserve the type name as-is
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let type_name = self.parse_identifier()?;
        // For V4, always use Custom to preserve the native type name
        Ok(Type::Custom(type_name))
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
    
    /// Expect a specific keyword
    fn expect_keyword(&mut self, keyword: &str) -> Result<(), ParseError> {
        if !self.peek_keyword(keyword) {
            return Err(ParseError::Expected(keyword.to_string()));
        }
        self.cursor += keyword.len();
        Ok(())
    }
    
    /// Skip to next section marker
    fn skip_to_next_section(&mut self) {
        while self.cursor < self.source.len() {
            if self.peek_keyword("interface:") ||
               self.peek_keyword("machine:") ||
               self.peek_keyword("actions:") ||
               self.peek_keyword("operations:") ||
               self.peek_keyword("domain:") ||
               self.peek_char('}') {
                break;
            }
            self.cursor += 1;
        }
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

        // Skip $<
        self.cursor += 2;

        // Skip optional empty parens
        if self.peek_char('(') {
            self.expect_char('(')?;
            self.expect_char(')')?;
        }

        self.skip_whitespace();

        // Parse handler body using native region scanner for proper Frame detection
        let body = self.parse_handler_body_with_scanner()?;

        Ok(ExitHandler {
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

    /// Try to parse a Frame statement
    fn try_parse_frame_statement(&mut self) -> Result<Option<Statement>, ParseError> {
        self.skip_whitespace();
        
        // Check for Frame statement markers
        if self.peek_string("->") {
            // Transition
            Ok(Some(self.parse_transition()?))
        } else if self.peek_string("=>") {
            // Forward
            Ok(Some(self.parse_forward()?))
        } else if self.peek_string("$$[+]") {
            // Stack push
            self.cursor += 5;
            Ok(Some(Statement::StackPush(StackPushAst {
                span: Span::new(self.cursor - 5, self.cursor),
                indent: 0,
            })))
        } else if self.peek_string("$$[-]") {
            // Stack pop
            self.cursor += 5;
            Ok(Some(Statement::StackPop(StackPopAst {
                span: Span::new(self.cursor - 5, self.cursor),
                indent: 0,
            })))
        } else if self.peek_char('^') {
            // Return or continue
            self.cursor += 1;
            if self.peek_char('>') {
                self.cursor += 1;
                Ok(Some(Statement::Continue(ContinueAst {
                    span: Span::new(self.cursor - 2, self.cursor),
                })))
            } else {
                // Parse optional return value
                self.skip_whitespace();
                let value = if !self.peek_char(';') && !self.peek_char('}') && !self.peek_char('\n') {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                Ok(Some(Statement::Return(ReturnAst {
                    value,
                    span: Span::new(self.cursor - 1, self.cursor),
                })))
            }
        } else {
            Ok(None)
        }
    }
    
    /// Check if current position starts a Frame statement
    fn is_frame_statement_start(&self) -> bool {
        self.peek_string("->") ||
        self.peek_string("=>") ||
        self.peek_string("$$[") ||
        self.peek_char('^')
    }
    
    /// Parse transition statement
    fn parse_transition(&mut self) -> Result<Statement, ParseError> {
        let start = self.cursor;
        
        // Skip ->
        self.cursor += 2;
        self.skip_whitespace();
        
        // Parse target state
        self.expect_char('$')?;
        let target = self.parse_identifier()?;
        
        // Parse optional arguments
        let args = if self.peek_char('(') {
            self.parse_call_args()?
        } else {
            vec![]
        };
        
        Ok(Statement::Transition(TransitionAst {
            target,
            args,
            span: Span::new(start, self.cursor),
            indent: 0,
        }))
    }

    /// Parse forward statement
    fn parse_forward(&mut self) -> Result<Statement, ParseError> {
        let start = self.cursor;

        // Skip =>
        self.cursor += 2;
        self.skip_whitespace();

        // Parse event name
        let event = self.parse_identifier()?;

        // Parse optional arguments
        let args = if self.peek_char('(') {
            self.parse_call_args()?
        } else {
            vec![]
        };

        Ok(Statement::Forward(ForwardAst {
            event,
            args,
            span: Span::new(start, self.cursor),
            indent: 0,
        }))
    }
    
    /// Parse call arguments
    fn parse_call_args(&mut self) -> Result<Vec<Expression>, ParseError> {
        self.expect_char('(')?;
        let mut args = vec![];
        
        while !self.peek_char(')') {
            self.skip_whitespace();
            
            if self.peek_char(')') {
                break;
            }
            
            args.push(self.parse_expression()?);
            
            if self.peek_char(',') {
                self.cursor += 1;
            }
        }
        
        self.expect_char(')')?;
        Ok(args)
    }
    
    /// Parse expression (simplified for now)
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.skip_whitespace();
        
        // Simple expression parsing - just identifiers and literals for now
        if self.peek_identifier() {
            let name = self.parse_identifier()?;
            Ok(Expression::Var(name))
        } else if self.peek_char('"') {
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
        } else if self.cursor < self.source.len() && self.source[self.cursor].is_ascii_digit() {
            // Number literal
            let start = self.cursor;
            while self.cursor < self.source.len() && self.source[self.cursor].is_ascii_digit() {
                self.cursor += 1;
            }
            let num_str = String::from_utf8_lossy(&self.source[start..self.cursor]);
            let num = num_str.parse::<i64>().unwrap_or(0);
            Ok(Expression::Literal(Literal::Int(num)))
        } else {
            // Default to null for now
            Ok(Expression::Literal(Literal::Null))
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
        let body_start = self.cursor;
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

        // ActionBody just stores span - splicer extracts content from original source
        Ok(ActionBody {
            span: Span::new(start, self.cursor),
        })
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
        let body_start = self.cursor;
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

        // OperationBody just stores span - splicer extracts content from original source
        Ok(OperationBody {
            span: Span::new(start, self.cursor),
        })
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

            // Check for var keyword or direct identifier
            let is_frame_var = if self.peek_keyword("var") {
                self.cursor += 3;
                self.skip_whitespace();
                true
            } else if self.peek_identifier() {
                // Could be a native declaration or Frame var without 'var' keyword
                true
            } else {
                // Skip unknown content to next line
                self.skip_to_next_line();
                continue;
            };

            if let Ok(var) = self.parse_domain_var(is_frame_var) {
                vars.push(var);
            } else {
                // Skip to next line if parsing fails
                self.skip_to_next_line();
            }
        }

        Ok(vars)
    }
    
    /// Parse a single domain variable
    fn parse_domain_var(&mut self, is_frame: bool) -> Result<DomainVar, ParseError> {
        let start = self.cursor;

        let name = self.parse_identifier()?;
        self.skip_whitespace();

        // Parse type annotation if present
        let var_type = if self.peek_char(':') {
            self.cursor += 1;
            self.skip_whitespace();
            self.parse_type()?
        } else {
            Type::Unknown
        };

        self.skip_whitespace();

        // Parse initializer if present
        let initializer = if self.peek_char('=') {
            self.cursor += 1;
            self.skip_whitespace();
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        // Skip to end of line or semicolon
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor];
            if ch == b'\n' || ch == b';' {
                if ch == b';' {
                    self.cursor += 1;
                }
                break;
            }
            self.cursor += 1;
        }
        
        Ok(DomainVar {
            name,
            var_type,
            initializer,
            is_frame,
            span: Span::new(start, self.cursor),
        })
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
    // Error Recovery Methods
    // ========================================================================

    /// Recover to next section marker, skipping malformed content
    fn recover_to_next_section(&mut self) {
        while self.cursor < self.source.len() {
            if self.is_section_keyword() || self.peek_char('}') {
                break;
            }
            // Skip to end of line
            self.skip_to_next_line();
        }
    }

    /// Recover to next state marker ($), skipping malformed content
    fn recover_to_next_state(&mut self) {
        while self.cursor < self.source.len() {
            self.skip_whitespace();
            if self.peek_state_start() || self.is_section_keyword() || self.peek_char('}') {
                break;
            }
            self.skip_to_next_line();
        }
    }

    /// Recover to next handler, skipping malformed content
    fn recover_to_next_handler(&mut self) {
        while self.cursor < self.source.len() {
            self.skip_whitespace();
            // Look for handler start: identifier(, $>, $<, or closing }
            if self.peek_identifier() || self.peek_string("$>") || self.peek_string("$<") || self.peek_char('}') {
                break;
            }
            self.skip_to_next_line();
        }
    }

    // ========================================================================
    // Native Region Scanner Integration
    // ========================================================================

    /// Get the appropriate native region scanner for the target language
    fn get_native_scanner(&self) -> Box<dyn NativeRegionScannerV3> {
        match self.target {
            TargetLanguage::Python3 => Box::new(NativeRegionScannerPyV3),
            TargetLanguage::TypeScript => Box::new(NativeRegionScannerTsV3),
            TargetLanguage::Rust => Box::new(NativeRegionScannerRustV3),
            TargetLanguage::CSharp => Box::new(NativeRegionScannerCsV3),
            TargetLanguage::C => Box::new(NativeRegionScannerCV3),
            TargetLanguage::Cpp => Box::new(NativeRegionScannerCppV3),
            TargetLanguage::Java => Box::new(NativeRegionScannerJavaV3),
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
                RegionV3::NativeText { .. } => {
                    // Skip native text - it's preserved by the splicer, not stored in AST
                }
                RegionV3::FrameSegment { span, kind, indent } => {
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
        kind: FrameSegmentKindV3,
        span: &RegionSpan,
        indent: usize,
    ) -> Result<Statement, ParseError> {
        let content = String::from_utf8_lossy(bytes);

        match kind {
            FrameSegmentKindV3::Transition => {
                // Parse transition: -> $State(args) or (exit_args) -> (enter_args) $State(state_args)
                let (target, args) = self.parse_transition_from_string(&content)?;
                Ok(Statement::Transition(TransitionAst {
                    target,
                    args,
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKindV3::Forward => {
                // Parse forward: => $^
                Ok(Statement::Forward(ForwardAst {
                    event: "^".to_string(), // Forward to parent
                    args: vec![],
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKindV3::StackPush => {
                Ok(Statement::StackPush(StackPushAst {
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
            FrameSegmentKindV3::StackPop => {
                Ok(Statement::StackPop(StackPopAst {
                    span: Span::new(span.start, span.end),
                    indent,
                }))
            }
        }
    }

    /// Parse transition string: "-> $State" or "-> $State(args)" or "(exit) -> (enter) $State(args)"
    fn parse_transition_from_string(&self, content: &str) -> Result<(String, Vec<Expression>), ParseError> {
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
//! Frame Parser - Builds Frame AST from source code
//!
//! This parser is responsible for parsing Frame-specific constructs and building
//! the Frame AST. It works in conjunction with the native parsers to create a
//! complete picture of the source code.

use super::frame_ast::*;
use super::system_parser::SystemParserV3;
use super::machine_parser::MachineParserV3;
use crate::frame_c::utils::RunError;
use std::collections::HashMap;

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
    
    /// Parse a complete Frame module
    pub fn parse_module(&mut self) -> Result<FrameAst, ParseError> {
        // Skip any leading whitespace or comments
        self.skip_whitespace();
        
        // Skip @@target directive if present
        if self.peek_string("@@target") {
            // Skip the entire line
            while self.cursor < self.source.len() && self.source[self.cursor] != b'\n' {
                self.cursor += 1;
            }
            if self.cursor < self.source.len() {
                self.cursor += 1; // Skip the newline
            }
            self.skip_whitespace();
        }
        
        // Check if this is a module or a single system
        if self.is_module() {
            self.parse_module_ast()
        } else {
            // Single system file
            let system = self.parse_system()?;
            Ok(FrameAst::System(system))
        }
    }
    
    /// Check if the source defines a module
    fn is_module(&self) -> bool {
        // Look for module keyword or multiple systems
        let content = String::from_utf8_lossy(&self.source);
        content.contains("module ") || content.matches("system ").count() > 1
    }
    
    /// Parse a module with multiple systems
    fn parse_module_ast(&mut self) -> Result<FrameAst, ParseError> {
        let start = self.cursor;
        
        // Parse module name if present
        let name = self.parse_module_name().unwrap_or_else(|| "unnamed".to_string());
        
        // Parse imports
        let imports = self.parse_imports()?;
        
        // Parse all systems
        let mut systems = Vec::new();
        while let Some(system) = self.try_parse_system()? {
            systems.push(system);
        }
        
        Ok(FrameAst::Module(ModuleAst {
            name,
            systems,
            imports,
            span: Span::new(start, self.cursor),
        }))
    }
    
    /// Parse module name
    fn parse_module_name(&mut self) -> Option<String> {
        // Look for "module ModuleName" pattern
        let content = String::from_utf8_lossy(&self.source[self.cursor..]);
        if let Some(idx) = content.find("module ") {
            self.cursor += idx + 7; // Skip "module "
            self.skip_whitespace();
            let name_start = self.cursor;
            while self.cursor < self.source.len() {
                let ch = self.source[self.cursor] as char;
                if !ch.is_alphanumeric() && ch != '_' {
                    break;
                }
                self.cursor += 1;
            }
            let name = String::from_utf8_lossy(&self.source[name_start..self.cursor]).to_string();
            Some(name)
        } else {
            None
        }
    }
    
    /// Parse import statements
    fn parse_imports(&mut self) -> Result<Vec<Import>, ParseError> {
        let mut imports = Vec::new();
        
        // Look for import statements
        while self.peek_keyword("import") {
            imports.push(self.parse_import()?);
        }
        
        Ok(imports)
    }
    
    /// Parse a single import statement
    fn parse_import(&mut self) -> Result<Import, ParseError> {
        let start = self.cursor;
        
        // Skip "import"
        self.expect_keyword("import")?;
        self.skip_whitespace();
        
        // Parse module path
        let module = self.parse_identifier()?;
        
        // Parse optional symbols
        let mut symbols = Vec::new();
        if self.peek_char('{') {
            self.cursor += 1; // Skip '{'
            self.skip_whitespace();
            
            while !self.peek_char('}') {
                symbols.push(self.parse_identifier()?);
                self.skip_whitespace();
                
                if self.peek_char(',') {
                    self.cursor += 1;
                    self.skip_whitespace();
                }
            }
            
            self.expect_char('}')?;
        }
        
        // Parse optional alias
        let alias = if self.peek_keyword("as") {
            self.expect_keyword("as")?;
            self.skip_whitespace();
            Some(self.parse_identifier()?)
        } else {
            None
        };
        
        Ok(Import {
            module,
            symbols,
            alias,
            span: Span::new(start, self.cursor),
        })
    }
    
    /// Try to parse a system
    fn try_parse_system(&mut self) -> Result<Option<SystemAst>, ParseError> {
        self.skip_whitespace();
        
        if self.peek_keyword("system") || self.peek_keyword("@@system") {
            Ok(Some(self.parse_system()?))
        } else {
            Ok(None)
        }
    }
    
    /// Parse a system definition
    pub fn parse_system(&mut self) -> Result<SystemAst, ParseError> {
        let start = self.cursor;
        
        // Skip "system" or "@@system"
        if self.peek_keyword("@@system") {
            self.cursor += 8;
        } else if self.peek_keyword("system") {
            self.cursor += 6;
        } else {
            return Err(ParseError::Expected("system keyword".to_string()));
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
        
        while !self.peek_char('}') {
            self.skip_whitespace();
            
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
            } else {
                // Skip unknown sections
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
        })
    }
    
    /// Parse system parameters
    fn parse_system_params(&mut self) -> Result<Vec<SystemParam>, ParseError> {
        self.expect_char('(')?;
        let mut params = vec![];
        
        while !self.peek_char(')') {
            self.skip_whitespace();
            
            let name = self.parse_identifier()?;
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
        
        while self.peek_method_start() {
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
        
        self.expect_char('}')?;
        
        Ok(StateAst {
            name,
            params,
            parent,
            handlers,
            enter,
            exit,
            span: Span::new(start, self.cursor),
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
    
    /// Skip whitespace and comments
    fn skip_whitespace(&mut self) {
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor] as char;
            if ch.is_whitespace() {
                self.cursor += 1;
            } else if self.peek_string("//") {
                // Skip line comment
                while self.cursor < self.source.len() && self.source[self.cursor] != b'\n' {
                    self.cursor += 1;
                }
            } else if self.peek_string("/*") {
                // Skip block comment
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
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let type_name = self.parse_identifier()?;
        
        Ok(match type_name.as_str() {
            "int" => Type::Int,
            "float" => Type::Float,
            "string" | "str" => Type::String,
            "bool" | "boolean" => Type::Bool,
            _ => Type::Custom(type_name),
        })
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
        
        // Parse handler body
        let body = self.parse_handler_body()?;
        
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
        
        // Parse handler body
        let body = self.parse_handler_body()?;
        
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
        
        // Parse handler body
        let body = self.parse_handler_body()?;
        
        Ok(HandlerAst {
            event,
            params,
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
    
    /// Parse handler body
    fn parse_handler_body(&mut self) -> Result<HandlerBody, ParseError> {
        let start = self.cursor;
        
        self.expect_char('{')?;
        
        let mut statements = vec![];
        let mut depth = 1; // Track brace depth for nested blocks
        
        // Simple approach: collect everything between braces
        let body_start = self.cursor;
        
        while self.cursor < self.source.len() && depth > 0 {
            let ch = self.source[self.cursor];
            
            if ch == b'{' {
                depth += 1;
            } else if ch == b'}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            
            self.cursor += 1;
        }
        
        // Extract the body content
        let body_content = &self.source[body_start..self.cursor];
        
        // Now parse the body content for Frame statements
        let mut body_cursor = 0;
        while body_cursor < body_content.len() {
            // Skip whitespace
            while body_cursor < body_content.len() && body_content[body_cursor].is_ascii_whitespace() {
                body_cursor += 1;
            }
            
            if body_cursor >= body_content.len() {
                break;
            }
            
            // Check for Frame statements
            let remaining = &body_content[body_cursor..];
            
            if remaining.starts_with(b"->") {
                // Parse transition inline
                body_cursor += 2;
                // Skip whitespace
                while body_cursor < body_content.len() && body_content[body_cursor].is_ascii_whitespace() {
                    body_cursor += 1;
                }
                // Expect $
                if body_cursor < body_content.len() && body_content[body_cursor] == b'$' {
                    body_cursor += 1;
                    // Parse state name
                    let name_start = body_cursor;
                    while body_cursor < body_content.len() {
                        let ch = body_content[body_cursor];
                        if !ch.is_ascii_alphanumeric() && ch != b'_' {
                            break;
                        }
                        body_cursor += 1;
                    }
                    let target = String::from_utf8_lossy(&body_content[name_start..body_cursor]).to_string();
                    
                    // Skip optional ()
                    if body_cursor < body_content.len() && body_content[body_cursor] == b'(' {
                        // Skip to matching )
                        let mut paren_depth = 1;
                        body_cursor += 1;
                        while body_cursor < body_content.len() && paren_depth > 0 {
                            if body_content[body_cursor] == b'(' {
                                paren_depth += 1;
                            } else if body_content[body_cursor] == b')' {
                                paren_depth -= 1;
                            }
                            body_cursor += 1;
                        }
                    }
                    
                    statements.push(Statement::Transition(TransitionAst {
                        target,
                        args: vec![],
                        span: Span::new(body_start + body_cursor, body_start + body_cursor),
                    }));
                }
            } else if remaining.starts_with(b"=>") {
                // Forward - similar parsing
                body_cursor += 2;
                // For now, skip to next statement
                while body_cursor < body_content.len() && body_content[body_cursor] != b'\n' {
                    body_cursor += 1;
                }
            } else if remaining.starts_with(b"^") {
                // Return or continue
                body_cursor += 1;
                if body_cursor < body_content.len() && body_content[body_cursor] == b'>' {
                    // Continue
                    body_cursor += 1;
                    statements.push(Statement::Continue(ContinueAst {
                        span: Span::new(body_start + body_cursor - 2, body_start + body_cursor),
                    }));
                } else {
                    // Return
                    statements.push(Statement::Return(ReturnAst {
                        value: None,
                        span: Span::new(body_start + body_cursor - 1, body_start + body_cursor),
                    }));
                }
            } else {
                // Native code - collect until next Frame statement or newline
                let native_start = body_cursor;
                while body_cursor < body_content.len() {
                    if body_content[body_cursor] == b'\n' {
                        body_cursor += 1;
                        break;
                    }
                    if body_cursor + 1 < body_content.len() {
                        let next2 = &body_content[body_cursor..body_cursor + 2];
                        if next2 == b"->" || next2 == b"=>" || next2 == b"$$" {
                            break;
                        }
                    }
                    if body_content[body_cursor] == b'^' {
                        break;
                    }
                    body_cursor += 1;
                }
                
                if body_cursor > native_start {
                    let content = String::from_utf8_lossy(&body_content[native_start..body_cursor]).trim().to_string();
                    if !content.is_empty() {
                        statements.push(Statement::Native(NativeBlock {
                            content,
                            language: self.target,
                            span: Span::new(body_start + native_start, body_start + body_cursor),
                        }));
                    }
                }
            }
        }
        
        self.expect_char('}')?;
        
        Ok(HandlerBody {
            statements,
            span: Span::new(start, self.cursor),
        })
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
            })))
        } else if self.peek_string("$$[-]") {
            // Stack pop
            self.cursor += 5;
            Ok(Some(Statement::StackPop(StackPopAst {
                span: Span::new(self.cursor - 5, self.cursor),
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
    
    fn parse_actions(&mut self) -> Result<Vec<ActionAst>, ParseError> {
        // TODO: Implement actions parsing
        self.expect_keyword("actions:")?;
        self.skip_to_next_section();
        Ok(vec![])
    }
    
    fn parse_operations(&mut self) -> Result<Vec<OperationAst>, ParseError> {
        // TODO: Implement operations parsing
        self.expect_keyword("operations:")?;
        self.skip_to_next_section();
        Ok(vec![])
    }
    
    fn parse_domain(&mut self) -> Result<Vec<DomainVar>, ParseError> {
        // TODO: Implement domain parsing
        self.expect_keyword("domain:")?;
        self.skip_to_next_section();
        Ok(vec![])
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
system TrafficLight {
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
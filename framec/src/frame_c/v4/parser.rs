// Frame v4 Parser - System-focused parsing with MIR support
//
// The v4 parser focuses on Frame structure while using MIR
// to track Frame constructs within native code blocks.

use super::scanner::{Token, TokenType};
use super::ast::*;
use super::error::ErrorsAcc;
use super::native_scanner::{get_scanner, NativeScanner};
use super::mir::MirBlock;
use super::TargetLanguage;
use std::collections::HashMap;

pub fn parse(tokens: Vec<Token>, source: &str, target: TargetLanguage) -> Result<SystemAst, ErrorsAcc> {
    let mut parser = Parser::new(tokens, source, target);
    parser.parse()
}

struct Parser {
    tokens: Vec<Token>,
    source: String,
    current: usize,
    errors: ErrorsAcc,
    target_language: TargetLanguage,
}

impl Parser {
    fn new(tokens: Vec<Token>, source: &str, target: TargetLanguage) -> Self {
        Self {
            tokens,
            source: source.to_string(),
            current: 0,
            errors: ErrorsAcc::new(),
            target_language: target,
        }
    }

    fn parse(&mut self) -> Result<SystemAst, ErrorsAcc> {
        // Parse target pragma if present
        let target = self.parse_target_pragma()?;
        
        // Collect native imports
        let native_imports = self.parse_imports();
        
        // Parse annotations
        let annotations = self.parse_annotations();
        
        // Parse system (accept either System token from @@system or keyword "system")
        if !self.check(TokenType::System) {
            self.error(&format!("Expected system declaration, got {:?}", self.peek()));
            return Err(self.errors.clone());
        }
        self.advance();
        let name = self.expect_identifier()?;
        
        // Parse system parameters
        let params = if self.peek() == Some(TokenType::LeftParen) {
            self.parse_system_params()?
        } else {
            SystemParams::default()
        };
        
        self.expect(TokenType::LeftBrace)?;
        
        // Parse system body blocks (in canonical order)
        let mut operations = None;
        let mut interface = None;
        let mut machine = None;
        let mut actions = None;
        let mut domain = None;
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if self.check(TokenType::Operations) {
                if operations.is_some() {
                    self.error("Duplicate operations block");
                }
                operations = Some(self.parse_operations()?);
            } else if self.check(TokenType::Interface) {
                if interface.is_some() {
                    self.error("Duplicate interface block");
                }
                interface = Some(self.parse_interface()?);
            } else if self.check(TokenType::Machine) {
                if machine.is_some() {
                    self.error("Duplicate machine block");
                }
                machine = Some(self.parse_machine()?);
            } else if self.check(TokenType::Actions) {
                if actions.is_some() {
                    self.error("Duplicate actions block");
                }
                actions = Some(self.parse_actions()?);
            } else if self.check(TokenType::Domain) {
                if domain.is_some() {
                    self.error("Duplicate domain block");
                }
                domain = Some(self.parse_domain()?);
            } else {
                self.error(&format!("Unexpected token in system body: {:?}", self.peek()));
                self.advance();
            }
        }
        
        self.expect(TokenType::RightBrace)?;
        
        // Parse any trailing native code (like test code after system)
        let trailing_native_code = if !self.is_at_end() && self.peek() != Some(TokenType::Eof) {
            let mut code = String::new();
            while !self.is_at_end() && self.peek() != Some(TokenType::Eof) {
                if let Some(token) = self.current_token() {
                    code.push_str(&token.lexeme);
                    code.push(' ');
                }
                self.advance();
            }
            Some(code)
        } else {
            None
        };
        
        let source_location = if let Some(first) = self.tokens.first() {
            first.location.clone()
        } else {
            SourceLocation::unknown()
        };
        
        if self.errors.has_errors() {
            Err(self.errors.clone())
        } else {
            Ok(SystemAst {
                target,
                native_imports,
                annotations,
                name,
                params,
                operations,
                interface,
                machine,
                actions,
                domain,
                source_location,
            })
        }
    }

    fn parse_target_pragma(&mut self) -> Result<String, ErrorsAcc> {
        // Look for @@target at the beginning
        if self.check(TokenType::FrameAnnotation) {
            let token = self.advance();
            if token.lexeme.starts_with("@@target") {
                // Extract target language
                let parts: Vec<&str> = token.lexeme.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Ok(parts[1].to_string());
                }
            }
        }
        
        // If no @@target, try to infer from file extension
        // For now, return a default
        Ok("python".to_string())
    }

    fn parse_imports(&mut self) -> Vec<String> {
        let mut imports = Vec::new();
        
        // Collect all native code that appears before system declaration
        while !self.check(TokenType::System) && !self.is_at_end() {
            if self.check(TokenType::NativeCode) {
                let token = self.advance();
                imports.push(token.lexeme.trim().to_string());
            } else if self.check(TokenType::NativeAnnotation) || 
                      self.check(TokenType::FrameAnnotation) {
                // Skip annotations for now, they'll be parsed separately
                self.advance();
            } else {
                self.advance();
            }
        }
        
        imports
    }

    fn parse_annotations(&mut self) -> Vec<Annotation> {
        let mut annotations = Vec::new();
        
        while self.check(TokenType::NativeAnnotation) || 
              self.check(TokenType::FrameAnnotation) {
            let token = self.advance();
            
            if token.token_type == TokenType::FrameAnnotation {
                // Parse Frame annotation
                let name = token.lexeme.trim_start_matches("@@").to_string();
                // TODO: Parse annotation arguments
                annotations.push(Annotation::Frame {
                    name,
                    args: HashMap::new(),
                });
            } else {
                // Native annotation - store as opaque
                annotations.push(Annotation::Native {
                    content: token.lexeme,
                });
            }
        }
        
        annotations
    }

    fn parse_system_params(&mut self) -> Result<SystemParams, ErrorsAcc> {
        self.expect(TokenType::LeftParen)?;
        
        let mut params = SystemParams::default();
        
        while !self.check(TokenType::RightParen) && !self.is_at_end() {
            // Check for start state params $(...)
            if self.check(TokenType::State) && self.peek_lexeme() == Some("$") {
                self.advance();
                if self.check(TokenType::LeftParen) {
                    self.advance();
                    params.start_state_params = self.parse_parameter_list()?;
                    self.expect(TokenType::RightParen)?;
                }
            }
            // Check for enter params $>(...)
            else if self.check(TokenType::Enter) {
                self.advance();
                if self.check(TokenType::LeftParen) {
                    self.advance();
                    params.enter_params = self.parse_parameter_list()?;
                    self.expect(TokenType::RightParen)?;
                }
            }
            // Plain domain parameter
            else if self.check(TokenType::Identifier) {
                let name = self.advance().lexeme;
                params.domain_params.push(Parameter {
                    name,
                    type_hint: None,
                });
            }
            
            // Handle comma
            if self.check(TokenType::Comma) {
                self.advance();
            }
        }
        
        self.expect(TokenType::RightParen)?;
        Ok(params)
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, ErrorsAcc> {
        let mut params = Vec::new();
        
        if !self.check(TokenType::RightParen) {
            loop {
                let name = self.expect_identifier()?;
                
                // Check for type hint (: type)
                let type_hint = if self.check(TokenType::Colon) {
                    self.advance();
                    // Consume type as native code
                    Some(self.parse_native_type()?)
                } else {
                    None
                };
                
                params.push(Parameter { name, type_hint });
                
                if !self.check(TokenType::Comma) {
                    break;
                }
                self.advance();
            }
        }
        
        Ok(params)
    }

    fn parse_native_type(&mut self) -> Result<String, ErrorsAcc> {
        // Parse native type annotation - could be complex like List[str] or simple like int
        let mut type_str = String::new();
        let mut bracket_depth = 0;
        
        while !self.is_at_end() {
            if self.check(TokenType::LeftBracket) {
                bracket_depth += 1;
                type_str.push('[');
                self.advance();
            } else if self.check(TokenType::RightBracket) && bracket_depth > 0 {
                bracket_depth -= 1;
                type_str.push(']');
                self.advance();
            } else if bracket_depth == 0 && 
                      (self.check(TokenType::Comma) || 
                       self.check(TokenType::RightParen) ||
                       self.check(TokenType::LeftBrace)) {
                break;
            } else if self.check(TokenType::Identifier) || self.check(TokenType::NativeCode) {
                type_str.push_str(&self.advance().lexeme);
            } else {
                break;
            }
        }
        
        Ok(type_str)
    }

    fn parse_operations(&mut self) -> Result<OperationsBlock, ErrorsAcc> {
        let location = self.current_location();
        self.expect(TokenType::Operations)?;
        self.expect(TokenType::Colon)?;
        
        let mut methods = Vec::new();
        
        while !self.check_block_keyword() && !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.parse_method()?);
        }
        
        Ok(OperationsBlock {
            methods,
            source_location: location,
        })
    }

    fn parse_interface(&mut self) -> Result<InterfaceBlock, ErrorsAcc> {
        let location = self.current_location();
        self.expect(TokenType::Interface)?;
        self.expect(TokenType::Colon)?;
        
        let mut methods = Vec::new();
        
        while !self.check_block_keyword() && !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.parse_interface_method()?);
        }
        
        Ok(InterfaceBlock {
            methods,
            source_location: location,
        })
    }

    fn parse_interface_method(&mut self) -> Result<InterfaceMethod, ErrorsAcc> {
        let location = self.current_location();
        
        // The scanner may have tokenized the method name and parentheses separately
        // We need to handle both cases:
        // 1. name ( params )
        // 2. name() directly as identifier
        let name = if self.check(TokenType::Identifier) {
            self.expect_identifier()?
        } else {
            // Try to extract name from whatever token we have
            let token = self.current_token();
            if let Some(t) = token {
                let lexeme = t.lexeme.clone();
                // Extract method name if it's something like "save()"
                if let Some(paren_pos) = lexeme.find('(') {
                    self.advance();
                    lexeme[..paren_pos].to_string()
                } else {
                    return Err(self.error_at_current("Expected method name"));
                }
            } else {
                return Err(self.error_at_current("Expected method name"));
            }
        };
        
        // Handle parameters - might already be past the opening paren
        if self.check(TokenType::LeftParen) {
            self.expect(TokenType::LeftParen)?;
        }
        
        let params = if !self.check(TokenType::RightParen) {
            self.parse_parameter_list()?
        } else {
            Vec::new()
        };
        
        if self.check(TokenType::RightParen) {
            self.expect(TokenType::RightParen)?;
        }
        
        let return_type = if self.check(TokenType::Colon) {
            self.advance();
            Some(self.parse_native_type()?)
        } else {
            None
        };
        
        Ok(InterfaceMethod {
            name,
            params,
            return_type,
            source_location: location,
        })
    }

    fn parse_machine(&mut self) -> Result<MachineBlock, ErrorsAcc> {
        let location = self.current_location();
        self.expect(TokenType::Machine)?;
        self.expect(TokenType::Colon)?;
        
        let mut states = Vec::new();
        
        while self.check(TokenType::State) {
            states.push(self.parse_state()?);
        }
        
        Ok(MachineBlock {
            states,
            source_location: location,
        })
    }

    fn parse_state(&mut self) -> Result<State, ErrorsAcc> {
        let location = self.current_location();
        
        self.expect(TokenType::State)?;
        let name = self.previous().lexeme.trim_start_matches('$').to_string();
        
        // Parse state parameters if present
        let params = if self.check(TokenType::LeftParen) {
            self.advance();
            let p = self.parse_parameter_list()?;
            self.expect(TokenType::RightParen)?;
            p
        } else {
            Vec::new()
        };
        
        // Check for parent state (hierarchical)
        let parent = None; // TODO: Implement hierarchical states
        
        self.expect(TokenType::LeftBrace)?;
        
        let mut handlers = Vec::new();
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            handlers.push(self.parse_handler()?);
        }
        
        self.expect(TokenType::RightBrace)?;
        
        Ok(State {
            name,
            params,
            parent,
            handlers,
            source_location: location,
        })
    }

    fn parse_handler(&mut self) -> Result<Handler, ErrorsAcc> {
        let location = self.current_location();
        
        let (handler_type, name) = if self.check(TokenType::Enter) {
            self.advance();
            (HandlerType::Enter, None)
        } else if self.check(TokenType::Exit) {
            self.advance();
            (HandlerType::Exit, None)
        } else {
            let event_name = self.expect_identifier()?;
            (HandlerType::Event, Some(event_name))
        };
        
        self.expect(TokenType::LeftParen)?;
        let params = self.parse_parameter_list()?;
        self.expect(TokenType::RightParen)?;
        
        let return_type = if self.check(TokenType::Colon) {
            self.advance();
            Some(self.parse_native_type()?)
        } else {
            None
        };
        
        self.expect(TokenType::LeftBrace)?;
        
        // Capture native code as MIR block
        let mir_block = self.parse_handler_body()?;
        
        self.expect(TokenType::RightBrace)?;
        
        Ok(Handler {
            handler_type,
            name,
            params,
            return_type,
            mir_block,
            source_location: location,
        })
    }

    fn parse_handler_body(&mut self) -> Result<MirBlock, ErrorsAcc> {
        let mut native_code = String::new();
        let mut need_space = false;
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            // Collect ALL tokens as native code, not just NativeCode tokens
            // This preserves the original source including identifiers, operators, etc.
            let token = &self.tokens[self.current];
            
            // Only break on actual structural tokens (not state references in transitions)
            if token.token_type == TokenType::RightBrace {
                break;
            }
            
            // Special handling for System token - in handler bodies, it could be a system call
            // not a system declaration, so treat it as native code
            if token.token_type == TokenType::System {
                if need_space {
                    native_code.push(' ');
                }
                native_code.push_str("system");
                self.advance();
                need_space = true;
                continue;
            }
            
            // Add appropriate spacing between tokens
            if need_space && !token.lexeme.is_empty() {
                // Check if we need a space between previous and current token
                let last_char = native_code.chars().last().unwrap_or(' ');
                let first_char = token.lexeme.chars().next().unwrap_or(' ');
                
                // Add space between identifiers/keywords
                if (last_char.is_alphanumeric() || last_char == '_') &&
                   (first_char.is_alphanumeric() || first_char == '_') {
                    native_code.push(' ');
                }
            }
            
            native_code.push_str(&token.lexeme);
            need_space = !token.lexeme.is_empty();
            
            self.advance();
        }
        
        // Use native scanner to extract Frame constructs
        let scanner = get_scanner(self.target_language);
        let mir_block = scanner.scan_native_block(&native_code)
            .map_err(|e| {
                let mut err = ErrorsAcc::new();
                err.add_error(format!("MIR scanning error: {}", e), self.current_location());
                err
            })?;
        
        Ok(mir_block)
    }

    fn parse_actions(&mut self) -> Result<ActionsBlock, ErrorsAcc> {
        let location = self.current_location();
        self.expect(TokenType::Actions)?;
        self.expect(TokenType::Colon)?;
        
        let mut methods = Vec::new();
        
        while !self.check_block_keyword() && !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.parse_method()?);
        }
        
        Ok(ActionsBlock {
            methods,
            source_location: location,
        })
    }

    fn parse_method(&mut self) -> Result<Method, ErrorsAcc> {
        let location = self.current_location();
        
        // Check for static keyword (for operations)
        let is_static = if self.peek_lexeme() == Some("static") {
            self.advance();
            true
        } else {
            false
        };
        
        let name = self.expect_identifier()?;
        
        self.expect(TokenType::LeftParen)?;
        let params = self.parse_parameter_list()?;
        self.expect(TokenType::RightParen)?;
        
        let return_type = if self.check(TokenType::Colon) {
            self.advance();
            Some(self.parse_native_type()?)
        } else {
            None
        };
        
        self.expect(TokenType::LeftBrace)?;
        
        // Capture native code block - collect ALL tokens like in handler_body
        let mut native_code = String::new();
        let mut need_space = false;
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let token = &self.tokens[self.current];
            
            // Only break on actual structural tokens (not state references in transitions)
            if token.token_type == TokenType::RightBrace {
                break;
            }
            
            // Add appropriate spacing between tokens
            if need_space && !token.lexeme.is_empty() {
                let last_char = native_code.chars().last().unwrap_or(' ');
                let first_char = token.lexeme.chars().next().unwrap_or(' ');
                
                if (last_char.is_alphanumeric() || last_char == '_') &&
                   (first_char.is_alphanumeric() || first_char == '_') {
                    native_code.push(' ');
                }
            }
            
            native_code.push_str(&token.lexeme);
            need_space = !token.lexeme.is_empty();
            
            self.advance();
        }
        
        self.expect(TokenType::RightBrace)?;
        
        // Use MIR to scan for Frame constructs in methods too
        let scanner = get_scanner(self.target_language);
        let mir_block = scanner.scan_native_block(&native_code)
            .map_err(|e| {
                let mut err = ErrorsAcc::new();
                err.add_error(format!("MIR scanning error: {}", e), location.clone());
                err
            })?;
        
        Ok(Method {
            name,
            is_static,
            params,
            return_type,
            mir_block,
            source_location: location,
        })
    }

    fn parse_domain(&mut self) -> Result<DomainBlock, ErrorsAcc> {
        let location = self.current_location();
        self.expect(TokenType::Domain)?;
        self.expect(TokenType::Colon)?;
        
        let mut variables = Vec::new();
        
        while !self.check_block_keyword() && !self.check(TokenType::RightBrace) && !self.is_at_end() {
            variables.push(self.parse_domain_variable()?);
        }
        
        Ok(DomainBlock {
            variables,
            source_location: location,
        })
    }

    fn parse_domain_variable(&mut self) -> Result<DomainVariable, ErrorsAcc> {
        let location = self.current_location();
        
        // Parse variable declaration (native syntax)
        // Could be: name = value, let name = value, var name: type = value, etc.
        let mut name = String::new();
        let mut type_hint = None;
        let mut initializer = None;
        
        // Skip any leading keywords (let, var, const, etc.)
        if self.peek_lexeme() == Some("let") || 
           self.peek_lexeme() == Some("var") || 
           self.peek_lexeme() == Some("const") {
            self.advance();
        }
        
        // Get the variable name
        name = self.expect_identifier()?;
        
        // Check for type annotation
        if self.check(TokenType::Colon) {
            self.advance();
            type_hint = Some(self.parse_native_type()?);
        }
        
        // Check for initializer
        if self.peek_lexeme() == Some("=") {
            self.advance();
            // Capture everything until newline or next domain var as initializer
            let mut init = String::new();
            while !self.is_at_end() {
                if self.check(TokenType::NativeCode) || self.check(TokenType::Identifier) {
                    init.push_str(&self.advance().lexeme);
                } else if self.check_block_keyword() || self.check(TokenType::RightBrace) {
                    break;
                } else {
                    self.advance();
                    break;
                }
            }
            initializer = Some(init);
        }
        
        Ok(DomainVariable {
            name,
            type_hint,
            initializer,
            source_location: location,
        })
    }

    // Helper methods
    
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == Some(token_type)
    }

    fn check_keyword(&self, keyword: &str) -> bool {
        self.peek_lexeme() == Some(keyword)
    }

    fn check_block_keyword(&self) -> bool {
        matches!(
            self.peek(),
            Some(TokenType::Operations) | Some(TokenType::Interface) | 
            Some(TokenType::Machine) | Some(TokenType::Actions) | 
            Some(TokenType::Domain)
        )
    }

    fn peek(&self) -> Option<TokenType> {
        self.tokens.get(self.current).map(|t| t.token_type.clone())
    }

    fn peek_lexeme(&self) -> Option<&str> {
        self.tokens.get(self.current).map(|t| t.lexeme.as_str())
    }

    fn peek_identifier(&self) -> Option<&str> {
        if self.tokens.get(self.current)?.token_type == TokenType::Identifier {
            Some(&self.tokens[self.current].lexeme)
        } else {
            None
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    
    fn current_token(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek() == Some(TokenType::Eof) || self.current >= self.tokens.len()
    }

    fn expect(&mut self, token_type: TokenType) -> Result<Token, ErrorsAcc> {
        if self.check(token_type.clone()) {
            Ok(self.advance())
        } else {
            self.error(&format!("Expected {:?}, got {:?}", token_type, self.peek()));
            Err(self.errors.clone())
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), ErrorsAcc> {
        if self.check_keyword(keyword) {
            self.advance();
            Ok(())
        } else {
            self.error(&format!("Expected keyword '{}', got {:?}", keyword, self.peek_lexeme()));
            Err(self.errors.clone())
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ErrorsAcc> {
        if self.check(TokenType::Identifier) {
            Ok(self.advance().lexeme)
        } else {
            self.error(&format!("Expected identifier, got {:?}", self.peek()));
            Err(self.errors.clone())
        }
    }

    fn current_location(&self) -> SourceLocation {
        if let Some(token) = self.tokens.get(self.current) {
            token.location.clone()
        } else if let Some(token) = self.tokens.last() {
            token.location.clone()
        } else {
            SourceLocation::unknown()
        }
    }

    fn error(&mut self, message: &str) {
        let location = self.current_location();
        let error_msg = format!("{} at {}:{}", message, location.line, location.column);
        self.errors.push_error(error_msg);
    }
    
    fn error_at_current(&mut self, message: &str) -> ErrorsAcc {
        self.error(message);
        self.errors.clone()
    }
}
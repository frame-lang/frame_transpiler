// Frame v4 Scanner - Simplified tokenization
//
// The v4 scanner focuses on Frame structural tokens
// Native code blocks are captured as raw strings

use super::error::ErrorsAcc;
use super::ast::SourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Frame annotations
    FrameAnnotation,    // @@persist, @@system, @@target
    NativeAnnotation,   // @decorator, #[attribute], [Attribute]
    
    // Keywords
    System,
    Operations,
    Interface,
    Machine,
    Actions,
    Domain,
    
    // State machine
    State,              // $StateName
    Enter,              // $>
    Exit,               // $<
    Transition,         // ->
    ChangeState,        // :> (if still supported)
    Forward,            // =>
    StackPush,          // $$[+]
    StackPop,           // $$[-]
    Parent,             // =>
    
    // Delimiters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    
    // Identifiers and literals
    Identifier,
    NativeCode,         // Opaque native code block
    
    // Special
    Eof,
}

pub fn scan(source: &str, file_path: &str) -> Result<Vec<Token>, ErrorsAcc> {
    let mut scanner = Scanner::new(source, file_path);
    scanner.scan_tokens()
}

struct Scanner {
    source: String,
    file_path: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    fn new(source: &str, file_path: &str) -> Self {
        Self {
            source: source.to_string(),
            file_path: file_path.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<Vec<Token>, ErrorsAcc> {
        let mut errors = ErrorsAcc::new();
        
        // First, scan for @@target pragma if present
        self.skip_whitespace();
        if self.peek() == Some('@') && self.peek_next() == Some('@') {
            if self.source[self.current..].starts_with("@@target") {
                self.start = self.current;
                // Scan the entire @@target line
                while self.peek() != Some('\n') && !self.is_at_end() {
                    self.advance();
                }
                self.add_token(TokenType::FrameAnnotation);
                if self.peek() == Some('\n') {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
            }
        }
        
        // Scan all native code before @@system as a single NativeCode token
        self.skip_whitespace();
        let native_start = self.current;
        while !self.is_at_end() {
            // Check if we've hit @@system
            if self.peek() == Some('@') && self.peek_next() == Some('@') {
                if self.source[self.current..].starts_with("@@system") {
                    // Emit native code token if we have any
                    if self.current > native_start {
                        self.start = native_start;
                        self.add_token(TokenType::NativeCode);
                    }
                    break;
                }
            }
            if self.advance() == '\n' {
                self.line += 1;
                self.column = 1;
            }
        }
        
        // Now scan the rest normally
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                errors.push_error(e);
            }
        }
        
        self.add_token(TokenType::Eof);
        
        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(self.tokens.clone())
        }
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        
        match c {
            // Whitespace
            ' ' | '\r' | '\t' => {
                self.column += 1;
                Ok(())
            }
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(())
            }
            
            // Single character tokens
            '{' => {
                self.add_token(TokenType::LeftBrace);
                Ok(())
            }
            '}' => {
                self.add_token(TokenType::RightBrace);
                Ok(())
            }
            '(' => {
                self.add_token(TokenType::LeftParen);
                Ok(())
            }
            ')' => {
                self.add_token(TokenType::RightParen);
                Ok(())
            }
            '[' => {
                // Could be bracket or C# annotation
                if self.peek() == Some('[') {
                    // C++ annotation [[...]]
                    self.scan_cpp_annotation()
                } else {
                    self.add_token(TokenType::LeftBracket);
                    Ok(())
                }
            }
            ']' => {
                self.add_token(TokenType::RightBracket);
                Ok(())
            }
            ',' => {
                self.add_token(TokenType::Comma);
                Ok(())
            }
            
            // Multi-character tokens
            ':' => {
                if self.peek() == Some('>') {
                    self.advance();
                    self.add_token(TokenType::ChangeState);
                } else {
                    self.add_token(TokenType::Colon);
                }
                Ok(())
            }
            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    self.add_token(TokenType::Transition);
                    Ok(())
                } else {
                    // Part of native code
                    self.scan_native_code()
                }
            }
            '=' => {
                if self.peek() == Some('>') {
                    self.advance();
                    self.add_token(TokenType::Forward);
                } else {
                    // Just an equals sign
                    self.add_token(TokenType::Identifier); // Treat = as identifier for now
                }
                Ok(())
            }
            
            // Frame-specific
            '$' => self.scan_frame_construct(),
            
            // Annotations
            '@' => self.scan_annotation(),
            '#' => {
                if self.peek() == Some('[') {
                    self.scan_rust_annotation()
                } else {
                    // Python comment or native code
                    self.scan_native_code()
                }
            }
            
            // Comments
            '/' => {
                if self.peek() == Some('/') {
                    // Single line comment
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else if self.peek() == Some('*') {
                    // Multi-line comment
                    self.advance(); // consume *
                    while !(self.peek() == Some('*') && self.peek_next() == Some('/')) {
                        if self.is_at_end() {
                            return Err("Unterminated comment".to_string());
                        }
                        if self.advance() == '\n' {
                            self.line += 1;
                            self.column = 1;
                        }
                    }
                    self.advance(); // consume *
                    self.advance(); // consume /
                    Ok(())
                } else {
                    // Part of native code
                    self.scan_native_code()
                }
            }
            
            // String literals
            '"' | '\'' | '`' => self.scan_string_literal(c),
            
            // Identifiers and keywords
            _ if c.is_alphabetic() || c == '_' => self.scan_identifier(),
            
            // Numbers
            _ if c.is_numeric() => self.scan_number(),
            
            // Native code block (fallback for other characters)
            _ => {
                // For unexpected characters, just skip them for now
                self.advance();
                Ok(())
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                    self.column += 1;
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => break,
            }
        }
    }
    
    fn scan_frame_construct(&mut self) -> Result<(), String> {
        // Could be $State, $>, $<, or $$
        if self.peek() == Some('$') {
            self.advance();
            if self.peek() == Some('[') {
                self.advance();
                if self.peek() == Some('+') {
                    self.advance();
                    if self.peek() == Some(']') {
                        self.advance();
                        self.add_token(TokenType::StackPush);
                        return Ok(());
                    }
                } else if self.peek() == Some('-') {
                    self.advance();
                    if self.peek() == Some(']') {
                        self.advance();
                        self.add_token(TokenType::StackPop);
                        return Ok(());
                    }
                }
            }
            return Err("Invalid Frame stack operation".to_string());
        } else if self.peek() == Some('>') {
            self.advance();
            self.add_token(TokenType::Enter);
            Ok(())
        } else if self.peek() == Some('<') {
            self.advance();
            self.add_token(TokenType::Exit);
            Ok(())
        } else if self.peek() == Some('^') {
            self.advance();
            self.add_token(TokenType::Parent);
            Ok(())
        } else if self.peek().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false) {
            // State name
            while self.peek().map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false) {
                self.advance();
            }
            self.add_token(TokenType::State);
            Ok(())
        } else if self.peek() == Some('(') {
            // Start state params $(...)
            self.add_token(TokenType::State);
            Ok(())
        } else {
            Err("Invalid Frame construct".to_string())
        }
    }

    fn scan_annotation(&mut self) -> Result<(), String> {
        if self.peek() == Some('@') {
            // Frame annotation @@
            self.advance();
            
            // Capture the annotation text
            let start = self.current;
            while self.peek().map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false) {
                self.advance();
            }
            
            // Special case: @@system is both annotation and keyword
            let annotation_text = &self.source[start..self.current];
            if annotation_text == "system" {
                // Treat @@system as the system keyword for v4
                self.add_token(TokenType::System);
            } else {
                // Regular Frame annotation like @@persist, @@target
                self.add_token(TokenType::FrameAnnotation);
            }
        } else {
            // Native annotation @
            while self.peek().map(|c| c != '\n' && c != ' ').unwrap_or(false) {
                self.advance();
            }
            self.add_token(TokenType::NativeAnnotation);
        }
        Ok(())
    }

    fn scan_rust_annotation(&mut self) -> Result<(), String> {
        // #[...]
        self.advance(); // consume [
        let mut depth = 1;
        while depth > 0 && !self.is_at_end() {
            match self.advance() {
                '[' => depth += 1,
                ']' => depth -= 1,
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                }
                _ => {}
            }
        }
        self.add_token(TokenType::NativeAnnotation);
        Ok(())
    }

    fn scan_cpp_annotation(&mut self) -> Result<(), String> {
        // [[...]]
        self.advance(); // consume second [
        while !(self.peek() == Some(']') && self.peek_next() == Some(']')) {
            if self.is_at_end() {
                return Err("Unterminated C++ annotation".to_string());
            }
            self.advance();
        }
        self.advance(); // consume ]
        self.advance(); // consume ]
        self.add_token(TokenType::NativeAnnotation);
        Ok(())
    }

    fn scan_string_literal(&mut self, delimiter: char) -> Result<(), String> {
        // Consume the opening delimiter
        
        while self.peek() != Some(delimiter) && !self.is_at_end() {
            if self.peek() == Some('\\') {
                self.advance(); // Consume backslash
                if !self.is_at_end() {
                    self.advance(); // Consume escaped character
                }
            } else if self.peek() == Some('\n') {
                self.line += 1;
                self.column = 1;
                self.advance();
            } else {
                self.advance();
            }
        }
        
        if self.is_at_end() {
            return Err(format!("Unterminated string literal"));
        }
        
        self.advance(); // Consume closing delimiter
        
        // The entire string including quotes is NativeCode
        self.add_token(TokenType::NativeCode);
        Ok(())
    }
    
    fn scan_number(&mut self) -> Result<(), String> {
        while self.peek().map(|c| c.is_numeric() || c == '.').unwrap_or(false) {
            self.advance();
        }
        
        // Numbers are also considered native code
        self.add_token(TokenType::NativeCode);
        Ok(())
    }
    
    fn scan_identifier(&mut self) -> Result<(), String> {
        while self.peek().map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false) {
            self.advance();
        }
        
        let text = self.source[self.start..self.current].to_string();
        let token_type = match text.as_str() {
            "operations" => TokenType::Operations,
            "interface" => TokenType::Interface,
            "machine" => TokenType::Machine,
            "actions" => TokenType::Actions,
            "domain" => TokenType::Domain,
            _ => TokenType::Identifier,
        };
        
        self.add_token(token_type);
        Ok(())
    }

    fn scan_native_code(&mut self) -> Result<(), String> {
        // Capture native code while respecting string literals and comments
        let mut in_string = false;
        let mut in_char = false;
        let mut string_delimiter = '\0';
        let mut escape_next = false;
        
        while !self.is_at_end() {
            if escape_next {
                escape_next = false;
                self.advance();
                continue;
            }
            
            let c = self.peek().unwrap_or('\0');
            let next = self.peek_next();
            
            // Handle string/char literals to avoid false positives
            if !in_string && !in_char {
                // Check for string start
                if c == '"' || c == '\'' || c == '`' {
                    if c == '\'' {
                        in_char = true;
                    } else {
                        in_string = true;
                        string_delimiter = c;
                    }
                    self.advance();
                    continue;
                }
                
                // Check for Frame constructs outside strings
                if c == '$' {
                    // Could be Frame construct or native code
                    if let Some(next_char) = next {
                        if next_char == '>' || next_char == '<' || next_char == '(' ||
                           next_char == '^' || next_char.is_alphabetic() || next_char == '_' {
                            break;  // Frame construct found
                        }
                        if next_char == '$' {
                            // Check for $$[+] or $$[-]
                            if self.source.chars().nth(self.current + 2) == Some('[') {
                                break;  // Stack operation
                            }
                        }
                    }
                }
                
                // Check for transitions
                if c == '-' && next == Some('>') {
                    break;
                }
                if c == '=' && next == Some('>') {
                    break;
                }
                if c == ':' && next == Some('>') {
                    break;
                }
                
                // Check for block delimiters
                if c == '{' || c == '}' || c == ';' {
                    // These might end the native code block
                    if self.current > self.start {
                        self.add_token(TokenType::NativeCode);
                        return Ok(());
                    }
                }
            } else if in_string {
                // Handle string end
                if c == '\\' {
                    escape_next = true;
                } else if c == string_delimiter {
                    in_string = false;
                }
            } else if in_char {
                // Handle char end
                if c == '\\' {
                    escape_next = true;
                } else if c == '\'' {
                    in_char = false;
                }
            }
            
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
            
            self.advance();
        }
        
        if self.current > self.start {
            self.add_token(TokenType::NativeCode);
        }
        
        Ok(())
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        self.column += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        let token_len = self.current - self.start;
        let start_column = if self.column > token_len {
            self.column - token_len
        } else {
            1
        };
        let location = SourceLocation::new(
            self.file_path.clone(),
            self.line,
            start_column,
            self.start,
            token_len,
        );
        
        self.tokens.push(Token {
            token_type,
            lexeme,
            location,
        });
    }
}
use crate::compiler::Exe;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;


pub(crate) struct Scanner {
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    token_str: String,
    pub has_errors: bool,
    pub errors: String,
    line: usize,
    keywords: HashMap<String, TokenType>,
    //    match_type:MatchType,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Scanner {
        let keywords: HashMap<String, TokenType> = [
            ("None".to_string(), TokenType::None_),
            ("true".to_string(), TokenType::True),
            ("false".to_string(), TokenType::False),
            ("var".to_string(), TokenType::Var),
            ("const".to_string(), TokenType::Const),
            ("if".to_string(), TokenType::If),
            ("elif".to_string(), TokenType::Elif),
            ("else".to_string(), TokenType::Else),
            ("loop".to_string(), TokenType::Loop),
            ("for".to_string(), TokenType::For),
            ("while".to_string(), TokenType::While),
            ("in".to_string(), TokenType::In),
            ("continue".to_string(), TokenType::Continue),
            ("break".to_string(), TokenType::Break),
            ("enum".to_string(), TokenType::Enum),
            ("fn".to_string(), TokenType::Function),
            ("system".to_string(), TokenType::System),
            ("interface:".to_string(), TokenType::InterfaceBlock),
            ("machine:".to_string(), TokenType::MachineBlock),
            ("actions:".to_string(), TokenType::ActionsBlock),
            ("operations:".to_string(), TokenType::OperationsBlock),
            ("domain:".to_string(), TokenType::DomainBlock),
            ("self".to_string(), TokenType::Self_),
            ("return".to_string(), TokenType::Return_),
            ("import".to_string(), TokenType::Import),
            ("from".to_string(), TokenType::From),
            ("as".to_string(), TokenType::As),
            ("module".to_string(), TokenType::Module),
            ("async".to_string(), TokenType::Async),
            ("await".to_string(), TokenType::Await),
            ("lambda".to_string(), TokenType::Lambda),
            ("try".to_string(), TokenType::Try),
            ("except".to_string(), TokenType::Except),
            ("finally".to_string(), TokenType::Finally),
            ("raise".to_string(), TokenType::Raise),
            ("with".to_string(), TokenType::With),
            ("and".to_string(), TokenType::And),
            ("or".to_string(), TokenType::Or),
            ("not".to_string(), TokenType::Not),
            ("xor".to_string(), TokenType::LogicalXor),
            ("is".to_string(), TokenType::Is),
        ]
        .iter()
        .cloned()
        .collect();

        let chars: Vec<char> = source.chars().collect();
        Scanner {
            source,
            chars,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            token_str: String::new(),
            has_errors: false,
            errors: String::new(),
            line: 1,
            keywords,
            //     match_type:MatchType::None,
        }
    }

    // NOTE! The self param is NOT &self. That is how
    // the member variable token can move ownership to the
    // caller.
    pub fn scan_tokens(mut self) -> (bool, String, Vec<Token>) {
        // Scan header
        while self.is_whitespace() {
            self.advance();
        }
        // if self.peek() == '`' {
        //     self.sync_start();
        //     if !self.match_first_header_token() {
        //         return (self.has_errors, self.errors.clone(), self.tokens);
        //     }
        //     self.sync_start();
        //     while !self.is_at_end() {
        //         if self.peek() == '`' {
        //             self.add_string_token_literal(TokenType::SuperString, TokenLiteral::None);
        //             self.sync_start();
        //             if self.match_last_header_token() {
        //                 break;
        //             }
        //         }
        //         self.advance();
        //     }
        // }

        while !self.is_at_end() {
            self.sync_start();
            self.scan_token();
        }

        // todo: the literal needs to be an optional type of generic object
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            TokenLiteral::None,
            self.line,
            self.start,
            len,
        ));
        (self.has_errors, self.errors.clone(), self.tokens)
    }

    fn is_whitespace(&self) -> bool {
        if self.peek() == ' ' || self.peek() == '\n' || self.peek() == '\r' || self.peek() == '\t' {
            return true;
        }
        false
    }

    // fn match_first_header_token(&mut self) -> bool {
    //     for _i in 0..3 {
    //         if !self.match_char('`') {
    //             self.error(self.line, "Malformed header token.");
    //             return false;
    //         }
    //     }
    //     self.add_string_token_literal(TokenType::ThreeTicks, TokenLiteral::None);
    //
    //     true
    // }

    // fn match_last_header_token(&mut self) -> bool {
    //     for _i in 0..3 {
    //         if !self.match_char('`') {
    //             return false;
    //         }
    //     }
    //     self.add_string_token_literal(TokenType::ThreeTicks, TokenLiteral::None);
    //
    //     true
    // }

    fn sync_start(&mut self) {
        self.start = self.current;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            '[' => self.add_token(TokenType::LBracket),
            ']' => self.add_token(TokenType::RBracket),
            '|' => {
                if self.match_char('|') {
                    // || is no longer supported - use 'or' keyword instead
                    // Check if it's part of old HSM syntax (||.) or (||[)
                    if self.peek() == '.' || self.peek() == '[' {
                        self.error(self.line, "Hierarchical state machine syntax '||.' and '||[' have been removed.");
                    } else {
                        self.error(self.line, "Operator '||' has been removed. Use 'or' keyword instead.");
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenType::PipeEqual);  // |= bitwise OR compound assignment
                } else {
                    self.add_token(TokenType::Pipe)
                }
            }
            '*' => {
                if self.match_char('*') {
                    if self.match_char('=') {
                        self.add_token(TokenType::StarStarEqual);  // **= compound assignment
                    } else {
                        self.add_token(TokenType::StarStar);
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenType::StarEqual);  // *= compound assignment
                } else {
                    self.add_token(TokenType::Star);
                }
            }
            '+' => {
                if self.match_char('+') {
                    self.add_token(TokenType::PlusPlus);
                } else if self.match_char('=') {
                    self.add_token(TokenType::PlusEqual);  // += compound assignment
                } else {
                    self.add_token(TokenType::Plus);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);  // != is still valid for not-equal
                } else if self.peek() == '/' && self.peek_next() == '/' {
                    // !/! pattern matching syntax removed
                    self.error(self.line, "Pattern matching syntax '!//' has been removed.");
                } else {
                    // ! for negation is no longer supported - use 'not' keyword
                    self.error(self.line, "Operator '!' has been removed. Use 'not' keyword instead.");
                }
            }
            '$' => {
                enum StackType {
                    Push,
                    Pop,
                }
                if self.match_char('>') {
                    self.add_token(TokenType::EnterStateMsg);
                } else if self.match_char('@') {
                    self.add_token(TokenType::DollarAt);
                } else if self.match_char('^') {
                    self.add_token(TokenType::ParentState);
                } else if self.match_char('$') {

                    let st;
                    if self.match_char('[') {
                        if self.match_char('+') {
                            st = StackType::Push;
                        } else if self.match_char('-') {
                            st = StackType::Pop;
                        } else {
                            self.error(self.line, &format!("Unexpected character {}.", c));
                            return;
                        }
                        if !self.match_char(']') {
                            self.error(self.line, &format!("Unexpected character {}.", c));
                            return;
                        }
                        match st {
                            StackType::Push => {
                                self.add_token(TokenType::StateStackOperationPush);
                                return;
                            }
                            StackType::Pop => {
                                self.add_token(TokenType::StateStackOperationPop);
                                return;
                            }
                        }
                    }
                } else {
                    self.add_token(TokenType::State)
                }
            }
            '^' => {
                // Old caret return syntax removed - use 'return' keyword
                self.error(self.line, "Unexpected character '^'. Old return syntax has been removed. Use 'return' or 'return value' instead.");
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else if self.match_char('>') {
                    if self.match_char('=') {
                        self.add_token(TokenType::RightShiftEqual);  // >>= right shift compound assignment
                    } else {
                        self.add_token(TokenType::RightShift);  // >> right shift
                    }
                } else {
                    self.add_token(TokenType::GT);
                }
            }
            '<' => {
                if self.match_char('$') {
                    self.add_token(TokenType::ExitStateMsg);
                } else if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        self.add_token(TokenType::LeftShiftEqual);  // <<= left shift compound assignment
                    } else {
                        self.add_token(TokenType::LeftShift);  // << left shift
                    }
                } else {
                    self.add_token(TokenType::LT);
                }
            }
            '&' => {
                if self.match_char('&') {
                    // && is no longer supported - use 'and' keyword instead
                    self.error(self.line, "Operator '&&' has been removed. Use 'and' keyword instead.");
                } else if self.match_char('|') {
                    // &| operator has been removed - use 'xor' keyword instead
                    self.error(self.line, "Operator '&|' has been removed. Use 'xor' keyword instead.");
                } else if self.match_char('=') {
                    self.add_token(TokenType::AmpersandEqual);  // &= bitwise AND compound assignment
                } else {
                    self.add_token(TokenType::Ampersand)
                }
            }
            '?' => {
                // Question mark is no longer used for ternary operators
                // Could potentially be used for optional types in the future
                self.error(self.line, "Unexpected character '?'. Ternary operators have been removed. Use if/elif/else statements instead.");
            }
            '~' => {
                if self.match_char('/') {
                    if self.match_char('/') {
                        self.add_token(TokenType::MatchEmptyString);
                    } else {
                        // String match syntax removed
                        self.error(self.line, "String match syntax '~/' has been removed. Use if/elif/else statements instead.");
                    }
                } else {
                    // Bitwise NOT operator
                    self.add_token(TokenType::Tilde);
                }
            }
            '@' => {
                if self.peek() == '@' {
                    // Found @@ - consume second @
                    self.advance(); // consume second '@'
                    self.add_token(TokenType::AtAt);
                } else {
                    self.add_token(TokenType::At);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                //    self.line += 1;
            }
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenType::Transition);
                } else if self.match_char('-') {
                    self.add_token(TokenType::DashDash);
                } else if self.match_char('=') {
                    self.add_token(TokenType::DashEqual);  // -= compound assignment
                } else {
                    // Always emit Dash token, let parser handle negative numbers in context
                    self.add_token(TokenType::Dash);
                }
            }
            '{' => {
                if self.match_char('-') {
                    if self.match_char('-') {
                        self.multi_line_comment();
                    } else {
                        panic!("Unexpected character.");
                    }
                } else {
                    self.add_token(TokenType::OpenBrace);
                }
            }
            '}' => {
                self.add_token(TokenType::CloseBrace);
            }
            ':' => {
                if self.match_char('|') {
                    // Test terminator removed
                    self.error(self.line, "Test terminator ':|' has been removed. Use if/elif/else statements instead.");
                } else if self.match_char('/') {
                    // Enum match syntax removed
                    self.error(self.line, "Enum match syntax ':/' has been removed. Use if/elif/else statements instead.");
                } else {
                    self.add_token(TokenType::Colon);
                }
            }
            ';' => self.add_token(TokenType::Semicolon),
            '"' => self.string(),
            // Backtick support removed - no longer needed in Frame
            '#' => {
                // Hash is only used for attributes now
                if self.match_char('[') {
                    self.add_token(TokenType::OuterAttributeOrDomainParams) // #[
                } else if self.match_char('!') {
                    if self.match_char('[') {
                        // #![
                        self.add_token(TokenType::InnerAttribute);
                    } else {
                        self.error(self.line, &format!("Unexpected character {}.", c));
                    }
                } else if self.match_char('/') {
                    // Number match syntax removed
                    self.error(self.line, "Number match syntax '#/' has been removed. Use if/elif/else statements instead.");
                } else {
                    // Old system declaration syntax removed
                    self.error(self.line, "Unexpected character '#'. Old system declaration syntax has been removed. Use 'system Name { }' instead.");
                }
            }
            '=' => {
                if self.match_char('>') {
                    self.add_token(TokenType::Dispatch);
                } else if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equals);
                }
            }
            '/' => {
                if self.match_char('/') {
                    self.single_line_comment();
                } else if self.match_char('*') {
                    self.c_style_multiline_comment();
                } else if self.match_char('=') {
                    self.add_token(TokenType::SlashEqual);  // /= compound assignment
                } else {
                    self.add_token(TokenType::ForwardSlash);
                }
            }
            '.' => {
                if self.is_digit(self.peek()) {
                    self.number(false);
                } else {
                    self.add_token(TokenType::Dot);
                }
            }
            '%' => {
                if self.match_char('=') {
                    self.add_token(TokenType::PercentEqual);  // %= compound assignment
                } else {
                    self.add_token(TokenType::Percent);
                }
            }
            ',' => self.add_token(TokenType::Comma),
            _ => {
                if !self.block_keyword(c) {
                    if self.is_digit(c) {
                        self.number(true);
                    } else if self.is_alpha(c) {
                        self.identifier();
                    } else {
                        self.error(self.line, &format!("Found unexpected character '{}'.", c));
                    }
                }
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.chars[self.current];
        if c == 'i' {
            let _debug = 1;
        }
        if c != expected {
            return false;
        }
        self.current += 1;
        self.token_str = self.chars[self.start..self.current].iter().collect();

        true
    }

    // Fixed: now properly handles UTF-8 characters
    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        self.token_str = self.chars[self.start..self.current].iter().collect();
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.chars[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            return '\0';
        }
        return self.chars[self.current + 1];
    }

    fn is_digit(&self, c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn number(&mut self, mut is_integer: bool) {
        if is_integer {
            // consume whole number
            while self.is_digit(self.peek()) {
                self.advance();
            }

            if self.peek() == '.' {
                is_integer = false;
                // consume the '.'
                self.advance();
            }
        }

        // consume mantissa, if present
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if is_integer {
            let s: String = self.chars[self.start..self.current].iter().collect();
            let result = s.parse::<i32>();
            match result {
                Ok(number) => {
                    self.add_token_literal(TokenType::Number, TokenLiteral::Integer(number));
                }
                Err(err) => {
                    self.error(self.line, &format!("Malformed integer number {}", err));
                }
            }
        } else {
            // is float
            let s: String = self.chars[self.start..self.current].iter().collect();
            let result = s.parse::<f32>();
            match result {
                Ok(number) => {
                    self.add_token_literal(TokenType::Number, TokenLiteral::Float(number));
                }
                Err(err) => {
                    self.error(self.line, &format!("Malformed float number: {}", err));
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        // See if the identifier is a reserved word.
        let text: String = self.chars[self.start..self.current].iter().collect();

        // Special handling for "system" keyword
        if text == "system" {
            // Check if this is "system.return"
            let saved_current = self.current;
            if self.peek() == '.' {
                self.advance(); // consume '.'
                // Check if next word is "return"
                if self.peek() == 'r' {
                    let start_of_return = self.current;
                    while self.is_alpha_numeric(self.peek()) {
                        self.advance();
                    }
                    let next_word: String = self.chars[start_of_return..self.current].iter().collect();
                    if next_word == "return" {
                        // This is "system.return" - make it a single token
                        self.add_token(TokenType::SystemReturn);
                        return;
                    }
                }
                // Not "system.return", restore position
                self.current = saved_current;
            }
            // Just "system" by itself
            self.add_token(TokenType::System);
        } else if let Some(keyword) = self.keywords.get(&text) {
            let tok_type = *keyword;
            self.add_token(tok_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    // TODO: handle EOF w/ error
    fn single_line_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        self.add_token(TokenType::SingleLineComment);
    }

    // TODO: handle EOF w/ error
    // TODO: Update/remove multiline comments.
    fn multi_line_comment(&mut self) {
        while !self.is_at_end() {
            while self.peek() != '-' {
                self.advance();
            }
            self.advance();
            if self.peek() != '-' {
                continue;
            }
            self.advance();
            if self.peek() != '}' {
                continue;
            }
            self.advance();

            self.add_token(TokenType::MultiLineComment);
            return;
        }
    }

    // Handle C-style multiline comments /* ... */
    fn c_style_multiline_comment(&mut self) {
        // We've already consumed '/*', now look for '*/'
        while !self.is_at_end() {
            // Check for nested line breaks to maintain line count
            if self.peek() == '\n' {
                self.line += 1;
            }
            
            // Look for the closing */
            if self.peek() == '*' && self.peek_next() == '/' {
                // Consume the '*'
                self.advance();
                // Consume the '/'
                self.advance();
                self.add_token(TokenType::CStyleMultiLineComment);
                return;
            }
            self.advance();
        }
        
        // If we reach here, we hit EOF without finding the closing */
        self.error(self.line, "Unterminated C-style comment. Expected '*/' before end of file.");
    }

    // Scan the string looking for the end of the match test ('/')
    // or the end of the current match string ('|').
    // match_string_test -> '/' match_string_pattern ('|' match_string_pattern)* '/'

    fn scan_string_match(&mut self) {
        while self.peek() != '/' {
            if self.peek() == '|' {
                self.add_token_sync_start(TokenType::MatchString);
                self.advance();
                self.add_token_sync_start(TokenType::Pipe);
            }
            self.advance();
        }
        self.add_token_sync_start(TokenType::MatchString);
        self.advance();
        self.add_token_sync_start(TokenType::ForwardSlash);
    }

    fn block_keyword(&mut self, first_char:char) -> bool {
        // TODO: handle this:
        // #M1
        //     -in-
        // ##

        let start_pos = self.current;
        // let mut block_name:&str;

        let block_sections = [
            ("interface:", TokenType::InterfaceBlock),
            ("machine:", TokenType::MachineBlock),
            ("actions:", TokenType::ActionsBlock),
            ("operations:", TokenType::OperationsBlock),
            ("domain:", TokenType::DomainBlock),
        ];

        // TODO: this is **horribly** inefficient.

        for (block_name, token_type) in block_sections.iter() {

            for (i, c) in block_name.chars().enumerate() {
                if i == 0 {
                    if !block_name.starts_with(first_char) {
                        break;
                    }
                } else if !self.match_char(c) {
                    break;
                }
                if i == block_name.len() - 1 {
                    self.add_token(*token_type);
                    return true;
                }
            }

            self.current = start_pos;
        }

        self.current = start_pos;
        false
    }

    fn is_alpha(&self, c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn add_token_sync_start(&mut self, tok_type: TokenType) {
        self.add_token_literal(tok_type, TokenLiteral::None);
        self.sync_start();
    }

    fn add_token(&mut self, tok_type: TokenType) {
        Exe::debug_print(&format!("{:?}", tok_type));
        self.add_token_literal(tok_type, TokenLiteral::None);
    }

    fn add_token_literal(&mut self, tok_type: TokenType, literal: TokenLiteral) {
        let lex = self.chars[self.start..self.current].iter().collect::<String>();
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            tok_type, lex, literal, self.line, self.start, len,
        ));
    }

    fn add_string_token_literal(&mut self, tok_type: TokenType, literal: TokenLiteral) {
        let lex = self.chars[self.start + 1..self.current - 1].iter().collect::<String>();
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            tok_type, lex, literal, self.line, self.start, len,
        ));
    }

    fn error(&mut self, line: usize, error_msg: &str) {
        let error = &format!("Line {} : Error: {}\n", line, error_msg);
        self.has_errors = true;
        self.errors.push_str(error);
        self.add_token(TokenType::Error);
    }

    fn string(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();
            if c == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
            } else if c == '\n' {
                // self.line += 1;
            } else if c == '"' {
                break;
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        self.add_string_token_literal(TokenType::String, TokenLiteral::None);
    }

    // Backtick/SuperString support removed - no longer needed in Frame
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum TokenType {
    Eof,
    Identifier,
    State,
    GT, // >
    // GTx2,                         // >>
    // GTx3,                         // >>>
    Plus,         // +
    PlusPlus,     // ++
    Dash,         // -
    DashDash,     // --
    Star,         // *
    StarStar,     // **
    EqualEqual,   // ==
    Bang,         // !
    BangEqual,    // !=
    GreaterEqual, // >=
    LessEqual,    // <=
    LT,           // <
    Percent,      // %
    // LTx2,                         // <<
    // LTx3,                         // <<<
    Ampersand,                    // &
    Pipe,                         // |
    // REMOVED: Caret (^) - use 'return' keyword
    // REMOVED: ReturnAssign (^=) - use 'return = value'
    LogicalAnd,                   // &&
    LogicalXor,                   // xor keyword
    System,                       // 'system' keyword for modern syntax (reserved)
    SystemReturn,                 // 'system.return' for setting interface return value
    Self_,                        // self
    Return_,                      // return
    EnterStateMsg,                   // $>
    ExitStateMsg,                    // <$
    OuterAttributeOrDomainParams, // #[
    InnerAttribute,               // #![
    InterfaceBlock,               // interface:
    MachineBlock,                 // machine:
    ActionsBlock,                 // actions:
    OperationsBlock,              // operations:
    DomainBlock,                  // domain:
    LParen,                       // (
    RParen,                       // )
    LBracket,                     // [
    RBracket,                     // ]
    Transition,                   // ->
    //    ChangeState,                  // ->>
    String,      // "foo"
    // REMOVED: ThreeTicks (```) - not used
    Number,                 // 1, 1.01
    Var,                    // var keyword
    Const,                  // const keyword
    //    New,              // new keyword
    Loop,                   // loop keyword
    For,                    // for keyword
    While,                  // while keyword
    If,                     // if keyword
    Elif,                   // elif keyword
    Else,                   // else keyword
    Continue, // continue keyword
    Break,    // break keyword
    In,       // 'in' keyword
    Enum,     // 'enum' keyword
    Function, // 'fn' keyword
    Import,   // 'import' keyword
    From,     // 'from' keyword
    As,       // 'as' keyword
    Module,   // 'module' keyword
    Async,    // 'async' keyword
    Await,    // 'await' keyword
    Lambda,   // 'lambda' keyword
    Try,      // 'try' keyword
    Except,   // 'except' keyword
    Finally,  // 'finally' keyword
    Raise,    // 'raise' keyword
    With,     // 'with' keyword
    And,      // 'and' keyword (Python logical AND)
    Or,       // 'or' keyword (Python logical OR)
    Not,      // 'not' keyword (Python logical NOT)
    // SingleLineComment, // --- comment
    MultiLineComment, // {-- comments --}
    CStyleMultiLineComment, // /* C-style comments */
    OpenBrace,        // {
    CloseBrace,       // }
    True,             // true
    False,            // false
    None_,            // None (standard null value)
    Colon,            // :
    Semicolon,        // ;
    Comma,            // ,
    Dispatch,         // =>
    Equals,           // =
    //    DeclAssignment,          // ':='
    ForwardSlash,            // /
    MatchString,             // '/<any characters>/' - contains <string>
    MatchEmptyString,        // '~//'
    MatchNull,               // '!//'
    SingleLineComment,       // '//'
    // REMOVED: Pattern matching tokens
    // REMOVED: StringMatchStart ('~/') - string patterns removed
    // REMOVED: NumberMatchStart ('#/') - number patterns removed  
    // REMOVED: EnumMatchStart (':/') - enum patterns removed
    // REMOVED: ColonBar (:|) - test terminator removed
    StateStackOperationPush, // $$[+]
    StateStackOperationPop,  // $$[-]
    ParentState,             // $^ - parent state reference
    Dot,                     // .
    At,                      // @
    AtAt,                    // @@
    DollarAt,                // $@ - current event reference
    PipePipe,                // ||
    PipePipeDot,             // ||.
    PipePipeLBracket,        // ||[
    // REMOVED: Hash (#) - old system syntax removed
    
    // Compound assignment operators (v0.39)
    PlusEqual,               // +=
    DashEqual,               // -=
    StarEqual,               // *=
    SlashEqual,              // /=
    PercentEqual,            // %=
    StarStarEqual,           // **=
    AmpersandEqual,          // &=
    PipeEqual,               // |=
    LeftShiftEqual,          // <<=
    RightShiftEqual,         // >>=
    
    // Bitwise operators (v0.39)
    Tilde,                   // ~ (bitwise NOT)
    LeftShift,               // <<
    RightShift,              // >>
    
    // Identity operators (v0.39)
    Is,                      // 'is' keyword
    IsNot,                   // 'is not' (handled as two tokens)
    
    Error,
}

impl Display for TokenType {
    #[allow(clippy::all)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{}", self)
        match self {
            TokenType::Plus => write!(f, "+"),
            _ => write!(f, "TODO"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    Integer(i32),
    Float(f32),
    // Double(f64),
    None,
}

impl Display for TokenLiteral {
    #[allow(clippy::all)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{}", self)
        write!(f, "TODO")
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
    pub start: usize,
    pub length: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: TokenLiteral,
        line: usize,
        start: usize,
        length: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            start,
            length,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

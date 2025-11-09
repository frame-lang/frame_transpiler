#![allow(dead_code)] // Scanner tokens and methods are part of the language API

use crate::compiler::Exe;
use crate::frame_c::visitors::TargetLanguage;
use std::collections::HashMap;
use std::convert::TryFrom;
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
    pub target_language: Option<TargetLanguage>,
    pub scanning_mode: ScanningMode,
    pub target_regions: Vec<TargetRegion>,
    pending_target_annotation: bool,
    active_target_region: Option<ActiveTargetRegion>,
    //    match_type:MatchType,
}

impl Scanner {
    #[inline]
    fn is_space_like_char(c: char) -> bool {
        // Unicode-aware whitespace including NBSP and other editor-inserted spaces
        c.is_whitespace()
            || matches!(
                c,
                '\u{00A0}' // NBSP
                    | '\u{2000}' // EN QUAD
                    | '\u{2001}' // EM QUAD
                    | '\u{2002}' // EN SPACE
                    | '\u{2003}' // EM SPACE
                    | '\u{2004}' // THREE-PER-EM SPACE
                    | '\u{2005}' // FOUR-PER-EM SPACE
                    | '\u{2006}' // SIX-PER-EM SPACE
                    | '\u{2007}' // FIGURE SPACE
                    | '\u{2008}' // PUNCTUATION SPACE
                    | '\u{2009}' // THIN SPACE
                    | '\u{200A}' // HAIR SPACE
                    | '\u{202F}' // NARROW NO-BREAK SPACE
                    | '\u{205F}' // MEDIUM MATHEMATICAL SPACE
                    | '\u{3000}' // IDEOGRAPHIC SPACE
            )
    }
    pub(crate) fn new(source: String) -> Scanner {
        // Normalize UTF-8 BOM at start to avoid interfering with SOL detection
        let normalized = if source.starts_with('\u{FEFF}') {
            source.trim_start_matches('\u{FEFF}').to_string()
        } else {
            source
        };
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
            ("del".to_string(), TokenType::Del),
            ("enum".to_string(), TokenType::Enum),
            ("fn".to_string(), TokenType::Function),
            ("class".to_string(), TokenType::Class),
            ("assert".to_string(), TokenType::Assert),
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
            ("native".to_string(), TokenType::Native),
            ("async".to_string(), TokenType::Async),
            ("await".to_string(), TokenType::Await),
            ("lambda".to_string(), TokenType::Lambda),
            ("try".to_string(), TokenType::Try),
            ("except".to_string(), TokenType::Except),
            ("finally".to_string(), TokenType::Finally),
            ("raise".to_string(), TokenType::Raise),
            ("throw".to_string(), TokenType::Raise),
            ("with".to_string(), TokenType::With),
            ("and".to_string(), TokenType::And),
            ("or".to_string(), TokenType::Or),
            ("not".to_string(), TokenType::Not),
            ("is".to_string(), TokenType::Is),
            ("yield".to_string(), TokenType::Yield),
            ("match".to_string(), TokenType::Match),
            ("case".to_string(), TokenType::Case),
            ("super".to_string(), TokenType::Super),
            // ("type".to_string(), TokenType::Type), // Removed to allow type() function calls
            ("cls".to_string(), TokenType::Cls),
            ("property".to_string(), TokenType::Property),
            ("classmethod".to_string(), TokenType::ClassMethod),
            ("setter".to_string(), TokenType::Setter),
            ("deleter".to_string(), TokenType::Deleter),
        ]
        .iter()
        .cloned()
        .collect();

        let chars: Vec<char> = normalized.chars().collect();
        Scanner {
            source: normalized,
            chars,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            token_str: String::new(),
            has_errors: false,
            errors: String::new(),
            line: 1,
            keywords,
            target_language: None,
            scanning_mode: ScanningMode::TargetDiscovery,
            target_regions: Vec::new(),
            pending_target_annotation: false,
            active_target_region: None,
            //     match_type:MatchType::None,
        }
    }

    fn handle_target_language_declaration(&mut self) {
        self.pending_target_annotation = false;

        let Some(last_token) = self.tokens.last() else {
            self.error(self.line, "Expected target language after @target");
            return;
        };

        if last_token.token_type != TokenType::Identifier {
            self.error(
                last_token.line,
                "Expected target language identifier after @target",
            );
            return;
        }

        let language_lexeme = last_token.lexeme.clone();
        match TargetLanguage::try_from(language_lexeme.as_str()) {
            Ok(language) => {
                if let Some(existing) = self.target_language {
                    if existing != language {
                        self.error(
                            last_token.line,
                            "Multiple @target annotations with different languages are not allowed",
                        );
                        return;
                    }
                } else {
                    self.target_language = Some(language);
                    self.switch_scanning_mode(ScanningMode::TargetSpecific(language));
                }
            }
            Err(_) => {
                let message = format!(
                    "Unknown target language '{}' in @target annotation",
                    language_lexeme
                );
                self.error(last_token.line, &message);
            }
        }
    }

    // NOTE! The self param is NOT &self. That is how
    // the member variable token can move ownership to the
    // caller.
    pub fn scan_tokens(mut self) -> (bool, String, Vec<Token>, Vec<TargetRegion>) {
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

            match self.scanning_mode {
                ScanningMode::TargetDiscovery => {
                    if self.is_whitespace() {
                        self.advance();
                        continue;
                    }

                    self.scan_token();

                    if !self.pending_target_annotation
                        && !matches!(
                            self.tokens.last().map(|t| t.token_type),
                            Some(TokenType::TargetAnnotation)
                        )
                    {
                        self.switch_scanning_mode(ScanningMode::FrameCommon);
                    }
                }
                ScanningMode::TargetSpecific(target) => {
                    self.consume_target_specific_region(target);
                }
                ScanningMode::FrameCommon => {
                    self.scan_token();
                }
            }
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
        (
            self.has_errors,
            self.errors.clone(),
            self.tokens,
            self.target_regions,
        )
    }

    fn is_whitespace(&self) -> bool {
        // Treat all Unicode whitespace and space-like characters as whitespace
        Self::is_space_like_char(self.peek())
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
        // Skip any space-like characters (Unicode-aware, includes NBSP and variants).
        if Self::is_space_like_char(c) {
            return;
        }
        match c {
            // Revert Python-specific quote skipping at scanner level; handle strings via existing paths
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            '[' => self.add_token(TokenType::LBracket),
            ']' => self.add_token(TokenType::RBracket),
            '|' => {
                if self.match_char('|') {
                    // || is no longer supported - use 'or' keyword instead
                    // Check if it's part of old HSM syntax (||.) or (||[)
                    if self.peek() == '.' || self.peek() == '[' {
                        self.error(self.line, "Hierarchical state machine syntax not supported. Use standard state transitions instead.");
                    } else {
                        self.error(self.line, "Use 'or' keyword instead of '||' operator");
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenType::PipeEqual); // |= bitwise OR compound assignment
                } else {
                    self.add_token(TokenType::Pipe)
                }
            }
            '*' => {
                if self.match_char('*') {
                    if self.match_char('=') {
                        self.add_token(TokenType::StarStarEqual); // **= compound assignment
                    } else {
                        self.add_token(TokenType::StarStar);
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenType::StarEqual); // *= compound assignment
                } else {
                    self.add_token(TokenType::Star);
                }
            }
            '+' => {
                if self.match_char('+') {
                    self.add_token(TokenType::PlusPlus);
                } else if self.match_char('=') {
                    self.add_token(TokenType::PlusEqual); // += compound assignment
                } else {
                    self.add_token(TokenType::Plus);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual); // != not-equal
                } else if self.peek() == '/' && self.peek_next() == '/' {
                    // !// legacy pattern matching syntax removed
                    self.error(self.line, "Pattern matching syntax '!//' has been removed.");
                } else {
                    // Allow '!' in TypeScript native contexts; keep Python policy error elsewhere
                    if matches!(self.target_language, Some(TargetLanguage::TypeScript)) {
                        self.add_token(TokenType::Bang);
                    } else {
                        self.error(self.line, "Use 'not' keyword instead of '!' operator");
                    }
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
                if self.match_char('=') {
                    self.add_token(TokenType::CaretEqual); // ^= bitwise XOR compound assignment
                } else {
                    self.add_token(TokenType::Caret); // ^ bitwise XOR operator
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else if self.match_char('>') {
                    if self.match_char('=') {
                        self.add_token(TokenType::RightShiftEqual); // >>= right shift compound assignment
                    } else {
                        self.add_token(TokenType::RightShift); // >> right shift
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
                        self.add_token(TokenType::LeftShiftEqual); // <<= left shift compound assignment
                    } else {
                        self.add_token(TokenType::LeftShift); // << left shift
                    }
                } else {
                    self.add_token(TokenType::LT);
                }
            }
            '&' => {
                if self.match_char('&') {
                    // In TypeScript targets, '&&' is valid inside native regions. Do not error here.
                    if !matches!(self.target_language, Some(TargetLanguage::TypeScript)) {
                        self.error(self.line, "Use 'and' keyword instead of '&&' operator");
                    }
                    // No token emission needed; native region handling will own this content.
                } else if self.match_char('|') {
                    // &| operator has been removed - use '^' operator instead
                    self.error(
                        self.line,
                        "Operator '&|' has been removed. Use '^' operator for XOR instead.",
                    );
                } else if self.match_char('=') {
                    self.add_token(TokenType::AmpersandEqual); // &= bitwise AND compound assignment
                } else {
                    self.add_token(TokenType::Ampersand)
                }
            }
            '?' => {
                // Question mark is no longer used for ternary operators
                // Could potentially be used for optional types in the future
                self.error(
                    self.line,
                    "Ternary operator '?' not supported. Use if/else statements instead",
                );
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
                if self.match_keyword("target") {
                    self.pending_target_annotation = true;
                    self.add_token(TokenType::TargetAnnotation);
                } else if self.peek() == '=' {
                    self.advance(); // consume '='
                    self.add_token(TokenType::AtEqual);
                } else if self.peek() == '@' {
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
                    self.add_token(TokenType::DashEqual); // -= compound assignment
                } else {
                    // Always emit Dash token, let parser handle negative numbers in context
                    self.add_token(TokenType::Dash);
                }
            }
            '{' => {
                // Check if this starts a multiline comment {--
                // but ONLY if we're not already inside a Python comment
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
                if self.match_char(':') {
                    // Module separator for qualified names
                    self.add_token(TokenType::ColonColon);
                } else if self.match_char('|') {
                    // Test terminator removed
                    self.error(self.line, "Test terminator ':|' has been removed. Use if/elif/else statements instead.");
                } else if self.match_char('/') {
                    // Enum match syntax removed
                    self.error(self.line, "Enum match syntax ':/' has been removed. Use if/elif/else statements instead.");
                } else if self.match_char('=') {
                    // Walrus operator (assignment expression)
                    self.add_token(TokenType::Walrus);
                } else {
                    self.add_token(TokenType::Colon);
                }
            }
            ';' => self.add_token(TokenType::Semicolon),
            '"' => self.string(),
            // Backtick support removed - no longer needed in Frame
            '#' => {
                if self.peek() == '['
                    && self.is_next_target_directive()
                    && self.consume_target_directive()
                {
                    return;
                }
                // Python-style single-line comment
                self.python_comment();
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
                    if self.match_char('=') {
                        self.add_token(TokenType::FloorDivideEqual); // //= floor division compound assignment
                    } else {
                        self.add_token(TokenType::FloorDivide); // // floor division operator
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenType::SlashEqual); // /= compound assignment
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
                    self.add_token(TokenType::PercentEqual); // %= compound assignment
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
                        if self.pending_target_annotation {
                            self.handle_target_language_declaration();
                        }
                    } else {
                        // Be tolerant of native TypeScript strings/templates in FrameCommon mode
                        if matches!(self.target_language, Some(TargetLanguage::TypeScript)) {
                            if c == '\'' || c == '"' {
                                self.skip_ts_string(c);
                            } else if c == '`' {
                                self.skip_ts_template();
                            } else {
                                // Provide helpful error messages for common Unicode quote characters
                                match c {
                                    '\u{2018}' | '\u{2019}' => self.error(
                                        self.line,
                                        "Found Unicode smart quote. Use ASCII single quote (') instead.",
                                    ),
                                    '\u{201C}' | '\u{201D}' => self.error(
                                        self.line,
                                        "Found Unicode smart quote. Use ASCII double quote (\") instead.",
                                    ),
                                    _ => self
                                        .error(self.line, &format!("Found unexpected character '{}'.", c)),
                                }
                            }
                        } else if matches!(self.target_language, Some(TargetLanguage::Python3)) {
                            if c == '\'' || c == '"' {
                                self.skip_py_string(c);
                            } else if c == '#' {
                                // Skip the rest of the line for Python comments
                                while !self.is_at_end() && self.peek() != '\n' {
                                    self.advance();
                                }
                            } else {
                                match c {
                                    '\u{2018}' | '\u{2019}' => self.error(
                                        self.line,
                                        "Found Unicode smart quote. Use ASCII single quote (') instead.",
                                    ),
                                    '\u{201C}' | '\u{201D}' => self.error(
                                        self.line,
                                        "Found Unicode smart quote. Use ASCII double quote (\") instead.",
                                    ),
                                    _ => self
                                        .error(self.line, &format!("Found unexpected character '{}'.", c)),
                                }
                            }
                        } else {
                            // Provide helpful error messages for common Unicode quote characters
                            match c {
                                '\u{2018}' | '\u{2019}' => self.error(
                                    self.line,
                                    "Found Unicode smart quote. Use ASCII single quote (') instead.",
                                ),
                                '\u{201C}' | '\u{201D}' => self.error(
                                    self.line,
                                    "Found Unicode smart quote. Use ASCII double quote (\") instead.",
                                ),
                                _ => self
                                    .error(self.line, &format!("Found unexpected character '{}'.", c)),
                            }
                        }
                    }
                }
            }
        }
    }

    // Skip a TypeScript single- or double-quoted string, handling escapes.
    fn skip_ts_string(&mut self, delimiter: char) {
        loop {
            if self.is_at_end() {
                break;
            }
            let ch = self.advance();
            if ch == '\\' {
                // Skip escaped char
                if !self.is_at_end() {
                    self.advance();
                }
                continue;
            }
            if ch == delimiter {
                break;
            }
            // Update line counter on newline inside strings
            if ch == '\n' {
                // already handled in advance()
            }
        }
    }

    // Skip a Python string, handling single, double, and triple-quoted strings with escapes.
    fn skip_py_string(&mut self, delimiter: char) {
        let is_triple = self.peek() == delimiter && self.peek_next() == delimiter;
        if is_triple {
            // consume the remaining two delimiters
            self.advance();
            self.advance();
            loop {
                if self.is_at_end() {
                    break;
                }
                let ch = self.advance();
                if ch == delimiter && self.peek() == delimiter && self.peek_next() == delimiter {
                    self.advance();
                    self.advance();
                    break;
                }
            }
            return;
        }
        // single-line string with escapes
        loop {
            if self.is_at_end() {
                break;
            }
            let ch = self.advance();
            if ch == '\\' {
                if !self.is_at_end() {
                    self.advance();
                }
                continue;
            }
            if ch == delimiter {
                break;
            }
            // line counting handled by advance()
        }
    }

    // Skip a TypeScript template literal, supporting nested ${ ... } expressions.
    fn skip_ts_template(&mut self) {
        let mut expr_depth: i32 = 0;
        let mut inner_string: Option<char> = None;
        loop {
            if self.is_at_end() {
                break;
            }
            let ch = self.advance();
            if ch == '\\' {
                // Skip escaped char
                if !self.is_at_end() {
                    self.advance();
                }
                continue;
            }

            // If inside a quoted string within an expression, only look for its closing quote
            if let Some(delim) = inner_string {
                if ch == delim {
                    inner_string = None;
                }
                continue;
            }

            // Not inside a quoted string
            match ch {
                '\'' | '"' if expr_depth > 0 => {
                    // Enter inner quoted string within an expression
                    inner_string = Some(ch);
                }
                '`' => {
                    if expr_depth == 0 {
                        // End of the outer template
                        break;
                    } else {
                        // Nested template inside an expression: skip it fully
                        self.skip_ts_template();
                    }
                }
                '$' => {
                    if self.peek() == '{' {
                        self.advance();
                        expr_depth += 1;
                    }
                }
                '}' => {
                    if expr_depth > 0 {
                        expr_depth -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn switch_scanning_mode(&mut self, new_mode: ScanningMode) {
        if self.scanning_mode == new_mode {
            return;
        }

        match (&self.scanning_mode, &new_mode) {
            (ScanningMode::FrameCommon, ScanningMode::TargetSpecific(target))
            | (ScanningMode::TargetDiscovery, ScanningMode::TargetSpecific(target)) => {
                self.start_target_region(*target);
            }
            (ScanningMode::TargetSpecific(_), ScanningMode::FrameCommon) => {
                self.end_target_region();
            }
            _ => {}
        }

        self.scanning_mode = new_mode;
    }

    fn start_target_region(&mut self, target: TargetLanguage) {
        if self.active_target_region.is_some() {
            return;
        }

        self.active_target_region = Some(ActiveTargetRegion {
            start_position: self.current,
            start_line: self.line,
            target,
        });
    }

    fn end_target_region(&mut self) {
        let Some(active) = self.active_target_region.take() else {
            return;
        };

        if active.start_position >= self.current {
            return;
        }

        let raw_content: String = self.chars[active.start_position..self.current]
            .iter()
            .collect();

        if raw_content.trim().is_empty() {
            return;
        }

        let source_map = Self::build_target_source_map(active.start_line, &raw_content);

        self.target_regions.push(TargetRegion {
            start_position: active.start_position,
            end_position: Some(self.current),
            raw_content,
            target: active.target,
            source_map,
        });
    }

    fn consume_target_specific_region(&mut self, target: TargetLanguage) {
        if self.active_target_region.is_none() {
            self.start_target_region(target);
        }

        let mut brace_depth: usize = 0;
        let mut string_state: Option<StringState> = None;
        let mut block_comment: Option<BlockCommentKind> = None;
        let mut line_comment = false;

        while !self.is_at_end() {
            if brace_depth == 0
                && string_state.is_none()
                && block_comment.is_none()
                && !line_comment
                && self.detect_frame_boundary()
            {
                break;
            }

            let ch = self.advance();

            if line_comment {
                if ch == '\n' {
                    line_comment = false;
                }
                continue;
            }

            if let Some(kind) = block_comment {
                match kind {
                    BlockCommentKind::SlashStar => {
                        if ch == '*' && self.peek() == '/' {
                            self.advance();
                            block_comment = None;
                        }
                    }
                }
                continue;
            }

            if let Some(mut state) = string_state.take() {
                if state.escape {
                    state.escape = false;
                    string_state = Some(state);
                    continue;
                }

                if ch == '\\' {
                    state.escape = true;
                    string_state = Some(state);
                    continue;
                }

                let mut string_completed = false;

                if state.triple {
                    if ch == state.delimiter
                        && self.peek() == state.delimiter
                        && self.peek_at(1) == state.delimiter
                    {
                        self.advance();
                        self.advance();
                        string_completed = true;
                    }
                } else if ch == state.delimiter {
                    string_completed = true;
                }

                if !string_completed {
                    string_state = Some(state);
                }

                continue;
            }

            match ch {
                '/' if matches!(
                    target,
                    TargetLanguage::TypeScript
                        | TargetLanguage::C
                        | TargetLanguage::Cpp
                        | TargetLanguage::Java
                        | TargetLanguage::CSharp
                        | TargetLanguage::Rust
                ) =>
                {
                    if self.peek() == '/' {
                        self.advance();
                        line_comment = true;
                        continue;
                    }
                    if self.peek() == '*' {
                        self.advance();
                        block_comment = Some(BlockCommentKind::SlashStar);
                        continue;
                    }
                }
                '#' if matches!(target, TargetLanguage::Python3) => {
                    line_comment = true;
                    continue;
                }
                '\'' | '"' => {
                    let triple = matches!(target, TargetLanguage::Python3)
                        && self.peek() == ch
                        && self.peek_at(1) == ch;
                    if triple {
                        self.advance();
                        self.advance();
                    }
                    string_state = Some(StringState {
                        delimiter: ch,
                        triple,
                        escape: false,
                    });
                }
                '`' if matches!(target, TargetLanguage::TypeScript) => {
                    string_state = Some(StringState {
                        delimiter: '`',
                        triple: false,
                        escape: false,
                    });
                }
                '{' => {
                    brace_depth = brace_depth.saturating_add(1);
                }
                '}' => {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                    }
                }
                _ => {}
            }
        }

        self.switch_scanning_mode(ScanningMode::FrameCommon);
    }

    fn detect_frame_boundary(&self) -> bool {
        if self.current >= self.chars.len() {
            return true;
        }

        let mut idx = self.current;
        let mut seen_line_start = idx == 0;

        if !seen_line_start && idx > 0 {
            let mut back = idx;
            while back > 0 {
                let ch = self.chars[back - 1];
                if ch == '\n' {
                    seen_line_start = true;
                    break;
                } else if Self::is_space_like_char(ch) && ch != '\n' {
                    back -= 1;
                } else {
                    break;
                }
            }
        }

        if !seen_line_start {
            return false;
        }

        while idx < self.chars.len() && Self::is_space_like_char(self.chars[idx]) {
            idx += 1;
        }

        if idx >= self.chars.len() {
            return true;
        }

        match self.chars[idx] {
            '}' => return true,
            '#' => {
                if idx + 1 < self.chars.len() && self.chars[idx + 1] == '[' {
                    return true;
                }
            }
            _ => {}
        }

        const FRAME_BOUNDARY_KEYWORDS: [&str; 14] = [
            "system",
            "module",
            "enum",
            "class",
            "fn",
            "function",
            "interface",
            "machine",
            "domain",
            "domain:",
            "actions",
            "actions:",
            "operations",
            "operations:",
        ];

        for keyword in FRAME_BOUNDARY_KEYWORDS.iter() {
            if self.matches_keyword_from(idx, keyword) {
                return true;
            }
        }

        false
    }

    fn matches_keyword_from(&self, idx: usize, keyword: &str) -> bool {
        let keyword_chars: Vec<char> = keyword.chars().collect();
        if idx + keyword_chars.len() > self.chars.len() {
            return false;
        }

        for (offset, expected) in keyword_chars.iter().enumerate() {
            if self.chars[idx + offset] != *expected {
                return false;
            }
        }

        let after = idx + keyword_chars.len();
        if after < self.chars.len() {
            let boundary = self.chars[after];
            if self.is_alpha_numeric_char(boundary) {
                return false;
            }
        }

        true
    }

    fn is_alpha_numeric_char(&self, c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
    }

    fn build_target_source_map(frame_start_line: usize, raw_content: &str) -> TargetSourceMap {
        let line_count = raw_content.lines().count();
        let offsets: Vec<usize> = if line_count == 0 {
            Vec::new()
        } else {
            (0..line_count).collect()
        };

        TargetSourceMap {
            frame_start_line,
            target_line_offsets: offsets,
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

    fn match_keyword(&mut self, keyword: &str) -> bool {
        let mut temp_index = self.current;
        for expected in keyword.chars() {
            if temp_index >= self.chars.len() {
                return false;
            }
            let actual = self.chars[temp_index];
            if actual != expected {
                return false;
            }
            temp_index += 1;
        }

        if temp_index < self.chars.len() {
            let boundary = self.chars[temp_index];
            if self.is_alpha_numeric(boundary) {
                return false;
            }
        }

        self.current = temp_index;
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

    fn peek_at(&self, offset: usize) -> char {
        let pos = self.current + offset;
        if pos >= self.chars.len() {
            return '\0';
        }
        self.chars[pos]
    }

    fn is_digit(&self, c: char) -> bool {
        ('0'..='9').contains(&c)
    }

    fn number(&mut self, mut is_integer: bool) {
        // Check for Python numeric literals (0b, 0o, 0x)
        if self.chars[self.start] == '0' && self.current - self.start == 1 {
            let next_char = self.peek();
            if next_char == 'b' || next_char == 'B' {
                // Binary literal (with optional underscores)
                self.advance(); // consume 'b' or 'B'
                while self.peek() == '0' || self.peek() == '1' || self.peek() == '_' {
                    self.advance();
                }
                let _s: String = self.chars[self.start..self.current].iter().collect();
                // For Python, we just pass the literal through as-is
                self.add_token_literal(TokenType::Number, TokenLiteral::Integer(0)); // Placeholder value
                return;
            } else if next_char == 'o' || next_char == 'O' {
                // Octal literal (with optional underscores)
                self.advance(); // consume 'o' or 'O'
                while ('0'..='7').contains(&self.peek()) || self.peek() == '_' {
                    self.advance();
                }
                let _s: String = self.chars[self.start..self.current].iter().collect();
                self.add_token_literal(TokenType::Number, TokenLiteral::Integer(0)); // Placeholder value
                return;
            } else if next_char == 'x' || next_char == 'X' {
                // Hexadecimal literal (with optional underscores)
                self.advance(); // consume 'x' or 'X'
                while self.is_digit(self.peek())
                    || ('a'..='f').contains(&self.peek())
                    || ('A'..='F').contains(&self.peek())
                    || self.peek() == '_'
                {
                    self.advance();
                }
                let _s: String = self.chars[self.start..self.current].iter().collect();
                self.add_token_literal(TokenType::Number, TokenLiteral::Integer(0)); // Placeholder value
                return;
            }
        }

        if is_integer {
            // consume whole number (with optional underscores)
            while self.is_digit(self.peek()) || self.peek() == '_' {
                self.advance();
            }

            if self.peek() == '.' {
                is_integer = false;
                // consume the '.'
                self.advance();
            }
        }

        // consume mantissa, if present (with optional underscores)
        while self.is_digit(self.peek()) || self.peek() == '_' {
            self.advance();
        }

        // Check for scientific notation (e or E)
        if self.peek() == 'e' || self.peek() == 'E' {
            is_integer = false; // Scientific notation is always treated as float
            self.advance(); // consume 'e' or 'E'

            // Handle optional sign
            if self.peek() == '+' || self.peek() == '-' {
                self.advance();
            }

            // Consume exponent digits
            while self.is_digit(self.peek()) || self.peek() == '_' {
                self.advance();
            }
        }

        // Check for complex number suffix (j or J)
        if self.peek() == 'j' || self.peek() == 'J' {
            self.advance(); // consume 'j' or 'J'
                            // Complex number - treat as special token
            let _s: String = self.chars[self.start..self.current].iter().collect();
            self.add_token_literal(TokenType::ComplexNumber, TokenLiteral::Float(0.0)); // Placeholder value
            return;
        }

        if is_integer {
            let s: String = self.chars[self.start..self.current].iter().collect();
            // Remove underscores before parsing (Python allows them for readability)
            let clean_s: String = s.chars().filter(|c| *c != '_').collect();
            let result = clean_s.parse::<i32>();
            match result {
                Ok(number) => {
                    self.add_token_literal(TokenType::Number, TokenLiteral::Integer(number));
                }
                Err(err) => {
                    self.error(self.line, &format!("Invalid number format: {}", err));
                }
            }
        } else {
            // is float
            let s: String = self.chars[self.start..self.current].iter().collect();
            // Remove underscores before parsing (Python allows them for readability)
            let clean_s: String = s.chars().filter(|c| *c != '_').collect();
            let result = clean_s.parse::<f32>();
            match result {
                Ok(number) => {
                    self.add_token_literal(TokenType::Number, TokenLiteral::Float(number));
                }
                Err(err) => {
                    self.error(self.line, &format!("Invalid decimal number: {}", err));
                }
            }
        }
    }

    fn identifier(&mut self) {
        // Check for string prefixes before consuming the identifier
        let first_char = self.chars[self.start];
        let has_quote_ahead = self.peek() == '"'
            || (self.peek() == '"' && self.peek_at(1) == '"' && self.peek_at(2) == '"');

        // Check for f-strings, raw strings, byte strings
        if (first_char == 'f'
            || first_char == 'F'
            || first_char == 'r'
            || first_char == 'R'
            || first_char == 'b'
            || first_char == 'B')
            && has_quote_ahead
        {
            // Handle string prefix
            return self.prefixed_string(first_char);
        }

        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        // See if the identifier is a reserved word.
        let text: String = self.chars[self.start..self.current].iter().collect();

        // Special handling for "system" keyword
        if text == "system" {
            // Check if this is "system.something"
            let saved_current = self.current;
            if self.peek() == '.' {
                self.advance(); // consume '.'
                                // Check if next word is an identifier
                if self.is_alpha(self.peek()) {
                    let start_of_identifier = self.current;
                    while self.is_alpha_numeric(self.peek()) {
                        self.advance();
                    }
                    let identifier: String = self.chars[start_of_identifier..self.current]
                        .iter()
                        .collect();

                    if identifier == "return" {
                        // This is "system.return" - special case
                        self.add_token(TokenType::SystemReturn);
                        return;
                    } else {
                        // This is "system.identifier" - create a SystemMethodCall token
                        // Store the method name in the token's lexeme for later parsing
                        self.add_token_literal(TokenType::SystemMethodCall, TokenLiteral::None);
                        return;
                    }
                }
                // Not "system.identifier", restore position
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

    // Python-style single-line comment with #
    fn python_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        self.add_token(TokenType::PythonComment);
    }

    fn consume_target_directive(&mut self) -> bool {
        if self.peek() != '[' {
            return false;
        }

        // Enter attribute parsing
        self.advance(); // consume '['
        let mut annot = String::new();
        while !self.is_at_end() {
            let c = self.advance();
            if c == ']' {
                break;
            }
            annot.push(c);
        }

        let trimmed = annot.trim();

        if let Some(rest) = trimmed.strip_prefix("target") {
            let lang_part = rest.trim_start_matches(|c: char| c == ':' || c.is_whitespace());
            let lang_name = lang_part.trim();
            let msg = format!(
                "Inline #[target: {}] annotations are no longer supported",
                lang_name
            );
            self.error(self.line, &msg);
        }

        // Skip rest of the line after the inline annotation
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        if !self.is_at_end() && self.peek() == '\n' {
            self.advance();
        }

        true
    }

    fn is_next_target_directive(&self) -> bool {
        if self.peek() != '[' {
            return false;
        }

        let mut idx = self.current + 1; // position after '['
        while idx < self.chars.len() && self.chars[idx].is_whitespace() {
            idx += 1;
        }

        let mut keyword = String::new();
        while idx < self.chars.len() {
            let ch = self.chars[idx];
            if ch == ':' || ch == ']' || ch.is_whitespace() {
                break;
            }
            keyword.push(ch);
            idx += 1;
        }

        keyword.eq_ignore_ascii_case("target")
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

    // Removed C-style comments - no longer supported in Frame v0.40

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

    fn block_keyword(&mut self, first_char: char) -> bool {
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
        let lex = self.chars[self.start..self.current]
            .iter()
            .collect::<String>();
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            tok_type, lex, literal, self.line, self.start, len,
        ));
    }

    fn add_string_token_literal(&mut self, tok_type: TokenType, literal: TokenLiteral) {
        let lex = self.chars[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            tok_type, lex, literal, self.line, self.start, len,
        ));
    }

    fn add_prefixed_string_token(&mut self, tok_type: TokenType) {
        // For prefixed strings, we want to preserve the entire lexeme including prefix and quotes
        let lex = self.chars[self.start..self.current]
            .iter()
            .collect::<String>();
        let len = self.current - self.start;
        self.tokens.push(Token::new(
            tok_type,
            lex,
            TokenLiteral::None,
            self.line,
            self.start,
            len,
        ));
    }

    fn error(&mut self, line: usize, error_msg: &str) {
        let error = &format!("Line {} : Error: {}\n", line, error_msg);
        self.has_errors = true;
        self.errors.push_str(error);
        self.add_token(TokenType::Error);
    }

    fn string(&mut self) {
        // Check for triple-quoted string
        if self.peek() == '"' && self.peek_at(1) == '"' {
            self.advance(); // consume second quote
            self.advance(); // consume third quote
            return self.triple_quoted_string();
        }

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
            self.error(self.line, "Missing closing quote for string");
            return;
        }

        self.advance();

        self.add_string_token_literal(TokenType::String, TokenLiteral::None);
    }

    fn string_single(&mut self) {
        // Handle triple single-quoted string '''...'''
        if self.peek() == '\'' && self.peek_at(1) == '\'' {
            self.advance(); // second '
            self.advance(); // third '
            // Scan until closing '''
            while !self.is_at_end() {
                if self.peek() == '\'' && self.peek_at(1) == '\'' && self.peek_at(2) == '\'' {
                    self.advance();
                    self.advance();
                    self.advance();
                    // Represent triple-quoted as a string token
                    self.add_prefixed_string_token(TokenType::TripleQuotedString);
                    return;
                }
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance();
            }
            // Unterminated triple single-quoted string
            self.error(self.line, "Missing closing triple quotes (''')");
            return;
        }

        // Regular single-quoted string with escapes
        while !self.is_at_end() {
            let c = self.peek();
            if c == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
            } else if c == '\n' {
                // allow newline; line count handled
            } else if c == '\'' {
                break;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Missing closing quote for string");
            return;
        }

        self.advance(); // consume closing '
        // Represent as generic string token
        self.add_string_token_literal(TokenType::String, TokenLiteral::None);
    }

    fn prefixed_string(&mut self, prefix: char) {
        // Already consumed the prefix letter, now consume the quote(s)
        if self.peek() == '"' {
            self.advance(); // consume first quote

            // Check for triple-quoted string
            if self.peek() == '"' && self.peek_at(1) == '"' {
                self.advance(); // consume second quote
                self.advance(); // consume third quote

                // Handle triple-quoted prefixed string
                self.scan_triple_quoted_content();

                match prefix {
                    'f' | 'F' => self.add_prefixed_string_token(TokenType::FString),
                    'r' | 'R' => self.add_prefixed_string_token(TokenType::RawString),
                    'b' | 'B' => self.add_prefixed_string_token(TokenType::ByteString),
                    _ => unreachable!(),
                }
            } else {
                // Regular prefixed string
                self.scan_regular_string_content(prefix == 'r' || prefix == 'R');

                match prefix {
                    'f' | 'F' => self.add_prefixed_string_token(TokenType::FString),
                    'r' | 'R' => self.add_prefixed_string_token(TokenType::RawString),
                    'b' | 'B' => self.add_prefixed_string_token(TokenType::ByteString),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn triple_quoted_string(&mut self) {
        self.scan_triple_quoted_content();
        self.add_prefixed_string_token(TokenType::TripleQuotedString);
    }

    fn scan_triple_quoted_content(&mut self) {
        // Scan until we find three quotes in a row
        while !self.is_at_end() {
            if self.peek() == '"' && self.peek_at(1) == '"' && self.peek_at(2) == '"' {
                self.advance(); // consume first closing quote
                self.advance(); // consume second closing quote
                self.advance(); // consume third closing quote
                return;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // Unterminated triple-quoted string
        self.error(self.line, "Missing closing triple quotes (\"\"\")");
    }

    fn scan_regular_string_content(&mut self, is_raw: bool) {
        while !self.is_at_end() {
            let c = self.peek();
            if !is_raw && c == '\\' {
                self.advance();
                if self.is_at_end() {
                    break;
                }
            } else if c == '\n' {
                // Allow newlines in regular strings for now
            } else if c == '"' {
                break;
            }
            self.advance();
        }

        // Unterminated string
        if self.is_at_end() {
            self.error(self.line, "Missing closing quote for string");
            return;
        }

        self.advance(); // consume closing quote
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
    Ampersand, // &
    Pipe,      // |
    // REMOVED: Caret (^) - use 'return' keyword
    // REMOVED: ReturnAssign (^=) - use 'return = value'
    LogicalAnd,                   // &&
    System,                       // 'system' keyword for modern syntax (reserved)
    SystemReturn,                 // 'system.return' for setting interface return value
    SystemMethodCall,             // 'system.methodName' for calling interface methods
    Self_,                        // self
    Return_,                      // return
    EnterStateMsg,                // $>
    ExitStateMsg,                 // <$
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
    String,             // "foo"
    FString,            // f"foo {bar}" - formatted string literal (v0.40)
    RawString,          // r"foo\bar" - raw string literal (v0.40)
    ByteString,         // b"foo" - byte string literal (v0.40)
    TripleQuotedString, // """foo""" - multi-line string (v0.40)
    // REMOVED: ThreeTicks (```) - not used
    Number,        // 1, 1.01
    ComplexNumber, // 3+4j, 2.5j - complex numbers (v0.56)
    Var,           // var keyword
    Const,         // const keyword
    //    New,              // new keyword
    Loop,             // loop keyword
    For,              // for keyword
    While,            // while keyword
    If,               // if keyword
    Elif,             // elif keyword
    Else,             // else keyword
    Continue,         // continue keyword
    Break,            // break keyword
    Del,              // 'del' keyword (v0.50)
    In,               // 'in' keyword
    Enum,             // 'enum' keyword
    Function,         // 'fn' keyword
    Import,           // 'import' keyword
    From,             // 'from' keyword
    As,               // 'as' keyword
    Module,           // 'module' keyword
    Native,           // 'native' keyword
    Async,            // 'async' keyword
    Await,            // 'await' keyword
    Lambda,           // 'lambda' keyword
    Try,              // 'try' keyword
    Except,           // 'except' keyword
    Finally,          // 'finally' keyword
    Raise,            // 'raise' keyword
    With,             // 'with' keyword
    And,              // 'and' keyword (Python logical AND)
    Or,               // 'or' keyword (Python logical OR)
    Not,              // 'not' keyword (Python logical NOT)
    Yield,            // 'yield' keyword (v0.42)
    YieldFrom,        // 'yield from' (handled as two tokens) (v0.42)
    Match,            // 'match' keyword (v0.44)
    Case,             // 'case' keyword (v0.44)
    PythonComment,    // # Python-style comment
    MultiLineComment, // {-- Frame documentation comments --}
    FloorDivide,      // // floor division operator
    OpenBrace,        // {
    CloseBrace,       // }
    True,             // true
    False,            // false
    None_,            // None (standard null value)
    Colon,            // :
    Semicolon,        // ;
    Comma,            // ,
    Dispatch,         // =>
    TargetAnnotation, // @target declaration
    Equals,           // =
    Walrus,           // := (assignment expression operator)
    ForwardSlash,     // /
    MatchString,      // '/<any characters>/' - contains <string>
    MatchEmptyString, // '~//'
    MatchNull,        // '!//'
    // REMOVED: Pattern matching tokens
    // REMOVED: StringMatchStart ('~/') - string patterns removed
    // REMOVED: NumberMatchStart ('#/') - number patterns removed
    // REMOVED: EnumMatchStart (':/') - enum patterns removed
    // REMOVED: ColonBar (:|) - test terminator removed
    StateStackOperationPush, // $$[+]
    StateStackOperationPop,  // $$[-]
    ParentState,             // $^ - parent state reference
    Dot,                     // .
    ColonColon,              // :: - module separator for qualified names
    At,                      // @ - matrix multiplication (v0.40)
    AtEqual,                 // @= - matrix multiplication compound assignment (v0.40)
    AtAt,                    // @@ - reserved for future use
    DollarAt,                // $@ - current event reference
    PipePipe,                // ||
    PipePipeDot,             // ||.
    PipePipeLBracket,        // ||[
    // REMOVED: Hash (#) - old system syntax removed

    // Compound assignment operators (v0.39)
    PlusEqual,        // +=
    DashEqual,        // -=
    StarEqual,        // *=
    SlashEqual,       // /=
    FloorDivideEqual, // //=
    PercentEqual,     // %=
    StarStarEqual,    // **=
    AmpersandEqual,   // &=
    PipeEqual,        // |=
    LeftShiftEqual,   // <<=
    RightShiftEqual,  // >>=
    CaretEqual,       // ^= (bitwise XOR assignment, v0.40)

    // Bitwise operators (v0.39/v0.40)
    Tilde,      // ~ (bitwise NOT)
    LeftShift,  // <<
    RightShift, // >>
    Caret,      // ^ (bitwise XOR, v0.40)

    // Identity operators (v0.39)
    Is,          // 'is' keyword
    IsNot,       // 'is not' (handled as two tokens)
    Class,       // 'class' keyword
    Assert,      // 'assert' keyword
    Super,       // 'super' keyword for parent class access
    Cls,         // 'cls' keyword for class methods
    Property,    // 'property' keyword/decorator
    ClassMethod, // 'classmethod' keyword/decorator
    Setter,      // 'setter' keyword for property setters
    Deleter,     // 'deleter' keyword for property deleters
    Type,        // 'type' keyword for type aliases (Python 3.12+)

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanningMode {
    TargetDiscovery,
    FrameCommon,
    TargetSpecific(TargetLanguage),
}

#[derive(Debug, Clone)]
pub struct TargetRegion {
    pub start_position: usize,
    pub end_position: Option<usize>,
    pub raw_content: String,
    pub target: TargetLanguage,
    pub source_map: TargetSourceMap,
}

// Transitional alias: prefer `NativeRegion` in new code to reflect that this
// represents a native-language region captured during scanning. This will be
// renamed in a future cleanup once all call sites migrate.
pub type NativeRegion = TargetRegion;

#[derive(Debug, Clone, Default)]
pub struct TargetSourceMap {
    pub frame_start_line: usize,
    pub target_line_offsets: Vec<usize>,
}

#[derive(Debug, Clone)]
struct ActiveTargetRegion {
    start_position: usize,
    start_line: usize,
    target: TargetLanguage,
}

#[derive(Clone, Copy)]
struct StringState {
    delimiter: char,
    triple: bool,
    escape: bool,
}

#[derive(Clone, Copy)]
enum BlockCommentKind {
    SlashStar,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn scan_target_regions(source: &str) -> Vec<TargetRegion> {
        let scanner = Scanner::new(source.to_string());
        let (_has_errors, _errors, _tokens, regions) = scanner.scan_tokens();
        regions
    }

    #[test]
    fn at_target_emits_target_annotation_token() {
        let source = "@target python_3\nsystem Example {}\n";
        let scanner = Scanner::new(source.to_string());
        let (has_errors, errors, tokens, _regions) = scanner.scan_tokens();
        assert!(!has_errors, "scanner reported errors: {}", errors);
        assert!(
            tokens.len() >= 2,
            "expected annotation tokens, got {:?}",
            tokens
        );
        assert_eq!(tokens[0].token_type, TokenType::TargetAnnotation);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "python_3");
    }

    #[test]
    fn typescript_inline_target_directive_is_rejected() {
        let source = r#"
system Example {
    actions:
        ts_action() {
            #[target: typescript]
            if (true) {
                /* block comment */
                doSomething();
            }
        }
}
"#;

        let scanner = Scanner::new(source.to_string());
        let (has_errors, errors, _tokens, regions) = scanner.scan_tokens();
        assert!(
            has_errors,
            "expected scanner error for inline target directive"
        );
        assert!(
            errors.contains("Inline #[target: typescript] annotations are no longer supported"),
            "unexpected error text: {}",
            errors
        );
        assert_eq!(regions.len(), 0, "no target regions should be recorded");
    }

    #[test]
    fn python_inline_target_directive_is_rejected() {
        let source = r#"
fn helper() {
    #[target: python]
    text = '''multi-line'''
    print(text)
}
"#;

        let scanner = Scanner::new(source.to_string());
        let (has_errors, errors, _tokens, regions) = scanner.scan_tokens();
        assert!(
            has_errors,
            "expected scanner error for inline target directive"
        );
        assert!(
            errors.contains("Inline #[target: python] annotations are no longer supported"),
            "unexpected error text: {}",
            errors
        );
        assert_eq!(regions.len(), 0, "no target regions should be recorded");
    }

    #[test]
    fn unicode_nbsp_is_treated_as_whitespace() {
        let nbsp = '\u{00A0}';
        let mut source = String::new();
        source.push_str("@target python\n");
        source.push(nbsp);
        source.push(nbsp);
        source.push_str("system S {}\n");
        let scanner = Scanner::new(source);
        let (has_errors, errors, tokens, _regions) = scanner.scan_tokens();
        assert!(!has_errors, "scanner reported errors: {}", errors);
        assert!(tokens.iter().any(|t| t.token_type == TokenType::System));
    }

    #[test]
    fn crlf_line_endings_are_supported() {
        let source = "@target python\r\nsystem S {\r\n}\r\n".to_string();
        let scanner = Scanner::new(source);
        let (has_errors, errors, _tokens, _regions) = scanner.scan_tokens();
        assert!(!has_errors, "scanner reported errors: {}", errors);
    }

    #[test]
    fn utf8_bom_is_ignored() {
        let source = "\u{FEFF}@target python\nsystem S {}\n".to_string();
        let scanner = Scanner::new(source);
        let (has_errors, errors, tokens, _regions) = scanner.scan_tokens();
        assert!(!has_errors, "scanner reported errors: {}", errors);
        assert!(
            tokens.iter().any(|t| t.token_type == TokenType::TargetAnnotation),
            "expected TargetAnnotation token"
        );
    }

    #[test]
    fn cr_only_line_endings_supported() {
        // Old Mac-style CR-only line endings should not cause scanner errors
        let source = "@target python\rsystem S {\r}\r".to_string();
        let scanner = Scanner::new(source);
        let (has_errors, errors, _tokens, _regions) = scanner.scan_tokens();
        assert!(!has_errors, "scanner reported errors: {}", errors);
    }
}

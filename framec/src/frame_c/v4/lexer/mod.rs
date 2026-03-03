//! Frame Lexer (Stage 1 of the V4 Pipeline)
//!
//! Converts Frame system body bytes into a typed token stream.
//! Operates in two modes:
//! - **Structural**: Tokenizes Frame keywords, identifiers, operators, delimiters
//! - **NativeAware**: Detects Frame constructs within native code, passes the rest through
//!
//! The Parser (Stage 2) controls mode switching by calling `enter_native_mode()` and
//! `enter_structural_mode()`.

use std::collections::VecDeque;
use crate::frame_c::v4::native_region_scanner::unified::SyntaxSkipper;
use crate::frame_c::v4::native_region_scanner::python::PythonSkipper;
use crate::frame_c::v4::native_region_scanner::typescript::TypeScriptSkipper;
use crate::frame_c::v4::native_region_scanner::rust::RustSkipper;
use crate::frame_c::v4::native_region_scanner::c::CSkipper;
use crate::frame_c::v4::native_region_scanner::cpp::CppSkipper;
use crate::frame_c::v4::native_region_scanner::java::JavaSkipper;
use crate::frame_c::v4::native_region_scanner::csharp::CSharpSkipper;
use crate::frame_c::v4::frame_ast::Span;
use crate::frame_c::visitors::TargetLanguage;

// ============================================================================
// Token Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ===== Frame Structural Keywords =====
    Interface,           // "interface"
    Machine,             // "machine"
    Actions,             // "actions"
    Operations,          // "operations"
    Domain,              // "domain"
    Var,                 // "var"

    // ===== Frame Statements =====
    Return,              // "return" (Frame return sugar)

    // ===== State Syntax =====
    /// "$StateName" — state reference in transitions (-> $Foo)
    StateRef(String),
    /// "$>" — enter event handler
    EnterHandler,
    /// "<$" — exit event handler
    ExitHandler,
    /// "$.varName" — state variable reference (read/write)
    StateVarRef(String),
    /// "$^" — parent state reference (HSM forward: => $^)
    ParentRef,

    // ===== Transition & Control =====
    Arrow,               // "->"
    FatArrow,            // "=>"
    PushState,           // "push$"
    PopState,            // "pop$"

    // ===== Context Syntax =====
    ContextParam(String),    // "@@.paramName"
    ContextReturn,           // "@@:return"
    ContextEvent,            // "@@:event"
    ContextData(String),     // "@@:data[key]"
    ContextParams(String),   // "@@:params[key]"

    // ===== Delimiters =====
    LBrace,              // "{"
    RBrace,              // "}"
    LParen,              // "("
    RParen,              // ")"
    LBracket,            // "["
    RBracket,            // "]"
    Comma,               // ","
    Colon,               // ":" — param/type separator, return type
    SectionColon,        // ":" after section keyword (interface:, machine:, etc.)
    Equals,              // "="
    Dot,                 // "."
    Semicolon,           // ";"
    Star,                // "*" — for C pointer types (char*, int**)
    Ampersand,           // "&" — for Rust reference types (&str, &mut)

    // ===== Identifiers & Literals =====
    Ident(String),       // alphanumeric identifier
    IntLit(i64),         // integer literal
    FloatLit(f64),       // float literal
    StringLit(String),   // string literal
    BoolLit(bool),       // true/false

    // ===== Native Code (only in native-aware mode) =====
    NativeCode(String),  // opaque native code chunk

    // ===== Meta =====
    Newline,             // significant newline (if needed for grammar)
    Eof,                 // end of token stream
}

/// A token with its source span.
#[derive(Debug, Clone, PartialEq)]
pub struct Spanned {
    pub token: Token,
    pub span: Span,
}

/// Lexer errors.
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    /// Unexpected byte in Frame structural context
    UnexpectedByte { byte: u8, span: Span },
    /// Unterminated string literal
    UnterminatedString { span: Span },
    /// Unterminated comment
    UnterminatedComment { span: Span },
    /// Invalid Frame construct syntax
    InvalidFrameConstruct { text: String, span: Span },
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedByte { byte, span } =>
                write!(f, "Unexpected byte '{}' (0x{:02x}) at position {}", *byte as char, byte, span.start),
            LexError::UnterminatedString { span } =>
                write!(f, "Unterminated string literal at position {}", span.start),
            LexError::UnterminatedComment { span } =>
                write!(f, "Unterminated comment at position {}", span.start),
            LexError::InvalidFrameConstruct { text, span } =>
                write!(f, "Invalid Frame construct '{}' at position {}", text, span.start),
        }
    }
}

impl std::error::Error for LexError {}

/// Lexer mode: structural (Frame syntax) or native-aware (handler bodies).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LexerMode {
    /// Tokenize Frame syntax: keywords, identifiers, operators, delimiters
    Structural,
    /// Detect Frame constructs in native code, pass everything else through
    NativeAware,
}

// ============================================================================
// Lexer
// ============================================================================

pub struct Lexer<'a> {
    source: &'a [u8],
    cursor: usize,
    end: usize,           // end of system body
    native_end: usize,    // end of current native block (NativeAware mode only)
    mode: LexerMode,
    #[allow(dead_code)]
    lang: TargetLanguage,
    skipper: Box<dyn SyntaxSkipper>,
    pending: VecDeque<Spanned>,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer for a system body.
    ///
    /// `source` is the full source bytes.
    /// `body_span` is the span of the system body (inside braces).
    /// `lang` determines language-specific string/comment awareness.
    pub fn new(source: &'a [u8], body_span: Span, lang: TargetLanguage) -> Self {
        let skipper = create_skipper(lang);
        Lexer {
            source,
            cursor: body_span.start,
            end: body_span.end,
            native_end: 0,
            mode: LexerMode::Structural,
            lang,
            skipper,
            pending: VecDeque::new(),
        }
    }

    /// Get the next token.
    pub fn next_token(&mut self) -> Result<Spanned, LexError> {
        if let Some(tok) = self.pending.pop_front() {
            return Ok(tok);
        }
        self.advance()?;
        Ok(self.pending.pop_front().unwrap_or(Spanned {
            token: Token::Eof,
            span: Span::new(self.cursor, self.cursor),
        }))
    }

    /// Peek at the next token type without consuming it.
    pub fn peek(&mut self) -> Result<&Token, LexError> {
        if self.pending.is_empty() {
            self.advance()?;
            if self.pending.is_empty() {
                self.pending.push_back(Spanned {
                    token: Token::Eof,
                    span: Span::new(self.cursor, self.cursor),
                });
            }
        }
        Ok(&self.pending.front().unwrap().token)
    }

    /// Peek at the next Spanned token without consuming it.
    pub fn peek_spanned(&mut self) -> Result<&Spanned, LexError> {
        if self.pending.is_empty() {
            self.advance()?;
            if self.pending.is_empty() {
                self.pending.push_back(Spanned {
                    token: Token::Eof,
                    span: Span::new(self.cursor, self.cursor),
                });
            }
        }
        Ok(self.pending.front().unwrap())
    }

    /// Switch to native-aware mode.
    /// `body_end` is the byte position of the closing `}` (exclusive boundary).
    /// Called by the Parser when entering a handler/action/operation body.
    pub fn enter_native_mode(&mut self, body_end: usize) {
        self.mode = LexerMode::NativeAware;
        self.native_end = body_end;
        self.pending.clear();
    }

    /// Switch back to structural mode.
    /// Called by the Parser when exiting a handler/action/operation body.
    pub fn enter_structural_mode(&mut self) {
        self.mode = LexerMode::Structural;
        self.pending.clear();
    }

    /// Current cursor position in the source.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Set cursor position (used by parser to skip past body close brace).
    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos;
        self.pending.clear();
    }

    /// Current lexer mode.
    pub fn mode(&self) -> LexerMode {
        self.mode
    }

    /// Access the skipper (useful for parser to call body_closer).
    pub fn skipper(&self) -> &dyn SyntaxSkipper {
        &*self.skipper
    }

    /// Find the matching close brace for an open brace at `open_pos`.
    /// Uses the language-specific BodyCloser to handle strings/comments.
    pub fn find_close_brace(&self, open_pos: usize) -> Option<usize> {
        let mut closer = self.skipper.body_closer();
        closer.close_byte(self.source, open_pos).ok()
    }

    /// Access the source bytes (for parser to pass to BodyCloser if needed).
    pub fn source(&self) -> &[u8] {
        self.source
    }

    // ========================================================================
    // Internal: dispatch to mode-specific advance
    // ========================================================================

    fn advance(&mut self) -> Result<(), LexError> {
        match self.mode {
            LexerMode::Structural => self.advance_structural(),
            LexerMode::NativeAware => self.advance_native(),
        }
    }

    // ========================================================================
    // Structural Mode
    // ========================================================================

    fn advance_structural(&mut self) -> Result<(), LexError> {
        self.skip_whitespace_and_comments();

        if self.cursor >= self.end {
            self.emit(Token::Eof, self.cursor, self.cursor);
            return Ok(());
        }

        let start = self.cursor;
        let b = self.source[self.cursor];

        match b {
            // Exit handler: <$
            b'<' if self.peek_byte(1) == Some(b'$') => {
                self.cursor += 2;
                self.emit(Token::ExitHandler, start, self.cursor);
            }

            // State syntax: $
            b'$' => self.lex_dollar(start)?,

            // Transition: ->
            b'-' if self.peek_byte(1) == Some(b'>') => {
                self.cursor += 2;
                self.emit(Token::Arrow, start, self.cursor);
            }

            // Forward: =>
            b'=' if self.peek_byte(1) == Some(b'>') => {
                self.cursor += 2;
                self.emit(Token::FatArrow, start, self.cursor);
            }

            // Equals
            b'=' => {
                self.cursor += 1;
                self.emit(Token::Equals, start, self.cursor);
            }

            // Delimiters
            b'{' => { self.cursor += 1; self.emit(Token::LBrace, start, self.cursor); }
            b'}' => { self.cursor += 1; self.emit(Token::RBrace, start, self.cursor); }
            b'(' => { self.cursor += 1; self.emit(Token::LParen, start, self.cursor); }
            b')' => { self.cursor += 1; self.emit(Token::RParen, start, self.cursor); }
            b'[' => { self.cursor += 1; self.emit(Token::LBracket, start, self.cursor); }
            b']' => { self.cursor += 1; self.emit(Token::RBracket, start, self.cursor); }
            b',' => { self.cursor += 1; self.emit(Token::Comma, start, self.cursor); }
            b':' => { self.cursor += 1; self.emit(Token::Colon, start, self.cursor); }
            b'.' => { self.cursor += 1; self.emit(Token::Dot, start, self.cursor); }
            b';' => { self.cursor += 1; self.emit(Token::Semicolon, start, self.cursor); }
            b'*' => { self.cursor += 1; self.emit(Token::Star, start, self.cursor); }
            b'&' => { self.cursor += 1; self.emit(Token::Ampersand, start, self.cursor); }

            // Negative number
            b'-' if self.peek_byte(1).map_or(false, |c| c.is_ascii_digit()) => {
                self.lex_number(start)?;
            }

            // Numbers
            b'0'..=b'9' => self.lex_number(start)?,

            // Strings
            b'"' | b'\'' => self.lex_string(start)?,

            // Identifiers and keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.lex_identifier_or_keyword(start)?,

            _ => {
                self.cursor += 1;
                return Err(LexError::UnexpectedByte {
                    byte: b,
                    span: Span::new(start, self.cursor),
                });
            }
        }

        Ok(())
    }

    // ========================================================================
    // Native-Aware Mode
    // ========================================================================

    fn advance_native(&mut self) -> Result<(), LexError> {
        let end = self.native_end;

        if self.cursor >= end {
            self.emit(Token::Eof, self.cursor, self.cursor);
            return Ok(());
        }

        let native_start = self.cursor;
        let mut at_sol = self.cursor == 0
            || (self.cursor > 0 && self.source[self.cursor - 1] == b'\n');
        let mut indent = 0usize;

        while self.cursor < end {
            let b = self.source[self.cursor];

            // Track SOL and indent
            if at_sol {
                if b == b' ' || b == b'\t' {
                    indent += 1;
                    self.cursor += 1;
                    continue;
                }

                // Save start of this line (before indent) BEFORE SOL detection advances cursor
                let frame_line_start = self.cursor.saturating_sub(indent);

                // Try SOL Frame statement detection
                if let Some(frame_tokens) = self.try_sol_frame_statement(end)? {
                    // Emit preceding native code (everything before this Frame line)
                    if native_start < frame_line_start {
                        let text = String::from_utf8_lossy(
                            &self.source[native_start..frame_line_start]
                        ).to_string();
                        if !text.is_empty() {
                            self.emit(Token::NativeCode(text), native_start, frame_line_start);
                        }
                    }

                    // Emit the Frame tokens
                    for tok in frame_tokens {
                        self.pending.push_back(tok);
                    }
                    return Ok(());
                }

                at_sol = false;
                indent = 0;
            }

            // Try to skip strings and comments (language-specific)
            if let Some(new_pos) = self.skipper.skip_comment(self.source, self.cursor, end) {
                self.cursor = new_pos;
                continue;
            }

            if let Some(new_pos) = self.skipper.skip_string(self.source, self.cursor, end) {
                self.cursor = new_pos;
                continue;
            }

            // Mid-line Frame constructs
            match b {
                b'\n' => {
                    self.cursor += 1;
                    at_sol = true;
                    indent = 0;
                    continue;
                }

                // State variable: $.varName
                b'$' if self.cursor + 1 < end && self.source[self.cursor + 1] == b'.' => {
                    // Emit preceding native code
                    if native_start < self.cursor {
                        let text = String::from_utf8_lossy(
                            &self.source[native_start..self.cursor]
                        ).to_string();
                        self.emit(Token::NativeCode(text), native_start, self.cursor);
                    }
                    let var_start = self.cursor;
                    self.cursor += 2; // Skip "$."
                    let name = self.scan_identifier();
                    self.emit(Token::StateVarRef(name), var_start, self.cursor);
                    return Ok(());
                }

                // Context syntax: @@
                b'@' if self.cursor + 1 < end && self.source[self.cursor + 1] == b'@' => {
                    // Emit preceding native code
                    if native_start < self.cursor {
                        let text = String::from_utf8_lossy(
                            &self.source[native_start..self.cursor]
                        ).to_string();
                        self.emit(Token::NativeCode(text), native_start, self.cursor);
                    }
                    self.lex_context_construct(end)?;
                    return Ok(());
                }

                _ => {
                    self.cursor += 1;
                }
            }
        }

        // Emit remaining native code
        if native_start < self.cursor {
            let text = String::from_utf8_lossy(
                &self.source[native_start..self.cursor]
            ).to_string();
            if !text.is_empty() {
                self.emit(Token::NativeCode(text), native_start, self.cursor);
            }
        }

        Ok(())
    }

    // ========================================================================
    // SOL Frame Statement Detection (Native-Aware Mode)
    // ========================================================================

    fn try_sol_frame_statement(
        &mut self,
        end: usize,
    ) -> Result<Option<Vec<Spanned>>, LexError> {
        let pos = self.cursor;

        if pos >= end {
            return Ok(None);
        }

        let b = self.source[pos];

        // Check for backtick prefix (V4 embedded Frame statement syntax)
        let mut check_pos = pos;
        if b == b'`' {
            check_pos += 1;
            while check_pos < end
                && (self.source[check_pos] == b' ' || self.source[check_pos] == b'\t')
            {
                check_pos += 1;
            }
            if check_pos >= end {
                return Ok(None);
            }
        }

        let cb = self.source[check_pos];

        // ---- Transition: -> ----
        if cb == b'-' && check_pos + 1 < end && self.source[check_pos + 1] == b'>' {
            return self.lex_sol_transition(check_pos, end);
        }

        // ---- Forward: => ----
        if cb == b'=' && check_pos + 1 < end && self.source[check_pos + 1] == b'>' {
            return self.lex_sol_forward(check_pos, end);
        }

        // ---- Transition with exit args: (exit_args) -> ... ----
        if cb == b'(' {
            if let Some(result) = self.try_lex_exit_args_transition(check_pos, end)? {
                return Ok(Some(result));
            }
        }

        // ---- push$ ----
        if cb == b'p' && check_pos + 4 < end
            && self.source[check_pos + 1] == b'u'
            && self.source[check_pos + 2] == b's'
            && self.source[check_pos + 3] == b'h'
            && self.source[check_pos + 4] == b'$'
        {
            self.cursor = check_pos + 5;
            let tokens = vec![Spanned {
                token: Token::PushState,
                span: Span::new(check_pos, self.cursor),
            }];
            self.skip_to_newline(end);
            return Ok(Some(tokens));
        }

        // ---- pop$ (standalone) ----
        if cb == b'p' && check_pos + 3 < end
            && self.source[check_pos + 1] == b'o'
            && self.source[check_pos + 2] == b'p'
            && self.source[check_pos + 3] == b'$'
        {
            self.cursor = check_pos + 4;
            let tokens = vec![Spanned {
                token: Token::PopState,
                span: Span::new(check_pos, self.cursor),
            }];
            self.skip_to_newline(end);
            return Ok(Some(tokens));
        }

        // ---- return <expr> (Frame return sugar) ----
        if cb == b'r' && check_pos + 6 <= end
            && &self.source[check_pos..check_pos + 6] == b"return"
        {
            let after_return = check_pos + 6;
            if after_return < end
                && (self.source[after_return] == b' ' || self.source[after_return] == b'\t')
            {
                self.cursor = after_return;
                let mut tokens = vec![Spanned {
                    token: Token::Return,
                    span: Span::new(check_pos, self.cursor),
                }];
                // Capture the expression after return as native code
                let line_end = self.skipper.find_line_end(self.source, self.cursor, end);
                if self.cursor < line_end {
                    let expr = String::from_utf8_lossy(
                        &self.source[self.cursor..line_end]
                    ).to_string();
                    tokens.push(Spanned {
                        token: Token::NativeCode(expr),
                        span: Span::new(self.cursor, line_end),
                    });
                }
                self.cursor = line_end;
                if self.cursor < end && self.source[self.cursor] == b'\n' {
                    self.cursor += 1;
                }
                return Ok(Some(tokens));
            }
        }

        Ok(None)
    }

    /// Lex a transition statement at SOL: -> $State, -> (args) $State, -> pop$, -> => $State
    fn lex_sol_transition(
        &mut self,
        arrow_pos: usize,
        end: usize,
    ) -> Result<Option<Vec<Spanned>>, LexError> {
        self.cursor = arrow_pos + 2; // Skip ->
        let mut tokens = vec![Spanned {
            token: Token::Arrow,
            span: Span::new(arrow_pos, self.cursor),
        }];

        self.skip_inline_whitespace();

        // Check for -> => $State (transition forward)
        if self.cursor + 1 < end
            && self.source[self.cursor] == b'='
            && self.source[self.cursor + 1] == b'>'
        {
            let fa_start = self.cursor;
            self.cursor += 2;
            tokens.push(Spanned {
                token: Token::FatArrow,
                span: Span::new(fa_start, self.cursor),
            });
            self.skip_inline_whitespace();
        }

        // Check for enter args: (args)
        if self.cursor < end && self.source[self.cursor] == b'(' {
            if let Some(paren_end) = self.skipper.balanced_paren_end(
                self.source, self.cursor, end
            ) {
                let args_text = String::from_utf8_lossy(
                    &self.source[self.cursor..paren_end]
                ).to_string();
                tokens.push(Spanned {
                    token: Token::NativeCode(args_text),
                    span: Span::new(self.cursor, paren_end),
                });
                self.cursor = paren_end;
                self.skip_inline_whitespace();
            }
        }

        // Check for pop$ after ->
        if self.cursor + 3 < end
            && self.source[self.cursor] == b'p'
            && self.source[self.cursor + 1] == b'o'
            && self.source[self.cursor + 2] == b'p'
            && self.source[self.cursor + 3] == b'$'
        {
            let pop_start = self.cursor;
            self.cursor += 4;
            tokens.push(Spanned {
                token: Token::PopState,
                span: Span::new(pop_start, self.cursor),
            });
        }
        // State ref: $StateName
        else if self.cursor < end && self.source[self.cursor] == b'$' {
            let sr_start = self.cursor;
            self.cursor += 1; // Skip $
            if self.cursor < end && self.source[self.cursor] == b'^' {
                // $^ parent ref
                self.cursor += 1;
                tokens.push(Spanned {
                    token: Token::ParentRef,
                    span: Span::new(sr_start, self.cursor),
                });
            } else {
                let name = self.scan_identifier();
                tokens.push(Spanned {
                    token: Token::StateRef(name),
                    span: Span::new(sr_start, self.cursor),
                });
            }

            // Check for state args: $State(args) — skip empty parens $State()
            // Emit args BEFORE the StateRef to match parser's expected pattern:
            // Arrow → NativeCode(args) → StateRef
            if self.cursor < end && self.source[self.cursor] == b'(' {
                let paren_start = self.cursor;
                if let Some(paren_end) = self.skipper.balanced_paren_end(
                    self.source, self.cursor, end
                ) {
                    let args_text = String::from_utf8_lossy(
                        &self.source[paren_start..paren_end]
                    ).to_string();
                    // Only emit args token if there's actual content (not just "()")
                    let inner = args_text.trim_start_matches('(').trim_end_matches(')').trim();
                    if !inner.is_empty() {
                        // Insert args BEFORE the StateRef token we just pushed
                        let state_ref = tokens.pop().unwrap();
                        tokens.push(Spanned {
                            token: Token::NativeCode(args_text),
                            span: Span::new(paren_start, paren_end),
                        });
                        tokens.push(state_ref);
                    }
                    self.cursor = paren_end;
                }
            }
        }

        // Skip rest of line — comments/semicolons after Frame statements are noise
        self.skip_to_newline(end);
        Ok(Some(tokens))
    }

    /// Lex a forward statement at SOL: => $State or => $^
    fn lex_sol_forward(
        &mut self,
        fa_pos: usize,
        end: usize,
    ) -> Result<Option<Vec<Spanned>>, LexError> {
        self.cursor = fa_pos + 2; // Skip =>
        let mut tokens = vec![Spanned {
            token: Token::FatArrow,
            span: Span::new(fa_pos, self.cursor),
        }];

        self.skip_inline_whitespace();

        if self.cursor < end && self.source[self.cursor] == b'$' {
            let sr_start = self.cursor;
            self.cursor += 1;
            if self.cursor < end && self.source[self.cursor] == b'^' {
                self.cursor += 1;
                tokens.push(Spanned {
                    token: Token::ParentRef,
                    span: Span::new(sr_start, self.cursor),
                });
            } else {
                let name = self.scan_identifier();
                tokens.push(Spanned {
                    token: Token::StateRef(name),
                    span: Span::new(sr_start, self.cursor),
                });
            }
        }

        // Skip rest of line — comments/semicolons after Frame statements are noise
        self.skip_to_newline(end);
        Ok(Some(tokens))
    }

    /// Try to lex transition with exit args: (exit_args) -> (enter_args) $State
    fn try_lex_exit_args_transition(
        &mut self,
        paren_pos: usize,
        end: usize,
    ) -> Result<Option<Vec<Spanned>>, LexError> {
        if let Some(paren_end) = self.skipper.balanced_paren_end(
            self.source, paren_pos, end
        ) {
            let mut k = paren_end;
            while k < end && (self.source[k] == b' ' || self.source[k] == b'\t') {
                k += 1;
            }
            if k + 1 < end && self.source[k] == b'-' && self.source[k + 1] == b'>' {
                // This is (exit_args) -> ...
                let exit_args = String::from_utf8_lossy(
                    &self.source[paren_pos..paren_end]
                ).to_string();
                let mut tokens = vec![Spanned {
                    token: Token::NativeCode(exit_args),
                    span: Span::new(paren_pos, paren_end),
                }];

                let arrow_start = k;
                self.cursor = k + 2;
                tokens.push(Spanned {
                    token: Token::Arrow,
                    span: Span::new(arrow_start, self.cursor),
                });

                self.skip_inline_whitespace();

                // Optional enter args
                if self.cursor < end && self.source[self.cursor] == b'(' {
                    if let Some(pe2) = self.skipper.balanced_paren_end(
                        self.source, self.cursor, end
                    ) {
                        let enter_args = String::from_utf8_lossy(
                            &self.source[self.cursor..pe2]
                        ).to_string();
                        tokens.push(Spanned {
                            token: Token::NativeCode(enter_args),
                            span: Span::new(self.cursor, pe2),
                        });
                        self.cursor = pe2;
                        self.skip_inline_whitespace();
                    }
                }

                // State ref
                if self.cursor < end && self.source[self.cursor] == b'$' {
                    let sr_start = self.cursor;
                    self.cursor += 1;
                    let name = self.scan_identifier();
                    tokens.push(Spanned {
                        token: Token::StateRef(name),
                        span: Span::new(sr_start, self.cursor),
                    });
                }

                self.skip_to_newline(end);
                return Ok(Some(tokens));
            }
        }
        Ok(None)
    }

    // ========================================================================
    // Context Construct Lexing (Native-Aware Mode)
    // ========================================================================

    fn lex_context_construct(&mut self, end: usize) -> Result<(), LexError> {
        let start = self.cursor;
        self.cursor += 2; // Skip "@@"

        if self.cursor < end && self.source[self.cursor] == b'.' {
            // @@.param
            self.cursor += 1; // Skip "."
            let name = self.scan_identifier();
            self.emit(Token::ContextParam(name), start, self.cursor);
        } else if self.cursor < end && self.source[self.cursor] == b':' {
            self.cursor += 1; // Skip ":"
            if self.cursor + 5 < end
                && &self.source[self.cursor..self.cursor + 6] == b"return"
            {
                self.cursor += 6;
                self.emit(Token::ContextReturn, start, self.cursor);
            } else if self.cursor + 4 < end
                && &self.source[self.cursor..self.cursor + 5] == b"event"
            {
                self.cursor += 5;
                self.emit(Token::ContextEvent, start, self.cursor);
            } else if self.cursor + 3 < end
                && &self.source[self.cursor..self.cursor + 4] == b"data"
            {
                self.cursor += 4;
                let key = self.scan_bracket_key(end);
                self.emit(Token::ContextData(key), start, self.cursor);
            } else if self.cursor + 5 < end
                && &self.source[self.cursor..self.cursor + 6] == b"params"
            {
                self.cursor += 6;
                let key = self.scan_bracket_key(end);
                self.emit(Token::ContextParams(key), start, self.cursor);
            } else {
                // Unknown @@: variant — emit as native
                let text = String::from_utf8_lossy(
                    &self.source[start..self.cursor]
                ).to_string();
                self.emit(Token::NativeCode(text), start, self.cursor);
            }
        } else {
            // Just "@@" without . or : — emit as native
            let text = String::from_utf8_lossy(
                &self.source[start..self.cursor]
            ).to_string();
            self.emit(Token::NativeCode(text), start, self.cursor);
        }

        Ok(())
    }

    // ========================================================================
    // Dollar-sign Lexing (Structural Mode)
    // ========================================================================

    fn lex_dollar(&mut self, start: usize) -> Result<(), LexError> {
        self.cursor += 1; // Skip $

        if self.cursor >= self.end {
            return Err(LexError::InvalidFrameConstruct {
                text: "$".to_string(),
                span: Span::new(start, self.cursor),
            });
        }

        let next = self.source[self.cursor];

        match next {
            // $> — enter handler
            b'>' => {
                self.cursor += 1;
                self.emit(Token::EnterHandler, start, self.cursor);
            }
            // $^ — parent ref
            b'^' => {
                self.cursor += 1;
                self.emit(Token::ParentRef, start, self.cursor);
            }
            // $.varName — state variable ref
            b'.' => {
                self.cursor += 1;
                let name = self.scan_identifier();
                self.emit(Token::StateVarRef(name), start, self.cursor);
            }
            // $StateName — state reference
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => {
                let name = self.scan_identifier();
                self.emit(Token::StateRef(name), start, self.cursor);
            }
            _ => {
                return Err(LexError::InvalidFrameConstruct {
                    text: format!("${}", next as char),
                    span: Span::new(start, self.cursor + 1),
                });
            }
        }

        Ok(())
    }

    // ========================================================================
    // Identifier and Keyword Lexing
    // ========================================================================

    fn lex_identifier_or_keyword(&mut self, start: usize) -> Result<(), LexError> {
        let word = self.scan_identifier();

        // Check for push$ and pop$
        if word == "push" && self.cursor < self.end && self.source[self.cursor] == b'$' {
            self.cursor += 1;
            self.emit(Token::PushState, start, self.cursor);
            return Ok(());
        }
        if word == "pop" && self.cursor < self.end && self.source[self.cursor] == b'$' {
            self.cursor += 1;
            self.emit(Token::PopState, start, self.cursor);
            return Ok(());
        }

        // Check for section keywords (with look-ahead for section colon)
        match word.as_str() {
            "interface" => {
                self.emit(Token::Interface, start, self.cursor);
                self.try_emit_section_colon();
            }
            "machine" => {
                self.emit(Token::Machine, start, self.cursor);
                self.try_emit_section_colon();
            }
            "actions" => {
                self.emit(Token::Actions, start, self.cursor);
                self.try_emit_section_colon();
            }
            "operations" => {
                self.emit(Token::Operations, start, self.cursor);
                self.try_emit_section_colon();
            }
            "domain" => {
                self.emit(Token::Domain, start, self.cursor);
                self.try_emit_section_colon();
            }
            "var" => self.emit(Token::Var, start, self.cursor),
            "return" => self.emit(Token::Return, start, self.cursor),
            "true" => self.emit(Token::BoolLit(true), start, self.cursor),
            "false" => self.emit(Token::BoolLit(false), start, self.cursor),
            _ => self.emit(Token::Ident(word), start, self.cursor),
        }

        Ok(())
    }

    /// After a section keyword, look ahead for `:` and emit SectionColon if found.
    fn try_emit_section_colon(&mut self) {
        let saved = self.cursor;
        self.skip_whitespace_and_comments();
        if self.cursor < self.end && self.source[self.cursor] == b':' {
            let colon_start = self.cursor;
            self.cursor += 1;
            self.emit(Token::SectionColon, colon_start, self.cursor);
        } else {
            // No colon found — restore cursor
            self.cursor = saved;
        }
    }

    // ========================================================================
    // Number Lexing
    // ========================================================================

    fn lex_number(&mut self, start: usize) -> Result<(), LexError> {
        // Handle negative sign
        if self.source[self.cursor] == b'-' {
            self.cursor += 1;
        }

        // Consume digits
        while self.cursor < self.end && self.source[self.cursor].is_ascii_digit() {
            self.cursor += 1;
        }

        // Check for float: digits followed by . and more digits
        if self.cursor < self.end && self.source[self.cursor] == b'.'
            && self.cursor + 1 < self.end && self.source[self.cursor + 1].is_ascii_digit()
        {
            self.cursor += 1; // Skip .
            while self.cursor < self.end && self.source[self.cursor].is_ascii_digit() {
                self.cursor += 1;
            }
            let text = std::str::from_utf8(&self.source[start..self.cursor]).unwrap_or("0.0");
            let value = text.parse::<f64>().unwrap_or(0.0);
            self.emit(Token::FloatLit(value), start, self.cursor);
        } else {
            let text = std::str::from_utf8(&self.source[start..self.cursor]).unwrap_or("0");
            let value = text.parse::<i64>().unwrap_or(0);
            self.emit(Token::IntLit(value), start, self.cursor);
        }

        Ok(())
    }

    // ========================================================================
    // String Lexing
    // ========================================================================

    fn lex_string(&mut self, start: usize) -> Result<(), LexError> {
        let quote = self.source[self.cursor];
        self.cursor += 1;

        let mut content = String::new();
        while self.cursor < self.end {
            let b = self.source[self.cursor];
            if b == b'\\' && self.cursor + 1 < self.end {
                // Escape sequence — include the escaped character
                content.push(self.source[self.cursor + 1] as char);
                self.cursor += 2;
                continue;
            }
            if b == quote {
                self.cursor += 1;
                self.emit(Token::StringLit(content), start, self.cursor);
                return Ok(());
            }
            content.push(b as char);
            self.cursor += 1;
        }

        Err(LexError::UnterminatedString {
            span: Span::new(start, self.cursor),
        })
    }

    // ========================================================================
    // Helpers
    // ========================================================================

    fn emit(&mut self, token: Token, start: usize, end: usize) {
        self.pending.push_back(Spanned {
            token,
            span: Span::new(start, end),
        });
    }

    fn peek_byte(&self, offset: usize) -> Option<u8> {
        let pos = self.cursor + offset;
        if pos < self.end {
            Some(self.source[pos])
        } else {
            None
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.cursor < self.end {
            let b = self.source[self.cursor];
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                self.cursor += 1;
                continue;
            }
            // Always handle // line comments in structural sections
            // (Frame structural syntax uses // regardless of target language)
            if b == b'/' && self.cursor + 1 < self.end && self.source[self.cursor + 1] == b'/' {
                self.cursor += 2;
                while self.cursor < self.end && self.source[self.cursor] != b'\n' {
                    self.cursor += 1;
                }
                continue;
            }
            // Try to skip comments via SyntaxSkipper (handles #, /* */, etc.)
            if let Some(new_pos) = self.skipper.skip_comment(
                self.source, self.cursor, self.end
            ) {
                self.cursor = new_pos;
                continue;
            }
            break;
        }
    }

    fn skip_inline_whitespace(&mut self) {
        let end = if self.mode == LexerMode::NativeAware { self.native_end } else { self.end };
        while self.cursor < end {
            let b = self.source[self.cursor];
            if b == b' ' || b == b'\t' {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    fn scan_identifier(&mut self) -> String {
        let start = self.cursor;
        let end = if self.mode == LexerMode::NativeAware { self.native_end } else { self.end };
        while self.cursor < end {
            let b = self.source[self.cursor];
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.cursor += 1;
            } else {
                break;
            }
        }
        String::from_utf8_lossy(&self.source[start..self.cursor]).to_string()
    }

    fn scan_bracket_key(&mut self, end: usize) -> String {
        if self.cursor < end && self.source[self.cursor] == b'[' {
            self.cursor += 1; // Skip [
            let key_start = self.cursor;
            while self.cursor < end && self.source[self.cursor] != b']' {
                self.cursor += 1;
            }
            let key = String::from_utf8_lossy(
                &self.source[key_start..self.cursor]
            ).to_string();
            if self.cursor < end {
                self.cursor += 1; // Skip ]
            }
            key
        } else {
            String::new()
        }
    }

    /// Advance cursor to end of current line (consuming any trailing content).
    fn advance_to_line_end(&mut self, end: usize) {
        let line_end = self.skipper.find_line_end(self.source, self.cursor, end);
        self.cursor = line_end;
        // Skip newline character
        if self.cursor < end && self.source[self.cursor] == b'\n' {
            self.cursor += 1;
        }
    }

    /// Skip to actual newline, ignoring language-specific comment/semicolon boundaries.
    /// Used after Frame statements (transitions, forwards) where trailing comments are noise.
    fn skip_to_newline(&mut self, end: usize) {
        while self.cursor < end && self.source[self.cursor] != b'\n' {
            self.cursor += 1;
        }
        if self.cursor < end {
            self.cursor += 1; // skip \n
        }
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a language-specific SyntaxSkipper.
fn create_skipper(lang: TargetLanguage) -> Box<dyn SyntaxSkipper> {
    match lang {
        TargetLanguage::Python3 => Box::new(PythonSkipper),
        TargetLanguage::TypeScript => Box::new(TypeScriptSkipper),
        TargetLanguage::Rust => Box::new(RustSkipper),
        TargetLanguage::C => Box::new(CSkipper),
        TargetLanguage::Cpp => Box::new(CppSkipper),
        TargetLanguage::Java => Box::new(JavaSkipper),
        TargetLanguage::CSharp => Box::new(CSharpSkipper),
        _ => Box::new(PythonSkipper), // Default fallback
    }
}

/// Convenience function to lex an entire system body in structural mode.
/// Useful for testing. For the full pipeline, use the `Lexer` struct directly.
pub fn lex(source: &[u8], body_span: Span, lang: TargetLanguage) -> Result<Vec<Spanned>, LexError> {
    let mut lexer = Lexer::new(source, body_span, lang);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token()?;
        if tok.token == Token::Eof {
            break;
        }
        tokens.push(tok);
    }
    Ok(tokens)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: lex source bytes as a Python system body
    fn lex_py(src: &str) -> Vec<Token> {
        let bytes = src.as_bytes();
        let span = Span::new(0, bytes.len());
        lex(bytes, span, TargetLanguage::Python3)
            .unwrap()
            .into_iter()
            .map(|s| s.token)
            .collect()
    }

    // Helper: create a lexer for Python
    fn make_lexer(src: &[u8]) -> Lexer<'_> {
        Lexer::new(src, Span::new(0, src.len()), TargetLanguage::Python3)
    }

    // ============================
    // Structural Mode Tests
    // ============================

    #[test]
    fn test_empty_source() {
        let tokens = lex_py("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_section_keywords() {
        let tokens = lex_py("interface:");
        assert_eq!(tokens, vec![Token::Interface, Token::SectionColon]);
    }

    #[test]
    fn test_section_keyword_with_space_before_colon() {
        let tokens = lex_py("machine :");
        assert_eq!(tokens, vec![Token::Machine, Token::SectionColon]);
    }

    #[test]
    fn test_all_section_keywords() {
        let tokens = lex_py("interface: machine: actions: operations: domain:");
        assert_eq!(tokens, vec![
            Token::Interface, Token::SectionColon,
            Token::Machine, Token::SectionColon,
            Token::Actions, Token::SectionColon,
            Token::Operations, Token::SectionColon,
            Token::Domain, Token::SectionColon,
        ]);
    }

    #[test]
    fn test_identifier() {
        let tokens = lex_py("myMethod");
        assert_eq!(tokens, vec![Token::Ident("myMethod".to_string())]);
    }

    #[test]
    fn test_method_signature() {
        let tokens = lex_py("start(msg: str): int");
        assert_eq!(tokens, vec![
            Token::Ident("start".to_string()),
            Token::LParen,
            Token::Ident("msg".to_string()),
            Token::Colon,
            Token::Ident("str".to_string()),
            Token::RParen,
            Token::Colon,
            Token::Ident("int".to_string()),
        ]);
    }

    #[test]
    fn test_state_ref() {
        let tokens = lex_py("$Idle");
        assert_eq!(tokens, vec![Token::StateRef("Idle".to_string())]);
    }

    #[test]
    fn test_enter_exit_handlers() {
        let tokens = lex_py("$> <$");
        assert_eq!(tokens, vec![Token::EnterHandler, Token::ExitHandler]);
    }

    #[test]
    fn test_state_var_ref() {
        let tokens = lex_py("$.counter");
        assert_eq!(tokens, vec![Token::StateVarRef("counter".to_string())]);
    }

    #[test]
    fn test_parent_ref() {
        let tokens = lex_py("$^");
        assert_eq!(tokens, vec![Token::ParentRef]);
    }

    #[test]
    fn test_arrow() {
        let tokens = lex_py("->");
        assert_eq!(tokens, vec![Token::Arrow]);
    }

    #[test]
    fn test_fat_arrow() {
        let tokens = lex_py("=>");
        assert_eq!(tokens, vec![Token::FatArrow]);
    }

    #[test]
    fn test_push_pop() {
        let tokens = lex_py("push$ pop$");
        assert_eq!(tokens, vec![Token::PushState, Token::PopState]);
    }

    #[test]
    fn test_delimiters() {
        let tokens = lex_py("{ } ( ) [ ] , : = . ;");
        assert_eq!(tokens, vec![
            Token::LBrace, Token::RBrace,
            Token::LParen, Token::RParen,
            Token::LBracket, Token::RBracket,
            Token::Comma, Token::Colon,
            Token::Equals, Token::Dot,
            Token::Semicolon,
        ]);
    }

    #[test]
    fn test_integer_literal() {
        let tokens = lex_py("42");
        assert_eq!(tokens, vec![Token::IntLit(42)]);
    }

    #[test]
    fn test_negative_integer() {
        let tokens = lex_py("-7");
        assert_eq!(tokens, vec![Token::IntLit(-7)]);
    }

    #[test]
    fn test_float_literal() {
        let tokens = lex_py("3.14");
        assert_eq!(tokens, vec![Token::FloatLit(3.14)]);
    }

    #[test]
    fn test_string_literal() {
        let tokens = lex_py(r#""hello""#);
        assert_eq!(tokens, vec![Token::StringLit("hello".to_string())]);
    }

    #[test]
    fn test_single_quote_string() {
        let tokens = lex_py("'world'");
        assert_eq!(tokens, vec![Token::StringLit("world".to_string())]);
    }

    #[test]
    fn test_bool_literals() {
        let tokens = lex_py("true false");
        assert_eq!(tokens, vec![Token::BoolLit(true), Token::BoolLit(false)]);
    }

    #[test]
    fn test_var_keyword() {
        let tokens = lex_py("var x = 0");
        assert_eq!(tokens, vec![
            Token::Var,
            Token::Ident("x".to_string()),
            Token::Equals,
            Token::IntLit(0),
        ]);
    }

    #[test]
    fn test_return_keyword() {
        let tokens = lex_py("return");
        assert_eq!(tokens, vec![Token::Return]);
    }

    #[test]
    fn test_full_interface_section() {
        let tokens = lex_py("interface:\n    start()\n    stop(msg: str)");
        assert_eq!(tokens, vec![
            Token::Interface, Token::SectionColon,
            Token::Ident("start".to_string()), Token::LParen, Token::RParen,
            Token::Ident("stop".to_string()), Token::LParen,
            Token::Ident("msg".to_string()), Token::Colon,
            Token::Ident("str".to_string()), Token::RParen,
        ]);
    }

    #[test]
    fn test_state_block_header() {
        let tokens = lex_py("$Running {\n    $>()\n}");
        assert_eq!(tokens, vec![
            Token::StateRef("Running".to_string()),
            Token::LBrace,
            Token::EnterHandler, Token::LParen, Token::RParen,
            Token::RBrace,
        ]);
    }

    #[test]
    fn test_domain_section() {
        let src = "domain:\n    var count: int = 0\n    var name = \"hello\"";
        let tokens = lex_py(src);
        assert_eq!(tokens, vec![
            Token::Domain, Token::SectionColon,
            Token::Var, Token::Ident("count".to_string()), Token::Colon,
            Token::Ident("int".to_string()), Token::Equals, Token::IntLit(0),
            Token::Var, Token::Ident("name".to_string()), Token::Equals,
            Token::StringLit("hello".to_string()),
        ]);
    }

    #[test]
    fn test_full_method_signature_with_alias() {
        // foo(a: int, b: str): str = "myfoo"
        let tokens = lex_py(r#"foo(a: int, b: str): str = "myfoo""#);
        assert_eq!(tokens, vec![
            Token::Ident("foo".to_string()),
            Token::LParen,
            Token::Ident("a".to_string()), Token::Colon, Token::Ident("int".to_string()),
            Token::Comma,
            Token::Ident("b".to_string()), Token::Colon, Token::Ident("str".to_string()),
            Token::RParen,
            Token::Colon, Token::Ident("str".to_string()),
            Token::Equals,
            Token::StringLit("myfoo".to_string()),
        ]);
    }

    #[test]
    fn test_comments_are_skipped() {
        // Python comment should be skipped
        let tokens = lex_py("interface: # this is a comment\n    start()");
        assert_eq!(tokens, vec![
            Token::Interface, Token::SectionColon,
            Token::Ident("start".to_string()), Token::LParen, Token::RParen,
        ]);
    }

    // ============================
    // Native-Aware Mode Tests
    // ============================

    #[test]
    fn test_native_simple_code() {
        let src = b"    x = 42\n    y = x + 1\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token().unwrap();
            if tok.token == Token::Eof { break; }
            tokens.push(tok.token);
        }
        // All native code, no Frame constructs
        assert_eq!(tokens.len(), 1);
        assert!(matches!(&tokens[0], Token::NativeCode(_)));
    }

    #[test]
    fn test_native_transition_at_sol() {
        let src = b"    -> $Running\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::Arrow);

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::StateRef("Running".to_string()));
    }

    #[test]
    fn test_native_state_var_midline() {
        let src = b"x = $.counter + 1\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::NativeCode("x = ".to_string()));

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::StateVarRef("counter".to_string()));

        let tok3 = lexer.next_token().unwrap();
        assert!(matches!(tok3.token, Token::NativeCode(_)));
    }

    #[test]
    fn test_native_context_param() {
        let src = b"result = @@.value\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::NativeCode("result = ".to_string()));

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::ContextParam("value".to_string()));
    }

    #[test]
    fn test_native_context_return() {
        let src = b"x = @@:return\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::NativeCode("x = ".to_string()));

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::ContextReturn);
    }

    #[test]
    fn test_native_context_data() {
        let src = b"v = @@:data[mykey]\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::NativeCode("v = ".to_string()));

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::ContextData("mykey".to_string()));
    }

    #[test]
    fn test_native_push_at_sol() {
        let src = b"    push$\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok = lexer.next_token().unwrap();
        assert_eq!(tok.token, Token::PushState);
    }

    #[test]
    fn test_native_pop_at_sol() {
        let src = b"    pop$\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok = lexer.next_token().unwrap();
        assert_eq!(tok.token, Token::PopState);
    }

    #[test]
    fn test_native_return_sugar() {
        let src = b"    return x + 1\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::Return);

        let tok2 = lexer.next_token().unwrap();
        assert!(matches!(tok2.token, Token::NativeCode(_)));
    }

    #[test]
    fn test_native_forward() {
        let src = b"    => $^\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::FatArrow);

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::ParentRef);
    }

    #[test]
    fn test_native_backtick_prefix() {
        let src = b"    `-> $Running\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::Arrow);

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::StateRef("Running".to_string()));
    }

    #[test]
    fn test_native_mixed_code_and_frame() {
        let src = b"x = 1\n    -> $Next\ny = 2\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        // First: "x = 1\n" as native code (before the transition line)
        let tok1 = lexer.next_token().unwrap();
        assert!(matches!(&tok1.token, Token::NativeCode(s) if s.contains("x = 1")));

        // Then the transition tokens
        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::Arrow);

        let tok3 = lexer.next_token().unwrap();
        assert_eq!(tok3.token, Token::StateRef("Next".to_string()));

        // Then "y = 2\n" as native code
        let tok4 = lexer.next_token().unwrap();
        assert!(matches!(&tok4.token, Token::NativeCode(s) if s.contains("y = 2")));
    }

    #[test]
    fn test_native_state_var_in_string_ignored() {
        // $.counter inside a Python string should NOT be detected as a Frame construct
        let src = b"x = \"$.counter\"\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token().unwrap();
            if tok.token == Token::Eof { break; }
            tokens.push(tok.token);
        }

        // The $.counter is inside a string, so no StateVarRef should be emitted
        assert!(tokens.iter().all(|t| !matches!(t, Token::StateVarRef(_))),
            "$.counter inside string should not be detected as Frame construct");
    }

    #[test]
    fn test_native_transition_in_comment_ignored() {
        // -> $State inside a comment should NOT be detected
        let src = b"# -> $State\nx = 1\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token().unwrap();
            if tok.token == Token::Eof { break; }
            tokens.push(tok.token);
        }

        // The -> inside a comment should not produce Arrow token
        assert!(tokens.iter().all(|t| !matches!(t, Token::Arrow)),
            "-> inside comment should not be detected as transition");
    }

    #[test]
    fn test_mode_switching() {
        // Start in structural, switch to native, switch back
        let src = b"interface: $Idle { x = 1 }";
        let mut lexer = make_lexer(src);

        // Structural: interface:
        let t1 = lexer.next_token().unwrap();
        assert_eq!(t1.token, Token::Interface);
        let t2 = lexer.next_token().unwrap();
        assert_eq!(t2.token, Token::SectionColon);

        // Structural: $Idle
        let t3 = lexer.next_token().unwrap();
        assert_eq!(t3.token, Token::StateRef("Idle".to_string()));

        // Structural: {
        let t4 = lexer.next_token().unwrap();
        assert_eq!(t4.token, Token::LBrace);

        // Switch to native mode (body ends at position of })
        let body_end = src.len() - 1; // position of }
        lexer.enter_native_mode(body_end);

        // Native: "x = 1 " (the space before })
        let t5 = lexer.next_token().unwrap();
        assert!(matches!(&t5.token, Token::NativeCode(s) if s.contains("x = 1")));

        // Switch back to structural
        lexer.enter_structural_mode();
        lexer.set_cursor(body_end);

        // Structural: }
        let t6 = lexer.next_token().unwrap();
        assert_eq!(t6.token, Token::RBrace);
    }

    #[test]
    fn test_context_event() {
        let src = b"name = @@:event\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::NativeCode("name = ".to_string()));

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::ContextEvent);
    }

    #[test]
    fn test_context_params() {
        let src = b"v = @@:params[age]\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let _tok1 = lexer.next_token().unwrap();
        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::ContextParams("age".to_string()));
    }

    #[test]
    fn test_native_pop_transition() {
        // -> pop$ is a pop transition
        let src = b"    -> pop$\n";
        let mut lexer = make_lexer(src);
        lexer.enter_native_mode(src.len());

        let tok1 = lexer.next_token().unwrap();
        assert_eq!(tok1.token, Token::Arrow);

        let tok2 = lexer.next_token().unwrap();
        assert_eq!(tok2.token, Token::PopState);
    }

    #[test]
    fn test_lex_convenience() {
        let src = b"interface: machine:";
        let tokens = lex(src, Span::new(0, src.len()), TargetLanguage::Python3).unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Interface);
        assert_eq!(tokens[1].token, Token::SectionColon);
        assert_eq!(tokens[2].token, Token::Machine);
        assert_eq!(tokens[3].token, Token::SectionColon);
    }

    #[test]
    fn test_handler_signature() {
        // Handler in machine section: eventName(params) {
        let tokens = lex_py("start(msg: str) {");
        assert_eq!(tokens, vec![
            Token::Ident("start".to_string()),
            Token::LParen,
            Token::Ident("msg".to_string()),
            Token::Colon,
            Token::Ident("str".to_string()),
            Token::RParen,
            Token::LBrace,
        ]);
    }
}

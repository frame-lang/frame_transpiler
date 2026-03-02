//! Frame Parser (Stage 2 of the V4 Pipeline)
//!
//! Recursive descent parser that consumes tokens from the Lexer and builds
//! a complete `SystemAst`. The parser controls the Lexer's mode switching:
//! - Structural mode for section headers, method signatures, state blocks
//! - Native-aware mode for handler/action/operation bodies
//!
//! After parsing, the AST contains every Frame statement and every native code
//! chunk — no further source scanning is needed.

use crate::frame_c::v4::frame_ast::*;
use crate::frame_c::v4::lexer::{Lexer, Token, Spanned, LexError, LexerMode};
use crate::frame_c::visitors::TargetLanguage;

// ============================================================================
// Parse Error
// ============================================================================

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}-{}: {}", self.span.start, self.span.end, self.message)
    }
}

impl std::error::Error for ParseError {}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        let span = match &e {
            LexError::UnexpectedByte { span, .. } => span.clone(),
            LexError::UnterminatedString { span } => span.clone(),
            LexError::UnterminatedComment { span } => span.clone(),
            LexError::InvalidFrameConstruct { span, .. } => span.clone(),
        };
        ParseError {
            message: e.to_string(),
            span,
        }
    }
}

// ============================================================================
// Parser
// ============================================================================

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Create a new Parser wrapping a Lexer.
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser { lexer }
    }

    /// Parse the system body into a SystemAst.
    /// `name` is the system name (extracted by the Segmenter).
    pub fn parse_system(&mut self, name: String) -> Result<SystemAst, ParseError> {
        let start = self.lexer.cursor();
        let mut system = SystemAst::new(name, Span::new(start, start));

        // Parse sections until Eof
        loop {
            let tok = self.peek()?;
            match tok {
                Token::Eof => break,

                Token::Interface => {
                    self.advance()?;
                    self.expect_section_colon()?;
                    system.section_order.push(SystemSectionKind::Interface);
                    let methods = self.parse_interface_methods()?;
                    system.interface = methods;
                }

                Token::Machine => {
                    self.advance()?;
                    self.expect_section_colon()?;
                    system.section_order.push(SystemSectionKind::Machine);
                    let machine = self.parse_machine()?;
                    system.machine = Some(machine);
                }

                Token::Actions => {
                    self.advance()?;
                    self.expect_section_colon()?;
                    system.section_order.push(SystemSectionKind::Actions);
                    let actions = self.parse_actions()?;
                    system.actions = actions;
                }

                Token::Operations => {
                    self.advance()?;
                    self.expect_section_colon()?;
                    system.section_order.push(SystemSectionKind::Operations);
                    let operations = self.parse_operations()?;
                    system.operations = operations;
                }

                Token::Domain => {
                    self.advance()?;
                    self.expect_section_colon()?;
                    system.section_order.push(SystemSectionKind::Domain);
                    let domain = self.parse_domain()?;
                    system.domain = domain;
                }

                _ => {
                    let spanned = self.advance()?;
                    return Err(ParseError {
                        message: format!("Expected section keyword, found {:?}", spanned.token),
                        span: spanned.span,
                    });
                }
            }
        }

        system.span = Span::new(start, self.lexer.cursor());
        Ok(system)
    }

    // ========================================================================
    // Interface Section
    // ========================================================================

    fn parse_interface_methods(&mut self) -> Result<Vec<InterfaceMethod>, ParseError> {
        let mut methods = Vec::new();

        loop {
            let tok = self.peek()?;
            match tok {
                // Next section or end of system
                Token::Machine | Token::Actions | Token::Operations
                | Token::Domain | Token::Eof => break,
                // Another interface keyword (duplicate section)
                Token::Interface => break,
                // Method name
                Token::Ident(_) => {
                    let method = self.parse_interface_method()?;
                    methods.push(method);
                }
                _ => {
                    let spanned = self.advance()?;
                    return Err(ParseError {
                        message: format!(
                            "Expected method name in interface, found {:?}",
                            spanned.token
                        ),
                        span: spanned.span,
                    });
                }
            }
        }

        Ok(methods)
    }

    fn parse_interface_method(&mut self) -> Result<InterfaceMethod, ParseError> {
        let name_tok = self.expect_ident()?;
        let name = name_tok.0;
        let start = name_tok.1.start;

        // Parse parameter list: (params)
        let params = if self.check(&Token::LParen)? {
            self.advance()?; // (
            let params = self.parse_method_params()?;
            self.expect_token(&Token::RParen)?; // )
            params
        } else {
            vec![]
        };

        // Optional return type: : Type
        let return_type = if self.check(&Token::Colon)? {
            self.advance()?; // :
            Some(self.parse_type()?)
        } else {
            None
        };

        // Optional return init: = expr (can be multi-token like "self.x + a")
        let return_init = if self.check(&Token::Equals)? {
            self.advance()?; // =
            // Scan source bytes from cursor to end of line to capture the full expression
            let src = self.lexer.source();
            let init_start = self.lexer.cursor();
            let mut pos = init_start;
            while pos < src.len() && src[pos] != b'\n' {
                pos += 1;
            }
            let mut init_text = std::str::from_utf8(&src[init_start..pos])
                .unwrap_or("").trim().to_string();
            // Strip surrounding quotes for string literal aliases
            if (init_text.starts_with('"') && init_text.ends_with('"'))
                || (init_text.starts_with('\'') && init_text.ends_with('\''))
            {
                init_text = init_text[1..init_text.len() - 1].to_string();
            }
            // Advance lexer cursor past this expression
            self.lexer.set_cursor(pos);
            if init_text.is_empty() { None } else { Some(init_text) }
        } else {
            None
        };

        Ok(InterfaceMethod {
            name,
            params,
            return_type,
            return_init,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_method_params(&mut self) -> Result<Vec<MethodParam>, ParseError> {
        let mut params = Vec::new();

        loop {
            if self.check(&Token::RParen)? {
                break;
            }

            let (name, span) = self.expect_ident()?;
            let param_type = if self.check(&Token::Colon)? {
                self.advance()?;
                self.parse_type()?
            } else {
                Type::Unknown
            };

            let default = if self.check(&Token::Equals)? {
                self.advance()?;
                let tok = self.advance()?;
                match tok.token {
                    Token::StringLit(s) => Some(s),
                    Token::IntLit(i) => Some(i.to_string()),
                    Token::FloatLit(f) => Some(f.to_string()),
                    Token::Ident(s) => Some(s),
                    Token::BoolLit(b) => Some(b.to_string()),
                    _ => None,
                }
            } else {
                None
            };

            params.push(MethodParam {
                name,
                param_type,
                default,
                span,
            });

            if self.check(&Token::Comma)? {
                self.advance()?; // ,
            }
        }

        Ok(params)
    }

    // ========================================================================
    // Machine Section
    // ========================================================================

    fn parse_machine(&mut self) -> Result<MachineAst, ParseError> {
        let start = self.lexer.cursor();
        let mut states = Vec::new();

        loop {
            let tok = self.peek()?;
            match tok {
                // State declaration: $StateName
                Token::StateRef(_) => {
                    let state = self.parse_state()?;
                    states.push(state);
                }
                // Next section or end
                Token::Interface | Token::Actions | Token::Operations
                | Token::Domain | Token::Eof => break,
                Token::Machine => break,
                _ => {
                    let spanned = self.advance()?;
                    return Err(ParseError {
                        message: format!(
                            "Expected state declaration in machine, found {:?}",
                            spanned.token
                        ),
                        span: spanned.span,
                    });
                }
            }
        }

        Ok(MachineAst {
            states,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_state(&mut self) -> Result<StateAst, ParseError> {
        let spanned = self.advance()?;
        let state_name = match spanned.token {
            Token::StateRef(name) => name,
            _ => return Err(self.unexpected(&spanned, "state name ($StateName)")),
        };
        let start = spanned.span.start;

        // Optional parent: => $ParentName
        let parent = if self.check(&Token::FatArrow)? {
            self.advance()?; // =>
            let parent_tok = self.advance()?;
            match parent_tok.token {
                Token::StateRef(name) => Some(name),
                _ => return Err(self.unexpected(&parent_tok, "parent state ($ParentName)")),
            }
        } else {
            None
        };

        // Optional state params: (params)
        let params = if self.check(&Token::LParen)? {
            self.advance()?;
            let params = self.parse_state_params()?;
            self.expect_token(&Token::RParen)?;
            params
        } else {
            vec![]
        };

        // State body: { ... }
        let brace_tok = self.expect_token(&Token::LBrace)?;
        let body_start = brace_tok.span.start;
        let body_close = self.lexer.find_close_brace(body_start)
            .ok_or_else(|| ParseError {
                message: format!("Unmatched '{{' for state {}", state_name),
                span: brace_tok.span.clone(),
            })?;

        let mut state = StateAst::new(state_name, Span::new(start, body_close + 1));
        state.parent = parent;
        state.params = params;
        state.body_span = Span::new(body_start + 1, body_close);

        // Parse state body contents
        self.parse_state_body(&mut state, body_close)?;

        // Skip past closing brace
        self.lexer.set_cursor(body_close + 1);

        Ok(state)
    }

    fn parse_state_body(
        &mut self,
        state: &mut StateAst,
        body_close: usize,
    ) -> Result<(), ParseError> {
        loop {
            let tok = self.peek()?;
            match tok {
                Token::RBrace | Token::Eof => break,

                // State variable declaration: $.varName
                Token::StateVarRef(_) => {
                    // Check if this is a state var declaration ($.name: type = init)
                    // by looking ahead for : or =
                    let sv = self.parse_state_var_decl()?;
                    state.state_vars.push(sv);
                }

                // Enter handler: $>
                Token::EnterHandler => {
                    let handler = self.parse_enter_handler(body_close)?;
                    state.enter = Some(handler);
                }

                // Exit handler: <$
                Token::ExitHandler => {
                    let handler = self.parse_exit_handler(body_close)?;
                    state.exit = Some(handler);
                }

                // Event handler: identifier(params) { body }
                Token::Ident(_) => {
                    let handler = self.parse_event_handler(body_close)?;
                    state.handlers.push(handler);
                }

                // Default forward: => $^
                Token::FatArrow => {
                    self.advance()?;
                    if self.check(&Token::ParentRef)? {
                        self.advance()?;
                        state.default_forward = true;
                    }
                }

                _ => {
                    // Skip unknown tokens in state body
                    self.advance()?;
                }
            }
        }
        Ok(())
    }

    fn parse_state_var_decl(&mut self) -> Result<StateVarAst, ParseError> {
        let spanned = self.advance()?;
        let name = match spanned.token {
            Token::StateVarRef(n) => n,
            _ => return Err(self.unexpected(&spanned, "state variable ($.name)")),
        };
        let start = spanned.span.start;

        let var_type = if self.check(&Token::Colon)? {
            self.advance()?;
            self.parse_type()?
        } else {
            Type::Unknown
        };

        let init = if self.check(&Token::Equals)? {
            self.advance()?;
            Some(self.parse_simple_expression()?)
        } else {
            None
        };

        Ok(StateVarAst {
            name,
            var_type,
            init,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_state_params(&mut self) -> Result<Vec<StateParam>, ParseError> {
        let mut params = Vec::new();
        loop {
            if self.check(&Token::RParen)? {
                break;
            }
            let (name, span) = self.expect_ident()?;
            let param_type = if self.check(&Token::Colon)? {
                self.advance()?;
                self.parse_type()?
            } else {
                Type::Unknown
            };
            params.push(StateParam {
                name,
                param_type,
                span,
            });
            if self.check(&Token::Comma)? {
                self.advance()?;
            }
        }
        Ok(params)
    }

    // ========================================================================
    // Handler Parsing
    // ========================================================================

    fn parse_enter_handler(
        &mut self,
        _state_close: usize,
    ) -> Result<EnterHandler, ParseError> {
        let start_tok = self.advance()?; // $>
        let start = start_tok.span.start;

        // Optional params
        let params = if self.check(&Token::LParen)? {
            self.advance()?;
            let p = self.parse_event_params()?;
            self.expect_token(&Token::RParen)?;
            p
        } else {
            vec![]
        };

        // Body: { ... }
        let body = self.parse_body_block()?;

        Ok(EnterHandler {
            params,
            body,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_exit_handler(
        &mut self,
        _state_close: usize,
    ) -> Result<ExitHandler, ParseError> {
        let start_tok = self.advance()?; // <$
        let start = start_tok.span.start;

        // Optional params
        let params = if self.check(&Token::LParen)? {
            self.advance()?;
            let p = self.parse_event_params()?;
            self.expect_token(&Token::RParen)?;
            p
        } else {
            vec![]
        };

        // Body: { ... }
        let body = self.parse_body_block()?;

        Ok(ExitHandler {
            params,
            body,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_event_handler(
        &mut self,
        _state_close: usize,
    ) -> Result<HandlerAst, ParseError> {
        let (event_name, name_span) = self.expect_ident()?;
        let start = name_span.start;

        // Optional params
        let params = if self.check(&Token::LParen)? {
            self.advance()?;
            let p = self.parse_event_params()?;
            self.expect_token(&Token::RParen)?;
            p
        } else {
            vec![]
        };

        // Optional return type
        let return_type = if self.check(&Token::Colon)? {
            self.advance()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        // Body: { ... }
        let body = self.parse_body_block()?;

        Ok(HandlerAst {
            event: event_name,
            params,
            return_type,
            body,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_event_params(&mut self) -> Result<Vec<EventParam>, ParseError> {
        let mut params = Vec::new();
        loop {
            if self.check(&Token::RParen)? {
                break;
            }
            let (name, span) = self.expect_ident()?;
            let param_type = if self.check(&Token::Colon)? {
                self.advance()?;
                self.parse_type()?
            } else {
                Type::Unknown
            };
            params.push(EventParam {
                name,
                param_type,
                span,
            });
            if self.check(&Token::Comma)? {
                self.advance()?;
            }
        }
        Ok(params)
    }

    // ========================================================================
    // Body Block Parsing (Mode Switching)
    // ========================================================================

    /// Parse a body block: `{ ... }`. Switches lexer to native-aware mode,
    /// collects all tokens into statements, then switches back.
    fn parse_body_block(&mut self) -> Result<HandlerBody, ParseError> {
        let brace_tok = self.expect_token(&Token::LBrace)?;
        let open_pos = brace_tok.span.start;

        let close_pos = self.lexer.find_close_brace(open_pos)
            .ok_or_else(|| ParseError {
                message: "Unmatched '{' in handler body".to_string(),
                span: brace_tok.span.clone(),
            })?;

        // body_span includes braces — codegen's splice_handler_body_from_span() expects this
        let body_span = Span::new(open_pos, close_pos + 1);

        // Switch to native-aware mode (Lexer operates INSIDE braces)
        self.lexer.enter_native_mode(close_pos);

        // Collect native tokens into statements
        let mut statements = Vec::new();
        loop {
            let tok = self.lexer.next_token().map_err(ParseError::from)?;
            match tok.token {
                Token::Eof => break,

                Token::NativeCode(code) => {
                    if !code.trim().is_empty() {
                        statements.push(Statement::NativeCode(code));
                    }
                }

                Token::Arrow => {
                    // Transition: -> $State or -> pop$
                    let next = self.lexer.next_token().map_err(ParseError::from)?;
                    match next.token {
                        Token::StateRef(target) => {
                            statements.push(Statement::Transition(TransitionAst {
                                target,
                                args: vec![],
                                span: Span::new(tok.span.start, next.span.end),
                                indent: 0,
                            }));
                        }
                        Token::FatArrow => {
                            // -> => $State (transition forward)
                            let target_tok = self.lexer.next_token()
                                .map_err(ParseError::from)?;
                            if let Token::StateRef(target) = target_tok.token {
                                statements.push(
                                    Statement::TransitionForward(TransitionForwardAst {
                                        target,
                                        span: Span::new(
                                            tok.span.start,
                                            target_tok.span.end,
                                        ),
                                        indent: 0,
                                    }),
                                );
                            }
                        }
                        Token::PopState => {
                            statements.push(Statement::StackPop(StackPopAst {
                                span: Span::new(tok.span.start, next.span.end),
                                indent: 0,
                            }));
                        }
                        Token::NativeCode(args) => {
                            // -> (args) $State — args is native code for enter params
                            let target_tok = self.lexer.next_token()
                                .map_err(ParseError::from)?;
                            if let Token::StateRef(target) = target_tok.token {
                                // Store args as NativeExpr
                                statements.push(Statement::Transition(TransitionAst {
                                    target,
                                    args: vec![Expression::NativeExpr(args)],
                                    span: Span::new(
                                        tok.span.start,
                                        target_tok.span.end,
                                    ),
                                    indent: 0,
                                }));
                            }
                        }
                        _ => {}
                    }
                }

                Token::FatArrow => {
                    // Forward: => $^ or => $State
                    let next = self.lexer.next_token().map_err(ParseError::from)?;
                    match next.token {
                        Token::ParentRef | Token::StateRef(_) => {
                            let event = match &next.token {
                                Token::ParentRef => "$^".to_string(),
                                Token::StateRef(n) => n.clone(),
                                _ => unreachable!(),
                            };
                            statements.push(Statement::Forward(ForwardAst {
                                event,
                                args: vec![],
                                span: Span::new(tok.span.start, next.span.end),
                                indent: 0,
                            }));
                        }
                        _ => {}
                    }
                }

                Token::PushState => {
                    statements.push(Statement::StackPush(StackPushAst {
                        span: tok.span,
                        indent: 0,
                    }));
                }

                Token::PopState => {
                    statements.push(Statement::StackPop(StackPopAst {
                        span: tok.span,
                        indent: 0,
                    }));
                }

                Token::Return => {
                    // return <expr>
                    let next = self.lexer.next_token().map_err(ParseError::from)?;
                    let value = match next.token {
                        Token::NativeCode(code) => {
                            Some(Expression::NativeExpr(code.trim().to_string()))
                        }
                        Token::Eof => None,
                        _ => None,
                    };
                    statements.push(Statement::Return(ReturnAst {
                        value,
                        span: tok.span,
                    }));
                }

                Token::StateVarRef(name) => {
                    // State variable reference (mid-line in native code)
                    // The codegen will handle this via NativeCode chunks
                    // For now, store as NativeCode with the Frame syntax
                    statements.push(Statement::NativeCode(
                        format!("$.{}", name),
                    ));
                }

                Token::ContextParam(name) => {
                    statements.push(Statement::NativeCode(
                        format!("@@.{}", name),
                    ));
                }

                Token::ContextReturn => {
                    statements.push(Statement::NativeCode(
                        "@@:return".to_string(),
                    ));
                }

                Token::ContextEvent => {
                    statements.push(Statement::NativeCode(
                        "@@:event".to_string(),
                    ));
                }

                Token::ContextData(key) => {
                    statements.push(Statement::NativeCode(
                        format!("@@:data[{}]", key),
                    ));
                }

                Token::ContextParams(key) => {
                    statements.push(Statement::NativeCode(
                        format!("@@:params[{}]", key),
                    ));
                }

                Token::SystemReturn => {
                    statements.push(Statement::NativeCode(
                        "system.return".to_string(),
                    ));
                }

                _ => {
                    // Unknown token in native mode — skip
                }
            }
        }

        // Switch back to structural mode and skip past closing brace
        self.lexer.enter_structural_mode();
        self.lexer.set_cursor(close_pos + 1);

        Ok(HandlerBody {
            statements,
            span: body_span,
        })
    }

    // ========================================================================
    // Actions Section
    // ========================================================================

    fn parse_actions(&mut self) -> Result<Vec<ActionAst>, ParseError> {
        let mut actions = Vec::new();
        loop {
            let tok = self.peek()?;
            match tok {
                Token::Ident(_) => {
                    let action = self.parse_action()?;
                    actions.push(action);
                }
                Token::Interface | Token::Machine | Token::Operations
                | Token::Domain | Token::Eof | Token::Actions => break,
                _ => {
                    self.advance()?; // skip
                }
            }
        }
        Ok(actions)
    }

    fn parse_action(&mut self) -> Result<ActionAst, ParseError> {
        let (name, name_span) = self.expect_ident()?;
        let start = name_span.start;

        let params = if self.check(&Token::LParen)? {
            self.advance()?;
            let p = self.parse_action_params()?;
            self.expect_token(&Token::RParen)?;
            p
        } else {
            vec![]
        };

        let body = self.parse_body_block()?;
        let code = self.extract_span_content(&body.span);

        Ok(ActionAst {
            name,
            params,
            body: ActionBody { span: body.span, code: Some(code) },
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_action_params(&mut self) -> Result<Vec<ActionParam>, ParseError> {
        let mut params = Vec::new();
        loop {
            if self.check(&Token::RParen)? {
                break;
            }
            let (name, span) = self.expect_ident()?;
            let param_type = if self.check(&Token::Colon)? {
                self.advance()?;
                self.parse_type()?
            } else {
                Type::Unknown
            };
            params.push(ActionParam {
                name,
                param_type,
                default: None,
                span,
            });
            if self.check(&Token::Comma)? {
                self.advance()?;
            }
        }
        Ok(params)
    }

    // ========================================================================
    // Operations Section
    // ========================================================================

    fn parse_operations(&mut self) -> Result<Vec<OperationAst>, ParseError> {
        let mut ops = Vec::new();
        loop {
            let tok = self.peek()?;
            match tok {
                Token::Ident(_) => {
                    let op = self.parse_operation()?;
                    ops.push(op);
                }
                Token::Interface | Token::Machine | Token::Actions
                | Token::Domain | Token::Eof | Token::Operations => break,
                _ => {
                    self.advance()?;
                }
            }
        }
        Ok(ops)
    }

    fn parse_operation(&mut self) -> Result<OperationAst, ParseError> {
        // Check for `static` modifier
        let is_static = if let Token::Ident(name) = self.peek()? {
            name == "static"
        } else {
            false
        };
        if is_static {
            self.advance()?; // consume `static`
        }

        let (name, name_span) = self.expect_ident()?;
        let start = name_span.start;

        let params = if self.check(&Token::LParen)? {
            self.advance()?;
            let p = self.parse_operation_params()?;
            self.expect_token(&Token::RParen)?;
            p
        } else {
            vec![]
        };

        // Return type: : Type
        let return_type = if self.check(&Token::Colon)? {
            self.advance()?;
            self.parse_type()?
        } else {
            Type::Unknown
        };

        let body = self.parse_body_block()?;
        let code = self.extract_span_content(&body.span);

        Ok(OperationAst {
            name,
            params,
            return_type,
            body: OperationBody { span: body.span, code: Some(code) },
            is_static,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    fn parse_operation_params(&mut self) -> Result<Vec<OperationParam>, ParseError> {
        let mut params = Vec::new();
        loop {
            if self.check(&Token::RParen)? {
                break;
            }
            let (name, span) = self.expect_ident()?;
            let param_type = if self.check(&Token::Colon)? {
                self.advance()?;
                self.parse_type()?
            } else {
                Type::Unknown
            };
            params.push(OperationParam {
                name,
                param_type,
                default: None,
                span,
            });
            if self.check(&Token::Comma)? {
                self.advance()?;
            }
        }
        Ok(params)
    }

    // ========================================================================
    // Domain Section
    // ========================================================================

    fn parse_domain(&mut self) -> Result<Vec<DomainVar>, ParseError> {
        let mut vars = Vec::new();
        loop {
            let tok = self.peek()?;
            match tok {
                Token::Var | Token::Ident(_) => {
                    let dv = self.parse_domain_var()?;
                    vars.push(dv);
                }
                Token::Interface | Token::Machine | Token::Actions
                | Token::Operations | Token::Eof | Token::Domain => break,
                _ => {
                    self.advance()?;
                }
            }
        }
        Ok(vars)
    }

    fn parse_domain_var(&mut self) -> Result<DomainVar, ParseError> {
        // Domain vars can be:
        //   Frame style:  `var name: type = value` or `name: type = value`
        //   C style:      `int call_count` or `char* name = NULL`  (no `var`, no `:`)
        //   Rust style:   `name: &str` (handled by Frame-style with parse_type)
        let tok = self.peek()?;
        let has_var_keyword = matches!(tok, Token::Var);
        if has_var_keyword {
            self.advance()?; // consume `var`
        }
        let start = self.lexer.cursor();

        let (first_ident, _) = self.expect_ident()?;

        // Check what follows the first ident to detect C-style vs Frame-style
        // C-style: no `var`, next token is Star or another Ident (not `:`, `=`, section keyword)
        if !has_var_keyword {
            let next = self.peek()?;

            // Detect C pointer types: `char* name`, `int** ptr`
            if matches!(next, Token::Star) {
                let mut c_type = first_ident.clone();
                while self.check(&Token::Star)? {
                    self.advance()?;
                    c_type.push('*');
                }
                // After stars, expect the variable name
                let (name, _) = self.expect_ident()?;
                let raw_line = self.build_c_domain_raw(&c_type, &name)?;
                return Ok(DomainVar {
                    name,
                    var_type: Type::Custom(c_type),
                    initializer: None,
                    is_frame: false,
                    raw_code: Some(raw_line),
                    span: Span::new(start, self.lexer.cursor()),
                });
            }

            // Detect C value types: `int call_count`, `unsigned count`
            // Next token is Ident (another word) → C-style declaration
            if let Token::Ident(_) = next {
                let c_type = first_ident;
                let (name, _) = self.expect_ident()?;
                let raw_line = self.build_c_domain_raw(&c_type, &name)?;
                return Ok(DomainVar {
                    name,
                    var_type: Type::Custom(c_type),
                    initializer: None,
                    is_frame: false,
                    raw_code: Some(raw_line),
                    span: Span::new(start, self.lexer.cursor()),
                });
            }
        }

        // Frame style: first_ident is the name
        let name = first_ident;

        let var_type = if self.check(&Token::Colon)? {
            self.advance()?;
            self.parse_type()?
        } else {
            Type::Unknown
        };

        let initializer = if self.check(&Token::Equals)? {
            self.advance()?;
            Some(self.parse_simple_expression()?)
        } else {
            None
        };

        Ok(DomainVar {
            name,
            var_type,
            initializer,
            is_frame: true,
            raw_code: None,
            span: Span::new(start, self.lexer.cursor()),
        })
    }

    /// Build raw_code for a C-style domain variable, consuming optional `= initializer`.
    fn build_c_domain_raw(&mut self, c_type: &str, name: &str) -> Result<String, ParseError> {
        if self.check(&Token::Equals)? {
            // Scan source bytes to end of line for the initializer
            self.advance()?; // consume `=`
            let src = self.lexer.source();
            let init_start = self.lexer.cursor();
            let mut pos = init_start;
            while pos < src.len() && src[pos] != b'\n' {
                pos += 1;
            }
            let init_text = std::str::from_utf8(&src[init_start..pos])
                .unwrap_or("").trim().to_string();
            self.lexer.set_cursor(pos);
            Ok(format!("{} {} = {}", c_type, name, init_text))
        } else {
            Ok(format!("{} {}", c_type, name))
        }
    }

    // ========================================================================
    // Type Parsing
    // ========================================================================

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        // Scan raw source bytes for the type expression, since native types can be
        // arbitrarily complex: Vec<String>, HashMap<K,V>, string[], &str, *int, etc.
        let src = self.lexer.source();
        let start = self.lexer.cursor();
        let mut pos = start;
        let mut angle_depth = 0;
        let mut bracket_depth = 0;

        // Skip leading whitespace
        while pos < src.len() && (src[pos] == b' ' || src[pos] == b'\t') {
            pos += 1;
        }
        let type_start = pos;

        while pos < src.len() {
            let b = src[pos];
            match b {
                b'<' => { angle_depth += 1; pos += 1; }
                b'>' => { angle_depth -= 1; pos += 1; }
                b'[' => { bracket_depth += 1; pos += 1; }
                b']' => { bracket_depth -= 1; pos += 1; }
                // Stop at delimiters (only when not inside <> or [])
                b'\n' | b'{' if angle_depth == 0 && bracket_depth == 0 => break,
                b'=' | b')' | b',' if angle_depth == 0 && bracket_depth == 0 => break,
                // Type-valid characters: letters, digits, _, &, *, |, space, :, .
                _ => pos += 1,
            }
        }

        let type_text = std::str::from_utf8(&src[type_start..pos])
            .unwrap_or("").trim().to_string();
        self.lexer.set_cursor(pos);

        if type_text.is_empty() {
            return Ok(Type::Unknown);
        }

        match type_text.as_str() {
            "int" | "i32" | "i64" => Ok(Type::Int),
            "float" | "f32" | "f64" => Ok(Type::Float),
            "str" | "string" | "String" | "&str" => Ok(Type::String),
            "bool" => Ok(Type::Bool),
            _ => Ok(Type::Custom(type_text)),
        }
    }

    // ========================================================================
    // Expression Parsing (simplified)
    // ========================================================================

    fn parse_simple_expression(&mut self) -> Result<Expression, ParseError> {
        let tok = self.advance()?;
        match tok.token {
            Token::IntLit(i) => Ok(Expression::Literal(Literal::Int(i))),
            Token::FloatLit(f) => Ok(Expression::Literal(Literal::Float(f))),
            Token::StringLit(s) => Ok(Expression::Literal(Literal::String(s))),
            Token::BoolLit(b) => Ok(Expression::Literal(Literal::Bool(b))),
            Token::Ident(name) => {
                match name.as_str() {
                    "None" | "null" | "nullptr" | "nil" => {
                        Ok(Expression::NativeExpr(name))
                    }
                    _ => Ok(Expression::Var(name)),
                }
            }
            Token::LBracket => {
                // Collect everything until matching RBracket
                let mut content = String::from("[");
                let mut depth = 1;
                while depth > 0 {
                    let next = self.advance()?;
                    match &next.token {
                        Token::LBracket => { depth += 1; content.push('['); }
                        Token::RBracket => { depth -= 1; if depth > 0 { content.push(']'); } }
                        Token::Eof => break,
                        _ => {
                            // Extract source text for this token
                            let src = self.lexer.source();
                            let s = next.span.start.min(src.len());
                            let e = next.span.end.min(src.len());
                            content.push_str(std::str::from_utf8(&src[s..e]).unwrap_or(""));
                        }
                    }
                }
                content.push(']');
                Ok(Expression::NativeExpr(content))
            }
            Token::LBrace => {
                // Collect everything until matching RBrace (for dict literals etc.)
                let mut content = String::from("{");
                let mut depth = 1;
                while depth > 0 {
                    let next = self.advance()?;
                    match &next.token {
                        Token::LBrace => { depth += 1; content.push('{'); }
                        Token::RBrace => { depth -= 1; if depth > 0 { content.push('}'); } }
                        Token::Eof => break,
                        _ => {
                            let src = self.lexer.source();
                            let s = next.span.start.min(src.len());
                            let e = next.span.end.min(src.len());
                            content.push_str(std::str::from_utf8(&src[s..e]).unwrap_or(""));
                        }
                    }
                }
                content.push('}');
                Ok(Expression::NativeExpr(content))
            }
            _ => {
                // Fallback: extract the actual source text for this token
                let src = self.lexer.source();
                let s = tok.span.start.min(src.len());
                let e = tok.span.end.min(src.len());
                let text = std::str::from_utf8(&src[s..e]).unwrap_or("?");
                Ok(Expression::NativeExpr(text.to_string()))
            }
        }
    }

    // ========================================================================
    // Token Helpers
    // ========================================================================

    fn peek(&mut self) -> Result<&Token, ParseError> {
        self.lexer.peek().map_err(ParseError::from)
    }

    fn advance(&mut self) -> Result<Spanned, ParseError> {
        self.lexer.next_token().map_err(ParseError::from)
    }

    fn check(&mut self, expected: &Token) -> Result<bool, ParseError> {
        let tok = self.peek()?;
        Ok(std::mem::discriminant(tok) == std::mem::discriminant(expected))
    }

    fn expect_token(&mut self, expected: &Token) -> Result<Spanned, ParseError> {
        let tok = self.advance()?;
        if std::mem::discriminant(&tok.token) == std::mem::discriminant(expected) {
            Ok(tok)
        } else {
            Err(ParseError {
                message: format!("Expected {:?}, found {:?}", expected, tok.token),
                span: tok.span,
            })
        }
    }

    fn expect_ident(&mut self) -> Result<(String, Span), ParseError> {
        let tok = self.advance()?;
        match tok.token {
            Token::Ident(name) => Ok((name, tok.span)),
            _ => Err(self.unexpected(&tok, "identifier")),
        }
    }

    fn expect_section_colon(&mut self) -> Result<(), ParseError> {
        let tok = self.advance()?;
        match tok.token {
            Token::SectionColon => Ok(()),
            Token::Colon => Ok(()), // Accept regular colon too
            _ => Err(self.unexpected(&tok, "':'")),
        }
    }

    fn unexpected(&self, tok: &Spanned, expected: &str) -> ParseError {
        ParseError {
            message: format!("Expected {}, found {:?}", expected, tok.token),
            span: tok.span.clone(),
        }
    }

    /// Extract content from a body span (between braces, trimmed).
    fn extract_span_content(&self, span: &Span) -> String {
        let source = self.lexer.source();
        let end = span.end.min(source.len());
        let start = span.start.min(end);
        let text = std::str::from_utf8(&source[start..end]).unwrap_or("");
        text.trim_matches('\n').to_string()
    }
}

// ============================================================================
// Convenience Function
// ============================================================================

/// Parse a system body from source bytes.
/// `name` is the system name, `body_span` is the span inside the system braces.
pub fn parse_system(
    source: &[u8],
    name: String,
    body_span: Span,
    lang: TargetLanguage,
) -> Result<SystemAst, ParseError> {
    let lexer = Lexer::new(source, body_span, lang);
    let mut parser = Parser::new(lexer);
    parser.parse_system(name)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_py(src: &str) -> SystemAst {
        let bytes = src.as_bytes();
        let span = Span::new(0, bytes.len());
        parse_system(bytes, "TestSystem".to_string(), span, TargetLanguage::Python3)
            .expect("Parse failed")
    }

    #[test]
    fn test_empty_system() {
        let sys = parse_py("");
        assert_eq!(sys.name, "TestSystem");
        assert!(sys.interface.is_empty());
        assert!(sys.machine.is_none());
        assert!(sys.actions.is_empty());
        assert!(sys.domain.is_empty());
    }

    #[test]
    fn test_interface_simple() {
        let sys = parse_py("interface:\n    start()\n    stop()");
        assert_eq!(sys.interface.len(), 2);
        assert_eq!(sys.interface[0].name, "start");
        assert_eq!(sys.interface[1].name, "stop");
    }

    #[test]
    fn test_interface_with_params() {
        let sys = parse_py("interface:\n    send(msg: str, count: int)");
        assert_eq!(sys.interface.len(), 1);
        let m = &sys.interface[0];
        assert_eq!(m.name, "send");
        assert_eq!(m.params.len(), 2);
        assert_eq!(m.params[0].name, "msg");
        assert_eq!(m.params[0].param_type, Type::String);
        assert_eq!(m.params[1].name, "count");
        assert_eq!(m.params[1].param_type, Type::Int);
    }

    #[test]
    fn test_interface_with_return_type() {
        let sys = parse_py("interface:\n    getData(): int");
        assert_eq!(sys.interface[0].return_type, Some(Type::Int));
    }

    #[test]
    fn test_interface_with_alias() {
        let sys = parse_py(r#"interface:
    foo(a: int): str = "myFoo""#);
        assert_eq!(sys.interface[0].name, "foo");
        assert_eq!(sys.interface[0].return_init, Some("myFoo".to_string()));
    }

    #[test]
    fn test_machine_simple_state() {
        let sys = parse_py("machine:\n    $Idle {\n    }");
        let machine = sys.machine.unwrap();
        assert_eq!(machine.states.len(), 1);
        assert_eq!(machine.states[0].name, "Idle");
    }

    #[test]
    fn test_machine_state_with_handler() {
        let sys = parse_py(
            "machine:\n    $Idle {\n        start() {\n            -> $Running\n        }\n    }"
        );
        let machine = sys.machine.unwrap();
        assert_eq!(machine.states[0].name, "Idle");
        assert_eq!(machine.states[0].handlers.len(), 1);
        assert_eq!(machine.states[0].handlers[0].event, "start");

        // Check handler body has a transition
        let body = &machine.states[0].handlers[0].body;
        let has_transition = body.statements.iter().any(|s| {
            matches!(s, Statement::Transition(t) if t.target == "Running")
        });
        assert!(has_transition, "Handler body should contain transition to $Running");
    }

    #[test]
    fn test_machine_enter_handler() {
        let sys = parse_py(
            "machine:\n    $Init {\n        $>() {\n            x = 1\n        }\n    }"
        );
        let machine = sys.machine.unwrap();
        let state = &machine.states[0];
        assert!(state.enter.is_some());
        let enter = state.enter.as_ref().unwrap();
        // Body should contain native code
        assert!(!enter.body.statements.is_empty());
    }

    #[test]
    fn test_machine_exit_handler() {
        let sys = parse_py(
            "machine:\n    $Init {\n        <$() {\n            cleanup()\n        }\n    }"
        );
        let machine = sys.machine.unwrap();
        let state = &machine.states[0];
        assert!(state.exit.is_some());
    }

    #[test]
    fn test_domain_simple() {
        let sys = parse_py("domain:\n    var x = 0\n    var name = \"hello\"");
        assert_eq!(sys.domain.len(), 2);
        assert_eq!(sys.domain[0].name, "x");
        assert_eq!(sys.domain[1].name, "name");
    }

    #[test]
    fn test_domain_with_types() {
        let sys = parse_py("domain:\n    var count: int = 0");
        assert_eq!(sys.domain[0].name, "count");
        assert_eq!(sys.domain[0].var_type, Type::Int);
    }

    #[test]
    fn test_multiple_sections() {
        let sys = parse_py(
            "interface:\n    start()\n\nmachine:\n    $Idle {\n    }\n\ndomain:\n    var x = 0"
        );
        assert_eq!(sys.interface.len(), 1);
        assert!(sys.machine.is_some());
        assert_eq!(sys.domain.len(), 1);
    }

    #[test]
    fn test_section_order() {
        let sys = parse_py(
            "interface:\n    start()\nmachine:\n    $A {\n    }\ndomain:\n    var x = 0"
        );
        assert_eq!(sys.section_order, vec![
            SystemSectionKind::Interface,
            SystemSectionKind::Machine,
            SystemSectionKind::Domain,
        ]);
    }

    #[test]
    fn test_handler_with_native_and_transition() {
        let sys = parse_py(
            "machine:\n    $A {\n        go() {\n            x = 1\n            -> $B\n            y = 2\n        }\n    }\n    $B {\n    }"
        );
        let machine = sys.machine.unwrap();
        let handler = &machine.states[0].handlers[0];
        let stmts = &handler.body.statements;

        // Should have: NativeCode("x = 1"), Transition(B), NativeCode("y = 2")
        let has_native = stmts.iter().any(|s| matches!(s, Statement::NativeCode(_)));
        let has_transition = stmts.iter().any(|s| {
            matches!(s, Statement::Transition(t) if t.target == "B")
        });
        assert!(has_native, "Should have native code");
        assert!(has_transition, "Should have transition to $B");
    }

    #[test]
    fn test_handler_push_pop() {
        let sys = parse_py(
            "machine:\n    $A {\n        go() {\n            push$\n            -> $B\n        }\n    }\n    $B {\n        back() {\n            -> pop$\n        }\n    }"
        );
        let machine = sys.machine.unwrap();

        // First handler: push$ then transition
        let stmts_a = &machine.states[0].handlers[0].body.statements;
        let has_push = stmts_a.iter().any(|s| matches!(s, Statement::StackPush(_)));
        assert!(has_push, "Should have push$");

        // Second handler: -> pop$
        let stmts_b = &machine.states[1].handlers[0].body.statements;
        let has_pop = stmts_b.iter().any(|s| matches!(s, Statement::StackPop(_)));
        assert!(has_pop, "Should have pop$");
    }

    #[test]
    fn test_actions_section() {
        let sys = parse_py("actions:\n    doThing() {\n        print(1)\n    }");
        assert_eq!(sys.actions.len(), 1);
        assert_eq!(sys.actions[0].name, "doThing");
    }

    #[test]
    fn test_operations_section() {
        let sys = parse_py("operations:\n    getValue(): int {\n        return 42\n    }");
        assert_eq!(sys.operations.len(), 1);
        assert_eq!(sys.operations[0].name, "getValue");
        assert_eq!(sys.operations[0].return_type, Type::Int);
    }

    #[test]
    fn test_state_with_parent() {
        let sys = parse_py("machine:\n    $Child => $Parent {\n    }");
        let machine = sys.machine.unwrap();
        assert_eq!(machine.states[0].parent, Some("Parent".to_string()));
    }

    #[test]
    fn test_state_with_params() {
        let sys = parse_py("machine:\n    $Active(x: int, y: str) {\n    }");
        let machine = sys.machine.unwrap();
        assert_eq!(machine.states[0].params.len(), 2);
        assert_eq!(machine.states[0].params[0].name, "x");
    }

    #[test]
    fn test_handler_return_sugar() {
        let sys = parse_py(
            "machine:\n    $A {\n        get() {\n            return 42\n        }\n    }"
        );
        let machine = sys.machine.unwrap();
        let stmts = &machine.states[0].handlers[0].body.statements;
        let has_return = stmts.iter().any(|s| matches!(s, Statement::Return(_)));
        assert!(has_return, "Should have return statement");
    }

    #[test]
    fn test_forward_to_parent() {
        let sys = parse_py(
            "machine:\n    $Child {\n        evt() {\n            => $^\n        }\n    }"
        );
        let machine = sys.machine.unwrap();
        let stmts = &machine.states[0].handlers[0].body.statements;
        let has_forward = stmts.iter().any(|s| matches!(s, Statement::Forward(_)));
        assert!(has_forward, "Should have forward to parent");
    }

    #[test]
    fn test_multiple_states() {
        let sys = parse_py(
            "machine:\n    $Idle {\n    }\n    $Running {\n    }\n    $Done {\n    }"
        );
        let machine = sys.machine.unwrap();
        assert_eq!(machine.states.len(), 3);
        assert_eq!(machine.states[0].name, "Idle");
        assert_eq!(machine.states[1].name, "Running");
        assert_eq!(machine.states[2].name, "Done");
    }
}

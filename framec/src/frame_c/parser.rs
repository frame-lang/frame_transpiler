#![allow(clippy::unnecessary_wraps)]

use super::ast::AssignmentExprNode;
use super::ast::DeclOrStmtType;
use super::ast::ExprStmtType::*;
use super::ast::ExprType;
use super::ast::ExprType::*;
use super::ast::MessageType::{AnyMessage, CustomMessage};
use super::ast::TerminatorType::{Continue, Return};
use super::ast::*;
use super::scanner::*;
use super::symbol_table::*;
use crate::frame_c::utils::SystemHierarchy;
use downcast_rs::__std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use crate::frame_c::symbol_table::SymbolType::ParamSymbol;
//use crate::frame_c::ast::LoopStmtTypes::{LoopInfiniteStmt, LoopInStmt};

pub struct ParseError {
    // TODO:
    pub error: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            error: String::from(msg),
        }
    }
}

// TODO
impl fmt::Display for ParseError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "ParseError")
    }
}

// // @todo
// struct StateSemanticValidator {
//
//     // exitEventHandlerOpt:Option<Rc<RefCell<EventHandlerNode>>>,
//     // transtitions:Vec<Rc<RefCell<TransitionStatementNode>>>,
// }
//
// impl StateSemanticValidator {
//
//     pub fn new() -> StateSemanticValidator {
//         StateSemanticValidator {}
//     }
//
//     pub fn has_valid_exit_semantics(&self, _:&StateNode) -> bool {
//         // if any transition has exit args then
//         //  - there must be an exit handler for the state
//         //  - all exit args must be of same number and type for all transitions
//         //  - all transition exit args must match the exit handler parameter list
//
//         // for evt_handler in state_node.evt_handlers {
//         //
//         //     for statement in evt_handler.statements {
//         //         match statement {
//         //             DeclOrStmtType::StmtT {stmt_t} => {
//         //                 match stmt_t {
//         //                     StatementType::TransitionStmt{transition_statement}
//         //                         => {
//         //                         transition_statement.
//         //                     }
//         //                 }
//         //             },
//         //             _ => {},
//         //         }
//         //     }
//         // }
//
//         true
//     }
// }

pub struct Parser<'a> {
    tokens: &'a [Token],
    comments: &'a mut Vec<Token>,
    current: usize,
    current_token: String,
    current_tok_ref: &'a Token,
    current_event_symbol_opt: Option<Rc<RefCell<EventSymbol>>>,
    processed_tokens: String,
    //    reset_pos:usize,
    is_building_symbol_table: bool,
    arcanum: Arcanum,
    state_name_opt: Option<String>,
    had_error: bool,
    panic_mode: bool,
    errors: String,
    last_sync_token_idx: usize,
    system_hierarchy_opt: Option<SystemHierarchy>,
    is_parsing_rhs: bool,
    event_handler_has_transition: bool,
    is_action_context:bool,
    is_loop_context:bool,
    loop_stmt_idx:i32,
    interface_method_called:bool,
    pub generate_enter_args: bool,
    pub generate_exit_args: bool,
    pub generate_state_context: bool,
    pub generate_state_stack: bool,
    pub generate_change_state: bool,
    pub generate_transition_state: bool,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        tokens: &'a [Token],
        comments: &'a mut Vec<Token>,
        is_building_symbol_table: bool,
        arcanum: Arcanum,
    ) -> Parser<'a> {
        Parser {
            tokens,
            comments,
            current: 0,
            last_sync_token_idx: 0,
            current_token: String::from(""),
            current_event_symbol_opt: None,
            processed_tokens: String::from(""),
            is_building_symbol_table,
            arcanum,
            state_name_opt: None,
            had_error: false,
            panic_mode: false,
            errors: String::new(),
            current_tok_ref: &tokens[0],
            system_hierarchy_opt: None,
            is_parsing_rhs: false,
            event_handler_has_transition: false,
            generate_enter_args: false,
            generate_exit_args: false,
            generate_state_context: false,
            generate_state_stack: false,
            generate_change_state: false,
            generate_transition_state: false,
            is_action_context: false,
            is_loop_context: false,
            loop_stmt_idx: 0,
            interface_method_called: false,
        }
    }

    /* --------------------------------------------------------------------- */

    pub fn parse(&mut self) -> SystemNode {
        self.system()
    }

    /* --------------------------------------------------------------------- */

    pub fn get_arcanum(self) -> Arcanum {
        self.arcanum
    }

    /* --------------------------------------------------------------------- */

    pub fn get_system_hierarchy(self) -> SystemHierarchy {
        self.system_hierarchy_opt.unwrap()
    }

    /* --------------------------------------------------------------------- */

    pub fn get_all(self) -> (Arcanum, SystemHierarchy) {
        (self.arcanum, self.system_hierarchy_opt.unwrap())
    }

    /* --------------------------------------------------------------------- */

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    /* --------------------------------------------------------------------- */

    pub fn get_errors(&self) -> String {
        self.errors.clone()
    }

    /* --------------------------------------------------------------------- */

    // Helper functions

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        // cache off comments
        while self.check(TokenType::SingleLineComment) || self.check(TokenType::MultiLineComment) {
            self.comments.push(self.peek().clone());
            self.advance();
        }

        if self.check(TokenType::Error) {
            self.error_at_current("Unexpected token.");
            self.advance();
            return false;
        }

        for token_type in token_types {
            if self.check(*token_type) {
                //              println!("Consumed {:?}",token_type);
                if !self.is_at_end() {
                    self.advance();
                }

                return true;
            }
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
            self.current_tok_ref = &self.tokens[self.current];
            self.current_token = self.peek().lexeme.clone();
            self.processed_tokens.push(' ');
            self.processed_tokens.push_str(&self.peek().lexeme.clone());
            //            println!("Current token = {:?}",self.peek());
        }

        self.previous()
    }

    /* --------------------------------------------------------------------- */

    fn check(&self, token_type: TokenType) -> bool {
        let t = self.peek();
        if token_type == t.token_type {
            return true;
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    /* --------------------------------------------------------------------- */

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /* --------------------------------------------------------------------- */

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /* --------------------------------------------------------------------- */

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        self.error_at_current(message);
        Err(ParseError::new(message))
    }

    /* --------------------------------------------------------------------- */

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.tokens[self.current], message);
    }

    /* --------------------------------------------------------------------- */

    fn error_at_previous(&mut self, message: &str) {
        self.error_at(&self.tokens[self.current - 1], message);
    }

    /* --------------------------------------------------------------------- */

    // TODO: put the message in the ParseError
    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        self.had_error = true;

        let mut error_msg = format!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => error_msg.push_str(" at end"),
            TokenType::Error => error_msg.push_str(&format!(" at '{}'", token.lexeme)),
            _ => error_msg.push_str(&format!(" at '{}'", token.lexeme)),
        }

        self.errors
            .push_str(&format!("{} : {}\n", error_msg, message));

        //        println!("{} : {}", error_msg, message);
        // TODO:?
        //       ParseError::new( /* error_msg */ )
    }

    /* --------------------------------------------------------------------- */

    fn synchronize(&mut self, sync_tokens: &[TokenType]) -> bool {
        self.panic_mode = false;

        if self.is_at_end() {
            return false;
        }

        // in case not advancing
        if self.last_sync_token_idx == self.current {
            self.advance();
        }

        self.last_sync_token_idx = self.current;

        while self.peek().token_type != TokenType::Eof {
            for sync_token in sync_tokens {
                if *sync_token == self.peek().token_type {
                    return true;
                }
            }

            self.advance();
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn follows(&self, token: &Token, follows_vec: &[TokenType]) -> bool {
        for follows_token_type in follows_vec {
            if *follows_token_type == token.token_type {
                return true;
            }
        }

        let vec_comments = &vec![TokenType::SingleLineComment, TokenType::MultiLineComment];
        for comment_token_type in vec_comments {
            if *comment_token_type == token.token_type {
                return true;
            }
        }

        false
    }

    /* --------------------------------------------------------------------- */

    fn system(&mut self) -> SystemNode {
        let mut header = String::new();
        let mut interface_block_node_opt = Option::None;
        let mut machine_block_node_opt = Option::None;
        let mut actions_block_node_opt = Option::None;
        let mut domain_block_node_opt = Option::None;

        if self.match_token(&[TokenType::Eof]) {
            self.error_at_current("Empty system.");
            return SystemNode::new(
                String::from("error"),
                header,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                0,
            );
        }

        // Parse free-form header ```whatever```
        if self.match_token(&[TokenType::ThreeTicks]) {
            while self.match_token(&[TokenType::SuperString]) {
                let tok = self.previous();
                header.push_str(&*tok.lexeme.clone());
            }
            if self
                .consume(TokenType::ThreeTicks, "Expected '```'.")
                .is_err()
            {
                self.error_at_current("Expected closing ```.");
                let sync_tokens = &vec![TokenType::System];
                self.synchronize(sync_tokens);
            }
        }

        let attributes_opt = match self.attributes() {
            Ok(attributes_opt) => attributes_opt,
            Err(_parse_error) => None,
        };

        // TODO: Error handling
        if !self.match_token(&[TokenType::System]) {
            self.error_at_current("Expected #.");
            let sync_tokens = &vec![TokenType::Identifier];
            self.synchronize(sync_tokens);
        }
        if !self.match_token(&[TokenType::Identifier]) {
            self.error_at_current("Expected system identifer.");
            let sync_tokens = &vec![
                TokenType::InterfaceBlock,
                TokenType::MachineBlock,
                TokenType::ActionsBlock,
                TokenType::DomainBlock,
                TokenType::SystemEnd,
            ];
            self.synchronize(sync_tokens);
        }

        let id = self.previous();
        let system_name = id.lexeme.clone();

        self.system_hierarchy_opt = Some(SystemHierarchy::new(system_name.clone()));

        if self.is_building_symbol_table {
            //           let st = self.get_current_symtab();
            let system_symbol = SystemSymbol::new(system_name.clone());
            let system_symbol_rcref = Rc::new(RefCell::new(system_symbol));
            // TODO: it would be better to find some way to bake the identifier scope into the SystemScope type
            self.arcanum.enter_scope(ParseScopeType::System {
                system_symbol: system_symbol_rcref,
            });
        } else {
            self.arcanum.set_parse_scope(&system_name);
        }

        // Parse optional system params.
        // #SystemName $[start_state_param:T] >[start_state_enter_param:U] #[domain_params:V]

        let mut system_start_state_state_params_opt: Option<Vec<ParameterNode>> = Option::None;

        if self.match_token(&[TokenType::State]) {
            if self.consume(TokenType::LBracket, "Expected '['").is_err() {
                let sync_tokens = &vec![
                    TokenType::GT,
                    TokenType::System,
                    TokenType::InterfaceBlock,
                    TokenType::ActionsBlock,
                    TokenType::MachineBlock,
                    TokenType::DomainBlock,
                    TokenType::SystemEnd,
                ];
                self.synchronize(sync_tokens);
            }
            match self.parameters() {
                Ok(Some(parameters)) => system_start_state_state_params_opt = Some(parameters),
                Ok(None) => {}
                Err(_) => {}
            }
        }

        let mut system_enter_params_opt: Option<Vec<ParameterNode>> = Option::None;

        if self.match_token(&[TokenType::GT]) {
            if self.consume(TokenType::LBracket, "Expected '['").is_err() {
                let sync_tokens = &vec![
                    TokenType::System,
                    TokenType::InterfaceBlock,
                    TokenType::ActionsBlock,
                    TokenType::MachineBlock,
                    TokenType::DomainBlock,
                    TokenType::SystemEnd,
                ];
                self.synchronize(sync_tokens);
            }
            match self.parameters() {
                Ok(Some(parameters)) => system_enter_params_opt = Some(parameters),
                Ok(None) => {}
                Err(_) => {}
            }
        }

        let mut domain_params_opt: Option<Vec<ParameterNode>> = Option::None;

        if self.match_token(&[TokenType::LBracket]) {
            match self.parameters() {
                Ok(Some(parameters)) => {
                    if !self.is_building_symbol_table {
                        // check system domain params override a domain variable and match type
                        for param in &parameters {
                            let name = &param.param_name;
                            let domain_symbol_rcref_opt =
                                self.arcanum.lookup(name, &IdentifierDeclScope::DomainBlock);
                            if domain_symbol_rcref_opt.is_none() {
                                self.error_at_current(&format!(
                                    "System domain parameter '{}' does not exist in the domain.",
                                    name
                                ));
                                let sync_tokens = &vec![
                                    TokenType::InterfaceBlock,
                                    TokenType::MachineBlock,
                                    TokenType::ActionsBlock,
                                    TokenType::DomainBlock,
                                    TokenType::SystemEnd,
                                ];
                                self.synchronize(sync_tokens);
                            } else {
                                // domain var exists, check type matches
                                let symbol_type_rcref = domain_symbol_rcref_opt.unwrap();
                                let symbol_type = symbol_type_rcref.borrow();
                                match &*symbol_type {
                                    SymbolType::DomainVariable {
                                        domain_variable_symbol_rcref,
                                    } => {
                                        let domain_variable_symbol =
                                            domain_variable_symbol_rcref.borrow();
                                        let domain_variable_symbol_type_node_opt =
                                            &domain_variable_symbol.var_type;
                                        let param_type_node_opt = &param.param_type_opt;
                                        if domain_variable_symbol_type_node_opt.is_none()
                                            && param_type_node_opt.is_none()
                                        {
                                            // ok
                                        } else if domain_variable_symbol_type_node_opt.is_some()
                                            && param_type_node_opt.is_some()
                                        {
                                            // maybe ok, check types match
                                            let domain_variable_type_node =
                                                domain_variable_symbol_type_node_opt
                                                    .as_ref()
                                                    .unwrap();
                                            let param_type_node =
                                                param_type_node_opt.as_ref().unwrap();
                                            if domain_variable_type_node
                                                .get_type_str()
                                                .ne(&param_type_node.get_type_str())
                                            {
                                                // error - one has a type and the other does not.
                                                self.error_at_current(&format!("System domain parameter '{}' type does not match domain variable type.",name));
                                                let sync_tokens = &vec![
                                                    TokenType::InterfaceBlock,
                                                    TokenType::MachineBlock,
                                                    TokenType::ActionsBlock,
                                                    TokenType::DomainBlock,
                                                    TokenType::SystemEnd,
                                                ];
                                                self.synchronize(sync_tokens);
                                            }
                                        } else {
                                            // error - one has a type and the other does not.
                                            self.error_at_current(&format!("System domain parameter '{}' type does not match domain variable type.",name));
                                            let sync_tokens = &vec![
                                                TokenType::InterfaceBlock,
                                                TokenType::MachineBlock,
                                                TokenType::ActionsBlock,
                                                TokenType::DomainBlock,
                                                TokenType::SystemEnd,
                                            ];
                                            self.synchronize(sync_tokens);
                                        }
                                    }
                                    _ => {
                                        self.error_at_current(&format!(
                                            "Compiler error - wrong type found for '{}'.",
                                            name
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    domain_params_opt = Some(parameters)
                }
                Ok(None) => {}
                Err(_) => {}
            }
        }

        if self.match_token(&[TokenType::InterfaceBlock]) {
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let x = self.interface_block();
            interface_block_node_opt = Option::Some(x);
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
        }

        if self.match_token(&[TokenType::MachineBlock]) {
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            machine_block_node_opt = Option::Some(self.machine_block());
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
        }

        if !self.is_building_symbol_table {
            // validate system start state params
            if let Some(machine_block_node) = machine_block_node_opt.as_ref() {
                if machine_block_node.states.is_empty() {
                    if system_start_state_state_params_opt.is_none() {
                        // ok - no states or start state params
                    } else {
                        // error - no start state but start state params exist
                        self.error_at_current(
                            "System start state parameters declared but no start state exists.",
                        );
                    }

                    if system_enter_params_opt.is_none() {
                        // ok - no states or enter event params
                    } else {
                        // error - no start state but enter event params exist
                        self.error_at_current("System start state enter parameters declared but no start state exists.");
                    }
                } else {
                    // there are states
                    let start_state_rcref_opt = machine_block_node.states.get(0);
                    if let Some(start_state_rcref) = start_state_rcref_opt {
                        let start_state = start_state_rcref.borrow();

                        if start_state.params_opt.is_none()
                            && system_start_state_state_params_opt.is_none()
                        {
                            // ok
                        } else if start_state.params_opt.is_some()
                            && system_start_state_state_params_opt.is_none()
                        {
                            // error - mismatched params
                            self.error_at_current("Start state parameters declared but no system start state parameters are declared.");
                        } else if start_state.params_opt.is_none()
                            && system_start_state_state_params_opt.is_some()
                        {
                            self.error_at_current(
                                "System start state parameters declared but no start state exists.",
                            );
                        } else {
                            // both state and system have params. verify they match
                            let system_start_state_state_params =
                                system_start_state_state_params_opt.as_ref().unwrap();
                            let start_state_params_vec = start_state.params_opt.as_ref().unwrap();
                            //  if let Some(start_state_params_vec) = &start_state.params_opt {
                            if start_state_params_vec.len() != system_start_state_state_params.len()
                            {
                                // error
                                self.error_at_current("System start state parameters do not match actual start state parameters.");
                            } else {
                                // loop through parameter lists and confirm identical
                                // let mut i = 0;
                                for (i, state_param) in start_state_params_vec.iter().enumerate() {
                                    let system_start_state_state_param =
                                        system_start_state_state_params.get(i).unwrap();
                                    if system_start_state_state_param != state_param {
                                        // error
                                        self.error_at_current("System start state parameters do not match actual start state parameters.");
                                    }
                                    // i += 1;
                                }
                            }
                        }

                        // validate start state enter params.
                        // start state and system enter params must be identical.

                        if let Some(enter_event_handler) = &start_state.enter_event_handler_opt {
                            let y = enter_event_handler.as_ref().borrow();
                            let z = &y.event_symbol_rcref;
                            let a = z.borrow();
                            let enter_event_handler_params_opt = &a.params_opt;
                            if enter_event_handler_params_opt.is_none() {
                                if system_enter_params_opt.is_none() {
                                    // ok
                                } else {
                                    // error
                                    self.error_at_current("System has enter parameters but start state enter handler does not.");
                                }
                            } else {
                                // enter_event_handler_params_opt.is_some()

                                if system_enter_params_opt.is_none() {
                                    // error
                                    self.error_at_current("Start state has enter parameters but system does not define any.");
                                } else {
                                    // system_enter_params_opt.is_some()
                                    // compare system enter params w/ start state enter params
                                    let system_enter_params =
                                        system_enter_params_opt.as_ref().unwrap();
                                    let enter_event_handler_params =
                                        &enter_event_handler_params_opt.as_ref().unwrap();
                                    if system_enter_params.len() != enter_event_handler_params.len()
                                    {
                                        // error
                                        self.error_at_current("Start state and system enter parameters are different.");
                                    } else {
                                        // let mut i = 0;
                                        for (i, param) in system_enter_params.iter().enumerate() {
                                            let parameter_symbol =
                                                enter_event_handler_params.get(i).unwrap();
                                            if parameter_symbol.name.ne(&param.param_name) {
                                                // error
                                                self.error_at_current("Start state and system enter parameters are different.");
                                            } else if parameter_symbol.param_type_opt.is_none()
                                                && param.param_type_opt.is_none()
                                            {
                                                // ok
                                            } else if (parameter_symbol.param_type_opt.is_none()
                                                && param.param_type_opt.is_some())
                                                || (parameter_symbol.param_type_opt.is_some()
                                                    && param.param_type_opt.is_none())
                                            {
                                                // error
                                                self.error_at_current("Start state and system enter parameters are different.");
                                            } else {
                                                // parameter_symbol.param_type_opt.is_some() && param.param_type_opt.is_some()
                                                let param_symbol_type = parameter_symbol
                                                    .param_type_opt
                                                    .as_ref()
                                                    .unwrap();
                                                let param_type =
                                                    param.param_type_opt.as_ref().unwrap();
                                                if param_symbol_type != param_type {
                                                    // error
                                                    self.error_at_current("System enter params do not match start state enter params.");
                                                }
                                            }
                                            // i = i + 1;
                                        }
                                    }
                                }
                            }
                        } else if system_enter_params_opt.is_some() {
                            // error - no event handlers but there are system enter event params
                            self.error_at_current("System has enter parameters but the start state does not have an enter event handler.");
                        } else {
                            // ok - no system enter event params
                        }
                    }
                }
            } else {
                // no machine block therefore no states therefore no start state
                if system_start_state_state_params_opt.is_some() {
                    // error - system start state params specified but no machine block
                    self.error_at_current(
                        "System start state parameters declared but no start state exists.",
                    );
                }
            }
        }

        if self.match_token(&[TokenType::ActionsBlock]) {
            actions_block_node_opt = Option::Some(self.actions_block());
        }

        if self.match_token(&[TokenType::DomainBlock]) {
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            domain_block_node_opt = Option::Some(self.domain_block());
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
        }

        if !self.match_token(&[TokenType::SystemEnd]) {
            self.error_at_current("Expected ##.");
        }

        let line = self.previous().line;

        self.arcanum.exit_parse_scope();

        SystemNode::new(
            system_name,
            header,
            attributes_opt,
            system_start_state_state_params_opt,
            system_enter_params_opt,
            domain_params_opt,
            interface_block_node_opt,
            machine_block_node_opt,
            actions_block_node_opt,
            domain_block_node_opt,
            line,
        )
    }

    /* --------------------------------------------------------------------- */

    fn attributes(&mut self) -> Result<Option<HashMap<String, AttributeNode>>, ParseError> {
        let mut attributes: HashMap<String, AttributeNode> = HashMap::new();

        loop {
            if self.match_token(&[TokenType::InnerAttribute]) {
                // not supported yet
                let parse_error = ParseError::new(
                    "Found '#![' token - inner attribute syntax not currently supported.",
                );
                return Err(parse_error);
            } else if self.match_token(&[TokenType::OuterAttribute]) {
                let attribute_node = match self.attribute() {
                    Ok(attribute_node) => attribute_node,
                    Err(err) => {
                        return Err(err);
                    }
                };
                attributes.insert(attribute_node.get_name(), attribute_node);
                if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                    return Err(parse_error);
                }
            } else {
                break;
            }
        }

        if attributes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(attributes))
        }
    }

    /* --------------------------------------------------------------------- */

    fn attribute(&mut self) -> Result<AttributeNode, ParseError> {
        // attribute name: identifier (identifier | : | .)*
        let mut name;
        if self.match_token(&[TokenType::Identifier]) {
            name = self.previous().lexeme.clone();
        } else {
            self.error_at_current("Expected attribute name.");
            let parse_error = ParseError::new("TODO");
            return Err(parse_error);
        }
        while self.match_token(&[TokenType::Identifier, TokenType::Colon, TokenType::Dot]) {
            name.push_str(&self.previous().lexeme.clone());
        }

        if self.match_token(&[TokenType::LParen]) {
            // MetaListIdents
            match self.meta_list_idents() {
                Ok(idents) => {
                    let attrib_idents = AttributeMetaListIdents::new(name, idents);
                    return Ok(AttributeNode::MetaListIdents {
                        attr: attrib_idents,
                    });
                }
                Err(err) => return Err(err),
            }
        } else if let Err(err) = self.consume(TokenType::Equals, "Expected '='") {
            // equals
            return Err(err);
        }

        // attribute value: string
        let value;
        if self.match_token(&[TokenType::String]) {
            value = self.previous().lexeme.clone();
        } else {
            self.error_at_current("Expected attribute value.");
            let parse_error = ParseError::new("TODO");
            return Err(parse_error);
        }
        let attr_namevalue = AttributeMetaNameValueStr::new(name, value);
        Ok(AttributeNode::MetaNameValueStr {
            attr: attr_namevalue,
        })
    }

    /* --------------------------------------------------------------------- */

    //  ( ',' Name )* ')'

    fn meta_list_idents(&mut self) -> Result<Vec<String>, ParseError> {
        let mut idents: Vec<String> = Vec::new();
        loop {
            if !self.match_token(&[TokenType::Identifier]) {
                break;
            }
            let ident = self.previous().lexeme.clone();
            idents.push(ident);
            if !self.match_token(&[TokenType::Colon]) {
                break;
            }
        }

        if let Err(err) = self.consume(TokenType::RParen, "Expected ')'") {
            // equals
            return Err(err);
        }

        Ok(idents)
    }

    /* --------------------------------------------------------------------- */

    fn interface_block(&mut self) -> InterfaceBlockNode {
        if self.is_building_symbol_table {
            let interface_symbol = Rc::new(RefCell::new(InterfaceBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::InterfaceBlock {
                interface_block_scope_symbol_rcref: interface_symbol,
            });
        } else {
            self.arcanum
                .set_parse_scope(InterfaceBlockScopeSymbol::scope_name());
        }

        let x = &self.arcanum.current_symtab;
        self.arcanum.debug_print_current_symbols(x.clone());

        let mut interface_methods = Vec::new();

        // NOTE: this loop peeks() ahead and then interface_method() consumes
        // the identifier. Not sure if this is the best way.

        while self.match_token(&[TokenType::Identifier]) {
            match self.interface_method() {
                Ok(interface_method_node) => {
                    interface_methods.push(interface_method_node);
                }
                Err(_parse_error) => {
                    let sync_tokens = &vec![
                        TokenType::Identifier,
                        TokenType::MachineBlock,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                        TokenType::SystemEnd,
                    ];
                    self.synchronize(sync_tokens);
                }
            }
        }

        let y = &self.arcanum.current_symtab;
        self.arcanum.debug_print_current_symbols(y.clone());

        self.arcanum.exit_parse_scope();

        InterfaceBlockNode::new(interface_methods)
    }

    /* --------------------------------------------------------------------- */

    // interface_method -> identifier ('[' parameters ']')? (':' return_type)?

    fn interface_method(&mut self) -> Result<Rc<RefCell<InterfaceMethodNode>>, ParseError> {
        let name = self.previous().lexeme.clone();

        let mut params_opt: Option<Vec<ParameterNode>> = Option::None;
        let mut return_type_opt: Option<TypeNode> = Option::None;
        let mut alias_opt: Option<MessageNode> = Option::None;

        if self.match_token(&[TokenType::LBracket]) {
            match self.parameters() {
                Ok(Some(parameters)) => params_opt = Some(parameters),
                Ok(None) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
            }
        }

        // Parse return type
        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => return_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        // Parse alias
        if self.match_token(&[TokenType::At]) {
            if self.consume(TokenType::LParen, "Expected '('").is_err() {
                self.error_at_current("Expected '('.");
                let sync_tokens = &vec![TokenType::Pipe];
                self.synchronize(sync_tokens);
            }

            match self.message() {
                Ok(MessageType::CustomMessage { message_node }) => alias_opt = Some(message_node),
                Ok(AnyMessage { .. }) => {
                    self.error_at_previous("Expected message, found '||*");
                    let sync_tokens = &vec![
                        TokenType::RParen,
                        TokenType::MachineBlock,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                        TokenType::SystemEnd,
                    ];
                    self.synchronize(sync_tokens);
                }
                Err(err) => return Err(err),
            }

            if self.consume(TokenType::RParen, "Expected ')'").is_err() {
                let sync_tokens = &vec![
                    TokenType::Identifier,
                    TokenType::MachineBlock,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::SystemEnd,
                ];
                self.synchronize(sync_tokens);
            }
        }

        let mut param_symbols_opt = None;
        match &params_opt {
            Some(param_nodes) => {
                let mut vec = Vec::new();
                for param_node in param_nodes {
                    let param_symbol = ParameterSymbol::new(
                        param_node.param_name.clone(),
                        param_node.param_type_opt.clone(),
                        IdentifierDeclScope::None,
                    );
                    vec.push(param_symbol);
                }
                param_symbols_opt = Some(vec);
            }
            None => {}
        }

        // if the alias exists, that is the name of the event message.
        // if not, the interface method name becomes the event message name.

        let msg = match &alias_opt {
            Some(alias) => alias.name.clone(),
            None => name.clone(),
        };

        // get or create the event symbol for the message we found
        let event_symbol_rcref;
        match self.arcanum.get_event(&*msg, &self.state_name_opt) {
            Some(_existing_event_symbol_rc_ref) => {
                // found message
                // event_symbol_rcref = existing_event_symbol_rc_ref.clone();
            }
            None => {
                let event_symbol = EventSymbol::new(
                    &self.arcanum.symbol_config,
                    &msg,
                    Some(name.clone()),
                    param_symbols_opt,
                    return_type_opt.clone(),
                    self.state_name_opt.clone(),
                );
                event_symbol_rcref = Rc::new(RefCell::new(event_symbol));
                self.arcanum.declare_event(Rc::clone(&event_symbol_rcref));

                // This is the first time we are seeing this event.
                // Set flag so parameters and return type are added to event symbol
                // during this parse.
                //               is_declaring_event = true;
            }
        }

        let interface_method_node =
            InterfaceMethodNode::new(name.clone(), params_opt, return_type_opt, alias_opt);
        let interface_method_rcref = Rc::new(RefCell::new(interface_method_node));

        if self.is_building_symbol_table {
            let mut interface_method_symbol = InterfaceMethodSymbol::new(name);
            // TODO: note what is being done. We are linking to the AST node generated in the syntax pass.
            // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
            // we could copy this information out of the node and into the symbol.
            interface_method_symbol.set_ast_node(Rc::clone(&interface_method_rcref));
            let interface_method_symbol_rcref = Rc::new(RefCell::new(interface_method_symbol));
            let interface_method_symbol_t = SymbolType::InterfaceMethod {
                interface_method_symbol_rcref,
            };
            // TODO: just insert into arcanum directly
           let ret = self.arcanum
                .current_symtab
                .borrow_mut()
                .insert_symbol(&interface_method_symbol_t);
            match ret {
                Ok(()) => {}
                Err(err_msg) => {
                    self.error_at_previous(err_msg.as_str());
                    return Err(ParseError::new(err_msg.as_str()))
                }
            }
        } else {
            // TODO? - link action symbol to action declaration node
        }

        Ok(interface_method_rcref)
    }

    /* --------------------------------------------------------------------- */

    fn type_decl(&mut self) -> Result<TypeNode, ParseError> {
        let mut is_reference = false;

        if self.match_token(&[TokenType::SuperString]) {
            let id = self.previous();
            let type_str = id.lexeme.clone();
            Ok(TypeNode::new(true, false, None, type_str))
        } else {
            if self.match_token(&[TokenType::And]) {
                is_reference = true
            }
            let mut frame_event_part_opt = None;
            if self.match_token(&[TokenType::At]) {
                frame_event_part_opt = Some(FrameEventPart::Event { is_reference })
            } else if !self.match_token(&[TokenType::Identifier]) {
                self.error_at_current("Expected return type name.");
                return Err(ParseError::new("TODO"));
            }

            let id = self.previous();
            let type_str = id.lexeme.clone();

            Ok(TypeNode::new(
                false,
                is_reference,
                frame_event_part_opt,
                type_str,
            ))
        }
    }

    /* --------------------------------------------------------------------- */

    // message => '|' ( identifier | string | '>' | '>>' | '>>>' | '<' | '<<' | '<<<' ) '|'

    fn message(&mut self) -> Result<MessageType, ParseError> {
        let message_node;

        if self.peek().token_type == TokenType::At {
            if let Err(parse_error) = self.consume(TokenType::At, "Expected '@'.") {
                return Err(parse_error);
            }
        }
        if self.match_token(&[TokenType::AnyMessage]) {
            let tok = self.previous();

            return Ok(MessageType::AnyMessage { line: tok.line });
        }
        if !self.match_token(&[TokenType::Pipe]) {
            self.error_at_previous("Expected '|'.");
            return Err(ParseError::new("TODO"));
        }

        let tt = self.peek().token_type;
        match tt {
            TokenType::Identifier
            | TokenType::String
            | TokenType::GT
            | TokenType::GTx2
            | TokenType::GTx3
            | TokenType::LT
            | TokenType::LTx2
            | TokenType::LTx3 => message_node = self.create_message_node(tt),
            _ => {
                self.error_at_current("Expected '|'");
                return Err(ParseError::new("TODO"));
            }
        }

        if let Err(parse_error) = self.consume(TokenType::Pipe, "Expected '|'.") {
            return Err(parse_error);
        }

        Ok(MessageType::CustomMessage { message_node })
    }

    /* --------------------------------------------------------------------- */

    fn create_message_node(&mut self, token_type: TokenType) -> MessageNode {
        self.match_token(&[token_type]);
        let id = self.previous();
        let name = id.lexeme.clone();

        MessageNode::new(name, id.line)
    }


    /* --------------------------------------------------------------------- */

    // TODO- see if all parameter lists can use a common parsing function and AST/symbol data and logic.
    // This is currently to implement scope for parameters for actions but should
    // be expanded to other parameter types if possible.

    fn parameters_scope(&mut self) -> Result<Option<Vec<ParameterNode>>, ParseError> {
        self.is_loop_context = true;
        if self.is_building_symbol_table {
            let params_scope_symbol_rcref = Rc::new(RefCell::new(ParamsScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::Params {
                params_scope_symbol_rcref
            });
        } else {
            // give each loop in a scope a unique name
            let scope_name = &format!("{}",ParamsScopeSymbol::scope_name());
            self.arcanum
                .set_parse_scope(ParamsScopeSymbol::scope_name());
        }

        let ret = self.parameters2();

        self.arcanum.exit_parse_scope();

        ret
    }


    /* --------------------------------------------------------------------- */

    // TODO - unify parameters() and parameters2(). parameters2() is called by
    // methods that manage the scope and expect parameters2() to insert parameter symbols.

    // TODO: consider removing ParseError as it is currently not returned.
    fn parameters2(&mut self) -> Result<Option<Vec<ParameterNode>>, ParseError> {
        let mut parameters: Vec<ParameterNode> = Vec::new();

        loop {
            match self.parameter() {
                Ok(parameter_opt) => match parameter_opt {
                    Some(parameter_node) => {
                        let param_name = &parameter_node.param_name.clone();
                        let mut param_type_opt: Option<TypeNode> = None;
                        if parameter_node.param_type_opt.is_some() {
                            let pt =
                                &parameter_node.param_type_opt.as_ref().unwrap().clone();
                            param_type_opt = Some(pt.clone());
                        }
                        let scope = self.arcanum.get_current_identifier_scope();
                        let param_symbol = ParameterSymbol::new(
                            param_name.clone(),
                            param_type_opt.clone(),
                            scope,
                        );
                        let param_symbol_rcref =
                            Rc::new(RefCell::new(param_symbol));
                        let param_symbol_enum = SymbolType::ParamSymbol {param_symbol_rcref};
                       //  let params_scope = ParseScopeType::Params {params_scope_symbol_rcref};

                        if self.is_building_symbol_table {
                            let ret = self.arcanum.insert_symbol(param_symbol_enum);
                            match ret {
                                Ok(()) => {}
                                Err(err_msg) => {
                                    self.error_at_previous(err_msg.as_str());
                                    return Err(ParseError::new(err_msg.as_str()))
                                }
                            }
                        } else {

                        }

                        parameters.push(parameter_node);
                    }
                    None => {
                        break;
                    }
                },
                Err(_parse_error) => {
                    let sync_tokens = &vec![
                        TokenType::Identifier,
                        TokenType::Colon,
                        TokenType::RBracket,
                        TokenType::MachineBlock,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                        TokenType::SystemEnd,
                    ];
                    self.synchronize(sync_tokens);
                    if !self.follows(
                        self.peek(),
                        &[TokenType::Identifier, TokenType::Colon, TokenType::RBracket],
                    ) {
                        break;
                    }
                }
            }
            if self.match_token(&[TokenType::RBracket]) {
                break;
            } else if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {

                return Err(parse_error);
            }
        }

        if !parameters.is_empty() {
            Ok(Some(parameters))
        } else {
            self.error_at_current("Error - empty list declaration.");
            Err(ParseError::new("Error - empty list declaration."))
        }
    }

    /* --------------------------------------------------------------------- */

    // Just get the parameters here. The calling routine will either build or
    // validate with the EventSymbol.

    // TODO: consider removing ParseError as it is currently not returned.
    fn parameters(&mut self) -> Result<Option<Vec<ParameterNode>>, ParseError> {
        let mut parameters: Vec<ParameterNode> = Vec::new();

        loop {
            match self.parameter() {
                Ok(parameter_opt) => match parameter_opt {
                    Some(parameter_node) => {
                        parameters.push(parameter_node);
                    }
                    None => {
                        break;
                    }
                },
                Err(_parse_error) => {
                    let sync_tokens = &vec![
                        TokenType::Identifier,
                        TokenType::Colon,
                        TokenType::RBracket,
                        TokenType::MachineBlock,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                        TokenType::SystemEnd,
                    ];
                    self.synchronize(sync_tokens);
                    if !self.follows(
                        self.peek(),
                        &[TokenType::Identifier, TokenType::Colon, TokenType::RBracket],
                    ) {
                        break;
                    }
                }
            }
            if self.match_token(&[TokenType::RBracket]) {
                break;
            } else if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {

                return Err(parse_error);
            }
        }

        if !parameters.is_empty() {
            Ok(Some(parameters))
        } else {
            self.error_at_current("Error - empty list declaration.");
            Err(ParseError::new("Error - empty list declaration."))
        }
    }

    /* --------------------------------------------------------------------- */

    // parameter -> param_name ( ':' param_type )?

    fn parameter(&mut self) -> Result<Option<ParameterNode>, ParseError> {
        if !self.match_token(&[TokenType::Identifier]) {
            self.error_at_current("Expected parameter name.");
            return Err(ParseError::new("TODO"));
        }

        let id = self.previous();
        let param_name = id.lexeme.clone();

        let mut param_type_opt: Option<TypeNode> = None;

        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => param_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }

            // let id = self.previous();
            // let param_type = id.lexeme.clone();

            //param_type_opt = Some(param_type);
        }

        let scope = self.arcanum.get_current_identifier_scope();
        let param_node = ParameterNode::new(param_name, param_type_opt, scope);

        if self.is_building_symbol_table {

        }
        Ok(Some(param_node))
    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn machine_block(&mut self) -> MachineBlockNode {
        if self.is_building_symbol_table {
            let machine_symbol = Rc::new(RefCell::new(MachineBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::MachineBlock {
                machine_scope_symbol_rcref: machine_symbol,
            });
        } else {
            self.arcanum
                .set_parse_scope(MachineBlockScopeSymbol::scope_name());
        }

        let mut states = Vec::new();

        while self.match_token(&[TokenType::State]) {
            match self.state() {
                Ok(state_rcref) => {
                    states.push(state_rcref);
                }
                Err(_) => {
                    self.error_at_current("Error parsing Machine Block.");
                    let sync_tokens = &vec![TokenType::State];
                    if self.synchronize(sync_tokens) {
                        continue;
                    } else {
                        let sync_tokens = &vec![
                            TokenType::ActionsBlock,
                            TokenType::DomainBlock,
                            TokenType::SystemEnd,
                        ];
                        self.synchronize(sync_tokens);
                        break;
                    }
                }
            }
        }

        self.arcanum.exit_parse_scope();

        MachineBlockNode::new(states)
    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn actions_block(&mut self) -> ActionsBlockNode {
        if self.is_building_symbol_table {
            let actions_block_scope_symbol = Rc::new(RefCell::new(ActionsBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::ActionsBlock {
                actions_block_scope_symbol_rcref: actions_block_scope_symbol,
            });
        } else {
            self.arcanum
                .set_parse_scope(ActionsBlockScopeSymbol::scope_name());
        }

        let mut actions = Vec::new();

        while self.match_token(&[TokenType::Identifier]) {
            if let Ok(action_decl_node) = self.action_scope() {
                actions.push(action_decl_node);
            }
        }

        self.arcanum.exit_parse_scope();

        ActionsBlockNode::new(actions)
    }

    /* --------------------------------------------------------------------- */
    //
    // fn action_decl(&mut self) -> Result<Rc<RefCell<ActionNode>>, ParseError> {
    //     let action_name = self.previous().lexeme.clone();
    //
    //     let mut params: Option<Vec<ParameterNode>> = Option::None;
    //
    //     if self.match_token(&[TokenType::LBracket]) {
    //         params = match self.parameters() {
    //             Ok(Some(parameters)) => Some(parameters),
    //             Ok(None) => None,
    //             Err(parse_error) => return Err(parse_error),
    //         }
    //     }
    //
    //     let mut type_opt: Option<TypeNode> = None;
    //
    //     if self.match_token(&[TokenType::Colon]) {
    //         match self.type_decl() {
    //             Ok(type_node) => type_opt = Some(type_node),
    //             Err(parse_error) => return Err(parse_error),
    //         }
    //     }
    //
    //     let mut code_opt: Option<String> = None;
    //
    //     if self.match_token(&[TokenType::OpenBrace]) {
    //         // TODO - figure out how this needes to be added to statements
    //         if self.match_token(&[TokenType::SuperString]) {
    //             let token = self.previous();
    //             code_opt = Some(token.lexeme.clone());
    //         }
    //         if self.is_building_symbol_table {
    //             let event_handler_symbol =
    //                 EventHandlerScopeSymbol::new(&msg, Rc::clone(&event_symbol_rcref));
    //             let event_handler_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_symbol));
    //
    //             self.arcanum.enter_scope(ParseScopeType::Action {
    //                 event_handler_scope_symbol_rcref,
    //             });
    //         } else {
    //             self.arcanum.set_parse_scope(&msg);
    //         }
    //
    //         if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
    //             return Err(parse_error);
    //         }
    //     }
    //
    //     let action_decl_node = ActionNode::new(action_name.clone(), params, type_opt, code_opt);
    //
    //     if self.is_building_symbol_table {
    //         let s = action_name;
    //         let mut action_decl_symbol = ActionDeclSymbol::new(s);
    //         // TODO: note what is being done. We are linking to the AST node generated in the syntax pass.
    //         // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
    //         // we could copy this information out of the node and into the symbol.
    //         // TODO: just insert into arcanum directly
    //         self.arcanum
    //             .current_symtab
    //             .borrow_mut()
    //             .insert_symbol(&action_decl_symbol_t);
    //     } else {
    //         let action_decl_rcref = Rc::new(RefCell::new(action_decl_node));
    //
    //         action_decl_symbol.set_ast_node(Rc::clone(&action_decl_rcref));
    //         let action_decl_symbol_rcref = Rc::new(RefCell::new(action_decl_symbol));
    //         let action_decl_symbol_t = SymbolType::ActionScope {
    //             action_decl_symbol_rcref,
    //         };
    //     }
    //
    //     Ok(action_decl_rcref)
    // }


    /* --------------------------------------------------------------------- */

    // This method wraps the call to the action_context() call which does
    // the parsing. Here the scope stack is managed including
    // the scope symbol creation and association with the AST node.

    fn action_scope(&mut self) -> Result<Rc<RefCell<ActionNode>>, ParseError> {

        let action_name = self.previous().lexeme.clone();

        // The 'is_action_context' flag is used to determine which statements are valid
        // to be called in the context of an action. Transitions, for example, are not
        // allowed.
        self.is_action_context = true;

       if self.is_building_symbol_table {
            // syntax pass
            let action_symbol = ActionScopeSymbol::new(action_name.clone());
//            action_symbol_opt = Some(action_symbol);

            let action_scope_symbol_rcref = Rc::new(RefCell::new(action_symbol));
            let action_symbol_parse_scope_t = ParseScopeType::Action {
                action_scope_symbol_rcref
            };
            self.arcanum.enter_scope(action_symbol_parse_scope_t);

        } else {
            // semantic pass
            // link action symbol to action declaration node
            let a = self.arcanum.current_symtab.borrow().lookup(&*action_name, &IdentifierDeclScope::None);
            // see if we can get the action symbol set in the syntax pass. if so, then move
            // all this to the calling function and pass inthe symbol
            self.arcanum
                .set_parse_scope(&action_name);
        }

        let ret = self.action(action_name.clone());

        if self.is_building_symbol_table {
            match &ret {
                Ok(action_node_rcref) => {
                    // associate AST node with symbol
                    let a = action_node_rcref.borrow();
                    let b = self.arcanum.lookup_action(&action_name.clone());
                    let c = b.unwrap();
                    let mut d = c.borrow_mut();
                    d.ast_node_opt = Some(action_node_rcref.clone());

                }
                Err(err)  => {
                    // just return the error
                }
            }
        }

        self.arcanum.exit_parse_scope();

        self.is_action_context = false;

        ret

    }

    /* --------------------------------------------------------------------- */

    fn action(&mut self, action_name:String) -> Result<Rc<RefCell<ActionNode>>, ParseError>  {

        let mut params: Option<Vec<ParameterNode>> = Option::None;

        if self.match_token(&[TokenType::LBracket]) {
            params = match self.parameters_scope() {
                Ok(Some(parameters)) => Some(parameters),
                Ok(None) => None,
                Err(parse_error) => return Err(parse_error),
            }
        }

        let mut type_opt: Option<TypeNode> = None;

        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        let mut code_opt: Option<String> = None;
        let mut statements = Vec::new();
        let mut terminator_node_opt = None;
        let mut is_implemented = false;

        if self.match_token(&[TokenType::OpenBrace]) {
            is_implemented = true;
            // TODO - figure out how this needes to be added to statements
            // if self.match_token(&[TokenType::SuperString]) {
            //     let token = self.previous();
            //     code_opt = Some(token.lexeme.clone());
            // }

            statements = self.statements();

            if self.match_token(&[TokenType::Caret]) {
                if self.match_token(&[TokenType::LParen]) {
                    let expr_t = match self.decorated_unary_expression() {
                        Ok(Some(expr_t)) => expr_t,
                        _ => {
                            self.error_at_current("Expected expression as return value.");
                           //  self.arcanum.exit_parse_scope();
                            return Err(ParseError::new("TODO"));
                        }
                    };

                    if let Err(parse_error) = self.consume(TokenType::RParen, "Expected ')'.") {
                       // self.arcanum.exit_parse_scope();
                        return Err(parse_error);
                    }

                    terminator_node_opt = Some(TerminatorExpr::new(
                        Return,
                        Some(expr_t),
                        self.previous().line,
                    ));
                } else {
                    terminator_node_opt = Some(TerminatorExpr::new(Return, None, self.previous().line));
                }
            }

            if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
             //   self.arcanum.exit_parse_scope();
                return Err(parse_error);
            } else {

            }
        }

        let action_node = ActionNode::new(action_name.clone(), params, is_implemented, statements, terminator_node_opt, type_opt, code_opt);
        // let action_node_rcref = Rc::new(RefCell::new(action_node));
        //
        // if self.is_building_symbol_table {
        //     // syntactic pass.
        //     // Add reference from action symbol to the ActionNode.
        //     // TODO: note what is being done. We are linking to the AST node generated in the
        //     // TODO: **syntax** pass (not the semantic pass).
        //     // The  AST tree built during the syntax pass is otherwise disposed of, but not these
        //     // references squirrled away in the symbol table.
        //     // This may be fine but feels wrong. Alternatively
        //     // we could copy this information out of the node and into the symbol.
        //
        //     let action_node_rcref = Rc::new(RefCell::new(action_node));
        //
        //     action_symbol.set_ast_node(Rc::clone(&action_node_rcref));
        //     let action_symbol_rcref = Rc::new(RefCell::new(action_decl_symbol));
        //     let action_decl_symbol_t = SymbolType::ActionScope {
        //         action_decl_symbol_rcref,
        //     };
        // }


        let x = RefCell::new(action_node);
        let y = Rc::new(x);
        Ok(y)
    }


    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn domain_block(&mut self) -> DomainBlockNode {
        self.arcanum
            .debug_print_current_symbols(self.arcanum.get_current_symtab());
        if self.is_building_symbol_table {
            let domain_symbol = Rc::new(RefCell::new(DomainBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::DomainBlock {
                domain_block_scope_symbol_rcref: domain_symbol,
            });
        } else {
            self.arcanum
                .set_parse_scope(DomainBlockScopeSymbol::scope_name());
        }

        let mut domain_variables = Vec::new();
        let mut enums = Vec::new();

        while self.match_token(&[TokenType::Var, TokenType::Const, TokenType::Enum]) {
            if self.previous().token_type == TokenType::Enum {
                match self.enum_decl() {
                    Ok(enum_decl_node) => {
                        enums.push(enum_decl_node);
                    },
                    Err(_parse_err) => {
                        let sync_tokens = &vec![TokenType::Var, TokenType::Const, TokenType::SystemEnd];
                        self.synchronize(sync_tokens);
                    },
                }
            } else {
                match self.variable_decl(IdentifierDeclScope::DomainBlock) {
                    Ok(domain_variable_node) => domain_variables.push(domain_variable_node),
                    Err(_parse_err) => {
                        let sync_tokens = &vec![TokenType::Var, TokenType::Const, TokenType::SystemEnd];
                        self.synchronize(sync_tokens);
                    }
                }
            }

        }

        self.arcanum
            .debug_print_current_symbols(self.arcanum.get_current_symtab());
        self.arcanum.exit_parse_scope();

        DomainBlockNode::new(domain_variables,enums)
    }


    //* --------------------------------------------------------------------- *//

    // enum Days {
    //     SUNDAY
    //     MONDAY = 2
    //     TUESDAY = 2
    // }

    fn enum_decl(
        &mut self,
    ) -> Result<Rc<RefCell<EnumDeclNode>>, ParseError> {

        let identifier = match self.match_token(&[TokenType::Identifier]) {
            false => {
                self.error_at_current("Expected enum identifier");
                return Err(ParseError::new("TODO"));
            }
            true => self.previous().lexeme.clone(),
        };

        if !self.match_token(&[TokenType::OpenBrace]) {
            self.error_at_current("Expected enum {identifier} '{'.");
            return Err(ParseError::new("TODO"));
        }

        let mut enums = Vec::new();
        let mut enum_value = 0;
        while self.match_token(&[TokenType::Identifier]) {
            let identifier = self.previous().lexeme.clone();
            if self.match_token(&[TokenType::Equals]) {
                if self.match_token(&[TokenType::Number]) {
                    let tok = self.previous();
                    let tok_lit = &tok.literal;
                    if let TokenLiteral::Integer(value) = tok_lit {
                        enum_value = *value;
                    } else {
                        let err_msg = "Expected integer in enum assignment. Found float.";
                        self.error_at_current(err_msg.clone());
                        return Err(ParseError::new(err_msg));
                    }
                } else {
                    let err_msg = "Expected number after '='.";
                    self.error_at_current(err_msg.clone());
                    return Err(ParseError::new(err_msg));
                }
            }
            let enumerator_node = Rc::new(EnumeratorDeclNode::new(identifier, enum_value));
            enums.push(enumerator_node);
            enum_value = enum_value + 1;
        }

        if !self.match_token(&[TokenType::CloseBrace]) {
            self.error_at_current("Expected '}' for enum {identifier}.");
            return Err(ParseError::new("TODO"));
        }

        let enum_decl_node = EnumDeclNode::new(identifier.clone(),enums);
        let enum_decl_node_rcref = Rc::new(RefCell::new(enum_decl_node));

        if self.is_building_symbol_table {
            // syntactic pass
            let scope = self.arcanum.get_current_identifier_scope();
            let mut enum_symbol = EnumSymbol::new(identifier.clone(),scope);

            // TODO: note what is being done. We are linking to the AST node generated in the syntax pass.
            // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
            // we could copy this information out of the node and into the symbol.
            enum_symbol.set_ast_node(Rc::clone(&enum_decl_node_rcref));

            let enum_symbol_rcref = Rc::new(RefCell::new(enum_symbol));
            let enum_symbol_t = SymbolType::EnumDeclSymbolT {
                enum_symbol_rcref
            };
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let ret = self.arcanum
                .current_symtab
                .borrow_mut()
                .insert_symbol(&enum_symbol_t);
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            match ret {
                Ok(()) => {}
                Err(err_msg) => {
                    self.error_at_previous(err_msg.as_str());
                    return Err(ParseError::new(err_msg.as_str()))
                }
            }
        } else {
            // semantic pass

            // TODO
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let x = self.arcanum.lookup(&identifier, &IdentifierDeclScope::None);
            let y = x.unwrap();
            let z = y.borrow();
            match &*z {
                SymbolType::EnumDeclSymbolT {
                    enum_symbol_rcref,
                } => {
                    // assign enum decl node to symbol created in syntactic pass
                    enum_symbol_rcref.borrow_mut().ast_node_opt =
                        Some(enum_decl_node_rcref.clone());
                }
                _ => return Err(ParseError::new("Unrecognized enum scope.")),
            }
        }
        Ok(enum_decl_node_rcref)
    }

    //* --------------------------------------------------------------------- *//

    fn variable_decl(
        &mut self,
        identifier_decl_scope: IdentifierDeclScope,
    ) -> Result<Rc<RefCell<VariableDeclNode>>, ParseError> {
        let is_constant = match self.previous().token_type {
            TokenType::Var => false,
            TokenType::Const => true,
            _ => return Err(ParseError::new("TODO")),
        };

        let name = match self.match_token(&[TokenType::Identifier]) {
            false => {
                self.error_at_current("Expected declaration identifier");
                return Err(ParseError::new("TODO"));
            }
            true => self.previous().lexeme.clone(),
        };

        let mut type_node_opt: Option<TypeNode> = None;

        if self.match_token(&[TokenType::Colon]) {
            match self.type_decl() {
                Ok(type_node) => type_node_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        let mut initializer_expr_t_opt= None;

        if self.match_token(&[TokenType::Equals]) {
            match self.equality() {
                Ok(Some(LiteralExprT { literal_expr_node }))
                => initializer_expr_t_opt = Some(LiteralExprT { literal_expr_node }),
                Ok(Some(VariableExprT { var_node: id_node }))
                => initializer_expr_t_opt = Some(VariableExprT { var_node: id_node }),
                Ok(Some(ActionCallExprT { action_call_expr_node }))
                // TODO this may be dead code. CallChainLiteralExprT may do it all
                => initializer_expr_t_opt = Some(ActionCallExprT { action_call_expr_node }),
                Ok(Some(ExprListT { expr_list_node }))
                => initializer_expr_t_opt = Some(ExprListT { expr_list_node }),
                Ok(Some(CallChainLiteralExprT { call_chain_expr_node }))
                => initializer_expr_t_opt = Some(CallChainLiteralExprT { call_chain_expr_node }),
                Ok(Some(UnaryExprT { unary_expr_node }))
                => initializer_expr_t_opt = Some(UnaryExprT { unary_expr_node }),
                Ok(Some(BinaryExprT { binary_expr_node }))
                => initializer_expr_t_opt = Some(BinaryExprT { binary_expr_node }),
                Ok(Some(FrameEventExprT { frame_event_part }))
                => initializer_expr_t_opt = Some(FrameEventExprT { frame_event_part }),
                _ => {
                    let err_msg = "Unexpected assignment expression value.";
                    self.error_at_current(err_msg);
                    return Err(ParseError::new(err_msg))
                },
            }
        } else if matches!(self.peek().token_type,TokenType::In) {
            // pass
            let x = 1;

        } else {
            // All variables should be initialized to something.
            let err_msg = "Expected '='. All variables must be initialized.";
            self.error_at_current(err_msg);
            return Err(ParseError::new(err_msg));
        }

        let variable_decl_node = VariableDeclNode::new(
            name.clone(),
            type_node_opt.clone(),
            is_constant,
            initializer_expr_t_opt,
            identifier_decl_scope.clone(),
        );

        let variable_decl_node_rcref = Rc::new(RefCell::new(variable_decl_node));

        if self.is_building_symbol_table {
            // syntactic pass
            // add variable to current symbol table
            let scope = self.arcanum.get_current_identifier_scope();
            let variable_symbol = VariableSymbol::new(name, type_node_opt, scope);
            let variable_symbol_rcref = Rc::new(RefCell::new(variable_symbol));
            let variable_symbol_t = match identifier_decl_scope {
                IdentifierDeclScope::DomainBlock => SymbolType::DomainVariable {
                    domain_variable_symbol_rcref: variable_symbol_rcref,
                },
                IdentifierDeclScope::StateVar => SymbolType::StateVariable {
                    state_variable_symbol_rcref: variable_symbol_rcref,
                },
                IdentifierDeclScope::EventHandlerVar => SymbolType::EventHandlerVariable {
                    event_handler_variable_symbol_rcref: variable_symbol_rcref,
                },
                IdentifierDeclScope::LoopVar => SymbolType::LoopVar {
                    loop_variable_symbol_rcref: variable_symbol_rcref
                },
                IdentifierDeclScope::BlockVar => SymbolType::BlockVar {
                    block_variable_symbol_rcref: variable_symbol_rcref
                },
                _ => return Err(ParseError::new("Unrecognized variable scope.")),
            };
            // TODO: make current_symtab private
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let ret = self.arcanum
                .current_symtab
                .borrow_mut()
                .insert_symbol(&variable_symbol_t);
            match ret {
                Ok(()) => {}
                Err(err_msg) => {
                    self.error_at_current(err_msg.as_str());
                    return Err(ParseError::new(err_msg.as_str()))
                }
            }
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
        } else {
            // semantic pass

            // TODO
            self.arcanum
                .debug_print_current_symbols(self.arcanum.get_current_symtab());
            let x = self.arcanum.lookup(&name, &IdentifierDeclScope::None);
            let y = x.unwrap();
            let z = y.borrow();
            match &*z {
                SymbolType::DomainVariable {
                    domain_variable_symbol_rcref,
                } => {
                    domain_variable_symbol_rcref.borrow_mut().ast_node_opt =
                        Some(variable_decl_node_rcref.clone());
                }
                SymbolType::StateVariable {
                    state_variable_symbol_rcref,
                } => {
                    //                    let a = state_variable_symbol_rcref.borrow();
                    state_variable_symbol_rcref.borrow_mut().ast_node_opt =
                        Some(variable_decl_node_rcref.clone());
                }
                SymbolType::EventHandlerVariable {
                    event_handler_variable_symbol_rcref,
                } => {
                    event_handler_variable_symbol_rcref.borrow_mut().ast_node_opt =
                        Some(variable_decl_node_rcref.clone());
                }
                SymbolType::LoopVar {
                    loop_variable_symbol_rcref,
                } => {
                    loop_variable_symbol_rcref.borrow_mut().ast_node_opt =
                        Some(variable_decl_node_rcref.clone());
                }
                _ => return Err(ParseError::new("Unrecognized variable scope.")),
            }
            // now need to keep current_symtab when in semantic parse phase and link to
            // ast nodes as necessary.
        }

        Ok(variable_decl_node_rcref)
    }

    /* --------------------------------------------------------------------- */

    // TODO return result
    //    fn state(&mut self) -> Rc<RefCell<StateNode>> {
    fn state(&mut self) -> Result<Rc<RefCell<StateNode>>, ParseError> {
        let line = self.previous().line;

        // TODO
        if !self.match_token(&[TokenType::Identifier]) {
            // error message and synchronize
            self.error_at_current("Expected state name.");
            let sync_tokens = &vec![
                TokenType::State,
                TokenType::ActionsBlock,
                TokenType::DomainBlock,
                TokenType::SystemEnd,
            ];
            self.synchronize(sync_tokens);

            let state_node = StateNode::new(
                String::from("error"),
                None,
                None,
                Option::None,
                Vec::new(),
                Option::None,
                Option::None,
                None,
                0,
            );
            let state_node_rcref = Rc::new(RefCell::new(state_node));
            return Ok(state_node_rcref);
        }
        let id = self.previous();
        let state_name = id.lexeme.clone();

        self.state_name_opt = Some(state_name.clone());

        let state_symbol_rcref;
        if self.is_building_symbol_table {
            if self.arcanum.get_state(&state_name).is_some() {
                self.error_at_previous(&format!("Duplicate state name {}.", &state_name));
            }
            let state_symbol = StateSymbol::new(&state_name, self.arcanum.get_current_symtab());
            state_symbol_rcref = Rc::new(RefCell::new(state_symbol));
            self.arcanum.enter_scope(ParseScopeType::State {
                state_symbol: state_symbol_rcref.clone(),
            });
        } else {
            self.arcanum.set_parse_scope(&state_name);
            state_symbol_rcref = self.arcanum.get_state(&state_name).unwrap();
        }

        // parse state parameters e.g. $S1[x]
        //   let params:Option<Vec<ParameterNode>>
        let mut pop_state_params_scope = false;
        let mut params_opt = None;
        if self.match_token(&[TokenType::LBracket]) {
            // generate StateContext mechanism for state parameter support
            self.generate_state_context = true;
            match self.parameters() {
                Ok(Some(parameters)) => {
                    pop_state_params_scope = true;
                    if self.is_building_symbol_table {
                        match self.arcanum.get_state(&state_name) {
                            Some(state_symbol) => {
                                let state_params_scope_symbol = StateParamsScopeSymbol::new();
                                let state_params_scope_symbol_rcref =
                                    Rc::new(RefCell::new(state_params_scope_symbol));
                                self.arcanum.enter_scope(ParseScopeType::StateParams {
                                    state_params_scope_symbol_rcref,
                                });
                                for param in &parameters {
                                    let scope = self.arcanum.get_current_identifier_scope();
                                    let x = state_symbol.borrow_mut().add_parameter(
                                        param.param_name.clone(),
                                        param.param_type_opt.clone(),
                                        scope,
                                    );
                                    let ret = self.arcanum.insert_symbol(x);
                                    match ret {
                                        Ok(()) => {}
                                        Err(err_msg) => {
                                            self.error_at_previous(err_msg.as_str());
                                            return Err(ParseError::new(err_msg.as_str()))
                                        }
                                    }
                                }
                            }
                            None => {
                                return Err(ParseError::new(&format!(
                                    "Fatal error: unable to find state {}.",
                                    state_name.clone()
                                )));
                            }
                        }
                    } else {
                        self.arcanum
                            .set_parse_scope(StateParamsScopeSymbol::scope_name());
                    }
                    params_opt = Some(parameters);
                }
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {}
            }
        }

        let mut dispatch_opt: Option<DispatchNode> = None;

        // Dispatch clause.
        // '=>' '$' state_id
        if self.match_token(&[TokenType::Dispatch]) {
            match self.consume(TokenType::State, "Expected '$'") {
                Ok(_) => {
                    if self.match_token(&[TokenType::Identifier]) {
                        let id = self.previous();
                        let target_state_name = id.lexeme.clone();

                        let target_state_ref = StateRefNode::new(target_state_name);
                        dispatch_opt = Some(DispatchNode::new(target_state_ref, id.line));
                    } else {
                        self.error_at_current("Expected dispatch target state identifier.");
                        let sync_tokens = &vec![
                            TokenType::Pipe,
                            TokenType::State,
                            TokenType::ActionsBlock,
                            TokenType::DomainBlock,
                            TokenType::SystemEnd,
                        ];
                        self.synchronize(sync_tokens);
                    }
                }
                Err(_) => {
                    // synchronize to next event handler, state, remaining blocks or the end token
                    let sync_tokens = &vec![
                        TokenType::Pipe,
                        TokenType::State,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                        TokenType::SystemEnd,
                    ];
                    self.synchronize(sync_tokens);
                }
            }
        }

        // add to hierarchy

        match &dispatch_opt {
            Some(dispatch_node) => match &mut self.system_hierarchy_opt {
                Some(system_hierarchy) => {
                    let target_state_name = dispatch_node.target_state_ref.name.clone();
                    system_hierarchy.add_node(state_name.clone(), target_state_name);
                }
                None => {
                    return Err(ParseError::new("System Hierarchy should always be here."));
                }
            },
            None => match &mut self.system_hierarchy_opt {
                Some(system_hierarchy) => {
                    system_hierarchy.add_node(state_name.clone(), String::new());
                }
                None => {
                    return Err(ParseError::new("System Hierarchy should always be here."));
                }
            },
        }

        // state local variables
        let mut vars_opt = None;
        let mut vars = Vec::new();

        if self.is_building_symbol_table {
            let state_local_scope_struct = StateLocalScopeSymbol::new();
            let state_local_scope_symbol_rcref = Rc::new(RefCell::new(state_local_scope_struct));
            let state_local_scope = ParseScopeType::StateLocal {
                state_local_scope_symbol_rcref,
            };
            self.arcanum.enter_scope(state_local_scope);
        } else {
            self.arcanum
                .set_parse_scope(StateLocalScopeSymbol::scope_name());
        }

        // variable decl
        // let v     (mutable)
        // const c   (immutable)
        while self.match_token(&[TokenType::Var, TokenType::Const]) {
            self.generate_state_context = true;
            match self.variable_decl(IdentifierDeclScope::StateVar) {
                Ok(variable_node) => {
                    vars.push(variable_node);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        if !vars.is_empty() {
            vars_opt = Some(vars);
        }

        // State Calls
        let mut calls_opt = None;
        let mut calls = Vec::new();

        // @TODO - add reference syntax
        while self.match_token(&[TokenType::Identifier]) {
            match self.variable_or_call_expr(IdentifierDeclScope::None) {
                Ok(Some(CallChainLiteralExprT {
                    call_chain_expr_node,
                })) => calls.push(call_chain_expr_node),
                Ok(Some(_)) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {} // continue
            }
        }

        if !calls.is_empty() {
            calls_opt = Some(calls);
        }

        // Parse any event handlers.

        // TODO: make this Option?
        let mut evt_handlers: Vec<Rc<RefCell<EventHandlerNode>>> = Vec::new();
        let mut enter_event_handler = Option::None;
        let mut exit_event_handler = Option::None;

        let mut event_names = HashMap::new();

        loop {
            while self.match_token(&[TokenType::SingleLineComment]) {
                // consume
                // @TODO: fix this. see https://app.asana.com/0/1199651557660024/1199953268166075/f
                // this is a hack because we don't use
                // match on the next tests but instead use peek().
                // this causes an error for this situation:
                /*
                $State
                    |continueEvent|
                    >       --- continue terminator
                    |returnEvent|
                    ^       --- return terminator

                 */
            }

            // TODO - figure out AnyMessage. Is it working?
            if self.peek().token_type == TokenType::At
                || self.peek().token_type == TokenType::Pipe
                || self.peek().token_type == TokenType::AnyMessage
            {
                while self.peek().token_type == TokenType::At
                    || self.peek().token_type == TokenType::Pipe
                    || self.peek().token_type == TokenType::AnyMessage
                {
                    match self.event_handler() {
                        Ok(eh_opt) => {
                            if let Some(eh) = eh_opt {
                                let eh_rcref = Rc::new(RefCell::new(eh));

                                // find enter/exit event handlers
                                {
                                    // new scope to make BC happy
                                    let eh_ref = eh_rcref.as_ref().borrow();
                                    let evt = eh_ref.event_symbol_rcref.as_ref().borrow();

                                    if evt.is_enter_msg {
                                        if enter_event_handler.is_some() {
                                            self.error_at_current(&format!(
                                                "State ${} has more than one enter event handler.",
                                                &state_name
                                            ));
                                        } else {
                                            enter_event_handler = Some(eh_rcref.clone());
                                        }
                                    } else if evt.is_exit_msg {
                                        if exit_event_handler.is_some() {
                                            self.error_at_current(&format!(
                                                "State ${} has more than one exit event handler.",
                                                &state_name
                                            ));
                                        } else {
                                            exit_event_handler = Some(eh_rcref.clone());
                                        }
                                    } else {
                                        if event_names.contains_key(&evt.msg) {
                                            let err_msg = &format!("Event handler {} already exists.", evt.msg);
                                            self.error_at_previous(&err_msg);
//                                            return Err(ParseError::new(err_msg));

                                        } else {
                                            event_names.insert(evt.msg.clone(),evt.msg.clone() );
                                        }

                                    }
                                }

                                self.current_event_symbol_opt = None;
                                evt_handlers.push(eh_rcref);
                            }
                        }
                        Err(_) => {
                            let sync_tokens = &vec![
                                TokenType::Pipe,
                                TokenType::State,
                                TokenType::ActionsBlock,
                                TokenType::DomainBlock,
                                TokenType::SystemEnd,
                            ];
                            self.synchronize(sync_tokens);
                        }
                    }
                }
            } else {
                let follows_vec = &vec![
                    TokenType::State,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::SystemEnd,
                ];
                if self.follows(self.peek(), follows_vec) {
                    // next token is expected
                    break;
                } else {
                    self.error_at_current("Unexpected token in event handler message");
                    let sync_tokens = &vec![
                        TokenType::Pipe,
                        TokenType::AnyMessage,
                        TokenType::State,
                        TokenType::ActionsBlock,
                        TokenType::DomainBlock,
                    ];
                    if !self.synchronize(sync_tokens) {
                        return Err(ParseError::new("TODO"));
                    }
                }
            }
        }

        // TODO: Moved this down here as I think is a bug to hve it above but not sure.
        self.arcanum.exit_parse_scope(); // state block scope (StateBlockScopeSymbol)

        let state_node = StateNode::new(
            state_name,
            params_opt,
            vars_opt,
            calls_opt,
            evt_handlers,
            enter_event_handler,
            exit_event_handler,
            dispatch_opt,
            line,
        );
        let state_node_rcref = Rc::new(RefCell::new(state_node));

        // If this is the 2nd pass, set the reference to the AST state node.
        if !self.is_building_symbol_table {
            // let state_validator = StateSemanticValidator::new();
            // if !state_validator.has_valid_exit_semantics(&state_node_rcref.borrow()) {
            //     return Err(ParseError::new("TODO"));
            // }
            state_symbol_rcref
                .borrow_mut()
                .set_state_node(Rc::clone(&state_node_rcref));
        }

        self.state_name_opt = None;

        if pop_state_params_scope {
            self.arcanum.exit_parse_scope(); // state params scope
        }
        self.arcanum.exit_parse_scope(); // state scope

        Ok(state_node_rcref)
    }

    /* --------------------------------------------------------------------- */

    // event_handler -> '|' Identifier '|' event_handler_terminator

    fn event_handler(&mut self) -> Result<Option<EventHandlerNode>, ParseError> {
        let message_type: MessageType;
        // Hack - there is a weird bug w/ Clion that doesn't let msg be uninitialized.
        // It just hangs upon exiting the method.
        let mut msg: String = "".to_string();
        let line_number: usize;
        self.interface_method_called = false;

        self.event_handler_has_transition = false;
        //    let a = self.message();

        match self.message() {
            Ok(MessageType::AnyMessage { line }) => {
                line_number = line;
                message_type = AnyMessage { line }
            }
            Ok(MessageType::CustomMessage { message_node }) => {
                line_number = message_node.line;
                msg = message_node.name.clone();

                message_type = CustomMessage { message_node };
            }
            Err(parse_error) => {
                self.error_at_current("Error parsing event handler message.");
                return Err(parse_error);
            }
        }

        let mut is_declaring_event = false;

        if self.is_building_symbol_table {
            let event_symbol_rcref;

            // get or create the event symbol for the message we found
            match self.arcanum.get_event(&*msg, &self.state_name_opt) {
                Some(x) => {
                    event_symbol_rcref = Rc::clone(&x);
                }
                None => {
                    let event_symbol = EventSymbol::new(
                        &self.arcanum.symbol_config,
                        &msg,
                        None,
                        None,
                        None,
                        self.state_name_opt.clone(),
                    );
                    event_symbol_rcref = Rc::new(RefCell::new(event_symbol));
                    self.arcanum.declare_event(Rc::clone(&event_symbol_rcref));

                    // This is the first time we are seeing this event.
                    // Set flag so parameters and return type are added to event symbol
                    // during this parse.
                    is_declaring_event = true;
                }
            }

            // create the event handler symbol and enter the event handler scope
            let event_handler_symbol =
                EventHandlerScopeSymbol::new(&msg, Rc::clone(&event_symbol_rcref));
            let event_handler_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_symbol));

            self.arcanum.enter_scope(ParseScopeType::EventHandler {
                event_handler_scope_symbol_rcref,
            });
        } else {
            self.arcanum.set_parse_scope(&msg);
        }

        // Remember to pop param scope at end if it is entered.
        let mut pop_params_scope = false;

        // Parse event handler parameters
        if self.match_token(&[TokenType::LBracket]) {
            if msg == self.arcanum.symbol_config.enter_msg_symbol {
                self.generate_state_context = true;
            }

            match self.parameters() {
                Ok(Some(parameters)) => {
                    // have parsed params - make sure they match w/ symbol
                    // pop scope at end.
                    pop_params_scope = true;
                    if self.is_building_symbol_table {
                        let event_symbol_rcref =
                            self.arcanum.get_event(&*msg, &self.state_name_opt).unwrap();

                        // if this is the first encounter w/ this event
                        // then add parameters to the event symbol.
                        // TODO: Not sure how this overlaps w/ the symbol table
                        // having an event parameter scope but maybe (probably is)
                        // duplicative.

                        if is_declaring_event {
                            // add the parameters to the symbol
                            let mut vec = Vec::new();
                            for param_node in &parameters {
                                let param_symbol = ParameterSymbol::new(
                                    param_node.param_name.clone(),
                                    param_node.param_type_opt.clone(),
                                    IdentifierDeclScope::None,
                                );
                                vec.push(param_symbol);
                            }
                            event_symbol_rcref.borrow_mut().params_opt = Some(vec);
                        } else {
                            // validate event handler's parameters match the event symbol's parameters
                            if event_symbol_rcref.borrow().params_opt.is_none()
                                && !parameters.is_empty()
                            {
                                self.error_at_current(&format!("Event handler {} parameters do not match a previous declaration."
                                                               ,msg
                                ));
                            }
                        }

                        let event_handler_params_scope_struct =
                            EventHandlerParamsScopeSymbol::new(event_symbol_rcref);
                        let event_handler_params_scope_symbol_rcref =
                            Rc::new(RefCell::new(event_handler_params_scope_struct));
                        let event_handler_params_scope = ParseScopeType::EventHandlerParams {
                            event_handler_params_scope_symbol_rcref,
                        };
                        self.arcanum.enter_scope(event_handler_params_scope);
                        let mut event_symbol_params_opt: Option<Vec<ParameterSymbol>> = None;

                        let event_symbol_rcref =
                            match self.arcanum.get_event(&msg, &self.state_name_opt) {
                                Some(x) => x,
                                None => {
                                    return Err(ParseError::new(&format!(
                                        "Fatal error - could not find event {}.",
                                        msg
                                    )));
                                }
                            };

                        let mut event_handler_params_scope_symbol =
                            EventHandlerParamsScopeSymbol::new(Rc::clone(&event_symbol_rcref));
                        let event_symbol_rcref =
                            self.arcanum.get_event(&msg, &self.state_name_opt).unwrap();
                        {
                            match &event_symbol_rcref.borrow().params_opt {
                                Some(symbol_params) => {
                                    // compare existing event symbol params w/ parsed ones
                                    for (i, x) in symbol_params.iter().enumerate() {
                                        match parameters.get(i) {
                                            Some(parameter_node) => {
                                                if x.is_eq(parameter_node) {
                                                    let scope =
                                                        self.arcanum.get_current_identifier_scope();
                                                    let symbol_type =
                                                        event_handler_params_scope_symbol
                                                            .add_parameter(
                                                                parameter_node.param_name.clone(),
                                                                parameter_node
                                                                    .param_type_opt
                                                                    .clone(),
                                                                scope,
                                                            );
                                                    self.arcanum.debug_print_current_symbols(
                                                        self.arcanum.get_current_symtab(),
                                                    );
                                                    let ret = self.arcanum.insert_symbol(symbol_type);
                                                    self.arcanum.debug_print_current_symbols(
                                                        self.arcanum.get_current_symtab(),
                                                    );
                                                    match ret {
                                                        Ok(()) => {}
                                                        Err(err_msg) => {
                                                            self.error_at_previous(err_msg.as_str());
                                                            return Err(ParseError::new(err_msg.as_str()))
                                                        }
                                                    }
                                                } else {
                                                    // TODO
                                                    self.error_at_current(
                                                        "Parameters for event handler do not match declaration in interface or a previous event handler for the message.",
                                                    );
                                                }
                                            }
                                            None => {
                                                self.error_at_current(
                                                    "Incorrect number of parameters",
                                                );
                                            }
                                        }
                                    }
                                }
                                None => {
                                    // this is the first time we've seen parameters for this event.
                                    // Take them as the definitive list.
                                    let mut event_symbol_params = Vec::new();

                                    for param in &parameters {
                                        let param_name = &param.param_name.clone();
                                        let mut param_type_opt: Option<TypeNode> = None;
                                        if param.param_type_opt.is_some() {
                                            let pt =
                                                &param.param_type_opt.as_ref().unwrap().clone();
                                            param_type_opt = Some(pt.clone());
                                        }
                                        let scope = self.arcanum.get_current_identifier_scope();
                                        let b = ParameterSymbol::new(
                                            param_name.clone(),
                                            param_type_opt.clone(),
                                            scope,
                                        );
                                        // add to Arcanum event symbol
                                        event_symbol_params.push(b);

                                        // add to event handler scope symbol (needed for lookups using the scope chain)
                                        let scope = self.arcanum.get_current_identifier_scope();
                                        let x = event_handler_params_scope_symbol.add_parameter(
                                            param_name.clone(),
                                            param_type_opt.clone(),
                                            scope,
                                        );
                                        let ret = self.arcanum.insert_symbol(x);
                                        match ret {
                                            Ok(()) => {}
                                            Err(err_msg) => {
                                                self.error_at_previous(err_msg.as_str());
                                                return Err(ParseError::new(err_msg.as_str()))
                                            }
                                        }
                                    }
                                    event_symbol_params_opt = Some(event_symbol_params);
                                }
                            }
                        }
                        if let Some(parameter_symbols) = event_symbol_params_opt {
                            event_symbol_rcref.borrow_mut().params_opt = Some(parameter_symbols)
                        }
                    } else {
                        // leave these comments to show how to debug scope errors.
                        //                       self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                        self.arcanum
                            .set_parse_scope(EventHandlerParamsScopeSymbol::scope_name());
                        //                       self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                    }
                }
                Ok(None) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
            }
        } else {
            // no parameter list
            let event_symbol_rcref = self.arcanum.get_event(&msg, &self.state_name_opt).unwrap();
            if event_symbol_rcref.borrow().params_opt.is_some() {
                self.error_at_current(&format!(
                    "Event handler {} parameters do not match a previous declaration.",
                    msg
                ));
            }
        }

        // Parse return type
        if self.match_token(&[TokenType::Colon]) {
            let return_type_opt;
            match self.type_decl() {
                Ok(type_node) => return_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
            if is_declaring_event {
                // declaring event so add return type to event symbol
                // let id = self.previous();
                // let return_type = id.lexeme.clone();

                let event_symbol_rcref =
                    self.arcanum.get_event(&*msg, &self.state_name_opt).unwrap();
                event_symbol_rcref.borrow_mut().ret_type_opt = return_type_opt;
            } else {
                let event_symbol_rcref =
                    self.arcanum.get_event(&*msg, &self.state_name_opt).unwrap();
                let symbol_rettype_node_opt = &event_symbol_rcref.borrow().ret_type_opt;
                if symbol_rettype_node_opt.is_none() != return_type_opt.is_none() {
                    self.error_at_current(&format!(
                        "Event handler {} return type does not match a previous declaration.",
                        msg
                    ));
                } else {
                    let symbol_return_type = symbol_rettype_node_opt.as_ref().unwrap();
                    let event_handler_return_type = return_type_opt.as_ref().unwrap();
                    if symbol_return_type != event_handler_return_type {
                        self.error_at_current(&format!(
                            "Event handler {} return type does not match a previous declaration.",
                            msg
                        ));
                    }
                }
            }
        } else {
            // no declared return type
            let event_symbol_rcref = self.arcanum.get_event(&*msg, &self.state_name_opt).unwrap();
            let symbol_rettype_node_opt = &event_symbol_rcref.borrow().ret_type_opt;

            if symbol_rettype_node_opt.is_some() {
                self.error_at_current(&format!(
                    "Event handler {} return type does not match a previous declaration.",
                    msg
                ));
            }
        }

        if self.is_building_symbol_table {
            let event_handler_local_scope_struct = EventHandlerLocalScopeSymbol::new();
            let event_handler_local_scope_symbol_rcref =
                Rc::new(RefCell::new(event_handler_local_scope_struct));
            let event_handler_local_scope = ParseScopeType::EventHandlerLocal {
                event_handler_local_scope_symbol_rcref,
            };
            self.arcanum.enter_scope(event_handler_local_scope);
        } else {
            self.arcanum
                .set_parse_scope(EventHandlerLocalScopeSymbol::scope_name());
        }

        let event_symbol_rcref = self.arcanum.get_event(&*msg, &self.state_name_opt).unwrap();
        self.current_event_symbol_opt = Some(event_symbol_rcref);

        let statements = self.statements();
        let event_symbol_rcref = self.arcanum.get_event(&msg, &self.state_name_opt).unwrap();
        let ret_event_symbol_rcref = Rc::clone(&event_symbol_rcref);
        let terminator_node = match self.event_handler_terminator(event_symbol_rcref) {
            Ok(terminator_node) => terminator_node,
            Err(_parse_error) => {
                // TODO: this vec keeps the parser from hanging. don't know why
                let sync_tokens = &vec![
                    TokenType::Pipe,
                    TokenType::State,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::SystemEnd,
                ];
                self.synchronize(sync_tokens);
                // create "dummy" node to keep processing
                // TODO: 1) make line # an int so as to set it to -1 when it is a dummy node and 2) confirm this is the best way
                // to keep going
                TerminatorExpr::new(TerminatorType::Return, None, 0)
            }
        };

        // The state name must be set in an enclosing context. Otherwise fail
        // with extreme prejudice.

        let st_name = match &self.state_name_opt {
            Some(state_name) => state_name.clone(),
            None => {
                return Err(ParseError::new(&format!("[line {}] Fatal error - event handler {} missing enclosing state context. Please notify bugs@frame-lang.org.",line_number,msg)));
            }
        };

        // TODO: Moved this down here as I think is a bug to hve it above but not sure.
        self.arcanum.exit_parse_scope(); // event handler local block scope (EventHandlerLocalScopeSymbol)
        if pop_params_scope {
            self.arcanum.exit_parse_scope(); // event handler params scope (EventHandlerParamsScopeSymbol)
        }
        self.arcanum.exit_parse_scope(); // event handler lscope (EventHandlerScopeSymbol)

        if self.panic_mode {
            return Err(ParseError::new("TODO"));
        }

        self.current_event_symbol_opt = None;

        Ok(Some(EventHandlerNode::new(
            st_name,
            message_type,
            statements,
            terminator_node,
            ret_event_symbol_rcref,
            self.event_handler_has_transition,
            line_number,
        )))
    }

    /* --------------------------------------------------------------------- */

    // event_handler_terminator -> '^' | '>'

    // TODO: - explore just returning the TerminatorType
    fn event_handler_terminator(
        &mut self,
        _: Rc<RefCell<EventSymbol>>,
    ) -> Result<TerminatorExpr, ParseError> {
        // let x = event_symbol_rcfef.borrow();
        // let ret_type = match &x.ret_type_opt {
        //     Some(ret_type) => ret_type.clone(),
        //     None => None,
        // };

        if self.match_token(&[TokenType::Caret]) {
            if self.match_token(&[TokenType::LParen]) {
                let expr_t = match self.expression() {
                    Ok(Some(expr_t)) => expr_t,
                    _ => {
                        // TODO - err_msg everywhere for ParseErrors
                        self.error_at_current("Expected expression as return value.");
                        return Err(ParseError::new("TODO"));
                    }
                };

                if let Err(parse_error) = self.consume(TokenType::RParen, "Expected ')'.") {
                    return Err(parse_error);
                }

                Ok(TerminatorExpr::new(
                    Return,
                    Some(expr_t),
                    self.previous().line,
                ))
            } else {
                Ok(TerminatorExpr::new(Return, None, self.previous().line))
            }
        } else if self.match_token(&[TokenType::ElseContinue]) {
            Ok(TerminatorExpr::new(Continue, None, self.previous().line))
        } else {
            let mut err_msg = format!("Expected event handler terminator." );
            if self.interface_method_called {
                err_msg = format!("Interface method call must be last statement in an event handler.")
            }
            self.error_at_current(&err_msg);
            Err(ParseError::new(&err_msg))
        }
    }

    /* --------------------------------------------------------------------- */

    // statements ->

    // TODO: need result and optional
    #[allow(clippy::vec_init_then_push)] // false positive in 1.51, fixed by 1.55
    fn statements(&mut self) -> Vec<DeclOrStmtType> {
        let mut statements = Vec::new();
        let mut is_err = false;

        self.loop_stmt_idx = 0;

        loop {

            match self.decl_or_stmt() {
                Ok(opt_smt) => match opt_smt {
                    Some(statement) => {
                        match &statement {
                            DeclOrStmtType::StmtT { stmt_t } => {
                                // Transitions or state changes must be the last statement in
                                // an event handler.
                                match stmt_t {
                                    StatementType::TransitionStmt { .. } => {
                                        statements.push(statement);
                                        // must be last statement in event handler so return
                                        return statements;
                                    }
                                    StatementType::ExpressionStmt { expr_stmt_t } => {
                                        if let ExprStmtType::CallChainLiteralStmtT {call_chain_literal_stmt_node} = expr_stmt_t {
                                            match call_chain_literal_stmt_node.call_chain_literal_expr_node.call_chain.get(0) {
                                                Some(CallChainLiteralNodeType::InterfaceMethodCallT {interface_method_call_expr_node}) => {
                                                    // interface method call must be last statement.
                                                    self.interface_method_called = true;
                                                    statements.push(statement); // return statements;

                                                    return statements;
                                                }
                                                _ => {
                                                    statements.push(statement); // return statements;
                                                }
                                            }
                                        } else {
                                            statements.push(statement); // return statements;
                                        }

                                    }
                                    StatementType::ChangeStateStmt { .. } => {
                                        statements.push(statement);
                                        // state changes disallowed in actions
                                        if self.is_action_context {
                                            self.error_at_current("Transitions disallowed in actions.");
                                            // is_err = true;
                                        }
                                        // must be last statement so return
                                        return statements;
                                    }
                                    StatementType::LoopStmt { .. } => {
                                        statements.push(statement);
                                        self.loop_stmt_idx = self.loop_stmt_idx + 1;
                                    }
                                    StatementType::ContinueStmt { .. } => {
                                        if self.is_loop_context {
                                            statements.push(statement);
                                        } else {
                                            is_err = true;
                                        }
                                    }
                                    StatementType::BreakStmt { .. } => {
                                        if self.is_loop_context {
                                            statements.push(statement);
                                        } else {
                                            is_err = true;
                                        }
                                    }
                                    _ => {
                                        statements.push(statement);
                                    }
                                }
                            }
                            _ => {
                                statements.push(statement);
                            }
                        }
                    }
                    None => {
                        return statements;
                    }
                },
                Err(_err) => {
                    is_err = true;
                }
            }

            if is_err {
                is_err = false;
                let sync_tokens = &vec![
                    TokenType::Identifier,
                    TokenType::LParen,
                    TokenType::Caret,
                    TokenType::GT,
                    TokenType::System,
                    TokenType::State,
                    TokenType::PipePipe,
                    TokenType::Dot,
                    TokenType::Colon,
                    TokenType::Pipe,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::SystemEnd,
                ];
                self.synchronize(sync_tokens);
            }
        }
    }

    /* --------------------------------------------------------------------- */

    fn decl_or_stmt(&mut self) -> Result<Option<DeclOrStmtType>, ParseError> {
        if self.match_token(&[TokenType::Var, TokenType::Const]) {
            // this is hardcoded and needs to be set based on context. specifically BlockVar
            match self.variable_decl(IdentifierDeclScope::EventHandlerVar) {
                Ok(var_decl_t_rc_ref) => {
                    return Ok(Some(DeclOrStmtType::VarDeclT { var_decl_t_rc_ref }));
                }
                Err(parse_error) => {
                    return Err(parse_error);
                }
            }
        }

        match self.statement() {
            Ok(opt_smt) => match opt_smt {
                Some(stmt_t) => Ok(Some(DeclOrStmtType::StmtT { stmt_t })),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    /* --------------------------------------------------------------------- */

    // statement ->

    fn statement(&mut self) -> Result<Option<StatementType>, ParseError> {
        let mut expr_t_opt: Option<ExprType> = None;

        match self.expression() {
            Ok(et_opt) => expr_t_opt = et_opt,
            Err(_) => {
                let sync_tokens = &vec![
                    TokenType::Caret,
                    TokenType::ElseContinue,
                    TokenType::Identifier,
                    TokenType::Pipe,
                    TokenType::State,
                    TokenType::ActionsBlock,
                    TokenType::DomainBlock,
                    TokenType::SystemEnd,
                ];
                self.synchronize(sync_tokens);
            }
        }

        match expr_t_opt {
            Some(expr_t) => {
                if self.is_bool_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.bool_test(expr_t);
                    return match result {
                        Ok(bool_test_node) => {
                            let bool_test_t = TestType::BoolTest { bool_test_node };
                            let test_stmt_node = TestStatementNode::new(bool_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };
                } else if self.is_string_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.string_match_test(expr_t);
                    return match result {
                        Ok(string_match_test_node) => {
                            let match_test_t = TestType::StringMatchTest {
                                string_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };
                } else if self.is_number_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.number_match_test(expr_t);
                    return match result {
                        Ok(number_match_test_node) => {
                            let match_test_t = TestType::NumberMatchTest {
                                number_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };
                } else if self.is_enum_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current("Not a testable expression.");
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.enum_match_test(expr_t);
                    return match result {
                        Ok(enum_match_test_node) => {
                            let match_test_t = TestType::EnumMatchTest {
                                enum_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt { test_stmt_node };
                            Ok(Some(test_stmt_t))
                        }
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        }
                    };

                }

                match expr_t {
                    ExprListT { expr_list_node } => {
                        // path for transitions w/ an exit params group
                        if self.match_token(&[TokenType::Transition]) {
                            match self.transition(Some(expr_list_node)) {
                                Ok(Some(stmt_t)) => return Ok(Some(stmt_t)),
                                Ok(None) => return Err(ParseError::new("TODO")),
                                Err(parse_err) => return Err(parse_err),
                            }
                        } else {
                            let expr_list_stmt_node = ExprListStmtNode::new(expr_list_node);
                            let expr_stmt_t = ExprListStmtT {expr_list_stmt_node};
                            return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                            // self.error_at_previous(
                            //     "Expected '->' token following expression list.",
                            // );
                            // return Err(ParseError::new("TODO"));
                        }
                    }
                    CallExprT { call_expr_node } => {
                        let call_stmt_node = CallStmtNode::new(call_expr_node);
                        let expr_stmt_t: ExprStmtType = CallStmtT { call_stmt_node };
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    }
                    CallExprListT { .. } => {
                        // this should never happen as it is the () in a call like foo()
                        return Err(ParseError::new("TODO"));
                    }
                    VariableExprT { var_node } => {
                        let variable_stmt_node = VariableStmtNode::new(var_node);
                        let expr_stmt_t: ExprStmtType =
                            ExprStmtType::VariableStmtT { variable_stmt_node };
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    }
                    // TODO: remove this - doesn't make any sense
                    ActionCallExprT {
                        action_call_expr_node,
                    } => {
                        let action_call_stmt_node = ActionCallStmtNode::new(action_call_expr_node);
                        let expr_stmt_t: ExprStmtType = ActionCallStmtT {
                            action_call_stmt_node,
                        };
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    }
                    CallChainLiteralExprT {
                        call_chain_expr_node,
                    } => {
                        let call_chain_literal_stmt_node =
                            CallChainLiteralStmtNode::new(call_chain_expr_node);
                        let expr_stmt_t: ExprStmtType = ExprStmtType::CallChainLiteralStmtT {
                            call_chain_literal_stmt_node,
                        };
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    }
                    // TODO: $$[+] isn't a true expression as there is no return value defined (yet)
                    // Could define it to return the pushed context.
                    StateStackOperationExprT {
                        state_stack_op_node,
                    } => {
                        let state_stack_operation_statement_node =
                            StateStackOperationStatementNode::new(state_stack_op_node);
                        return Ok(Some(StatementType::StateStackStmt {
                            state_stack_operation_statement_node,
                        }));
                    }
                    AssignmentExprT {
                        assignment_expr_node,
                    } => {
                        let assignment_stmt_node = AssignmentStmtNode::new(assignment_expr_node);
                        let expr_stmt_t: ExprStmtType = ExprStmtType::AssignmentStmtT {
                            assignment_stmt_node,
                        };
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    }
                    EnumeratorExprT { enum_expr_node } => {
                        let enumerator_stmt_node = EnumeratorStmtNode::new(enum_expr_node);
                        let expr_stmt_t: ExprStmtType = ExprStmtType::EnumeratorStmtT {
                            enumerator_stmt_node,
                        };
                        return Ok(Some(StatementType::ExpressionStmt {expr_stmt_t}));
                    }
                    BinaryExprT { binary_expr_node } => {
                        let binary_stmt_node = BinaryStmtNode::new(binary_expr_node);
                        let expr_stmt_t: ExprStmtType = ExprStmtType::BinaryStmtT {
                            binary_stmt_node,
                        };
                        return Ok(Some(StatementType::ExpressionStmt {expr_stmt_t}));
                    }
                    LiteralExprT { literal_expr_node } => {
                        // Superstring is the only permitted literal type to be a statement.
                        if  literal_expr_node.token_t == TokenType::SuperString {
                            let super_string_stmt_node = SuperStringStmtNode::new(literal_expr_node);
                            return Ok(Some(StatementType::SuperStringStmt {super_string_stmt_node}))
                        }
                        self.error_at_previous("Literal statements not allowed.");
                        return Err(ParseError::new("TODO"));
                    }
                    FrameEventExprT { .. } => {
                        self.error_at_previous("Frame Event statements not allowed.");
                        return Err(ParseError::new("TODO"));
                    }
                    UnaryExprT { .. } => {
                        self.error_at_previous("Unary expression statements not allowed.");
                        return Err(ParseError::new("TODO"));
                    }


                }
            }
            None => {
                // This path is for transitions w/o an exit params group
                if self.match_token(&[TokenType::Transition]) {
                    return match self.transition(None) {
                        Ok(Some(transition)) => Ok(Some(transition)),
                        Ok(_) => Err(ParseError::new("TODO")),
                        Err(parse_error) => Err(parse_error),
                    }
                }
            }
        }

        if self.match_token(&[TokenType::ChangeState]) {
            return match self.change_state() {
                Ok(Some(state_context_t)) => Ok(Some(state_context_t)),
                Ok(None) => Err(ParseError::new("TODO")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::Loop]) {
            return match self.loop_statement_scope() {
                Ok(Some(loop_stmt_t)) => Ok(Some(loop_stmt_t)),
                Ok(None) => Err(ParseError::new("TODO")),
                Err(parse_error) => Err(parse_error),
            };
        }

        if self.match_token(&[TokenType::Continue]) {
            let continue_stmt_node = ContinueStmtNode::new();
            return Ok(Some(StatementType::ContinueStmt {continue_stmt_node}));
        }
        if self.match_token(&[TokenType::Break]) {
            let break_stmt_node = BreakStmtNode::new();
            return Ok(Some(StatementType::BreakStmt {break_stmt_node}));
        }
        if self.match_token(&[TokenType::SuperString]) {

        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // This method detects if an expression can be tested:
    // (a = 1) ? --- not testable
    // (a + b) ? --- not testable (TODO: review but think not
    // not sure about frame event and parts. for now yes.

    fn is_testable_expression(&self, expr_t: &ExprType) -> bool {
        match expr_t {
            AssignmentExprT { .. } => false,
            UnaryExprT { .. } => true,
            BinaryExprT { .. } => true,
            ExprListT { expr_list_node } => {
                if expr_list_node.exprs_t.len() != 1 {
                    return false;
                }

                let first_expr_t = expr_list_node.exprs_t.first().unwrap();
                self.is_testable_expression(first_expr_t)
            }
            _ => true,
        }
    }

    /* --------------------------------------------------------------------- */

    fn is_bool_test(&self) -> bool {
        self.peek().token_type == TokenType::BoolTestTrue
            || self.peek().token_type == TokenType::BoolTestFalse
    }

    /* --------------------------------------------------------------------- */

    fn is_string_match_test(&self) -> bool {
        self.peek().token_type == TokenType::StringTest
    }

    /* --------------------------------------------------------------------- */

    fn is_number_match_test(&self) -> bool {
        self.peek().token_type == TokenType::NumberTest
    }

    /* --------------------------------------------------------------------- */

    fn is_enum_match_test(&self) -> bool {
        self.peek().token_type == TokenType::EnumTest
    }

    /* --------------------------------------------------------------------- */

    // TODO
    // fn is_regex_test(&self) -> bool {
    //
    //     //panic!("not implemented")
    //     false
    // }

    /* --------------------------------------------------------------------- */

    // bool_test -> ('?' | '?!') bool_test_true_branch (':' bool_test_else_branch)? '::'

    fn bool_test(&mut self, expr_t: ExprType) -> Result<BoolTestNode, ParseError> {
        let is_negated: bool;

        // '?'
        if self.match_token(&[TokenType::BoolTestTrue]) {
            is_negated = false;

        // ?!
        } else if self.match_token(&[TokenType::BoolTestFalse]) {
            is_negated = true;
        } else {
            return Err(ParseError::new("TODO"));
        }

        let mut conditional_branches: Vec<BoolTestConditionalBranchNode> = Vec::new();

        let first_branch_node =
            match self.bool_test_conditional_branch_statements(is_negated, expr_t) {
                Ok(branch_node) => branch_node,
                Err(parse_error) => return Err(parse_error),
            };

        conditional_branches.push(first_branch_node);

        while self.match_token(&[TokenType::ElseContinue]) {
            match self.bool_test_else_continue_branch() {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                }
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' bool_test_else_branch)?
        let mut bool_test_else_node_opt: Option<BoolTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            bool_test_else_node_opt = Option::from(match self.bool_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) =
            self.consume(TokenType::ColonColon, "Expected TestTerminator.")
        {
            return Err(parse_error);
        }

        Ok(BoolTestNode::new(
            conditional_branches,
            bool_test_else_node_opt,
        ))
    }

    /* --------------------------------------------------------------------- */

    // bool_test_body -> statements* branch_terminator?

    fn bool_test_else_continue_branch(
        &mut self,
    ) -> Result<BoolTestConditionalBranchNode, ParseError> {
        let expr_t: ExprType;
        let result = self.expression();
        match result {
            Ok(expression_opt) => match expression_opt {
                Some(et) => {
                    expr_t = et;
                }
                None => {
                    return Err(ParseError::new("TODO"));
                }
            },
            Err(parse_error) => return Err(parse_error),
        }

        let is_negated: bool;

        // '?'
        if self.match_token(&[TokenType::BoolTestTrue]) {
            is_negated = false;

        // ?!
        } else if self.match_token(&[TokenType::BoolTestFalse]) {
            is_negated = true;
        } else {
            return Err(ParseError::new("TODO"));
        }

        self.bool_test_conditional_branch_statements(is_negated, expr_t)
    }


    /* --------------------------------------------------------------------- */

    // bool_test_conditional_branch_statements -> statements* branch_terminator?

    fn bool_test_conditional_branch_statements(
        &mut self,
        is_negated: bool,
        expr_t: ExprType,
    ) -> Result<BoolTestConditionalBranchNode, ParseError> {
        let scope_name = &format!("bool_test_conditional_branch");
        if self.is_building_symbol_table {
            let block_scope_rcref = Rc::new(RefCell::new(BlockScope::new(scope_name)));
            self.arcanum.enter_scope(ParseScopeType::Block {
                block_scope_rcref,
            });
        } else {
//            let scope_name = &format!("{}.{}",LoopStmtScopeSymbol::scope_name(),self.loop_stmt_idx);
            self.arcanum
                .set_parse_scope(scope_name);
        }
        let ret = self.bool_test_conditional_branch_statements_scope(is_negated, expr_t);
        // exit block scope
        self.arcanum.exit_parse_scope();
        ret
    }

    /* --------------------------------------------------------------------- */

    // bool_test_conditional_branch_statements -> statements* branch_terminator?

    fn bool_test_conditional_branch_statements_scope(
        &mut self,
        is_negated: bool,
        expr_t: ExprType,
    ) -> Result<BoolTestConditionalBranchNode, ParseError> {
        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_expr_opt) => Ok(BoolTestConditionalBranchNode::new(
                is_negated,
                expr_t,
                statements,
                branch_terminator_expr_opt,
            )),
            Err(parse_error) => {
                    Err(parse_error)
            },
        }
    }

    /* --------------------------------------------------------------------- */

    // bool_test_else_branch -> statements* branch_terminator?

    fn bool_test_else_branch(&mut self) -> Result<BoolTestElseBranchNode, ParseError> {
        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_expr_opt) => Ok(BoolTestElseBranchNode::new(
                statements,
                branch_terminator_expr_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // branch_terminator -> '^' | ':>'

    // TODO: explore returning a TerminatorType rather than node
    fn branch_terminator(&mut self) -> Result<Option<TerminatorExpr>, ParseError> {
        if self.match_token(&[TokenType::Caret]) {
            if self.match_token(&[TokenType::LParen]) {
                let expr_t = match self.decorated_unary_expression() {
                    Ok(Some(expr_t)) => expr_t,
                    _ => {
                        self.error_at_current("Expected expression as return value.");
                        return Err(ParseError::new("TODO"));
                    }
                };

                if let Err(parse_error) = self.consume(TokenType::RParen, "Expected ')'.") {
                    return Err(parse_error);
                }
                return Ok(Some(TerminatorExpr::new(
                    Return,
                    Some(expr_t),
                    self.previous().line,
                )));
            } else {
                return Ok(Some(TerminatorExpr::new(
                    Return,
                    None,
                    self.previous().line,
                )));
            }
        } else if self.match_token(&[TokenType::GT]) {
            return Ok(Some(TerminatorExpr::new(
                Continue,
                None,
                self.previous().line,
            )));
        } else {
            Ok(None)
        }
    }

    /* --------------------------------------------------------------------- */

    // '^' '('
    //           return_expr -> expression ')'

    // fn return_expr(&mut self, expr_t:ExpressionType) -> Result<StringMatchTestNode,ParseError> {
    //
    // }

    /* --------------------------------------------------------------------- */

    // string_match_test -> '?~'  ('/' match_string ('|' match_string)* '/' (statement* branch_terminator?) ':>')+ ':' (statement* branch_terminator?) '::'

    fn string_match_test(&mut self, expr_t: ExprType) -> Result<StringMatchTestNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::StringTest, "Expected '?~'.") {
            return Err(parse_error);
        }

        let mut conditional_branches: Vec<StringMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.string_match_test_match_branch() {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        while self.match_token(&[TokenType::ElseContinue]) {
            match self.string_match_test_match_branch() {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                }
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' match_test_else_branch)?
        let mut else_branch_opt: Option<StringMatchTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            else_branch_opt = Option::from(match self.string_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) =
            self.consume(TokenType::ColonColon, "Expected TestTerminator.")
        {
            return Err(parse_error);
        }

        Ok(StringMatchTestNode::new(
            expr_t,
            conditional_branches,
            else_branch_opt,
        ))
    }

    /* --------------------------------------------------------------------- */

    // string_match_test ->  ('/' match_string ('|' match_string)* '/' (statement* branch_terminator?) ':>')+  '::'

    fn string_match_test_match_branch(
        &mut self,
    ) -> Result<StringMatchTestMatchBranchNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::ForwardSlash, "Expected '/'.") {
            return Err(parse_error);
        }

        let mut match_strings: Vec<String> = Vec::new();

        if !self.match_token(&[TokenType::MatchString]) {
            return Err(ParseError::new("TODO"));
        }

        //        let token = self.previous();
        let match_string_tok = self.previous();
        let match_pattern_string = match_string_tok.lexeme.clone();
        match_strings.push(match_pattern_string);

        while self.match_token(&[TokenType::Pipe]) {
            if !self.match_token(&[TokenType::MatchString]) {
                return Err(ParseError::new("TODO"));
            }

            //           let token = self.previous();
            let match_string_tok = self.previous();
            let match_pattern_string = match_string_tok.lexeme.clone();
            match_strings.push(match_pattern_string);
        }

        let string_match_pattern_node = StringMatchTestPatternNode::new(match_strings);

        if let Err(parse_error) = self.consume(TokenType::ForwardSlash, "Expected '/'.") {
            return Err(parse_error);
        }

        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_t_opt) => Ok(StringMatchTestMatchBranchNode::new(
                string_match_pattern_node,
                statements,
                branch_terminator_t_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // match_test_else_branch -> statements* branch_terminator?

    fn string_match_test_else_branch(
        &mut self,
    ) -> Result<StringMatchTestElseBranchNode, ParseError> {
        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_opt) => Ok(StringMatchTestElseBranchNode::new(
                statements,
                branch_terminator_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // expression -> TODO

    fn expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        self.assignment()
    }

    /* --------------------------------------------------------------------- */

    // expression -> TODO

    fn assignment(&mut self) -> Result<Option<ExprType>, ParseError> {
        let l_value = match self.equality() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        if self.match_token(&[TokenType::Equals]) {
            // this changes the tokens generated for expression lists
            // like (a) and (a b c)
            self.is_parsing_rhs = true;

            let line = self.previous().line;
            let r_value = match self.equality() {
                Ok(Some(expr_type)) => {
                    self.is_parsing_rhs = false;
                    expr_type
                }
                Ok(None) => {
                    self.is_parsing_rhs = false;
                    return Ok(None);
                }
                Err(parse_error) => {
                    self.is_parsing_rhs = false;
                    return Err(parse_error);
                }
            };

            let assignment_expr_node = AssignmentExprNode::new(l_value, r_value, line);
            return Ok(Some(AssignmentExprT {
                assignment_expr_node,
            }));
        }

        // if self.match_token(&[TokenType::DeclAssignment]) {
        //     // this changes the tokens generated for expression lists
        //     // like (a) and (a b c)
        //     self.is_parsing_rhs = true;
        //
        //     let line = self.previous().line;
        //     let r_value = match self.equality() {
        //         Ok(Some(expr_type)) => {
        //             self.is_parsing_rhs = false;
        //             expr_type
        //         }
        //         Ok(None) => {
        //             self.is_parsing_rhs = false;
        //             return Ok(None);
        //         }
        //         Err(parse_error) => {
        //             self.is_parsing_rhs = false;
        //             return Err(parse_error);
        //         }
        //     };
        //
        //     let assignment_expr_node = AssignmentExprNode::new(l_value, r_value, true,line);
        //     return Ok(Some(AssignmentExprT {
        //         assignment_expr_node,
        //     }));
        // }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn equality(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.comparison() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            //           let line = self.previous().line;
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.comparison() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn comparison(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.term() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[
            TokenType::GT,
            TokenType::GreaterEqual,
            TokenType::LT,
            TokenType::LessEqual,
        ]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.term() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn term(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.factor() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::Dash, TokenType::Plus]) {
            let operator_token = self.previous().clone();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.factor() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => {

                    let err_msg = format!("Expected binary expression. Found \"{} {}\".", l_value.to_string(), operator_token.lexeme);
                    self.error_at_current(&err_msg);
                    let parse_error = ParseError::new(
                        err_msg.as_str(),
                    );
                    return Err(parse_error);
                },
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn factor(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.logical_xor() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::ForwardSlash, TokenType::Star]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.logical_xor() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn logical_xor(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.logical_or() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::LogicalXor]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.logical_or() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn logical_or(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.logical_and() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::PipePipe]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.logical_and() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn logical_and(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut l_value = match self.decorated_unary_expression() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&[TokenType::LogicalAnd]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.decorated_unary_expression() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT { binary_expr_node };
        }

        Ok(Some(l_value))
    }


    /* --------------------------------------------------------------------- */

    fn decorated_unary_expression(&mut self) -> Result<Option<ExprType>, ParseError> {

        match self.prefix_unary_expression() {
            Ok(Some(expr_t)) => {
                return Ok(Some(expr_t));
            }
            Ok(None) => {
                match self.unary_expression2() {
                    Ok(Some(expr_t)) => return Ok(Some(expr_t)),
                    Ok(None) => return Ok(None),
                    Err(parse_err) => return Err(parse_err),
                }
            },
            Err(parse_error) => {
                return Err(parse_error)
            },
        }
    }

    /* --------------------------------------------------------------------- */

    fn prefix_unary_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        if self.match_token(&[TokenType::PlusPlus]) {
            match self.unary_expression2() {
                Ok(Some(mut expr_t)) => {
                    match expr_t {
                        ExprType::CallChainLiteralExprT {ref mut call_chain_expr_node} => {
                            call_chain_expr_node.inc_dec = IncDecExpr::PreInc;
                        }
                        ExprType::LiteralExprT {ref mut literal_expr_node} => {
                            literal_expr_node.inc_dec = IncDecExpr::PreInc;
                        }
                        // ExprType::ExprListT {ref mut expr_list_node} => {
                        //     expr_list_node.inc_dec = IncDecExpr::PreInc;
                        // }
                        ExprType::ExprListT {ref mut expr_list_node} => {
                     //       expr_list_node.inc_dec = IncDecExpr::PreDec;
                            for expr_t in &mut expr_list_node.exprs_t {
                                match expr_t {
                                    ExprType::CallChainLiteralExprT {ref mut call_chain_expr_node} => {
                                        call_chain_expr_node.inc_dec = IncDecExpr::PreDec;
                                    }
                                    ExprType::LiteralExprT {ref mut literal_expr_node} => {
                                        literal_expr_node.inc_dec = IncDecExpr::PreDec;
                                    }
                                    _ => {
                                        let err_msg = "Can not increment/decrement something that cannot be assigned.";
                                        self.error_at_current(err_msg);
                                        return Err(ParseError::new(err_msg));
                                    }

                                }
                            }
                        }
                        _ => {

                        }
                    }
                    return Ok(Some(expr_t));
                },
                Ok(None) => return Ok(None),
                Err(parse_err) => return Err(parse_err),
            }
        } else if self.match_token(&[TokenType::DashDash]) {
            match self.unary_expression2() {
                Ok(Some(mut expr_t)) => {
                    match expr_t {
                        ExprType::CallChainLiteralExprT {ref mut call_chain_expr_node} => {
                            call_chain_expr_node.inc_dec = IncDecExpr::PreDec;
                        }
                        ExprType::LiteralExprT {ref mut literal_expr_node} => {
                            literal_expr_node.inc_dec = IncDecExpr::PreDec;
                        }
                        ExprType::ExprListT {ref mut expr_list_node} => {
  //                          expr_list_node.inc_dec = IncDecExpr::PreDec;
                            for expr_t in &mut expr_list_node.exprs_t {
                                match expr_t {
                                    ExprType::CallChainLiteralExprT {ref mut call_chain_expr_node} => {
                                        call_chain_expr_node.inc_dec = IncDecExpr::PreDec;
                                    }
                                    ExprType::LiteralExprT {ref mut literal_expr_node} => {
                                        literal_expr_node.inc_dec = IncDecExpr::PreDec;
                                    }
                                    _ => {
                                        let err_msg = "Can not increment/decrement something that cannot be assigned.";
                                        self.error_at_current(err_msg);
                                        return Err(ParseError::new(err_msg));
                                    }

                                }
                            }
                        }
                        _ => {

                        }
                    }
                    return Ok(Some(expr_t));
                },
                Ok(None) => return Ok(None),
                Err(parse_err) => return Err(parse_err),
            }
        }

        return self.postfix_unary_expression();
    }


    /* --------------------------------------------------------------------- */

    fn postfix_unary_expression(&mut self) -> Result<Option<ExprType>, ParseError> {
        match self.unary_expression2() {
            Ok(Some(CallChainLiteralExprT {
                        mut call_chain_expr_node,
                    })) => {
                let mut x = CallChainLiteralExprT {call_chain_expr_node};
                self.post_inc_dec_expression(&mut x);
                return Ok(Some(x));
            }
            Ok(Some(expr_t)) => return Ok(Some(expr_t)),
            Err(parse_error) => return Err(parse_error),
            Ok(None) => return Ok(None),
        }
    }

    /* --------------------------------------------------------------------- */

    // unary_expression -> TODO

    fn unary_expression2(&mut self) -> Result<Option<ExprType>, ParseError> {

        if self.match_token(&[TokenType::Bang, TokenType::Dash]) {
            let token = self.previous();
            let mut operator_type = OperatorType::get_operator_type(&token.token_type);
            if operator_type == OperatorType::Minus {
                // change this so the code gen doesn't have a space between the - and ID
                // -x rather than - x
                operator_type = OperatorType::Negated;
            }
            let right_expr_t = self.decorated_unary_expression();
            match right_expr_t {
                Ok(Some(x)) => {
                    let unary_expr_node = UnaryExprNode::new(operator_type, x);
                    return Ok(Some(UnaryExprT { unary_expr_node }));
                }
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {} // continue
            }
        }

        // '(' ')' | '(' expr+ ')'
        if self.match_token(&[TokenType::LParen]) {
            match self.expr_list() {
                Ok(Some(ExprListT {
                    expr_list_node: expr_node,
                })) => {
                    return Ok(Some(ExprListT {
                        expr_list_node: expr_node,
                    }))
                }
                Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {} // continue
            }
        }

        let mut scope = IdentifierDeclScope::None;

        //        let mut scope_override = false;
        if self.match_token(&[TokenType::System]) {
            if self.match_token(&[TokenType::Dot]) {
                scope = IdentifierDeclScope::DomainBlock;
            } else {
                // System reference
                //               scope = IdentifierDeclScope::System;
                let id_node = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::System,
                    false,
                    self.previous().line,
                );
                let var_scope = id_node.scope.clone();
                let var_node = VariableNode::new(id_node, var_scope, None);
                return Ok(Some(VariableExprT { var_node }));
            }

        //           scope_override = true;
        } else if self.match_token(&[TokenType::State]) {
            if self.match_token(&[TokenType::LBracket]) {
                return if self.match_token(&[TokenType::Identifier]) {
                    //                    let id = self.previous().lexeme.clone();
                    let id_node = IdentifierNode::new(
                        self.previous().clone(),
                        None,
                        IdentifierDeclScope::StateParam,
                        false,
                        self.previous().line,
                    );
                    let var_scope = id_node.scope.clone();
                    let symbol_type_rcref_opt =
                        self.arcanum.lookup(&id_node.name.lexeme, &var_scope);

                    let var_node = VariableNode::new(id_node, var_scope, symbol_type_rcref_opt);
                    if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                        return Err(parse_error); // TODO
                    }
                    Ok(Some(VariableExprT { var_node }))
                } else {
                    self.error_at_current("Expected identifier.");
                    Err(ParseError::new("TODO")) // TODO
                };
            } else if self.match_token(&[TokenType::Dot]) {
                return if self.match_token(&[TokenType::Identifier]) {
                    let id_node = IdentifierNode::new(
                        self.previous().clone(),
                        None,
                        IdentifierDeclScope::StateVar,
                        false,
                        self.previous().line,
                    );
                    let var_scope = id_node.scope.clone();
                    let symbol_type_rcref_opt =
                        self.arcanum.lookup(&id_node.name.lexeme, &var_scope);
                    let var_node = VariableNode::new(id_node, var_scope, symbol_type_rcref_opt);
                    Ok(Some(VariableExprT { var_node }))
                } else {
                    self.error_at_current("Expected identifier.");
                    Err(ParseError::new("TODO"))
                };
            } else {
                self.error_at_current("Unexpected token.");
                return Err(ParseError::new("TODO"));
            }
        } else if self.match_token(&[TokenType::PipePipeLBracket]) {
            //           if self.match_token(&[TokenType::LBracket]) {
            let id_node;
            let var_node;
            if self.match_token(&[TokenType::Identifier]) {
                id_node = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::EventHandlerParam,
                    false,
                    self.previous().line,
                );
                let var_scope = id_node.scope.clone();
                let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme, &var_scope);
                var_node = VariableNode::new(id_node, var_scope, symbol_type_rcref_opt);
            } else {
                self.error_at_current("Expected identifier.");
                return Err(ParseError::new("TODO"));
            }
            if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                return Err(parse_error);
            }
            return Ok(Some(VariableExprT { var_node }));
        } else if self.match_token(&[TokenType::PipePipeDot]) {
            let id_node;
            if self.match_token(&[TokenType::Identifier]) {
                let id_tok = self.previous().clone();
                id_node = IdentifierNode::new(
                    id_tok,
                    None,
                    IdentifierDeclScope::EventHandlerVar,
                    false,
                    self.previous().line,
                );
            } else {
                self.error_at_current("Expected identifier.");
                return Err(ParseError::new("TODO"));
            }

            let var_scope = id_node.scope.clone();
            let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme, &var_scope);
            let var_node = VariableNode::new(id_node, var_scope, symbol_type_rcref_opt);
            return Ok(Some(VariableExprT { var_node }));
        } else if self.match_token(&[TokenType::New]) {
            if self.match_token(&[TokenType::Identifier]) {
                match self.variable_or_call_expr(IdentifierDeclScope::None) {
                    Ok(Some(CallChainLiteralExprT {
                        mut call_chain_expr_node,
                    })) => {
                        call_chain_expr_node.is_new_expr = true;
                        return Ok(Some(CallChainLiteralExprT {
                            call_chain_expr_node,
                        }));
                    }
                    Ok(Some(_)) => return Err(ParseError::new("TODO")),
                    Err(parse_error) => return Err(parse_error),
                    Ok(None) => {} // continue
                }
            } else {
                self.error_at_current("Expected class.");
                return Err(ParseError::new("TODO"));
            }
        } else {
            // self.error_at_current("Expected identifier.");
            // return Err(ParseError::new("TODO"));
        }
        //     }
        // }

        // @TODO need to determine if this is the best way to
        // deal w/ references. We can basically put & in front
        // of a wide range of syntax it doesn't apply to.
        let mut is_reference = false;
        if self.match_token(&[TokenType::And]) {
            is_reference = true;
        }

        // TODO: I think only identifier is allowed?
        if self.match_token(&[TokenType::Identifier]) {
            match self.variable_or_call_expr(scope) {
                Ok(Some(VariableExprT { mut var_node })) => {
                    var_node.id_node.is_reference = is_reference;
                    return Ok(Some(VariableExprT { var_node }));
                }
                Ok(Some(CallExprT {
                    call_expr_node: method_call_expr_node,
                })) => {
                    return Ok(Some(CallExprT {
                        call_expr_node: method_call_expr_node,
                    }))
                }
                Ok(Some(ActionCallExprT {
                    action_call_expr_node,
                })) => {
                    return Ok(Some(ActionCallExprT {
                        action_call_expr_node,
                    }))
                }
                Ok(Some(CallChainLiteralExprT {
                    mut call_chain_expr_node,
                })) => {
                    // set the is_reference on first variable in the call chain
                    let call_chain_first_node_opt = call_chain_expr_node.call_chain.get_mut(0);
                    if let Some(call_chain_first_node) = call_chain_first_node_opt {
                        call_chain_first_node.setIsReference(is_reference);
                    }


                    let mut x = CallChainLiteralExprT {call_chain_expr_node};
                   // self.post_inc_dec_expression(&mut x);
                    return Ok(Some(x));
                    // return Ok(Some(CallChainLiteralExprT {
                    //     call_chain_expr_node,
                    // }));
                }
                Ok(Some(EnumeratorExprT {
                            enum_expr_node,
                        })) => {
                    return Ok(Some(EnumeratorExprT {
                        enum_expr_node,
                    }))
                }
                Ok(Some(_)) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {} // continue
            }
        }

        // number | string | bool | null | nil
        match self.literal_expr() {
            Ok(Some(mut literal_expr_node)) => {
                literal_expr_node.is_reference = is_reference;
                return Ok(Some(LiteralExprT { literal_expr_node }));
            }
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {} // continue
        }

        // $$[+] | $$[-]
        match self.stack_operation() {
            Ok(Some(state_stack_op_node)) => {
                return Ok(Some(StateStackOperationExprT {
                    state_stack_op_node,
                }))
            }
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {} // continue
        }

        // @ | @|| | @[x] | @^
        match self.frame_event_part(is_reference) {
            Ok(Some(frame_event_part)) => return Ok(Some(FrameEventExprT { frame_event_part })),
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {} // continue
        }

        // loop ...
        // match self.loop_expression() {
        //     Ok(Some(loop_types)) => return Ok(Some(LoopExprT {loop_types})),
        //     Err(parse_error) => return Err(parse_error),
        //     Ok(None) => {} // continue
        // }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    //

    fn stack_operation(&mut self) -> Result<Option<StateStackOperationNode>, ParseError> {
        if self.match_token(&[TokenType::StateStackOperationPush]) {
            self.generate_state_stack = true;
            let ssot = StateStackOperationNode::new(StateStackOperationType::Push);
            return Ok(Some(ssot));
        } else if self.match_token(&[TokenType::StateStackOperationPop]) {
            self.generate_state_stack = true;
            let ssot = StateStackOperationNode::new(StateStackOperationType::Pop);
            return Ok(Some(ssot));
        }

        Ok(None)
    }


    /* --------------------------------------------------------------------- */

    // TODO - update other scopes to follow this patter so that
    // TODO - all return paths automatically pop scope.

    fn loop_statement_scope(&mut self) -> Result<Option<StatementType>, ParseError> {
        // for all loop types, push a symbol table for new scope
        self.is_loop_context = true;
        if self.is_building_symbol_table {
            let scope_name = &format!("{}.{}",LoopStmtScopeSymbol::scope_name(),self.loop_stmt_idx);
            let loop_stmt_scope_symbol_rcref = Rc::new(RefCell::new(LoopStmtScopeSymbol::new(scope_name)));
            self.arcanum.enter_scope(ParseScopeType::Loop {
                loop_scope_symbol_rcref:loop_stmt_scope_symbol_rcref,
            });
        } else {
            // give each loop in a scope a unique name
            let scope_name = &format!("{}.{}",LoopStmtScopeSymbol::scope_name(),self.loop_stmt_idx);
            self.arcanum
                .set_parse_scope(scope_name);
        }
        // parse loop
        let ret = self.loop_statement_context();
        // exit loop scope
        self.arcanum.exit_parse_scope();
        self.is_loop_context = false;
        ret
    }

    /* --------------------------------------------------------------------- */

    // loop { foo() }
    // loop var x = 0; x < 10; x++ { foo(x) }
    // loop var x in range(5) { foo(x) }
    // loop .. { foo() continue break }

    fn loop_statement_context(&mut self) -> Result<Option<StatementType>, ParseError> {

        if self.match_token(&[TokenType::OpenBrace]) {
            // loop { foo() }
            return self.loop_infinite_statement();
        }

        let mut init_stmt = LoopFirstStmt::None;

        if self.match_token(&[TokenType::Var]) {
            // loop var x:int = 0; ...
            match self.variable_decl(IdentifierDeclScope::LoopVar) {
                Ok(var_decl_t_rc_ref) => {
                    init_stmt = LoopFirstStmt::VarDecl {
                        var_decl_node_rcref:var_decl_t_rc_ref
                    };
                }
                Err(parse_error) => {
                    return Err(parse_error);
                }
            }
        } else {
            // loop y in foo() { bar(y) }
            let first_expr_result =  self.expression();
            match first_expr_result {
                Ok(Some(expr_type)) => {
                    init_stmt = match expr_type {
                        VariableExprT { var_node } => {
                            LoopFirstStmt::Var {
                                var_node
                            }
                        }
                        AssignmentExprT {assignment_expr_node} => {
                            LoopFirstStmt::VarAssign {
                                assign_expr_node: assignment_expr_node
                            }
                        }
                        CallChainLiteralExprT {call_chain_expr_node} => {

                            LoopFirstStmt::CallChain {
                                call_chain_expr_node
                            }
                        }
                        _ => {
                            // TODO - improve error msg
                            let err_msg = format!("Invalid initial clause in loop.");
                            self.error_at_current(&err_msg);
                            let parse_error = ParseError::new(
                                err_msg.as_str(),
                            );
                            return Err(parse_error);
                        }
                    };
                    // if let AssignmentExprT {assignment_expr_node} = expr_type {
                    //     init_stmt = LoopFirstStmt::VarAssignInit {
                    //         assign_expr_node: assignment_expr_node
                    //     };
                    // }

                   // loop_first_expr_opt = Some(expr_type);
                }
                Ok(None) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }

        if self.match_token(&[TokenType::Semicolon]) {
            // loop var x:int = 0; ...
            // loop x = 0; ...
            return self.loop_for_statement(Some(init_stmt));
        }

        // loop var x:int in range(5) { foo(x) }
        // loop x in range(5) { foo(x) }
        if self.match_token(&[TokenType::In]) {

            return self.loop_in_statement(init_stmt);

            // match  self.expression() {
            //     Ok(Some(expr_t)) => {
            //         return self.loop_in_statement(Box::new(expr_t));
            //     }
            //     _ => {
            //         // TODO - improve error msg
            //         let err_msg = format!("Invalid initial clause for 'loop in' statement.");
            //         self.error_at_current(&err_msg);
            //         let parse_error = ParseError::new(
            //             err_msg.as_str(),
            //         );
            //         return Err(parse_error);
            //     }
            // }
            // if let Some(expr_type) =  {
            //     return self.loop_in_statement(Box::new(expr_type));
            // }
        }

        return  Err(ParseError::new("Unrecognized loop syntax."));
    }


    /* --------------------------------------------------------------------- */

    fn loop_infinite_statement(&mut self) -> Result<Option<StatementType>, ParseError> {

        let statements = self.statements();

        if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
            return Err(parse_error);
        }

        let loop_infinite_stmt_node = LoopInfiniteStmtNode::new(statements);

        let loop_stmt_node = LoopStmtNode::new(
            LoopStmtTypes::LoopInfiniteStmt { loop_infinite_stmt_node }
        );
        let stmt_type = StatementType::LoopStmt {loop_stmt_node};
        return Ok(Some(stmt_type));

    }

    /* --------------------------------------------------------------------- */

    fn loop_for_statement(&mut self, init_stmt: Option<LoopFirstStmt>) -> Result<Option<StatementType>, ParseError>  {

        let mut statements = Vec::new();
        let mut test_expr_opt = Option::None;
        let mut inc_dec_expr_opt = Option::None;

        let second_expr_result =  self.expression();
        match second_expr_result {
            Ok(Some(expr_type)) => {
                test_expr_opt = Some(expr_type);
            }
            Ok(None) => {}
            Err(err) => {
                return Err(err);
            }
        }
        if self.match_token(&[TokenType::Semicolon]) {}
        let third_expr_result =  self.expression();
        match third_expr_result {
            Ok(Some(expr_type)) => {
                inc_dec_expr_opt = Some(expr_type);
            }
            Ok(None) => {}
            Err(err) => {
                return Err(err);
            }
        }

        // statements block
        if self.match_token(&[TokenType::OpenBrace]) {
            statements = self.statements();

            if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
                return Err(parse_error);
            }

            let loop_for_stmt_node = LoopForStmtNode::new(init_stmt,
                                                      test_expr_opt,
                                                      inc_dec_expr_opt,
                                                      statements );

            let loop_stmt_node = LoopStmtNode::new(
                LoopStmtTypes::LoopForStmt { loop_for_stmt_node }
            );
            let stmt_type = StatementType::LoopStmt {loop_stmt_node};
            return Ok(Some(stmt_type));
        } else {
            return Err(ParseError::new("Missing loop open brace '{'"));
        }
    }


    /* --------------------------------------------------------------------- */

    fn loop_in_statement(&mut self,loop_first_stmt: LoopFirstStmt) -> Result<Option<StatementType>, ParseError>  {

        let mut statements = Vec::new();
        let iterable_expr;
        let second_expr_result =  self.expression();
        match second_expr_result {
            Ok(Some(expr_type)) => {
                iterable_expr = Box::new(expr_type);
            }
            Ok(None) => {
                self.error_at_current("Expected loop iterable expression.");
                return Err(ParseError::new("Expected loop iterable expression."));
            }
            Err(err) => {
                return Err(err);
            }
        }

        // statements block
        if self.match_token(&[TokenType::OpenBrace]) {
            statements = self.statements();

            if let Err(parse_error) = self.consume(TokenType::CloseBrace, "Expected '}'.") {
                return Err(parse_error);
            }

            // let expr_t = match x {
            //     LoopFirstStmt::VarDecl {var_node} => {
            //         VariableExprT {v}
            //     }
            //     _ => {
            //         let err_msg = "Expected variable or var declaration.";
            //         self.error_at_current(err_msg.clone());
            //         return Err(ParseError::new(err_msg));
            //     }
            // };
            let loop_in_stmt_node = LoopInStmtNode::new(loop_first_stmt,
                                                          iterable_expr,
                                                          statements );

            let loop_stmt_node = LoopStmtNode::new(
                LoopStmtTypes::LoopInStmt { loop_in_stmt_node }
            );
            let stmt_type = StatementType::LoopStmt {loop_stmt_node};
            return Ok(Some(stmt_type));
        } else {
            return Err(ParseError::new("Missing loop open brace '{'"));
        }
    }

        /* --------------------------------------------------------------------- */

    // Parse FrameEvent "part" identifier:
    // @||  - Event message
    // @[p] - Event parameter
    // @^   - Event return object/value

    fn frame_event_part(
        &mut self,
        is_reference: bool,
    ) -> Result<Option<FrameEventPart>, ParseError> {
        if !self.match_token(&[TokenType::At]) {
            return Ok(None);
        }

        // '@' '||'
        if self.match_token(&[TokenType::PipePipe]) {
            return Ok(Some(FrameEventPart::Message { is_reference }));
        }

        // '@' '[' identifier ']'
        if self.match_token(&[TokenType::LBracket]) {
            if self.match_token(&[TokenType::Identifier]) {
                let id_tok = self.previous().clone();

                if let Err(parse_error) = self.consume(TokenType::RBracket, "Expected ']'.") {
                    return Err(parse_error);
                }

                // TODO!! must test for existence
                let param_symbol_rcref;
                let symbol_type_rcref_opt = self
                    .arcanum
                    .lookup(&id_tok.lexeme, &IdentifierDeclScope::None);
                match symbol_type_rcref_opt {
                    Some(symbol_type_rcref) => {
                        let symbol_type = symbol_type_rcref.borrow();

                        match &*symbol_type {
                            SymbolType::EventHandlerParam {
                                event_handler_param_symbol_rcref,
                            } => {
                                param_symbol_rcref = event_handler_param_symbol_rcref.clone();
                            }
                            _ => {
                                self.error_at_current(&format!(
                                    "{} is not an event parameter.",
                                    id_tok.lexeme
                                ));
                                return Err(ParseError::new(""));
                            }
                        }
                    }
                    None => {
                        self.error_at_current(&format!(
                            "Unknown event parameter - {}.",
                            id_tok.lexeme
                        ));
                        return Err(ParseError::new(""));
                    }
                }

                return Ok(Some(FrameEventPart::Param {
                    param_symbol_rcref,
                    is_reference,
                }));
            } else {
                self.error_at_current("Expected identifier.");
                return Err(ParseError::new("TODO"));
            }
        }

        // '@' '^'
        if self.match_token(&[TokenType::Caret]) {
            return Ok(Some(FrameEventPart::Return { is_reference }));
        }

        // @
        Ok(Some(FrameEventPart::Event { is_reference }))
    }

    /* --------------------------------------------------------------------- */

    // expr_list -> '(' expression* ')'

    fn expr_list(&mut self) -> Result<Option<ExprType>, ParseError> {
        let mut expressions: Vec<ExprType> = Vec::new();

        loop {
            if self.match_token(&[TokenType::RParen]) {
                break;
            }
            match self.expression() {
                Ok(Some(expression)) => {
                    expressions.push(expression);
                }
                // should see a list of valid expressions until ')'
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            }
            if self.peek().token_type == TokenType::RParen {
                continue;
            }
            if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {
                return Err(parse_error);
            }
        }

        let expr_list = ExprListT {
            expr_list_node: ExprListNode::new(expressions),
        };
        Ok(Some(expr_list))
    }

    /* --------------------------------------------------------------------- */

    // expr_list -> '(' expression* ')'
    //
    // fn expr_list(&mut self) -> Result<Option<ExprType>, ParseError> {
    //     let mut expressions: Vec<ExprType> = Vec::new();
    //
    //     while !self.match_token(&[TokenType::RParen]) {
    //         match self.expression() {
    //             Ok(Some(expression)) => {
    //                 expressions.push(expression);
    //             }
    //             // should see a list of valid expressions until ')'
    //             Ok(None) => return Ok(None),
    //             Err(parse_error) => return Err(parse_error),
    //         }
    //         if self.peek().token_type == TokenType::Comma {
    //             if let Err(parse_error) = self.consume(TokenType::Comma, "Expected comma.") {
    //                 return Err(parse_error);
    //             }
    //         }
    //     }
    //
    //     let expr_list = ExprListT {
    //         expr_list_node: ExprListNode::new(expressions),
    //     };
    //     Ok(Some(expr_list))
    // }


    /* --------------------------------------------------------------------- */

    fn post_inc_dec_expression(&mut self, expr_t:&mut ExprType) -> Result<(), ParseError> {

        let mut inc_dec = IncDecExpr::None;

        if self.match_token(&[TokenType::PlusPlus]) {
            inc_dec = IncDecExpr::PostInc;
        } else if self.match_token(&[TokenType::DashDash]) {
            inc_dec = IncDecExpr::PostDec;
        }

        match expr_t {
            ExprType::CallChainLiteralExprT{ref mut call_chain_expr_node} => {
                call_chain_expr_node.inc_dec = inc_dec;
            }
            _ => {
                let err_msg = "Expression can not be auto incremented or decrecmented";
                self.error_at_current(err_msg);
                return Err(ParseError::new(err_msg));
            }
        }

        return Ok(());
    }

    /* --------------------------------------------------------------------- */

    // TODO: create a new return type that is narrowed to just the types this method returns.
    // TODO: change the return type to be CallChainLiteralExprT as it doesn't return anything else.
    fn variable_or_call_expr(
        &mut self,
        explicit_scope: IdentifierDeclScope,
    ) -> Result<Option<ExprType>, ParseError> {
        let mut scope: IdentifierDeclScope;

        let mut id_node = IdentifierNode::new(
            self.previous().clone(),
            None,
            explicit_scope.clone(),
            false,
            self.previous().line,
        );
        let mut call_chain: std::collections::VecDeque<CallChainLiteralNodeType> =
            std::collections::VecDeque::new();

        // Loop over the tokens looking for "callable" tokens (methods and identifiers)
        // separated by '.' and build the "call_chain".

        let mut is_first_node = true;
        loop {
            // test for method call
            if self.match_token(&[TokenType::LParen]) {
                let r = self.method_call(id_node);
                match r {
                    Ok(method_call_expr_node) => {
                        if !self.is_building_symbol_table {
                            if !is_first_node {
                                // if not first node in the chain then the node is just an method
                                // call on another object
                                let call_t = CallChainLiteralNodeType::CallT {
                                    call: method_call_expr_node,
                                };
                                call_chain.push_back(call_t);
                            } else {
                                // is first or only node in a call chain. Determine if an action,
                                // interface or external call.
                                let method_name = method_call_expr_node.identifier.name.lexeme.clone();
                                let action_decl_symbol_opt = self.arcanum.lookup_action(&method_name);

                                match action_decl_symbol_opt {
                                    Some(ads) => {
                                        // first node is an action

                                        let action_symbol_opt    =
                                            self.arcanum.lookup_action(&method_name);

                                        match action_symbol_opt {
                                            Some(action_scope_symbol) => {

                                                // validate signature

                                                let a = action_scope_symbol.borrow();
                                                let b = a.ast_node_opt.as_ref().unwrap();
                                                let c = &b.borrow().params;
                                                // check if difference in the existance of parameters
                                                if (!c.is_none() && method_call_expr_node.call_expr_list.exprs_t.is_empty()) ||
                                                    (c.is_none() && !method_call_expr_node.call_expr_list.exprs_t.is_empty()) {
                                                    let err_msg = format!("Incorrect number of arguments for action '{}'.",method_name);
                                                    self.error_at_previous(&err_msg);
                                                    let parse_error = ParseError::new(
                                                        err_msg.as_str(),
                                                    );
                                                    return Err(parse_error);
                                                }

                                                // check parameter count equals argument count
                                                match &b.borrow().params {
                                                    Some(symbol_params) => {
                                                        if symbol_params.len() != method_call_expr_node.call_expr_list.exprs_t.len() {
                                                            let err_msg = format!("Number of arguments does not match parameters for action '{}'.", method_name);
                                                            self.error_at_previous(&err_msg);
                                                            let parse_error = ParseError::new(
                                                                err_msg.as_str(),
                                                            );
                                                            return Err(parse_error);
                                                        }
                                                    }
                                                    None => {}
                                                }

                                                let mut action_call_expr_node =
                                                    ActionCallExprNode::new(method_call_expr_node);
                                                action_call_expr_node.set_action_symbol(&Rc::clone(&ads));
                                                call_chain.push_back(CallChainLiteralNodeType::ActionCallT {
                                                    action_call_expr_node,
                                                });
                                            }
                                            None => {
                                                // first node is not an action or interface call.
                                                let call_t = CallChainLiteralNodeType::CallT {
                                                    call: method_call_expr_node,
                                                };
                                                call_chain.push_back(call_t);
                                            }
                                        }


                                    }
                                    None => {
                                        // first node is not an action. see if it is an interface call
                                        let interface_method_symbol_opt =
                                            self.arcanum.lookup_interface_method(&method_name);

                                        match interface_method_symbol_opt {
                                            Some(interface_method_symbol) => {
                                                // first node is an interface call.
                                                if self.is_action_context {
                                                    // iface calls disallowed in actions.
                                                    let err_msg = format!("Interface calls disallowed inside of actions.");
                                                    self.error_at_current(&err_msg);
                                                    let parse_error = ParseError::new(
                                                        err_msg.as_str(),
                                                    );
                                                    return Err(parse_error);
                                                }

                                                // validate signature

                                                let a = interface_method_symbol.borrow();
                                                let b = a.ast_node_opt.as_ref().unwrap();
                                                let c = &b.borrow().params;
                                                // check if difference in the existance of parameters
                                                if (!c.is_none() && method_call_expr_node.call_expr_list.exprs_t.is_empty()) ||
                                                    (c.is_none() && !method_call_expr_node.call_expr_list.exprs_t.is_empty()) {
                                                    let err_msg = format!("Incorrect number of arguments for interface '{}'.", method_name);
                                                    self.error_at_previous(&err_msg);
                                                    let parse_error = ParseError::new(
                                                        err_msg.as_str(),
                                                    );
                                                    return Err(parse_error);
                                                }

                                                // check parameter count equals argument count
                                                match &b.borrow().params {
                                                    Some(symbol_params) => {
                                                        if symbol_params.len() != method_call_expr_node.call_expr_list.exprs_t.len() {
                                                            let err_msg = format!("Number of arguments does not match parameters for interface call '{}'.", method_name);
                                                            self.error_at_previous(&err_msg);
                                                            let parse_error = ParseError::new(
                                                                err_msg.as_str(),
                                                            );
                                                            return Err(parse_error);
                                                        }
                                                    }
                                                    None => {}
                                                }

                                                let mut interface_method_call_expr_node =
                                                    InterfaceMethodCallExprNode::new(
                                                        method_call_expr_node,
                                                    );
                                                interface_method_call_expr_node
                                                    .set_interface_symbol(&Rc::clone(
                                                        &interface_method_symbol,
                                                    ));
                                                call_chain.push_back(
                                                    CallChainLiteralNodeType::InterfaceMethodCallT {
                                                        interface_method_call_expr_node,
                                                    },
                                                );
                                            }
                                            None => {
                                                // first node is not an action or interface call.
                                                let call_t = CallChainLiteralNodeType::CallT {
                                                    call: method_call_expr_node,
                                                };
                                                call_chain.push_back(call_t);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => return Err(ParseError::new("TODO")),
                }
            } else {
                let token_name = id_node.name.lexeme.clone();
                match self.get_identifier_scope(&id_node, &explicit_scope) {
                    Ok(id_decl_scope) => {
                        scope = id_decl_scope
                    },
                    Err(err) => return Err(err),
                }
                let node = if scope == IdentifierDeclScope::None || !is_first_node {
                    CallChainLiteralNodeType::IdentifierNodeT { id_node }
                } else {
                    // variables (or parameters) must be
                    // the first (or only) node in the call chain

                    let symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>> = self
                        .arcanum
                        .lookup(&id_node.name.lexeme, &explicit_scope)
                        .clone();
                    match &symbol_type_rcref_opt {
                        Some(symbol_t) => {
                            match &*symbol_t.borrow() {
                                SymbolType::EnumDeclSymbolT {enum_symbol_rcref} => {
                                    let enum_symbol = enum_symbol_rcref.borrow();

                                    let enum_decl_node = enum_symbol.ast_node_opt.as_ref().unwrap().borrow();
                                    // match '.'
                                    if !self.match_token(&[TokenType::Dot]) {
                                        let msg = &format!(
                                            "Expected '.' after enum {} identifier.",
                                            enum_symbol.name
                                        );
                                        self.error_at_current(msg);
                                        return Err(ParseError::new(msg));
                                    }

                                    if self.match_token(&[TokenType::Identifier]) {
                                        let enumerator_name = &self.previous().lexeme;
                                        let mut found_enumerator = false;
                                        for enum_decl_node in &enum_symbol.ast_node_opt.as_ref().unwrap().borrow().enums {
                                            if *enumerator_name == enum_decl_node.name {
                                                found_enumerator = true;
                                                break;
                                            }
                                        }
                                        if !found_enumerator {
                                            let msg = &format!(
                                                "Expected enumerator for {} - found {}.",
                                                enum_symbol.name,
                                                enumerator_name,
                                            );
                                            self.error_at_current(msg);
                                            return Err(ParseError::new(msg));
                                        }

                                        let enum_expr_node = EnumeratorExprNode::new(enum_decl_node.name.clone(), enumerator_name.clone());
                                        return Ok(Some(ExprType::EnumeratorExprT {enum_expr_node}));
                                    } else {
                                        return Err(ParseError::new("TODO"));
                                    }

                                }
                                _ => {}
                            }
                        }
                        None => {
                            // todo
                        }
                    }
                    let var_node =
                        VariableNode::new(id_node, scope, (&symbol_type_rcref_opt).clone());
                    CallChainLiteralNodeType::VariableNodeT { var_node }
                };
                call_chain.push_back(node);
            }

            // end of chain if no  '.'
            if !self.match_token(&[TokenType::Dot]) {
                break;
            }

            if self.match_token(&[TokenType::Identifier]) {
                id_node = IdentifierNode::new(
                    self.previous().clone(),
                    None,
                    IdentifierDeclScope::None,
                    false,
                    self.previous().line,
                );
            } else {
                return Err(ParseError::new("TODO"));
            }
            is_first_node = false;
        }

        let call_chain_literal_expr_node = CallChainLiteralExprNode::new(call_chain);
        Ok(Some(CallChainLiteralExprT {
            call_chain_expr_node: call_chain_literal_expr_node,
        }))
    }

    /* --------------------------------------------------------------------- */

    fn get_identifier_scope(
        &mut self,
        identifier_node: &IdentifierNode,
        explicit_scope: &IdentifierDeclScope,
    ) -> Result<IdentifierDeclScope, ParseError> {
        let mut scope: IdentifierDeclScope = IdentifierDeclScope::None;
        // find the variable in the arcanum
        let symbol_type_rcref_opt: Option<Rc<RefCell<SymbolType>>> = self
            .arcanum
            .lookup(&identifier_node.name.lexeme, explicit_scope);
        match &symbol_type_rcref_opt {
            Some(symbol_type_rcref) => {
                let symbol_type = symbol_type_rcref.borrow();
                match &*symbol_type {
                    SymbolType::DomainVariable {
                        domain_variable_symbol_rcref,
                    } => {
                        scope = domain_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::StateParam {
                        state_param_symbol_rcref,
                    } => {
                        scope = state_param_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::StateVariable {
                        state_variable_symbol_rcref,
                    } => {
                        scope = state_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerVariable {
                        event_handler_variable_symbol_rcref,
                    } => {
                        scope = event_handler_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerParam {
                        event_handler_param_symbol_rcref,
                    } => {
                        scope = event_handler_param_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerLocalScope {
                        event_handler_local_scope_rcref,
                    } => {
                        scope = IdentifierDeclScope::None;
                    }
                    SymbolType::EnumDeclSymbolT {
                        enum_symbol_rcref,
                    } => {
                        scope = enum_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::LoopVar {
                        loop_variable_symbol_rcref,
                    } => {
                        scope = loop_variable_symbol_rcref.borrow().scope.clone();
                    }
                    SymbolType::EventHandlerScope {
                        event_handler_scope_symbol,
                    } => {
                        // this will be a lookup for a varible that clashes with
                        // the name of an event. Disregard.
                        // scope = loop_variable_symbol_rcref.borrow().scope.clone();
                    }
                    _ => {
                        // scope = IdentifierDeclScope::None;
                        let msg = &format!(
                            "Error - unknown scope identifier {}."
                            ,identifier_node.name );
                        self.error_at_current(msg);
                        return Err(ParseError::new(msg));

                       // return Err(ParseError::new(&format!("Error - unknown scope identifier {}.",identifier_node.name)));
                    }
                }
            }
            None => {}
        };

        if !self.is_building_symbol_table
            && *explicit_scope != IdentifierDeclScope::None
            && *explicit_scope != scope
        {
            let msg = &format!(
                "Identifier {} - invalid scope identifier.",
                identifier_node.name.lexeme
            );
            self.error_at_current(msg);
            return Err(ParseError::new(msg));
        }

        Ok(scope)
    }

    /* --------------------------------------------------------------------- */

    // method_call ->

    fn method_call(&mut self, identifer_node: IdentifierNode) -> Result<CallExprNode, ParseError> {
        let call_expr_list_node;
        match self.expr_list() {
            Ok(Some(ExprListT { expr_list_node })) => {
                // need to differentiate between regular expression lists and call expression lists
                // for formatting.
                call_expr_list_node = CallExprListNode::new(expr_list_node.exprs_t);
                //    call_expr_list_node = CallExprListT {call_expr_list_node};
            }
            Ok(Some(_)) | Ok(None) => return Err(ParseError::new("TODO")), // TODO: must return an ExprList
            Err(parse_error) => return Err(parse_error),
        }

        let method_call_expr_node = CallExprNode::new(identifer_node, call_expr_list_node, None);
        //        let method_call_expression_type = ExpressionType::MethodCallExprType {method_call_expr_node};
        Ok(method_call_expr_node)
    }

    /* --------------------------------------------------------------------- */

    // literal_expression -> '(' expression* ')'

    fn literal_expr(&mut self) -> Result<Option<LiteralExprNode>, ParseError> {
        // TODO: move this vec to the scanner
        let literal_tokens = vec![
            TokenType::SuperString,
            TokenType::String,
            TokenType::Number,
            TokenType::True,
            TokenType::False,
            TokenType::Null,
            TokenType::Nil,
        ];

        for literal_tok in literal_tokens {
            if self.match_token(&[literal_tok]) {
                return Ok(Some(LiteralExprNode::new(
                    literal_tok,
                    self.previous().lexeme.clone(),
                )));
            }
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // state_context ->

    fn state_context(
        &mut self,
        enter_args_opt: Option<ExprListNode>,
    ) -> Result<Option<StateContextType>, ParseError> {
        if self.match_token(&[TokenType::StateStackOperationPop]) {
            Ok(Some(StateContextType::StateStackPop {}))
        } else {
            // parse state ref e.g. '$S1'
            if !self.match_token(&[TokenType::State]) {
                return Err(ParseError::new("Missing $"));
            }

            if !self.match_token(&[TokenType::Identifier]) {
                return Err(ParseError::new("Missing state identifier"));
            }

            let state_id = self.previous();
            let name = state_id.lexeme.clone();

            // parse optional state ref expression list
            // '(' ')' | '(' expr ')'
            let mut state_ref_args_opt = None;
            if self.match_token(&[TokenType::LParen]) {
                match self.expr_list() {
                    Ok(Some(ExprListT { expr_list_node })) => {
                        state_ref_args_opt = Some(expr_list_node)
                    }
                    Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                    Err(parse_error) => return Err(parse_error),
                    Ok(None) => {} // continue
                }
            }

            let state_context_node =
                StateContextNode::new(StateRefNode::new(name), state_ref_args_opt, enter_args_opt);

            Ok(Some(StateContextType::StateRef { state_context_node }))
        }
    }

    /* --------------------------------------------------------------------- */

    // transition : exitArgs '->' enterArgs transitionLabel stateRef stateArgs

    fn transition(
        &mut self,
        exit_args_opt: Option<ExprListNode>,
    ) -> Result<Option<StatementType>, ParseError> {
        self.generate_transition_state = true;

        if self.is_action_context {
            let err_msg = format!("Transitions disallowed inside of actions.");
            self.error_at_current(&err_msg);
            let parse_error = ParseError::new(
                err_msg.as_str(),
            );
            return Err(parse_error);
        }
        let eh_rc_refcell = self.current_event_symbol_opt.as_ref().unwrap().clone();
        let evt_symbol = eh_rc_refcell.borrow();

        if evt_symbol.is_exit_msg {
            self.error_at_current("Transition disallowed in exit event handler.")
        }

        if exit_args_opt.is_some() {
            // need exit args generated
            self.generate_exit_args = true;
        }

        let mut enter_msg_with_enter_args: bool = false;
        let mut enter_args_opt: Option<ExprListNode> = None;
        let mut transition_label: Option<String> = None;

        // enterArgs: '(' ')' | '(' expr ')'
        if self.match_token(&[TokenType::LParen]) {
            if evt_symbol.is_enter_msg {
                enter_msg_with_enter_args = true;
            }
            // need enter args generated
            self.generate_enter_args = true;
            match self.expr_list() {
                Ok(Some(ExprListT { expr_list_node })) => enter_args_opt = Some(expr_list_node),
                Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {} // continue
            }
        }

        // transition label string
        if self.match_token(&[TokenType::String]) {
            transition_label = Some(self.previous().lexeme.clone());
        }

        // Transition dispatch
        // -> => $Next
        let mut forward_event = false;
        if self.match_token(&[TokenType::Dispatch]) {
            forward_event = true;
            if enter_msg_with_enter_args {
                self.error_at_current(
                    "Transition dispatch disallowed in enter message with enter event parameters.",
                )
            }
        }

        let state_context_t;
        match self.state_context(enter_args_opt) {
            Ok(Some(scn)) => state_context_t = scn,
            Ok(None) => return Err(ParseError::new("TODO")),
            Err(parse_error) => return Err(parse_error),
        }

        // this is so we can know to declare a StateContext at the
        // top of the event handler.
        self.event_handler_has_transition = true;

        Ok(Some(StatementType::TransitionStmt {
            transition_statement: TransitionStatementNode {
                target_state_context_t: state_context_t,
                exit_args_opt,
                label_opt: transition_label,
                forward_event,
            },
        }))
    }

    /* --------------------------------------------------------------------- */

    // change_state : '->>' change_state_label state_ref

    fn change_state(&mut self) -> Result<Option<StatementType>, ParseError> {
        self.generate_change_state = true;

        let mut label_opt: Option<String> = None;

        // change_state label string
        if self.match_token(&[TokenType::String]) {
            label_opt = Some(self.previous().lexeme.clone());
        }

        let state_context_t;
        match self.state_context(None) {
            Ok(Some(scn)) => state_context_t = scn,
            Ok(None) => return Err(ParseError::new("TODO")),
            Err(parse_error) => return Err(parse_error),
        }

        Ok(Some(StatementType::ChangeStateStmt {
            change_state_stmt: ChangeStateStatementNode {
                state_context_t,
                label_opt,
            },
        }))
    }

    /* --------------------------------------------------------------------- */

    // match_number_test -> '?#'  ('/' match_number_pattern  ('|' match_number_pattern)* '/' (statement* branch_terminator?) ':>')+ ':' (statement* branch_terminator?) '::'

    fn number_match_test(&mut self, expr_t: ExprType) -> Result<NumberMatchTestNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::NumberTest, "Expected '?#'.") {
            return Err(parse_error);
        }

        let mut conditional_branches: Vec<NumberMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.number_match_test_match_branch() {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        while self.match_token(&[TokenType::ElseContinue]) {
            match self.number_match_test_match_branch() {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                }
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' match_test_else_branch)?
        let mut else_branch_opt: Option<NumberMatchTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            else_branch_opt = Option::from(match self.number_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) =
            self.consume(TokenType::ColonColon, "Expected TestTerminator.")
        {
            return Err(parse_error);
        }

        Ok(NumberMatchTestNode::new(
            expr_t,
            conditional_branches,
            else_branch_opt,
        ))
    }

    /* --------------------------------------------------------------------- */

    // number_match_test ->  ('/' match_number '/' (statement* branch_terminator?) ':>')+  '::'

    fn number_match_test_match_branch(
        &mut self,
    ) -> Result<NumberMatchTestMatchBranchNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::ForwardSlash, "Expected '/'.") {
            return Err(parse_error);
        }

        let mut match_numbers = Vec::new();

        if !self.match_token(&[TokenType::Number]) {
            return Err(ParseError::new("TODO"));
        }

        //        let token = self.previous();
        let match_number_tok = self.previous();
        let match_pattern_number = match_number_tok.lexeme.clone();
        let number_match_pattern_node = NumberMatchTestPatternNode::new(match_pattern_number);
        match_numbers.push(number_match_pattern_node);

        while self.match_token(&[TokenType::Pipe]) {
            if !self.match_token(&[TokenType::Number]) {
                return Err(ParseError::new("TODO"));
            }

            //            let token = self.previous();
            let match_number_tok = self.previous();
            let match_pattern_number = match_number_tok.lexeme.clone();
            let number_match_pattern_node = NumberMatchTestPatternNode::new(match_pattern_number);
            match_numbers.push(number_match_pattern_node);
        }

        if let Err(parse_error) = self.consume(TokenType::ForwardSlash, "Expected '/'.") {
            return Err(parse_error);
        }
        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_t_opt) => Ok(NumberMatchTestMatchBranchNode::new(
                match_numbers,
                statements,
                branch_terminator_t_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // number_match_test_else_branch -> statements* branch_terminator?

    fn number_match_test_else_branch(
        &mut self,
    ) -> Result<NumberMatchTestElseBranchNode, ParseError> {
        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_opt) => Ok(NumberMatchTestElseBranchNode::new(
                statements,
                branch_terminator_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // match_enum_test -> '?:' '(' enum_type ')  ('/' match_enum_pattern  ('|' match_enum_pattern)* '/' (statement* branch_terminator?) ':>')+ ':' (statement* branch_terminator?) '::'

    fn enum_match_test(&mut self, expr_t: ExprType) -> Result<EnumMatchTestNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::EnumTest, "Expected '?:'.") {
            return Err(parse_error);
        }

        if let Err(parse_error) = self.consume(TokenType::LParen, "Expected '('.") {
            return Err(parse_error);
        }

        if !self.match_token(&[TokenType::Identifier]) {
            let err_msg = format!("Expected enum type name. Found {}.", self.previous().lexeme);
            self.error_at_current(&err_msg);
            let parse_error = ParseError::new(
                err_msg.as_str(),
            );
            return Err(parse_error);
        }

        let mut enum_type_name = String::new();
        let mut enum_symbol_rcref_opt = None;

        if !self.is_building_symbol_table {
        //     // semantic pass
        //

            enum_type_name = self.previous().lexeme.clone();
            let enum_symbol_t_rcref_opt = self.arcanum.lookup(enum_type_name.as_str(), &IdentifierDeclScope::DomainBlock);
            match enum_symbol_t_rcref_opt {
                None => {
                    let err_msg = &format!(
                        "Enumerated type '{}' does not exist.",
                        enum_type_name
                    );
                    self.error_at_current(err_msg);
                    let parse_error = ParseError::new(
                        err_msg.as_str(),
                    );
                    return Err(parse_error);
                }
                Some(symbol_t_rcref) => {
                    let symbol_t = symbol_t_rcref.borrow();
                    match &*symbol_t {
                        SymbolType::EnumDeclSymbolT {enum_symbol_rcref:enum_symbol_rcref_local} => {
                            // ok - enum type symbol exists
                            // let enum_symbol = enum_symbol_rcref.borrow();
                            enum_symbol_rcref_opt = Some(enum_symbol_rcref_local.clone());

                        }
                        _ => {
                            let err_msg = &format!(
                                "Enumerated type '{}' does not exist.",
                                enum_type_name
                            );
                            self.error_at_current(err_msg);
                            let parse_error = ParseError::new(
                                err_msg.as_str(),
                            );
                            return Err(parse_error);
                        }
                    }
                }
            }

        }


        if let Err(parse_error) = self.consume(TokenType::RParen, "Expected ')'.") {
            return Err(parse_error);
        }

        let mut conditional_branches: Vec<EnumMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.enum_match_test_match_branch(&enum_symbol_rcref_opt) {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        while self.match_token(&[TokenType::ElseContinue]) {
            match self.enum_match_test_match_branch(&enum_symbol_rcref_opt) {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                }
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' match_test_else_branch)?
        let mut else_branch_opt: Option<EnumMatchTestElseBranchNode> = None;
        if self.match_token(&[TokenType::Colon]) {
            else_branch_opt = Option::from(match self.enum_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) =
        self.consume(TokenType::ColonColon, "Expected TestTerminator.")
        {
            return Err(parse_error);
        }

        Ok(EnumMatchTestNode::new(
            expr_t,
            conditional_branches,
            else_branch_opt,
        ))
    }

    /* --------------------------------------------------------------------- */

    // enum_match_test ->  ('/' match_enum '/' (statement* branch_terminator?) ':>')+  '::'

    fn enum_match_test_match_branch(
        &mut self,
        enum_symbol_rcref_opt:&Option<Rc<RefCell<EnumSymbol>>>
    ) -> Result<EnumMatchTestMatchBranchNode, ParseError> {
        if let Err(parse_error) = self.consume(TokenType::ForwardSlash, "Expected '/'.") {
            return Err(parse_error);
        }

        let mut match_enums = Vec::new();

        if !self.match_token(&[TokenType::Identifier]) {
            return Err(ParseError::new("TODO"));
        }

        let match_enum_tok = self.previous();
        let match_pattern_enum = match_enum_tok.lexeme.clone();

        if !self.is_building_symbol_table {
            let mut found_match = false;
            let a = enum_symbol_rcref_opt.as_ref().unwrap().as_ref();
            let c = a.clone();
            let b = c.borrow();
            let c = b.ast_node_opt.as_ref().unwrap();
            let d = c.borrow();
            for e in &d.enums {
                if match_pattern_enum == e.name {
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                let err_msg = format!("'{}' is not an enumeration in enum type {}",match_pattern_enum, d.name );
                self.error_at_current(&err_msg);
                return Err(ParseError::new(&err_msg));
            }
        }

        let match_enum_tok = self.previous();
        let match_pattern_enum = match_enum_tok.lexeme.clone();
        let enum_match_pattern_node = EnumMatchTestPatternNode::new(match_pattern_enum);
        match_enums.push(enum_match_pattern_node);

        while self.match_token(&[TokenType::Pipe]) {
            if !self.match_token(&[TokenType::Enum]) {
                return Err(ParseError::new("TODO"));
            }

            //            let token = self.previous();
            let match_enum_tok = self.previous();
            let match_pattern_enum = match_enum_tok.lexeme.clone();
            let enum_match_pattern_node = EnumMatchTestPatternNode::new(match_pattern_enum);
            match_enums.push(enum_match_pattern_node);
        }

        if let Err(parse_error) = self.consume(TokenType::ForwardSlash, "Expected '/'.") {
            return Err(parse_error);
        }
        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_t_opt) => Ok(EnumMatchTestMatchBranchNode::new(
                match_enums,
                statements,
                branch_terminator_t_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // enum_match_test_else_branch -> statements* branch_terminator?

    fn enum_match_test_else_branch(
        &mut self,
    ) -> Result<EnumMatchTestElseBranchNode, ParseError> {
        let statements = self.statements();
        let result = self.branch_terminator();
        match result {
            Ok(branch_terminator_opt) => Ok(EnumMatchTestElseBranchNode::new(
                statements,
                branch_terminator_opt,
            )),
            Err(parse_error) => Err(parse_error),
        }
    }

}


use super::ast::*;
use super::ast::DeclOrStmtType;
use super::scanner::*;
use super::scanner::TokenType::*;
use super::ast::ExprType;
use std::fmt;
use super::ast::TerminatorType::{Return, Continue};
use super::ast::ExprType::*;
use super::ast::ExprStmtType::*;
use super::symbol_table::*;
use super::ast::MessageType::{AnyMessage, CustomMessage};
use downcast_rs::__std::cell::RefCell;
use std::rc::Rc;
use super::symbol_table::SymbolType::*;
use super::ast::AssignmentExprNode;
use crate::frame_c::utils::{SystemHierarchy};
use std::collections::HashMap;

pub struct ParseError {
    // TODO:
    pub error:String,
}

impl ParseError {
    fn new(msg:&str) -> ParseError {

        ParseError {
            error:String::from(msg),
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
    tokens:&'a Vec<Token>,
    comments:&'a mut Vec<Token>,
    current:usize,
    current_token:String,
    current_tok_ref:&'a Token,
    processed_tokens:String,
//    reset_pos:usize,
    is_building_symbol_table:bool,
    arcanum:Arcanum,
    state_name_opt:Option<String>,
    had_error:bool,
    panic_mode:bool,
    errors:String,
    last_sync_token_idx:usize,
    system_hierarchy_opt:Option<SystemHierarchy>,
    is_parsing_rhs:bool,
    event_handler_has_transition:bool,
    pub generate_exit_args:bool,
    pub generate_state_context:bool,
    pub generate_state_stack:bool,
    pub generate_change_state:bool,
    pub generate_transition_state:bool,


}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens:&'a Vec<Token>, comments:&'a mut Vec<Token>, is_building_symbol_table:bool,
                      arcanum:Arcanum) -> Parser<'a> {

        Parser {
            tokens,
            comments,
            current: 0,
            last_sync_token_idx:0,
            current_token:String::from(""),
            processed_tokens:String::from(""),
            is_building_symbol_table,
            arcanum,
            state_name_opt:None,
            had_error:false,
            panic_mode:false,
            errors:String::new(),
            current_tok_ref:&tokens[0],
            system_hierarchy_opt:None,
            is_parsing_rhs:false,
            event_handler_has_transition:false,
            generate_exit_args:false,
            generate_state_context:false,
            generate_state_stack:false,
            generate_change_state:false,
            generate_transition_state:false,
        }
    }

    /* --------------------------------------------------------------------- */

    pub fn parse<'b>(&'b mut self) -> SystemNode {
        self.system()
    }

    /* --------------------------------------------------------------------- */

    pub fn get_arcanum(self) -> Arcanum {
        self.arcanum
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

    fn match_token(&mut self, token_types:&Vec<TokenType>) -> bool {
        // cache off comments
        while self.check(SingleLineCommentTok) ||
            self.check(MultiLineCommentTok) {
            self.comments.push(self.peek().clone());
            self.advance();
        }

        if self.check(ErrorTok) {
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

                return true
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
            self.processed_tokens.push_str(" ");
            self.processed_tokens.push_str(&self.peek().lexeme.clone());
//            println!("Current token = {:?}",self.peek());
        }

        self.previous()
    }

    /* --------------------------------------------------------------------- */

    fn check(&self, token_type:TokenType) -> bool {

        let t = self.peek();
        if token_type == t.token_type {
            return true;
        }

        return false;
    }

    /* --------------------------------------------------------------------- */

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::EofTok => true,
            _ => false,
        }
    }

    /* --------------------------------------------------------------------- */

    fn peek(&self) -> & Token {
        &self.tokens[self.current]
    }

    /* --------------------------------------------------------------------- */

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /* --------------------------------------------------------------------- */

    fn consume(&mut self, token_type:TokenType, message:&str) -> Result<&Token,ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        self.error_at_current(message);
        Err(ParseError::new("TODO"))
    }

    /* --------------------------------------------------------------------- */

    fn error_at_current(&mut self, message:&str) {
        self.error_at(&self.tokens[self.current], message);
    }

    /* --------------------------------------------------------------------- */

    fn error_at_previous(&mut self, message:&str)  {
        self.error_at(&self.tokens[self.current - 1], message);
    }

    /* --------------------------------------------------------------------- */

    // TODO: put the message in the ParseError
    fn error_at(&mut self, token:&Token, message:&str) {

        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        self.had_error = true;

        let mut error_msg = format!("[line {}] Error",token.line);

         match token.token_type {
             TokenType::EofTok => error_msg.push_str(&format!(" at end")),
             TokenType::ErrorTok => error_msg.push_str(&format!(" at '{}'",token.lexeme)),
             _ => error_msg.push_str(&format!(" at '{}'",token.lexeme))
         }

        self.errors.push_str( &format!("{} : {}\n", error_msg, message));

//        println!("{} : {}", error_msg, message);
        // TODO:?
        //       ParseError::new( /* error_msg */ )
    }

    /* --------------------------------------------------------------------- */

    fn synchronize(&mut self,sync_tokens:&Vec<TokenType>) -> bool {
        self.panic_mode = false;

        if self.is_at_end() {
            return false;
        }

        // in case not advancing
        if self.last_sync_token_idx == self.current {
            self.advance();
        }

        self.last_sync_token_idx = self.current;

        while self.peek().token_type != TokenType::EofTok {
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

    fn follows(&self,token:&Token,follows_vec:&Vec<TokenType>) -> bool {
        for follows_token_type in follows_vec {
            if *follows_token_type == token.token_type {
                return true;
            }
        }

        let vec_comments = &vec![SingleLineCommentTok,MultiLineCommentTok];
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

        if self.match_token(&vec![EofTok]) {
            self.error_at_current("Empty system.");
            return SystemNode::new(String::from("error"),
                                   header,
                                   None,
                                   None,
                                   None,
                                   None,
                                   None,
                                   0)
        }

        // Parse free-form header ```whatever```
        if self.match_token(&vec![ThreeTicksTok]) {
            while self.match_token(&vec![SuperStringTok]) {
                let tok = self.previous();
                header.push_str(&*tok.lexeme.clone());
            }
            if let Err(_) =  self.consume(TokenType::ThreeTicksTok, "Expected '```'.") {
                self.error_at_current("Expected closing ```.");
                let sync_tokens = &vec![SystemTok];
                self.synchronize(sync_tokens);
            }
        }

        let attributes_opt = match self.attributes() {
            Ok(attributes_opt) => attributes_opt,
            Err(_parse_error) => {
                None
            },
        };

        // TODO: Error handling
        if !self.match_token(&vec![SystemTok]) {
            self.error_at_current("Expected #.");
            let sync_tokens = &vec![IdentifierTok];
            self.synchronize(sync_tokens);
        }
        if !self.match_token(&vec![IdentifierTok]) {
            self.error_at_current("Expected system identifer.");
            let sync_tokens = &vec![InterfaceBlockTok, MachineBlockTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
            self.synchronize(sync_tokens);
        }

        let id = self.previous();
        let system_name = id.lexeme.clone();

        self.system_hierarchy_opt = Some(SystemHierarchy::new(system_name.clone()));

        if self.is_building_symbol_table {
//           let st = self.get_current_symtab();
           let system_symbol = SystemSymbol::new(system_name.clone());
           let x = Rc::new(RefCell::new(system_symbol));
            // TODO: it would be better to find some way to bake the identifier scope into the SystemScope type
            self.arcanum.enter_scope(ParseScopeType::SystemScope {system_symbol:x});
        } else {
            self.arcanum.set_parse_scope(&system_name);
        }

        if self.match_token(&vec![InterfaceBlockTok]) {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            let x = self.interface_block();
            interface_block_node_opt = Option::Some(x);
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
        }

        if self.match_token(&vec![MachineBlockTok]) {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            machine_block_node_opt = Option::Some(self.machine_block());
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
        }

        if self.match_token(&vec![ActionsBlockTok]) {
            actions_block_node_opt = Option::Some(self.actions_block());
        }

        if self.match_token(&vec![DomainBlockTok]) {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            domain_block_node_opt = Option::Some(self.domain_block());
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
        }

        if !self.match_token(&vec![SystemEndTok]) {
            self.error_at_current("Expected ##.");
        }

        let line = self.previous().line;

        self.arcanum.exit_parse_scope();

        SystemNode::new(system_name,
                        header,
                        attributes_opt,
                        interface_block_node_opt,
                        machine_block_node_opt,
                        actions_block_node_opt,
                        domain_block_node_opt,
                        line)
    }

    /* --------------------------------------------------------------------- */


    fn attributes(&mut self) -> Result<Option<HashMap<String,AttributeNode>>,ParseError> {
        let mut attributes:HashMap<String,AttributeNode> = HashMap::new();

        loop {
            if self.match_token(&vec![InnerAttributeTok]) {
                // not supported yet
                let parse_error = ParseError::new("Found '#![' token - inner attribute syntax not currently supported.");
                return Err(parse_error);
            } else if self.match_token(&vec![OuterAttributeTok]) {
                let attribute_node = match self.attribute() {
                    Ok(attribute_node) => {
                        attribute_node
                    },
                    Err(err) => {
                        return Err(err);
                    },
                };
                attributes.insert(attribute_node.name.clone(),attribute_node);
                if let Err(parse_error) =  self.consume(RBracketTok, "Expected ']'.") {
                    return Err(parse_error);
                }
            } else {
                break;
            }
        }

        if attributes.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(attributes))
        }
    }

    /* --------------------------------------------------------------------- */

    fn attribute(&mut self) -> Result<AttributeNode,ParseError> {
        let name ;
        let value;

        if self.match_token(&vec![IdentifierTok]) {
           name = self.previous().lexeme.clone();
        } else {
            self.error_at_current("Expected attribute name.");
            let parse_error = ParseError::new("TODO");
            return Err(parse_error);

        }
        if let Err(_) = self.consume(TokenType::EqualsTok, "Expected '('") {
            self.error_at_current("Expected '='.");
            let parse_error = ParseError::new("TODO");
            return Err(parse_error);
        }
        if self.match_token(&vec![StringTok]) {
            value = self.previous().lexeme.clone();
        } else {
            self.error_at_current("Expected attribute value.");
            let parse_error = ParseError::new("TODO");
            return Err(parse_error);

        }
        return Ok(AttributeNode::new(name,value));
    }

    /* --------------------------------------------------------------------- */

    fn interface_block(&mut self) -> InterfaceBlockNode {

        if self.is_building_symbol_table {
            let interface_symbol = Rc::new(RefCell::new(InterfaceBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::InterfaceBlockScope { interface_block_scope_symbol_rcref: interface_symbol });
        } else {
            self.arcanum.set_parse_scope(InterfaceBlockScopeSymbol::scope_name());
        }

        let x = &self.arcanum.current_symtab;
        self.arcanum.debug_print_current_symbols(x.clone());

        let mut interface_methods = Vec::new();

        // NOTE: this loop peeks() ahead and then interface_method() consumes
        // the identifier. Not sure if this is the best way.

        while self.match_token(&vec![TokenType::IdentifierTok]) {
            match self.interface_method() {
                Ok(interface_method_node) => {
                    interface_methods.push(interface_method_node);
                },
                Err(_parse_error) => {
                    let sync_tokens = &vec![IdentifierTok, MachineBlockTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
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

    fn interface_method(&mut self) -> Result<Rc<RefCell<InterfaceMethodNode>>,ParseError> {

        let name = self.previous().lexeme.clone();

        let mut params_opt:Option<Vec<ParameterNode>> = Option::None;
        let mut return_type_opt:Option<TypeNode> = Option::None;
        let mut alias_opt:Option<MessageNode> = Option::None;

        if self.match_token(&vec![TokenType::LBracketTok]) {
            match self.parameters() {
                Ok(Some(parameters)) => params_opt = Some(parameters),
                Ok(None) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
            }
        }

        // Parse return type
        if self.match_token(&vec![TokenType::ColonTok]) {

            match self.type_decl() {
                Ok(type_node) => return_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        // Parse alias
        if self.match_token(&vec![TokenType::AtTok]) {
            if let Err(_) = self.consume(TokenType::LParenTok, "Expected '('") {
                self.error_at_current("Expected '('.");
                let sync_tokens = &vec![PipeTok];
                self.synchronize(sync_tokens);
            }

            match self.message() {
                Ok(MessageType::CustomMessage { message_node }) => alias_opt = Some(message_node),
                Ok(AnyMessage {..}) => {
                    self.error_at_previous("Expected message, found '||*");
                    let sync_tokens = &vec![RParenTok, MachineBlockTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                    self.synchronize(sync_tokens);
                },
                Err(err) => return Err(err),
            }

            if let Err(_) = self.consume(TokenType::RParenTok, "Expected ')'") {
                let sync_tokens = &vec![IdentifierTok, MachineBlockTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                self.synchronize(sync_tokens);
            }
        }

        let mut param_symbols_opt = None;
        match &params_opt {
            Some(param_nodes) => {
                let mut vec = Vec::new();
                for param_node in param_nodes {
                    let param_symbol = ParameterSymbol::new(param_node.param_name.clone(),param_node.param_type_opt.clone(),IdentifierDeclScope::None);
                    vec.push(param_symbol);
                }
                param_symbols_opt = Some(vec);
            },
            None => {},
        }

        // if the alias exists, that is the name of the event message.
        // if not, the interface method name becomes the event message name.

        let msg = match &alias_opt {
            Some(alias) => alias.name.clone(),
            None => name.clone(),
        };

        // get or create the event symbol for the message we found
        let event_symbol_rcref ;
        match self.arcanum.get_event(&*msg,&self.state_name_opt) {
            Some(_existing_event_symbol_rc_ref) => {
                // found message
                // event_symbol_rcref = existing_event_symbol_rc_ref.clone();
            },
            None => {
                let event_symbol = EventSymbol::new(&self.arcanum.symbol_config
                                                    ,&msg
                                                    ,Some(name.clone())
                                                    ,param_symbols_opt
                                                    ,return_type_opt.clone()
                                                    ,self.state_name_opt.clone());
                event_symbol_rcref = Rc::new(RefCell::new(event_symbol));
                self.arcanum.declare_event(Rc::clone(&event_symbol_rcref));

                // This is the first time we are seeing this event.
                // Set flag so parameters and return type are added to event symbol
                // during this parse.
 //               is_declaring_event = true;
            }
        }

        let interface_method_node = InterfaceMethodNode::new(name.clone(), params_opt, return_type_opt, alias_opt);
        let interface_method_rcref = Rc::new(RefCell::new(interface_method_node));

        if self.is_building_symbol_table {
            let mut interface_method_symbol = InterfaceMethodSymbol::new(name.clone());
            // TODO: note what is being done. We are linking to the AST node generated in the syntax pass.
            // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
            // we could copy this information out of the node and into the symbol.
            interface_method_symbol.set_ast_node(Rc::clone(&interface_method_rcref));
            let interface_method_symbol_rcref = Rc::new(RefCell::new(interface_method_symbol));
            let interface_method_symbol_t = InterfaceMethodSymbolT { interface_method_symbol_rcref:interface_method_symbol_rcref.clone() };
            // TODO: just insert into arcanum directly
            self.arcanum.current_symtab.borrow_mut().insert_symbol(&interface_method_symbol_t);
        } else {
            // link action symbol to action declaration node
        }

        Ok(interface_method_rcref)
    }


    /* --------------------------------------------------------------------- */


    fn type_decl(&mut self) -> Result<TypeNode,ParseError> {
        let mut is_reference = false;

        if self.match_token(&vec![TokenType::SuperStringTok]) {
            let id = self.previous();
            let type_str = id.lexeme.clone();
            Ok(TypeNode::new(true,false, type_str))
        } else {
            if self.match_token(&vec![TokenType::AndTok]) {
                is_reference = true
            }
            if !self.match_token(&vec![TokenType::IdentifierTok]) {
                self.error_at_current("Expected return type name.");
                return Err(ParseError::new("TODO"));
            }

            let id = self.previous();
            let type_str = id.lexeme.clone();

            Ok(TypeNode::new(false,is_reference, type_str))
        }

    }

    /* --------------------------------------------------------------------- */

    // message => '|' ( identifier | string | '>' | '>>' | '>>>' | '<' | '<<' | '<<<' ) '|'

    fn message(&mut self) -> Result<MessageType,ParseError> {

        let message_node;

        if self.peek().token_type == AtTok {
            if let Err(parse_error) =  self.consume(AtTok, "Expected '@'.") {
                return Err(parse_error);
            }
        }
        if self.match_token(&vec![AnyMessageTok]) {
            let tok = self.previous();

            return Ok(MessageType::AnyMessage {line:tok.line});
        }
        if !self.match_token(&vec![TokenType::PipeTok]) {
            self.error_at_previous("Expected '|'.");
            return Err(ParseError::new("TODO"));
        }

        let tt = self.peek().token_type;
        match tt {
            IdentifierTok | StringTok |
            GTTok | GTx2Tok | GTx3Tok |
            LTTok | LTx2Tok | LTx3Tok =>
                message_node = self.create_message_node(tt),
            _ => {
                self.error_at_current("Expected '|'");
                return Err(ParseError::new("TODO"));

            }
        }

        if let Err(parse_error) =  self.consume(TokenType::PipeTok, "Expected '|'.") {
            return Err(parse_error);
        }

        Ok(MessageType::CustomMessage {message_node})
    }

    /* --------------------------------------------------------------------- */

    fn create_message_node(&mut self, token_type:TokenType) -> MessageNode {
        self.match_token(&vec![token_type]);
        let id = self.previous();
        let name = id.lexeme.clone();

        MessageNode::new(name,id.line)
    }

    /* --------------------------------------------------------------------- */

    // Just get the parameters here. The calling routine will either build or
    // validate with the EventSymbol.

    // TODO: consider removing ParseError as it is currently not returned.
    fn parameters(&mut self) -> Result<Option<Vec<ParameterNode>>,ParseError> {

        let mut parameters:Vec<ParameterNode> = Vec::new();

        while !self.match_token(&vec![TokenType::RBracketTok]) {

             match self.parameter() {
                Ok(parameter_opt) => {
                    match parameter_opt {
                        Some(parameter_node) => {
                            parameters.push(parameter_node);
                        },
                        None => {
                            break;
                        }
                    }
                },
                Err(_parse_error) => {
                    let sync_tokens = &vec![IdentifierTok, ColonTok, RBracketTok, MachineBlockTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                    self.synchronize(sync_tokens);
                    if !self.follows(self.peek(),&vec![IdentifierTok,ColonTok,RBracketTok]) {
                        break;
                    }
                }
            }

        }

        if parameters.len() > 0 {
            return Ok(Some(parameters));
        }

        return Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // parameter -> param_name ( ':' param_type )?

    fn parameter(&mut self) -> Result<Option<ParameterNode>,ParseError> {

        if !self.match_token(&vec![TokenType::IdentifierTok]) {
            self.error_at_current("Expected parameter name.");
            return Err(ParseError::new("TODO"));
        }

        let id = self.previous();
        let param_name = id.lexeme.clone();

        let mut param_type_opt:Option<TypeNode> = None;

        if self.match_token(&vec![TokenType::ColonTok]) {
            match self.type_decl() {
                Ok(type_node) => param_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }

            // let id = self.previous();
            // let param_type = id.lexeme.clone();

            //param_type_opt = Some(param_type);
        }

        let scope = self.arcanum.get_current_identifier_scope();
        Ok(Some(ParameterNode::new(param_name,param_type_opt,scope)))
    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn machine_block(&mut self) -> MachineBlockNode {

        if self.is_building_symbol_table {
            let machine_symbol = Rc::new(RefCell::new(MachineBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::MachineBlockScope { machine_scope_symbol_rcref: machine_symbol });
        } else {
            self.arcanum.set_parse_scope(MachineBlockScopeSymbol::scope_name());
        }

        let mut states = Vec::new();

        while self.match_token(&vec![TokenType::StateTok]) {
            match self.state() {
                Ok(state_rcref) => {
                    states.push(state_rcref);
                },
                Err(_) => {
                    self.error_at_current("Error parsing Machine Block.");
                    let sync_tokens = &vec![StateTok];
                    if self.synchronize(sync_tokens) {
                        continue;
                    } else {
                        let sync_tokens = &vec![ActionsBlockTok, DomainBlockTok, SystemEndTok];
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
            self.arcanum.enter_scope(ParseScopeType::ActionsBlockScope { actions_block_scope_symbol_rcref: actions_block_scope_symbol });
        }

        let mut actions = Vec::new();

        while self.match_token(&vec![IdentifierTok]) {
            match self.action_decl() {
                Ok(action_decl_node) =>  actions.push(action_decl_node),
                Err(_) => {

                }
            }
        }

        if self.is_building_symbol_table {
            self.arcanum.exit_parse_scope();
        }

        ActionsBlockNode::new(actions)
    }

    /* --------------------------------------------------------------------- */

    fn action_decl(&mut self) -> Result<Rc<RefCell<ActionNode>>,ParseError> {

        let action_name = self.previous().lexeme.clone();

        let mut params:Option<Vec<ParameterNode>> = Option::None;

        if self.match_token(&vec![LBracketTok]) {
            params = match self.parameters() {
                Ok(Some(parameters)) =>  Some(parameters),
                Ok(None) =>  None,
                Err(parse_error) => return Err(parse_error),
            }
        }

        let mut type_opt:Option<TypeNode> = None;

        if self.match_token(&vec![ColonTok]) {

            match self.type_decl() {
                Ok(type_node) => type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        let mut code_opt:Option<String> = None;

        if self.match_token(&vec![OpenBraceTok]) {

            if self.match_token(&vec![SuperStringTok]) {
                let token = self.previous();
                code_opt = Some(token.lexeme.clone());
            }

            if let Err(parse_error) =  self.consume(CloseBraceTok, "Expected '}'.") {
                return Err(parse_error);
            }
        }

        let action_decl_node = ActionNode::new(action_name.clone(), params, type_opt, code_opt);
        let action_decl_rcref = Rc::new(RefCell::new(action_decl_node));

        if self.is_building_symbol_table {
            let s = action_name.clone();
            let mut action_decl_symbol = ActionDeclSymbol::new(s);
            // TODO: note what is being done. We are linking to the AST node generated in the syntax pass.
            // This AST tree is otherwise disposed of. This may be fine but feels wrong. Alternatively
            // we could copy this information out of the node and into the symbol.
            action_decl_symbol.set_ast_node(Rc::clone(&action_decl_rcref));
            let action_decl_symbol_rcref = Rc::new(RefCell::new(action_decl_symbol));
            let action_decl_symbol_t = ActionDeclSymbolT { action_decl_symbol_rcref };
            // TODO: just insert into arcanum directly
            self.arcanum.current_symtab.borrow_mut().insert_symbol(&action_decl_symbol_t);
        } else {
            // link action symbol to action declaration node
        }

        Ok(action_decl_rcref)

    }

    /* --------------------------------------------------------------------- */

    // TODO: Return result
    fn domain_block(&mut self) -> DomainBlockNode {

        if self.is_building_symbol_table {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            let domain_symbol = Rc::new(RefCell::new(DomainBlockScopeSymbol::new()));
            self.arcanum.enter_scope(ParseScopeType::DomainBlockScope { domain_block_scope_symbol_rcref: domain_symbol });
        } else {
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            self.arcanum.set_parse_scope(DomainBlockScopeSymbol::scope_name());
        }

        let mut domain_variables = Vec::new();

        while self.match_token(&vec![VarTok, ConstTok]) {
            match self.variable_decl(IdentifierDeclScope::DomainBlock) {
                Ok(domain_variable_node) =>  domain_variables.push(domain_variable_node),
                Err(_parse_err) => {
                    let sync_tokens = &vec![VarTok, ConstTok, SystemEndTok];
                    self.synchronize(sync_tokens);
                },
            }
        }

        self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
        self.arcanum.exit_parse_scope();

        DomainBlockNode::new(domain_variables)
    }

    //* --------------------------------------------------------------------- *//

    fn variable_decl(&mut self,identifier_decl_scope:IdentifierDeclScope) -> Result<Rc<RefCell<VariableDeclNode>>,ParseError> {

        let is_constant = match self.previous().token_type {
            VarTok => false,
            ConstTok => true,
            _ => return Err(ParseError::new("TODO")),
        };

        let name = match self.match_token(&vec![IdentifierTok]) {
            false => {
                self.error_at_current("Expected declaration identifier");
                return Err(ParseError::new("TODO"))
            },
            true => self.previous().lexeme.clone()
        };


        let mut type_node_opt:Option<TypeNode> = None;

        if self.match_token(&vec![ColonTok]) {
            // if !self.match_token(&vec![TokenType::IdentifierTok]) {
            //     self.error_at_previous("Expected parameter type.");
            //     return Err(ParseError::new("TODO"));
            // }
            //
            // let type_name = self.previous().lexeme.clone();
            //
            // type_opt = Some(type_name);
            match self.type_decl() {
                Ok(type_node) => type_node_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
        }

        let initializer_expr_t_opt;

        if self.match_token(&vec![EqualsTok]) {

            match self.equality() {
                Ok(Some(LiteralExprT {literal_expr_node}))
                    => initializer_expr_t_opt = Some(LiteralExprT {literal_expr_node}),
                Ok(Some(VariableExprT { var_node: id_node }))
                    => initializer_expr_t_opt = Some(VariableExprT { var_node: id_node }),
                Ok(Some(ActionCallExprT {action_call_expr_node}))
                // TODO this may be dead code. CallChainLiteralExprT may do it all
                => initializer_expr_t_opt = Some(ActionCallExprT {action_call_expr_node}),
                Ok(Some(ExprListT {expr_list_node}))
                    => initializer_expr_t_opt = Some(ExprListT {expr_list_node}),
                Ok(Some(CallChainLiteralExprT { call_chain_expr_node }))
                    => initializer_expr_t_opt = Some(CallChainLiteralExprT {call_chain_expr_node}),
                Ok(Some(UnaryExprT { unary_expr_node }))
                    => initializer_expr_t_opt = Some(UnaryExprT {unary_expr_node}),
                Ok(Some(BinaryExprT { binary_expr_node }))
                    => initializer_expr_t_opt = Some(BinaryExprT {binary_expr_node}),
                Ok(Some(FrameEventExprT { frame_event_part }))
                    => initializer_expr_t_opt = Some(FrameEventExprT {frame_event_part}),
                _ => {
                    self.error_at_current("Unexpected assignment expression value.");
                    return Err(ParseError::new("TODO"))
                },
            }
        } else {
            // All variables should be initialized to something.
            self.error_at_current("Expected '='. All variables must be initialized.");
            return Err(ParseError::new("TODO"));
        }

        let variable_decl_node = VariableDeclNode::new(name.clone(), type_node_opt.clone(), is_constant, initializer_expr_t_opt,identifier_decl_scope.clone());
        let variable_decl_node_rcref = Rc::new(RefCell::new(variable_decl_node));

        if self.is_building_symbol_table { // syntactic pass
            // add variable to current symbol table
            let scope = self.arcanum.get_current_identifier_scope();
            let variable_symbol = VariableSymbol::new(name.clone(),type_node_opt,scope);
            let variable_symbol_rcref = Rc::new(RefCell::new(variable_symbol));
            let variable_symbol_t = match identifier_decl_scope {
                IdentifierDeclScope::DomainBlock => SymbolType::DomainVariableSymbolT { domain_variable_symbol_rcref: variable_symbol_rcref },
                IdentifierDeclScope::StateVar => SymbolType::StateVariableSymbolT { state_variable_symbol_rcref: variable_symbol_rcref },
                IdentifierDeclScope::EventHandlerVar => SymbolType::EventHandlerVariableSymbolT { event_handler_variable_symbol_rcref: variable_symbol_rcref },
                _ => return Err(ParseError::new("Unrecognized variable scope."))
            };
            // TODO: make current_symtab private
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            self.arcanum.current_symtab.borrow_mut().insert_symbol(&variable_symbol_t);
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());

        } else { // semantic pass

            // TODO
            self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
            let x = self.arcanum.lookup(&name,&IdentifierDeclScope::None);
            let y = x.unwrap();
            let z = y.borrow();
            match &*z {

                DomainVariableSymbolT {domain_variable_symbol_rcref} => {
                    domain_variable_symbol_rcref.borrow_mut().ast_node = Some(variable_decl_node_rcref.clone());
                },
                StateVariableSymbolT {state_variable_symbol_rcref} => {
//                    let a = state_variable_symbol_rcref.borrow();
                    state_variable_symbol_rcref.borrow_mut().ast_node = Some(variable_decl_node_rcref.clone());
                },
                EventHandlerVariableSymbolT {event_handler_variable_symbol_rcref} => {
                    event_handler_variable_symbol_rcref.borrow_mut().ast_node = Some(variable_decl_node_rcref.clone());
                },
                _ => {
                    return Err(ParseError::new("Unrecognized variable scope."))
                }
            }
            // now need to keep current_symtab when in semantic parse phase and link to
            // ast nodes as necessary.
        }


        Ok(variable_decl_node_rcref)

    }

    /* --------------------------------------------------------------------- */

    // TODO return result
//    fn state(&mut self) -> Rc<RefCell<StateNode>> {
    fn state(&mut self) -> Result<Rc<RefCell<StateNode>>,ParseError> {

        let line = self.previous().line;

        // TODO
        if !self.match_token(&vec![TokenType::IdentifierTok]) {

            // error message and synchronize
            self.error_at_current("Expected state name.");
            let sync_tokens = &vec![StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
            self.synchronize(sync_tokens);

            let state_node = StateNode::new(String::from("error"),
                                            None,
                                            None,
                                            Option::None,
                                            Vec::new(),
                                            Option::None,
                                            Option::None,
                                            None,
                                            0);
            let state_node_rcref = Rc::new(RefCell::new(state_node));
            return Ok(state_node_rcref);
        }
        let id = self.previous();
        let state_name = id.lexeme.clone();

        self.state_name_opt = Some(state_name.clone());

        let state_symbol_rcref;
        if self.is_building_symbol_table {
            if self.arcanum.get_state(&state_name).is_some() {
                self.error_at_previous(&format!("Duplicate state name {}.",&state_name));
            }
            let state_symbol = StateSymbol::new(&state_name, self.arcanum.get_current_symtab());
            state_symbol_rcref = Rc::new(RefCell::new(state_symbol));
            self.arcanum.enter_scope(ParseScopeType::StateScope{state_symbol: state_symbol_rcref.clone()});
        } else {
            self.arcanum.set_parse_scope(&state_name);
            state_symbol_rcref = self.arcanum.get_state(&state_name).unwrap();
        }

        // parse state parameters e.g. $S1[x]
     //   let params:Option<Vec<ParameterNode>>
        let mut pop_state_params_scope = false;
        let mut params_opt = None;
        if self.match_token(&vec![TokenType::LBracketTok]) {
            // generate StateContext mechanism for state parameter support
            self.generate_state_context = true;
            match self.parameters() {
                Ok(Some(parameters)) => {
                    pop_state_params_scope = true;
                    if self.is_building_symbol_table {
                        match self.arcanum.get_state(&state_name) {
                            Some(state_symbol) => {
                                let state_params_scope_symbol = StateParamsScopeSymbol::new();
                                let state_params_scope_symbol_rcref = Rc::new(RefCell::new(state_params_scope_symbol));
                                self.arcanum.enter_scope(ParseScopeType::StateParamsScope { state_params_scope_symbol_rcref });
                                for param in &parameters {
                                    let scope = self.arcanum.get_current_identifier_scope();
                                    let x = state_symbol.borrow_mut().add_parameter(param.param_name.clone(), param.param_type_opt.clone(), scope);
                                    self.arcanum.insert_symbol(x);
                                }
                            },
                            None => {
                                return Err(ParseError::new(&format!("Fatal error: unable to find state {}.",state_name.clone())));

                            },
                        }
                    } else {
                        self.arcanum.set_parse_scope(StateParamsScopeSymbol::scope_name());
                    }
                    params_opt = Some(parameters);
                },
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {},
            }

        }

        let mut dispatch_opt:Option<DispatchNode> = None;

        // Dispatch clause.
        // '=>' '$' state_id
        if self.match_token(&vec![TokenType::DispatchTok]) {
            match self.consume(TokenType::StateTok, "Expected '$'") {
                Ok(_) => {
                    if self.match_token(&vec![TokenType::IdentifierTok]) {
                        let id = self.previous();
                        let target_state_name = id.lexeme.clone();

                        let target_state_ref = StateRefNode::new(target_state_name);
                        dispatch_opt = Some(DispatchNode::new(target_state_ref,id.line));
                    } else {
                        self.error_at_current("Expected dispatch target state identifier.");
                        let sync_tokens = &vec![PipeTok, StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                        self.synchronize(sync_tokens);
                    }

                },
                Err(_) => {
                    // synchronize to next event handler, state, remaining blocks or the end token
                    let sync_tokens = &vec![PipeTok, StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                    self.synchronize(sync_tokens);
                }
            }
        }

        // add to hierarchy

        match &dispatch_opt {
            Some(dispatch_node) => {
                match &mut self.system_hierarchy_opt {
                    Some(system_hierarchy) => {
                        let target_state_name = dispatch_node.target_state_ref.name.clone();
                        system_hierarchy.add_node(state_name.clone(),target_state_name.clone());
                    },
                    None => {
                        return Err(ParseError::new("System Hierarchy should always be here."));
                    }
                }
            },
            None => {
                match &mut self.system_hierarchy_opt {
                    Some(system_hierarchy) => {
                        system_hierarchy.add_node(state_name.clone(),String::new());
                    },
                    None => {
                        return Err(ParseError::new("System Hierarchy should always be here."));
                    }
                }
            }

        }

        // state local variables
        let mut vars_opt = None;
        let mut vars = Vec::new();

        if self.is_building_symbol_table {
            let state_local_scope_struct = StateLocalScopeSymbol::new();
            let state_local_scope_symbol_rcref = Rc::new(RefCell::new(state_local_scope_struct));
            let state_local_scope = ParseScopeType::StateLocalScope { state_local_scope_symbol_rcref };
            self.arcanum.enter_scope(state_local_scope);
        } else {
            self.arcanum.set_parse_scope(StateLocalScopeSymbol::scope_name());
        }

        // variable decl
        // let v     (mutable)
        // const c   (immutable)
        while self.match_token(&vec![VarTok, ConstTok]) {
            self.generate_state_context = true;
            match self.variable_decl(IdentifierDeclScope::StateVar) {
                Ok(variable_node) =>  {
                    vars.push(variable_node);
                },
                Err(err) => {
                    return Err(err);
                },
            }
        }

        if vars.len() > 0 {
            vars_opt = Some(vars);
        }

        // State Calls
        let mut calls_opt = None;
        let mut calls = Vec::new();

        // @TODO - add reference syntax
        while self.match_token(&vec![IdentifierTok]) {
            match self.variable_or_call_expr(IdentifierDeclScope::None) {
                // Ok(Some(VariableExprT { var_node: id_node }))
                //     => {
                //     // TODO: better error handling
                //     return Err(ParseError::new("TODO"));
                // },
                // Ok(Some(CallExprT { call_expr_node }))
                //     => calls.push(CallExprT{call_expr_node}),
                // Ok(Some(ActionCallExprT { action_call_expr_node}))
                //     => calls.push(action_call_expr_node),
                Ok(Some(CallChainLiteralExprT { call_chain_expr_node }))
                    => calls.push(call_chain_expr_node),
                Ok(Some(_)) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {}, // continue
            }
        }

        if calls.len() > 0 {
            calls_opt = Some(calls);
        }

        // Parse any event handlers.

        // TODO: make this Option?
        let mut evt_handlers:Vec<Rc<RefCell<EventHandlerNode>>> = Vec::new();
        let mut enter_event_handler = Option::None;
        let mut exit_event_handler = Option::None;

        loop {
            while self.match_token(&vec![SingleLineCommentTok]) {
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


            if self.peek().token_type == AtTok ||
                self.peek().token_type == PipeTok ||
                self.peek().token_type == AnyMessageTok {

                while self.peek().token_type == AtTok ||
                    self.peek().token_type == PipeTok ||
                    self.peek().token_type == AnyMessageTok {

                    match self.event_handler() {
                        Ok(eh_opt) => {
                            match eh_opt {
                                Some(eh) => {

                                    let eh_rcref = Rc::new(RefCell::new(eh));
                                    // find enter/exit event handlers
                                    {
                                        // new scope to make BC happy
                                        let eh_ref = eh_rcref.as_ref().borrow();
                                        let evt = eh_ref.event_symbol_rcref.as_ref().borrow();

                                        if evt.is_enter_msg {
                                            if enter_event_handler.is_some() {
                                                self.error_at_current(&format!("State ${} has more than one enter event handler.", &state_name));
                                            } else {
                                                enter_event_handler = Some(eh_rcref.clone());
                                            }
                                        } else if evt.is_exit_msg {
                                            if exit_event_handler.is_some() {
                                                self.error_at_current(&format!("State ${} has more than one exit event handler.", &state_name));
                                            } else {
                                                exit_event_handler = Some(eh_rcref.clone());
                                            }
                                        }
                                    }

                                    evt_handlers.push(eh_rcref);
                                },
                                None => {},
                            }
                        },
                        Err(_) => {
                            let sync_tokens = &vec![PipeTok, StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                            self.synchronize(sync_tokens);
                        }
                    }
                }

            } else {
                let follows_vec = &vec![StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                if self.follows(self.peek(),follows_vec) {
                    // next token is expected
                    break;
                } else {
                    self.error_at_current("Unexpected token in event handler message");
                    let sync_tokens = &vec![PipeTok,AnyMessageTok,StateTok,ActionsBlockTok,DomainBlockTok];
                    if !self.synchronize(sync_tokens) {
                        return Err(ParseError::new("TODO"));
                    }
                }
            }
        }

        // TODO: Moved this down here as I think is a bug to hve it above but not sure.
        self.arcanum.exit_parse_scope(); // state block scope (StateBlockScopeSymbol)

        let state_node = StateNode::new(state_name.clone(),
                                        params_opt,
                                        vars_opt,
                                        calls_opt,
                                        evt_handlers,
                                        enter_event_handler,
                                        exit_event_handler,
                                        dispatch_opt,
                                        line);
        let state_node_rcref = Rc::new(RefCell::new(state_node));

        // If this is the 2nd pass, set the reference to the AST state node.
        if !self.is_building_symbol_table {
            // let state_validator = StateSemanticValidator::new();
            // if !state_validator.has_valid_exit_semantics(&state_node_rcref.borrow()) {
            //     return Err(ParseError::new("TODO"));
            // }
            state_symbol_rcref.borrow_mut().set_state_node(Rc::clone(&state_node_rcref));
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

    fn event_handler(&mut self) -> Result<Option<EventHandlerNode>,ParseError> {

        let message_type:MessageType;
        // Hack - there is a weird bug w/ Clion that doesn't let msg be uninitialized.
        // It just hangs upon exiting the method.
        let mut msg: String = "".to_string();
        let line_number:usize ;

        self.event_handler_has_transition = false;
        let a = self.message();

        match a {
            Ok(MessageType::AnyMessage{line}) => {
                line_number = line;
                message_type = AnyMessage{line}
            },
            Ok(MessageType::CustomMessage {message_node}) => {
                line_number = message_node.line;
                msg = message_node.name.clone();

                message_type = CustomMessage {message_node};
            },
            Err(parse_error) => {
                self.error_at_current("Error parsing event handler message.");
                return Err(parse_error)
            },
        }

        let mut is_declaring_event = false;
        if self.is_building_symbol_table {
            let event_symbol_rcref;

            // get or create the event symbol for the message we found
            match self.arcanum.get_event(&*msg,&self.state_name_opt) {
                Some(x) => {
                    event_symbol_rcref = Rc::clone(&x);
                },
                None => {
                    let event_symbol = EventSymbol::new(&self.arcanum.symbol_config
                                                        ,&msg
                                                        ,None
                                                        ,None
                                                        ,None
                                                        ,self.state_name_opt.clone());
                    event_symbol_rcref = Rc::new(RefCell::new(event_symbol));
                    self.arcanum.declare_event(Rc::clone(&event_symbol_rcref));

                    // This is the first time we are seeing this event.
                    // Set flag so parameters and return type are added to event symbol
                    // during this parse.
                    is_declaring_event = true;
                }
            }

            // create the event handler symbol and enter the event handler scope
            let event_handler_symbol
                = EventHandlerScopeSymbol::new(&msg, Rc::clone(&event_symbol_rcref));
            let event_handler_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_symbol));
            self.arcanum.enter_scope(ParseScopeType::EventHandlerScope { event_handler_scope_symbol_rcref });

        } else {
            self.arcanum.set_parse_scope(&msg);
        }

        // Remember to pop param scope at end if it is entered.
        let mut pop_params_scope = false;

        // Parse event handler parameters
        if self.match_token(&vec![TokenType::LBracketTok]) {
            if msg == self.arcanum.symbol_config.enter_msg_symbol {
                self.generate_state_context = true;
            }

            match self.parameters() {
                Ok(Some(parameters)) => {
                    // have parsed params - make sure they match w/ symbol
                    // pop scope at end.
                    pop_params_scope = true;
                    if self.is_building_symbol_table {
                        let event_symbol_rcref =  self.arcanum.get_event(&*msg,&self.state_name_opt).unwrap();

                        // if this is the first encounter w/ this event
                        // then add parameters to the event symbol.
                        // TODO: Not sure how this overlaps w/ the symbol table
                        // having an event parameter scope but maybe (probably is)
                        // duplicative.

                        if is_declaring_event {
                            // add the parameters to the symbol
                            let mut vec = Vec::new();
                            for param_node in &parameters {
                                let param_symbol = ParameterSymbol::new(param_node.param_name.clone(),param_node.param_type_opt.clone(),IdentifierDeclScope::None);
                                vec.push(param_symbol);
                            }
                            event_symbol_rcref.borrow_mut().params_opt = Some(vec);
                        }


                        let event_handler_params_scope_struct = EventHandlerParamsScopeSymbol::new(event_symbol_rcref);
                        let event_handler_params_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_params_scope_struct));
                        let event_handler_params_scope = ParseScopeType::EventHandlerParamsScope { event_handler_params_scope_symbol_rcref };
                        self.arcanum.enter_scope(event_handler_params_scope);
                        let mut event_symbol_params_opt:Option<Vec<ParameterSymbol>> = None;

                        let event_symbol_rcref
                            = match self.arcanum.get_event(&msg,&self.state_name_opt) {
                            Some(x) => x,
                            None => {
                                return Err(ParseError::new(&format!("Fatal error - could not find event {}.",msg)));
                            },
                        };

                        let mut event_handler_params_scope_symbol
                            = EventHandlerParamsScopeSymbol::new(Rc::clone(&event_symbol_rcref));
                        let event_symbol_rcref = self.arcanum.get_event(&msg,&self.state_name_opt).unwrap();
                        {
                            match &event_symbol_rcref.borrow().params_opt {
                                Some(symbol_params) => {
                                    // compare existing event symbol params w/ parsed ones
                                    for (i, x) in symbol_params.iter().enumerate() {
                                        match parameters.get(i) {
                                            Some(parameter_node) => {
                                                if x.is_eq(parameter_node) {
                                                    let scope = self.arcanum.get_current_identifier_scope();
                                                    let symbol_type= event_handler_params_scope_symbol.add_parameter(parameter_node.param_name.clone(), parameter_node.param_type_opt.clone(),scope);
                                                    self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                                                    self.arcanum.insert_symbol(symbol_type);
                                                    self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                                                } else {
                                                    // TODO
                                                    self.error_at_current("Bad parameter to event handler");

                                                    let sync_tokens = &vec![PipeTok, CaretTok, StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                                                    self.synchronize(sync_tokens);

                                                }
                                            },
                                            None => {
                                                return Err(ParseError::new("Fatal error - Bad parameter."));
                                            },
                                        }
                                    }
                                },
                                None => {
                                    // this is the first time we've seen parameters for this event.
                                    // Take them as the definitive list.
                                    let mut event_symbol_params = Vec::new();

                                    for param in &parameters {
                                        let param_name = &param.param_name.clone();
                                        let mut param_type_opt: Option<TypeNode> = None;
                                        if param.param_type_opt.is_some() {
                                            let pt = &param.param_type_opt.as_ref().unwrap().clone();
                                            param_type_opt = Some(pt.clone());
                                        }
                                        let scope = self.arcanum.get_current_identifier_scope();
                                        let b = ParameterSymbol::new(param_name.clone(), param_type_opt.clone(), scope);
                                        // add to Arcanum event symbol
                                        event_symbol_params.push(b);

                                        // add to event handler scope symbol (needed for lookups using the scope chain)
                                        let scope = self.arcanum.get_current_identifier_scope();
                                        let x = event_handler_params_scope_symbol.add_parameter(param_name.clone(), param_type_opt.clone(),scope);
                                        self.arcanum.insert_symbol(x);
                                    }
                                    event_symbol_params_opt = Some(event_symbol_params);
                                },
                            }
                        }
                        match event_symbol_params_opt {
                            Some(parameter_symbols)
                                => event_symbol_rcref.borrow_mut().params_opt = Some(parameter_symbols),
                            None => {}
                        }

                    } else {
                        // leave these comments to show how to debug scope errors.
 //                       self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                        self.arcanum.set_parse_scope(EventHandlerParamsScopeSymbol::scope_name());
 //                       self.arcanum.debug_print_current_symbols(self.arcanum.get_current_symtab());
                    }
                },
                Ok(None) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
            }
        }

        // Parse return type
        if self.match_token(&vec![TokenType::ColonTok]) {
            let return_type_opt;
            match self.type_decl() {
                Ok(type_node) => return_type_opt = Some(type_node),
                Err(parse_error) => return Err(parse_error),
            }
            if is_declaring_event {
                // declaring event so add return type to event symbol
                // let id = self.previous();
                // let return_type = id.lexeme.clone();

                let event_symbol_rcref = self.arcanum.get_event(&*msg,&self.state_name_opt).unwrap();
                event_symbol_rcref.borrow_mut().ret_type_opt = return_type_opt;
            }
        }

        if self.is_building_symbol_table {
            let event_handler_local_scope_struct = EventHandlerLocalScopeSymbol::new();
            let event_handler_local_scope_symbol_rcref = Rc::new(RefCell::new(event_handler_local_scope_struct));
            let event_handler_local_scope = ParseScopeType::EventHandlerLocalScope { event_handler_local_scope_symbol_rcref };
            self.arcanum.enter_scope(event_handler_local_scope);
        } else {
            self.arcanum.set_parse_scope(EventHandlerLocalScopeSymbol::scope_name());
        }

        let statements = self.statements();

        let event_symbol_rcref = self.arcanum.get_event(&msg,&self.state_name_opt).unwrap();
        let ret_event_symbol_rcref = Rc::clone(&event_symbol_rcref);
        let terminator_node = match self.event_handler_terminator(event_symbol_rcref) {
            Ok(terminator_node) => terminator_node,
            Err(_parse_error) => {
                // TODO: this vec keeps the parser from hanging. don't know why
                let sync_tokens = &vec![PipeTok, StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                self.synchronize(sync_tokens);
                // create "dummy" node to keep processing
                // TODO: 1) make line # an int so as to set it to -1 when it is a dummy node and 2) confirm this is the best way
                // to keep going
                TerminatorExpr::new(TerminatorType::Return, None, 0)
            },
        };

        // The state name must be set in an enclosing context. Otherwise fail
        // with extreme prejudice.

        let st_name = match &self.state_name_opt {
            Some(state_name) => {
                state_name.clone()
            }
            None => {
                return Err(ParseError::new(&format!("[line {}] Fatal error - event handler {} missing enclosing state context. Please notify bugs@frame-lang.org.",line_number,msg)));
            },
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

        Ok(Some(EventHandlerNode::new(st_name.clone(),
                                      message_type,
                                      statements,
                                      terminator_node,
                                      ret_event_symbol_rcref,
                                      self.event_handler_has_transition,line_number)))
    }

    /* --------------------------------------------------------------------- */

    // event_handler_terminator -> '^' | '>'

    // TODO: - explore just returning the TerminatorType
    fn event_handler_terminator(&mut self,_:Rc<RefCell<EventSymbol>>) ->  Result<TerminatorExpr,ParseError> {

        // let x = event_symbol_rcfef.borrow();
        // let ret_type = match &x.ret_type_opt {
        //     Some(ret_type) => ret_type.clone(),
        //     None => None,
        // };

        if self.match_token(&vec![TokenType::CaretTok]) {
            if self.match_token(&vec![TokenType::LParenTok]) {

                let expr_t = match self.unary_expression() {
                    Ok(Some(expr_t)) => expr_t,
                    _ => {
                        self.error_at_current("Expected expression as return value.");
                        return Err(ParseError::new("TODO"));
                    }
                };

                if let Err(parse_error) =  self.consume(RParenTok, "Expected ')'.") {
                    return Err(parse_error);
                }
                Ok(TerminatorExpr::new(Return, Some(expr_t), self.previous().line))
            } else {
                Ok(TerminatorExpr::new(Return, None, self.previous().line))
            }
        } else if self.match_token(&vec![TokenType::ElseContinueTok]) {
            Ok(TerminatorExpr::new(Continue, None,  self.previous().line))
        } else {
            self.error_at_current("Expected event handler terminator.");
            return Err(ParseError::new("TODO"));
        }
    }

    /* --------------------------------------------------------------------- */

    // statements ->

    // TODO: need result and optional
    fn statements(&mut self) -> Vec<DeclOrStmtType> {

        let mut statements = Vec::new();

        loop   {
            // let result = self.decl_or_stmt();
            match self.decl_or_stmt() {
                Ok(opt_smt) => {
                    match opt_smt {
                        Some(statement) => {
                            statements.push(statement);
                        },
                        None => {
                            return statements;
                        }
                    }
                },
                Err(_err) => {
                    let sync_tokens = &vec![IdentifierTok, LParenTok, CaretTok, GTTok, SystemTok, StateTok, PipePipeTok, DotTok, ColonTok, PipeTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                    self.synchronize(sync_tokens);
                },

            }
        }

//        statements
    }

    /* --------------------------------------------------------------------- */

    fn decl_or_stmt(&mut self) -> Result<Option<DeclOrStmtType>,ParseError> {
        if self.match_token(&vec![VarTok, ConstTok]) {
            match self.variable_decl(IdentifierDeclScope::EventHandlerVar) {
                Ok(var_decl_t_rc_ref) => {
                    return Ok(Some(DeclOrStmtType::VarDeclT { var_decl_t_rc_ref }));
                },
                Err(parse_error) => {
                    return Err(parse_error);
                },
            }
        }

        match self.statement() {
            Ok(opt_smt) => {
                match opt_smt {
                    Some(stmt_t)
                        => Ok(Some(DeclOrStmtType::StmtT { stmt_t })),
                    None => Ok(None),
                }
            },
            Err(err) => Err(err),
        }
    }

    /* --------------------------------------------------------------------- */

    // statement ->

    fn statement(&mut self) -> Result<Option<StatementType>,ParseError> {

        let mut expr_t_opt:Option<ExprType> = None;
        match self.expression() {
            Ok(et_opt) => expr_t_opt = et_opt,
            Err(_) => {
                let sync_tokens = &vec![IdentifierTok, PipeTok, StateTok, ActionsBlockTok, DomainBlockTok, SystemEndTok];
                self.synchronize(sync_tokens);
            },
        }

        match expr_t_opt {

            Some(expr_t) => {
                if self.is_bool_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current(&format!("Not a testable expression."));
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.bool_test(expr_t);
                    return match result {
                        Ok(bool_test_node) => {
                            let bool_test_t = TestType::BoolTest {
                                bool_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(bool_test_t);
                            let test_stmt_t = StatementType::TestStmt {
                                test_stmt_node,
                            };
                            Ok(Some(test_stmt_t))
                        },
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        },
                    }
                } else if self.is_string_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current(&format!("Not a testable expression."));
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.string_match_test(expr_t);
                    return match result {
                        Ok(string_match_test_node) => {
                            let match_test_t = TestType::StringMatchTest {
                                string_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt {
                                test_stmt_node,
                            };
                            Ok(Some(test_stmt_t))
                        },
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        },
                    }
                } else if self.is_number_match_test() {
                    if !self.is_testable_expression(&expr_t) {
                        self.error_at_current(&format!("Not a testable expression."));
                        return Err(ParseError::new("TODO"));
                    }
                    let result = self.number_match_test(expr_t);
                    return match result {
                        Ok(number_match_test_node) => {
                            let match_test_t = TestType::NumberMatchTest {
                                number_match_test_node,
                            };
                            let test_stmt_node = TestStatementNode::new(match_test_t);
                            let test_stmt_t = StatementType::TestStmt {
                                test_stmt_node,
                            };
                            Ok(Some(test_stmt_t))
                        },
                        Err(parse_error) => {
                            // TODO: ?
                            Err(parse_error)
                        },
                    }
                }

                match expr_t {
                    ExprListT { expr_list_node } => {
                        // path for transitions w/ an exit params group
                        if self.match_token(&vec![TokenType::TransitionTok]) {
                            match self.transition(Some(expr_list_node)) {
                                Ok(Some(stmt_t)) => return Ok(Some(stmt_t)),
                                Ok(None) => return Err(ParseError::new("TODO")),
                                Err(parse_err) => return Err(parse_err),
                            }
                        } else {
                            self.error_at_previous("Expected '->' token following expression list.");
                            return Err(ParseError::new("TODO"));
                        }
                    },
                    CallExprT { call_expr_node } => {
                        let call_stmt_node = CallStmtNode::new(call_expr_node);
                        let expr_stmt_t:ExprStmtType = CallStmtT { call_stmt_node };
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    },
                    CallExprListT { .. } => {
                        // this should never happen as it is the () in a call like foo()
                        return Err(ParseError::new("TODO"));
                    },
                    VariableExprT { var_node } => {
                        let variable_stmt_node = VariableStmtNode::new(var_node);
                        let expr_stmt_t:ExprStmtType = ExprStmtType::VariableStmtT {variable_stmt_node};
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    },
                    // TODO: remove this - doesn't make any sense
                    ActionCallExprT { action_call_expr_node } => {
                        let action_call_stmt_node = ActionCallStmtNode::new(action_call_expr_node);
                        let expr_stmt_t:ExprStmtType = ActionCallStmtT { action_call_stmt_node };
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    },
                    CallChainLiteralExprT { call_chain_expr_node } => {
                        let call_chain_literal_stmt_node = CallChainLiteralStmtNode::new(call_chain_expr_node);
                        let expr_stmt_t:ExprStmtType = ExprStmtType::CallChainLiteralStmtT {call_chain_literal_stmt_node};
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    },
                    // TODO: $$[+] isn't a true expression as there is no return value defined (yet)
                    // Could define it to return the pushed context.
                    StateStackOperationExprT {state_stack_op_node } => {
                        let state_stack_operation_statement_node = StateStackOperationStatementNode::new(state_stack_op_node);
                        return Ok(Some(StatementType::StateStackStmt {state_stack_operation_statement_node}));
                    },
                    AssignmentExprT { assignment_expr_node } => {
                        let assignment_stmt_node = AssignmentStmtNode::new(assignment_expr_node);
                        let expr_stmt_t:ExprStmtType = ExprStmtType::AssignmentStmtT {assignment_stmt_node};
                        return Ok(Some(StatementType::ExpressionStmt { expr_stmt_t }));
                    },
                    LiteralExprT { .. } => {
                        self.error_at_previous("Literal statements not allowed.");
                        return Err(ParseError::new("TODO"));
                    },
                    FrameEventExprT {..} => {
                        self.error_at_previous("Frame Event statements not allowed.");
                        return Err(ParseError::new("TODO"));
                    },
                    UnaryExprT {..} => {
                        self.error_at_previous("Unary expression statements not allowed.");
                        return Err(ParseError::new("TODO"));
                    },
                    BinaryExprT {..} => {
                        self.error_at_previous("Binary expression statements not allowed.");
                        return Err(ParseError::new("TODO"));
                    },
                }
            },
            None => {
                // This path is for transitions w/o an exit params group
                if self.match_token(&vec![TransitionTok]) {
                    match self.transition(None) {
                        Ok(Some(transition)) => return Ok(Some(transition)),
                        Ok(_) => return Err(ParseError::new("TODO")),
                        Err(parse_error) => return Err(parse_error),
                    }
                }
            }

        }

        if self.match_token(&vec!(ChangeStateTok)) {
            return match self.change_state() {
                Ok(Some(state_context_t)) => Ok(Some(state_context_t)),
                Ok(None) => Err(ParseError::new("TODO")),
                Err(parse_error) => Err(parse_error),
            }
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // This method detects if an expression can be tested:
    // (a = 1) ? --- not testable
    // (a + b) ? --- not testable (TODO: review but think not
    // not sure about frame event and parts. for now yes.

    fn is_testable_expression(&self, expr_t:&ExprType) -> bool {
        match expr_t {
            AssignmentExprT {..} => {
                false
            },
            UnaryExprT {..} => {
                true
            },
            BinaryExprT {..} => {
                true
            },
            ExprListT {expr_list_node} => {
                if expr_list_node.exprs_t.len() != 1 {
                    return false;
                }

                let first_expr_t = expr_list_node.exprs_t.first().unwrap();
                return self.is_testable_expression(first_expr_t);
            }
            _ => {
                true
            },
        }
    }

    /* --------------------------------------------------------------------- */

    fn is_bool_test(&self) -> bool {
        self.peek().token_type == TokenType::BoolTestTrueTok ||
            self.peek().token_type == TokenType::BoolTestFalseTok
    }

    /* --------------------------------------------------------------------- */

    fn is_string_match_test(&self) -> bool {
        self.peek().token_type == TokenType::StringTestTok
    }

    /* --------------------------------------------------------------------- */

    fn is_number_match_test(&self) -> bool {
        self.peek().token_type == TokenType::NumberTestTok
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

    fn bool_test(&mut self, expr_t: ExprType) -> Result<BoolTestNode,ParseError> {

        let is_negated:bool;

        // '?'
        if self.match_token(&vec![BoolTestTrueTok]) {
            is_negated = false;

        // ?!
        } else if self.match_token(&vec![BoolTestFalseTok]) {
            is_negated = true;
        } else {
            return Err(ParseError::new("TODO"));
        }

        let mut conditional_branches:Vec<BoolTestConditionalBranchNode> = Vec::new();

        let first_branch_node = match self.bool_test_conditional_branch_statements(is_negated, expr_t) {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        while self.match_token(&vec![ElseContinueTok]) {
            match self.bool_test_else_continue_branch() {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                },
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' bool_test_else_branch)?
        let mut bool_test_else_node_opt:Option<BoolTestElseBranchNode> = None;
        if self.match_token(&vec![ColonTok]) {
            bool_test_else_node_opt = Option::from(match self.bool_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) =  self.consume(TestTerminatorTok, "Expected TestTerminator.") {
            return Err(parse_error);
        }

        return Ok(BoolTestNode::new( conditional_branches, bool_test_else_node_opt));
    }

    /* --------------------------------------------------------------------- */

    // bool_test_body -> statements* branch_terminator?

    fn bool_test_else_continue_branch(&mut self) -> Result<BoolTestConditionalBranchNode,ParseError> {

        let expr_t: ExprType;
        let result = self.expression();
        match result {
            Ok(expression_opt) => {
                match expression_opt {
                    Some(et) => {
                        expr_t = et;
                    },
                    None => {
                        return Err(ParseError::new("TODO"));
                    },
                }
            },
            Err(parse_error) => return Err(parse_error),
        }

        let is_negated:bool;

        // '?'
        if self.match_token(&vec![BoolTestTrueTok]) {
            is_negated = false;

            // ?!
        } else if self.match_token(&vec![BoolTestFalseTok]) {
            is_negated = true;
        } else {
            return Err(ParseError::new("TODO"));
        }

       self.bool_test_conditional_branch_statements(is_negated,expr_t)

    }


    /* --------------------------------------------------------------------- */

    // bool_test_conditional_branch_statements -> statements* branch_terminator?

    fn bool_test_conditional_branch_statements(&mut self, is_negated:bool, expr_t: ExprType) -> Result<BoolTestConditionalBranchNode,ParseError> {

        let statements = self.statements();

        let result = self.branch_terminator();

        return match result {
            Ok(branch_terminator_t_opt) => {
                Ok(BoolTestConditionalBranchNode::new(is_negated, expr_t, statements, branch_terminator_t_opt))
            },
            Err(parse_error) => Err(parse_error),
        }
    }

    /* --------------------------------------------------------------------- */

    // bool_test_else_branch -> statements* branch_terminator?

    fn bool_test_else_branch(&mut self) -> Result<BoolTestElseBranchNode,ParseError> {

        let statements = self.statements();

        let result = self.branch_terminator();


        return match result {
            Ok(branch_terminator_opt) => {
                Ok(BoolTestElseBranchNode::new(statements, branch_terminator_opt))
            },
            Err(parse_error) => Err(parse_error),
        }

    }

    /* --------------------------------------------------------------------- */

    // branch_terminator -> ^ | '>'

    // TODO: explore returning a TerminatorType rather than node
    fn branch_terminator(&mut self) -> Result<Option<TerminatorExpr>,ParseError> {

        if self.match_token(&vec![TokenType::CaretTok]) {
            if self.match_token(&vec![TokenType::LParenTok]) {

                let expr_t = match self.unary_expression() {
                    Ok(Some(expr_t)) => expr_t,
                    _ => {
                        self.error_at_current("Expected expression as return value.");
                        return Err(ParseError::new("TODO"));
                    }
                };

                if let Err(parse_error) =  self.consume(RParenTok, "Expected ')'.") {
                    return Err(parse_error);
                }
                return Ok(Some(TerminatorExpr::new(Return, Some(expr_t),  self.previous().line)));
            } else {
                return Ok(Some(TerminatorExpr::new(Return, None, self.previous().line)));
            }
        } else if self.match_token(&vec![TokenType::GTTok]) {
            return Ok(Some(TerminatorExpr::new(Continue, None,  self.previous().line)));
        } else {
            return Ok(None);
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

    fn string_match_test(&mut self, expr_t: ExprType) -> Result<StringMatchTestNode,ParseError> {

        if let Err(parse_error) =  self.consume(StringTestTok, "Expected '?~'.") {
            return Err(parse_error);
        }

        let mut conditional_branches:Vec<StringMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.string_match_test_match_branch() {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        while self.match_token(&vec![ElseContinueTok]) {
            match self.string_match_test_match_branch() {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                },
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' match_test_else_branch)?
        let mut else_branch_opt:Option<StringMatchTestElseBranchNode> = None;
        if self.match_token(&vec![ColonTok]) {
            else_branch_opt = Option::from(match self.string_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) =  self.consume(TestTerminatorTok, "Expected TestTerminator.") {
            return Err(parse_error);
        }

        return Ok(StringMatchTestNode::new( expr_t,conditional_branches, else_branch_opt));

    }

    /* --------------------------------------------------------------------- */

    // string_match_test ->  ('/' match_string ('|' match_string)* '/' (statement* branch_terminator?) ':>')+  '::'

    fn string_match_test_match_branch(&mut self) -> Result<StringMatchTestMatchBranchNode,ParseError> {

        if let Err(parse_error) =  self.consume(ForwardSlashTok, "Expected '/'.") {
            return Err(parse_error);
        }

        let mut match_strings:Vec<String> = Vec::new();

        if !self.match_token(&vec![MatchStringTok]) {
            return Err(ParseError::new("TODO"));
        }

//        let token = self.previous();
        let match_string_tok = self.previous();
        let match_pattern_string = match_string_tok.lexeme.clone();
        match_strings.push(match_pattern_string);

        while self.match_token(&vec![PipeTok]) {
            if !self.match_token(&vec![MatchStringTok]) {
                return Err(ParseError::new("TODO"));
            }

 //           let token = self.previous();
            let match_string_tok = self.previous();
            let match_pattern_string = match_string_tok.lexeme.clone();
            match_strings.push(match_pattern_string);
        }

        let string_match_pattern_node = StringMatchTestPatternNode::new(match_strings);

        if let Err(parse_error) =  self.consume(ForwardSlashTok, "Expected '/'.") {
            return Err(parse_error);
        }

        let statements = self.statements();

        let result = self.branch_terminator();

        return match result {
            Ok(branch_terminator_t_opt) => {
                Ok(StringMatchTestMatchBranchNode::new(string_match_pattern_node, statements, branch_terminator_t_opt))
            },
            Err(parse_error) => Err(parse_error),
        }
    }


    /* --------------------------------------------------------------------- */

    // match_test_else_branch -> statements* branch_terminator?

    fn string_match_test_else_branch(&mut self) -> Result<StringMatchTestElseBranchNode,ParseError> {

        let statements = self.statements();

        let result = self.branch_terminator();

        return match result {
            Ok(branch_terminator_opt) => {
                Ok(StringMatchTestElseBranchNode::new(statements, branch_terminator_opt))
            },
            Err(parse_error) => Err(parse_error),
        }
    }


    /* --------------------------------------------------------------------- */

    // expression -> TODO

    fn expression(&mut self) -> Result<Option<ExprType>,ParseError> {

        self.assignment()
    }

    /* --------------------------------------------------------------------- */

    // expression -> TODO

    fn assignment (&mut self) -> Result<Option<ExprType>,ParseError> {

        let l_value = match self.equality() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        if self.match_token(&vec![TokenType::EqualsTok]) {
            // this changes the tokens generated for expression lists
            // like (a) and (a b c)
            self.is_parsing_rhs = true;

            let line = self.previous().line;
            let r_value = match self.equality() {
                Ok(Some(expr_type)) => {
                    self.is_parsing_rhs = false;
                    expr_type
                },
                Ok(None) => {
                    self.is_parsing_rhs = false;
                    return Ok(None)
                },
                Err(parse_error) => {
                    self.is_parsing_rhs = false;
                    return Err(parse_error)
                },
            };

            let assignment_expr_node = AssignmentExprNode::new(l_value, r_value,line);
            return Ok(Some(AssignmentExprT {assignment_expr_node}));
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */


    fn equality(&mut self) -> Result<Option<ExprType>,ParseError> {

        let mut l_value = match self.comparison() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&vec![TokenType::BangEqualTok,
                                                TokenType::EqualEqualTok]) {
 //           let line = self.previous().line;
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.comparison() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT {binary_expr_node};
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn comparison(&mut self) -> Result<Option<ExprType>,ParseError> {
        let mut l_value = match self.term() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&vec![TokenType::GTTok,
                                                TokenType::GreaterEqualTok,
                                                TokenType::LTTok,
                                                TokenType::LessEqualTok ]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.term() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT {binary_expr_node};
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn term(&mut self) -> Result<Option<ExprType>,ParseError> {
        let mut l_value = match self.factor() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&vec![TokenType::DashTok,
                                                TokenType::PlusTok ]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.factor() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT {binary_expr_node};
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn factor(&mut self) -> Result<Option<ExprType>,ParseError> {
        let mut l_value = match self.logical_xor() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&vec![TokenType::ForwardSlashTok,
                                                TokenType::StarTok ]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.logical_xor() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT {binary_expr_node};
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn logical_xor(&mut self) -> Result<Option<ExprType>,ParseError> {
        let mut l_value = match self.logical_or() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&vec![TokenType::LogicalXorTok]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.logical_or() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT {binary_expr_node};
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn logical_or(&mut self) -> Result<Option<ExprType>,ParseError> {
        let mut l_value = match self.logical_and() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&vec![TokenType::PipePipeTok]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.logical_and() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT {binary_expr_node};
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    fn logical_and(&mut self) -> Result<Option<ExprType>,ParseError> {
        let mut l_value = match self.unary_expression() {
            Ok(Some(expr_type)) => expr_type,
            Ok(None) => return Ok(None),
            Err(parse_error) => return Err(parse_error),
        };

        while self.match_token(&vec![TokenType::LogicalAndTok]) {
            let operator_token = self.previous();
            let op_type = OperatorType::get_operator_type(&operator_token.token_type);
            let r_value = match self.unary_expression() {
                Ok(Some(expr_type)) => expr_type,
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            };

            let binary_expr_node = BinaryExprNode::new(l_value, op_type, r_value);
            l_value = BinaryExprT {binary_expr_node};
        }

        Ok(Some(l_value))
    }

    /* --------------------------------------------------------------------- */

    // unary_expression -> TODO

    fn unary_expression(&mut self) -> Result<Option<ExprType>,ParseError> {

        if self.match_token(&vec![BangTok,DashTok]) {
            let token = self.previous();
            let mut operator_type = OperatorType::get_operator_type(&token.token_type);
            if operator_type == OperatorType::Minus {
                // change this so the code gen doesn't have a space between the - and ID
                // -x rather than - x
                operator_type = OperatorType::Negated;
            }
            let right_expr_t = self.unary_expression();
            match right_expr_t {
                Ok(Some(x)) => {
                    let unary_expr_node = UnaryExprNode::new(operator_type,x);
                    return Ok(Some(UnaryExprT {unary_expr_node}));
                },
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {}, // continue
            }
        }

        // '(' ')' | '(' expr+ ')'
        if self.match_token(&vec![LParenTok]) {
            match self.expr_list() {
                Ok(Some(ExprListT { expr_list_node: expr_node }))
                    => return Ok(Some(ExprListT { expr_list_node: expr_node })),
                Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {}, // continue
            }
        }

        let mut scope = IdentifierDeclScope::None;

//        let mut scope_override = false;
        if self.match_token(&vec![SystemTok]) {
            if let Err(parse_error) = self.consume(DotTok, "Expected '.'.") {
                return Err(parse_error); // TODO
            }
            scope = IdentifierDeclScope::DomainBlock;
 //           scope_override = true;
        } else if self.match_token(&vec![StateTok]) {
            if self.match_token(&vec![LBracketTok]) {
                return if self.match_token(&vec![IdentifierTok]) {
//                    let id = self.previous().lexeme.clone();
                    let id_node = IdentifierNode::new(self.previous().clone(), None, IdentifierDeclScope::StateParam, false,self.previous().line);
                    let var_scope = id_node.scope.clone();
                    let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme,&var_scope).clone();

                    let var_node = VariableNode::new(id_node,var_scope,symbol_type_rcref_opt);
                    if let Err(parse_error) = self.consume(RBracketTok, "Expected ']'.") {
                        return Err(parse_error); // TODO
                    }
                    Ok(Some(VariableExprT { var_node }))
                } else {
                    self.error_at_current("Expected identifier.");
                    Err(ParseError::new("TODO")) // TODO
                }
            } else if self.match_token(&vec![DotTok]) {
                return if self.match_token(&vec![IdentifierTok]) {
                    let id_node = IdentifierNode::new(self.previous().clone(), None, IdentifierDeclScope::StateVar, false, self.previous().line);
                    let var_scope = id_node.scope.clone();
                    let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme,&var_scope).clone();
                    let var_node = VariableNode::new(id_node,var_scope,symbol_type_rcref_opt);
                    Ok(Some(VariableExprT { var_node }))
                } else {
                    self.error_at_current("Expected identifier.");
                    Err(ParseError::new("TODO"))
                }
            } else {
                self.error_at_current("Unexpected token.");
                return Err(ParseError::new("TODO"));
            }
        } else if self.match_token(&vec![PipePipeLBracketTok]) {
 //           if self.match_token(&vec![LBracketTok]) {
            let id_node;
            let var_node;
            if self.match_token(&vec![IdentifierTok]) {
                id_node = IdentifierNode::new(self.previous().clone(), None, IdentifierDeclScope::EventHandlerParam, false,self.previous().line);
                let var_scope = id_node.scope.clone();
                let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme,&var_scope).clone();
                var_node = VariableNode::new(id_node, var_scope, symbol_type_rcref_opt);
            } else {
                self.error_at_current("Expected identifier.");
                return Err(ParseError::new("TODO"));
            }
            if let Err(parse_error) =  self.consume(RBracketTok, "Expected ']'.") {
                return Err(parse_error);
            }
            return Ok(Some(VariableExprT { var_node }));
        } else if self.match_token(&vec![PipePipeDotTok]) {
                let id_node;
                if self.match_token(&vec![IdentifierTok]) {
                    let id_tok = self.previous().clone();
                    id_node = IdentifierNode::new(id_tok, None, IdentifierDeclScope::EventHandlerVar, false, self.previous().line);
                } else {
                    self.error_at_current("Expected identifier.");
                    return Err(ParseError::new("TODO"));
                }

                let var_scope = id_node.scope.clone();
                let symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme,&var_scope).clone();
                let var_node = VariableNode::new(id_node,var_scope,symbol_type_rcref_opt);
                return Ok(Some(VariableExprT { var_node }));
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
        if self.match_token(&vec![AndTok]) {
            is_reference = true;
        }

        // TODO: I think only identifier is allowed?
        if self.match_token(&vec![IdentifierTok]) {
            match self.variable_or_call_expr(scope) {
                Ok(Some(VariableExprT { mut var_node })) => {
                    var_node.id_node.is_reference = is_reference;
                    return Ok(Some(VariableExprT { var_node }))
                },
                Ok(Some(CallExprT { call_expr_node: method_call_expr_node }))
                    => return Ok(Some(CallExprT { call_expr_node: method_call_expr_node })),
                Ok(Some(ActionCallExprT { action_call_expr_node}))
                    => return Ok(Some(ActionCallExprT { action_call_expr_node })),
                Ok(Some(CallChainLiteralExprT { mut call_chain_expr_node })) => {
                    // set the is_reference on first variable in the call chain
                    let call_chain_first_node_opt =  call_chain_expr_node.call_chain.get_mut(0);
                    if let Some(call_chain_first_node) = call_chain_first_node_opt {
                        call_chain_first_node.setIsReference(is_reference);
                    }

                    return Ok(Some(CallChainLiteralExprT { call_chain_expr_node }))
                },
                Ok(Some(_)) => return Err(ParseError::new("TODO")),
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {}, // continue
            }
        }

        // number | string | bool | null | nil
        match self.literal_expr() {
            Ok(Some(mut literal_expr_node)) => {
                literal_expr_node.is_reference = is_reference;
                return Ok(Some(LiteralExprT { literal_expr_node }));
            },
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {}, // continue
        }

        // $$[+] | $$[-]
        match self.stack_operation() {
            Ok(Some(state_stack_op_node)) => return Ok(Some( StateStackOperationExprT {state_stack_op_node} )),
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {}, // continue
        }

        // @ | @|| | @[x] | @^
        match self.frame_event_part(is_reference) {
            Ok(Some(frame_event_part)) => return Ok(Some(FrameEventExprT {frame_event_part})),
            Err(parse_error) => return Err(parse_error),
            Ok(None) => {}, // continue
        }

        Ok(None)
    }


    /* --------------------------------------------------------------------- */

    //

    fn stack_operation(&mut self) -> Result<Option<StateStackOperationNode>,ParseError> {

        if self.match_token(&vec![StateStackOperationPushTok]) {
            self.generate_state_stack = true;
            let ssot = StateStackOperationNode::new(StateStackOperationType::Push);
            return Ok(Some(ssot));
        } else if self.match_token(&vec![StateStackOperationPopTok]) {
            self.generate_state_stack = true;
            let ssot = StateStackOperationNode::new(StateStackOperationType::Pop);
            return Ok(Some(ssot));
        }

        return Ok(None);
    }

    /* --------------------------------------------------------------------- */

    // Parse FrameEvent "part" identifier:
    // @||  - Event message
    // @[p] - Event parameter
    // @^   - Event return object/value

    fn frame_event_part(&mut self,is_reference:bool) -> Result<Option<FrameEventPart>,ParseError> {

        if !self.match_token(&vec![AtTok]) {
            return Ok(None);
        }

        // '@' '||'
        if self.match_token(&vec![PipePipeTok]) {
            return Ok(Some(FrameEventPart::Message {is_reference}));
        }

        // '@' '[' identifier ']'
        if self.match_token(&vec![LBracketTok]) {
            if self.match_token(&vec![IdentifierTok]) {
                let id_tok = self.previous().clone();

                if let Err(parse_error) =  self.consume(RBracketTok, "Expected ']'.") {
                    return Err(parse_error);
                }
                return Ok(Some(FrameEventPart::Param{param_tok:id_tok,is_reference}));
            } else {
                self.error_at_current("Expected identifier.");
                return Err(ParseError::new("TODO"));
            }
        }

        // '@' '^'
        if self.match_token(&vec![CaretTok]) {
            return Ok(Some(FrameEventPart::Return {is_reference}));
        }

        // @
        Ok(Some(FrameEventPart::Event {is_reference}))
    }

    /* --------------------------------------------------------------------- */

    // expr_list -> '(' expression* ')'

    fn expr_list(&mut self) -> Result<Option<ExprType>,ParseError> {

        let mut expressions:Vec<ExprType> = Vec::new();

        while !self.match_token(&vec![RParenTok]) {
            match self.expression() {
                Ok(Some(expression)) => {
                    expressions.push(expression);
                },
                // should see a list of valid expressions until ')'
                Ok(None) => return Ok(None),
                Err(parse_error) => return Err(parse_error),
            }
        }

        let expr_list = ExprListT { expr_list_node: ExprListNode::new(expressions)};
        Ok(Some(expr_list))
    }

    /* --------------------------------------------------------------------- */

    // TODO: create a new return type that is narrowed to just the types this method returns.
    // TODO: change the return type to be CallChainLiteralExprT as it doesn't return anything else.
    fn variable_or_call_expr(&mut self, explicit_scope: IdentifierDeclScope) -> Result<Option<ExprType>,ParseError> {

        let mut scope:IdentifierDeclScope;

        let mut id_node = IdentifierNode::new(self.previous().clone(), None, explicit_scope.clone(),false,self.previous().line);
        let mut call_chain:std::collections::VecDeque<CallChainLiteralNodeType> = std::collections::VecDeque::new();

        // Loop over the tokens looking for "callable" tokens (methods and identifiers)
        // separated by '.' and build the "call_chain".

        let mut is_first_node = true;
        loop {

            // test for method call
            if self.match_token(&vec![LParenTok]) {
                let r = self.method_call(id_node);
                match r {
                    Ok(method_call_expr_node) => {

                        if !self.is_building_symbol_table {

                            let s = method_call_expr_node.identifier.name.lexeme.clone();
                            let action_decl_symbol_opt = self.arcanum.lookup_action(&s);

                            // test if identifier is in the arcanum. If so, its an action. If not, its an
                            // external call.

                            match action_decl_symbol_opt {
                                Some(ads) => {
                                    // action
                                    let mut action_call_expr_node = ActionCallExprNode::new(method_call_expr_node);
                                    action_call_expr_node.set_action_symbol(&Rc::clone(&ads));
                                    call_chain.push_back(CallChainLiteralNodeType::ActionCallT { action_call_expr_node });
                                },
                                None => {
                                    let interface_method_symbol_opt = self.arcanum.lookup_interface_method(&s);

                                    match interface_method_symbol_opt {
                                        Some(interface_method_symbol) => {
                                            let mut interface_method_call_expr_node = InterfaceMethodCallExprNode::new(method_call_expr_node);
                                            interface_method_call_expr_node.set_interface_symbol(&Rc::clone(&interface_method_symbol));
                                            call_chain.push_back(CallChainLiteralNodeType::InterfaceMethodCallT { interface_method_call_expr_node });

                                        },
                                        None => {
                                            // external call
                                            // if method_call_expr_node.identifier.scope == IdentifierDeclScope::DomainBlock {
                                            //     // change to interface block as it is a method call #.iface()
                                            //     method_call_expr_node.identifier.scope = IdentifierDeclScope::InterfaceBlock;
                                            // }
                                            let call_t = CallChainLiteralNodeType::CallT {call:method_call_expr_node};
                                            call_chain.push_back(call_t);
                                        }
                                    }

                                },
                            }
                        }
                    },
                    _  => return Err(ParseError::new("TODO")),
                }
            } else  {
                match self.get_identifier_scope(&id_node,&explicit_scope) {
                    Ok(id_decl_scope) => scope = id_decl_scope,
                    Err(err) => return Err(err),
                }
                let node = if scope == IdentifierDeclScope::None || !is_first_node {
                    CallChainLiteralNodeType::IdentifierNodeT {id_node}
                } else {
                    // variables (or parameters) must be
                    // the first (or only) node in the call chain
                    let symbol_type_rcref_opt:Option<Rc<RefCell<SymbolType>>>;
                    symbol_type_rcref_opt = self.arcanum.lookup(&id_node.name.lexeme,&explicit_scope).clone();
                    let var_node = VariableNode::new(id_node, scope, (&symbol_type_rcref_opt).clone());
                    CallChainLiteralNodeType::VariableNodeT {var_node}
                };
                call_chain.push_back(node);
            }

            // end of chain if no  '.'
            if !self.match_token(&vec![DotTok]) {
                break;
            }

            if self.match_token(&vec![IdentifierTok]) {
                id_node = IdentifierNode::new(self.previous().clone(), None, IdentifierDeclScope::None, false,self.previous().line);
            } else {
                return Err(ParseError::new("TODO"));
            }
            is_first_node = false;
        }

        let call_chain_literal_expr_node = CallChainLiteralExprNode::new(call_chain);
        return Ok(Some(CallChainLiteralExprT {call_chain_expr_node:call_chain_literal_expr_node}));

    }


    /* --------------------------------------------------------------------- */

    fn get_identifier_scope(&mut self,identifier_node:&IdentifierNode,explicit_scope:&IdentifierDeclScope) -> Result<IdentifierDeclScope,ParseError> {
        let symbol_type_rcref_opt:Option<Rc<RefCell<SymbolType>>>;
        let mut scope:IdentifierDeclScope = IdentifierDeclScope::None;
        // find the variable in the arcanum
        symbol_type_rcref_opt = self.arcanum.lookup(&identifier_node.name.lexeme,&explicit_scope).clone();
        match &symbol_type_rcref_opt {
            Some(symbol_type_rcref) => {
                let symbol_type = symbol_type_rcref.borrow();
                match &*symbol_type {
                    DomainVariableSymbolT{domain_variable_symbol_rcref} => {
                        scope = domain_variable_symbol_rcref.borrow().scope.clone();
                    },
                    StateParamSymbolT{state_param_symbol_rcref} => {
                        scope = state_param_symbol_rcref.borrow().scope.clone();
                    },
                    StateVariableSymbolT{state_variable_symbol_rcref} => {
                        scope = state_variable_symbol_rcref.borrow().scope.clone();
                    },
                    EventHandlerVariableSymbolT {event_handler_variable_symbol_rcref} => {
                        scope = event_handler_variable_symbol_rcref.borrow().scope.clone();
                    },
                    EventHandlerParamSymbolT{event_handler_param_symbol_rcref} => {
                        scope = event_handler_param_symbol_rcref.borrow().scope.clone();
                    },
                    _ => {
                        return Err(ParseError::new("Error - unknown scope identifier."));
                    }
                }
            }
            None => {},
        };

        if !self.is_building_symbol_table {
            if *explicit_scope != IdentifierDeclScope::None && *explicit_scope != scope {
                let msg = &format!("Identifier {} - invalid scope identifier.",identifier_node.name.lexeme);
                self.error_at_current(msg);
                return Err(ParseError::new(msg));
            }
        }

        Ok(scope)
    }

    /* --------------------------------------------------------------------- */

    // method_call ->

    fn method_call(&mut self,identifer_node:IdentifierNode) -> Result<CallExprNode,ParseError> {

        let call_expr_list_node;
        match self.expr_list() {
            Ok(Some(ExprListT {expr_list_node})) => {
                // need to differentiate between regular expression lists and call expression lists
                // for formatting.
                call_expr_list_node = CallExprListNode::new(expr_list_node.exprs_t);
            //    call_expr_list_node = CallExprListT {call_expr_list_node};
            },
            Ok(Some(_)) |
            Ok(None) => return Err(ParseError::new("TODO")), // TODO: must return an ExprList
            Err(parse_error) => return Err(parse_error),
        }

        let method_call_expr_node = CallExprNode::new(identifer_node, call_expr_list_node, None);
//        let method_call_expression_type = ExpressionType::MethodCallExprType {method_call_expr_node};
        return Ok(method_call_expr_node);
    }

    /* --------------------------------------------------------------------- */

    // literal_expression -> '(' expression* ')'

    fn literal_expr(&mut self) -> Result<Option<LiteralExprNode>,ParseError> {

        // TODO: move this vec to the scanner
        let literal_tokens = vec![SuperStringTok,StringTok,NumberTok,TrueTok,FalseTok,NullTok,NilTok];

        for literal_tok in literal_tokens {
            if self.match_token(&vec![literal_tok])
            {
                return Ok(Some(LiteralExprNode::new(literal_tok, self.previous().lexeme.clone())))
            }
        }

        Ok(None)
    }

    /* --------------------------------------------------------------------- */

    // state_context ->

    fn state_context(&mut self, enter_args_opt:Option<ExprListNode>) -> Result<Option<StateContextType>,ParseError> {

        if self.match_token(&vec![TokenType::StateStackOperationPopTok]) {
            Ok(Some(StateContextType::StateStackPop {}))
        } else {
            // parse state ref e.g. '$S1'
            if !self.match_token(&vec![TokenType::StateTok]) {
                return Err(ParseError::new("Missing $"));
            }

            if !self.match_token(&vec![TokenType::IdentifierTok]) {
                return Err(ParseError::new("Missing state identifier"));
            }

            let state_id = self.previous();
            let name = state_id.lexeme.clone();

            // parse optional state ref expression list
            // '(' ')' | '(' expr ')'
            let mut state_ref_args_opt = None;
            if self.match_token(&vec![LParenTok]) {
                match self.expr_list() {
                    Ok(Some(ExprListT { expr_list_node }))
                    => state_ref_args_opt = Some(expr_list_node),
                    Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                    Err(parse_error) => return Err(parse_error),
                    Ok(None) => {}, // continue
                }
            }

            let state_context_node = StateContextNode::new(
                StateRefNode::new(name),
                state_ref_args_opt,
                enter_args_opt,
            );

            Ok(Some(StateContextType::StateRef { state_context_node }))
        }
    }


    /* --------------------------------------------------------------------- */

    // state_context ->

    fn change_state_context(&mut self, _:Option<ExprListNode>) -> Result<Option<StateContextType>,ParseError> {


        // parse state ref e.g. '$S1'
        if !self.match_token(&vec![TokenType::StateTok]) {
            return Err(ParseError::new("Missing $"));
        }

        if !self.match_token(&vec![TokenType::IdentifierTok]) {
            return Err(ParseError::new("Missing state identifier."));
        }

        let state_id = self.previous();
        let name = state_id.lexeme.clone();

        let state_context_node = StateContextNode::new(
            StateRefNode::new(name),
            None,
            None,
        );

        Ok(Some(StateContextType::StateRef { state_context_node }))

    }

    /* --------------------------------------------------------------------- */

    // transition : exitArgs '->' enterArgs transitionLabel stateRef stateArgs

    fn transition(&mut self, exit_args_opt:Option<ExprListNode>) -> Result<Option<StatementType>,ParseError> {

        self.generate_transition_state = true;

        if exit_args_opt.is_some() {
            // need exit args generated
            self.generate_exit_args = true;
        }
        let mut enter_args_opt:Option<ExprListNode> = None;
        let mut transition_label:Option<String> = None;

        // enterArgs: '(' ')' | '(' expr ')'
        if self.match_token(&vec![LParenTok]) {
            // need StateContext mechanism
            self.generate_state_context = true;
            match self.expr_list() {
                Ok(Some(ExprListT {expr_list_node}))
                    => enter_args_opt = Some(expr_list_node),
                Ok(Some(_)) => return Err(ParseError::new("TODO")), // TODO
                Err(parse_error) => return Err(parse_error),
                Ok(None) => {}, // continue
            }
        }

        // transition label string
        if self.match_token(&vec![StringTok]) {
            transition_label = Some(self.previous().lexeme.clone());
        }

        let state_context_t;
        match self.state_context(enter_args_opt) {
            Ok(Some(scn))
                => state_context_t = scn,
            Ok(None) => return Err(ParseError::new("TODO")),
            Err(parse_error) => return Err(parse_error),
        }

        // this is so we can know to declare a StateContext at the
        // top of the event handler.
        self.event_handler_has_transition = true;

        return Ok(Some(StatementType::TransitionStmt {
            transition_statement: TransitionStatementNode {
                target_state_context_t: state_context_t,
                exit_args_opt,
                label_opt:transition_label,
            }}
        ));
    }

    /* --------------------------------------------------------------------- */

    // change_state : '->>' change_state_label state_ref

    fn change_state(&mut self) -> Result<Option<StatementType>,ParseError> {

        self.generate_change_state = true;

        let mut label_opt:Option<String> = None;

        // change_state label string
        if self.match_token(&vec![StringTok]) {
            label_opt = Some(self.previous().lexeme.clone());
        }

        let state_context_t;
        match self.change_state_context(None) {
            Ok(Some(scn)) => state_context_t = scn,
            Ok(None) => return Err(ParseError::new("TODO")),
            Err(parse_error) => return Err(parse_error),
        }

        return Ok(Some(StatementType::ChangeStateStmt {
            change_state_stmt: ChangeStateStatementNode {
                state_context_t,
                label_opt,
            }}
        ));
    }


    /* --------------------------------------------------------------------- */

    // match_number_test -> '?#'  ('/' match_number_pattern  ('|' match_number_pattern)* '/' (statement* branch_terminator?) ':>')+ ':' (statement* branch_terminator?) '::'

    fn number_match_test(&mut self, expr_t: ExprType) -> Result<NumberMatchTestNode,ParseError> {

        if let Err(parse_error) =  self.consume(NumberTestTok
                                                , "Expected '?#'.") {
            return Err(parse_error);
        }

        let mut conditional_branches:Vec<NumberMatchTestMatchBranchNode> = Vec::new();

        let first_branch_node = match self.number_match_test_match_branch() {
            Ok(branch_node) => branch_node,
            Err(parse_error) => return Err(parse_error),
        };

        conditional_branches.push(first_branch_node);

        while self.match_token(&vec![ElseContinueTok]) {
            match self.number_match_test_match_branch() {
                Ok(branch_node) => {
                    conditional_branches.push(branch_node);
                },
                Err(parse_error) => return Err(parse_error),
            }
        }

        // (':' match_test_else_branch)?
        let mut else_branch_opt:Option<NumberMatchTestElseBranchNode> = None;
        if self.match_token(&vec![ColonTok]) {
            else_branch_opt = Option::from(match self.number_match_test_else_branch() {
                Ok(statements_t_opt) => statements_t_opt,
                Err(parse_error) => return Err(parse_error),
            });
        }

        // '::'
        if let Err(parse_error) =  self.consume(TestTerminatorTok, "Expected TestTerminator.") {
            return Err(parse_error);
        }

        return Ok(NumberMatchTestNode::new( expr_t,conditional_branches, else_branch_opt));

    }

    /* --------------------------------------------------------------------- */

    // number_match_test ->  ('/' match_number '/' (statement* branch_terminator?) ':>')+  '::'

    fn number_match_test_match_branch(&mut self) -> Result<NumberMatchTestMatchBranchNode,ParseError> {

        if let Err(parse_error) =  self.consume(ForwardSlashTok, "Expected '/'.") {
            return Err(parse_error);
        }

        let mut match_numbers = Vec::new();

        if !self.match_token(&vec![NumberTok]) {
            return Err(ParseError::new("TODO"));
        }

//        let token = self.previous();
        let match_number_tok = self.previous();
        let match_pattern_number = match_number_tok.lexeme.clone();
        let number_match_pattern_node = NumberMatchTestPatternNode::new(match_pattern_number);
        match_numbers.push(number_match_pattern_node);

        while self.match_token(&vec![PipeTok]) {
            if !self.match_token(&vec![NumberTok]) {
                return Err(ParseError::new("TODO"));
            }

//            let token = self.previous();
            let match_number_tok = self.previous();
            let match_pattern_number = match_number_tok.lexeme.clone();
            let number_match_pattern_node = NumberMatchTestPatternNode::new(match_pattern_number);
            match_numbers.push(number_match_pattern_node);
        }
        
        if let Err(parse_error) =  self.consume(ForwardSlashTok, "Expected '/'.") {
            return Err(parse_error);
        }

        let statements = self.statements();

        let result = self.branch_terminator();

        return match result {
            Ok(branch_terminator_t_opt) => {
                Ok(NumberMatchTestMatchBranchNode::new(match_numbers, statements, branch_terminator_t_opt))
            },
            Err(parse_error) => Err(parse_error),
        }
    }

   /* --------------------------------------------------------------------- */

    // number_match_test_else_branch -> statements* branch_terminator?

    fn number_match_test_else_branch(&mut self) -> Result<NumberMatchTestElseBranchNode,ParseError> {

        let statements = self.statements();

        let result = self.branch_terminator();

        return match result {
            Ok(branch_terminator_opt) => {
                Ok(NumberMatchTestElseBranchNode::new(statements, branch_terminator_opt))
            },
            Err(parse_error) => Err(parse_error),
        }

    }
}

// TODO fix these issues and disable warning suppression
#![allow(unknown_lints)]
#![allow(clippy::branches_sharing_code)]
#![allow(clippy::single_match)]
#![allow(clippy::ptr_arg)]
#![allow(non_snake_case)]

use crate::frame_c::ast::*;
use crate::frame_c::config::*;
use crate::frame_c::scanner::{Token, TokenType};
use crate::frame_c::symbol_table::*;
use crate::frame_c::visitors::golang_visitor::ExprContext::Rvalue;
use crate::frame_c::visitors::*;

#[derive(PartialEq)]
enum ExprContext {
    None,
    Lvalue,
    Rvalue,
}

pub struct GolangVisitor {
    compiler_version: String,
    config: GolangConfig,
    code: String,
    dent: usize,
    current_state_name_opt: Option<String>,
    current_event_msg: String,
    current_event_ret_type: String,
    arcanium: Arcanum,
    symbol_config: SymbolConfig,
    comments: Vec<Token>,
    current_comment_idx: usize,
    first_event_handler: bool,
    system_name: String,
    first_state_name: String,
    warnings: Vec<String>,
    has_states: bool,
    errors: Vec<String>,
    visiting_call_chain_literal_variable: bool,
    generate_state_stack: bool,
    generate_change_state: bool,
    current_var_type: String,
    expr_context: ExprContext,
}

impl GolangVisitor {
    //* --------------------------------------------------------------------- *//

    pub fn new(
        arcanium: Arcanum,
        config: FrameConfig,
        generate_state_stack: bool,
        generate_change_state: bool,
        compiler_version: &str,
        comments: Vec<Token>,
    ) -> GolangVisitor {
        let golang_config = config.codegen.golang;

        GolangVisitor {
            compiler_version: compiler_version.to_string(),
            code: String::from(""),
            dent: 0,
            current_state_name_opt: None,
            current_event_msg: String::new(),
            current_event_ret_type: String::new(),
            arcanium,
            symbol_config: SymbolConfig::new(),
            comments,
            current_comment_idx: 0,
            first_event_handler: true,
            system_name: String::new(),
            first_state_name: String::new(),
            has_states: false,
            errors: Vec::new(),
            warnings: Vec::new(),
            visiting_call_chain_literal_variable: false,
            //            generate_exit_args,
            //           generate_state_context,
            generate_state_stack,
            generate_change_state,
            //           generate_transition_state,
            current_var_type: String::new(),
            expr_context: ExprContext::None,
            config: golang_config,
        }
    }

    //* --------------------------------------------------------------------- *//

    pub fn format_type(&self, type_node: &TypeNode) -> String {
        let mut s = String::new();

        if let Some(frame_event_part) = &type_node.frame_event_part_opt {
            match frame_event_part {
                FrameEventPart::Event { is_reference } => {
                    if *is_reference {
                        s.push('&');
                    }
                    s.push_str(&*self.config.code.frame_event_type_name.clone());
                }
                _ => {}
            }
        } else {
            if type_node.is_reference {
                s.push('&');
            }

            s.push_str(&type_node.type_str.clone());
        }

        s
    }

    //* --------------------------------------------------------------------- *//

    pub fn get_code(&self) -> String {
        if !self.errors.is_empty() {
            let mut error_list = String::new();
            for error in &self.errors {
                error_list.push_str(&error.clone());
            }
            error_list
        } else {
            self.code.clone()
        }
    }

    //* --------------------------------------------------------------------- *//

    fn get_variable_type(&mut self, symbol_type: &SymbolType) -> String {
        let var_type = match &*symbol_type {
            SymbolType::DomainVariable {
                domain_variable_symbol_rcref,
            } => match &domain_variable_symbol_rcref.borrow().var_type {
                Some(type_node) => self.format_type(type_node),
                None => String::from("<?>"),
            },
            SymbolType::StateParam {
                state_param_symbol_rcref,
            } => match &state_param_symbol_rcref.borrow().param_type_opt {
                Some(type_node) => self.format_type(type_node),
                None => String::from("<?>"),
            },
            SymbolType::StateVariable {
                state_variable_symbol_rcref,
            } => match &state_variable_symbol_rcref.borrow().var_type {
                Some(type_node) => self.format_type(type_node),
                None => String::from("<?>"),
            },
            SymbolType::EventHandlerParam {
                event_handler_param_symbol_rcref,
            } => match &event_handler_param_symbol_rcref.borrow().param_type_opt {
                Some(type_node) => self.format_type(type_node),
                None => String::from("<?>"),
            },
            SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref,
            } => match &event_handler_variable_symbol_rcref.borrow().var_type {
                Some(type_node) => self.format_type(type_node),
                None => String::from("<?>"),
            },

            _ => {
                self.errors.push("Unknown scope.".to_string());
                return "error".to_string(); // won't get emitted
            }
        };

        var_type
    }

    //* --------------------------------------------------------------------- *//

    fn first_letter_to_lower_case(&self, s1: &String) -> String {
        let mut c = s1.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn first_letter_to_upper_case(&self, s1: &String) -> String {
        let mut c = s1.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_state_name(&self, state_name: &str) -> String {
        format!("{}_{}", self.config.code.state_type, state_name)
    }

    //* --------------------------------------------------------------------- *//

    fn format_variable_expr(&mut self, variable_node: &VariableNode) -> String {
        let mut code = String::new();

        match variable_node.scope {
            IdentifierDeclScope::System => {
                code.push('m');
            }
            IdentifierDeclScope::DomainBlock => {
                code.push_str(&format!("m.{}", variable_node.id_node.name.lexeme));
            }
            IdentifierDeclScope::StateParam => {
                let var_node = variable_node;
                let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                let var_symbol = var_symbol_rcref.borrow();
                let var_type = self.get_variable_type(&*var_symbol);

                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }
                code.push_str(&format!(
                    "m._compartment_.StateArgs[\"{}\"]",
                    variable_node.id_node.name.lexeme,
                ));
                // if is being used as an rval, cast it.
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!(".({})", var_type));
                }
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::StateVar => {
                let var_node = variable_node;
                let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                let var_symbol = var_symbol_rcref.borrow();
                let var_type = self.get_variable_type(&*var_symbol);

                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }
                code.push_str(&format!(
                    "m._compartment_.StateVars[\"{}\"]",
                    variable_node.id_node.name.lexeme,
                ));
                // if is being used as an rval, cast it.
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!(".({})", var_type));
                }
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::EventHandlerParam => {
                let var_node = variable_node;
                let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                let var_symbol = var_symbol_rcref.borrow();
                let var_type = self.get_variable_type(&*var_symbol);

                code.push_str(&format!(
                    "e.Params[\"{}\"]",
                    variable_node.id_node.name.lexeme
                ));
                // if is being used as an rval, cast it.
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!(".({})", var_type));
                }
            }
            IdentifierDeclScope::EventHandlerVar => {
                code.push_str(&variable_node.id_node.name.lexeme.to_string());
            }
            IdentifierDeclScope::None => {
                // TODO: Explore labeling Variables as "extern" scope
                code.push_str(&variable_node.id_node.name.lexeme.to_string());
            } // Actions?
            _ => self.errors.push("Illegal scope.".to_string()),
        }

        code
    }

    //* --------------------------------------------------------------------- *//

    fn format_parameter_list(&mut self, params_in: &Option<Vec<ParameterNode>>) {
        if params_in.is_none() {
            return;
        }

        let params = match params_in {
            Some(params) => params,
            None => {
                return;
            }
        };

        let mut separator = "";
        for param in params {
            self.add_code(separator);
            let param_type: String = match &param.param_type_opt {
                Some(ret_type) => self.format_type(ret_type),
                None => String::from("<?>"),
            };
            self.add_code(&format!("{} {}", param.param_name, param_type));
            separator = ",";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_actions_parameter_list(
        &mut self,
        params: &Vec<ParameterNode>,
        subclass_actions: &mut String,
    ) {
        let mut separator = "";
        for param in params {
            self.add_code(separator);
            subclass_actions.push_str(separator);
            let param_type: String = match &param.param_type_opt {
                Some(type_node) => self.format_type(type_node),
                None => String::from("<?>"),
            };
            self.add_code(&format!("{} {}", param.param_name, param_type));
            subclass_actions.push_str(&format!("{} {}", param_type, param.param_name));
            separator = ",";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_action_name(&mut self, action_name: &String) -> String {
        action_name.to_string()
    }

    //* --------------------------------------------------------------------- *//

    pub fn run(&mut self, system_node: &SystemNode) {
        // Initialize configuration values from spec attributes.

        match &system_node.attributes_opt {
            Some(attributes) => {
                for value in (*attributes).values() {
                    match value {
                        AttributeNode::MetaNameValueStr { attr } => {
                            match attr.name.as_str() {
                                // TODO: constants
                                "stateType" => self.config.code.state_type = attr.value.clone(),
                                "managed" => {
                                    self.config.code.manager = attr.value.clone();
                                    self.config.code.managed = true;
                                }
                                _ => {}
                            }
                        }
                        AttributeNode::MetaListIdents { attr } => {
                            match attr.name.as_str() {
                                "derive" => {
                                    for ident in &attr.idents {
                                        match ident.as_str() {
                                            // TODO: constants and figure out mom vs managed
                                            //  "Managed" => self.config.code.managed = true,
                                            "Marshal" => self.config.code.marshal = true,
                                            _ => {}
                                        }
                                    }
                                }
                                "managed" => {
                                    self.config.code.managed = true;
                                    if attr.idents.len() != 1 {
                                        self.errors.push(
                                            "Attribute 'managed' takes 1 parameter".to_string(),
                                        );
                                    }
                                    match attr.idents.get(0) {
                                        Some(manager_type) => {
                                            self.config.code.manager = manager_type.clone();
                                        }
                                        None => {
                                            self.errors.push(
                                                "Attribute 'managed' missing manager type."
                                                    .to_string(),
                                            );
                                        }
                                    }
                                }
                                _ => {
                                    self.errors.push("Unknown attribute".to_string());
                                }
                            }
                        }
                    }
                }

                self.config.code.marshal_system_state_var = format!("{}State", &system_node.name);
            }
            None => {}
        }

        if self.config.code.state_type.is_empty() {
            self.config.code.state_type = format!("{}State", system_node.name);
        }
        self.config.code.system_struct_type = format!(
            "{}Struct",
            self.first_letter_to_lower_case(&system_node.name)
        );
        if self.config.code.compartment_type.is_empty() {
            self.config.code.compartment_type = format!("{}Compartment", system_node.name);
        }

        system_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn add_code(&mut self, s: &str) {
        self.code.push_str(&*s.to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn newline(&mut self) {
        self.code.push_str(&*format!("\n{}", self.dent()));
    }

    //* --------------------------------------------------------------------- *//

    fn dent(&self) -> String {
        (0..self.dent).map(|_| "    ").collect::<String>()
    }

    //* --------------------------------------------------------------------- *//

    fn indent(&mut self) {
        self.dent += 1;
    }

    //* --------------------------------------------------------------------- *//

    fn outdent(&mut self) {
        self.dent -= 1;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_decl_stmts(&mut self, decl_stmt_types: &Vec<DeclOrStmtType>) {
        for decl_stmt_t in decl_stmt_types.iter() {
            match decl_stmt_t {
                DeclOrStmtType::VarDeclT { var_decl_t_rc_ref } => {
                    let variable_decl_node = var_decl_t_rc_ref.borrow();
                    variable_decl_node.accept(self);
                }
                DeclOrStmtType::StmtT { stmt_t } => {
                    match stmt_t {
                        StatementType::ExpressionStmt { expr_stmt_t } => {
                            match expr_stmt_t {
                                ExprStmtType::ActionCallStmtT {
                                    action_call_stmt_node,
                                } => action_call_stmt_node.accept(self), // // TODO
                                ExprStmtType::CallStmtT { call_stmt_node } => {
                                    call_stmt_node.accept(self)
                                }
                                ExprStmtType::CallChainLiteralStmtT {
                                    call_chain_literal_stmt_node,
                                } => call_chain_literal_stmt_node.accept(self),
                                ExprStmtType::AssignmentStmtT {
                                    assignment_stmt_node,
                                } => assignment_stmt_node.accept(self),
                                ExprStmtType::VariableStmtT { variable_stmt_node } => {
                                    variable_stmt_node.accept(self)
                                }
                                ExprStmtType::ExprListStmtT { expr_list_stmt_node } => {
                                    expr_list_stmt_node.accept(self)
                                }
                                // ExprStmtType::LoopStmtT { loop_stmt_node } => {
                                //     loop_stmt_node.accept(self)
                                // }
                            }
                        }
                        StatementType::TransitionStmt {
                            transition_statement,
                        } => {
                            transition_statement.accept(self);
                        }
                        StatementType::TestStmt { test_stmt_node } => {
                            test_stmt_node.accept(self);
                        }
                        StatementType::StateStackStmt {
                            state_stack_operation_statement_node,
                        } => {
                            state_stack_operation_statement_node.accept(self);
                        }
                        StatementType::ChangeStateStmt { change_state_stmt } => {
                            change_state_stmt.accept(self);
                        }
                        StatementType::LoopStmt {loop_stmt_node} => {
                            loop_stmt_node.accept(self);
                        }
                        StatementType::NoStmt => {
                            // TODO
                            self.errors.push("Unknown error.".to_string());
                        }
                    }
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_machinery(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        self.add_code("//=============== Machinery and Mechanisms ==============//");
        self.newline();
        if system_node.get_first_state().is_some() {
            self.newline();
            self.add_code(&format!(
                "func (m *{}Struct) _transition_(compartment *{}) {{",
                self.format_internal_system_name(&system_node.name),
                self.config.code.compartment_type,
            ));
            self.indent();
            self.newline();
            self.add_code("m._nextCompartment_ = compartment");
            self.outdent();
            self.newline();
            self.add_code("}");

            self.newline();
            self.newline();
            self.add_code(&format!(
                "func (m *{}Struct) _do_transition_(nextCompartment *{}) {{",
                self.format_internal_system_name(&system_node.name),
                self.config.code.compartment_type
            ));

            self.indent();
            self.newline();
            self.add_code(&format!(
                "m._mux_(&{}{{Msg: \"<\", Params: m._compartment_.ExitArgs, Ret: nil}})",
                self.config.code.frame_event_type_name
            ));
            self.newline();
            self.add_code("m._compartment_ = nextCompartment");
            self.newline();
            self.add_code(&format!(
                "m._mux_(&{}{{Msg: \">\", Params: m._compartment_.EnterArgs, Ret: nil}})",
                self.config.code.frame_event_type_name
            ));
            self.outdent();
            self.newline();
            self.add_code("}");

            if self.generate_state_stack {
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "func (m *{}Struct) _stateStack_push_(compartment *{}) {{",
                    self.first_letter_to_lower_case(&self.system_name),
                    self.config.code.compartment_type,
                ));
                self.indent();
                self.newline();
                self.add_code("m._stateStack_.Push(compartment)");
                self.outdent();
                self.newline();
                self.add_code("}");
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "func (m *{}Struct) _stateStack_pop_() *{} {{",
                    self.first_letter_to_lower_case(&self.system_name),
                    self.config.code.compartment_type
                ));
                self.indent();
                self.newline();
                self.add_code("compartment := m._stateStack_.Pop()");
                self.newline();

                self.add_code("return compartment");
                self.outdent();
                self.newline();
                self.add_code("}");
            }
            if self.generate_change_state {
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "func (m *{}Struct) _changeState_(compartment *{}) {{",
                    self.first_letter_to_lower_case(&system_node.name),
                    self.config.code.compartment_type,
                ));
                self.indent();
                self.newline();
                self.add_code("m._compartment_ = compartment");
                self.outdent();
                self.newline();
                self.add_code("}");
            }
            self.newline();
        }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_subclass(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        self.add_code("/********************************************************");
        self.newline();
        self.newline();
        self.add_code("// Unimplemented Actions");
        self.newline();
        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            for action_rcref in &actions_block_node.actions {
                let action_node = action_rcref.borrow();
                if action_node.code_opt.is_none() {
                    action_node.accept_action_decl(self);
                }
            }
        }
        self.newline();
        self.newline();
        self.add_code("********************************************************/");
    }

    //* --------------------------------------------------------------------- *//

    /// Generate a return statement within a handler. Call this rather than adding a return
    /// statement directly to ensure that the control-flow state is properly maintained.
    fn generate_return(&mut self) {
        self.newline();
        self.add_code("return");
        self.config.code.this_branch_transitioned = false;
    }

    /// Generate a return statement if the current branch contained a transition or change-state.
    fn generate_return_if_transitioned(&mut self) {
        if self.config.code.this_branch_transitioned {
            self.generate_return();
        }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_comment(&mut self, line: usize) {
        // can't use self.newline() or self.add_code() due to double borrow.
        let mut generated_comment = false;
        while self.current_comment_idx < self.comments.len()
            && line >= self.comments[self.current_comment_idx].line
        {
            let comment = &self.comments[self.current_comment_idx];
            if comment.token_type == TokenType::SingleLineComment {
                self.code
                    .push_str(&*format!("  // {}", &comment.lexeme[3..]));
                self.code.push_str(&*format!(
                    "\n{}",
                    (0..self.dent).map(|_| "    ").collect::<String>()
                ));
            } else {
                let len = &comment.lexeme.len() - 3;
                self.code
                    .push_str(&*format!("/* {}", &comment.lexeme[3..len]));
                self.code.push_str(&*"*/".to_string());
            }

            self.current_comment_idx += 1;
            generated_comment = true;
        }
        if generated_comment {
            //            self.code.push_str(&*format!("\n{}",(0..self.dent).map(|_| "\t").collect::<String>()));
        }
    }

    //* --------------------------------------------------------------------- *//

    // TODO
    fn generate_state_ref_change_state(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) {
        let target_state_name = match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            _ => {
                self.errors
                    .push("Change state target not found.".to_string());
                "error"
            }
        };

        self.newline();
        self.add_code(&format!(
            "compartment := New{}({})",
            self.config.code.compartment_type,
            self.generate_state_ref_code(target_state_name)
        ));

        self.newline();

        // -- Enter Arguments --

        let enter_args_opt = match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { state_context_node } => &state_context_node.enter_args_opt,
            StateContextType::StateStackPop {} => &None,
        };

        if let Some(enter_args) = enter_args_opt {
            // Note - searching for event keyed with "State:>"
            // e.g. "S1:>"

            let mut msg: String = String::from(target_state_name);
            msg.push(':');
            msg.push_str(&self.symbol_config.enter_msg_symbol);

            if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt) {
                match &event_sym.borrow().params_opt {
                    Some(event_params) => {
                        if enter_args.exprs_t.len() != event_params.len() {
                            panic!("Fatal error: misaligned parameters to arguments.")
                        }
                        let mut param_symbols_it = event_params.iter();
                        for expr_t in &enter_args.exprs_t {
                            match param_symbols_it.next() {
                                Some(p) => {
                                    let _param_type = match &p.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => String::from("<?>"),
                                    };
                                    let mut expr = String::new();
                                    expr_t.accept_to_string(self, &mut expr);
                                    self.add_code(&format!(
                                        "compartment.EnterArgs[\"{}\"] = {};",
                                        p.name, expr
                                    ));
                                    self.newline();
                                }
                                None => panic!(
                                    "Invalid number of arguments for \"{}\" event handler.",
                                    msg
                                ),
                            }
                        }
                    }
                    None => panic!("Invalid number of arguments for \"{}\" event handler.", msg),
                }
            } else {
                self.warnings.push(format!("State {} does not have an enter event handler but is being passed parameters in a change state", target_state_name));
            }
        }

        /*  -- State Arguments -- */
        let target_state_args_opt = match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_args_opt
            }
            StateContextType::StateStackPop {} => &Option::None,
        };

        if let Some(state_args) = target_state_args_opt {
            //            let mut params_copy = Vec::new();
            if let Some(state_sym) = self.arcanium.get_state(target_state_name) {
                match &state_sym.borrow().params_opt {
                    Some(event_params) => {
                        let mut param_symbols_it = event_params.iter();
                        // Loop through the ARGUMENTS...
                        for expr_t in &state_args.exprs_t {
                            // ...and validate w/ the PARAMETERS
                            match param_symbols_it.next() {
                                Some(param_symbol_rcref) => {
                                    let param_symbol = param_symbol_rcref.borrow();
                                    let _param_type = match &param_symbol.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => String::from("<?>"),
                                    };
                                    let mut expr = String::new();
                                    expr_t.accept_to_string(self, &mut expr);
                                    self.add_code(&format!(
                                        "compartment.StateArgs[\"{}\"] = {};",
                                        param_symbol.name, expr
                                    ));
                                    self.newline();
                                }
                                None => panic!(
                                    "Invalid number of arguments for \"{}\" state parameters.",
                                    target_state_name
                                ),
                            }
                            //
                        }
                    }
                    None => {}
                }
            } else {
                panic!("TODO");
            }
        } // -- State Arguments --

        // -- State Variables --

        let target_state_rcref_opt = self.arcanium.get_state(target_state_name);

        match target_state_rcref_opt {
            Some(q) => {
                //                target_state_vars = "stateVars".to_string();
                if let Some(state_symbol_rcref) = self.arcanium.get_state(&q.borrow().name) {
                    let state_symbol = state_symbol_rcref.borrow();
                    let state_node = &state_symbol.state_node.as_ref().unwrap().borrow();
                    // generate local state variables
                    if state_node.vars_opt.is_some() {
                        //                        let mut separator = "";
                        for var_rcref in state_node.vars_opt.as_ref().unwrap() {
                            let var = var_rcref.borrow();
                            let _var_type = match &var.type_opt {
                                Some(var_type) => var_type.get_type_str(),
                                None => String::from("<?>"),
                            };
                            let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
                            let mut expr_code = String::new();
                            expr_t.accept_to_string(self, &mut expr_code);
                            self.add_code(&format!(
                                "compartment.StateVars[\"{}\"] = {};",
                                var.name, expr_code
                            ));
                            self.newline();
                        }
                    }
                }
            }
            None => {
                //                code = target_state_vars.clone();
            }
        }
        self.newline();
        self.add_code("m._changeState_(compartment)");
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_code(&self, target_state_name: &str) -> String {
        self.format_state_name(target_state_name)
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_transition(&mut self, transition_statement: &TransitionStatementNode) {
        let target_state_name = match &transition_statement.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            _ => {
                self.errors.push("Unknown error.".to_string());
                ""
            }
        };

        match &transition_statement.label_opt {
            Some(label) => {
                self.newline();
                self.add_code(&format!("// {}", label));
            }
            None => {}
        }

        // -- Exit Arguments --

        if let Some(exit_args) = &transition_statement.exit_args_opt {
            if !exit_args.exprs_t.is_empty() {
                let mut msg: String = String::new();
                if let Some(state_name) = &self.current_state_name_opt {
                    msg = state_name.clone();
                }
                msg.push(':');
                msg.push_str(&self.symbol_config.exit_msg_symbol);

                if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt)
                {
                    match &event_sym.borrow().params_opt {
                        Some(event_params) => {
                            if exit_args.exprs_t.len() != event_params.len() {
                                self.errors.push(
                                    "Fatal error: misaligned parameters to arguments.".to_string(),
                                );
                            }
                            let mut param_symbols_it = event_params.iter();

                            // Loop through the ARGUMENTS...
                            for expr_t in &exit_args.exprs_t {
                                // ...and validate w/ the PARAMETERS
                                match param_symbols_it.next() {
                                    Some(p) => {
                                        let mut expr = String::new();
                                        self.newline();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.add_code(&format!(
                                            "m._compartment_.ExitArgs[\"{}\"] = {}",
                                            p.name, expr
                                        ));
                                    }
                                    None => self.errors.push(format!(
                                        "Invalid number of arguments for \"{}\" event handler.",
                                        msg
                                    )),
                                }
                            }
                        }
                        None => self
                            .errors
                            .push("Fatal error: misaligned parameters to arguments.".to_string()),
                    }
                } else {
                    let current_state_name = &self.current_state_name_opt.as_ref().unwrap();
                    self.errors.push(format!(
                        "Missing exit event handler for transition from ${} to ${}.",
                        current_state_name, &target_state_name
                    ));
                }
            }
        }

        // -- Enter Arguments --

        //      if self.generate_state_context {
        self.newline();
        self.add_code(&format!(
            "compartment := New{}({})",
            self.config.code.compartment_type,
            self.generate_state_ref_code(target_state_name)
        ));
        //     }

        if transition_statement.forward_event {
            self.newline();
            self.add_code("compartment._forwardEvent_ = e");
        }

        let enter_args_opt = match &transition_statement.target_state_context_t {
            StateContextType::StateRef { state_context_node } => &state_context_node.enter_args_opt,
            StateContextType::StateStackPop {} => &None,
        };

        if let Some(enter_args) = enter_args_opt {
            // Note - searching for event keyed with "State:>"
            // e.g. "S1:>"

            let mut msg: String = String::from(target_state_name);
            msg.push(':');
            msg.push_str(&self.symbol_config.enter_msg_symbol);

            if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt) {
                match &event_sym.borrow().params_opt {
                    Some(event_params) => {
                        if enter_args.exprs_t.len() != event_params.len() {
                            self.errors.push(
                                "Fatal error: misaligned parameters to arguments.".to_string(),
                            );
                        }
                        let mut param_symbols_it = event_params.iter();
                        for expr_t in &enter_args.exprs_t {
                            match param_symbols_it.next() {
                                Some(p) => {
                                    let mut expr = String::new();
                                    expr_t.accept_to_string(self, &mut expr);
                                    self.newline();
                                    self.add_code(&format!(
                                        "compartment.EnterArgs[\"{}\"] = {}",
                                        p.name, expr
                                    ));
                                }
                                None => self.errors.push(format!(
                                    "Invalid number of arguments for \"{}\" event handler.",
                                    msg
                                )),
                            }
                        }
                    }
                    None => self.errors.push(format!(
                        "Invalid number of arguments for \"{}\" event handler.",
                        msg
                    )),
                }
            } else {
                self.warnings.push(format!("State {} does not have an enter event handler but is being passed parameters in a transition", target_state_name));
            }
        }

        // -- State Arguments --

        let target_state_args_opt = match &transition_statement.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_args_opt
            }
            StateContextType::StateStackPop {} => &Option::None,
        };
        //
        if let Some(state_args) = target_state_args_opt {
            //            let mut params_copy = Vec::new();
            if let Some(state_sym) = self.arcanium.get_state(target_state_name) {
                match &state_sym.borrow().params_opt {
                    Some(event_params) => {
                        let mut param_symbols_it = event_params.iter();
                        // Loop through the ARGUMENTS...
                        for expr_t in &state_args.exprs_t {
                            // ...and validate w/ the PARAMETERS
                            match param_symbols_it.next() {
                                Some(param_symbol_rcref) => {
                                    let param_symbol = param_symbol_rcref.borrow();
                                    let mut expr = String::new();
                                    expr_t.accept_to_string(self, &mut expr);
                                    self.newline();
                                    self.add_code(&format!(
                                        "compartment.StateArgs[\"{}\"] = {}",
                                        param_symbol.name, expr
                                    ));
                                    self.newline();
                                }
                                None => self.errors.push(format!(
                                    "Invalid number of arguments for \"{}\" state parameters.",
                                    target_state_name
                                )),
                            }
                            //
                        }
                    }
                    None => {}
                }
            } else {
                self.errors.push("TODO".to_string());
            }
        } // -- State Arguments --

        // -- State Variables --

        let target_state_rcref_opt = self.arcanium.get_state(target_state_name);

        match target_state_rcref_opt {
            Some(q) => {
                //                target_state_vars = "stateVars".to_string();
                if let Some(state_symbol_rcref) = self.arcanium.get_state(&q.borrow().name) {
                    let state_symbol = state_symbol_rcref.borrow();
                    let state_node = &state_symbol.state_node.as_ref().unwrap().borrow();
                    // generate local state variables
                    if state_node.vars_opt.is_some() {
                        //                        let mut separator = "";
                        for var_rcref in state_node.vars_opt.as_ref().unwrap() {
                            let var = var_rcref.borrow();
                            let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
                            let mut expr_code = String::new();
                            expr_t.accept_to_string(self, &mut expr_code);
                            self.newline();
                            self.add_code(&format!(
                                "compartment.StateVars[\"{}\"] = {}",
                                var.name, expr_code
                            ));
                            self.newline();
                        }
                    }
                }
            }
            None => {
                //                code = target_state_vars.clone();
            }
        }

        self.newline();
        self.add_code("m._transition_(compartment)");
    }

    //* --------------------------------------------------------------------- *//

    // NOTE!!: it is *currently* disallowed to send state or event arguments to a state stack pop target
    // So currently this method just sets any exitArgs and pops the context from the state stack.

    fn generate_state_stack_pop_transition(
        &mut self,
        transition_statement: &TransitionStatementNode,
    ) {
        self.newline();
        match &transition_statement.label_opt {
            Some(label) => {
                self.add_code(&format!("// {}", label));
                self.newline();
            }
            None => {}
        }

        // -- Exit Arguments --

        if let Some(exit_args) = &transition_statement.exit_args_opt {
            if !exit_args.exprs_t.is_empty() {
                // Note - searching for event keyed with "State:<"
                // e.g. "S1:<"

                let mut msg: String = String::new();
                if let Some(state_name) = &self.current_state_name_opt {
                    msg = state_name.clone();
                }
                msg.push(':');
                msg.push_str(&self.symbol_config.exit_msg_symbol);

                if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt)
                {
                    match &event_sym.borrow().params_opt {
                        Some(event_params) => {
                            if exit_args.exprs_t.len() != event_params.len() {
                                self.errors.push(
                                    "Fatal error: misaligned parameters to arguments.".to_string(),
                                );
                            }
                            let mut param_symbols_it = event_params.iter();

                            // Loop through the ARGUMENTS...
                            for expr_t in &exit_args.exprs_t {
                                // ...and validate w/ the PARAMETERS
                                match param_symbols_it.next() {
                                    Some(p) => {
                                        let mut expr = String::new();
                                        self.newline();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.add_code(&format!(
                                            "m._compartment_.ExitArgs(\"{}\", {})",
                                            p.name, expr
                                        ));
                                    }
                                    None => self.errors.push(format!(
                                        "Invalid number of arguments for \"{}\" event handler.",
                                        msg
                                    )),
                                }
                            }
                        }
                        None => self
                            .errors
                            .push("Fatal error: misaligned parameters to arguments.".to_string()),
                    }
                } else {
                    self.errors.push("TODO".to_string());
                }
            }
        }

        self.add_code("compartment := m._stateStack_pop_()");
        self.newline();
        self.add_code("m._transition_(compartment)");
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_stack_pop_change_state(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) {
        self.newline();
        match &change_state_stmt_node.label_opt {
            Some(label) => {
                self.add_code(&format!("// {}", label));
                self.newline();
            }
            None => {}
        }

        self.add_code("compartment := m._stateStack_pop_()");
        self.newline();
        self.add_code("m._changeState_(compartment)");
    }

    //* --------------------------------------------------------------------- *//

    //* --------------------------------------------------------------------- *//

    fn generate_new_fn(&mut self, domain_vec: &Vec<(String, String)>, system_node: &SystemNode) {
        self.newline();
        self.newline();
        // format system params,if any.
        let mut separator = String::new();
        let mut new_params: String = match &system_node.start_state_state_params_opt {
            Some(param_list) => {
                let mut params = String::new();
                for param_node in param_list {
                    let param_type = match &param_node.param_type_opt {
                        Some(type_node) => self.format_type(type_node),
                        None => String::new(),
                    };
                    params.push_str(&format!(
                        "{}{} {}",
                        separator, param_node.param_name, &*param_type
                    ));
                    separator = String::from(",");
                }
                params
            }
            None => String::new(),
        };
        match &system_node.start_state_enter_params_opt {
            Some(param_list) => {
                for param_node in param_list {
                    let param_type = match &param_node.param_type_opt {
                        Some(type_node) => self.format_type(type_node),
                        None => String::new(),
                    };
                    new_params.push_str(&format!(
                        "{}{} {}",
                        separator, param_node.param_name, &*param_type
                    ));
                    separator = String::from(",");
                }
            }
            None => {}
        };
        match &system_node.domain_params_opt {
            Some(param_list) => {
                for param_node in param_list {
                    let param_type = match &param_node.param_type_opt {
                        Some(type_node) => self.format_type(type_node),
                        None => String::new(),
                    };
                    new_params.push_str(&format!(
                        "{}{} {}",
                        separator, param_node.param_name, &*param_type
                    ));
                    separator = String::from(",");
                }
            }
            None => {}
        };
        if self.config.code.managed {
            if new_params.is_empty() {
                self.add_code(&format!(
                    "func New{}(manager {}) {} {{",
                    system_node.name,
                    self.config.code.manager,
                    self.first_letter_to_upper_case(&system_node.name),
                ));
            } else {
                self.add_code(&format!(
                    "func New{}(manager {} {}) {} {{",
                    system_node.name,
                    self.config.code.manager,
                    new_params,
                    self.first_letter_to_upper_case(&system_node.name),
                ));
            }
        } else {
            self.add_code(&format!(
                "func New{}({}) {} {{",
                system_node.name,
                new_params,
                self.first_letter_to_upper_case(&system_node.name)
            ));
        }

        self.indent();
        self.newline();
        self.add_code(&format!(
            "m := &{}Struct{{}}",
            self.first_letter_to_lower_case(&system_node.name)
        ));

        if self.config.code.managed {
            self.newline();
            self.add_code("m._manager_ = manager");
        }
        self.newline();
        self.newline();
        self.add_code("// Validate interfaces");
        self.newline();
        self.add_code(&format!(
            "var _ {} = m",
            self.first_letter_to_upper_case(&system_node.name),
        ));
        self.newline();
        if system_node.actions_block_node_opt.is_some() {
            self.add_code(&format!("var _ {}_actions = m", &system_node.name,));
        }
        if self.generate_state_stack {
            self.newline();
            self.add_code("// History mechanism used in spec. Create state stack.");
            self.newline();
            self.add_code(&format!(
                "m._stateStack_ = &Stack{{stack: make([]{}, 0)}}",
                self.config.code.compartment_type
            ));
            self.newline();
        }
        self.newline();
        self.newline();
        self.add_code("// Create and intialize start state compartment.");
        self.newline();
        self.add_code(&format!(
            "m._compartment_ = New{}({})",
            self.config.code.compartment_type,
            self.format_state_name(self.first_state_name.as_str())
        ));

        // Initialize state arguments.
        match &system_node.start_state_state_params_opt {
            Some(params) => {
                for param in params {
                    self.newline();
                    self.add_code(&format!(
                        "m._compartment_.StateArgs[\"{}\"] = {}",
                        param.param_name, param.param_name,
                    ));
                }
            }
            None => {}
        }

        match system_node.get_first_state() {
            Some(state_rcref) => {
                let state_node = state_rcref.borrow();
                match &state_node.vars_opt {
                    Some(vars) => {
                        for var_rcref in vars {
                            let var_decl_node = var_rcref.borrow();
                            let expr_t = var_decl_node.initializer_expr_t_opt.as_ref().unwrap();
                            let mut expr_code = String::new();
                            expr_t.accept_to_string(self, &mut expr_code);

                            self.newline();
                            self.add_code(&format!(
                                "m._compartment_.StateVars[\"{}\"] = {}",
                                var_decl_node.name, expr_code,
                            ));
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }

        if let Some(enter_params) = &system_node.start_state_enter_params_opt {
            for param in enter_params {
                self.newline();
                self.add_code(&format!(
                    "m._compartment_.EnterArgs[\"{}\"] = {}",
                    param.param_name, param.param_name,
                ));
            }
        }

        if !domain_vec.is_empty() {
            self.newline();
            self.newline();
            self.add_code("// Override domain variables.");

            for x in domain_vec {
                let domain_var_name = x.0.clone();
                let mut domain_var_initializer = x.1.clone();
                self.newline();
                match &system_node.domain_params_opt {
                    Some(param_nodes) => {
                        for param_node in param_nodes {
                            if param_node.param_name == domain_var_name {
                                // found domain var name in domain param list
                                // init to param rather than default initial value
                                domain_var_initializer = param_node.param_name.clone();
                                break;
                            }
                        }
                        self.add_code(&format!(
                            "m.{} = {}",
                            domain_var_name, domain_var_initializer
                        ))
                    }
                    None => self.add_code(&format!(
                        "m.{} = {}",
                        domain_var_name, domain_var_initializer
                    )),
                }
            }
        }

        self.newline();
        self.newline();
        self.add_code("// Send system start event");

        if let Some(_enter_params) = &system_node.start_state_enter_params_opt {
            self.newline();
            self.add_code(&format!(
                "e := {}{{Msg:\">\", Params:m._compartment_.EnterArgs}}",
                self.config.code.frame_event_type_name
            ));
        } else {
            self.newline();
            self.add_code(&format!(
                "e := {}{{Msg:\">\"}}",
                self.config.code.frame_event_type_name
            ));
        }

        self.newline();
        self.add_code("m._mux_(&e)");
        self.newline();
        self.add_code("return m");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();

        if self.generate_state_stack {
            self.generate_stack();
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_load_fn(&mut self, domain_vec: &Vec<(String, String)>, system_node: &SystemNode) {
        self.newline();
        self.newline();
        let mut manager_param = String::new();
        if self.config.code.managed {
            manager_param = String::from(&format!("manager {}, ", self.config.code.manager));
        }
        self.add_code(&format!(
            "func Load{}({}data []byte) {} {{",
            system_node.name, manager_param, system_node.name
        ));

        self.indent();
        self.newline();
        self.add_code(&format!(
            "m := &{}Struct{{}}",
            self.first_letter_to_lower_case(&system_node.name),
        ));
        self.newline();
        if self.config.code.managed {
            self.add_code("m._manager_ = manager");
        }
        self.newline();
        self.newline();
        self.add_code("// Validate interfaces");
        self.newline();
        self.add_code(&format!(
            "var _ {} = m",
            self.first_letter_to_upper_case(&system_node.name),
        ));
        self.newline();
        if system_node.actions_block_node_opt.is_some() {
            self.add_code(&format!("var _ {}_actions = m", &system_node.name,));
        }
        self.newline();
        self.newline();
        self.add_code("// Unmarshal");
        self.newline();
        self.add_code("var marshal marshalStruct");
        self.newline();
        self.add_code("err := json.Unmarshal(data, &marshal)");
        self.newline();
        self.add_code("if err != nil {");
        self.indent();
        self.newline();
        self.add_code("return nil");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("// Initialize machine");
        self.newline();
        self.add_code(&format!(
            "m._compartment_ = &marshal.{}",
            self.config.code.compartment_type
        ));
        self.newline();
        for x in domain_vec {
            self.newline();
            self.add_code(&format!(
                "m.{} = marshal.{}",
                x.0,
                self.first_letter_to_upper_case(&x.0)
            ))
        }
        self.newline();
        self.newline();
        self.add_code("return m");
        self.newline();

        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn generate_marshal_json_fn(&mut self, domain_vec: &Vec<(String, String)>) {
        self.newline();
        self.newline();
        self.add_code(&format!(
            "func (m *{}) MarshalJSON() ([]byte, error) {{",
            self.config.code.system_struct_type
        ));
        self.indent();
        self.newline();
        self.add_code("data := marshalStruct{");
        self.indent();
        self.newline();
        self.add_code(&format!(
            "{}: *m._compartment_,",
            self.config.code.compartment_type
        ));
        for x in domain_vec {
            self.newline();
            self.add_code(&format!(
                "{}: m.{},",
                self.first_letter_to_upper_case(&x.0),
                x.0
            ))
        }
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.add_code("return json.Marshal(data)");
        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn generate_marshal_fn(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        self.add_code(&format!(
            "func (m *{}Struct) Marshal() []byte {{",
            self.first_letter_to_lower_case(&system_node.name)
        ));
        self.indent();
        self.newline();
        self.add_code("data, err := json.Marshal(m)");
        self.newline();
        self.add_code("if err != nil {");
        self.indent();
        self.newline();
        self.add_code("return nil");
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.add_code("return data");
        self.newline();

        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn format_internal_system_name(&self, system_name: &String) -> String {
        self.first_letter_to_lower_case(system_name)
    }

    //* --------------------------------------------------------------------- *//

    fn generate_compartment(&mut self) {
        self.newline();
        self.newline();
        self.add_code("//=============== Compartment ==============//");
        self.newline();
        self.newline();
        self.add_code(&format!(
            "type {} struct {{",
            self.config.code.compartment_type,
        ));
        self.indent();
        self.newline();
        self.add_code(&format!("State {}", self.config.code.state_type));
        self.newline();
        self.add_code("StateArgs map[string]interface{}");
        self.newline();
        self.add_code("StateVars map[string]interface{}");
        self.newline();
        self.add_code("EnterArgs map[string]interface{}");
        self.newline();
        self.add_code("ExitArgs map[string]interface{}");
        self.newline();
        self.add_code(&format!(
            "_forwardEvent_ *{}",
            self.config.code.frame_event_type_name
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code(&format!(
            "func New{}(state {}) *{} {{",
            self.config.code.compartment_type,
            self.config.code.state_type,
            self.config.code.compartment_type,
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "c := &{}{{State: state}}",
            self.config.code.compartment_type,
        ));
        self.newline();
        self.add_code("c.StateArgs = make(map[string]interface{})");
        self.newline();
        self.add_code("c.StateVars = make(map[string]interface{})");
        self.newline();
        self.add_code("c.EnterArgs = make(map[string]interface{})");
        self.newline();
        self.add_code("c.ExitArgs = make(map[string]interface{})");

        self.newline();
        self.add_code("return c");
        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    // This method will generate a stack with @methods Push and Pop.
    fn generate_stack(&mut self) {
        self.newline();
        self.add_code("type Stack struct {");
        self.indent();
        self.newline();
        self.add_code(&format!("stack []{}", self.config.code.compartment_type));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code(&format!(
            "func (s *Stack) Push(compartment *{}) {{",
            self.config.code.compartment_type
        ));
        self.indent();
        self.newline();
        self.add_code("s.stack = append(s.stack, *compartment)");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code(&format!(
            "func (s *Stack) Pop() *{} {{",
            self.config.code.compartment_type
        ));
        self.newline();
        self.indent();
        self.newline();
        self.add_code("l := len(s.stack)");
        self.newline();
        self.add_code("if l == 0 {");
        self.indent();
        self.newline();
        self.add_code(&format!(
            "panic({})",
            "\"Attempted to pop when history stack is empty\""
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("res := s.stack[l-1]");
        self.newline();
        self.add_code("s.stack = s.stack[:l-1]");
        self.newline();
        self.add_code("return &res");
        self.outdent();
        self.newline();
        self.add_code("}")
    }
}

//* --------------------------------------------------------------------- *//

impl AstVisitor for GolangVisitor {
    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) {
        self.system_name = system_node.name.clone();

        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        match system_node.get_first_state() {
            Some(x) => {
                self.first_state_name = x.borrow().name.clone();
                self.has_states = true;
            }
            None => {}
        }

        self.add_code(&format!("// {}", self.compiler_version));
        self.newline();
        self.add_code(
            "// get include files at https://github.com/frame-lang/frame-ancillary-files",
        );
        self.newline();
        self.add_code(&system_node.header);

        let state_prefix = if !self.config.code.state_type.is_empty() {
            self.config.code.state_type.clone()
        } else {
            String::new()
        };

        let mut domain_vec: Vec<(String, String)> = Vec::new();
        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            // get init expression and cache code
            for var_rcref in &domain_block_node.member_variables {
                let var_name = var_rcref.borrow().name.clone();
                let var = var_rcref.borrow();
                let var_init_expr = var.initializer_expr_t_opt.as_ref().unwrap();
                let mut init_expression = String::new();
                var_init_expr.accept_to_string(self, &mut init_expression);
                // push for later initialization
                domain_vec.push((var_name.clone(), init_expression));
            }
        }

        // generate New factory
        self.generate_new_fn(&domain_vec, system_node);

        self.newline();
        self.newline();
        self.add_code(&format!("type {} uint", state_prefix));
        self.newline();
        self.newline();

        // Define state constants
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            self.add_code("const (");
            self.indent();
            self.newline();
            let len = machine_block_node.states.len();
            let mut current = 0;
            for state_node_rcref in &machine_block_node.states {
                self.add_code(&self.format_state_name(&state_node_rcref.borrow().name));
                // self.format_state_name(&state_node_rcref.borrow().name);
                if current == 0 {
                    self.add_code(&format!(" {} = iota", state_prefix));
                }

                current += 1;
                if current != len {
                    self.newline();
                }
            }
            self.outdent();
            self.newline();
            self.add_code(")");
        }

        if self.config.code.marshal {
            self.newline();
            self.newline();
            self.add_code("type Marshal interface {");
            self.indent();
            self.newline();
            self.add_code("Marshal() []byte");
            self.outdent();
            self.newline();
            self.add_code("}");
        }

        // define public interface
        self.newline();
        self.newline();
        self.add_code(&format!(
            "type {} interface {{",
            self.first_letter_to_upper_case(&system_node.name)
        ));
        self.indent();
        if self.config.code.marshal {
            self.newline();
            self.add_code("Marshal");
        }

        // TODO: create visitor for this
        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            for interface_method_node_rcref in &interface_block_node.interface_methods {
                self.newline();
                let interface_method_node = interface_method_node_rcref.borrow();
                self.add_code(&format!(
                    "{}(",
                    self.first_letter_to_upper_case(&interface_method_node.name)
                ));

                self.format_parameter_list(&interface_method_node.params);

                let return_type = match &interface_method_node.return_type_opt {
                    Some(type_node) => self.format_type(type_node),
                    None => "".to_string(),
                };
                self.add_code(&format!(") {}", return_type));
            }
        }

        self.outdent();
        self.newline();
        self.add_code("}");

        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            self.newline();
            self.newline();
            self.add_code(&format!("type {}_actions interface {{", &system_node.name));
            self.indent();

            // TODO: create visitor for this
            for action_decl_node_rcref in &actions_block_node.actions {
                self.newline();
                let action_decl_node = action_decl_node_rcref.borrow();
                let action_ret_type: String = match &action_decl_node.type_opt {
                    Some(type_node) => self.format_type(type_node),
                    None => String::from(""),
                };

                let action_name = self.format_action_name(&action_decl_node.name);
                self.add_code(&format!("{}(", action_name));
                match &action_decl_node.params {
                    Some(params) => {
                        // TODO: subclass_code should be refactored
                        // as I don't think it applies here. Review.
                        let mut subclass_code = String::new();
                        self.format_actions_parameter_list(params, &mut subclass_code);
                    }
                    None => {}
                }
                self.add_code(&format!(") {}", action_ret_type));
            }

            self.outdent();
            self.newline();
            self.add_code("}");
            self.newline();
        }

        // define machine struct
        self.newline();
        self.newline();
        self.add_code(&format!(
            "type {} struct {{",
            self.config.code.system_struct_type
        ));
        self.indent();

        if self.config.code.managed {
            self.newline();
            self.add_code(&format!("_manager_ {}", &self.config.code.manager));
        }
        self.newline();
        self.add_code(&format!(
            "_compartment_ *{}",
            self.config.code.compartment_type
        ));
        self.newline();
        self.add_code(&format!(
            "_nextCompartment_ *{}",
            self.config.code.compartment_type
        ));
        if self.generate_state_stack {
            self.newline();
            self.add_code("_stateStack_ *Stack");
        }

        // declare member variables
        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            for var_rcref in &domain_block_node.member_variables {
                let var_name = var_rcref.borrow().name.clone();
                let var_type = match &var_rcref.borrow().type_opt {
                    Some(type_node) => self.format_type(type_node),
                    None => String::from("<?>"), // TODO this should generate an error instead
                };
                self.newline();
                self.add_code(&format!("{} {}", var_name, var_type));
            }
        }

        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();

        // generate the marshal struct

        if self.config.code.marshal {
            self.newline();
            self.add_code("type marshalStruct struct {");
            self.indent();
            self.newline();
            // TODO - CHANGE CONFIG
            // self.add_code(&format!(
            //     "{} {}",
            //     self.config.code.marshal_system_state_var,
            //     self.config.code.state_type,
            // ));

            self.add_code(&self.config.code.compartment_type.clone());
            if let Some(domain_block_node) = &system_node.domain_block_node_opt {
                for var_rcref in &domain_block_node.member_variables {
                    let var_name = var_rcref.borrow().name.clone();
                    let var_type = match &var_rcref.borrow().type_opt {
                        Some(type_node) => self.format_type(type_node),
                        None => String::from("<?>"), // TODO this should generate an error instead
                    };
                    self.newline();
                    self.add_code(&format!(
                        "{} {}",
                        self.first_letter_to_upper_case(&var_name),
                        var_type
                    ));
                    // get init expression and cache code
                    let var = var_rcref.borrow();
                    let var_init_expr = var.initializer_expr_t_opt.as_ref().unwrap();
                    let mut init_expression = String::new();
                    var_init_expr.accept_to_string(self, &mut init_expression);
                    // push for later initialization
                }
            }
            self.outdent();
            self.newline();
            self.add_code("}");
        }

        if self.config.code.marshal {
            // generate Load() factory
            self.format_load_fn(&domain_vec, system_node);

            // generate MarshalJSON() factory
            self.generate_marshal_json_fn(&domain_vec);
            self.generate_marshal_fn(system_node);
        }

        // end of generate constructor

        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            interface_block_node.accept(self);
        }

        // generate _mux_

        self.newline();
        self.add_code("//====================== Multiplexer ====================//");
        self.newline();
        self.newline();

        self.add_code(&format!(
            "func (m *{}Struct) _mux_(e *{}) {{",
            self.first_letter_to_lower_case(&system_node.name),
            self.config.code.frame_event_type_name
        ));
        self.indent();

        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            self.newline();
            self.add_code("switch m._compartment_.State {");
            for state_node_rcref in &machine_block_node.states {
                let state_name = self.format_state_name(&state_node_rcref.borrow().name);
                self.newline();
                self.add_code(&format!("case {}:", state_name));
                self.indent();
                self.newline();
                self.add_code(&format!("m._{}_(e)", state_name));
                self.outdent();
            }
            self.newline();
            self.add_code("}");
            self.newline();
            self.newline();
            self.add_code("if m._nextCompartment_ != nil {");
            self.indent();
            self.newline();
            self.add_code("nextCompartment := m._nextCompartment_");
            self.newline();
            self.add_code("m._nextCompartment_ = nil");
            self.newline();
            self.add_code("if nextCompartment._forwardEvent_ != nil && ");
            self.newline();
            self.add_code("   nextCompartment._forwardEvent_.Msg == \">\" {");
            self.indent();
            self.newline();
            self.add_code(&format!(
                "m._mux_(&{}{{Msg: \"<\", Params: m._compartment_.ExitArgs, Ret: nil}})",
                self.config.code.frame_event_type_name
            ));
            self.newline();
            self.add_code("m._compartment_ = nextCompartment");
            self.newline();
            self.add_code("m._mux_(nextCompartment._forwardEvent_)");
            self.outdent();
            self.newline();
            self.add_code("} else {");
            self.indent();
            self.newline();
            self.add_code("m._do_transition_(nextCompartment)");
            self.newline();
            self.add_code("if nextCompartment._forwardEvent_ != nil {");
            self.indent();
            self.newline();
            self.add_code("m._mux_(nextCompartment._forwardEvent_)");
            self.outdent();
            self.newline();
            self.add_code("}");
            self.outdent();
            self.newline();
            self.add_code("}");
            self.newline();
            self.add_code("nextCompartment._forwardEvent_ = nil");
            self.outdent();
            self.newline();
            self.add_code("}");
        }
        self.outdent();
        self.newline();
        self.add_code("}");

        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }

        if self.has_states {
            self.generate_machinery(system_node);
        }

        // TODO: add comments back
        // self.newline();
        // self.generate_comment(system_node.line);
        // self.newline();
        // self.outdent();
        // self.newline();
        // self.add_code("}");
        // self.newline();

        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            self.newline();
            self.add_code("//===================== Actions Block ===================//");
            self.newline();

            for action_rcref in &actions_block_node.actions {
                let action_node = action_rcref.borrow();
                if action_node.code_opt.is_some() {
                    action_node.accept_action_impl(self);
                }
            }

            self.generate_subclass(system_node);
        }

        self.generate_compartment();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_messages_enum(&mut self, _interface_block_node: &InterfaceBlockNode) {
        panic!("Error - visit_frame_messages_enum() only used in Rust.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_parameters(&mut self, _interface_block_node: &InterfaceBlockNode) {
        panic!("visit_interface_parameters() not valid for target language.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
    ) {
        self.add_code(&format!(
            "m.{}",
            self.first_letter_to_upper_case(
                &interface_method_call_expr_node
                    .identifier
                    .name
                    .lexeme
                    .to_string()
            )
        ));
        interface_method_call_expr_node.call_expr_list.accept(self);

        // TODO: review this return as I think it is a nop.
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node_to_string(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
        output: &mut String,
    ) {
        output.push_str(&format!(
            "m.{}",
            self.first_letter_to_upper_case(
                &interface_method_call_expr_node
                    .identifier
                    .name
                    .lexeme
                    .to_string()
            )
        ));
        interface_method_call_expr_node
            .call_expr_list
            .accept_to_string(self, output);

        // TODO: review this return as I think it is a nop.
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_block_node(&mut self, interface_block_node: &InterfaceBlockNode) {
        self.newline();
        self.add_code("//===================== Interface Block ===================//");
        self.newline();

        for interface_method_node_rcref in &interface_block_node.interface_methods {
            let interface_method_node = interface_method_node_rcref.borrow();
            interface_method_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_node(&mut self, interface_method_node: &InterfaceMethodNode) {
        self.newline();
        let return_type = match &interface_method_node.return_type_opt {
            Some(type_node) => self.format_type(type_node),
            None => "".to_string(),
        };

        // see if an alias exists.
        let method_name_or_alias: &String;

        match &interface_method_node.alias {
            Some(alias_message_node) => {
                method_name_or_alias = &alias_message_node.name;
            }
            None => {
                method_name_or_alias = &interface_method_node.name;
            }
        }

        self.add_code(&format!(
            "func (m *{}Struct) {}(",
            self.first_letter_to_lower_case(&self.system_name),
            self.first_letter_to_upper_case(&interface_method_node.name),
        ));

        self.format_parameter_list(&interface_method_node.params);

        self.add_code(&format!(") {} {{", return_type));
        self.indent();
        if interface_method_node.params.is_some() {
            //params_param_code = String::from("params");
            self.newline();
            self.add_code("params := make(map[string]interface{})");
            match &interface_method_node.params {
                Some(params) => {
                    for param in params {
                        let pname = &param.param_name;
                        self.newline();
                        self.add_code(&format!("params[\"{}\"] = {}", pname, pname));
                    }
                }
                None => {}
            }
        }
        self.newline();
        self.add_code(&format!(
            "e := {}{{Msg:\"{}\"",
            self.config.code.frame_event_type_name, method_name_or_alias,
        ));
        if interface_method_node.params.is_some() {
            self.add_code(", Params:params");
        }
        self.add_code("}");
        self.newline();
        self.add_code("m._mux_(&e)");

        match &interface_method_node.return_type_opt {
            Some(return_type) => {
                self.newline();
                self.add_code(&format!("return  e.Ret.({})", return_type.get_type_str()));
            }
            None => {}
        }

        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_machine_block_node(&mut self, machine_block_node: &MachineBlockNode) {
        self.newline();
        self.newline();
        self.add_code("//===================== Machine Block ===================//");

        for state_node_rcref in &machine_block_node.states {
            state_node_rcref.borrow().accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_block_node(&mut self, actions_block_node: &ActionsBlockNode) {
        self.newline();
        self.newline();
        self.add_code("//===================== Actions Block ===================//");
        self.newline();

        for action_decl_node_rcref in &actions_block_node.actions {
            let action_decl_node = action_decl_node_rcref.borrow();
            action_decl_node.accept(self);
        }

        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_node_rust_trait(&mut self, _: &ActionsBlockNode) {
        panic!("Error - visit_action_node_rust_trait() not implemented.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_node_rust_impl(&mut self, _: &ActionsBlockNode) {
        panic!("Error - visit_actions_node_rust_impl() not implemented.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_block_node(&mut self, domain_block_node: &DomainBlockNode) {
        self.newline();
        self.newline();

        for variable_decl_node_rcref in &domain_block_node.member_variables {
            let variable_decl_node = variable_decl_node_rcref.borrow();
            variable_decl_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_node(&mut self, state_node: &StateNode) {
        self.generate_comment(state_node.line);
        self.current_state_name_opt = Some(state_node.name.clone());
        self.newline();
        self.newline();
        let type_node = TypeNode::new(
            false,
            false,
            Some(FrameEventPart::Event {
                is_reference: false,
            }),
            String::new(),
        );
        self.add_code(&format!(
            "func (m *{}Struct) _{}_(e *{}) {{",
            self.first_letter_to_lower_case(&self.system_name),
            self.format_state_name(&state_node.name),
            self.format_type(&type_node)
        ));
        self.indent();

        if let Some(calls) = &state_node.calls_opt {
            for call in calls {
                self.newline();
                call.accept(self);
            }
        }

        self.first_event_handler = true; // context for formatting
        self.newline();
        self.add_code("switch e.Msg {");

        if !state_node.evt_handlers_rcref.is_empty() {
            for evt_handler_node in &state_node.evt_handlers_rcref {
                evt_handler_node.as_ref().borrow().accept(self);
            }
        }
        self.newline();
        self.add_code("}");

        match &state_node.dispatch_opt {
            Some(dispatch) => {
                dispatch.accept(self);
            }
            None => {}
        }

        self.outdent();
        self.newline();
        self.add_code("}");

        self.current_state_name_opt = None;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_node(&mut self, evt_handler_node: &EventHandlerNode) {
        self.current_event_ret_type = evt_handler_node.get_event_ret_type();
        self.newline();
        self.generate_comment(evt_handler_node.line);
        //        let mut generate_final_close_paren = true;
        if let MessageType::CustomMessage { message_node } = &evt_handler_node.msg_t {
            // Get msg name so we can look up the type of parameters
            // for this event later.
            self.current_event_msg = message_node.name.clone();

            self.add_code(&format!("case \"{}\":", message_node.name));
        }
        self.generate_comment(evt_handler_node.line);

        self.indent();

        match &evt_handler_node.msg_t {
            MessageType::CustomMessage { .. } => {
                // Note: this is a bit convoluted as we cant use self.add_code() inside the
                // if statements as it is a double borrow (sigh).

                let params_code: Vec<String> = Vec::new();

                // NOW add the code. Sheesh.
                for param_code in params_code {
                    self.newline();
                    self.add_code(&param_code);
                }
            }
            _ => {}
        }

        // Generate statements
        self.visit_decl_stmts(&evt_handler_node.statements);

        let terminator_node = &evt_handler_node.terminator_node;
        terminator_node.accept(self);
        self.outdent();

        // this controls formatting here
        self.first_event_handler = false;
        self.current_event_msg = String::new();
        self.current_event_ret_type = String::new();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_terminator_node(
        &mut self,
        evt_handler_terminator_node: &TerminatorExpr,
    ) {
        self.newline();
        match &evt_handler_terminator_node.terminator_type {
            TerminatorType::Return => match &evt_handler_terminator_node.return_expr_t_opt {
                Some(expr_t) => {
                    self.add_code("e.Ret = ");
                    expr_t.accept(self);
                    self.generate_return();
                    self.newline();
                }
                None => self.generate_return(),
            },
            TerminatorType::Continue => {
                self.generate_return_if_transitioned();
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_statement_node(&mut self, method_call_statement: &CallStmtNode) {
        self.newline();
        method_call_statement.call_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node(&mut self, method_call: &CallExprNode) {
        if let Some(call_chain) = &method_call.call_chain {
            for callable in call_chain {
                callable.callable_accept(self);
                self.add_code(".");
            }
        }

        self.add_code(&method_call.identifier.name.lexeme.to_string());

        method_call.call_expr_list.accept(self);

        self.add_code("");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node_to_string(
        &mut self,
        method_call: &CallExprNode,
        output: &mut String,
    ) {
        if let Some(call_chain) = &method_call.call_chain {
            for callable in call_chain {
                callable.callable_accept(self);
                output.push('.');
            }
        }

        output.push_str(&method_call.identifier.name.lexeme.to_string());

        method_call.call_expr_list.accept_to_string(self, output);

        output.push_str("");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node(&mut self, call_expr_list: &CallExprListNode) {
        let mut separator = "";
        self.add_code("(");

        for expr in &call_expr_list.exprs_t {
            self.add_code(separator);
            expr.accept(self);
            separator = ",";
        }

        self.add_code(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node_to_string(
        &mut self,
        call_expr_list: &CallExprListNode,
        output: &mut String,
    ) {
        let mut separator = "";
        output.push('(');

        for expr in &call_expr_list.exprs_t {
            output.push_str(separator);
            expr.accept_to_string(self, output);
            separator = ",";
        }

        output.push(')');
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node(&mut self, action_call: &ActionCallExprNode) {
        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        self.add_code(&format!("m.{}", &action_name));
        action_call.call_expr_list.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(
        &mut self,
        action_call: &ActionCallExprNode,
        output: &mut String,
    ) {
        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        output.push_str(&format!("m.{}", action_name.as_str()));
        action_call.call_expr_list.accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(&mut self, action_call_stmt_node: &ActionCallStmtNode) {
        self.newline();
        action_call_stmt_node.action_call_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_transition_statement_node(&mut self, transition_statement: &TransitionStatementNode) {
        match &transition_statement.target_state_context_t {
            StateContextType::StateRef { .. } => {
                self.generate_state_ref_transition(transition_statement)
            }
            StateContextType::StateStackPop {} => {
                self.generate_state_stack_pop_transition(transition_statement)
            }
        };

        self.config.code.this_branch_transitioned = true;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_ref_node(&mut self, state_ref: &StateRefNode) {
        self.add_code(&state_ref.name.to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_change_state_statement_node(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) {
        match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { .. } => {
                self.generate_state_ref_change_state(change_state_stmt_node)
            }
            StateContextType::StateStackPop {} => {
                self.generate_state_stack_pop_change_state(change_state_stmt_node)
            }
        };

        self.config.code.this_branch_transitioned = true;
    }

    //* --------------------------------------------------------------------- *//

    // TODO: ??
    fn visit_parameter_node(&mut self, _: &ParameterNode) {
        // self.add_code(&format!("{}",parameter_node.name));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_dispatch_node(&mut self, dispatch_node: &DispatchNode) {
        self.newline();
        self.add_code(&format!(
            "m._{}_(e)",
            self.format_state_name(&dispatch_node.target_state_ref.name)
        ));
        self.generate_comment(dispatch_node.line);
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_test_statement_node(&mut self, test_stmt_node: &TestStatementNode) {
        match &test_stmt_node.test_t {
            TestType::BoolTest { bool_test_node } => {
                bool_test_node.accept(self);
            }
            TestType::StringMatchTest {
                string_match_test_node,
            } => {
                string_match_test_node.accept(self);
            }
            TestType::NumberMatchTest {
                number_match_test_node,
            } => {
                number_match_test_node.accept(self);
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_node(&mut self, bool_test_node: &BoolTestNode) {
        let mut if_or_else_if = "if ";

        self.newline();
        for branch_node in &bool_test_node.conditional_branch_nodes {
            if branch_node.is_negated {
                self.add_code(&format!("{}!(", if_or_else_if));
            } else {
                self.add_code(if_or_else_if);
            }

            branch_node.expr_t.accept(self);

            if branch_node.is_negated {
                self.add_code(")");
            }
            self.add_code(" {");
            self.indent();

            branch_node.accept(self);
            self.generate_return_if_transitioned();
            self.outdent();
            self.newline();
            self.add_code("}");

            if_or_else_if = " else if ";
        }

        // (':' bool_test_else_branch)?
        if let Some(bool_test_else_branch_node) = &bool_test_node.else_branch_node_opt {
            bool_test_else_branch_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    // NOTE: Interface method calls must be treated specially since they may transition.
    //
    // The current approach is conservative, essentially assuming that an interface method call
    // always transitions. This assumption imposes the following restrictions:
    //
    //  * Interface method calls cannot occur in a chain (they must be a standalone call).
    //  * Interface method calls terminate the execution of their handler (like transitions).
    //
    // It would be possible to lift these restrictions and track the execution of a handler more
    // precisely, but this would require embedding some logic in the generated code and would make
    // handlers harder to reason about. The conservative approach has the advantage of both
    // simplifying the implementation and reasoning about Frame programs.

    fn visit_call_chain_literal_statement_node(
        &mut self,
        method_call_chain_literal_stmt_node: &CallChainLiteralStmtNode,
    ) {
        self.newline();

        // special case for interface method calls
        let call_chain = &method_call_chain_literal_stmt_node
            .call_chain_literal_expr_node
            .call_chain;
        if call_chain.len() == 1 {
            if let CallChainLiteralNodeType::InterfaceMethodCallT {
                interface_method_call_expr_node,
            } = &call_chain[0]
            {
                self.config.code.this_branch_transitioned = true;
                interface_method_call_expr_node.accept(self);
                self.generate_return();
                return;
            }
        }

        // standard case
        method_call_chain_literal_stmt_node
            .call_chain_literal_expr_node
            .accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node(
        &mut self,
        method_call_chain_expression_node: &CallChainLiteralExprNode,
    ) {
        // TODO: maybe put this in an AST node

        let mut separator = "";

        for node in &method_call_chain_expression_node.call_chain {
            self.add_code(separator);
            match &node {
                CallChainLiteralNodeType::IdentifierNodeT { id_node } => {
                    id_node.accept(self);
                }
                CallChainLiteralNodeType::CallT { call } => {
                    call.accept(self);
                }
                CallChainLiteralNodeType::InterfaceMethodCallT { .. } => {
                    self.errors.push(String::from(
                        "Error: Interface method calls may not appear in call chains.",
                    ));
                }
                CallChainLiteralNodeType::ActionCallT {
                    action_call_expr_node,
                } => {
                    action_call_expr_node.accept(self);
                }
                CallChainLiteralNodeType::VariableNodeT { var_node } => {
                    self.visiting_call_chain_literal_variable = true;
                    var_node.accept(self);
                    self.visiting_call_chain_literal_variable = false;
                }
            }
            separator = ".";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node_to_string(
        &mut self,
        method_call_chain_expression_node: &CallChainLiteralExprNode,
        output: &mut String,
    ) {
        let mut separator = "";

        for node in &method_call_chain_expression_node.call_chain {
            output.push_str(separator);
            match &node {
                CallChainLiteralNodeType::IdentifierNodeT { id_node } => {
                    id_node.accept_to_string(self, output);
                }
                CallChainLiteralNodeType::CallT { call } => {
                    call.accept_to_string(self, output);
                }
                CallChainLiteralNodeType::InterfaceMethodCallT { .. } => {
                    self.errors.push(String::from(
                        "Error: Interface method calls may not appear in call chains.",
                    ));
                    //                    interface_method_call_expr_node.accept_to_string(self, output);
                }
                CallChainLiteralNodeType::ActionCallT {
                    action_call_expr_node,
                } => {
                    action_call_expr_node.accept_to_string(self, output);
                }
                CallChainLiteralNodeType::VariableNodeT { var_node } => {
                    var_node.accept_to_string(self, output);
                }
            }
            separator = ".";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_conditional_branch_node(
        &mut self,
        bool_test_true_branch_node: &BoolTestConditionalBranchNode,
    ) {
        self.visit_decl_stmts(&bool_test_true_branch_node.statements);

        match &bool_test_true_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code("e.Ret = ");
                            expr_t.accept(self);
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_else_branch_node(
        &mut self,
        bool_test_else_branch_node: &BoolTestElseBranchNode,
    ) {
        self.add_code(" else {");
        self.indent();

        self.visit_decl_stmts(&bool_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &bool_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code("e.Ret = ");
                            expr_t.accept(self);
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }

        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_node(&mut self, string_match_test_node: &StringMatchTestNode) {
        let mut if_or_else_if = "if";

        self.newline();
        for match_branch_node in &string_match_test_node.match_branch_nodes {
            self.add_code(&format!("{} ", if_or_else_if));
            // TODO: use string_match_test_node.expr_t.accept(self) ?
            match &string_match_test_node.expr_t {
                ExprType::CallExprT {
                    call_expr_node: method_call_expr_node,
                } => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT {
                    action_call_expr_node,
                } => action_call_expr_node.accept(self),
                ExprType::CallChainLiteralExprT {
                    call_chain_expr_node,
                } => call_chain_expr_node.accept(self),
                ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                ExprType::ExprListT { expr_list_node } => {
                    // must be only 1 expression in the list
                    if expr_list_node.exprs_t.len() != 1 {
                        // TODO: how to do this better.
                        self.errors
                            .push("Error - expression list is not testable.".to_string());
                    }
                    let x = expr_list_node.exprs_t.first().unwrap();
                    x.accept(self);
                }

                _ => self.errors.push("TODO".to_string()),
            }

            // TODO: use accept
            // self.add_code(&format!(" == \""));
            // match_branch_node.string_match_pattern_node.accept(self);
            // self.add_code(&format!("\") {{"));

            let mut first_match = true;
            for match_string in &match_branch_node
                .string_match_pattern_node
                .match_pattern_strings
            {
                if first_match {
                    self.add_code(&format!(" == \"{}\"", match_string));
                    first_match = false;
                } else {
                    self.add_code(" || ");
                    match &string_match_test_node.expr_t {
                        ExprType::CallExprT {
                            call_expr_node: method_call_expr_node,
                        } => method_call_expr_node.accept(self),
                        ExprType::ActionCallExprT {
                            action_call_expr_node,
                        } => action_call_expr_node.accept(self),
                        ExprType::CallChainLiteralExprT {
                            call_chain_expr_node,
                        } => call_chain_expr_node.accept(self),
                        ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                        _ => self.errors.push("TODO".to_string()),
                    }
                    self.add_code(&format!(" == \"{}\"", match_string));
                }
            }
            self.add_code(" {");
            self.indent();

            match_branch_node.accept(self);
            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code("}");

            if_or_else_if = " else if";
        }

        // (':' string_test_else_branch)?
        if let Some(string_match_else_branch_node) = &string_match_test_node.else_branch_node_opt {
            string_match_else_branch_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_match_branch_node(
        &mut self,
        string_match_test_match_branch_node: &StringMatchTestMatchBranchNode,
    ) {
        self.visit_decl_stmts(&string_match_test_match_branch_node.statements);

        match &string_match_test_match_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code("e.Ret = ");
                            expr_t.accept(self);
                            //                            self.add_code(";");
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_else_branch_node(
        &mut self,
        string_match_test_else_branch_node: &StringMatchTestElseBranchNode,
    ) {
        self.add_code(" else {");
        self.indent();

        self.visit_decl_stmts(&string_match_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &string_match_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code("e.Ret = ");
                            expr_t.accept(self);
                            //                           self.add_code(";");
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }

        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_pattern_node(
        &mut self,
        _string_match_test_else_branch_node: &StringMatchTestPatternNode,
    ) {
        // TODO
        self.errors.push("Not implemented.".to_string());
    }

    //-----------------------------------------------------//

    fn visit_number_match_test_node(&mut self, number_match_test_node: &NumberMatchTestNode) {
        let mut if_or_else_if = "if";

        self.newline();
        for match_branch_node in &number_match_test_node.match_branch_nodes {
            self.add_code(&format!("{} ", if_or_else_if));
            match &number_match_test_node.expr_t {
                ExprType::CallExprT {
                    call_expr_node: method_call_expr_node,
                } => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT {
                    action_call_expr_node,
                } => action_call_expr_node.accept(self),
                ExprType::CallChainLiteralExprT {
                    call_chain_expr_node,
                } => call_chain_expr_node.accept(self),
                ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                ExprType::ExprListT { expr_list_node } => {
                    // must be only 1 expression in the list
                    if expr_list_node.exprs_t.len() != 1 {
                        // TODO: how to do this better.
                        self.errors
                            .push("Error - expression list is not testable.".to_string());
                    }
                    let x = expr_list_node.exprs_t.first().unwrap();
                    x.accept(self);
                }
                _ => self.errors.push("TODO.".to_string()),
            }

            let mut first_match = true;
            for match_number in &match_branch_node.number_match_pattern_nodes {
                if first_match {
                    self.add_code(&format!(" == {}", match_number.match_pattern_number));
                    first_match = false;
                } else {
                    self.add_code(" || ");
                    match &number_match_test_node.expr_t {
                        ExprType::CallExprT {
                            call_expr_node: method_call_expr_node,
                        } => method_call_expr_node.accept(self),
                        ExprType::ActionCallExprT {
                            action_call_expr_node,
                        } => action_call_expr_node.accept(self),
                        ExprType::CallChainLiteralExprT {
                            call_chain_expr_node,
                        } => call_chain_expr_node.accept(self),
                        ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                        _ => self.errors.push("TODO.".to_string()),
                    }
                    self.add_code(&format!(" == {}", match_number.match_pattern_number));
                }
            }

            self.add_code(" {");
            self.indent();

            match_branch_node.accept(self);
            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code("}");

            if_or_else_if = " else if";
        }

        // (':' number_test_else_branch)?
        if let Some(number_match_else_branch_node) = &number_match_test_node.else_branch_node_opt {
            number_match_else_branch_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_match_branch_node(
        &mut self,
        number_match_test_match_branch_node: &NumberMatchTestMatchBranchNode,
    ) {
        self.visit_decl_stmts(&number_match_test_match_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &number_match_test_match_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code("e.Ret = ");
                            expr_t.accept(self);
                            //                            self.add_code(";");
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_else_branch_node(
        &mut self,
        number_match_test_else_branch_node: &NumberMatchTestElseBranchNode,
    ) {
        self.add_code(" else {");
        self.indent();

        self.visit_decl_stmts(&number_match_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &number_match_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code("e.Ret = ");
                            expr_t.accept(self);
                            //                            self.add_code(";");
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }

        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_pattern_node(
        &mut self,
        match_pattern_node: &NumberMatchTestPatternNode,
    ) {
        self.add_code(&match_pattern_node.match_pattern_number.to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node(&mut self, expr_list: &ExprListNode) {
        let mut separator = "";
        self.add_code("(");
        for expr in &expr_list.exprs_t {
            self.add_code(separator);
            expr.accept(self);
            separator = ",";
        }
        self.add_code(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node_to_string(
        &mut self,
        expr_list: &ExprListNode,
        output: &mut String,
    ) {
        //        self.add_code(&format!("{}(e);\n",dispatch_node.target_state_ref.name));

        let mut separator = "";
        output.push('(');
        for expr in &expr_list.exprs_t {
            output.push_str(separator);
            expr.accept_to_string(self, output);
            separator = ",";
        }
        output.push(')');
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node(&mut self, literal_expression_node: &LiteralExprNode) {
        match &literal_expression_node.token_t {
            TokenType::Number => self.add_code(&literal_expression_node.value.to_string()),
            TokenType::SuperString => self.add_code(&literal_expression_node.value.to_string()),
            TokenType::String => self.add_code(&format!("\"{}\"", literal_expression_node.value)),
            TokenType::True => self.add_code("true"),
            TokenType::False => self.add_code("false"),
            TokenType::Null => self.add_code("nil"),
            TokenType::Nil => self.add_code("nil"),
            _ => self
                .errors
                .push("TODO: visit_literal_expression_node".to_string()),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node_to_string(
        &mut self,
        literal_expression_node: &LiteralExprNode,
        output: &mut String,
    ) {
        // TODO: make a focused enum or the literals
        match &literal_expression_node.token_t {
            TokenType::Number => output.push_str(&literal_expression_node.value.to_string()),
            TokenType::String => {
                output.push_str(&format!("\"{}\"", literal_expression_node.value));
            }
            TokenType::True => {
                output.push_str("true");
            }
            TokenType::False => {
                output.push_str("false");
            }
            TokenType::Nil => {
                output.push_str("nil");
            }
            TokenType::Null => {
                output.push_str("nil");
            }
            TokenType::SuperString => {
                output.push_str(&literal_expression_node.value.to_string());
            }
            _ => self
                .errors
                .push("TODO: visit_literal_expression_node_to_string".to_string()),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node(&mut self, identifier_node: &IdentifierNode) {
        self.add_code(&identifier_node.name.lexeme.to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node_to_string(
        &mut self,
        identifier_node: &IdentifierNode,
        output: &mut String,
    ) {
        output.push_str(&identifier_node.name.lexeme.to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node(
        &mut self,
        _state_stack_operation_node: &StateStackOperationNode,
    ) {
        //        self.add_code(&format!("{}",identifier_node.name.lexeme));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node_to_string(
        &mut self,
        _state_stack_operation_node: &StateStackOperationNode,
        _output: &mut String,
    ) {
        //        self.add_code(&format!("{}",identifier_node.name.lexeme));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_statement_node(
        &mut self,
        state_stack_op_statement_node: &StateStackOperationStatementNode,
    ) {
        match state_stack_op_statement_node
            .state_stack_operation_node
            .operation_t
        {
            StateStackOperationType::Push => {
                self.newline();
                self.add_code("m._stateStack_push_(m._compartment_)");
            }
            StateStackOperationType::Pop => {
                self.add_code(&format!(
                    "{} compartment = _stateStack_pop_()",
                    self.config.code.compartment_type
                ));
            }
        }
    }
    //* --------------------------------------------------------------------- *//

    fn visit_state_context_node(&mut self, _state_context_node: &StateContextNode) {
        // TODO
        //        self.add_code(&format!("{}",identifier_node.name.lexeme));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_event_part(&mut self, frame_event_part: &FrameEventPart) {
        // TODO: make this code generate from settings
        match frame_event_part {
            FrameEventPart::Event {
                is_reference: _is_reference,
            } => self.add_code("e"),
            FrameEventPart::Message {
                is_reference: _is_reference,
            } => self.add_code("e.Msg"),
            FrameEventPart::Param {
                param_symbol_rcref,
                is_reference: _is_reference,
            } => {
                self.add_code(&format!(
                    "e.Params[\"{}\"]",
                    param_symbol_rcref.borrow().name
                ));
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    let event_symbol_opt_rcref = self
                        .arcanium
                        .get_event(&self.current_event_msg, &Option::None);
                    let x = &event_symbol_opt_rcref.unwrap();
                    let event_symbol = x.borrow();
                    let param_type: String = match &event_symbol.params_opt {
                        Some(param_symbols) => {
                            let mut param_type: String = String::new();
                            for param_symbol in param_symbols {
                                if param_symbol.name == param_symbol_rcref.borrow().name {
                                    match &param_symbol.param_type_opt {
                                        Some(type_node) => {
                                            param_type = self.format_type(type_node);
                                        }
                                        None => {
                                            self.errors.push(format!(
                                                "Error: {}[{}] type is not declared.",
                                                event_symbol.msg, param_symbol.name
                                            ));
                                            return;
                                        }
                                    };
                                    break;
                                }
                            }
                            param_type
                        }
                        None => {
                            self.errors.push(format!(
                                "Error: {}[{}] type is not declared.",
                                event_symbol.msg,
                                param_symbol_rcref.borrow().name
                            ));
                            "".to_string()
                        }
                    };
                    self.add_code(&format!(".({})", param_type));
                }
            }
            FrameEventPart::Return {
                is_reference: _is_reference,
            } => {
                self.add_code("e.Ret");
                if self.expr_context == Rvalue || self.expr_context == ExprContext::None {
                    self.add_code(&format!(".({})", &self.current_event_ret_type));
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    // TODO: this is not the right framemessage codegen
    fn visit_frame_event_part_to_string(
        &mut self,
        frame_event_part: &FrameEventPart,
        output: &mut String,
    ) {
        // TODO: make this code generate from settings
        match frame_event_part {
            FrameEventPart::Event {
                is_reference: _is_reference,
            } => output.push('e'),
            FrameEventPart::Message {
                is_reference: _is_reference,
            } => output.push_str("e.Msg"),
            FrameEventPart::Param {
                param_symbol_rcref,
                is_reference: _is_reference,
            } => {
                output.push_str(&format!(
                    "e.Params[\"{}\"]",
                    param_symbol_rcref.borrow().name,
                ));
                if self.expr_context == Rvalue || self.expr_context == ExprContext::None {
                    let var_type = match &param_symbol_rcref.borrow().param_type_opt {
                        Some(type_node) => self.format_type(type_node),
                        None => String::from("<?>"),
                    };
                    output.push_str(&format!(".({})", var_type));
                }
            }
            FrameEventPart::Return {
                is_reference: _is_reference,
            } => {
                output.push_str("e.Ret");
                if self.expr_context == Rvalue || self.expr_context == ExprContext::None {
                    output.push_str(&format!(".({})", &self.current_event_ret_type));
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_decl_node(&mut self, action_decl_node: &ActionNode) {
        let mut subclass_code = String::new();

        self.newline();

        let action_ret_type: String = match &action_decl_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from(""),
        };

        let action_name = self.format_action_name(&action_decl_node.name);
        self.add_code(&format!(
            "func (m *{}Struct) {}(",
            self.first_letter_to_lower_case(&self.system_name),
            action_name
        ));

        match &action_decl_node.params {
            Some(params) => {
                self.format_actions_parameter_list(params, &mut subclass_code);
            }
            None => {}
        }

        self.add_code(&format!(") {} {{}}", action_ret_type));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_impl_node(&mut self, action_node: &ActionNode) {
        let mut subclass_code = String::new();

        self.newline();
        self.newline();
        let action_ret_type: String = match &action_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from(""),
        };

        let action_name = self.format_action_name(&action_node.name);
        self.add_code(&format!(
            "func (m *{}Struct) {}(",
            self.first_letter_to_lower_case(&self.system_name),
            action_name
        ));

        match &action_node.params {
            Some(params) => {
                self.format_actions_parameter_list(params, &mut subclass_code);
            }
            None => {}
        }

        self.add_code(&format!(") {} {{", action_ret_type));
        self.indent();
        self.newline();
        self.add_code(action_node.code_opt.as_ref().unwrap().as_str());
        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
        self.visit_variable_decl_node(variable_decl_node);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
        let var_type = match &variable_decl_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from(""),
        };
        let var_name = &variable_decl_node.name;
        let var_init_expr = &variable_decl_node.initializer_expr_t_opt.as_ref().unwrap();
        self.newline();
        let mut code = String::new();
        // TODO: this may be a bit of a hack, but need to format e.Param[] differently
        // based on if lval or rval.
        self.current_var_type = var_type.clone(); // used for casting
        self.expr_context = ExprContext::Rvalue;
        var_init_expr.accept_to_string(self, &mut code);
        self.expr_context = ExprContext::None;
        self.current_var_type = "".to_string();
        self.add_code(&format!("var {} {} = {}", var_name, var_type, code));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node(&mut self, variable_node: &VariableNode) {
        let code = self.format_variable_expr(variable_node);
        self.add_code(&code);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node_to_string(
        &mut self,
        variable_node: &VariableNode,
        output: &mut String,
    ) {
        let code = self.format_variable_expr(variable_node);
        output.push_str(&code);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_stmt_node(&mut self, variable_stmt_node: &VariableStmtNode) {
        // TODO: what is this line about?
        self.generate_comment(variable_stmt_node.get_line());
        self.newline();
        let code = self.format_variable_expr(&variable_stmt_node.var_node);
        self.add_code(&code);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node(&mut self, assignment_expr_node: &AssignmentExprNode) {
        self.newline();
        let mut code = String::new();
        assignment_expr_node.accept_to_string(self, &mut code);
        self.add_code(&code);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node_to_string(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
        output: &mut String,
    ) {
        self.generate_comment(assignment_expr_node.line);
        self.expr_context = ExprContext::Lvalue;
        assignment_expr_node
            .l_value_box
            .accept_to_string(self, output);
        output.push_str(" = ");
        self.expr_context = ExprContext::Rvalue;
        assignment_expr_node
            .r_value_box
            .accept_to_string(self, output);
        self.expr_context = ExprContext::None;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_statement_node(&mut self, assignment_stmt_node: &AssignmentStmtNode) {
        self.generate_comment(assignment_stmt_node.get_line());
        assignment_stmt_node.assignment_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node(&mut self, unary_expr_node: &UnaryExprNode) {
        // TODO
        //       self.generate_comment(assignment_expr_node.line);
        unary_expr_node.operator.accept(self);
        unary_expr_node.right_rcref.borrow().accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node_to_string(
        &mut self,
        unary_expr_node: &UnaryExprNode,
        output: &mut String,
    ) {
        // TODO
        //       self.generate_comment(assignment_expr_node.line);
        unary_expr_node.operator.accept_to_string(self, output);
        unary_expr_node
            .right_rcref
            .borrow()
            .accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node(&mut self, binary_expr_node: &BinaryExprNode) {
        // TODO
        //       self.generate_comment(assignment_expr_node.line);

        if binary_expr_node.operator == OperatorType::LogicalXor {
            self.add_code("((");
            binary_expr_node.left_rcref.borrow().accept(self);
            self.add_code(") && !(");
            binary_expr_node.right_rcref.borrow().accept(self);
            self.add_code(")) || (!(");
            binary_expr_node.left_rcref.borrow().accept(self);
            self.add_code(") && (");
            binary_expr_node.right_rcref.borrow().accept(self);
            self.add_code("))");
        } else {
            binary_expr_node.left_rcref.borrow().accept(self);
            binary_expr_node.operator.accept(self);
            binary_expr_node.right_rcref.borrow().accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node_to_string(
        &mut self,
        binary_expr_node: &BinaryExprNode,
        output: &mut String,
    ) {
        if binary_expr_node.operator == OperatorType::LogicalXor {
            output.push_str("((");
            binary_expr_node
                .left_rcref
                .borrow()
                .accept_to_string(self, output);
            output.push_str(") && !(");
            binary_expr_node
                .right_rcref
                .borrow()
                .accept_to_string(self, output);
            output.push_str(")) || (!(");
            binary_expr_node
                .left_rcref
                .borrow()
                .accept_to_string(self, output);
            output.push_str(") && (");
            binary_expr_node
                .right_rcref
                .borrow()
                .accept_to_string(self, output);
            output.push_str("))");
        } else {
            binary_expr_node
                .left_rcref
                .borrow()
                .accept_to_string(self, output);
            binary_expr_node.operator.accept_to_string(self, output);
            binary_expr_node
                .right_rcref
                .borrow()
                .accept_to_string(self, output);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operator_type(&mut self, operator_type: &OperatorType) {
        match operator_type {
            OperatorType::Plus => self.add_code(" + "),
            OperatorType::Minus => self.add_code(" - "),
            OperatorType::Negated => self.add_code("-"),
            OperatorType::Multiply => self.add_code(" * "),
            OperatorType::Divide => self.add_code(" / "),
            OperatorType::Greater => self.add_code(" > "),
            OperatorType::GreaterEqual => self.add_code(" >= "),
            OperatorType::Less => self.add_code(" < "),
            OperatorType::LessEqual => self.add_code(" <= "),
            OperatorType::Not => self.add_code("!"),
            OperatorType::EqualEqual => self.add_code(" == "),
            OperatorType::NotEqual => self.add_code(" != "),
            OperatorType::LogicalAnd => self.add_code(" && "),
            OperatorType::LogicalOr => self.add_code(" || "),
            OperatorType::LogicalXor => self.add_code(""),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operator_type_to_string(&mut self, operator_type: &OperatorType, output: &mut String) {
        match operator_type {
            OperatorType::Plus => output.push_str(" + "),
            OperatorType::Minus => output.push_str(" - "),
            OperatorType::Negated => output.push('-'),
            OperatorType::Multiply => output.push_str(" * "),
            OperatorType::Divide => output.push_str(" / "),
            OperatorType::Greater => output.push_str(" > "),
            OperatorType::GreaterEqual => output.push_str(" >= "),
            OperatorType::Less => output.push_str(" < "),
            OperatorType::LessEqual => output.push_str(" <= "),
            OperatorType::Not => output.push('!'),
            OperatorType::EqualEqual => output.push_str(" == "),
            OperatorType::NotEqual => output.push_str(" != "),
            OperatorType::LogicalAnd => output.push_str(" && "),
            OperatorType::LogicalOr => output.push_str(" || "),
            OperatorType::LogicalXor => output.push_str(""),
        }
    }
}

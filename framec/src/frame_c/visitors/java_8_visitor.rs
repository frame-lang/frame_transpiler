// TODO fix these issues and disable warning suppression
#![allow(unknown_lints)]
#![allow(clippy::branches_sharing_code)]
#![allow(clippy::single_match)]
#![allow(clippy::ptr_arg)]
#![allow(non_snake_case)]

use crate::config::*;
use crate::frame_c::ast::*;
use crate::frame_c::scanner::{Token, TokenType};
use crate::frame_c::symbol_table::*;
use crate::frame_c::visitors::java_8_visitor::ExprContext::Rvalue;
use crate::frame_c::visitors::*;
// use yaml_rust::{YamlLoader, Yaml};

#[derive(PartialEq)]
enum ExprContext {
    None,
    Lvalue,
    Rvalue,
}
pub struct Java8Visitor {
    compiler_version: String,
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
    serialize: Vec<String>,
    deserialize: Vec<String>,
    subclass_code: Vec<String>,
    warnings: Vec<String>,
    has_states: bool,
    errors: Vec<String>,
    visiting_call_chain_literal_variable: bool,
    // generate_exit_args: bool,
    generate_state_context: bool,
    generate_state_stack: bool,
    generate_change_state: bool,

    current_var_type: String,
    expr_context: ExprContext,
    // generate_transition_state: bool,

    /* Persistence */
    managed: bool, //Generate Managed code
    marshal: bool, //Generate JSON code
    manager: String,
    // keeping track of traversal context
    this_branch_transitioned: bool,
    //config
    config: JavaConfig,
}

impl Java8Visitor {
    //* --------------------------------------------------------------------- *//

    pub fn new(
        arcanium: Arcanum,
        generate_state_context: bool,
        generate_state_stack: bool,
        generate_change_state: bool,
        // generate_transition_state: bool,
        compiler_version: &str,
        comments: Vec<Token>,
        config: FrameConfig,
    ) -> Java8Visitor {
        let java_config = config.codegen.java;
        Java8Visitor {
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
            serialize: Vec::new(),
            deserialize: Vec::new(),
            has_states: false,
            errors: Vec::new(),
            subclass_code: Vec::new(),
            warnings: Vec::new(),
            visiting_call_chain_literal_variable: false,
            // generate_exit_args,
            generate_state_context,
            generate_state_stack,
            generate_change_state,
            // generate_transition_state,
            current_var_type: String::new(),
            expr_context: ExprContext::None,

            /* Persistence */
            managed: false, // Generate Managed code
            marshal: false, // Generate Json code
            manager: String::new(),

            this_branch_transitioned: false,

            config: java_config,
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
                    // s.push_str(&*self.config.code.frame_event_type_name.clone());
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
                Some(x) => x.get_type_str(),
                None => String::from("<?>"),
            },
            SymbolType::StateParam {
                state_param_symbol_rcref,
            } => match &state_param_symbol_rcref.borrow().param_type_opt {
                Some(x) => x.get_type_str(),
                None => String::from("<?>"),
            },
            SymbolType::StateVariable {
                state_variable_symbol_rcref,
            } => match &state_variable_symbol_rcref.borrow().var_type {
                Some(x) => x.get_type_str(),
                None => String::from("<?>"),
            },
            SymbolType::EventHandlerParam {
                event_handler_param_symbol_rcref,
            } => match &event_handler_param_symbol_rcref.borrow().param_type_opt {
                Some(x) => x.get_type_str(),
                None => String::from("<?>"),
            },
            SymbolType::EventHandlerVariable {
                event_handler_variable_symbol_rcref,
            } => match &event_handler_variable_symbol_rcref.borrow().var_type {
                Some(x) => x.get_type_str(),
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

    pub fn format_variable_expr(&mut self, variable_node: &VariableNode) -> String {
        let mut code = String::new();

        match variable_node.scope {
            IdentifierDeclScope::System => {
                code.push_str("this");
            }
            IdentifierDeclScope::DomainBlock => {
                code.push_str(&format!("this.{}", variable_node.id_node.name.lexeme));
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
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!("({}) this._compartment_.stateArgs", var_type));
                } else {
                    code.push_str(&format!("this._compartment_.stateArgs"));
                }
                // if is being used as an rval, cast it.
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!(".get(\"{}\")", variable_node.id_node.name.lexeme));
                } else {
                    code.push_str(&format!(".put(\"{}\",", variable_node.id_node.name.lexeme));
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
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!("({}) this._compartment_.stateVars", var_type));
                } else {
                    code.push_str(&format!("this._compartment_.stateVars"));
                }
                // if is being used as an rval, cast it.
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!(".get(\"{}\")", variable_node.id_node.name.lexeme));
                } else {
                    code.push_str(&format!(".put(\"{}\",", variable_node.id_node.name.lexeme));
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

                if self.visiting_call_chain_literal_variable {
                    // code.push_str("String.valueOf((");
                    code.push_str("(");
                }
                code.push_str(&format!("({}) e._parameters", var_type));
                // if is being used as an rval, cast it.
                if self.expr_context == ExprContext::Rvalue
                    || self.expr_context == ExprContext::None
                {
                    code.push_str(&format!(".get(\"{}\")", variable_node.id_node.name.lexeme));
                }
                if self.visiting_call_chain_literal_variable {
                    code.push_str(")");
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

    fn format_parameter_list(&mut self, params: &Vec<ParameterNode>) {
        let mut separator = "";
        for param in params {
            self.add_code(separator);
            let param_type: String = match &param.param_type_opt {
                Some(ret_type) => ret_type.get_type_str(),
                None => String::from("<?>"),
            };
            self.add_code(&format!("{} {}", param_type, param.param_name));
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
                Some(ret_type) => ret_type.get_type_str(),
                None => String::from("<?>"),
            };
            separator = ", ";
            self.add_code(&format!("{} {}", param_type, param.param_name));
            subclass_actions.push_str(&format!("{} {}", param_type, param.param_name));
        }
    }

    //* --------------------------------------------------------------------- *//

    // fn format_parameter_list_to_string(&mut self,params:&Vec<ParameterNode>,output:&mut String) {
    //     let mut separator = "";
    //     for param in params {
    //         output.push_str(&format!("{}", separator));
    //         let param_type: String = match &param.param_type_opt {
    //             Some(ret_type) => ret_type.get_type_str(),
    //             None => String::from("<?>"),
    //         };
    //         output.push_str(&format!("{} {}", param_type, param.param_name));
    //         separator = ",";
    //     }
    // }

    //* --------------------------------------------------------------------- *//

    fn format_new_params(&mut self, system_node: &SystemNode) -> (String, String) {
        let mut ref_params: String = String::new(); // ref params is passing reference of constructor params
        let mut separator = String::new();
        let mut new_params: String = match &system_node.start_state_state_params_opt {
            Some(param_list) => {
                let mut params = String::new();
                for param_node in param_list {
                    let param_type = match &param_node.param_type_opt {
                        Some(type_node) => self.format_type(type_node),
                        None => String::new(),
                    };
                    separator = String::new();
                    params.push_str(&format!(
                        "{}{} {}",
                        separator, &*param_type, param_node.param_name
                    ));
                    ref_params.push_str(&format!("{}{}", separator, param_node.param_name));
                    separator = String::from(", ");
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
                        separator, &*param_type, param_node.param_name
                    ));
                    ref_params.push_str(&format!("{}{}", separator, param_node.param_name));
                    separator = String::from(", ");
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
                        separator, &*param_type, param_node.param_name
                    ));
                    ref_params.push_str(&format!("{}{}", separator, param_node.param_name));
                    separator = String::from(", ");
                }
            }
            None => {}
        };

        (new_params.to_string(), ref_params.to_string())
    }

    //* --------------------------------------------------------------------- *//

    fn format_action_name(&mut self, action_name: &String) -> String {
        return format!("{}_do", action_name);
    }

    //* --------------------------------------------------------------------- *//

    pub fn run(&mut self, system_node: &SystemNode) {
        match &system_node.attributes_opt {
            Some(attributes) => {
                for value in (*attributes).values() {
                    match value {
                        AttributeNode::MetaNameValueStr { attr } => {
                            match attr.name.as_str() {
                                // TODO: constants
                                // "stateType" => self.config.code.state_type = attr.value.clone(),
                                "managed" => {
                                    self.manager = attr.value.clone();
                                    self.managed = true;
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
                                            //  "Managed" => self.managed = true,
                                            "Marshal" => self.marshal = true,
                                            _ => {}
                                        }
                                    }
                                }
                                "managed" => {
                                    self.managed = true;
                                    if attr.idents.len() != 1 {
                                        self.errors.push(
                                            "Attribute 'managed' takes 1 parameter".to_string(),
                                        );
                                    }
                                    match attr.idents.get(0) {
                                        Some(manager_type) => {
                                            self.manager = manager_type.clone();
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

                // self.config.code.marshal_system_state_var = format!("{}State", &system_node.name);
            }
            None => {}
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

    fn newline_to_string(&mut self, output: &mut String) {
        output.push_str(&*format!("\n{}", self.dent()));
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
                                ExprStmtType::EnumeratorStmtT { enumerator_stmt_node } => {
                                    enumerator_stmt_node.accept(self)
                                }
                                ExprStmtType::BinaryStmtT { binary_stmt_node } => {
                                    binary_stmt_node.accept(self)
                                }
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
                        StatementType::ContinueStmt {continue_stmt_node} => {
                            continue_stmt_node.accept(self);
                        }
                        StatementType::BreakStmt {break_stmt_node} => {
                            break_stmt_node.accept(self);
                        }
                        StatementType::SuperStringStmt {super_string_stmt_node} => {
                            super_string_stmt_node.accept(self);
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

    /*
        private void _transition_(CompartmentParamsCompartment compartment) {
            this._nextCompartment_=compartment;
        }

        private void _doTransition_(CompartmentParamsCompartment nextCompartment) {
            this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
            this._compartment_ = _nextCompartment_;
            this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
        }
    */

    //* --------------------------------------------------------------------- *//

    fn generate_machinery(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        self.add_code("//=============== Machinery and Mechanisms ==============//");
        self.newline();
        if system_node.get_first_state().is_some() {
            self.newline();
            self.add_code("private int _state_;");
            self.newline();
            self.newline();
            self.add_code(&format!(
                "private void _transition_({}Compartment compartment) {{",
                self.system_name
            ));
            self.indent();
            self.newline();
            self.add_code("this._nextCompartment_ = compartment;");
            self.outdent();
            self.newline();
            self.add_code("}");

            self.newline();
            self.newline();
            self.add_code(&format!(
                "private void _doTransition_({}Compartment nextCompartment) {{",
                self.system_name
            ));

            self.indent();
            self.newline();
            self.add_code("this._mux_(new FrameEvent(\"<\", this._compartment_.exitArgs));");
            self.newline();
            self.add_code("this._compartment_ = nextCompartment;");
            self.newline();
            self.add_code("this._mux_(new FrameEvent(\">\", this._compartment_.enterArgs));");
            self.outdent();
            self.newline();
            self.add_code("}");

            if self.generate_state_stack {
                self.newline();
                self.newline();
                //if self.generate_state_context {
                self.add_code(&format!(
                    "private Stack<{}Compartment> _stateStack_ = new Stack<>();",
                    self.system_name
                ));
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "private void _stateStack_push_({}Compartment compartment) {{",
                    self.system_name
                ));
                self.indent();
                self.newline();
                self.add_code("_stateStack_.push(compartment);");
                self.outdent();
                self.newline();
                self.add_code("}");
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "private {}Compartment _stateStack_pop_() {{",
                    self.system_name
                ));
                self.indent();
                self.newline();
                self.add_code("return _stateStack_.pop();");
                //}

                self.outdent();
                self.newline();
                self.add_code("}");
            }
            if self.generate_change_state {
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "private void _changeState_({}Compartment compartment) {{",
                    self.system_name
                ));
                self.indent();
                self.newline();
                self.add_code("this._compartment_ = compartment;");
                self.outdent();
                self.newline();
                self.add_code("}");
            }

            self.newline();
            if self.arcanium.is_serializable() {
                for line in self.serialize.iter() {
                    self.code.push_str(&*line.to_string());
                    self.code.push_str(&*format!("\n{}", self.dent()));
                }

                for line in self.deserialize.iter() {
                    self.code.push_str(&*line.to_string());
                    self.code.push_str(&*format!("\n{}", self.dent()));
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_subclass(&mut self) {
        for line in self.subclass_code.iter() {
            self.code.push_str(&*line.to_string());
            self.code.push_str(&*format!("\n{}", self.dent()));
        }
    }

    //* --------------------------------------------------------------------- *//

    /// Generate a return statement within a handler. Call this rather than adding a return
    /// statement directly to ensure that the control-flow state is properly maintained.
    fn generate_return(&mut self) {
        self.newline();
        self.add_code("return;");
        self.this_branch_transitioned = false;
    }

    /// Generate a return statement if the current branch contained a transition or change-state.
    fn generate_return_if_transitioned(&mut self) {
        if self.this_branch_transitioned {
            self.generate_return();
        }
    }
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

        let state_ref_code = self.generate_state_ref_code(target_state_name);

        match &change_state_stmt_node.label_opt {
            Some(label) => {
                self.add_code(&format!("// {}", label));
                self.newline();
            }
            None => {}
        }
        self.newline();
        self.add_code(&format!(
            "{}Compartment compartment =  new {}Compartment({});",
            self.system_name, self.system_name, state_ref_code
        ));
        self.newline();
        // }

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
                            self.errors.push(
                                "Fatal error: misaligned parameters to arguments.".to_string(),
                            );
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
                                        "compartment.enterArgs.put(\"{}\",{});",
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
        let target_state_args_opt = match &change_state_stmt_node.state_context_t {
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
                                    let _param_type = match &param_symbol.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => String::from("<?>"),
                                    };
                                    let mut expr = String::new();
                                    expr_t.accept_to_string(self, &mut expr);
                                    self.add_code(&format!(
                                        "compartment.stateArgs.get(\"{}\", {});",
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
                            let _var_type = match &var.type_opt {
                                Some(var_type) => var_type.get_type_str(),
                                None => String::from("<?>"),
                            };
                            let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
                            let mut expr_code = String::new();
                            expr_t.accept_to_string(self, &mut expr_code);
                            self.add_code(&format!(
                                "compartment.stateVars.put(\"{}\", {});",
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
        self.add_code("this._changeState_(compartment);");
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_stack_pop_change_state(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) {
        self.newline();
        match &change_state_stmt_node.label_opt {
            Some(label) => {
                self.add_code(&format!("# {}", label));
                self.newline();
            }
            None => {}
        }
        self.add_code(&format!(
            "{}Compartment compartment = this._stateStack_pop_();",
            self.system_name
        ));
        //self.add_code("compartment = self.__state_stack_pop()");
        self.newline();
        self.add_code("this._changeState_(compartment);");
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_code(&self, target_state_name: &str) -> String {
        format!(
            "{}State.{}.getValue()",
            self.system_name,
            target_state_name.to_uppercase()
        )
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_transition(&mut self, transition_statement: &TransitionStatementNode) {
        self.newline();
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
                self.add_code(&format!("// {}", label));
                self.newline();
            }
            None => {}
        }

        // -- Exit Arguments --

        // let mut has_exit_args = false;
        if let Some(exit_args) = &transition_statement.exit_args_opt {
            if !exit_args.exprs_t.is_empty() {
                // has_exit_args = true;

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
                            // self.add_code("let exitArgs = {};");
                            // self.newline();
                            // Loop through the ARGUMENTS...
                            for expr_t in &exit_args.exprs_t {
                                // ...and validate w/ the PARAMETERS
                                match param_symbols_it.next() {
                                    Some(p) => {
                                        let _param_type = match &p.param_type_opt {
                                            Some(param_type) => param_type.get_type_str(),
                                            None => String::from("<?>"),
                                        };
                                        let mut expr = String::new();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.add_code(&format!(
                                            "this._compartment_.exitArgs.put(\"{}\", {});",
                                            p.name, expr
                                        ));
                                        self.newline();
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
                    self.errors.push("Unknown error.".to_string());
                }
            }
        }

        // -- Enter Arguments --

        let state_ref_code = self.generate_state_ref_code(target_state_name);

        self.add_code(&format!(
            "{}Compartment compartment =  new {}Compartment({});",
            self.system_name, self.system_name, state_ref_code
        ));
        self.newline();
        // }

        if transition_statement.forward_event {
            self.newline();
            self.add_code("compartment._forwardEvent = e;");
            self.newline();
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
                                    let _param_type = match &p.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => String::from("<?>"),
                                    };
                                    let mut expr = String::new();
                                    expr_t.accept_to_string(self, &mut expr);
                                    self.add_code(&format!(
                                        "compartment.enterArgs.put(\"{}\", {});",
                                        p.name, expr
                                    ));
                                    self.newline();
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
                                    let _param_type = match &param_symbol.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => String::from("<?>"),
                                    };
                                    let mut expr = String::new();
                                    expr_t.accept_to_string(self, &mut expr);
                                    self.add_code(&format!(
                                        "compartment.stateArgs.put(\"{}\", {});",
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
                            let _var_type = match &var.type_opt {
                                Some(var_type) => var_type.get_type_str(),
                                None => String::from("<?>"),
                            };
                            let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
                            let mut expr_code = String::new();
                            expr_t.accept_to_string(self, &mut expr_code);
                            self.add_code(&format!(
                                "compartment.stateVars.put(\"{}\", {});",
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
        self.add_code("this._transition_(compartment);");
    }

    //* --------------------------------------------------------------------- *//

    fn format_target_state_name(&self, state_name: &str) -> String {
        format!("_s{}_", state_name)
    }

    //* --------------------------------------------------------------------- *//

    // fn format_target_state_reference(&self, state_name: &str) -> String {
    //     format!("this::{}", self.format_target_state_name(state_name))
    // }

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
                            self.add_code(
                                "HashMap<String,Object> exitArgs = new HashMap<String,Object>();",
                            );
                            self.newline();
                            // Loop through the ARGUMENTS...
                            for expr_t in &exit_args.exprs_t {
                                // ...and validate w/ the PARAMETERS
                                match param_symbols_it.next() {
                                    Some(p) => {
                                        let mut expr = String::new();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.add_code(&format!(
                                            "exitArgs.put(\"{}\"],{});",
                                            p.name, expr
                                        ));
                                        self.newline();
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

        self.add_code(&format!(
            "{}Compartment compartment = this._stateStack_pop_();",
            self.system_name
        ));
        self.newline();
        self.add_code("this._transition_(compartment);");
        // if self.generate_state_context {
        //     self.add_code("stateContext = _stateStack_pop_();");
        // } else {
        //     self.add_code("FrameState state = _stateStack_pop_();");
        // }
        // self.newline();
        // if self.generate_exit_args {
        //     if self.generate_state_context {
        //         self.add_code("_transition_(stateContext.state,exitArgs,stateContext);");
        //     } else {
        //         self.add_code("_transition_(state,exitArgs);");
        //     }
        // } else if self.generate_state_context {
        //     self.add_code("_transition_(stateContext.state,stateContext);");
        // } else {
        //     self.add_code("_transition_(state);");
        // }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_compartment(&mut self, system_name: &str) {
        self.newline();
        self.add_code("//=============== Compartment ==============//");
        self.newline();
        self.newline();
        self.add_code(&format!("class {}Compartment {{", system_name));
        self.newline();
        self.indent();
        self.newline();
        self.add_code("public int getState() {");
        self.indent();
        self.newline();
        self.add_code("return state;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("public void setState(int state) {");
        self.indent();
        self.newline();
        self.add_code("this.state = state;");
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("public HashMap<String, Object> getStateArgs() {");
        self.indent();
        self.newline();
        self.add_code("return stateArgs;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("public void setStateArgs(HashMap<String, Object> stateArgs) {");
        self.indent();
        self.newline();
        self.add_code("this.stateArgs = stateArgs;");
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("public HashMap<String, Object> getStateVars() {");
        self.indent();
        self.newline();
        self.add_code("return stateVars;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("public void setStateVars(HashMap<String, Object> stateVars) {");
        self.indent();
        self.newline();
        self.add_code("this.stateVars = stateVars;");
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("public HashMap<String, Object> getEnterArgs() {");
        self.indent();
        self.newline();
        self.add_code("return enterArgs;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("public void setEnterArgs(HashMap<String, Object> enterArgs) {");
        self.indent();
        self.newline();
        self.add_code("this.enterArgs = enterArgs;");
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("public HashMap<String, Object> getExitArgs() {");
        self.indent();
        self.newline();
        self.add_code("return exitArgs;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("public void setExitArgs(HashMap<String, Object> exitArgs) {");
        self.indent();
        self.newline();
        self.add_code("this.exitArgs = exitArgs;");
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("public FrameEvent get_forwardEvent() {");
        self.indent();
        self.newline();
        self.add_code("return _forwardEvent;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("public void set_forwardEvent(FrameEvent _forwardEvent) {");
        self.indent();
        self.newline();
        self.add_code("this._forwardEvent = _forwardEvent;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();

        self.add_code("int state;");
        self.newline();
        self.add_code(&format!("{}Compartment(){{", system_name));
        self.newline();
        self.newline();
        self.add_code("}");
        self.newline();
        self.add_code(&format!("{}Compartment(int state) {{", system_name));
        // self.newline();
        self.indent();
        self.newline();
        self.add_code("this.state = state;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("HashMap<String, Object> stateArgs = new HashMap<String, Object>();");
        self.newline();
        self.add_code("HashMap<String, Object> stateVars = new HashMap<String, Object>();");
        self.newline();
        self.add_code("HashMap<String, Object> enterArgs = new HashMap<String, Object>();");
        self.newline();
        self.add_code("HashMap<String, Object> exitArgs = new HashMap<String, Object>();");
        self.newline();
        self.add_code("FrameEvent _forwardEvent = new FrameEvent();");
        // self.outdent();
        // self.newline();
        // self.add_code("}");
        self.newline();
        self.newline();
        self.add_code(&format!("public {}Compartment(int state, HashMap<String, Object> stateArgs, HashMap<String, Object> stateVars,", system_name));
        self.indent();
        self.indent();
        self.newline();
        self.add_code("HashMap<String, Object> enterArgs, HashMap<String, Object> exitArgs, FrameEvent _forwardEvent) {");
        self.outdent();
        self.newline();
        self.add_code("this.state = state;");
        self.newline();
        self.add_code("this.stateArgs = stateArgs;");
        self.newline();
        self.add_code("this.stateVars = stateVars;");
        self.newline();
        self.add_code("this.enterArgs = enterArgs;");
        self.newline();
        self.add_code("this.exitArgs = exitArgs;");
        self.newline();
        self.add_code("this._forwardEvent = _forwardEvent;");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn generate_new_fn(&mut self, system_node: &SystemNode) {
        self.indent();
        if system_node.get_first_state().is_some() {
            self.newline();
            self.newline();

            if self.generate_state_stack {
                self.add_code("// Create state stack.");
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "this._stateStack_ = new Stack<{}Compartment>();",
                    self.system_name
                ));
                self.newline();
                self.newline();
            }

            self.add_code("// Create and intialize start state compartment.");
            // self.newline();
            // self.add_code(&format!(
            //     "this.__state = self.{}",
            //     self.format_target_state_name(&self.first_state_name)
            // ));
        } else {
            self.add_code("// Create and intialize start state compartment.");
            // self.newline();
            // self.newline();
            // self.add_code("self.__state = None");
        }

        if self.managed {
            self.newline();
            self.add_code("this._manager = manager;");
        }

        self.newline();
        self.add_code(&format!(
            "this._compartment_ = new {}Compartment(this._state_);",
            system_node.name
        ));
        self.newline();
        self.add_code("this._nextCompartment_ = null;");

        // Initialize state arguments.
        match &system_node.start_state_state_params_opt {
            Some(params) => {
                for param in params {
                    self.newline();
                    self.add_code(&format!(
                        "this._compartment_.stateArgs.put(\"{}\", {});",
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
                                "this._compartment_.stateVars.put(\"{}\", {});",
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
                    "this._compartment_.enterArgs.put(\"{}\", {});",
                    param.param_name, param.param_name,
                ));
            }
        }

        //  if self.config.code.public_state_info {
        //      self.newline();
        //      self.add_code("this.state = this.#compartment.state.name");
        //  }

        self.newline();
        self.newline();
        // self.add_code("# Initialize domain");

        // if let Some(domain_block_node) = &system_node.domain_block_node_opt {
        //     domain_block_node.accept(self);
        // } else {
        //     self.newline();
        // }
        self.newline();
        self.add_code("// Send system start event");

        /* FrameEvent frameEvent = new FrameEvent(">", this._compartment_.enterArgs); */

        if let Some(_enter_params) = &system_node.start_state_enter_params_opt {
            self.newline();
            self.add_code(
                "FrameEvent frame_event = new FrameEvent(\">\", this._compartment_.enterArgs);",
            );
        } else {
            self.newline();
            self.add_code("FrameEvent frame_event = new FrameEvent(\">\", null);");
        }

        self.newline();
        self.add_code("this._mux_(frame_event);");

        self.outdent();
        self.newline();
    }

    //* --------------------------------------------------------------------- *//
    fn format_load_fn(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        let mut manager_param = "";
        if self.managed {
            manager_param = "manager";
        }
        self.add_code(&format!(
            "public static {} load{}({}Controller {}, {}Controller data){{",
            system_node.name, system_node.name, self.manager, manager_param, system_node.name,
        ));

        self.indent();
        self.newline();
        self.newline();
        if self.managed {
            self.add_code("data._manager = manager;");
        }
        // self.newline();
        // self.newline();
        // self.add_code("// Initialize machine");
        // self.newline();
        // // self.add_code("data.#compartment = .#compartment");
        // self.newline();
        // // for x in domain_vec {
        // //     self.newline();
        // //     self.add_code(&format!("this.{} = parseObj.{}", x.0, x.0))
        // // }
        // self.newline();
        self.newline();
        self.add_code("return data;");
        self.newline();

        self.outdent();
        self.newline();
        self.add_code("}");
    }
    //* --------------------------------------------------------------------- *//
    fn generate_json_fn(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        self.add_code(&format!("public {} marshal(){{", system_node.name));
        self.indent();
        self.newline();
        self.newline();
        self.add_code(&format!("{} data = this;", system_node.name));
        self.newline();
        self.add_code("return data;");
        self.newline();

        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
    }
    fn generate_state_info_method(&mut self) {
        self.newline();
        self.add_code("public String state_info(){");
        self.indent();
        self.newline();
        self.add_code("return String.valueOf(this._compartment_.state);");
        self.newline();
        self.add_code("}");
        self.newline();
        self.outdent();
    }
}

//* --------------------------------------------------------------------- *//

impl AstVisitor for Java8Visitor {
    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) {
        self.system_name = system_node.name.clone();
        self.add_code(&format!("// {}", self.compiler_version));
        self.newline();
        self.add_code(
            "// get include files at https://github.com/frame-lang/frame-ancillary-files",
        );
        self.newline();
        self.add_code(&system_node.header);
        self.newline();
        self.newline();
        self.add_code(&format!("class {} {{", system_node.name));
        self.indent();
        self.newline();

        /* private CompartmentParamsCompartment _compartment_;
        private CompartmentParamsCompartment _nextCompartment_; */
        self.newline();
        self.add_code(&format!(
            "private {}Compartment _compartment_;",
            system_node.name
        ));
        self.newline();
        self.add_code(&format!(
            "private {}Compartment _nextCompartment_;",
            system_node.name
        ));
        self.newline();
        self.newline();
        if self.managed {
            self.add_code(&format!("protected {} _manager;", self.manager));
        } // else {
          //     self.add_code(&format!("protected {} _manager;", self.system_name));
          // }
        self.newline();
        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        match system_node.get_first_state() {
            Some(x) => {
                self.first_state_name = x.borrow().name.clone();
                self.has_states = true;
            }
            None => {}
        }

        self.newline();
        self.add_code(&format!("{}(", system_node.name));

        // format system params,if any.
        let new_params: String = String::from(&self.format_new_params(system_node).0);

        if self.managed {
            if new_params.is_empty() {
                self.add_code(&format!("{} manager) {{", self.manager));
            } else {
                self.add_code(&format!("manager,{}) {{", new_params));
            }
        } else {
            self.add_code(&format!("{}) {{", new_params));
        }

        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        match system_node.get_first_state() {
            Some(x) => {
                self.first_state_name = x.borrow().name.clone();
                self.has_states = true;
            }
            None => {}
        }

        // generate constructor

        // if self.has_states {
        //     self.add_code(&format!("{}() {{", system_node.name));
        //     self.indent();
        //     self.newline();
        //     self.newline();
        //     self.add_code(&format!("_state_ = this::_s{}_;", self.first_state_name));
        //     if self.generate_state_context {
        //         self.newline();
        //         self.add_code(&format!(
        //             "_stateContext_ = new StateContext({});",
        //             self.format_target_state_reference(&self.first_state_name)
        //         ));
        //         if let Some(state_symbol_rcref) = self.arcanium.get_state(&self.first_state_name) {
        //             //   self.newline();
        //             let state_symbol = state_symbol_rcref.borrow();
        //             let state_node = &state_symbol.state_node.as_ref().unwrap().borrow();
        //             // generate local state variables
        //             if state_node.vars_opt.is_some() {
        //                 for var_rcref in state_node.vars_opt.as_ref().unwrap() {
        //                     let var = var_rcref.borrow();
        //                     let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
        //                     let mut expr_code = String::new();
        //                     expr_t.accept_to_string(self, &mut expr_code);
        //                     self.newline();
        //                     self.add_code(&format!(
        //                         "_stateContext_.addStateVar(\"{}\",{});",
        //                         var.name, expr_code
        //                     ));
        //                 }
        //             }
        //         }
        //     }

        //     self.outdent();
        //     self.newline();
        //     self.add_code("}");
        //     self.newline();
        // }

        // end of generate constructor

        self.generate_new_fn(system_node);
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();

        // Define state constants
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            self.add_code("// states enum");
            self.newline();
            self.add_code(&format!("private enum {}State {{", system_node.name));
            self.indent();
            self.newline();
            let len = machine_block_node.states.len();
            let mut current = 0;
            self.newline();
            for state_node_rcref in &machine_block_node.states {
                self.add_code(&format!(
                    "{}({})",
                    &state_node_rcref.borrow().name.to_uppercase(),
                    current
                ));
                current += 1;

                if current == len {
                    self.add_code(";");
                    self.newline();
                    self.newline();
                } else {
                    self.add_code(",");
                    self.newline();
                }
            }

            self.add_code("public final int value;");
            self.newline();
            self.newline();
            self.add_code(&format!("private {}State(int value) {{", system_node.name));
            self.indent();
            self.newline();
            self.add_code("this.value=value;");
            self.outdent();
            self.newline();
            self.add_code("}");
            self.newline();
            self.newline();
            self.add_code("public int getValue() {");
            self.indent();
            self.newline();
            self.add_code("return value;");
            self.outdent();
            self.newline();
            self.add_code("}");

            self.outdent();
            self.newline();
            self.add_code("}");
            self.newline();
        }

        if self.marshal {
            //generate Load() factory
            self.format_load_fn(system_node);
            self.generate_json_fn(system_node);
        }

        // generate _mux_

        self.newline();
        self.add_code("//====================== Multiplexer ====================//");
        self.newline();
        self.newline();

        self.add_code("private void _mux_(FrameEvent e) {");
        self.indent();

        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            self.newline();

            for (current_index, state_node_rcref) in machine_block_node.states.iter().enumerate() {
                let state_name = &self
                    .format_target_state_name(&state_node_rcref.borrow().name)
                    .to_string();
                let state_ref_code = self.generate_state_ref_code(&state_node_rcref.borrow().name);
                if current_index == 0 {
                    self.add_code(&format!(
                        "if(this._compartment_.state == {}) {{",
                        state_ref_code
                    ));
                } else {
                    self.add_code(&format!(
                        "else if(this._compartment_.state == {}) {{",
                        state_ref_code
                    ));
                }
                self.indent();
                self.newline();
                self.add_code(&format!("this.{}(e);", state_name));
                self.outdent();
                self.newline();
                self.add_code("}");
            }

            self.newline();
            self.newline();
            self.newline();
            self.add_code("if(this._nextCompartment_ != null) {");
            self.indent();
            self.newline();
            self.add_code(&format!(
                "{}Compartment nextCompartment = this._nextCompartment_;",
                system_node.name
            ));
            self.newline();
            self.add_code("this._nextCompartment_ = null;");
            self.newline();
            self.add_code("if(nextCompartment._forwardEvent != null && ");
            self.newline();
            self.add_code(" nextCompartment._forwardEvent._message == \">\") {");
            self.indent();
            self.newline();
            self.add_code("this._mux_(new FrameEvent( \"<\", this._compartment_.exitArgs));");
            self.newline();
            self.add_code("this._compartment_ = nextCompartment;");
            self.newline();
            self.add_code("this._mux_(nextCompartment._forwardEvent);");
            self.outdent();
            self.newline();
            self.add_code("} else {");
            self.indent();
            self.newline();
            self.add_code("this._doTransition_(nextCompartment);");
            self.newline();
            self.add_code("if(nextCompartment._forwardEvent != null) {");
            self.indent();
            self.newline();
            self.add_code("this._mux_(nextCompartment._forwardEvent);");
            self.outdent();
            self.newline();
            self.add_code("}");
            self.outdent();
            self.newline();
            self.add_code("}");
            self.newline();
            self.add_code("nextCompartment._forwardEvent = null;");
            self.outdent();
            self.newline();
            self.add_code("}");
            self.outdent();
            self.newline();
            self.add_code("}");
            self.newline();
        }

        self.serialize.push("".to_string());
        self.serialize.push("Bag _serialize__do() {".to_string());

        self.deserialize.push("".to_string());

        // @TODO: _do needs to be configurable.
        //     self.deserialize
        //         .push("void _deserialize__do(Bag data) {".to_string());

        //     self.subclass_code.push("".to_string());
        //     self.subclass_code
        //         .push("/********************\n".to_string());
        //     self.subclass_code.push(format!(
        //         "class {}Controller extends {} {{",
        //         system_node.name, system_node.name
        //     ));

        //     // generate constructor for controller
        //     // ref params is passing reference of constructor params
        //     let (new_params, ref_params): (String, String) = self.format_new_params(system_node);
        //     if self.managed {
        //     if new_params.is_empty() {
        //         self.subclass_code
        //             .push(format!("\t{}Controller({}Mom manager) {{", self.system_name, self.system_name).to_string());
        //         self.subclass_code.push("\t  super(manager);".to_string());
        //     } else {
        //         self.subclass_code.push(format!(
        //             "\t{}Controller({}) {{",
        //             self.system_name, new_params
        //         ));
        //         self.subclass_code
        //             .push(format!("\t  super({});", ref_params));
        //     }
        // }
        //     self.subclass_code.push("\t}".to_string());
        //     if let Some(interface_block_node) = &system_node.interface_block_node_opt {
        //         interface_block_node.accept(self);
        //     }
        // @TODO: _do needs to be configurable.
        self.deserialize
            .push("that._deserialize__do = function(data) {".to_string());

        self.subclass_code.push("".to_string());
        self.subclass_code
            .push("/********************\n".to_string());
        self.subclass_code.push(format!(
            "class {}Controller extends {} {{\n",
            system_node.name, system_node.name
        ));

        if self.managed {
            if new_params.is_empty() {
                self.subclass_code.push(format!(
                    "\t{}Controller({} manager) {{",
                    system_node.name, self.manager
                ));
                self.subclass_code.push("\t  super(manager);".to_string());
            } else {
                self.subclass_code.push(format!(
                    "\t{}Controller({} manager,{}) {{",
                    system_node.name, self.manager, new_params
                ));
                self.subclass_code.push(format!(
                    "\t  super({} manager{});",
                    self.manager, new_params
                ));
            }
        } else {
            self.subclass_code.push(format!(
                "\t{}Controller({}) {{",
                system_node.name, new_params
            ));
            self.subclass_code
                .push(format!("\t  super({});", new_params));
        }

        self.subclass_code.push("\t}".to_string());
        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            interface_block_node.accept(self);
        }
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }

        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            actions_block_node.accept(self);
        }

        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            domain_block_node.accept(self);
        }

        self.subclass_code.push("}".to_string());
        self.subclass_code
            .push("\n********************/".to_string());

        self.serialize.push("".to_string());
        self.serialize
            .push("\treturn JSON.stringify(bag);".to_string());
        self.serialize.push("}".to_string());
        self.serialize.push("".to_string());

        self.deserialize.push("".to_string());
        self.deserialize.push("}".to_string());

        if self.has_states {
            self.generate_machinery(system_node);
        }
        if self.config.code.public_state_info {
            self.generate_state_info_method()
        }

        // TODO: add comments back
        // self.newline();
        // self.generate_comment(system_node.line);
        // self.newline();
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();

        self.generate_compartment(&system_node.name);

        self.generate_subclass();
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
        self.add_code(
            &interface_method_call_expr_node
                .identifier
                .name
                .lexeme
                .to_string(),
        );
        interface_method_call_expr_node.call_expr_list.accept(self);

        // TODO: review this return as I think it is a nop.
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node_to_string(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
        output: &mut String,
    ) {
        output.push_str(
            &interface_method_call_expr_node
                .identifier
                .name
                .lexeme
                .to_string(),
        );
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
            Some(ret) => ret.get_type_str(),
            None => "void".to_string(),
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
            "public {} {}(",
            return_type, interface_method_node.name
        ));

        match &interface_method_node.params {
            Some(params) => {
                self.format_parameter_list(params);
            }
            None => {}
        }

        self.add_code(") {");
        self.indent();
        let params_param_code;
        if interface_method_node.params.is_some() {
            params_param_code = String::from("parameters");
            self.newline();
            self.add_code("HashMap<String,Object> parameters = new HashMap<String,Object>();");
            match &interface_method_node.params {
                Some(params) => {
                    for param in params {
                        let pname = &param.param_name;
                        self.newline();
                        self.add_code(&format!("parameters.put(\"{}\", {});\n", pname, pname));
                    }
                }
                None => {}
            }
        } else {
            params_param_code = String::from("null");
        }

        self.newline();
        self.add_code(&format!(
            "FrameEvent e = new FrameEvent(\"{}\", {});",
            method_name_or_alias, params_param_code
        ));
        self.newline();
        self.add_code("this._mux_(e);");

        match &interface_method_node.return_type_opt {
            Some(return_type) => {
                self.newline();
                self.add_code(&format!(
                    "return ({}) e._return;",
                    return_type.get_type_str()
                ));
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

        self.serialize.push("".to_string());
        self.serialize.push("\tvar stateName = null;".to_string());

        self.deserialize.push("".to_string());
        self.deserialize
            .push("\tconst bag = JSON.parse(data);".to_string());
        self.deserialize.push("".to_string());
        self.deserialize.push("\tswitch (bag.state) {".to_string());

        for state_node_rcref in &machine_block_node.states {
            state_node_rcref.borrow().accept(self);
        }

        self.serialize.push("".to_string());
        self.serialize.push("\tvar bag = {".to_string());
        self.serialize.push("\t\tstate : stateName,".to_string());
        self.serialize.push("\t\tdomain : {}".to_string());
        self.serialize.push("\t};".to_string());
        self.serialize.push("".to_string());

        self.deserialize.push("\t}".to_string());
        self.deserialize.push("".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_block_node(&mut self, actions_block_node: &ActionsBlockNode) {
        self.newline();
        self.newline();
        self.add_code("//===================== Actions Block ===================//");

        for action_rcref in &actions_block_node.actions {
            let action_node = action_rcref.borrow();
            if action_node.code_opt.is_some() {
                action_node.accept_action_impl(self);
            }
        }

        self.newline();
        self.newline();
        self.add_code("// Unimplemented Actions");
        self.newline();

        for action_rcref in &actions_block_node.actions {
            let action_node = action_rcref.borrow();
            if action_node.code_opt.is_none() {
                action_node.accept_action_decl(self);
            }
        }
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
        self.add_code("//===================== Domain Block ===================//");
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
        self.add_code(&format!(
            "private void _s{}_(FrameEvent e) {{",
            state_node.name
        ));
        self.indent();

        self.serialize.push(format!(
            "\tif (_state_ == _s{}_) stateName = \"{}\"",
            state_node.name, state_node.name
        ));

        self.deserialize.push(format!(
            "\t\tcase \"{}\": _state_ = _s{}_; break;",
            state_node.name, state_node.name
        ));

        if let Some(calls) = &state_node.calls_opt {
            for call in calls {
                self.newline();
                call.accept(self);
                self.add_code(";");
            }
        }

        self.first_event_handler = true; // context for formatting

        if !state_node.evt_handlers_rcref.is_empty() {
            for evt_handler_node in &state_node.evt_handlers_rcref {
                evt_handler_node.as_ref().borrow().accept(self);
            }
        }

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
            if self.first_event_handler {
                self.add_code(&format!("if(e._message == \"{}\") {{", message_node.name));
            } else {
                self.add_code(&format!(
                    "else if(e._message == \"{}\") {{",
                    message_node.name
                ));
            }
        } else {
            // AnyMessage ( ||* )
            if self.first_event_handler {
                // This logic is for when there is only the catch all event handler ||*
                self.add_code("if(true) {");
            } else {
                // other event handlers preceded ||*
                self.add_code("else {");
            }
        }
        self.indent();
        self.generate_comment(evt_handler_node.line);

        //  if let MessageType::CustomMessage {message_node} =
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
        self.newline();
        self.add_code("}");

        // this controls formatting here
        self.first_event_handler = false;
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
                    self.add_code("e._return = ");
                    expr_t.accept(self);
                    self.add_code(";");
                    self.newline();
                    self.generate_return();
                    self.newline();
                }
                None => self.generate_return(),
            },
            TerminatorType::Continue => {
                self.generate_return_if_transitioned();
                // self.add_code("break;")
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_statement_node(&mut self, method_call_statement: &CallStmtNode) {
        self.newline();
        method_call_statement.call_expr_node.accept(self);
        self.add_code(";");
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
        self.add_code(&action_name);

        action_call.call_expr_list.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(
        &mut self,
        action_call: &ActionCallExprNode,
        output: &mut String,
    ) {
        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        output.push_str(&action_name);

        action_call.call_expr_list.accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(&mut self, action_call_stmt_node: &ActionCallStmtNode) {
        self.newline();
        action_call_stmt_node.action_call_expr_node.accept(self);
        self.add_code(";");
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
            StateContextType::StateStackPop { .. } => {
                self.generate_state_stack_pop_change_state(change_state_stmt_node)
            }
        };
    }

    //* --------------------------------------------------------------------- *//

    // TODO: ??
    fn visit_parameter_node(&mut self, _: &ParameterNode) {
        // self.add_code(&format!("{}",parameter_node.name));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_dispatch_node(&mut self, dispatch_node: &DispatchNode) {
        self.newline();
        self.add_code(&format!("_s{}_(e);", dispatch_node.target_state_ref.name));
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
            TestType::EnumMatchTest {
                enum_match_test_node,
            } => {
                enum_match_test_node.accept(self);
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_node(&mut self, bool_test_node: &BoolTestNode) {
        let mut if_or_else_if = "if ";

        self.newline();
        for branch_node in &bool_test_node.conditional_branch_nodes {
            if branch_node.is_negated {
                self.add_code(&format!("{}(!(", if_or_else_if));
            } else {
                self.add_code(&format!("{}(", if_or_else_if));
            }

            branch_node.expr_t.accept(self);

            if branch_node.is_negated {
                self.add_code(")");
            }
            self.add_code(") {");
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
                self.this_branch_transitioned = true;
                interface_method_call_expr_node.accept(self);
                self.add_code(";");
                self.generate_return();
                return;
            }
        }

        // standard case
        method_call_chain_literal_stmt_node
            .call_chain_literal_expr_node
            .accept(self);
        self.add_code(";");
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
                    match &method_call_chain_expression_node.is_new_expr {
                        true => self.add_code("new "),
                        false => {}
                    }
                    call.accept(self);
                }
                CallChainLiteralNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    interface_method_call_expr_node.accept(self);
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
                CallChainLiteralNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    interface_method_call_expr_node.accept_to_string(self, output);
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
                            self.add_code("e._return = ");
                            expr_t.accept(self);
                            self.add_code(";");
                            self.newline();
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                        self.add_code("break;");
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
                            self.add_code("e._return = ");
                            expr_t.accept(self);
                            self.add_code(";");
                            self.newline();
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                        self.add_code("break;");
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
            self.add_code(&format!("{} ((", if_or_else_if));
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
                    self.add_code(&format!(" == \"{}\")", match_string));
                    first_match = false;
                } else {
                    self.add_code(" || (");
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
                    self.add_code(&format!(" == \"{}\")", match_string));
                }
            }
            self.add_code(") {");
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
                            self.add_code("e._return = ");
                            expr_t.accept(self);
                            self.add_code(";");
                            self.newline();
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                        self.add_code("break;");
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
                            self.add_code("e._return = ");
                            expr_t.accept(self);
                            self.add_code(";");
                            self.newline();
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                        self.add_code("break;");
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
            self.add_code(&format!("{} ((", if_or_else_if));
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
                    self.add_code(&format!(" == {})", match_number.match_pattern_number));
                    first_match = false;
                } else {
                    self.add_code(" || (");
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
                    self.add_code(&format!(" == {})", match_number.match_pattern_number));
                }
            }

            self.add_code(") {");
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
                            self.add_code("e._return = ");
                            expr_t.accept(self);
                            self.add_code(";");
                            self.newline();
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                        self.add_code("break;");
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
                            self.add_code("e._return = ");
                            expr_t.accept(self);
                            self.add_code(";");
                            self.newline();
                            self.generate_return();
                        }
                        None => self.generate_return(),
                    },
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                        self.add_code("break;");
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
            TokenType::Null => self.add_code("null"),
            TokenType::Nil => self.add_code("null"),
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
                output.push_str("null");
            }
            TokenType::Null => {
                output.push_str("null");
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
                // if self.generate_state_context {
                self.add_code("_stateStack_push_(this._compartment_);");
                // } else {
                //     self.add_code("_stateStack_push_(this._compartment_");
                // }
            }
            StateStackOperationType::Pop => {
                // if self.generate_state_context {
                self.add_code(&format!(
                    "{}Compartment compartment = this._stateStack_pop_();",
                    self.system_name
                ));
                // } else {
                //     self.add_code("FrameState state = _stateStack_pop_()");
                // }
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
            } => self.add_code("e._message"),
            FrameEventPart::Param {
                param_symbol_rcref,
                is_reference: _is_reference,
            } => {
                self.add_code(&format!(
                    "e._parameters.get(\"{}\")",
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
                self.add_code("e._return");
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
            } => output.push_str("e._message"),
            FrameEventPart::Param {
                param_symbol_rcref,
                is_reference: _is_reference,
            } => {
                output.push_str(&format!(
                    "e._parameters.get(\"{}\")",
                    param_symbol_rcref.borrow().name
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
                output.push_str("e._return");
                if self.expr_context == Rvalue || self.expr_context == ExprContext::None {
                    output.push_str(&format!(".({})", &self.current_event_ret_type));
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_node(&mut self, action_node: &ActionNode) {
        let mut subclass_code = String::new();

        self.newline();
        self.newline_to_string(&mut subclass_code);

        let action_ret_type: String = match &action_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from("void"),
        };

        let action_name = self.format_action_name(&action_node.name);
        self.add_code(&format!("protected {} {}(", action_ret_type, action_name));
        subclass_code.push_str(&format!("protected {} {}(", action_ret_type, action_name));
        match &action_node.params {
            Some(params) => {
                self.format_actions_parameter_list(params, &mut subclass_code);
            }
            None => {}
        }
        subclass_code.push_str(") {}");
        self.subclass_code.push(subclass_code);

        self.add_code(") { throw new UnsupportedOperationException(); }");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
        self.visit_variable_decl_node(variable_decl_node);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
        let var_type = match &variable_decl_node.type_opt {
            Some(x) => x.get_type_str(),
            None => String::from("<?>"),
        };
        let var_name = &variable_decl_node.name;
        let var_init_expr = &variable_decl_node.initializer_expr_t_opt.as_ref().unwrap();
        self.newline();
        let mut code = String::new();
        self.current_var_type = var_type.clone(); // used for casting
        self.expr_context = ExprContext::Rvalue;
        var_init_expr.accept_to_string(self, &mut code);
        self.expr_context = ExprContext::None;

        match &variable_decl_node.identifier_decl_scope {
            // IdentifierDeclScope::DomainBlock => {
            //     self.add_code(&format!("this.{} ", var_name));
            //     if !var_type.is_empty() {
            //         self.add_code(&format!(": {}", var_type));
            //     }
            //     self.add_code(&format!(" = {}", code));
            // }
            IdentifierDeclScope::DomainBlock => {
                if !var_type.is_empty() {
                    self.add_code(&format!("public {}", var_type));
                }
                self.add_code(&format!(" {} ", var_name));

                if !code.is_empty() {
                    self.add_code(&format!(" = {};", code));
                } else {
                    self.add_code(&format!(";"));
                }
            }
            IdentifierDeclScope::EventHandlerVar => {
                self.add_code(&format!("{} ", var_type));
                if !var_type.is_empty() {
                    self.add_code(&format!("{} ", var_name));
                }
                if !code.is_empty() {
                    self.add_code(&format!(" = {};", code));
                } else {
                    self.add_code(&format!(";"));
                }
            }
            _ => panic!("Error - unexpected scope for variable declaration"),
        }

        self.serialize
            .push(format!("\tbag.domain[\"{}\"] = {};", var_name, var_name));
        self.deserialize
            .push(format!("\t{} = bag.domain[\"{}\"];", var_name, var_name));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_impl_node(&mut self, action_node: &ActionNode) {
        let mut subclass_code = String::new();

        self.newline();
        self.newline();

        let action_ret_type: String = match &action_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from("void"),
        };

        let action_name = self.format_action_name(&action_node.name);
        self.add_code(&format!("protected {} {}(", action_ret_type, action_name));
        match &action_node.params {
            Some(params) => {
                self.format_actions_parameter_list(params, &mut subclass_code);
            }
            None => {}
        }

        self.add_code(") {");
        // self.subclass_code.push(subclass_code);
        self.indent();
        self.newline();
        self.add_code(action_node.code_opt.as_ref().unwrap().as_str());
        self.outdent();
        self.newline();
        self.add_code("}");
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

    // fn visit_assignment_expr_node(&mut self, assignment_expr_node: &AssignmentExprNode) {
    //     self.generate_comment(assignment_expr_node.line);
    //     self.newline();
    //     self.add_code("=");
    //     assignment_expr_node.r_value_box.accept(self);
    //     self.add_code(";");
    // }
    fn visit_assignment_expr_node(&mut self, assignment_expr_node: &AssignmentExprNode) {
        self.newline();
        let mut code = String::new();
        assignment_expr_node.accept_to_string(self, &mut code);
        self.add_code(&code);
    }
    //* --------------------------------------------------------------------- */
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
        output.push_str("");
        self.expr_context = ExprContext::Rvalue;
        assignment_expr_node
            .r_value_box
            .accept_to_string(self, output);
        output.push_str(");");
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

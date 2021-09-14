use convert_case::{Case, Casing};
use yaml_rust::Yaml;

use super::super::ast::*;
use super::super::scanner::{Token, TokenType};
use super::super::symbol_table::*;
use super::super::visitors::*;

struct ConfigFeatures {
    follow_rust_naming: bool,
    generate_action_impl: bool,
    runtime_support: bool,
}

struct Config {
    config_features: ConfigFeatures,
    actions_prefix: String,
    actions_suffix: String,
    action_prefix: String,
    action_suffix: String,
    enter_token: String,
    exit_token: String,
    enter_msg: String,
    exit_msg: String,
    enter_args_member_name: String,
    enter_args_suffix: String,
    exit_args_member_name: String,
    state_args_var: String,
    state_args_suffix: String,
    state_vars_var_name: String,
    state_vars_suffix: String,
    state_stack_var_name: String,
    state_context_name: String,
    state_context_suffix: String,
    state_context_var_name: String,
    state_context_method_suffix: String,
    this_state_context_var_name: String,
    frame_event_message_type_name: String,
    frame_event_type_name: String,
    frame_event_parameter_type_name: String,
    frame_event_parameters_type_name: String,
    frame_event_return: String,
    frame_event_variable_name: String,
    frame_event_parameters_attribute_name: String,
    frame_event_message_attribute_name: String,
    frame_event_return_attribute_name: String,
    state_var_name: String,
    state_handler_name_prefix: String,
    state_handler_name_suffix: String,
    state_enum_suffix: String,
    state_enum_traits: String,
    initialize_method_name: String,
    handle_event_method_name: String,
    transition_method_name: String,
    change_state_method_name: String,
    state_stack_push_method_name: String,
    state_stack_pop_method_name: String,
}

impl Config {
    fn new(rust_yaml: &Yaml) -> Config {
        // println!("{:?}", rust_yaml);
        let features_yaml = &rust_yaml["features"];
        let config_features = ConfigFeatures {
            follow_rust_naming: (&features_yaml["follow_rust_naming"])
                .as_bool()
                .unwrap_or(true)
                .to_string()
                .parse()
                .unwrap(),
            generate_action_impl: (&features_yaml["generate_action_impl"])
                .as_bool()
                .unwrap_or(true)
                .to_string()
                .parse()
                .unwrap(),
            runtime_support: (&features_yaml["runtime_support"])
                .as_bool()
                .unwrap_or(true)
                .to_string()
                .parse()
                .unwrap(),
        };
        let code_yaml = &rust_yaml["code"];

        Config {
            config_features,
            actions_prefix: (&code_yaml["actions_prefix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            actions_suffix: (&code_yaml["actions_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            action_prefix: (&code_yaml["action_prefix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            action_suffix: (&code_yaml["action_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            enter_token: String::from(">"),
            exit_token: String::from("<"),
            enter_msg: (&code_yaml["enter_msg"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            exit_msg: (&code_yaml["exit_msg"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            enter_args_member_name: (&code_yaml["enter_args_member_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            enter_args_suffix: (&code_yaml["enter_args_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            exit_args_member_name: (&code_yaml["exit_args_member_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_var_name: (&code_yaml["state_var_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_handler_name_prefix: (&code_yaml["state_handler_name_prefix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_handler_name_suffix: (&code_yaml["state_handler_name_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_enum_suffix: (&code_yaml["state_enum_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_enum_traits: (&code_yaml["state_enum_traits"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_context_var_name: (&code_yaml["state_context_var_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            this_state_context_var_name: (&code_yaml["this_state_context_var_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_context_method_suffix: (&code_yaml["state_context_method_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_type_name: (&code_yaml["frame_event_type_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_parameter_type_name: (&code_yaml["frame_event_parameter_type_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_parameters_type_name: (&code_yaml["frame_event_parameters_type_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_message_type_name: (&code_yaml["frame_event_message_type_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_return: (&code_yaml["frame_event_return"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_variable_name: (&code_yaml["frame_event_variable_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_parameters_attribute_name: (&code_yaml
                ["frame_event_parameters_attribute_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_message_attribute_name: (&code_yaml["frame_event_message_attribute_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            frame_event_return_attribute_name: (&code_yaml["frame_event_return_attribute_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_context_name: (&code_yaml["state_context_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_context_suffix: (&code_yaml["state_context_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_args_var: (&code_yaml["state_args_var"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_args_suffix: (&code_yaml["state_args_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_vars_var_name: (&code_yaml["state_vars_var_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_vars_suffix: (&code_yaml["state_vars_suffix"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_stack_var_name: (&code_yaml["state_stack_var_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            initialize_method_name: (&code_yaml["initialize_method_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            handle_event_method_name: (&code_yaml["handle_event_method_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            transition_method_name: (&code_yaml["transition_method_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            change_state_method_name: (&code_yaml["change_state_method_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_stack_push_method_name: (&code_yaml["state_stack_push_method_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
            state_stack_pop_method_name: (&code_yaml["state_stack_pop_method_name"])
                .as_str()
                .unwrap_or_default()
                .to_string(),
        }
    }
}

pub struct RustVisitor {
    config: Config,
    compiler_version: String,
    code: String,
    dent: usize,
    current_state_name_opt: Option<String>,
    current_event_ret_type: String,
    arcanum: Arcanum,
    symbol_config: SymbolConfig,
    comments: Vec<Token>,
    current_comment_idx: usize,
    system_name: String,
    first_state_name: String,
    serialize: Vec<String>,
    deserialize: Vec<String>,
    warnings: Vec<String>,
    has_states: bool,
    errors: Vec<String>,
    visiting_call_chain_literal_variable: bool,
    this_branch_transitioned: bool,
    generate_exit_args: bool,
    generate_state_context: bool,
    generate_state_stack: bool,
    generate_change_state: bool,
    generate_transition_state: bool,
    current_message: String,
}

impl RustVisitor {
    pub fn new(
        arcanum: Arcanum,
        config_yaml: &Yaml,
        generate_exit_args: bool,
        generate_state_context: bool,
        generate_state_stack: bool,
        generate_change_state: bool,
        generate_transition_state: bool,
        compiler_version: &str,
        comments: Vec<Token>,
    ) -> RustVisitor {
        let config = RustVisitor::load_config(config_yaml);

        RustVisitor {
            compiler_version: compiler_version.to_string(),
            code: String::from(""),
            dent: 0,
            current_state_name_opt: None,
            current_event_ret_type: String::new(),
            arcanum,
            symbol_config: SymbolConfig::new(),
            comments,
            current_comment_idx: 0,
            system_name: String::new(),
            first_state_name: String::new(),
            serialize: Vec::new(),
            deserialize: Vec::new(),
            has_states: false,
            errors: Vec::new(),
            warnings: Vec::new(),
            visiting_call_chain_literal_variable: false,
            this_branch_transitioned: false,
            generate_exit_args,
            generate_state_context,
            generate_state_stack,
            generate_change_state,
            generate_transition_state,
            current_message: String::new(),
            config,
        }
    }

    //* --------------------------------------------------------------------- *//

    fn load_config(config_yaml: &Yaml) -> Config {
        let codegen_yaml = &config_yaml["codegen"];
        let rust_yaml = &codegen_yaml["rust"];
        let config = Config::new(&rust_yaml);
        config
    }

    //* --------------------------------------------------------------------- *//

    /// Enter/exit messages are formatted "stateName:>" or "stateName:<".
    pub fn is_enter_or_exit_message(&self, msg: &str) -> bool {
        let split = msg.split(":");
        let vec: Vec<&str> = split.collect();
        vec.len() == 2
    }

    //* --------------------------------------------------------------------- *//

    pub fn get_msg_enum(&self, msg: &str) -> String {
        let unformatted = match msg {
            // ">>" => self.config.start_system_msg.clone(),
            // "<<" => self.config.stop_system_msg.clone(),
            ">" => self.config.enter_msg.clone(),
            "<" => self.config.exit_msg.clone(),
            _ => self.arcanum.get_interface_or_msg_from_msg(msg).unwrap(),
        };
        self.format_type_name(&unformatted)
    }

    //* --------------------------------------------------------------------- *//

    fn parse_event_name(&self, event_name: &str) -> (Option<String>, String) {
        let split = event_name.split(":");
        let vec: Vec<&str> = split.collect();
        if vec.len() == 1 {
            let event_name = vec.get(0).unwrap();
            (None, event_name.to_string().clone())
        } else {
            let state_name = vec.get(0).unwrap();
            let event_name = vec.get(1).unwrap();
            (Some(state_name.to_string()), event_name.to_string())
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_frame_event_parameter_name(
        &self,
        unparsed_event_name: &str,
        param_name: &str,
    ) -> String {
        let (state_name_opt, event_name) = self.parse_event_name(&unparsed_event_name);
        let unformatted_name = match &state_name_opt {
            Some(state_name) => format!(
                "{}_{}_{}",
                state_name,
                &*self.canonical_event_name(&event_name),
                param_name
            ),
            None => {
                let message_opt = self.arcanum.get_interface_or_msg_from_msg(&event_name);
                match &message_opt {
                    Some(canonical_message_name) => {
                        format!("{}_{}", canonical_message_name, param_name)
                    }
                    None => format!("<Error - unknown message {}>,", &unparsed_event_name),
                }
            }
        };
        self.format_value_name(&unformatted_name)
    }

    //* --------------------------------------------------------------------- *//

    fn canonical_event_name(&self, msg_name: &str) -> String {
        if msg_name.eq(&self.config.enter_token) {
            return self.config.enter_msg.clone();
        } else if msg_name.eq(&self.config.exit_token) {
            return self.config.exit_msg.clone();
        }
        return msg_name.to_string();
    }

    //* --------------------------------------------------------------------- *//

    pub fn get_code(&self) -> String {
        if self.errors.len() > 0 {
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

    fn format_variable_expr(&mut self, variable_node: &VariableNode) -> String {
        let mut code = String::new();

        match variable_node.scope {
            IdentifierDeclScope::DomainBlock => {
                if variable_node.id_node.is_reference {
                    code.push_str("&");
                }
                code.push_str(&format!(
                    "self.{}",
                    self.format_value_name(&variable_node.id_node.name.lexeme)
                ));
            }
            IdentifierDeclScope::StateParam => {
                let var_node = variable_node;
                let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                let _var_symbol = var_symbol_rcref.borrow();
                //               let var_type = self.get_variable_type(&*var_symbol);

                if self.visiting_call_chain_literal_variable {
                    code.push_str("(");
                }

                if var_node.id_node.is_reference {
                    code.push_str("&");
                }

                code.push_str(&format!(
                    "{}.{}.{}",
                    self.config.this_state_context_var_name,
                    self.config.state_args_var,
                    self.format_value_name(&variable_node.id_node.name.lexeme)
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push_str(")");
                }
            }
            IdentifierDeclScope::StateVar => {
                let var_node = variable_node;
                let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                let _var_symbol = var_symbol_rcref.borrow();
                //                let var_type = self.get_variable_type(&*var_symbol);

                if self.visiting_call_chain_literal_variable {
                    code.push_str("(");
                }

                if var_node.id_node.is_reference {
                    code.push_str("&");
                }

                code.push_str(&format!(
                    "{}.{}.{}",
                    self.config.this_state_context_var_name,
                    self.config.state_vars_var_name,
                    self.format_value_name(&variable_node.id_node.name.lexeme)
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push_str(")");
                }
            }
            IdentifierDeclScope::EventHandlerParam => {
                if self.visiting_call_chain_literal_variable {
                    code.push_str("(");
                }

                if variable_node.id_node.is_reference {
                    code.push_str("&");
                }

                // if generating state context and is the enter event...
                if self.generate_state_context && self.config.enter_token == self.current_message {
                    code.push_str(&format!(
                        "{}.{}.as_ref().unwrap().{}",
                        self.config.this_state_context_var_name,
                        self.config.enter_args_member_name,
                        self.format_value_name(&variable_node.id_node.name.lexeme)
                    ));
                } else if self.config.exit_token == self.current_message {
                    code.push_str(&format!(
                        "{}.{}.as_ref().unwrap().{}()",
                        self.config.frame_event_variable_name,
                        self.config.frame_event_parameters_attribute_name,
                        self.format_exit_param_getter(
                            self.current_state_name_opt.as_ref().unwrap(),
                            &self.config.exit_msg,
                            &variable_node.id_node.name.lexeme
                        )
                    ));
                } else {
                    let msg = match &self
                        .arcanum
                        .get_interface_or_msg_from_msg(&self.current_message)
                    {
                        Some(canonical_message_name) => format!("{}", canonical_message_name),
                        None => {
                            self.errors.push(format!(
                                "<Error - unknown message {}>,",
                                &self.current_message
                            ));
                            format!("<Error - unknown message {}>", &self.current_message)
                        }
                    };
                    code.push_str(&format!(
                        "{}.{}.as_ref().unwrap().{}()",
                        self.config.frame_event_variable_name,
                        self.config.frame_event_parameters_attribute_name,
                        self.format_event_param_getter(&msg, &variable_node.id_node.name.lexeme)
                    ));
                }

                if self.visiting_call_chain_literal_variable {
                    code.push_str(")");
                }
            }
            IdentifierDeclScope::EventHandlerVar => {
                if variable_node.id_node.is_reference {
                    code.push_str("&");
                }
                code.push_str(&self.format_value_name(&variable_node.id_node.name.lexeme));
            }
            IdentifierDeclScope::None => {
                // TODO: Explore labeling Variables as "extern" scope
                if variable_node.id_node.is_reference {
                    code.push_str("&");
                }
                code.push_str(&self.format_value_name(&variable_node.id_node.name.lexeme));
            } // Actions?
            _ => self.errors.push("Illegal scope.".to_string()),
        }

        code
    }

    //* --------------------------------------------------------------------- *//

    fn format_parameter_list(&mut self, params: &Vec<ParameterNode>) {
        for param in params {
            self.add_code(&format!(", "));
            let param_type: String = match &param.param_type_opt {
                Some(ret_type) => ret_type.get_type_str(),
                None => String::from("<?>"),
            };
            self.add_code(&format!(
                "{}: {}",
                self.format_value_name(&param.param_name),
                param_type
            ));
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_actions_parameter_list(&mut self, params: &Vec<ParameterNode>) {
        for param in params {
            self.add_code(", ");
            let param_type: String = match &param.param_type_opt {
                Some(ret_type) => ret_type.get_type_str(),
                None => String::from("<?>"),
            };
            self.add_code(&format!(
                "{}: {}",
                self.format_value_name(&param.param_name),
                param_type
            ));
        }
    }

    //* --------------------------------------------------------------------- *//

    fn new_state_var_name(&self) -> String {
        format!("new_{}", self.config.state_var_name)
    }

    fn old_state_var_name(&self) -> String {
        format!("old_{}", self.config.state_var_name)
    }

    fn new_state_context_var_name(&self) -> String {
        format!("new_{}", self.config.state_context_var_name)
    }

    fn state_enum_type_name(&self) -> String {
        self.format_type_name(&format!(
            "{}{}",
            self.system_name, self.config.state_enum_suffix
        ))
    }

    //* --------------------------------------------------------------------- *//

    // Formatting helper functions

    /// Format a "type-level" name, e.g. a type, trait, or enum variant.
    /// If Rust naming conventions are followed, these are in CamelCase.
    fn format_type_name(&self, name: &String) -> String {
        let mut formatted = name.clone();
        if self.config.config_features.follow_rust_naming {
            formatted = formatted.to_case(Case::UpperCamel);
        }
        formatted
    }

    /// Format a "value-level" name, e.g. a function, method, variable.
    /// If Rust naming conventions are followed, these are  in snake_case.
    fn format_value_name(&self, name: &String) -> String {
        let mut formatted = name.clone();
        if self.config.config_features.follow_rust_naming {
            formatted = formatted.to_case(Case::Snake);
        }
        formatted
    }

    fn format_getter_name(&self, member_name: &str) -> String {
        format!("get_{}", self.format_value_name(&member_name.to_string()))
    }

    fn format_setter_name(&self, member_name: &str) -> String {
        format!("set_{}", self.format_value_name(&member_name.to_string()))
    }

    // Type/case names

    fn format_enter_args_struct_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_type_name(&state_name.to_string()),
            self.config.enter_args_suffix
        )
    }

    fn format_state_args_struct_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_type_name(&state_name.to_string()),
            self.config.state_args_suffix
        )
    }

    fn format_state_context_struct_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_type_name(&state_name.to_string()),
            self.config.state_context_suffix
        )
    }

    fn format_state_vars_struct_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_type_name(&state_name.to_string()),
            self.config.state_vars_suffix
        )
    }

    // Method names

    fn format_action_name(&mut self, action_name: &String) -> String {
        format!(
            "{}{}{}",
            self.config.action_prefix,
            self.format_value_name(action_name),
            self.config.action_suffix
        )
    }

    fn format_event_param_getter(&self, message_name: &str, param_name: &str) -> String {
        self.format_getter_name(&format!("{}_{}", message_name, param_name))
    }

    fn format_exit_param_getter(
        &self,
        state_name: &str,
        message_name: &str,
        param_name: &str,
    ) -> String {
        self.format_getter_name(&format!("{}_{}_{}", state_name, message_name, param_name))
    }

    fn format_state_context_method_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_value_name(&state_name.to_string()),
            self.config.state_context_method_suffix
        )
    }

    fn format_state_handler_name(&self, state_name: &str) -> String {
        format!(
            "{}{}{}",
            self.config.state_handler_name_prefix,
            self.format_value_name(&state_name.to_string()),
            self.config.state_handler_name_suffix
        )
    }

    //* --------------------------------------------------------------------- *//

    pub fn run(&mut self, system_node: &SystemNode) {
        system_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn add_code(&mut self, s: &str) {
        self.code.push_str(&*format!("{}", s));
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
        return (0..self.dent).map(|_| "    ").collect::<String>();
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

    fn enter_block(&mut self) {
        self.add_code("{");
        self.indent();
        self.newline();
    }

    fn exit_block(&mut self) {
        self.outdent();
        self.newline();
        self.add_code("}");
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

    /// Generate an enum type that enumerates the states of the machine.
    fn generate_state_enum(&mut self, system_node: &SystemNode) {
        // add derived traits
        let mut traits = self.config.state_enum_traits.clone();
        match &system_node.attributes_opt {
            Some(attributes) => {
                if let Some(new_traits) = attributes.get("override_state_enum_traits") {
                    traits = new_traits.value.clone();
                }
                if let Some(new_traits) = attributes.get("extend_state_enum_traits") {
                    if traits.is_empty() {
                        traits = new_traits.value.clone();
                    } else {
                        traits = format!("{}, {}", traits, new_traits.value);
                    }
                }
            }
            None => {}
        }
        if !self.config.config_features.follow_rust_naming {
            self.add_code("#[allow(clippy::upper_case_acronyms)]");
            self.newline();
        }
        self.add_code("#[allow(dead_code)]");
        self.newline();
        if !traits.is_empty() {
            self.add_code(&format!("#[derive({})]", traits));
            self.newline();
        }

        // add the state enum type
        self.add_code(&format!("pub enum {} {{", self.state_enum_type_name()));
        self.indent();
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            for state in &machine_block_node.states {
                self.newline();
                self.add_code(&format!("{},", self.format_type_name(&state.borrow().name)));
            }
        }
        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the struct, enum, and supporting function definitions
    /// related to state contexts. State contexts include state parameters,
    /// state variables, and enter event parameters. Note that exit event
    /// parameters are handed by a separate mechanism.
    fn generate_state_context_defs(&mut self, system_node: &SystemNode) {
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            for state in &machine_block_node.states {
                let state_node = state.borrow();

                // generate state parameter declarations for this state
                let state_args_struct_name = self.format_state_args_struct_name(&state_node.name);
                let mut has_state_args = false;
                match &state_node.params_opt {
                    Some(params) => {
                        has_state_args = true;
                        let mut bound_names: Vec<String> = Vec::new();

                        self.add_code("#[allow(dead_code)]");
                        self.newline();
                        self.add_code(&format!("struct {} {{", state_args_struct_name));
                        self.indent();
                        for param in params {
                            let param_name = self.format_value_name(&param.param_name);
                            let param_type = match &param.param_type_opt {
                                Some(param_type) => param_type.get_type_str(),
                                None => String::from("<?>"),
                            };
                            self.newline();
                            self.add_code(&format!("{}: {},", param_name, param_type));
                            bound_names.push(param_name);
                        }
                        self.exit_block();
                        self.newline();
                        self.newline();

                        if self.config.config_features.runtime_support {
                            self.generate_environment_impl(&state_args_struct_name, bound_names);
                        }
                    }
                    None => {}
                }

                // generate state variable declarations for this state
                let state_vars_struct_name = self.format_state_vars_struct_name(&state_node.name);
                let mut has_state_vars = false;
                match &state_node.vars_opt {
                    Some(var_decl_nodes) => {
                        has_state_vars = true;
                        let mut bound_names: Vec<String> = Vec::new();

                        self.add_code("#[allow(dead_code)]");
                        self.newline();
                        self.add_code(&format!("struct {} {{", state_vars_struct_name));
                        self.indent();
                        for var_decl_node in var_decl_nodes {
                            let var_name = self.format_value_name(&var_decl_node.borrow().name);
                            let var_type = match &var_decl_node.borrow().type_opt {
                                Some(var_type) => var_type.get_type_str(),
                                None => "<?>".to_string(),
                            };
                            self.newline();
                            self.add_code(&format!("{}: {},", var_name, var_type));
                            bound_names.push(var_name);
                        }
                        self.exit_block();
                        self.newline();
                        self.newline();

                        if self.config.config_features.runtime_support {
                            self.generate_environment_impl(&state_vars_struct_name, bound_names);
                        }
                    }
                    None => {}
                }

                // generate enter event parameters for this state
                let enter_args_struct_name = self.format_enter_args_struct_name(&state_node.name);
                let mut has_enter_event_params = false;
                match &state_node.enter_event_handler_opt {
                    Some(enter_event_handler) => {
                        let eeh_ref = &enter_event_handler.borrow();
                        let event_symbol = eeh_ref.event_symbol_rcref.borrow();
                        match &event_symbol.params_opt {
                            Some(params) => {
                                has_enter_event_params = true;
                                let mut bound_names: Vec<String> = Vec::new();

                                self.add_code("#[allow(dead_code)]");
                                self.newline();
                                self.add_code(&format!("struct {} {{", enter_args_struct_name));
                                self.indent();
                                for param in params {
                                    let param_name = self.format_value_name(&param.name);
                                    let param_type = match &param.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => "<?>".to_string(),
                                    };
                                    self.newline();
                                    self.add_code(&format!("{}: {},", param_name, param_type));
                                    bound_names.push(param_name);
                                }
                                self.exit_block();
                                self.newline();
                                self.newline();

                                if self.config.config_features.runtime_support {
                                    self.generate_environment_impl(
                                        &enter_args_struct_name,
                                        bound_names,
                                    );
                                }
                            }
                            None => {}
                        }
                    }
                    None => {}
                }

                // generate state context struct for this state
                let context_struct_name = self.format_state_context_struct_name(&state_node.name);
                self.add_code("#[allow(dead_code)]");
                self.newline();
                self.add_code(&format!("struct {} {{", context_struct_name));
                self.indent();

                if has_state_args {
                    self.newline();
                    self.add_code(&format!(
                        "{}: {},",
                        self.config.state_args_var, state_args_struct_name
                    ));
                }

                if has_state_vars {
                    self.newline();
                    self.add_code(&format!(
                        "{}: {},",
                        self.config.state_vars_var_name, state_vars_struct_name
                    ));
                }

                if has_enter_event_params {
                    self.newline();
                    self.add_code(&format!(
                        "{}: Option<{}>,",
                        self.config.enter_args_member_name, enter_args_struct_name
                    ));
                }
                self.exit_block();
                self.newline();
                self.newline();

                // generate implementation of runtime state
                if self.config.config_features.runtime_support {
                    self.add_code(&format!("impl State for {} ", context_struct_name));
                    self.enter_block();

                    self.add_code("fn name(&self) -> &'static str ");
                    self.enter_block();
                    self.add_code(&format!("\"{}\"", &state_node.name));
                    self.exit_block();
                    self.newline();

                    self.add_code("fn state_arguments(&self) -> &dyn Environment ");
                    self.enter_block();
                    if has_state_args {
                        self.add_code(&format!("&self.{}", self.config.state_args_var));
                    } else {
                        self.add_code("EMPTY");
                    }
                    self.exit_block();
                    self.newline();

                    self.add_code("fn state_variables(&self) -> &dyn Environment ");
                    self.enter_block();
                    if has_state_vars {
                        self.add_code(&format!("&self.{}", self.config.state_vars_var_name));
                    } else {
                        self.add_code("EMPTY");
                    }
                    self.exit_block();
                    self.newline();

                    self.add_code("fn enter_arguments(&self) -> &dyn Environment ");
                    self.enter_block();
                    if has_enter_event_params {
                        self.add_code(&format!(
                            "match &self.{} ",
                            self.config.enter_args_member_name
                        ));
                        self.enter_block();
                        self.add_code("Some(args) => args,");
                        self.newline();
                        self.add_code("None => EMPTY");
                        self.exit_block();
                    } else {
                        self.add_code("EMPTY");
                    }
                    self.exit_block();

                    self.exit_block();
                    self.newline();
                    self.newline();
                }
            }

            // generate the enum type that unions all the state context types
            self.add_code("#[allow(dead_code)]");
            self.newline();
            self.add_code(&format!("enum {} {{", self.config.state_context_name));
            self.indent();
            for state in &machine_block_node.states {
                self.newline();
                let state_node = state.borrow();
                self.add_code(&format!(
                    "{}(RefCell<{}>),",
                    self.format_type_name(&state_node.name),
                    self.format_state_context_struct_name(&state_node.name)
                ));
            }
            self.exit_block();
            self.newline();
            self.newline();

            // generate methods to conveniently get specific state contexts
            // from a value of the enum type
            self.add_code("#[allow(dead_code)]");
            self.newline();
            self.add_code(&format!("impl {} {{", self.config.state_context_name));
            self.indent();
            for state in &machine_block_node.states {
                let state_node = state.borrow();
                self.newline();
                self.newline();
                self.add_code(&format!(
                    "fn {}(&self) -> &RefCell<{}> {{",
                    self.format_state_context_method_name(&state_node.name),
                    self.format_state_context_struct_name(&state_node.name)
                ));
                self.indent();
                self.newline();
                self.add_code("match self {");
                self.indent();
                self.newline();
                self.add_code(&format!(
                    "{}::{}(context) => context,",
                    self.config.state_context_name,
                    self.format_type_name(&state_node.name)
                ));
                self.newline();
                self.add_code(&format!(
                    "_ => panic!(\"Failed conversion to {}\"),",
                    self.format_state_context_struct_name(&state_node.name)
                ));
                self.exit_block();
                self.exit_block();
            }
            self.newline();
            self.exit_block();
        }
    }

    fn generate_environment_impl(&mut self, type_name: &str, bound_names: Vec<String>) {
        self.add_code(&format!("impl Environment for {} ", type_name));
        self.enter_block();
        self.add_code("fn lookup(&self, name: &str) -> Option<&dyn Any> ");
        self.enter_block();
        self.add_code("match name ");
        self.enter_block();
        for name in bound_names {
            self.add_code(&format!("\"{0}\" => Some(&self.{0}),", name));
            self.newline();
        }
        self.add_code("_ => None");
        self.exit_block();
        self.exit_block();
        self.exit_block();
        self.newline();
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the constructor function.
    fn generate_constructor(&mut self, system_node: &SystemNode) {
        self.add_code(&format!("pub fn new() -> {} {{", system_node.name));
        self.indent();

        let init_state_name = self.first_state_name.clone();

        // initial state variables
        let mut formatted_state_vars = String::new();
        let has_state_vars =
            self.generate_state_variables(&init_state_name, &mut formatted_state_vars);

        // initial state context
        if self.generate_state_context {
            self.generate_next_state_context(
                &init_state_name,
                false,
                false,
                has_state_vars,
                "",
                "",
                &formatted_state_vars,
            );
        }

        // begin create state machine
        self.newline();
        self.add_code(&format!("let mut machine = {} {{", system_node.name));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "{}: {}::{},",
            self.config.state_var_name,
            self.state_enum_type_name(),
            self.format_type_name(&self.first_state_name)
        ));

        // initialize the state stack
        if self.generate_state_stack {
            self.newline();
            self.add_code(&format!(
                "{}: Vec::new(),",
                self.config.state_stack_var_name
            ));
        }

        // initialize domain variables
        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            for variable_decl_node_rcref in &domain_block_node.member_variables {
                let variable_decl_node = variable_decl_node_rcref.borrow();
                let variable_name = self.format_value_name(&variable_decl_node.name);
                let var_init_expr = &variable_decl_node.initializer_expr_t_opt.as_ref().unwrap();
                let mut code = String::new();
                var_init_expr.accept_to_string(self, &mut code);
                self.newline();
                self.add_code(&format!("{}: {},", variable_name, code));
            }
        }

        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "{}: next_state_context,",
                self.config.state_context_var_name
            ));
        }

        self.outdent();
        self.newline();
        self.add_code("};");
        // end of create state machine

        // run the initialize method on the new machine
        self.newline();
        self.add_code(&format!(
            "machine.{}();",
            self.config.initialize_method_name
        ));
        self.newline();

        // return the new machine
        self.add_code("machine");

        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the initialize method.
    fn generate_initialize(&mut self) {
        self.add_code(&format!(
            "pub fn {}(&mut self) {{",
            self.config.initialize_method_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "let mut e = {}::new({}::{}, None);",
            self.config.frame_event_type_name,
            self.config.frame_event_message_type_name,
            self.config.enter_msg
        ));
        self.newline();
        self.add_code(&format!(
            "self.{}(&mut e);",
            self.config.handle_event_method_name
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the event handling and state transition machinery.
    fn generate_machinery(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        self.add_code("//=============== Machinery and Mechanisms ==============//");
        self.newline();
        if let Some(_) = system_node.get_first_state() {
            self.newline();
            self.generate_handle_event(&system_node);
            if self.generate_transition_state {
                self.newline();
                self.generate_transition();
            }
            if self.generate_state_stack {
                self.newline();
                self.generate_state_stack_methods();
            }
            if self.generate_change_state {
                self.newline();
                self.generate_change_state();
            }
            if self.arcanum.is_serializable() {
                for line in self.serialize.iter() {
                    self.code.push_str(&*format!("{}", line));
                    self.code.push_str(&*format!("\n{}", self.dent()));
                }
                for line in self.deserialize.iter() {
                    self.code.push_str(&*format!("{}", line));
                    self.code.push_str(&*format!("\n{}", self.dent()));
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the change_state method.
    fn generate_change_state(&mut self) {
        if self.generate_state_context {
            self.add_code(&format!(
                "fn {}(&mut self, {}: {}, {}: Rc<{}>) {{",
                self.config.change_state_method_name,
                self.new_state_var_name(),
                self.state_enum_type_name(),
                self.new_state_context_var_name(),
                self.config.state_context_name
            ));
        } else {
            self.add_code(&format!(
                "fn {}(&mut self, {}: {}) {{",
                self.config.change_state_method_name,
                self.new_state_var_name(),
                self.state_enum_type_name()
            ));
        }
        self.indent();
        self.newline();
        self.add_code(&format!(
            "self.{} = {};",
            self.config.state_var_name,
            self.new_state_var_name()
        ));
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "self.{} = Rc::clone(&{});",
                self.config.state_context_var_name,
                self.new_state_context_var_name()
            ));
        }
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the transition method.
    fn generate_transition(&mut self) {
        // generate method signature
        if self.generate_state_context {
            if self.generate_exit_args {
                self.add_code(&format!(
                    "fn {}(&mut self, {}: Option<Box<{}>>, {}: {}, {}: Rc<{}>) {{",
                    self.config.transition_method_name,
                    self.config.exit_args_member_name,
                    self.config.frame_event_parameters_type_name,
                    self.new_state_var_name(),
                    self.state_enum_type_name(),
                    self.new_state_context_var_name(),
                    self.config.state_context_name
                ));
            } else {
                self.add_code(&format!(
                    "fn {}(&mut self, {}: {}, {}: Rc<{}>) {{",
                    self.config.transition_method_name,
                    self.new_state_var_name(),
                    self.state_enum_type_name(),
                    self.new_state_context_var_name(),
                    self.config.state_context_name
                ));
            }
        } else {
            if self.generate_exit_args {
                self.add_code(&format!(
                    "fn {}(&mut self, {}: Option<Box<{}>>, {}: {}) {{",
                    self.config.transition_method_name,
                    self.config.exit_args_member_name,
                    self.config.frame_event_parameters_type_name,
                    self.new_state_var_name(),
                    self.state_enum_type_name()
                ));
            } else {
                self.add_code(&format!(
                    "fn {}(&mut self, {}: {}) {{",
                    self.config.transition_method_name,
                    self.new_state_var_name(),
                    self.state_enum_type_name()
                ));
            }
        }
        // generate method body
        self.indent();
        self.newline();
        if self.generate_exit_args {
            self.add_code(&format!(
                "let mut exit_event = {}::new({}::{}, {});",
                self.config.frame_event_type_name,
                self.config.frame_event_message_type_name,
                self.config.exit_msg,
                self.config.exit_args_member_name
            ));
        } else {
            self.add_code(&format!(
                "let mut exit_event = {}::new({}::{}, None);",
                self.config.frame_event_type_name,
                self.config.frame_event_message_type_name,
                self.config.exit_msg
            ));
        }
        self.newline();
        self.add_code(&format!(
            "self.{}(&mut exit_event);",
            self.config.handle_event_method_name
        ));
        self.newline();
        self.add_code(&format!(
            "self.{} = {};",
            self.config.state_var_name,
            self.new_state_var_name()
        ));
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "self.{} = Rc::clone(&{});",
                self.config.state_context_var_name,
                self.new_state_context_var_name()
            ));
        }
        self.newline();
        self.add_code(&format!(
            "let mut enter_event = {}::new({}::{}, None);",
            self.config.frame_event_type_name,
            self.config.frame_event_message_type_name,
            self.config.enter_msg
        ));
        self.newline();
        self.add_code(&format!(
            "self.{}(&mut enter_event);",
            &self.config.handle_event_method_name
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
    }

    /// Generate state stack methods.
    fn generate_state_stack_methods(&mut self) {
        self.newline();
        self.add_code(&format!(
            "fn {}(&mut self) {{",
            self.config.state_stack_push_method_name
        ));
        self.indent();
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "self.{}.push((self.{}, Rc::clone(&self.{})));",
                self.config.state_stack_var_name,
                self.config.state_var_name,
                self.config.state_context_var_name
            ));
        } else {
            self.add_code(&format!(
                "self.{}.push(self.{});",
                self.config.state_stack_var_name, self.config.state_var_name
            ));
        }
        self.outdent();
        self.newline();
        self.add_code(&format!("}}"));
        self.newline();
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "fn {}(&mut self) -> ({}, Rc<{}>) {{",
                self.config.state_stack_pop_method_name,
                self.state_enum_type_name(),
                self.config.state_context_name
            ));
        } else {
            self.add_code(&format!(
                "fn {}(&mut self) -> {} {{",
                self.config.state_stack_pop_method_name,
                self.state_enum_type_name()
            ));
        }
        self.indent();
        self.newline();
        self.add_code(&format!(
            "match self.{}.pop() {{",
            self.config.state_stack_var_name
        ));
        self.indent();
        self.newline();
        self.add_code("Some(elem) => elem,");
        self.newline();
        self.add_code("None => panic!(\"Error: attempted to pop when history stack is empty.\")");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    /// Generate a return statement within a handler. Call this rather than
    /// adding a return statement directly to ensure that the control-flow
    /// state is properly maintained.
    fn generate_return(&mut self) {
        self.newline();
        self.add_code("return;");
        self.this_branch_transitioned = false;
    }

    /// Generate a return statement if the current branch contained a
    /// transition or change-state.
    fn generate_return_if_transitioned(&mut self) {
        if self.this_branch_transitioned {
            self.generate_return();
        }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_handle_event(&mut self, system_node: &SystemNode) {
        self.add_code(&format!(
            "fn {}(&mut self, {}: &mut {}) {{",
            self.config.handle_event_method_name,
            self.config.frame_event_variable_name,
            self.config.frame_event_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!("match self.{} {{", self.config.state_var_name));
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            self.indent();
            for state in &machine_block_node.states {
                self.newline();
                self.add_code(&format!(
                    "{}::{} => self.{}({}),",
                    self.state_enum_type_name(),
                    self.format_type_name(&state.borrow().name),
                    self.format_state_handler_name(&state.borrow().name),
                    self.config.frame_event_variable_name
                ));
            }
        }
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn generate_comment(&mut self, line: usize) {
        // can't use self.newline() or self.add_code() due to double borrow.
        while self.current_comment_idx < self.comments.len()
            && line >= self.comments[self.current_comment_idx].line
        {
            let comment = &self.comments[self.current_comment_idx];
            if comment.token_type == TokenType::SingleLineCommentTok {
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
                self.code.push_str(&*format!("*/"));
            }

            self.current_comment_idx += 1;
        }
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the arguments to the exit event handler, passed in a
    /// transition statement. Returns `true` if any exit arguments were passed.
    fn generate_exit_arguments(
        &mut self,
        target_state_name: &str,
        exit_args: &ExprListNode,
    ) -> bool {
        if exit_args.exprs_t.len() <= 0 {
            return false;
        }
        // Search for event keyed with "State:<", e.g. "S1:<"
        let exit_msg = format!(
            "{}:{}",
            self.current_state_name_opt
                .as_ref()
                .unwrap_or(&String::new()),
            self.symbol_config.exit_msg_symbol
        );
        if let Some(event_sym) = self
            .arcanum
            .get_event(&exit_msg, &self.current_state_name_opt)
        {
            match &event_sym.borrow().params_opt {
                Some(event_params) => {
                    if exit_args.exprs_t.len() != event_params.len() {
                        self.errors
                            .push("Fatal error: misaligned parameters to arguments.".to_string());
                    }
                    let mut param_symbols_it = event_params.iter();
                    self.newline();
                    self.add_code(&format!(
                        "let mut {} = Box::new({}::new());",
                        self.config.exit_args_member_name,
                        self.config.frame_event_parameters_type_name
                    ));
                    // Loop through the ARGUMENTS...
                    for expr_t in &exit_args.exprs_t {
                        // ...and validate w/ the PARAMETERS
                        match param_symbols_it.next() {
                            Some(p) => {
                                let mut expr = String::new();
                                expr_t.accept_to_string(self, &mut expr);
                                let parameter_enum_name =
                                    self.format_frame_event_parameter_name(&exit_msg, &p.name);
                                self.newline();
                                self.add_code(&format!(
                                    "(*{}).{}({});",
                                    self.config.exit_args_member_name,
                                    self.format_setter_name(&parameter_enum_name),
                                    expr
                                ));
                            }
                            None => self.errors.push(format!(
                                "Invalid number of arguments for \"{}\" event handler.",
                                exit_msg
                            )),
                        }
                    }
                }
                None => self
                    .errors
                    .push(format!("Fatal error: misaligned parameters to arguments.")),
            }
        } else {
            let current_state_name = &self.current_state_name_opt.as_ref().unwrap();
            self.errors.push(format!(
                "Missing exit event handler for transition from ${} to ${}.",
                current_state_name, &target_state_name
            ));
        }
        true
    }

    /// Does a transition from the current state to the target state require
    /// enter arguments?
    fn requires_enter_arguments(&mut self, target_state_name: &str) -> bool {
        let enter_msg = format!(
            "{}:{}",
            target_state_name, &self.symbol_config.enter_msg_symbol
        );
        if let Some(event_sym) = self
            .arcanum
            .get_event(&enter_msg, &self.current_state_name_opt)
        {
            event_sym.borrow().params_opt.is_some()
        } else {
            false
        }
    }

    /// Generate the arguments to the enter event handler, passed in a
    /// transition statement. The formatted arguments are returned via a
    /// string reference. Returns `true` if any enter arguments were passed.
    fn generate_enter_arguments(
        &mut self,
        target_state_name: &str,
        enter_args: &ExprListNode,
        formatted_enter_args: &mut String,
    ) -> bool {
        let mut has_enter_args = false;
        // Search for event keyed with "State:>", e.g. "S1:>"
        let enter_msg = format!(
            "{}:{}",
            target_state_name, &self.symbol_config.enter_msg_symbol
        );
        formatted_enter_args.push_str(&format!(
            "{} {{ ",
            self.format_enter_args_struct_name(&target_state_name)
        ));
        if let Some(event_sym) = self
            .arcanum
            .get_event(&enter_msg, &self.current_state_name_opt)
        {
            match &event_sym.borrow().params_opt {
                Some(event_params) => {
                    has_enter_args = true;
                    if enter_args.exprs_t.len() != event_params.len() {
                        self.errors
                            .push(format!("Fatal error: misaligned parameters to arguments."));
                    }
                    let mut param_symbols_it = event_params.iter();
                    for expr_t in &enter_args.exprs_t {
                        match param_symbols_it.next() {
                            Some(p) => {
                                let mut expr = String::new();
                                expr_t.accept_to_string(self, &mut expr);
                                formatted_enter_args.push_str(&format!("{}: {}, ", p.name, expr));
                            }
                            None => self.errors.push(format!(
                                "Invalid number of arguments for \"{}\" event handler.",
                                enter_msg
                            )),
                        }
                    }
                }
                None => self.errors.push(format!(
                    "Invalid number of arguments for \"{}\" event handler.",
                    enter_msg
                )),
            }
        } else {
            self.warnings.push(format!("State {} does not have an enter event handler but is being passed parameters in a transition", target_state_name));
        }
        formatted_enter_args.push_str("}");
        has_enter_args
    }

    /// Generate the arguments to the next state in a transition/change-state.
    /// The formatted arguments are returned via a string reference. Returns
    /// `true` if any state arguments were passed.
    fn generate_state_arguments(
        &mut self,
        target_state_name: &str,
        state_args: &ExprListNode,
        formatted_state_args: &mut String,
    ) -> bool {
        if state_args.exprs_t.len() <= 0 {
            return false;
        }
        formatted_state_args.push_str(&format!(
            "{} {{ ",
            self.format_state_args_struct_name(&target_state_name)
        ));
        if let Some(state_sym) = self.arcanum.get_state(&target_state_name) {
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
                                formatted_state_args.push_str(&format!(
                                    "{}: {}, ",
                                    self.format_value_name(&param_symbol.name),
                                    expr
                                ));
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
            self.errors.push(format!("TODO"));
        }
        formatted_state_args.push_str("}");
        true
    }

    /// Generate the state variables for the next state after a transition or
    /// change-state. The formatted variable definitions are returned via a
    /// string reference. Returns `true` if the next state contains any state
    /// variables.
    fn generate_state_variables(
        &mut self,
        target_state_name: &str,
        formatted_state_vars: &mut String,
    ) -> bool {
        let mut has_state_vars = false;
        if let Some(state_symbol_rcref) = self.arcanum.get_state(&target_state_name) {
            let state_symbol = state_symbol_rcref.borrow();
            let state_node = &state_symbol.state_node.as_ref().unwrap().borrow();
            // generate local state variables
            if state_node.vars_opt.is_some() {
                has_state_vars = true;
                formatted_state_vars.push_str(&format!(
                    "{} {{",
                    self.format_state_vars_struct_name(&target_state_name)
                ));
                for var_rcref in state_node.vars_opt.as_ref().unwrap() {
                    let var = var_rcref.borrow();
                    let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
                    let mut expr_code = String::new();
                    expr_t.accept_to_string(self, &mut expr_code);
                    formatted_state_vars.push_str(&format!(
                        "{}: {},",
                        self.format_value_name(&var.name),
                        expr_code
                    ));
                }
                formatted_state_vars.push_str("}");
            }
        }
        has_state_vars
    }

    /// Generate code that initializes a new state context value stored in a
    /// local variable named `next_state_context`.
    fn generate_next_state_context(
        &mut self,
        target_state_name: &str,
        has_enter_args: bool,
        has_state_args: bool,
        has_state_vars: bool,
        enter_args: &str,
        state_args: &str,
        state_vars: &str,
    ) {
        self.newline();
        self.add_code(&format!(
            "let context = {} {{",
            self.format_state_context_struct_name(target_state_name)
        ));
        self.indent();
        if has_state_args {
            self.newline();
            self.add_code(&format!("{}: {},", self.config.state_args_var, state_args));
        }
        if has_state_vars {
            self.newline();
            self.add_code(&format!(
                "{}: {},",
                self.config.state_vars_var_name, state_vars
            ));
        }
        if has_enter_args {
            self.newline();
            self.add_code(&format!(
                "{}: {},",
                self.config.enter_args_member_name, enter_args
            ));
        }
        self.outdent();
        self.newline();
        self.add_code("};");
        self.newline();
        self.add_code(&format!(
            "let next_state_context = Rc::new({}::{}(RefCell::new(context)));",
            self.config.state_context_name,
            self.format_type_name(&target_state_name.to_string())
        ));
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_change_state(&mut self, change_state_stmt: &ChangeStateStatementNode) {
        self.newline();
        self.add_code("// Start change state");

        // get the name of the next state
        let target_state_name = match &change_state_stmt.state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            _ => {
                self.errors
                    .push("Change state target not found.".to_string());
                "error"
            }
        };

        // print the change-state label, if provided
        match &change_state_stmt.label_opt {
            Some(label) => {
                self.newline();
                self.add_code(&format!("// {}", label));
            }
            None => {}
        }

        // generate state arguments
        let mut has_state_args = false;
        let mut formatted_state_args = String::new();
        match &change_state_stmt.state_context_t {
            StateContextType::StateRef { state_context_node } => {
                if let Some(state_args) = &state_context_node.state_ref_args_opt {
                    has_state_args = self.generate_state_arguments(
                        &target_state_name,
                        &state_args,
                        &mut formatted_state_args,
                    );
                }
            }
            StateContextType::StateStackPop {} => {}
        };

        // generate state variables
        let mut formatted_state_vars = String::new();
        let has_state_vars =
            self.generate_state_variables(&target_state_name, &mut formatted_state_vars);

        // generate new state context
        let requires_enter_args = self.requires_enter_arguments(&target_state_name);
        if self.generate_state_context {
            self.generate_next_state_context(
                &target_state_name,
                requires_enter_args,
                has_state_args,
                has_state_vars,
                "None",
                &formatted_state_args,
                &formatted_state_vars,
            );
            self.newline();
            self.add_code(&format!(
                "drop({});",
                self.config.this_state_context_var_name
            ));
        }

        // call the change-state method
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "self.{}({}::{}, next_state_context);",
                self.config.change_state_method_name,
                self.state_enum_type_name(),
                self.format_type_name(&target_state_name.to_string())
            ));
        } else {
            self.add_code(&format!(
                "self.{}({}::{});",
                self.config.change_state_method_name,
                self.state_enum_type_name(),
                self.format_type_name(&target_state_name.to_string())
            ));
        }
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_transition(&mut self, transition_stmt: &TransitionStatementNode) {
        self.newline();
        self.add_code("// Start transition");

        // get the name of the next state
        let target_state_name = match &transition_stmt.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            _ => {
                self.errors.push("Transition target not found.".to_string());
                ""
            }
        };

        // print the transition label, if provided
        match &transition_stmt.label_opt {
            Some(label) => {
                self.newline();
                self.add_code(&format!("// {}", label));
            }
            None => {}
        }

        // generate exit arguments
        let mut has_exit_args = false;
        if let Some(exit_args) = &transition_stmt.exit_args_opt {
            has_exit_args = self.generate_exit_arguments(&target_state_name, &exit_args);
        }
        let exit_args = if has_exit_args {
            format!("Some({})", self.config.exit_args_member_name)
        } else {
            "None".to_string()
        };

        // generate enter arguments
        let mut has_enter_args = false;
        let mut formatted_enter_args = String::from("Some(");
        match &transition_stmt.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                if let Some(enter_args) = &state_context_node.enter_args_opt {
                    has_enter_args = self.generate_enter_arguments(
                        &target_state_name,
                        &enter_args,
                        &mut formatted_enter_args,
                    );
                }
            }
            StateContextType::StateStackPop {} => {}
        };
        formatted_enter_args.push_str(")");

        // generate state arguments
        let mut has_state_args = false;
        let mut formatted_state_args = String::new();
        match &transition_stmt.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                if let Some(state_args) = &state_context_node.state_ref_args_opt {
                    has_state_args = self.generate_state_arguments(
                        &target_state_name,
                        &state_args,
                        &mut formatted_state_args,
                    );
                }
            }
            StateContextType::StateStackPop {} => {}
        };

        // generate state variables
        let mut formatted_state_vars = String::new();
        let has_state_vars =
            self.generate_state_variables(&target_state_name, &mut formatted_state_vars);

        // generate new state context
        if self.generate_state_context {
            self.generate_next_state_context(
                &target_state_name,
                has_enter_args,
                has_state_args,
                has_state_vars,
                &formatted_enter_args,
                &formatted_state_args,
                &formatted_state_vars,
            );
            self.newline();
            self.add_code(&format!(
                "drop({});",
                self.config.this_state_context_var_name
            ));
        }

        // call the transition method
        self.newline();
        if self.generate_state_context {
            if self.generate_exit_args {
                self.add_code(&format!(
                    "self.{}({}, {}::{}, next_state_context);",
                    self.config.transition_method_name,
                    exit_args,
                    self.state_enum_type_name(),
                    self.format_type_name(&target_state_name.to_string())
                ));
            } else {
                self.add_code(&format!(
                    "self.{}({}::{}, next_state_context);",
                    self.config.transition_method_name,
                    self.state_enum_type_name(),
                    self.format_type_name(&target_state_name.to_string())
                ));
            }
        } else {
            if self.generate_exit_args {
                self.add_code(&format!(
                    "self.{}({}, {}::{});",
                    self.config.transition_method_name,
                    exit_args,
                    self.state_enum_type_name(),
                    self.format_type_name(&target_state_name.to_string())
                ));
            } else {
                self.add_code(&format!(
                    "self.{}({}::{});",
                    self.config.transition_method_name,
                    self.state_enum_type_name(),
                    self.format_type_name(&target_state_name.to_string())
                ));
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    // NOTE: Stack pop change-states do not support passing state arguments.
    // It's unclear whether this feature makes sense or how to support it.
    // On a pop transition, the state that is being changed to is not known
    // statically, so the programmer does not know how many arguments to pass.
    fn generate_state_stack_pop_change_state(
        &mut self,
        change_state_stmt: &ChangeStateStatementNode,
    ) {
        self.newline();
        self.add_code("// Start change state");

        // print the change-state label, if provided
        match &change_state_stmt.label_opt {
            Some(label) => {
                self.newline();
                self.add_code(&format!("// {}", label));
            }
            None => {}
        }

        // pop the state/context and pass to change-state method
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "drop({});",
                self.config.this_state_context_var_name
            ));
            self.newline();
            self.add_code(&format!(
                "let (next_state, next_state_context) = self.{}();",
                self.config.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state, next_state_context);",
                self.config.change_state_method_name
            ));
        } else {
            self.add_code(&format!(
                "let next_state = self.{}();",
                self.config.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state);",
                self.config.change_state_method_name
            ));
        }
    }

    //* --------------------------------------------------------------------- *//

    // NOTE: Stack pop transitions do not support passing state or event
    // arguments. It's unclear whether this feature makes sense or how to
    // support it. On a pop transition, the state that is being changed to is
    // not known statically, so the programmer does not know how many arguments
    // to pass. State variables are supported, however.
    fn generate_state_stack_pop_transition(
        &mut self,
        transition_statement: &TransitionStatementNode,
    ) {
        self.newline();
        self.add_code("// Start transition");

        // print the transition label, if provided
        match &transition_statement.label_opt {
            Some(label) => {
                self.newline();
                self.add_code(&format!("// {}", label));
            }
            None => {}
        }

        // pop the state/context and pass to transition method
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "drop({});",
                self.config.this_state_context_var_name
            ));
            self.newline();
            self.add_code(&format!(
                "let (next_state, next_state_context) = self.{}();",
                self.config.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state, next_state_context);",
                self.config.transition_method_name
            ));
        } else {
            self.add_code(&format!(
                "let next_state = self.{}();",
                self.config.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state);",
                self.config.transition_method_name
            ));
        }
    }
}

//* --------------------------------------------------------------------- *//

impl AstVisitor for RustVisitor {
    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) -> AstVisitorReturnType {
        self.system_name = system_node.name.clone();
        self.add_code(&format!("// {}", self.compiler_version));
        self.newline();
        self.add_code(&system_node.header);
        self.newline();
        self.add_code("#[allow(unused_imports)]");
        self.newline();
        self.add_code("use std::cell::RefCell;");
        self.newline();
        self.add_code("use std::collections::HashMap;");
        self.newline();
        self.add_code("#[allow(unused_imports)]");
        self.newline();
        self.add_code("use std::rc::Rc;");
        self.newline();
        if self.config.config_features.runtime_support {
            self.add_code("#[allow(unused_imports)]");
            self.newline();
            self.add_code("use std::any::Any;");
            self.newline();
            self.add_code("#[allow(unused_imports)]");
            self.newline();
            self.add_code("use frame_runtime::environment::{EMPTY, Environment};");
            self.newline();
            self.add_code("#[allow(unused_imports)]");
            self.newline();
            self.add_code("use frame_runtime::state::State;");
            self.newline();
        }

        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            interface_block_node.accept_frame_messages_enum(self);
            interface_block_node.accept_frame_parameters(self);
        }

        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        if !self.config.config_features.follow_rust_naming {
            self.newline();
            self.add_code("#[allow(non_camel_case_types)]");
        }
        self.newline();
        self.add_code(&format!(
            "enum {} {{",
            self.config.frame_event_parameter_type_name
        ));
        self.indent();
        self.newline();
        self.add_code("None,");

        let vec = self.arcanum.get_event_names();
        for unparsed_event_name in vec {
            match self
                .arcanum
                .get_event(&unparsed_event_name, &self.current_state_name_opt)
            {
                Some(event_sym) => {
                    let (_state_name_opt, event_name) =
                        self.parse_event_name(&event_sym.borrow().msg);

                    // enter messages handled in state context struct
                    if !(event_name.eq(&self.config.enter_token)) {
                        match &event_sym.borrow().params_opt {
                            Some(params) => {
                                for param in params {
                                    let param_type = match &param.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => "<?>".to_string().clone(),
                                    };
                                    self.newline();
                                    let parameter_enum_name = self
                                        .format_frame_event_parameter_name(
                                            &event_sym.borrow().msg,
                                            &param.name,
                                        );
                                    self.add_code(&format!(
                                        "{} {{ param: {} }},",
                                        self.format_type_name(&parameter_enum_name),
                                        param_type
                                    ));
                                }
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }

        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        self.newline();
        self.add_code(&format!("enum {} {{", self.config.frame_event_return));
        self.indent();
        self.newline();
        self.add_code("None,");

        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            for interface_method_node in &interface_block_node.interface_methods {
                let if_name = interface_method_node.borrow().name.clone();
                if let Some(return_type) = &interface_method_node.borrow().return_type_opt {
                    self.newline();
                    self.add_code(&format!(
                        "{} {{ return_value: {} }},",
                        self.format_type_name(&if_name),
                        return_type.get_type_str()
                    ));
                }
            }
        }
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        if !self.config.config_features.follow_rust_naming {
            self.newline();
            self.add_code("#[allow(non_snake_case)]");
        }
        self.newline();
        self.add_code(&format!("impl {} {{", self.config.frame_event_return));
        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            for interface_method_node in &interface_block_node.interface_methods {
                //let if_name = interface_method_node.name.clone();
                if let Some(return_type) = &interface_method_node.borrow().return_type_opt {
                    self.indent();
                    self.newline();
                    self.add_code(&format!(
                        "fn {}(&self) -> {} {{",
                        self.format_event_param_getter(&interface_method_node.borrow().name, "ret"),
                        return_type.get_type_str()
                    ));
                    self.indent();
                    self.newline();
                    self.add_code(&format!("match self {{"));
                    self.indent();
                    self.newline();
                    self.add_code(&format!(
                        "{}::{} {{ return_value }} => return_value.clone(),",
                        self.config.frame_event_return,
                        self.format_type_name(&interface_method_node.borrow().name)
                    ));
                    self.newline();
                    self.add_code(&format!("_ => panic!(\"Invalid return value\"),"));
                    self.outdent();
                    self.newline();
                    self.add_code("}");
                    self.outdent();
                    self.newline();
                    self.add_code("}");
                    self.outdent();
                    self.newline();
                }
            }
        }

        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        self.newline();
        self.add_code(&format!(
            "pub struct {} {{",
            self.config.frame_event_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "message: {},",
            self.config.frame_event_message_type_name
        ));
        self.newline();
        self.add_code(&format!(
            "parameters: Option<Box<{}>>,",
            self.config.frame_event_parameters_type_name
        ));
        self.newline();
        self.add_code(&format!("ret: {},", self.config.frame_event_return));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        self.newline();
        self.add_code(&format!("impl {} {{", self.config.frame_event_type_name));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "fn new(message: {}, parameters: Option<Box<{}>>) -> {} {{",
            self.config.frame_event_message_type_name,
            self.config.frame_event_parameters_type_name,
            self.config.frame_event_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!("{} {{", self.config.frame_event_type_name));
        self.indent();
        self.newline();
        self.add_code("message,");
        self.newline();
        self.add_code("parameters,");
        self.newline();
        self.add_code(&format!("ret: {}::None,", self.config.frame_event_return));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.generate_state_enum(&system_node);

        if self.generate_state_context {
            self.newline();
            self.newline();
            self.generate_state_context_defs(&system_node);
        }

        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            self.newline();
            self.newline();
            actions_block_node.accept_rust_trait(self);
        }

        self.newline();
        self.newline();
        self.add_code("// System Controller ");
        self.newline();
        if !self.config.config_features.follow_rust_naming {
            self.newline();
            self.add_code("#[allow(clippy::upper_case_acronyms)]");
        }
        self.newline();
        self.add_code("#[allow(dead_code)]");
        self.newline();
        self.add_code(&format!("pub struct {} {{", self.system_name));
        self.indent();
        self.newline();

        // generate state variable
        self.add_code(&format!(
            "{}: {},",
            &self.config.state_var_name,
            self.state_enum_type_name()
        ));

        // generate state context variable
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "{}: Rc<{}>,",
                self.config.state_context_var_name, self.config.state_context_name
            ));
        }

        // generate state stack variable
        if self.generate_state_stack {
            if self.generate_state_context {
                self.newline();
                self.add_code(&format!(
                    "{}: Vec<({}, Rc<{}>)>,",
                    self.config.state_stack_var_name,
                    self.state_enum_type_name(),
                    self.config.state_context_name
                ));
            } else {
                self.newline();
                self.add_code(&format!(
                    "{}: Vec<{}>,",
                    self.config.state_stack_var_name,
                    self.state_enum_type_name()
                ));
            }
        }

        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            domain_block_node.accept(self);
        }

        self.outdent();
        self.newline();

        self.add_code("}");

        self.newline();
        self.newline();

        self.add_code("#[allow(dead_code)]");
        self.newline();
        if !self.config.config_features.follow_rust_naming {
            self.add_code("#[allow(non_snake_case)]");
            self.newline();
        }
        self.add_code(&format!("impl {} {{", system_node.name));
        self.indent();

        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        match (&system_node).get_first_state() {
            Some(x) => {
                self.first_state_name = x.borrow().name.clone();
                self.has_states = true;
            }
            None => {}
        }

        if self.has_states {
            self.newline();
            self.newline();
            self.generate_constructor(&system_node);
            self.newline();
            self.newline();
            self.generate_initialize();
        }

        self.serialize.push("".to_string());
        self.serialize.push("Bag _serialize__do() {".to_string());

        self.deserialize.push("".to_string());

        // @TODO: _do needs to be configurable.
        self.deserialize
            .push("void _deserialize__do(Bag data) {".to_string());

        // self.subclass_code.push("".to_string());
        // self.subclass_code.push("/********************\n".to_string());
        // self.subclass_code.push(format!("public partial class {}Controller : {} {{",system_node.name,system_node.name));

        self.newline();
        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            interface_block_node.accept(self);
        }

        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }

        // self.subclass_code.push(format!("}}"));
        // self.subclass_code.push("\n********************/".to_string());

        // self.serialize.push("".to_string());
        // self.serialize.push("\treturn JSON.stringify(bag);".to_string());
        // self.serialize.push("}".to_string());
        // self.serialize.push("".to_string());
        //
        // self.deserialize.push("".to_string());
        // self.deserialize.push("}".to_string());

        if self.has_states {
            self.generate_machinery(system_node);
        }

        // TODO: add comments back
        // self.newline();
        // self.generate_comment(system_node.line);
        // self.newline();
        self.outdent();
        self.newline();
        self.add_code("} // end system controller");
        self.newline();

        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            actions_block_node.accept_rust_impl(self);
        }

        // self.generate_subclass();

        AstVisitorReturnType::SystemNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_messages_enum(
        &mut self,
        _interface_block_node: &InterfaceBlockNode,
    ) -> AstVisitorReturnType {
        self.newline();
        self.add_code("#[allow(dead_code)]");
        self.newline();
        self.add_code(&format!(
            "enum {} {{",
            self.config.frame_event_message_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!("{},", self.config.enter_msg));
        self.newline();
        self.add_code(&format!("{},", self.config.exit_msg));

        let events = self.arcanum.get_event_names();
        for event in &events {
            //    ret.push(k.clone());
            if self.is_enter_or_exit_message(&event) {
                continue;
            }
            let message_opt = self.arcanum.get_interface_or_msg_from_msg(&event);
            match message_opt {
                Some(canonical_message_name) => {
                    self.newline();
                    self.add_code(&format!(
                        "{},",
                        self.format_type_name(&canonical_message_name)
                    ));
                }
                None => {
                    self.newline();
                    self.add_code(&format!("<Error - unknown message {}>,", &event));
                }
            }
        }

        self.outdent();
        self.newline();
        self.add_code("}");

        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        self.newline();
        self.add_code(&format!(
            "impl std::fmt::Display for {} {{",
            self.config.frame_event_message_type_name
        ));
        self.indent();
        self.newline();
        self.add_code("fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {");
        self.indent();
        self.newline();
        self.add_code("match self {");
        self.indent();
        self.newline();
        self.add_code(&format!(
            "{}::{} => write!(f, \"{}\"),",
            self.config.frame_event_message_type_name, self.config.enter_msg, self.config.enter_msg
        ));
        self.newline();
        self.add_code(&format!(
            "{}::{} => write!(f, \"{}\"),",
            self.config.frame_event_message_type_name, self.config.exit_msg, self.config.exit_msg
        ));
        for event in &events {
            //    ret.push(k.clone());
            if self.is_enter_or_exit_message(&event) {
                continue;
            }
            let message_opt = self.arcanum.get_interface_or_msg_from_msg(&event);
            match message_opt {
                Some(canonical_message_name) => {
                    let formatted_message_name = self.format_type_name(&canonical_message_name);
                    self.newline();
                    self.add_code(&format!(
                        "{}::{} => write!(f, \"{}\"),",
                        self.config.frame_event_message_type_name,
                        formatted_message_name,
                        formatted_message_name
                    ));
                }
                None => {
                    self.newline();
                    self.add_code(&format!("<Error - unknown message {}>,", &event));
                }
            }
        }
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");

        AstVisitorReturnType::InterfaceBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_parameters(
        &mut self,
        _interface_block_node: &InterfaceBlockNode,
    ) -> AstVisitorReturnType {
        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        self.newline();
        self.add_code(&format!(
            "struct {} {{",
            self.config.frame_event_parameters_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "parameters: HashMap<String, {}>,",
            self.config.frame_event_parameter_type_name
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.add_code("#[allow(dead_code)]");
        if !self.config.config_features.follow_rust_naming {
            self.newline();
            self.add_code("#[allow(non_snake_case)]");
        }
        self.newline();
        self.add_code(&format!(
            "impl {} {{",
            self.config.frame_event_parameters_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "fn new() -> {} {{",
            self.config.frame_event_parameters_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "{} {{",
            self.config.frame_event_parameters_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "{}: HashMap::new(),",
            self.config.frame_event_parameters_attribute_name
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.outdent();
        self.newline();
        self.add_code("}");

        let vec = self.arcanum.get_event_names();
        for unparsed_event_name in vec {
            match self
                .arcanum
                .get_event(&unparsed_event_name, &self.current_state_name_opt)
            {
                Some(event_sym) => {
                    let (_state_name_opt, event_name) =
                        self.parse_event_name(&event_sym.borrow().msg);
                    if !(event_name.eq(&self.config.enter_token)) {
                        match &event_sym.borrow().params_opt {
                            Some(params) => {
                                for param in params {
                                    let param_type = match &param.param_type_opt {
                                        Some(param_type) => param_type.get_type_str(),
                                        None => "<?>".to_string().clone(),
                                    };
                                    let param_name = self.format_value_name(&param.name);
                                    self.newline();
                                    self.newline();
                                    let parameter_enum_name = self
                                        .format_frame_event_parameter_name(
                                            &unparsed_event_name,
                                            &param.name,
                                        );
                                    self.add_code(&format!(
                                        "fn {}(&mut self, {}: {}) {{",
                                        self.format_setter_name(&parameter_enum_name),
                                        param_name,
                                        param_type
                                    ));
                                    self.indent();
                                    self.newline();
                                    self.add_code(&format!(
                                        "self.{}.insert(",
                                        self.config.frame_event_parameters_attribute_name
                                    ));
                                    self.indent();
                                    self.newline();
                                    self.add_code(&format!(
                                        "String::from(\"{}\"),",
                                        parameter_enum_name
                                    ));
                                    self.newline();
                                    self.add_code(&format!(
                                        "{}::{} {{ param: {} }},",
                                        self.config.frame_event_parameter_type_name,
                                        self.format_type_name(&parameter_enum_name),
                                        param_name
                                    ));
                                    self.outdent();
                                    self.newline();
                                    self.add_code(&format!(");"));
                                    self.outdent();
                                    self.newline();
                                    self.add_code("}");
                                    self.newline();
                                    self.newline();
                                    self.add_code(&format!(
                                        "fn {}(&self) -> {} {{",
                                        self.format_getter_name(&parameter_enum_name),
                                        param_type
                                    ));
                                    self.indent();
                                    self.newline();
                                    self.add_code(&format!(
                                        "match self.{}.get(\"{}\") {{",
                                        self.config.frame_event_parameters_attribute_name,
                                        parameter_enum_name
                                    ));
                                    self.indent();
                                    self.newline();
                                    self.add_code("Some(parameter) => match parameter {");
                                    self.indent();
                                    self.newline();

                                    // let parameter_enum_name = self.format_frame_event_parameter_name(&parameter_enum_name
                                    //                                                                  ,&param.name);

                                    self.add_code(&format!(
                                        "{}::{} {{ param }} => param.clone(),",
                                        self.config.frame_event_parameter_type_name,
                                        self.format_type_name(&parameter_enum_name)
                                    ));
                                    self.newline();
                                    self.add_code("_ => panic!(\"Invalid parameter\"),");
                                    // self.outdent();
                                    // self.newline();
                                    // self.add_code("}"); // match self.parameters.get
                                    self.outdent();
                                    self.newline();
                                    self.add_code("},"); // Some(parameter)
                                    self.newline();
                                    self.add_code("None => panic!(\"Invalid parameter\"),");
                                    // self.outdent();
                                    // self.newline();
                                    self.outdent();
                                    self.newline();
                                    self.add_code("}");
                                    self.outdent();
                                    self.newline();
                                    self.add_code("}");
                                }
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
            // self.newline();
            // self.add_code(&format!("{},",x));
        }

        self.outdent();
        self.newline();
        self.add_code("}");

        // for interface_method_node in &interface_block_node.interface_methods {
        //     interface_method_node.accept(self);
        // }

        AstVisitorReturnType::InterfaceBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
    ) -> AstVisitorReturnType {
        self.add_code(&format!(
            "self.{}",
            interface_method_call_expr_node.identifier.name.lexeme
        ));
        interface_method_call_expr_node.call_expr_list.accept(self);
        //        self.add_code(&format!(""));
        // TODO: review this return as I think it is a nop.
        AstVisitorReturnType::InterfaceMethodCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node_to_string(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        output.push_str(&format!(
            "self.{}",
            interface_method_call_expr_node.identifier.name.lexeme
        ));
        interface_method_call_expr_node
            .call_expr_list
            .accept_to_string(self, output);
        //        self.add_code(&format!(""));

        // TODO: review this return as I think it is a nop.
        AstVisitorReturnType::InterfaceMethodCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_block_node(
        &mut self,
        interface_block_node: &InterfaceBlockNode,
    ) -> AstVisitorReturnType {
        self.newline();
        self.add_code("//===================== Interface Block ===================//");
        self.newline();

        for interface_method_node_rcref in &interface_block_node.interface_methods {
            let interface_method_node = interface_method_node_rcref.borrow();
            interface_method_node.accept(self);
        }
        AstVisitorReturnType::InterfaceBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_node(
        &mut self,
        interface_method_node: &InterfaceMethodNode,
    ) -> AstVisitorReturnType {
        self.newline();
        self.add_code(&format!(
            "pub fn {}(&mut self",
            self.format_value_name(&interface_method_node.name)
        ));

        match &interface_method_node.params {
            Some(params) => self.format_parameter_list(params).clone(),
            None => {}
        }

        self.add_code(")");
        match &interface_method_node.return_type_opt {
            Some(return_type) => {
                self.add_code(&format!(" -> {}", return_type.get_type_str()));
            }
            None => {}
        }
        self.add_code(" {");
        self.indent();
        let params_param_code;

        if interface_method_node.params.is_some() {
            params_param_code = String::from("Some(frame_parameters)");
            self.newline();
            self.add_code(&format!(
                "let mut frame_parameters = Box::new({}::new());",
                self.config.frame_event_parameters_type_name
            ));
            match &interface_method_node.params {
                Some(params) => {
                    for param in params {
                        let msg = self
                            .arcanum
                            .get_msg_from_interface_name(&interface_method_node.name);

                        let parameter_enum_name =
                            self.format_frame_event_parameter_name(&msg, &param.param_name);
                        self.newline();
                        self.add_code(&format!(
                            "(*frame_parameters).{}({});",
                            self.format_setter_name(&parameter_enum_name),
                            self.format_value_name(&param.param_name)
                        ));
                    }
                }
                None => {}
            }
        } else {
            params_param_code = String::from("None");
        }

        // self.indent();
        self.newline();
        self.add_code(&format!(
            "let mut {} = {}::new({}::{}, {});",
            self.config.frame_event_variable_name,
            self.config.frame_event_type_name,
            self.config.frame_event_message_type_name,
            self.format_type_name(&interface_method_node.name),
            &params_param_code
        ));
        // self.indent();
        // self.newline();
        // self.add_code(&format!("message : String::from(\"{}\"),", method_name_or_alias));
        // self.outdent();
        // self.newline();
        // self.add_code("};");
        self.newline();
        self.add_code(&format!(
            "self.{}(&mut {});",
            self.config.handle_event_method_name, self.config.frame_event_variable_name,
        ));

        match &interface_method_node.return_type_opt {
            Some(_return_type) => {
                self.newline();
                self.add_code(&format!(
                    "match {}.{} {{",
                    self.config.frame_event_variable_name,
                    self.config.frame_event_return_attribute_name
                ));
                self.indent();
                self.newline();
                self.add_code(&format!(
                    "{}::{} {{ return_value }} => return_value.clone(),",
                    self.config.frame_event_return,
                    self.format_type_name(&interface_method_node.name)
                ));
                self.newline();
                self.add_code(&format!(
                    "_ => panic!(\"Bad return value for {}\"),",
                    &interface_method_node.name
                ));
                self.outdent();
                self.newline();
                self.add_code("}");
            }
            None => {}
        }

        self.outdent();
        self.newline();
        self.add_code(&format!("}}"));
        self.newline();

        AstVisitorReturnType::InterfaceMethodNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_machine_block_node(
        &mut self,
        machine_block_node: &MachineBlockNode,
    ) -> AstVisitorReturnType {
        self.newline();
        self.add_code("//===================== Machine Block ===================//");

        // NOTE: this is initial work on a standardized persistence approach
        // self.serialize.push("".to_string());
        // self.serialize.push("\tvar stateName = null;".to_string());
        //
        // self.deserialize.push("".to_string());
        // self.deserialize.push("\tconst bag = JSON.parse(data);".to_string());
        // self.deserialize.push("".to_string());
        // self.deserialize.push("\tswitch (bag.state) {".to_string());

        for state_node_rcref in &machine_block_node.states {
            state_node_rcref.borrow().accept(self);
        }

        // self.serialize.push("".to_string());
        // self.serialize.push("\tvar bag = {".to_string());
        // self.serialize.push("\t\tstate : stateName,".to_string());
        // self.serialize.push("\t\tdomain : {}".to_string());
        // self.serialize.push("\t};".to_string());
        // self.serialize.push("".to_string());

        self.deserialize.push("\t}".to_string());
        self.deserialize.push("".to_string());

        AstVisitorReturnType::MachineBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_block_node(&mut self, _: &ActionsBlockNode) -> AstVisitorReturnType {
        panic!("Error - visit_actions_block_node() not called for Rust.");

        // AstVisitorReturnType::ActionBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_node_rust_trait(
        &mut self,
        actions_block_node: &ActionsBlockNode,
    ) -> AstVisitorReturnType {
        if self.config.config_features.generate_action_impl {
            self.add_code(&format!(
                "trait {}{}{} {{ ",
                self.config.actions_prefix, self.system_name, self.config.actions_suffix
            ));
            self.indent();

            // add action signatures
            for action_decl_node_rcref in &actions_block_node.actions {
                let action_decl_node = action_decl_node_rcref.borrow();
                action_decl_node.accept(self);
            }

            self.outdent();
            self.newline();
            self.add_code("}");
        }

        AstVisitorReturnType::ActionBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_node_rust_impl(
        &mut self,
        actions_block_node: &ActionsBlockNode,
    ) -> AstVisitorReturnType {
        if self.config.config_features.generate_action_impl {
            self.newline();
            self.add_code("#[allow(unused_variables)]");
            self.newline();
            self.add_code(&format!(
                "impl {}{}{} for {} {{ ",
                self.config.actions_prefix,
                self.system_name,
                self.config.actions_suffix,
                self.system_name
            ));
            self.indent();

            // add action implementations
            for action_decl_node_rcref in &actions_block_node.actions {
                let action_decl_node = action_decl_node_rcref.borrow();
                action_decl_node.accept_rust_impl(self);
            }

            self.outdent();
            self.newline();
            self.add_code("}");
        }

        AstVisitorReturnType::ActionBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_block_node(
        &mut self,
        domain_block_node: &DomainBlockNode,
    ) -> AstVisitorReturnType {
        self.newline();
        self.newline();
        self.add_code("//===================== Domain Block ===================//");
        self.newline();

        for variable_decl_node_rcref in &domain_block_node.member_variables {
            let variable_decl_node = variable_decl_node_rcref.borrow();
            variable_decl_node.accept_rust_domain_var_decl(self);
        }

        AstVisitorReturnType::DomainBlockNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_node(&mut self, state_node: &StateNode) -> AstVisitorReturnType {
        self.generate_comment(state_node.line);
        self.current_state_name_opt = Some(state_node.name.clone());
        self.newline();
        self.newline();

        self.add_code("#[allow(clippy::needless_return)]");
        self.newline();
        self.add_code("#[allow(unreachable_code)]");
        self.newline();
        self.add_code("#[allow(unreachable_patterns)]");
        self.newline();
        self.add_code("#[allow(unused_mut)]");
        self.newline();
        self.add_code("#[allow(unused_parens)]");
        self.newline();
        self.add_code("#[allow(unused_variables)]");
        self.newline();
        self.add_code(&format!(
            "fn {}(&mut self, {}: &mut {}) {{",
            self.format_state_handler_name(&state_node.name),
            self.config.frame_event_variable_name,
            self.config.frame_event_type_name
        ));
        self.indent();
        let state_name = &self.current_state_name_opt.as_ref().unwrap().clone();
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "let {0}_clone = Rc::clone(&self.{0});",
                self.config.state_context_var_name
            ));
            self.newline();
            self.add_code(&format!(
                "let mut {} = {}_clone.{}().borrow_mut();",
                self.config.this_state_context_var_name,
                self.config.state_context_var_name,
                self.format_state_context_method_name(&state_name)
            ));
        }

        // @TODO
        // self.serialize.push(format!("\tif (_state_ == _s{}_) stateName = \"{}\"",state_node.name,state_node.name));
        //
        // self.deserialize.push(format!("\t\tcase \"{}\": _state_ = _s{}_; break;",state_node.name,state_node.name));

        // this allows for logging and other kinds of calls for each event in the state
        if let Some(calls) = &state_node.calls_opt {
            for call in calls {
                self.newline();
                call.accept(self);
                self.add_code(";");
            }
            self.newline();
        }

        self.newline();
        self.add_code(&format!(
            "match {}.{} {{",
            self.config.frame_event_variable_name, self.config.frame_event_message_attribute_name
        ));
        self.indent();

        if state_node.evt_handlers_rcref.len() > 0 {
            for evt_handler_node in &state_node.evt_handlers_rcref {
                evt_handler_node.as_ref().borrow().accept(self);
            }
        }
        self.newline();
        self.add_code("_ => {}");
        self.outdent();
        self.newline();
        self.add_code("}");

        // generate call to parent handler, if applicable
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
        AstVisitorReturnType::StateNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_node(
        &mut self,
        evt_handler_node: &EventHandlerNode,
    ) -> AstVisitorReturnType {
        self.current_event_ret_type = evt_handler_node.get_event_ret_type();
        self.newline();
        self.generate_comment(evt_handler_node.line);
        //        let mut generate_final_close_paren = true;
        if let MessageType::CustomMessage { message_node } = &evt_handler_node.msg_t {
            self.current_message = message_node.name.clone();
            self.add_code(&format!(
                "{}::{} => {{",
                self.config.frame_event_message_type_name,
                self.get_msg_enum(&message_node.name)
            ));
        } else {
            // AnyMessage ( ||* )
            // This feature requires dynamic dispatch.
            panic!("||* not supported for Rust.");
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
        self.newline();
        self.add_code(&format!("}}"));

        // this controls formatting here
        self.current_message = String::new();
        self.current_event_ret_type = String::new();

        AstVisitorReturnType::EventHandlerNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_terminator_node(
        &mut self,
        evt_handler_terminator_node: &TerminatorExpr,
    ) -> AstVisitorReturnType {
        match &evt_handler_terminator_node.terminator_type {
            TerminatorType::Return => {
                match &evt_handler_terminator_node.return_expr_t_opt {
                    Some(expr_t) => {
                        self.newline();
                        self.add_code(&format!(
                            "{}.{} = ",
                            self.config.frame_event_variable_name,
                            self.config.frame_event_return_attribute_name
                        ));
                        self.add_code(&format!(
                            "{}::{} {{ return_value: ",
                            self.config.frame_event_return,
                            self.format_type_name(&self.current_message)
                        ));
                        expr_t.accept(self);

                        self.add_code(" };");
                    }
                    None => {}
                };
                self.generate_return();
            }
            TerminatorType::Continue => {
                self.generate_return_if_transitioned();
            }
        }
        AstVisitorReturnType::EventHandlerTerminatorNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_statement_node(
        &mut self,
        method_call_statement: &CallStmtNode,
    ) -> AstVisitorReturnType {
        self.newline();
        method_call_statement.call_expr_node.accept(self);
        self.add_code(&format!(";"));

        AstVisitorReturnType::CallStatementNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node(&mut self, method_call: &CallExprNode) -> AstVisitorReturnType {
        if let Some(call_chain) = &method_call.call_chain {
            for callable in call_chain {
                callable.callable_accept(self);
                self.add_code(&format!("."));
            }
        }

        self.add_code(&format!("{}", method_call.identifier.name.lexeme));

        method_call.call_expr_list.accept(self);

        self.add_code(&format!(""));

        AstVisitorReturnType::CallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node_to_string(
        &mut self,
        method_call: &CallExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        if let Some(call_chain) = &method_call.call_chain {
            for callable in call_chain {
                callable.callable_accept(self);
                output.push_str(&format!("."));
            }
        }

        output.push_str(&format!("{}", method_call.identifier.name.lexeme));

        method_call.call_expr_list.accept_to_string(self, output);

        output.push_str(&format!(""));

        AstVisitorReturnType::CallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node(
        &mut self,
        call_expr_list: &CallExprListNode,
    ) -> AstVisitorReturnType {
        let mut separator = "";
        self.add_code(&format!("("));

        for expr in &call_expr_list.exprs_t {
            self.add_code(&format!("{}", separator));
            expr.accept(self);
            separator = ",";
        }

        self.add_code(&format!(")"));

        AstVisitorReturnType::CallExprListNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node_to_string(
        &mut self,
        call_expr_list: &CallExprListNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        let mut separator = "";
        output.push_str(&format!("("));

        for expr in &call_expr_list.exprs_t {
            output.push_str(&format!("{}", separator));
            expr.accept_to_string(self, output);
            separator = ",";
        }

        output.push_str(&format!(")"));

        AstVisitorReturnType::CallExprListNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node(
        &mut self,
        action_call: &ActionCallExprNode,
    ) -> AstVisitorReturnType {
        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        self.add_code(&format!("self.{}", action_name));
        action_call.call_expr_list.accept(self);

        AstVisitorReturnType::ActionCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(
        &mut self,
        action_call: &ActionCallExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        output.push_str(&format!("self.{}", action_name));
        action_call.call_expr_list.accept_to_string(self, output);

        AstVisitorReturnType::ActionCallExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(
        &mut self,
        action_call_stmt_node: &ActionCallStmtNode,
    ) -> AstVisitorReturnType {
        self.newline();
        action_call_stmt_node.action_call_expr_node.accept(self);
        self.add_code(&format!(";"));

        AstVisitorReturnType::ActionCallStatementNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_transition_statement_node(
        &mut self,
        transition_statement: &TransitionStatementNode,
    ) -> AstVisitorReturnType {
        match &transition_statement.target_state_context_t {
            StateContextType::StateRef { .. } => {
                self.generate_state_ref_transition(transition_statement)
            }
            StateContextType::StateStackPop {} => {
                self.generate_state_stack_pop_transition(transition_statement)
            }
        };
        self.this_branch_transitioned = true;

        AstVisitorReturnType::CallStatementNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_ref_node(&mut self, state_ref: &StateRefNode) -> AstVisitorReturnType {
        self.add_code(&format!("{}", state_ref.name));

        AstVisitorReturnType::StateRefNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_change_state_statement_node(
        &mut self,
        change_state_stmt_node: &ChangeStateStatementNode,
    ) -> AstVisitorReturnType {
        match &change_state_stmt_node.state_context_t {
            StateContextType::StateRef { .. } => {
                self.generate_state_ref_change_state(change_state_stmt_node)
            }
            StateContextType::StateStackPop {} => {
                self.generate_state_stack_pop_change_state(change_state_stmt_node)
            }
        };
        self.this_branch_transitioned = true;

        AstVisitorReturnType::ChangeStateStmtNode {}
    }

    //* --------------------------------------------------------------------- *//

    // TODO: ??
    fn visit_parameter_node(&mut self, _: &ParameterNode) -> AstVisitorReturnType {
        // self.add_code(&format!("{}",parameter_node.name));

        AstVisitorReturnType::ParameterNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_dispatch_node(&mut self, dispatch_node: &DispatchNode) -> AstVisitorReturnType {
        self.newline();
        self.add_code(&format!(
            "self.{}({});",
            self.format_state_handler_name(&dispatch_node.target_state_ref.name),
            self.config.frame_event_variable_name
        ));
        self.generate_comment(dispatch_node.line);
        AstVisitorReturnType::DispatchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_test_statement_node(
        &mut self,
        test_stmt_node: &TestStatementNode,
    ) -> AstVisitorReturnType {
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

        AstVisitorReturnType::TestStatementNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_node(&mut self, bool_test_node: &BoolTestNode) -> AstVisitorReturnType {
        let mut if_or_else_if = "if ";

        self.newline();
        for branch_node in &bool_test_node.conditional_branch_nodes {
            if branch_node.is_negated {
                self.add_code(&format!("{}!(", if_or_else_if));
            } else {
                self.add_code(&format!("{}", if_or_else_if));
            }

            branch_node.expr_t.accept(self);

            if branch_node.is_negated {
                self.add_code(&format!(")"));
            }
            self.add_code(&format!(" {{"));
            self.indent();

            branch_node.accept(self);

            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code(&format!("}}"));

            if_or_else_if = " else if ";
        }

        // (':' bool_test_else_branch)?
        if let Some(bool_test_else_branch_node) = &bool_test_node.else_branch_node_opt {
            bool_test_else_branch_node.accept(self);
        }

        AstVisitorReturnType::BoolTestNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_statement_node(
        &mut self,
        method_call_chain_literal_stmt_node: &CallChainLiteralStmtNode,
    ) -> AstVisitorReturnType {
        self.newline();
        method_call_chain_literal_stmt_node
            .call_chain_literal_expr_node
            .accept(self);
        self.add_code(&format!(";"));
        AstVisitorReturnType::CallChainLiteralStmtNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node(
        &mut self,
        method_call_chain_expression_node: &CallChainLiteralExprNode,
    ) -> AstVisitorReturnType {
        // TODO: maybe put this in an AST node

        let mut separator = "";

        for node in &method_call_chain_expression_node.call_chain {
            self.add_code(&format!("{}", separator));
            match &node {
                CallChainLiteralNodeType::IdentifierNodeT { id_node } => {
                    id_node.accept(self);
                }
                CallChainLiteralNodeType::CallT { call } => {
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

        AstVisitorReturnType::CallChainLiteralExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node_to_string(
        &mut self,
        method_call_chain_expression_node: &CallChainLiteralExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        let mut separator = "";

        for node in &method_call_chain_expression_node.call_chain {
            output.push_str(&format!("{}", separator));
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
        AstVisitorReturnType::CallChainLiteralExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_conditional_branch_node(
        &mut self,
        bool_test_true_branch_node: &BoolTestConditionalBranchNode,
    ) -> AstVisitorReturnType {
        self.visit_decl_stmts(&bool_test_true_branch_node.statements);

        match &bool_test_true_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&format!("e._return = "));
                                expr_t.accept(self);
                                self.add_code(";");
                            }
                            None => {}
                        };
                        self.generate_return();
                    }
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }

        AstVisitorReturnType::BoolTestConditionalBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_else_branch_node(
        &mut self,
        bool_test_else_branch_node: &BoolTestElseBranchNode,
    ) -> AstVisitorReturnType {
        self.add_code(&format!(" else {{"));
        self.indent();

        self.visit_decl_stmts(&bool_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &bool_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code(&format!("e._return = ",));
                            expr_t.accept(self);
                            self.add_code(";");
                            self.generate_return();
                        }
                        None => {
                            self.generate_return();
                        }
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
        self.add_code(&format!("}}"));

        AstVisitorReturnType::BoolTestElseBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_node(
        &mut self,
        string_match_test_node: &StringMatchTestNode,
    ) -> AstVisitorReturnType {
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
                            .push(format!("Error - expression list is not testable."));
                    }
                    let x = expr_list_node.exprs_t.first().unwrap();
                    x.accept(self);
                }

                _ => self.errors.push(format!("TODO")),
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
                    self.add_code(&format!(".eq(\"{}\")", match_string));
                    first_match = false;
                } else {
                    self.add_code(&format!(" || "));
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
                        _ => self.errors.push(format!("TODO")),
                    }
                    self.add_code(&format!(".eq(\"{}\")", match_string));
                }
            }
            self.add_code(&format!(" {{"));
            self.indent();

            match_branch_node.accept(self);

            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code(&format!("}}"));

            if_or_else_if = " else if";
        }

        // (':' string_test_else_branch)?
        if let Some(string_match_else_branch_node) = &string_match_test_node.else_branch_node_opt {
            string_match_else_branch_node.accept(self);
        }

        AstVisitorReturnType::StringMatchTestNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_match_branch_node(
        &mut self,
        string_match_test_match_branch_node: &StringMatchTestMatchBranchNode,
    ) -> AstVisitorReturnType {
        self.visit_decl_stmts(&string_match_test_match_branch_node.statements);

        match &string_match_test_match_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&format!("e._return = "));
                                expr_t.accept(self);
                                self.add_code(";");
                            }
                            None => {}
                        };
                        self.generate_return();
                    }
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }

        AstVisitorReturnType::StringMatchTestMatchBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_else_branch_node(
        &mut self,
        string_match_test_else_branch_node: &StringMatchTestElseBranchNode,
    ) -> AstVisitorReturnType {
        self.add_code(&format!(" else {{"));
        self.indent();

        self.visit_decl_stmts(&string_match_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &string_match_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&format!("e._return = "));
                                expr_t.accept(self);
                                self.add_code(";");
                            }
                            None => {}
                        };
                        self.generate_return();
                    }
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
        self.add_code(&format!("}}"));

        AstVisitorReturnType::StringMatchElseBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_pattern_node(
        &mut self,
        _string_match_test_else_branch_node: &StringMatchTestPatternNode,
    ) -> AstVisitorReturnType {
        // TODO
        self.errors.push(format!("Not implemented."));
        AstVisitorReturnType::StringMatchTestPatternNode {}
    }

    //-----------------------------------------------------//

    fn visit_number_match_test_node(
        &mut self,
        number_match_test_node: &NumberMatchTestNode,
    ) -> AstVisitorReturnType {
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
                            .push(format!("Error - expression list is not testable."));
                    }
                    let x = expr_list_node.exprs_t.first().unwrap();
                    x.accept(self);
                }
                _ => self.errors.push(format!("TODO.")),
            }

            let mut first_match = true;
            for match_number in &match_branch_node.number_match_pattern_nodes {
                if first_match {
                    self.add_code(&format!(" == {}", match_number.match_pattern_number));
                    first_match = false;
                } else {
                    self.add_code(&format!(" || "));
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
                        _ => self.errors.push(format!("TODO.")),
                    }
                    self.add_code(&format!(" == {}", match_number.match_pattern_number));
                }
            }

            self.add_code(&format!(" {{"));
            self.indent();

            match_branch_node.accept(self);

            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code(&format!("}}"));

            if_or_else_if = " else if";
        }

        // (':' number_test_else_branch)?
        if let Some(number_match_else_branch_node) = &number_match_test_node.else_branch_node_opt {
            number_match_else_branch_node.accept(self);
        }

        AstVisitorReturnType::NumberMatchTestNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_match_branch_node(
        &mut self,
        number_match_test_match_branch_node: &NumberMatchTestMatchBranchNode,
    ) -> AstVisitorReturnType {
        self.visit_decl_stmts(&number_match_test_match_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &number_match_test_match_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&format!("e._return = "));
                                expr_t.accept(self);
                                self.add_code(";");
                            }
                            None => {}
                        };
                        self.generate_return();
                    }
                    TerminatorType::Continue => {
                        self.generate_return_if_transitioned();
                    }
                }
            }
            None => {
                self.generate_return_if_transitioned();
            }
        }

        AstVisitorReturnType::NumberMatchTestMatchBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_else_branch_node(
        &mut self,
        number_match_test_else_branch_node: &NumberMatchTestElseBranchNode,
    ) -> AstVisitorReturnType {
        self.add_code(&format!(" else {{"));
        self.indent();

        self.visit_decl_stmts(&number_match_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &number_match_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&format!("e._return = "));
                                expr_t.accept(self);
                                self.add_code(";");
                            }
                            None => {}
                        };
                        self.generate_return();
                    }
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
        self.add_code(&format!("}}"));

        AstVisitorReturnType::NumberMatchElseBranchNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_pattern_node(
        &mut self,
        match_pattern_node: &NumberMatchTestPatternNode,
    ) -> AstVisitorReturnType {
        self.add_code(&format!("{}", match_pattern_node.match_pattern_number));

        AstVisitorReturnType::NumberMatchTestPatternNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node(&mut self, expr_list: &ExprListNode) -> AstVisitorReturnType {
        let mut separator = "";
        self.add_code(&format!("("));
        for expr in &expr_list.exprs_t {
            self.add_code(&format!("{}", separator));
            expr.accept(self);
            separator = ",";
        }
        self.add_code(&format!(")"));

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node_to_string(
        &mut self,
        expr_list: &ExprListNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        //        self.add_code(&format!("{}(e);\n",dispatch_node.target_state_ref.name));

        let mut separator = "";
        output.push_str(&format!("("));
        for expr in &expr_list.exprs_t {
            output.push_str(&format!("{}", separator));
            expr.accept_to_string(self, output);
            separator = ",";
        }
        output.push_str(&format!(")"));

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node(
        &mut self,
        literal_expression_node: &LiteralExprNode,
    ) -> AstVisitorReturnType {
        match &literal_expression_node.token_t {
            TokenType::NumberTok => self.add_code(&format!("{}", literal_expression_node.value)),
            TokenType::SuperStringTok => {
                self.add_code(&format!("{}", literal_expression_node.value))
            }
            TokenType::StringTok => {
                // if literal_expression_node. {
                //     code.push_str("&");
                // }
                if literal_expression_node.is_reference {
                    self.add_code("&");
                }
                self.add_code(&format!(
                    "String::from(\"{}\")",
                    literal_expression_node.value
                ));
            }
            TokenType::TrueTok => self.add_code("true"),
            TokenType::FalseTok => self.add_code("false"),
            TokenType::NullTok => self.add_code("null"),
            TokenType::NilTok => self.add_code("null"),
            // TokenType::SuperStringTok => {
            //     self.add_code(&format!("{}", literal_expression_node.value));
            // },
            _ => self
                .errors
                .push(format!("TODO: visit_literal_expression_node")),
        }

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node_to_string(
        &mut self,
        literal_expression_node: &LiteralExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        // TODO: make a focused enum or the literals
        match &literal_expression_node.token_t {
            TokenType::NumberTok => output.push_str(&format!("{}", literal_expression_node.value)),
            TokenType::StringTok => {
                output.push_str(&format!(
                    "String::from(\"{}\")",
                    literal_expression_node.value
                ));
            }
            TokenType::TrueTok => {
                output.push_str("true");
            }
            TokenType::FalseTok => {
                output.push_str("false");
            }
            TokenType::NilTok => {
                output.push_str("null");
            }
            TokenType::NullTok => {
                output.push_str("null");
            }
            TokenType::SuperStringTok => {
                output.push_str(&format!("{}", literal_expression_node.value));
            }
            _ => self
                .errors
                .push(format!("TODO: visit_literal_expression_node_to_string")),
        }

        AstVisitorReturnType::ParentheticalExpressionNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node(&mut self, identifier_node: &IdentifierNode) -> AstVisitorReturnType {
        self.add_code(&format!("{}", identifier_node.name.lexeme));

        AstVisitorReturnType::IdentifierNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node_to_string(
        &mut self,
        identifier_node: &IdentifierNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        output.push_str(&format!("{}", identifier_node.name.lexeme));

        AstVisitorReturnType::IdentifierNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node(
        &mut self,
        _state_stack_operation_node: &StateStackOperationNode,
    ) -> AstVisitorReturnType {
        //        self.add_code(&format!("{}",identifier_node.name.lexeme));

        AstVisitorReturnType::StateStackOperationNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_node_to_string(
        &mut self,
        _state_stack_operation_node: &StateStackOperationNode,
        _output: &mut String,
    ) -> AstVisitorReturnType {
        //        self.add_code(&format!("{}",identifier_node.name.lexeme));

        AstVisitorReturnType::StateStackOperationNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_stack_operation_statement_node(
        &mut self,
        state_stack_op_statement_node: &StateStackOperationStatementNode,
    ) -> AstVisitorReturnType {
        match state_stack_op_statement_node
            .state_stack_operation_node
            .operation_t
        {
            StateStackOperationType::Push => {
                self.newline();
                self.add_code(&format!(
                    "self.{}();",
                    self.config.state_stack_push_method_name
                ));
            }
            StateStackOperationType::Pop => {
                if self.generate_state_context {
                    self.add_code(&format!(
                        "let {} = self.{}();",
                        self.config.state_context_var_name, self.config.state_stack_pop_method_name
                    ));
                    self.add_code(&format!(
                        "let state = {}.borrow().get_state();",
                        self.config.state_context_var_name
                    ));
                } else {
                    self.add_code(&format!(
                        "let state = self.{}();",
                        self.config.state_stack_pop_method_name
                    ));
                }
            }
        }
        AstVisitorReturnType::StateStackOperationStatementNode {}
    }
    //* --------------------------------------------------------------------- *//

    fn visit_state_context_node(
        &mut self,
        _state_context_node: &StateContextNode,
    ) -> AstVisitorReturnType {
        // TODO
        //        self.add_code(&format!("{}",identifier_node.name.lexeme));

        AstVisitorReturnType::StateContextNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_event_part(
        &mut self,
        frame_event_part: &FrameEventPart,
    ) -> AstVisitorReturnType {
        // TODO: make this code generate from settings
        match frame_event_part {
            FrameEventPart::Event { is_reference } => self.add_code(&format!(
                "{}{}",
                if *is_reference { "&" } else { "" },
                self.config.frame_event_variable_name
            )),
            FrameEventPart::Message { is_reference } => self.add_code(&format!(
                "{}{}.{}.to_string()",
                if *is_reference { "&" } else { "" },
                self.config.frame_event_variable_name,
                self.config.frame_event_message_attribute_name
            )),
            // FrameEventPart::Param {param_tok} => self.add_code(&format!("{}._parameters[\"{}\"]"
            //                                                             ,self.config.frame_event_variable_name
            FrameEventPart::Param {
                param_tok,
                is_reference,
            } => {
                self.add_code(&format!(
                    "{}{}.{}.as_ref().unwrap().{}()",
                    if *is_reference { "&" } else { "" },
                    self.config.frame_event_variable_name,
                    self.config.frame_event_parameters_attribute_name,
                    self.format_event_param_getter(&self.current_message, &param_tok.lexeme)
                ));
            }
            FrameEventPart::Return { is_reference } => {
                self.add_code(&format!(
                    "{}{}.{}.{}()",
                    if *is_reference { "&" } else { "" },
                    self.config.frame_event_variable_name,
                    self.config.frame_event_return_attribute_name,
                    self.format_event_param_getter(&self.current_message, "ret")
                ));
                // self.add_code(&format!("{}{}.{}"
                //                         ,if *is_reference {"&"} else {""}
                //                        ,self.config.frame_event_variable_name
                //                        ,self.config.frame_event_return_attribute_name))
            }
        }

        AstVisitorReturnType::FrameEventExprType {}
    }

    //* --------------------------------------------------------------------- *//

    // TODO: this is not the right framemessage codegen
    fn visit_frame_event_part_to_string(
        &mut self,
        frame_event_part: &FrameEventPart,
        output: &mut String,
    ) -> AstVisitorReturnType {
        // TODO: make this code generate from settings
        match frame_event_part {
            FrameEventPart::Event { is_reference } => output.push_str(&format!(
                "{}{}",
                if *is_reference { "&" } else { "" },
                self.config.frame_event_variable_name
            )),
            FrameEventPart::Message { is_reference } => output.push_str(&format!(
                "{}{}.{}.to_string()",
                if *is_reference { "&" } else { "" },
                self.config.frame_event_variable_name,
                self.config.frame_event_message_attribute_name
            )),
            FrameEventPart::Param {
                param_tok,
                is_reference,
            } => {
                output.push_str(&format!(
                    "{}{}.{}.as_ref().unwrap().{}()",
                    if *is_reference { "&" } else { "" },
                    self.config.frame_event_variable_name,
                    self.config.frame_event_parameters_attribute_name,
                    self.format_event_param_getter(&self.current_message, &param_tok.lexeme)
                ));
            }
            FrameEventPart::Return { is_reference } => output.push_str(&format!(
                "{}{}.{}.{}()",
                if *is_reference { "&" } else { "" },
                self.config.frame_event_variable_name,
                self.config.frame_event_return_attribute_name,
                self.format_event_param_getter(&self.current_message, "ret")
            )),
        }

        AstVisitorReturnType::FrameEventExprType {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_decl_node(&mut self, action_decl_node: &ActionNode) -> AstVisitorReturnType {
        //        let mut subclass_code = String::new();

        self.newline();
        //        self.newline_to_string(&mut subclass_code);

        let action_name = self.format_action_name(&action_decl_node.name);
        self.add_code(&format!("fn {}(&self", action_name));
        //        subclass_code.push_str(&format!("fn {}(",action_name));

        match &action_decl_node.params {
            Some(params) => {
                self.format_actions_parameter_list(params);
            }
            None => {}
        }
        // subclass_code.push_str(&format!(") {{}}"));
        // self.subclass_code.push(subclass_code);

        self.add_code(")");
        match &action_decl_node.type_opt {
            Some(ret_type) => {
                self.add_code(&format!(" -> {}", ret_type.get_type_str()));
            }
            None => {}
        };

        self.add_code(";");

        AstVisitorReturnType::ActionDeclNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_impl_node(&mut self, action_node: &ActionNode) -> AstVisitorReturnType {
        //        let mut subclass_code = String::new();

        self.newline();
        //        self.newline_to_string(&mut subclass_code);

        let action_name = self.format_action_name(&action_node.name);
        self.add_code(&format!("fn {}(&self", action_name));
        //        subclass_code.push_str(&format!("fn {}(",action_name));

        match &action_node.params {
            Some(params) => {
                self.format_actions_parameter_list(params);
            }
            None => {}
        }
        // subclass_code.push_str(&format!(") {{}}"));
        // self.subclass_code.push(subclass_code);

        self.add_code(")");
        match &action_node.type_opt {
            Some(ret_type) => {
                self.add_code(&format!(" -> {}", ret_type.get_type_str()));
            }
            None => {}
        };

        self.add_code(" {");

        match &action_node.code_opt {
            Some(code) => {
                self.indent();
                self.add_code(&*code);
                self.outdent();
            }
            None => {}
        }
        self.add_code("}");

        AstVisitorReturnType::ActionDeclNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_variable_decl_node(
        &mut self,
        variable_decl_node: &VariableDeclNode,
    ) -> AstVisitorReturnType {
        let var_type = match &variable_decl_node.type_opt {
            Some(x) => x.get_type_str(),
            None => String::from("<?>"),
        };
        let var_name = self.format_value_name(&variable_decl_node.name);
        self.newline();
        self.add_code(&format!("{}: {},", var_name, var_type));

        // currently unused serialization code
        // self.serialize.push(format!("\tbag.domain[\"{}\"] = {};",var_name,var_name));
        // self.deserialize.push(format!("\t{} = bag.domain[\"{}\"];",var_name,var_name));

        AstVisitorReturnType::VariableDeclNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_decl_node(
        &mut self,
        variable_decl_node: &VariableDeclNode,
    ) -> AstVisitorReturnType {
        let var_type = match &variable_decl_node.type_opt {
            Some(x) => format!(": {}", x.get_type_str()),
            None => String::new(),
        };
        let var_name = self.format_value_name(&variable_decl_node.name);
        let var_init_expr = &variable_decl_node.initializer_expr_t_opt.as_ref().unwrap();
        self.newline();
        let mut code = String::new();
        var_init_expr.accept_to_string(self, &mut code);
        self.add_code(&format!("let {}{} = {};", var_name, var_type, code));

        // currently unused serialization code
        // self.serialize.push(format!("\tbag.domain[\"{}\"] = {};",var_name,var_name));
        // self.deserialize.push(format!("\t{} = bag.domain[\"{}\"];",var_name,var_name));

        AstVisitorReturnType::VariableDeclNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node(&mut self, variable_node: &VariableNode) -> AstVisitorReturnType {
        let code = self.format_variable_expr(variable_node);
        self.add_code(&code);

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node_to_string(
        &mut self,
        variable_node: &VariableNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        let code = self.format_variable_expr(variable_node);
        output.push_str(&code);

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_stmt_node(
        &mut self,
        variable_stmt_node: &VariableStmtNode,
    ) -> AstVisitorReturnType {
        // TODO: what is this line about?
        self.generate_comment(variable_stmt_node.get_line());
        self.newline();
        let code = self.format_variable_expr(&variable_stmt_node.var_node);
        self.add_code(&code);

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
    ) -> AstVisitorReturnType {
        self.generate_comment(assignment_expr_node.line);
        self.newline();
        match &*assignment_expr_node.l_value_box {
            ExprType::FrameEventExprT { .. } => {
                let mut code = String::new();
                assignment_expr_node
                    .r_value_box
                    .accept_to_string(self, &mut code);
                self.add_code(&format!(
                    "{}.{} = ",
                    self.config.frame_event_variable_name,
                    self.config.frame_event_return_attribute_name
                ));

                self.add_code(&format!(
                    "{}::{} {{ return_value: {}}};",
                    self.config.frame_event_return,
                    self.format_type_name(&self.current_message),
                    code
                ));
            }
            _ => {
                assignment_expr_node.l_value_box.accept(self);
                self.add_code(" = ");
                assignment_expr_node.r_value_box.accept(self);
                self.add_code(";");
            }
        }

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node_to_string(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        self.generate_comment(assignment_expr_node.line);
        self.newline();
        self.newline_to_string(output);
        assignment_expr_node
            .l_value_box
            .accept_to_string(self, output);
        output.push_str(" = ");
        assignment_expr_node
            .r_value_box
            .accept_to_string(self, output);
        output.push_str(";");

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_statement_node(
        &mut self,
        assignment_stmt_node: &AssignmentStmtNode,
    ) -> AstVisitorReturnType {
        self.generate_comment(assignment_stmt_node.get_line());
        assignment_stmt_node.assignment_expr_node.accept(self);

        AstVisitorReturnType::AssignmentExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node(&mut self, unary_expr_node: &UnaryExprNode) -> AstVisitorReturnType {
        // TODO
        //       self.generate_comment(assignment_expr_node.line);
        unary_expr_node.operator.accept(self);
        unary_expr_node.right_rcref.borrow().accept(self);

        AstVisitorReturnType::UnaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_unary_expr_node_to_string(
        &mut self,
        unary_expr_node: &UnaryExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
        // TODO
        //       self.generate_comment(assignment_expr_node.line);
        unary_expr_node.operator.accept_to_string(self, output);
        unary_expr_node
            .right_rcref
            .borrow()
            .accept_to_string(self, output);

        AstVisitorReturnType::UnaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node(
        &mut self,
        binary_expr_node: &BinaryExprNode,
    ) -> AstVisitorReturnType {
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

        AstVisitorReturnType::BinaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node_to_string(
        &mut self,
        binary_expr_node: &BinaryExprNode,
        output: &mut String,
    ) -> AstVisitorReturnType {
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

        AstVisitorReturnType::BinaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operator_type(&mut self, operator_type: &OperatorType) -> AstVisitorReturnType {
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

        AstVisitorReturnType::BinaryExprNode {}
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operator_type_to_string(
        &mut self,
        operator_type: &OperatorType,
        output: &mut String,
    ) -> AstVisitorReturnType {
        match operator_type {
            OperatorType::Plus => output.push_str(" + "),
            OperatorType::Minus => output.push_str(" - "),
            OperatorType::Negated => output.push_str("-"),
            OperatorType::Multiply => output.push_str(" * "),
            OperatorType::Divide => output.push_str(" / "),
            OperatorType::Greater => output.push_str(" > "),
            OperatorType::GreaterEqual => output.push_str(" >= "),
            OperatorType::Less => output.push_str(" < "),
            OperatorType::LessEqual => output.push_str(" <= "),
            OperatorType::Not => output.push_str("!"),
            OperatorType::EqualEqual => output.push_str(" == "),
            OperatorType::NotEqual => output.push_str(" != "),
            OperatorType::LogicalAnd => output.push_str(" && "),
            OperatorType::LogicalOr => output.push_str(" || "),
            OperatorType::LogicalXor => output.push_str(""),
        }

        AstVisitorReturnType::BinaryExprNode {}
    }
}

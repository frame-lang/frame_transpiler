use convert_case::{Case, Casing};
use std::collections::HashSet;

use crate::frame_c::ast::*;
use crate::frame_c::config::*;
use crate::frame_c::scanner::{Token, TokenType};
use crate::frame_c::symbol_table::*;
use crate::frame_c::visitors::*;

pub struct RustVisitor {
    config: RustConfig,
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
    generate_enter_args: bool,
    generate_exit_args: bool,
    generate_state_context: bool,
    generate_state_stack: bool,
    generate_change_state: bool,
    generate_transition_state: bool,
    generate_change_state_hook: bool,
    generate_transition_hook: bool,
    current_message: String,
}

impl RustVisitor {
    pub fn new(
        arcanum: Arcanum,
        config: FrameConfig,
        generate_enter_args: bool,
        generate_exit_args: bool,
        generate_state_context: bool,
        generate_state_stack: bool,
        generate_change_state: bool,
        generate_transition_state: bool,
        compiler_version: &str,
        comments: Vec<Token>,
    ) -> RustVisitor {
        let rust_config = config.codegen.rust;
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
            generate_enter_args,
            generate_exit_args,
            generate_state_context,
            generate_state_stack,
            generate_change_state,
            generate_transition_state,
            generate_change_state_hook: rust_config.features.generate_hook_methods
                && generate_change_state,
            generate_transition_hook: rust_config.features.generate_hook_methods
                && generate_transition_state,
            current_message: String::new(),
            config: rust_config,
        }
    }

    //* --------------------------------------------------------------------- *//

    /// Enter/exit messages are formatted "stateName:>" or "stateName:<".
    pub fn is_enter_or_exit_message(&self, msg: &str) -> bool {
        let split = msg.split(':');

        split.count() == 2
    }

    //* --------------------------------------------------------------------- *//

    pub fn get_msg_enum(&self, msg: &str) -> String {
        let unformatted = match msg {
            // ">>" => self.config.code.start_system_msg.clone(),
            // "<<" => self.config.code.stop_system_msg.clone(),
            ">" => self.config.code.enter_msg.clone(),
            "<" => self.config.code.exit_msg.clone(),
            _ => self.arcanum.get_interface_or_msg_from_msg(msg).unwrap(),
        };
        self.format_type_name(&unformatted)
    }

    //* --------------------------------------------------------------------- *//

    #[allow(unknown_lints)]
    #[allow(clippy::branches_sharing_code)]
    fn parse_event_name(&self, event_name: &str) -> (Option<String>, String) {
        let split = event_name.split(':');
        let vec: Vec<&str> = split.collect();
        if vec.len() == 1 {
            let event_name = vec.get(0).unwrap();
            (None, event_name.to_string())
        } else {
            let state_name = vec.get(0).unwrap();
            let event_name = vec.get(1).unwrap();
            (Some(state_name.to_string()), event_name.to_string())
        }
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

    fn format_variable_expr(&mut self, variable_node: &VariableNode) -> String {
        let mut code = String::new();

        match variable_node.scope {
            IdentifierDeclScope::DomainBlock => {
                if variable_node.id_node.is_reference {
                    code.push('&');
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
                    code.push('(');
                }

                if var_node.id_node.is_reference {
                    code.push('&');
                }

                code.push_str(&format!(
                    "{}.{}.{}",
                    self.config.code.this_state_context_var_name,
                    self.config.code.state_args_var_name,
                    self.format_value_name(&variable_node.id_node.name.lexeme)
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::StateVar => {
                let var_node = variable_node;
                let var_symbol_rcref_opt = &var_node.symbol_type_rcref_opt;
                let var_symbol_rcref = var_symbol_rcref_opt.as_ref().unwrap();
                let _var_symbol = var_symbol_rcref.borrow();
                //                let var_type = self.get_variable_type(&*var_symbol);

                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }

                if var_node.id_node.is_reference {
                    code.push('&');
                }

                code.push_str(&format!(
                    "{}.{}.{}",
                    self.config.code.this_state_context_var_name,
                    self.config.code.state_vars_var_name,
                    self.format_value_name(&variable_node.id_node.name.lexeme)
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::EventHandlerParam => {
                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }

                if variable_node.id_node.is_reference {
                    code.push('&');
                }

                let event_type = self.format_state_event_type_name(
                    self.current_state_name_opt.as_ref().unwrap(),
                    &self.current_message,
                );
                code.push_str(&format!(
                    "{}.{}.{}().{}",
                    self.config.code.frame_event_variable_name,
                    self.config.code.frame_event_args_attribute_name,
                    self.format_args_method_name(&event_type),
                    self.format_value_name(&variable_node.id_node.name.lexeme)
                ));

                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::EventHandlerVar => {
                if variable_node.id_node.is_reference {
                    code.push('&');
                }
                code.push_str(&self.format_value_name(&variable_node.id_node.name.lexeme));
            }
            IdentifierDeclScope::None => {
                // TODO: Explore labeling Variables as "extern" scope
                if variable_node.id_node.is_reference {
                    code.push('&');
                }
                code.push_str(&self.format_value_name(&variable_node.id_node.name.lexeme));
            } // Actions?
            _ => self.errors.push("Illegal scope.".to_string()),
        }

        code
    }

    //* --------------------------------------------------------------------- *//

    fn format_parameter_list(&mut self, params: &[ParameterNode]) {
        for param in params {
            self.add_code(&", ".to_string());
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

    fn format_actions_parameter_list(&mut self, params: &[ParameterNode]) {
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

    fn new_var_name(&self, base_name: &str) -> String {
        format!("new_{}", base_name)
    }

    fn old_var_name(&self, base_name: &str) -> String {
        format!("old_{}", base_name)
    }

    fn system_type_name(&self) -> String {
        self.format_type_name(&self.system_name)
    }

    fn state_enum_type_name(&self) -> String {
        self.format_type_name(&format!(
            "{}{}",
            self.system_name, self.config.code.state_enum_suffix
        ))
    }

    fn action_trait_type_name(&self) -> String {
        self.format_type_name(&format!(
            "{}{}{}",
            self.config.code.actions_prefix,
            self.system_type_name(),
            self.config.code.actions_suffix
        ))
    }

    fn lifetime_ref_annotation(&self) -> &str {
        if self.config.features.runtime_support {
            "&'a "
        } else {
            ""
        }
    }

    fn lifetime_type_annotation(&self) -> &str {
        if self.config.features.runtime_support {
            "<'a>"
        } else {
            ""
        }
    }

    fn ref_if_runtime_support(&self) -> &str {
        if self.config.features.runtime_support {
            "&"
        } else {
            ""
        }
    }

    //* --------------------------------------------------------------------- *//

    /// Disable formatting/style warnings on generated type definitions.
    fn disable_type_style_warnings(&mut self) {
        if !self.config.features.follow_rust_naming {
            self.add_code("#[allow(clippy::upper_case_acronyms)]");
            self.newline();
            self.add_code("#[allow(non_camel_case_types)]");
            self.newline();
            self.add_code("#[allow(non_snake_case)]");
            self.newline();
        }
        self.add_code("#[allow(dead_code)]");
        self.newline();
    }

    /// Disable all formatting/style warnings on generated definitions.
    fn disable_all_style_warnings(&mut self) {
        self.add_code("#[allow(clippy::assign_op_pattern)]");
        self.newline();
        self.add_code("#[allow(clippy::branches_sharing_code)]");
        self.newline();
        self.add_code("#[allow(clippy::clone_on_copy)]");
        self.newline();
        self.add_code("#[allow(clippy::double_parens)]");
        self.newline();
        self.add_code("#[allow(clippy::match_single_binding)]");
        self.newline();
        self.add_code("#[allow(clippy::ptr_arg)]");
        self.newline();
        self.add_code("#[allow(clippy::single_match)]");
        self.newline();
        self.add_code("#[allow(clippy::wrong_self_convention)]");
        self.newline();
        self.add_code("#[allow(unused_variables)]");
        self.newline();
        self.disable_type_style_warnings();
    }

    // /// Disable formatting/style warnings on generated method definitions.

    // Formatting helper functions

    /// Format a "type-level" name, e.g. a type, trait, or enum variant.
    /// If Rust naming conventions are followed, these are in CamelCase.
    fn format_type_name(&self, name: &str) -> String {
        let mut formatted = name.to_string();
        if self.config.features.follow_rust_naming {
            formatted = formatted.to_case(Case::UpperCamel);
        }
        formatted
    }

    /// Format a "value-level" name, e.g. a function, method, variable.
    /// If Rust naming conventions are followed, these are  in snake_case.
    fn format_value_name(&self, name: &str) -> String {
        let mut formatted = name.to_string();
        if self.config.features.follow_rust_naming {
            formatted = formatted.to_case(Case::Snake);
        }
        formatted
    }

    fn format_getter_name(&self, member_name: &str) -> String {
        format!("get_{}", self.format_value_name(&member_name.to_string()))
    }

    // Type/case names

    /// Get an event name in a format usable in Rust types and enum cases.
    fn format_event_type_name(&self, raw_event_name: &str) -> String {
        let (state_name_opt, event_name) = self.parse_event_name(raw_event_name);
        match state_name_opt {
            Some(state_name) => self.format_state_event_type_name(&state_name, &event_name),
            None => self.format_type_name(&event_name),
        }
    }

    /// Get the event type name for a given state+event.
    fn format_state_event_type_name(&self, state_name: &str, event_name: &str) -> String {
        if event_name.eq(&self.config.code.enter_token) {
            self.format_enter_event_type_name(state_name)
        } else if event_name.eq(&self.config.code.exit_token) {
            self.format_exit_event_type_name(state_name)
        } else {
            self.format_type_name(&event_name.to_string())
        }
    }

    /// Get the event type name for the enter event of the given state.
    fn format_enter_event_type_name(&self, state_name: &str) -> String {
        self.format_type_name(&format!("{}{}", state_name, self.config.code.enter_msg))
    }

    /// Get the event type name for the exit event of the given state.
    fn format_exit_event_type_name(&self, state_name: &str) -> String {
        self.format_type_name(&format!("{}{}", state_name, self.config.code.exit_msg))
    }

    fn format_args_struct_name(&self, event_type_name: &str) -> String {
        format!("{}{}", event_type_name, self.config.code.event_args_suffix)
    }

    fn format_state_args_struct_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_type_name(&state_name.to_string()),
            self.config.code.state_args_suffix
        )
    }

    fn format_state_context_struct_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_type_name(&state_name.to_string()),
            self.config.code.state_context_suffix
        )
    }

    fn format_state_vars_struct_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_type_name(&state_name.to_string()),
            self.config.code.state_vars_suffix
        )
    }

    // Method names

    fn format_action_name(&mut self, action_name: &str) -> String {
        format!(
            "{}{}{}",
            self.config.code.action_prefix,
            self.format_value_name(action_name),
            self.config.code.action_suffix
        )
    }

    fn format_param_getter(&self, message_name: &str, param_name: &str) -> String {
        self.format_getter_name(&format!("{}_{}", message_name, param_name))
    }

    /// Get the name of the method that returns the particular args struct for
    /// this event.
    fn format_args_method_name(&self, event_type_name: &str) -> String {
        format!(
            "{}{}",
            self.format_value_name(&event_type_name.to_string()),
            self.config.code.event_args_method_suffix
        )
    }

    fn format_state_context_method_name(&self, state_name: &str) -> String {
        format!(
            "{}{}",
            self.format_value_name(&state_name.to_string()),
            self.config.code.state_context_method_suffix
        )
    }

    fn format_state_handler_name(&self, state_name: &str) -> String {
        format!(
            "{}{}{}",
            self.config.code.state_handler_name_prefix,
            self.format_value_name(&state_name.to_string()),
            self.config.code.state_handler_name_suffix
        )
    }

    //* --------------------------------------------------------------------- *//

    pub fn run(&mut self, system_node: &SystemNode) {
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

    fn enter_block(&mut self) {
        self.add_code(" {");
        self.indent();
        self.newline();
    }

    fn exit_block(&mut self) {
        self.outdent();
        self.newline();
        self.add_code("}");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_decl_stmts(&mut self, decl_stmt_types: &[DeclOrStmtType]) {
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
        let mut traits = self.config.code.state_enum_traits.clone();
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
        self.disable_type_style_warnings();
        if !traits.is_empty() {
            self.add_code(&format!("#[derive({})]", traits));
            self.newline();
        }

        // add the state enum type
        let state_enum_type = self.state_enum_type_name();
        self.add_code(&format!("pub enum {} {{", state_enum_type));
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

        // generate trivial runtime state impl if no state contexts
        if self.config.features.runtime_support && !self.generate_state_context {
            self.newline();
            self.newline();
            self.add_code(&format!("impl State for {}", state_enum_type));
            self.enter_block();

            self.add_code("fn name(&self) -> &'static str");
            self.enter_block();
            self.add_code("match *self {");
            self.indent();
            if let Some(machine_block_node) = &system_node.machine_block_node_opt {
                for state in &machine_block_node.states {
                    let state_name = &state.borrow().name;
                    self.newline();
                    self.add_code(&format!(
                        "{}::{} => \"{}\",",
                        state_enum_type,
                        self.format_type_name(state_name),
                        state_name
                    ));
                }
            } else {
                self.add_code("\"\"");
            }
            self.exit_block();
            self.exit_block();

            self.newline();
            self.add_code("fn state_arguments(&self) -> &dyn Environment { EMPTY }");
            self.newline();
            self.add_code("fn state_variables(&self) -> &dyn Environment { EMPTY }");
            self.exit_block();
        }
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the structs, enum, and supporting function definitions related
    /// to event arguments.
    fn generate_event_arg_defs(&mut self) {
        let mut has_params: HashSet<String> = HashSet::new();

        // generate an arg struct for all events that have parameters
        for event_name in self.arcanum.get_event_names() {
            if let Some(event_sym) = self.arcanum.get_event(&event_name, &None) {
                if let Some(params) = &event_sym.borrow().params_opt {
                    let event_type_name = self.format_event_type_name(&event_name);
                    let args_struct_name = self.format_args_struct_name(&event_type_name);
                    let mut bound_names: Vec<String> = Vec::new();

                    self.disable_type_style_warnings();
                    self.add_code(&format!("struct {} {{", args_struct_name));
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

                    // generate the env
                    if self.config.features.runtime_support {
                        self.generate_environment_impl(false, &args_struct_name, bound_names);
                    }
                    has_params.insert(event_type_name);
                }
            }
        }

        // generate the enum type that unions all the arg structs
        self.disable_type_style_warnings();
        self.add_code(&format!(
            "enum {}",
            self.config.code.frame_event_args_type_name
        ));
        self.enter_block();
        self.add_code("None,");
        for event_type_name in &has_params {
            self.newline();
            self.add_code(&format!(
                "{}({}),",
                &event_type_name,
                self.format_args_struct_name(event_type_name)
            ));
        }
        self.exit_block();
        self.newline();
        self.newline();

        // generate environment impl for enum type
        if self.config.features.runtime_support {
            self.add_code(&format!(
                "impl Environment for {}",
                self.config.code.frame_event_args_type_name
            ));
            self.enter_block();
            self.add_code("fn lookup(&self, name: &str) -> Option<&dyn Any>");
            self.enter_block();
            self.add_code("match self");
            self.enter_block();
            self.add_code(&format!(
                "{}::None => EMPTY.lookup(name),",
                self.config.code.frame_event_args_type_name
            ));
            for event_type_name in &has_params {
                self.newline();
                self.add_code(&format!(
                    "{}::{}(args) => args.lookup(name),",
                    self.config.code.frame_event_args_type_name, &event_type_name
                ));
            }
            self.exit_block();
            self.exit_block();
            self.exit_block();
            self.newline();
            self.newline();
        }

        if !has_params.is_empty() {
            // generate methods to conveniently get specific arg structs
            // from a value of the enum type
            self.disable_type_style_warnings();
            self.add_code(&format!(
                "impl {} {{",
                self.config.code.frame_event_args_type_name
            ));
            self.indent();
            for event_type_name in &has_params {
                self.newline();
                self.add_code(&format!(
                    "fn {}(&self) -> &{} {{",
                    self.format_args_method_name(event_type_name),
                    self.format_args_struct_name(event_type_name)
                ));
                self.indent();
                self.newline();
                self.add_code("match self {");
                self.indent();
                self.newline();
                self.add_code(&format!(
                    "{}::{}(args) => args,",
                    self.config.code.frame_event_args_type_name, event_type_name
                ));
                self.newline();
                self.add_code(&format!(
                    "_ => panic!(\"Failed conversion to {}\"),",
                    self.format_args_struct_name(event_type_name)
                ));
                self.exit_block();
                self.exit_block();
            }
            self.exit_block();
            self.newline();
            self.newline();
        }
    }

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

                        self.disable_type_style_warnings();
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

                        if self.config.features.runtime_support {
                            self.generate_environment_impl(
                                false,
                                &state_args_struct_name,
                                bound_names,
                            );
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

                        self.disable_type_style_warnings();
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

                        if self.config.features.runtime_support {
                            self.generate_environment_impl(
                                false,
                                &state_vars_struct_name,
                                bound_names,
                            );
                        }
                    }
                    None => {}
                }

                // generate state context struct for this state
                let context_struct_name = self.format_state_context_struct_name(&state_node.name);
                self.disable_type_style_warnings();
                self.add_code(&format!("struct {} {{", context_struct_name));
                self.indent();

                if has_state_args {
                    self.newline();
                    self.add_code(&format!(
                        "{}: {},",
                        self.config.code.state_args_var_name, state_args_struct_name
                    ));
                }

                if has_state_vars {
                    self.newline();
                    self.add_code(&format!(
                        "{}: {},",
                        self.config.code.state_vars_var_name, state_vars_struct_name
                    ));
                }

                self.exit_block();
                self.newline();
                self.newline();

                // generate implementation of runtime state
                if self.config.features.runtime_support {
                    self.add_code(&format!("impl State for {}", context_struct_name));
                    self.enter_block();

                    self.add_code("fn name(&self) -> &'static str");
                    self.enter_block();
                    self.add_code(&format!("\"{}\"", &state_node.name));
                    self.exit_block();
                    self.newline();

                    self.add_code("fn state_arguments(&self) -> &dyn Environment");
                    self.enter_block();
                    if has_state_args {
                        self.add_code(&format!("&self.{}", self.config.code.state_args_var_name));
                    } else {
                        self.add_code("EMPTY");
                    }
                    self.exit_block();
                    self.newline();

                    self.add_code("fn state_variables(&self) -> &dyn Environment");
                    self.enter_block();
                    if has_state_vars {
                        self.add_code(&format!("&self.{}", self.config.code.state_vars_var_name));
                    } else {
                        self.add_code("EMPTY");
                    }
                    self.exit_block();
                    self.newline();

                    self.exit_block();
                    self.newline();
                    self.newline();
                }
            }

            // generate the enum type that unions all the state context types
            self.disable_type_style_warnings();
            self.add_code(&format!(
                "enum {} {{",
                self.config.code.state_context_type_name
            ));
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

            self.disable_type_style_warnings();
            self.add_code(&format!(
                "impl {} {{",
                self.config.code.state_context_type_name
            ));
            self.indent();

            // generate method to get the state context as a runtime state
            if self.config.features.runtime_support {
                self.newline();
                self.add_code("fn as_runtime_state(&self) -> Ref<dyn State>");
                self.enter_block();
                self.add_code("match self {");
                self.indent();
                for state in &machine_block_node.states {
                    let state_node = state.borrow();
                    self.newline();
                    self.add_code(&format!(
                        "{}::{}(context) => Ref::map(context.borrow(), |c| c as &dyn State),",
                        self.config.code.state_context_type_name,
                        self.format_type_name(&state_node.name)
                    ));
                }
                self.exit_block();
                self.exit_block();
            }

            // generate methods to conveniently get specific state contexts
            // from a value of the enum type
            for state in &machine_block_node.states {
                let state_node = state.borrow();
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
                    self.config.code.state_context_type_name,
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
            self.exit_block();
        }
    }

    fn generate_environment_impl(
        &mut self,
        has_lifetime_annotation: bool,
        type_name: &str,
        bound_names: Vec<String>,
    ) {
        self.add_code("#[allow(clippy::match_single_binding)]");
        self.newline();
        self.add_code("#[allow(clippy::single_match)]");
        self.newline();
        if has_lifetime_annotation {
            self.add_code(&format!(
                "impl{0} Environment for {1}{0}",
                self.lifetime_type_annotation(),
                type_name
            ));
        } else {
            self.add_code(&format!("impl Environment for {}", type_name));
        }
        self.enter_block();
        self.add_code("fn lookup(&self, name: &str) -> Option<&dyn Any>");
        self.enter_block();
        self.add_code("match name");
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
        self.add_code(&format!(
            "pub fn new() -> {}{}",
            self.system_type_name(),
            self.lifetime_type_annotation()
        ));
        self.add_code(" {");
        self.indent();

        let init_state_name = self.first_state_name.clone();

        // initial state variables
        self.indent();
        let mut formatted_state_vars = String::new();
        let has_state_vars =
            self.generate_state_variables(&init_state_name, &mut formatted_state_vars);
        self.outdent();

        // initial state context
        if self.generate_state_context {
            self.generate_next_state_context(
                &init_state_name,
                false,
                has_state_vars,
                "",
                &formatted_state_vars,
            );
        }

        // begin create state machine
        self.newline();
        self.add_code(&format!("let mut machine = {}", self.system_type_name()));
        self.enter_block();
        self.add_code(&format!(
            "{}: {}::{},",
            self.config.code.state_var_name,
            self.state_enum_type_name(),
            self.format_type_name(&self.first_state_name)
        ));

        // initialize the state stack
        if self.generate_state_stack {
            self.newline();
            self.add_code(&format!(
                "{}: Vec::new(),",
                self.config.code.state_stack_var_name
            ));
        }

        // initialize runtime support
        if self.config.features.runtime_support {
            if !self.generate_state_context {
                self.newline();
                self.add_code(&format!(
                    "{}: RefCell::new({}::{}),",
                    self.config.code.state_cell_var_name,
                    self.state_enum_type_name(),
                    self.format_type_name(&self.first_state_name)
                ));
            }
            self.newline();
            self.add_code(&format!(
                "{}: CallbackManager::new(),",
                self.config.code.callback_manager_var_name
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
                self.config.code.state_context_var_name
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
            self.config.code.initialize_method_name
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
            self.config.code.initialize_method_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "let mut {} = {}::new({}::{}, {}{}::None);",
            self.config.code.frame_event_variable_name,
            self.config.code.frame_event_type_name,
            self.config.code.frame_event_message_type_name,
            self.config.code.enter_msg,
            self.ref_if_runtime_support(),
            self.config.code.frame_event_args_type_name,
        ));
        self.newline();
        self.add_code(&format!(
            "self.{}(&mut {});",
            self.config.code.handle_event_method_name, self.config.code.frame_event_variable_name
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
        if system_node.get_first_state().is_some() {
            self.newline();
            self.generate_handle_event(system_node);
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

    /// Generate the change_state method.
    fn generate_change_state(&mut self) {
        let old_state_context_var = self.old_var_name(&self.config.code.state_context_var_name);
        let new_state_context_var = self.new_var_name(&self.config.code.state_context_var_name);
        let old_state_var = self.old_var_name(&self.config.code.state_var_name);
        let new_state_var = self.new_var_name(&self.config.code.state_var_name);

        // generate method signature
        if self.generate_state_context {
            self.add_code(&format!(
                "fn {}(&mut self, {}: {}, {}: Rc<{}>) {{",
                self.config.code.change_state_method_name,
                new_state_var,
                self.state_enum_type_name(),
                new_state_context_var,
                self.config.code.state_context_type_name
            ));
        } else {
            self.add_code(&format!(
                "fn {}(&mut self, {}: {}) {{",
                self.config.code.change_state_method_name,
                new_state_var,
                self.state_enum_type_name()
            ));
        }
        self.indent();

        // save old state
        if self.generate_change_state_hook {
            self.newline();
            self.add_code(&format!(
                "let {} = self.{};",
                old_state_var, self.config.code.state_var_name
            ));
        }
        if self.config.features.runtime_support {
            if self.generate_state_context {
                self.newline();
                self.add_code(&format!(
                    "let {} = Rc::clone(&self.{});",
                    old_state_context_var, self.config.code.state_context_var_name
                ));
                self.newline();
                self.add_code(&format!(
                    "let old_runtime_state = {}.as_ref().as_runtime_state();",
                    old_state_context_var
                ));
            } else {
                let old_state_cell_name = self.old_var_name(&self.config.code.state_cell_var_name);
                self.newline();
                self.add_code(&format!(
                    "let {} = RefCell::new(self.{});",
                    old_state_cell_name, self.config.code.state_var_name
                ));
                self.newline();
                self.add_code(&format!(
                    "let old_runtime_state = {}.borrow();",
                    old_state_cell_name
                ));
            }
        }

        // update state
        self.newline();
        self.add_code(&format!(
            "self.{} = {};",
            self.config.code.state_var_name, new_state_var
        ));
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "self.{} = Rc::clone(&{});",
                self.config.code.state_context_var_name, new_state_context_var
            ));
        } else if self.config.features.runtime_support {
            self.newline();
            self.add_code(&format!(
                "self.{} = RefCell::new(self.{});",
                self.config.code.state_cell_var_name, self.config.code.state_var_name
            ));
        }

        // call hook method
        if self.generate_change_state_hook {
            self.newline();
            self.add_code(&format!(
                "self.{}({}, {});",
                self.config.code.change_state_hook_method_name, old_state_var, new_state_var,
            ));
        }

        // call transition callbacks
        if self.config.features.runtime_support {
            self.newline();
            if self.generate_state_context {
                self.add_code(&format!(
                    "let new_runtime_state = {}.as_runtime_state();",
                    new_state_context_var
                ));
            } else {
                self.add_code(&format!(
                    "let new_runtime_state = self.{}.borrow();",
                    self.config.code.state_cell_var_name
                ));
            }
            self.newline();
            self.add_code(&format!(
                "self.{}.change_state(",
                self.config.code.callback_manager_var_name
            ));
            self.indent();
            self.newline();
            self.add_code("old_runtime_state,");
            self.newline();
            self.add_code("new_runtime_state,");
            self.outdent();
            self.newline();
            self.add_code(");");
        }

        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    /// Generate the transition method.
    fn generate_transition(&mut self) {
        let old_state_context_var = self.old_var_name(&self.config.code.state_context_var_name);
        let new_state_context_var = self.new_var_name(&self.config.code.state_context_var_name);
        let old_state_var = self.old_var_name(&self.config.code.state_var_name);
        let new_state_var = self.new_var_name(&self.config.code.state_var_name);

        // generate method signature
        self.add_code(&format!(
            "fn {}(&mut self, ",
            self.config.code.transition_method_name
        ));
        if self.generate_exit_args {
            self.add_code(&format!(
                "{}: {}, ",
                self.config.code.exit_args_member_name, self.config.code.frame_event_args_type_name
            ));
        }
        if self.generate_enter_args {
            self.add_code(&format!(
                "{}: {}, ",
                self.config.code.enter_args_member_name,
                self.config.code.frame_event_args_type_name
            ));
        }
        self.add_code(&format!(
            "{}: {}",
            new_state_var,
            self.state_enum_type_name()
        ));
        if self.generate_state_context {
            self.add_code(&format!(
                ", {}: Rc<{}>",
                new_state_context_var, self.config.code.state_context_type_name
            ));
        }
        self.add_code(")");
        self.enter_block();

        // exit event for old state
        let exit_args = if self.generate_exit_args {
            self.config.code.exit_args_member_name.to_string()
        } else {
            format!("{}::None", self.config.code.frame_event_args_type_name)
        };
        self.add_code(&format!(
            "let mut exit_event = {}::new({}::{}, {}{});",
            self.config.code.frame_event_type_name,
            self.config.code.frame_event_message_type_name,
            self.config.code.exit_msg,
            self.ref_if_runtime_support(),
            exit_args
        ));
        self.newline();
        self.add_code(&format!(
            "self.{}(&mut exit_event);",
            self.config.code.handle_event_method_name
        ));

        // save old state
        if self.generate_transition_hook {
            self.newline();
            self.add_code(&format!(
                "let {} = self.{};",
                old_state_var, self.config.code.state_var_name
            ));
        }
        if self.config.features.runtime_support {
            if self.generate_state_context {
                self.newline();
                self.add_code(&format!(
                    "let {} = Rc::clone(&self.{});",
                    old_state_context_var, self.config.code.state_context_var_name
                ));
                self.newline();
                self.add_code(&format!(
                    "let old_runtime_state = {}.as_ref().as_runtime_state();",
                    old_state_context_var
                ));
            } else {
                let old_state_cell_name = self.old_var_name(&self.config.code.state_cell_var_name);
                self.newline();
                self.add_code(&format!(
                    "let {} = RefCell::new(self.{});",
                    old_state_cell_name, self.config.code.state_var_name
                ));
                self.newline();
                self.add_code(&format!(
                    "let old_runtime_state = {}.borrow();",
                    old_state_cell_name
                ));
            }
        }

        // update state
        self.newline();
        self.add_code(&format!(
            "self.{} = {};",
            self.config.code.state_var_name, new_state_var
        ));
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "self.{} = Rc::clone(&{});",
                self.config.code.state_context_var_name, new_state_context_var
            ));
        } else if self.config.features.runtime_support {
            self.newline();
            self.add_code(&format!(
                "self.{} = RefCell::new(self.{});",
                self.config.code.state_cell_var_name, self.config.code.state_var_name
            ));
        }

        // call hook method
        if self.generate_transition_hook {
            self.newline();
            self.add_code(&format!(
                "self.{}({}, {});",
                self.config.code.transition_hook_method_name, old_state_var, new_state_var,
            ));
        }

        // call transition callbacks
        if self.config.features.runtime_support {
            self.newline();
            if self.generate_state_context {
                self.add_code(&format!(
                    "let new_runtime_state = {}.as_runtime_state();",
                    new_state_context_var
                ));
            } else {
                self.add_code(&format!(
                    "let new_runtime_state = self.{}.borrow();",
                    self.config.code.state_cell_var_name
                ));
            }
            self.newline();
            self.add_code(&format!(
                "self.{}.transition(",
                self.config.code.callback_manager_var_name
            ));
            self.indent();
            self.newline();
            self.add_code("old_runtime_state,");
            self.newline();
            self.add_code("new_runtime_state,");
            self.newline();
            if self.generate_exit_args {
                self.add_code(&format!("&{},", self.config.code.exit_args_member_name));
            } else {
                self.add_code("EMPTY,");
            }
            self.newline();
            if self.generate_enter_args {
                self.add_code(&format!("&{},", self.config.code.enter_args_member_name));
            } else {
                self.add_code("EMPTY,");
            }
            self.outdent();
            self.newline();
            self.add_code(");");
        }

        // enter event for new state
        self.newline();
        let enter_args = if self.generate_enter_args {
            self.config.code.enter_args_member_name.to_string()
        } else {
            format!("{}::None", self.config.code.frame_event_args_type_name)
        };
        self.add_code(&format!(
            "let mut enter_event = {}::new({}::{}, {}{});",
            self.config.code.frame_event_type_name,
            self.config.code.frame_event_message_type_name,
            self.config.code.enter_msg,
            self.ref_if_runtime_support(),
            enter_args
        ));
        self.newline();
        self.add_code(&format!(
            "self.{}(&mut enter_event);",
            &self.config.code.handle_event_method_name
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
    }

    /// Generate state stack methods.
    fn generate_state_stack_methods(&mut self) {
        self.add_code(&format!(
            "fn {}(&mut self) {{",
            self.config.code.state_stack_push_method_name
        ));
        self.indent();
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "self.{}.push((self.{}, Rc::clone(&self.{})));",
                self.config.code.state_stack_var_name,
                self.config.code.state_var_name,
                self.config.code.state_context_var_name
            ));
        } else {
            self.add_code(&format!(
                "self.{}.push(self.{});",
                self.config.code.state_stack_var_name, self.config.code.state_var_name
            ));
        }
        self.outdent();
        self.newline();
        self.add_code(&"}".to_string());
        self.newline();
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "fn {}(&mut self) -> ({}, Rc<{}>) {{",
                self.config.code.state_stack_pop_method_name,
                self.state_enum_type_name(),
                self.config.code.state_context_type_name
            ));
        } else {
            self.add_code(&format!(
                "fn {}(&mut self) -> {} {{",
                self.config.code.state_stack_pop_method_name,
                self.state_enum_type_name()
            ));
        }
        self.indent();
        self.newline();
        self.add_code(&format!(
            "match self.{}.pop() {{",
            self.config.code.state_stack_var_name
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
        self.newline();
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
            self.config.code.handle_event_method_name,
            self.config.code.frame_event_variable_name,
            self.config.code.frame_event_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "match self.{} {{",
            self.config.code.state_var_name
        ));
        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            self.indent();
            for state in &machine_block_node.states {
                self.newline();
                self.add_code(&format!(
                    "{}::{} => self.{}({}),",
                    self.state_enum_type_name(),
                    self.format_type_name(&state.borrow().name),
                    self.format_state_handler_name(&state.borrow().name),
                    self.config.code.frame_event_variable_name
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
        }
    }

    //* --------------------------------------------------------------------- *//

    /// Generate code that evaluates a list of expressions and constructs a
    /// value of the appropriate argument struct. Writes the generated code
    /// as a struct literal expression to `arg_code`. Returns `true` if any
    /// arguments were passed.
    fn generate_arguments(
        &mut self,
        arg_struct_name: &str,
        param_names: Vec<&String>,
        arg_exprs: &ExprListNode,
        arg_code: &mut String,
    ) -> bool {
        // check to make sure the right number of arguments were passed
        if param_names.len() != arg_exprs.exprs_t.len() {
            self.errors.push(format!(
                "Incorrect number of arguments for {}: expected {} got {}",
                arg_struct_name,
                param_names.len(),
                arg_exprs.exprs_t.len()
            ));
            return false;
        }
        // check to see if there are any arguments to process
        if param_names.is_empty() {
            arg_code.push_str("None");
            return false;
        }
        // generate code to eval arguments and build struct
        arg_code.push_str(&format!("{} {{", &arg_struct_name));
        self.indent();
        for (param, arg) in param_names.iter().zip(arg_exprs.exprs_t.iter()) {
            let mut expr = String::new();
            arg.accept_to_string(self, &mut expr);
            self.newline_to_string(arg_code);
            arg_code.push_str(&format!("{}: {},", self.format_value_name(param), expr));
        }
        self.outdent();
        self.newline_to_string(arg_code);
        arg_code.push('}');
        true
    }

    /// Get the exit arguments from a transition statement and generate a value
    /// of the exit arguments struct. Writes the generated struct literal
    /// expression to `arg_code`. Returns `true` if any arguments were passed.
    fn generate_exit_arguments(
        &mut self,
        transition_stmt: &TransitionStatementNode,
        arg_code: &mut String,
    ) -> bool {
        let mut has_args = false;
        if let Some(exit_args) = &transition_stmt.exit_args_opt {
            // Search for event keyed with "State:<", e.g. "S1:<"
            let empty = String::new();
            let current_state_name = self.current_state_name_opt.as_ref().unwrap_or(&empty);
            let exit_msg = format!(
                "{}:{}",
                current_state_name, self.symbol_config.exit_msg_symbol
            );
            let exit_event_type_name = self.format_exit_event_type_name(current_state_name);
            let arg_struct_name = self.format_args_struct_name(&exit_event_type_name);
            if let Some(event_sym) = self
                .arcanum
                .get_event(&exit_msg, &self.current_state_name_opt)
            {
                match &event_sym.borrow().params_opt {
                    Some(event_params) => {
                        let param_names = event_params.iter().map(|p| &p.name).collect();
                        has_args = self.generate_arguments(
                            &arg_struct_name,
                            param_names,
                            exit_args,
                            arg_code,
                        );
                        arg_code.insert_str(
                            0,
                            &format!(
                                "{}::{}(",
                                self.config.code.frame_event_args_type_name, exit_event_type_name
                            ),
                        );
                        arg_code.push(')');
                    }
                    None => self.errors.push(format!(
                        "Invalid number of arguments for \"{}\" event handler.",
                        exit_msg
                    )),
                }
            } else {
                self.warnings.push(format!("State {} does not have an exit event handler but is being passed parameters in a transition", current_state_name));
            }
        } else {
            arg_code.push_str(&format!(
                "{}::None",
                self.config.code.frame_event_args_type_name
            ));
        }
        has_args
    }

    /// Get the enter arguments from a transition statement and generate a
    /// value of the enter arguments struct. Writes the generated struct
    /// literal expression to `arg_code`. Returns `true` if any arguments were
    /// passed.
    fn generate_enter_arguments(
        &mut self,
        target_state_name: &str,
        state_context_node: &StateContextNode,
        arg_code: &mut String,
    ) -> bool {
        let mut has_args = false;
        if let Some(enter_args) = &state_context_node.enter_args_opt {
            // Search for event keyed with "State:>", e.g. "S1:>"
            let enter_msg = format!(
                "{}:{}",
                target_state_name, &self.symbol_config.enter_msg_symbol
            );
            let enter_event_type_name = self.format_enter_event_type_name(target_state_name);
            let arg_struct_name = self.format_args_struct_name(&enter_event_type_name);
            if let Some(event_sym) = self
                .arcanum
                .get_event(&enter_msg, &self.current_state_name_opt)
            {
                match &event_sym.borrow().params_opt {
                    Some(event_params) => {
                        let param_names = event_params.iter().map(|p| &p.name).collect();
                        has_args = self.generate_arguments(
                            &arg_struct_name,
                            param_names,
                            enter_args,
                            arg_code,
                        );
                        arg_code.insert_str(
                            0,
                            &format!(
                                "{}::{}(",
                                self.config.code.frame_event_args_type_name, enter_event_type_name
                            ),
                        );
                        arg_code.push(')');
                    }
                    None => {
                        self.errors.push(format!(
                            "The \"{}\" event handler was passed arguments, but it does not accept any.",
                            enter_msg
                        ));
                    }
                }
            } else {
                self.warnings.push(format!("State {} does not have an enter event handler but is being passed parameters in a transition", target_state_name));
            }
        } else {
            arg_code.push_str(&format!(
                "{}::None",
                self.config.code.frame_event_args_type_name
            ));
        }
        has_args
    }

    /// Get the arguments to the next state from a transition statement and
    /// generate a value of the corresponding arguments struct. Writes the
    /// generated struct literal expression to `arg_code`. Returns `true` if
    /// any arguments were passed.
    fn generate_state_arguments(
        &mut self,
        target_state_name: &str,
        state_args: &ExprListNode,
        arg_code: &mut String,
    ) -> bool {
        let mut has_args = false;
        let arg_struct_name = self.format_state_args_struct_name(&target_state_name.to_string());
        if let Some(state_sym) = self.arcanum.get_state(target_state_name) {
            match &state_sym.borrow().params_opt {
                Some(event_params) => {
                    let mut param_names = Vec::new();
                    for param in event_params {
                        let name = &param.borrow().name;
                        param_names.push(name.clone());
                    }
                    has_args = self.generate_arguments(
                        &arg_struct_name,
                        param_names.iter().collect(),
                        state_args,
                        arg_code
                    );
                }
                None => self.errors.push(format!(
                    "The \"{}\" state was passed arguments in a transition, but it does not accept any.",
                    target_state_name
                )),
            }
        } else {
            self.errors.push(format!(
                "Could not find state {}, which was passed arguments in a transition.",
                target_state_name
            ));
        }
        has_args
    }

    /// Generate the state variables for the next state after a transition or
    /// change-state. Writes the generated struct literal expression to
    /// `var_code`. Returns `true` if the next state contains any state
    /// variables.
    fn generate_state_variables(&mut self, target_state_name: &str, var_code: &mut String) -> bool {
        let mut has_vars = false;
        if let Some(state_symbol_rcref) = self.arcanum.get_state(target_state_name) {
            let state_symbol = state_symbol_rcref.borrow();
            let state_node = &state_symbol.state_node.as_ref().unwrap().borrow();
            // generate local state variables
            if state_node.vars_opt.is_some() {
                has_vars = true;
                var_code.push_str(&format!(
                    "{} {{",
                    self.format_state_vars_struct_name(target_state_name)
                ));
                self.indent();
                for var_rcref in state_node.vars_opt.as_ref().unwrap() {
                    let var = var_rcref.borrow();
                    let expr_t = var.initializer_expr_t_opt.as_ref().unwrap();
                    let mut expr_code = String::new();
                    expr_t.accept_to_string(self, &mut expr_code);
                    self.newline_to_string(var_code);
                    var_code.push_str(&format!(
                        "{}: {},",
                        self.format_value_name(&var.name),
                        expr_code
                    ));
                }
                self.outdent();
                self.newline_to_string(var_code);
                var_code.push('}');
            }
        }
        has_vars
    }

    /// Generate code that initializes a new state context value stored in a
    /// local variable named `next_state_context`.
    fn generate_next_state_context(
        &mut self,
        target_state_name: &str,
        has_state_args: bool,
        has_state_vars: bool,
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
            self.add_code(&format!(
                "{}: {},",
                self.config.code.state_args_var_name, state_args
            ));
        }
        if has_state_vars {
            self.newline();
            self.add_code(&format!(
                "{}: {},",
                self.config.code.state_vars_var_name, state_vars
            ));
        }
        self.outdent();
        self.newline();
        self.add_code("};");
        self.newline();
        self.add_code(&format!(
            "let next_state_context = Rc::new({}::{}(RefCell::new(context)));",
            self.config.code.state_context_type_name,
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

        self.indent();

        // generate state arguments
        let mut has_state_args = false;
        let mut state_args_code = String::new();
        match &change_state_stmt.state_context_t {
            StateContextType::StateRef { state_context_node } => {
                if let Some(state_args) = &state_context_node.state_ref_args_opt {
                    has_state_args = self.generate_state_arguments(
                        target_state_name,
                        state_args,
                        &mut state_args_code,
                    );
                }
            }
            StateContextType::StateStackPop {} => {}
        };

        // generate state variables
        let mut state_vars_code = String::new();
        let has_state_vars = self.generate_state_variables(target_state_name, &mut state_vars_code);

        self.outdent();

        // generate new state context
        if self.generate_state_context {
            self.generate_next_state_context(
                target_state_name,
                has_state_args,
                has_state_vars,
                &state_args_code,
                &state_vars_code,
            );
            self.newline();
            self.add_code(&format!(
                "drop({});",
                self.config.code.this_state_context_var_name
            ));
        }

        // call the change-state method
        self.newline();
        if self.generate_state_context {
            self.add_code(&format!(
                "self.{}({}::{}, next_state_context);",
                self.config.code.change_state_method_name,
                self.state_enum_type_name(),
                self.format_type_name(&target_state_name.to_string())
            ));
        } else {
            self.add_code(&format!(
                "self.{}({}::{});",
                self.config.code.change_state_method_name,
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
        if self.generate_exit_args {
            let mut exit_args_code = String::new();
            self.generate_exit_arguments(transition_stmt, &mut exit_args_code);
            self.newline();
            self.add_code(&format!(
                "let {} = {};",
                self.config.code.exit_args_member_name, exit_args_code
            ));
        }

        // generate enter arguments
        if self.generate_enter_args {
            let mut has_enter_args = false;
            let mut enter_args_code = String::new();
            match &transition_stmt.target_state_context_t {
                StateContextType::StateRef { state_context_node } => {
                    has_enter_args = self.generate_enter_arguments(
                        target_state_name,
                        state_context_node,
                        &mut enter_args_code,
                    );
                }
                StateContextType::StateStackPop {} => {}
            }
            if !has_enter_args {
                enter_args_code = format!("{}::None", self.config.code.frame_event_args_type_name);
            }
            self.newline();
            self.add_code(&format!(
                "let {} = {};",
                self.config.code.enter_args_member_name, enter_args_code
            ));
        }

        // indent to generate parts of context at the right indentation level
        self.indent();

        // generate state arguments
        let mut has_state_args = false;
        let mut state_args_code = String::new();
        match &transition_stmt.target_state_context_t {
            StateContextType::StateRef { state_context_node } => {
                if let Some(state_args) = &state_context_node.state_ref_args_opt {
                    has_state_args = self.generate_state_arguments(
                        target_state_name,
                        state_args,
                        &mut state_args_code,
                    );
                }
            }
            StateContextType::StateStackPop {} => {}
        };

        // generate state variables
        let mut state_vars_code = String::new();
        let has_state_vars = self.generate_state_variables(target_state_name, &mut state_vars_code);

        // end indent for parts of context
        self.outdent();

        // generate new state context
        if self.generate_state_context {
            self.generate_next_state_context(
                target_state_name,
                has_state_args,
                has_state_vars,
                &state_args_code,
                &state_vars_code,
            );
            self.newline();
            self.add_code(&format!(
                "drop({});",
                self.config.code.this_state_context_var_name
            ));
        }

        // call the transition method
        self.newline();
        self.add_code(&format!(
            "self.{}(",
            self.config.code.transition_method_name
        ));
        if self.generate_exit_args {
            self.add_code(&format!("{}, ", self.config.code.exit_args_member_name));
        }
        if self.generate_enter_args {
            self.add_code(&format!("{}, ", self.config.code.enter_args_member_name));
        }
        self.add_code(&format!(
            "{}::{}",
            self.state_enum_type_name(),
            self.format_type_name(&target_state_name.to_string())
        ));
        if self.generate_state_context {
            self.add_code(", next_state_context");
        }
        self.add_code(");");
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
                self.config.code.this_state_context_var_name
            ));
            self.newline();
            self.add_code(&format!(
                "let (next_state, next_state_context) = self.{}();",
                self.config.code.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state, next_state_context);",
                self.config.code.change_state_method_name
            ));
        } else {
            self.add_code(&format!(
                "let next_state = self.{}();",
                self.config.code.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state);",
                self.config.code.change_state_method_name
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
                self.config.code.this_state_context_var_name
            ));
            self.newline();
            self.add_code(&format!(
                "let (next_state, next_state_context) = self.{}();",
                self.config.code.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state, next_state_context);",
                self.config.code.transition_method_name
            ));
        } else {
            self.add_code(&format!(
                "let next_state = self.{}();",
                self.config.code.state_stack_pop_method_name
            ));
            self.newline();
            self.add_code(&format!(
                "self.{}(next_state);",
                self.config.code.transition_method_name
            ));
        }
    }
}

//* --------------------------------------------------------------------- *//

impl AstVisitor for RustVisitor {
    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) {
        self.system_name = system_node.name.clone();
        self.add_code(&format!("// {}", self.compiler_version));
        self.newline();
        self.add_code(&system_node.header);
        self.newline();
        self.add_code("#[allow(unused_imports)]");
        self.newline();
        self.add_code("use std::cell::{Ref, RefCell};");
        self.newline();
        self.add_code("#[allow(unused_imports)]");
        self.newline();
        self.add_code("use std::rc::Rc;");
        self.newline();
        if self.config.features.runtime_support {
            self.add_code("#[allow(unused_imports)]");
            self.newline();
            self.add_code("use std::any::Any;");
            self.newline();
            self.add_code("use frame_runtime::callback::CallbackManager;");
            self.newline();
            self.add_code("#[allow(unused_imports)]");
            self.newline();
            self.add_code("use frame_runtime::environment::{Environment, EMPTY};");
            self.newline();
            self.add_code("use frame_runtime::machine::StateMachine;");
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
        self.newline();
        self.add_code(&format!(
            "enum {} {{",
            self.config.code.frame_event_return_type_name
        ));
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
        self.add_code("#[allow(clippy::clone_on_copy)]");
        self.newline();
        self.disable_type_style_warnings();
        self.add_code(&format!(
            "impl {} {{",
            self.config.code.frame_event_return_type_name
        ));
        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            for interface_method_node in &interface_block_node.interface_methods {
                //let if_name = interface_method_node.name.clone();
                if let Some(return_type) = &interface_method_node.borrow().return_type_opt {
                    self.indent();
                    self.newline();
                    self.add_code(&format!(
                        "fn {}(&self) -> {} {{",
                        self.format_param_getter(&interface_method_node.borrow().name, "ret"),
                        return_type.get_type_str()
                    ));
                    self.indent();
                    self.newline();
                    self.add_code(&"match self {".to_string());
                    self.indent();
                    self.newline();
                    self.add_code(&format!(
                        "{}::{} {{ return_value }} => return_value.clone(),",
                        self.config.code.frame_event_return_type_name,
                        self.format_type_name(&interface_method_node.borrow().name)
                    ));
                    self.newline();
                    self.add_code(&"_ => panic!(\"Invalid return value\"),".to_string());
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
        self.disable_type_style_warnings();
        self.add_code(&format!(
            "pub struct {}{} {{",
            self.config.code.frame_event_type_name,
            self.lifetime_type_annotation()
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "{}: {},",
            self.config.code.frame_event_message_attribute_name,
            self.config.code.frame_event_message_type_name
        ));
        self.newline();
        self.add_code(&format!(
            "{}: {}{},",
            self.config.code.frame_event_args_attribute_name,
            self.lifetime_ref_annotation(),
            self.config.code.frame_event_args_type_name
        ));
        self.newline();
        self.add_code(&format!(
            "{}: {},",
            self.config.code.frame_event_return_attribute_name,
            self.config.code.frame_event_return_type_name
        ));
        self.outdent();
        self.newline();
        self.add_code("}");
        self.newline();
        self.newline();
        self.disable_type_style_warnings();
        self.add_code(&format!(
            "impl{0} {1}{0} {{",
            self.lifetime_type_annotation(),
            self.config.code.frame_event_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "fn new({}: {}, {}: {}{}) -> {}{} {{",
            self.config.code.frame_event_message_attribute_name,
            self.config.code.frame_event_message_type_name,
            self.config.code.frame_event_args_attribute_name,
            self.lifetime_ref_annotation(),
            self.config.code.frame_event_args_type_name,
            self.config.code.frame_event_type_name,
            self.lifetime_type_annotation()
        ));
        self.indent();
        self.newline();
        self.add_code(&format!("{} {{", self.config.code.frame_event_type_name));
        self.indent();
        self.newline();
        self.add_code(&format!(
            "{},",
            self.config.code.frame_event_message_attribute_name
        ));
        self.newline();
        self.add_code(&format!(
            "{},",
            self.config.code.frame_event_args_attribute_name
        ));
        self.newline();
        self.add_code(&format!(
            "ret: {}::None,",
            self.config.code.frame_event_return_type_name
        ));
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
        self.generate_state_enum(system_node);
        self.newline();
        self.newline();
        self.generate_event_arg_defs();

        if self.generate_state_context {
            self.generate_state_context_defs(system_node);
            self.newline();
            self.newline();
        }

        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            actions_block_node.accept_rust_trait(self);
            self.newline();
            self.newline();
        } else if self.config.features.generate_hook_methods {
            let empty_actions_block_node = ActionsBlockNode {
                actions: Vec::new(),
            };
            empty_actions_block_node.accept_rust_trait(self);
            self.newline();
            self.newline();
        }

        // define state machine struct
        self.add_code("// System Controller ");
        self.newline();
        self.disable_type_style_warnings();
        self.add_code(&format!(
            "pub struct {}{}",
            self.system_type_name(),
            self.lifetime_type_annotation()
        ));
        self.enter_block();

        // generate state variable
        self.add_code(&format!(
            "{}: {},",
            &self.config.code.state_var_name,
            self.state_enum_type_name()
        ));

        // generate state context variable
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "{}: Rc<{}>,",
                self.config.code.state_context_var_name, self.config.code.state_context_type_name
            ));
        }

        // generate state stack variable
        if self.generate_state_stack {
            self.newline();
            if self.generate_state_context {
                self.add_code(&format!(
                    "{}: Vec<({}, Rc<{}>)>,",
                    self.config.code.state_stack_var_name,
                    self.state_enum_type_name(),
                    self.config.code.state_context_type_name
                ));
            } else {
                self.add_code(&format!(
                    "{}: Vec<{}>,",
                    self.config.code.state_stack_var_name,
                    self.state_enum_type_name()
                ));
            }
        }

        // generate runtime support variables
        if self.config.features.runtime_support {
            if !self.generate_state_context {
                self.newline();
                self.add_code(&format!(
                    "{}: RefCell<{}>,",
                    self.config.code.state_cell_var_name,
                    self.state_enum_type_name()
                ));
            }
            self.newline();
            self.add_code(&format!(
                "{}: CallbackManager<'a>,",
                self.config.code.callback_manager_var_name
            ));
        }

        // generate domain variables
        let mut domain_vars: Vec<String> = Vec::new();
        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            domain_block_node.accept(self);
            domain_vars = domain_block_node
                .member_variables
                .iter()
                .map(|decl_rc| self.format_value_name(&decl_rc.borrow().name))
                .collect();
        }

        self.exit_block();
        self.newline();
        self.newline();

        // add runtime support
        if self.config.features.runtime_support {
            self.generate_environment_impl(true, &self.system_type_name(), domain_vars);

            self.add_code(&format!(
                "impl{0} StateMachine{0} for {1}{0}",
                self.lifetime_type_annotation(),
                self.system_type_name()
            ));
            self.enter_block();

            self.add_code("fn current_state(&self) -> Ref<dyn State>");
            self.enter_block();
            if self.generate_state_context {
                self.add_code(&format!(
                    "self.{}.as_ref().as_runtime_state()",
                    self.config.code.state_context_var_name
                ));
            } else {
                self.add_code(&format!(
                    "Ref::map(self.{}.borrow(), |s| s as &dyn State)",
                    self.config.code.state_cell_var_name
                ));
            }
            self.exit_block();
            self.newline();

            self.add_code("fn domain_variables(&self) -> &dyn Environment");
            self.enter_block();
            self.add_code("self");
            self.exit_block();
            self.newline();

            self.add_code("fn callback_manager(&mut self) -> &mut CallbackManager<'a>");
            self.enter_block();
            self.add_code(&format!(
                "&mut self.{}",
                self.config.code.callback_manager_var_name
            ));
            self.exit_block();

            self.exit_block();
            self.newline();
            self.newline();
        }

        // add state machine methods
        self.disable_all_style_warnings();
        self.add_code(&format!(
            "impl{0} {1}{0} {{",
            self.lifetime_type_annotation(),
            self.system_type_name()
        ));
        self.indent();

        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        if let Some(first_state) = system_node.get_first_state() {
            self.first_state_name = first_state.borrow().name.clone();
            self.has_states = true;
        }

        if self.has_states {
            self.newline();
            self.newline();
            self.generate_constructor(system_node);
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
        } else if self.config.features.generate_hook_methods {
            let empty_actions_block_node = ActionsBlockNode {
                actions: Vec::new(),
            };
            empty_actions_block_node.accept_rust_impl(self);
            self.newline();
            self.newline();
        }

        // self.generate_subclass();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_frame_messages_enum(&mut self, _interface_block_node: &InterfaceBlockNode) {
        self.newline();
        self.disable_type_style_warnings();
        self.add_code(&format!(
            "enum {} {{",
            self.config.code.frame_event_message_type_name
        ));
        self.indent();
        self.newline();
        self.add_code(&format!("{},", self.config.code.enter_msg));
        self.newline();
        self.add_code(&format!("{},", self.config.code.exit_msg));

        let events = self.arcanum.get_event_names();
        for event in &events {
            //    ret.push(k.clone());
            if self.is_enter_or_exit_message(event) {
                continue;
            }
            let message_opt = self.arcanum.get_interface_or_msg_from_msg(event);
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
        self.disable_type_style_warnings();
        self.add_code(&format!(
            "impl std::fmt::Display for {} {{",
            self.config.code.frame_event_message_type_name
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
            self.config.code.frame_event_message_type_name,
            self.config.code.enter_msg,
            self.config.code.enter_msg
        ));
        self.newline();
        self.add_code(&format!(
            "{}::{} => write!(f, \"{}\"),",
            self.config.code.frame_event_message_type_name,
            self.config.code.exit_msg,
            self.config.code.exit_msg
        ));
        for event in &events {
            //    ret.push(k.clone());
            if self.is_enter_or_exit_message(event) {
                continue;
            }
            let message_opt = self.arcanum.get_interface_or_msg_from_msg(event);
            match message_opt {
                Some(canonical_message_name) => {
                    let formatted_message_name = self.format_type_name(&canonical_message_name);
                    self.newline();
                    self.add_code(&format!(
                        "{}::{} => write!(f, \"{}\"),",
                        self.config.code.frame_event_message_type_name,
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
    ) {
        self.add_code(&format!(
            "self.{}",
            self.format_value_name(&interface_method_call_expr_node.identifier.name.lexeme)
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
            "self.{}",
            self.format_value_name(&interface_method_call_expr_node.identifier.name.lexeme)
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
        self.add_code(&format!(
            "pub fn {}(&mut self",
            self.format_value_name(&interface_method_node.name)
        ));

        match &interface_method_node.params {
            Some(params) => {
                self.format_parameter_list(params);
            }
            None => {}
        }

        self.add_code(")");
        match &interface_method_node.return_type_opt {
            Some(return_type) => {
                self.add_code(&format!(" -> {}", return_type.get_type_str()));
            }
            None => {}
        }
        self.enter_block();

        let event_type_name = self.format_type_name(&interface_method_node.name);
        self.add_code(&format!(
            "let frame_args = {}::",
            self.config.code.frame_event_args_type_name
        ));
        if interface_method_node.params.is_some() {
            self.add_code(&format!(
                "{}({} {{ ",
                event_type_name,
                self.format_args_struct_name(&event_type_name)
            ));
            match &interface_method_node.params {
                Some(params) => {
                    for param in params {
                        self.add_code(&format!("{}, ", self.format_value_name(&param.param_name)));
                    }
                }
                None => {}
            }
            self.add_code("});");
        } else {
            self.add_code("None;")
        }

        self.newline();
        self.add_code(&format!(
            "let mut {} = {}::new({}::{}, {}frame_args);",
            self.config.code.frame_event_variable_name,
            self.config.code.frame_event_type_name,
            self.config.code.frame_event_message_type_name,
            event_type_name,
            self.ref_if_runtime_support()
        ));

        self.newline();
        self.add_code(&format!(
            "self.{}(&mut {});",
            self.config.code.handle_event_method_name, self.config.code.frame_event_variable_name,
        ));

        match &interface_method_node.return_type_opt {
            Some(_return_type) => {
                self.newline();
                self.add_code(&format!(
                    "match {}.{} {{",
                    self.config.code.frame_event_variable_name,
                    self.config.code.frame_event_return_attribute_name
                ));
                self.indent();
                self.newline();
                self.add_code(&format!(
                    "{}::{} {{ return_value }} => return_value.clone(),",
                    self.config.code.frame_event_return_type_name,
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
        self.add_code(&"}".to_string());
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_machine_block_node(&mut self, machine_block_node: &MachineBlockNode) {
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_block_node(&mut self, _: &ActionsBlockNode) {
        panic!("Error - visit_actions_block_node() not called for Rust.");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_node_rust_trait(&mut self, actions_block_node: &ActionsBlockNode) {
        if self.config.features.generate_action_impl {
            self.add_code("#[allow(clippy::ptr_arg)]");
            self.newline();
            self.disable_type_style_warnings();
            self.add_code(&format!(
                "trait {}{} {{ ",
                self.action_trait_type_name(),
                self.lifetime_type_annotation()
            ));
            self.indent();

            // add action signatures
            for action_decl_node_rcref in &actions_block_node.actions {
                let action_decl_node = action_decl_node_rcref.borrow();
                action_decl_node.accept(self);
            }

            // add hook signatures
            let old_state_var = self.old_var_name(&self.config.code.state_var_name);
            let new_state_var = self.new_var_name(&self.config.code.state_var_name);
            if self.generate_transition_hook {
                self.newline();
                self.add_code(&format!(
                    "fn {}(&self, {}: {enum_type}, {}: {enum_type});",
                    self.config.code.transition_hook_method_name,
                    old_state_var,
                    new_state_var,
                    enum_type = self.state_enum_type_name()
                ));
            }
            if self.generate_change_state_hook {
                self.newline();
                self.add_code(&format!(
                    "fn {}(&self, {}: {enum_type}, {}: {enum_type});",
                    self.config.code.change_state_hook_method_name,
                    old_state_var,
                    new_state_var,
                    enum_type = self.state_enum_type_name()
                ));
            }

            self.outdent();
            self.newline();
            self.add_code("}");
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_node_rust_impl(&mut self, actions_block_node: &ActionsBlockNode) {
        if self.config.features.generate_action_impl {
            self.newline();
            self.disable_all_style_warnings();
            self.add_code(&format!(
                "impl{0} {1}{0} for {2}{0} {{ ",
                self.lifetime_type_annotation(),
                self.action_trait_type_name(),
                self.system_type_name()
            ));
            self.indent();

            // add action implementations
            for action_decl_node_rcref in &actions_block_node.actions {
                let action_decl_node = action_decl_node_rcref.borrow();
                action_decl_node.accept_rust_impl(self);
            }

            // add empty hook implementations
            let old_state_var = self.old_var_name(&self.config.code.state_var_name);
            let new_state_var = self.new_var_name(&self.config.code.state_var_name);
            if self.generate_transition_hook {
                self.newline();
                self.add_code(&format!(
                    "fn {}(&self, {}: {enum_type}, {}: {enum_type}) {{}}",
                    self.config.code.transition_hook_method_name,
                    old_state_var,
                    new_state_var,
                    enum_type = self.state_enum_type_name()
                ));
            }
            if self.generate_change_state_hook {
                self.newline();
                self.add_code(&format!(
                    "fn {}(&self, {}: {enum_type}, {}: {enum_type}) {{}}",
                    self.config.code.change_state_hook_method_name,
                    old_state_var,
                    new_state_var,
                    enum_type = self.state_enum_type_name()
                ));
            }

            self.outdent();
            self.newline();
            self.add_code("}");
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_block_node(&mut self, domain_block_node: &DomainBlockNode) {
        self.newline();
        self.newline();
        self.add_code("//===================== Domain Block ===================//");
        self.newline();

        for variable_decl_node_rcref in &domain_block_node.member_variables {
            let variable_decl_node = variable_decl_node_rcref.borrow();
            variable_decl_node.accept_rust_domain_var_decl(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_node(&mut self, state_node: &StateNode) {
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
            self.config.code.frame_event_variable_name,
            self.config.code.frame_event_type_name
        ));
        self.indent();
        let state_name = &self.current_state_name_opt.as_ref().unwrap().clone();
        if self.generate_state_context {
            self.newline();
            self.add_code(&format!(
                "let {0}_clone = Rc::clone(&self.{0});",
                self.config.code.state_context_var_name
            ));
            self.newline();
            self.add_code(&format!(
                "let mut {} = {}_clone.{}().borrow_mut();",
                self.config.code.this_state_context_var_name,
                self.config.code.state_context_var_name,
                self.format_state_context_method_name(state_name)
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
            self.config.code.frame_event_variable_name,
            self.config.code.frame_event_message_attribute_name
        ));
        self.indent();

        if !state_node.evt_handlers_rcref.is_empty() {
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_node(&mut self, evt_handler_node: &EventHandlerNode) {
        self.current_event_ret_type = evt_handler_node.get_event_ret_type();
        self.newline();
        self.generate_comment(evt_handler_node.line);
        //        let mut generate_final_close_paren = true;
        if let MessageType::CustomMessage { message_node } = &evt_handler_node.msg_t {
            self.current_message = message_node.name.clone();
            self.add_code(&format!(
                "{}::{} => {{",
                self.config.code.frame_event_message_type_name,
                self.get_msg_enum(&message_node.name)
            ));
        } else {
            // AnyMessage ( ||* )
            // This feature requires dynamic dispatch.
            panic!("||* not supported for Rust.");
        }
        self.generate_comment(evt_handler_node.line);
        self.indent();

        if let MessageType::CustomMessage { .. } = &evt_handler_node.msg_t {
            // Note: this is a bit convoluted as we cant use self.add_code() inside the
            // if statements as it is a double borrow (sigh).

            let params_code: Vec<String> = Vec::new();

            // NOW add the code. Sheesh.
            for param_code in params_code {
                self.newline();
                self.add_code(&param_code);
            }
        }

        // Generate statements
        self.visit_decl_stmts(&evt_handler_node.statements);

        let terminator_node = &evt_handler_node.terminator_node;
        terminator_node.accept(self);
        self.outdent();
        self.newline();
        self.add_code(&"}".to_string());

        // this controls formatting here
        self.current_message = String::new();
        self.current_event_ret_type = String::new();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_terminator_node(
        &mut self,
        evt_handler_terminator_node: &TerminatorExpr,
    ) {
        match &evt_handler_terminator_node.terminator_type {
            TerminatorType::Return => {
                match &evt_handler_terminator_node.return_expr_t_opt {
                    Some(expr_t) => {
                        self.newline();
                        self.add_code(&format!(
                            "{}.{} = ",
                            self.config.code.frame_event_variable_name,
                            self.config.code.frame_event_return_attribute_name
                        ));
                        self.add_code(&format!(
                            "{}::{} {{ return_value: ",
                            self.config.code.frame_event_return_type_name,
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_statement_node(&mut self, method_call_statement: &CallStmtNode) {
        self.newline();
        method_call_statement.call_expr_node.accept(self);
        self.add_code(&";".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node(&mut self, method_call: &CallExprNode) {
        if let Some(call_chain) = &method_call.call_chain {
            for callable in call_chain {
                callable.callable_accept(self);
                self.add_code(&".".to_string());
            }
        }

        self.add_code(&method_call.identifier.name.lexeme.to_string());

        method_call.call_expr_list.accept(self);

        self.add_code(&format!(""));
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
                output.push_str(&".".to_string());
            }
        }

        output.push_str(&method_call.identifier.name.lexeme.to_string());

        method_call.call_expr_list.accept_to_string(self, output);

        output.push_str(&format!(""));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node(&mut self, call_expr_list: &CallExprListNode) {
        let mut separator = "";
        self.add_code(&"(".to_string());

        for expr in &call_expr_list.exprs_t {
            self.add_code(&separator.to_string());
            expr.accept(self);
            separator = ",";
        }

        self.add_code(&")".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expr_list_node_to_string(
        &mut self,
        call_expr_list: &CallExprListNode,
        output: &mut String,
    ) {
        let mut separator = "";
        output.push_str(&"(".to_string());

        for expr in &call_expr_list.exprs_t {
            output.push_str(&separator.to_string());
            expr.accept_to_string(self, output);
            separator = ",";
        }

        output.push_str(&")".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node(&mut self, action_call: &ActionCallExprNode) {
        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        self.add_code(&format!("self.{}", action_name));
        action_call.call_expr_list.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(
        &mut self,
        action_call: &ActionCallExprNode,
        output: &mut String,
    ) {
        let action_name = self.format_action_name(&action_call.identifier.name.lexeme);
        output.push_str(&format!("self.{}", action_name));
        action_call.call_expr_list.accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(&mut self, action_call_stmt_node: &ActionCallStmtNode) {
        self.newline();
        action_call_stmt_node.action_call_expr_node.accept(self);
        self.add_code(&";".to_string());
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
        self.this_branch_transitioned = true;
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
        self.this_branch_transitioned = true;
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
            "self.{}({});",
            self.format_state_handler_name(&dispatch_node.target_state_ref.name),
            self.config.code.frame_event_variable_name
        ));
        self.generate_comment(dispatch_node.line);
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
                self.add_code(&if_or_else_if.to_string());
            }

            branch_node.expr_t.accept(self);

            if branch_node.is_negated {
                self.add_code(&")".to_string());
            }
            self.add_code(&" {".to_string());
            self.indent();

            branch_node.accept(self);

            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code(&"}".to_string());

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
                self.this_branch_transitioned = true;
                if self.generate_state_context {
                    self.add_code(&format!(
                        "drop({});",
                        self.config.code.this_state_context_var_name
                    ));
                    self.newline();
                }
                interface_method_call_expr_node.accept(self);
                self.add_code(";");
                self.newline();
                self.add_code("return;");
                return;
            }
        }

        // standard case
        method_call_chain_literal_stmt_node
            .call_chain_literal_expr_node
            .accept(self);
        self.add_code(&";".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_literal_expr_node(
        &mut self,
        method_call_chain_expression_node: &CallChainLiteralExprNode,
    ) {
        // TODO: maybe put this in an AST node
        let mut separator = "";

        for node in &method_call_chain_expression_node.call_chain {
            self.add_code(&separator.to_string());
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
            output.push_str(&separator.to_string());
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
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&"e._return = ".to_string());
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_else_branch_node(
        &mut self,
        bool_test_else_branch_node: &BoolTestElseBranchNode,
    ) {
        self.add_code(&" else {".to_string());
        self.indent();

        self.visit_decl_stmts(&bool_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &bool_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code(&"e._return = ".to_string());
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
        self.add_code(&"}".to_string());
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
                    self.add_code(&format!(".eq(\"{}\")", match_string));
                    first_match = false;
                } else {
                    self.add_code(&" || ".to_string());
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
                    self.add_code(&format!(".eq(\"{}\")", match_string));
                }
            }
            self.add_code(&" {".to_string());
            self.indent();

            match_branch_node.accept(self);

            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code(&"}".to_string());

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
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&"e._return = ".to_string());
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_else_branch_node(
        &mut self,
        string_match_test_else_branch_node: &StringMatchTestElseBranchNode,
    ) {
        self.add_code(&" else {".to_string());
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
                                self.add_code(&"e._return = ".to_string());
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
        self.add_code(&"}".to_string());
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
                    self.add_code(&" || ".to_string());
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

            self.add_code(&" {".to_string());
            self.indent();

            match_branch_node.accept(self);

            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();
            self.add_code(&"}".to_string());

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
                    TerminatorType::Return => {
                        match &branch_terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code(&"e._return = ".to_string());
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_else_branch_node(
        &mut self,
        number_match_test_else_branch_node: &NumberMatchTestElseBranchNode,
    ) {
        self.add_code(&" else {".to_string());
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
                                self.add_code(&"e._return = ".to_string());
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
        self.add_code(&"}".to_string());
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
        self.add_code(&"(".to_string());
        for expr in &expr_list.exprs_t {
            self.add_code(&separator.to_string());
            expr.accept(self);
            separator = ",";
        }
        self.add_code(&")".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node_to_string(
        &mut self,
        expr_list: &ExprListNode,
        output: &mut String,
    ) {
        //        self.add_code(&format!("{}(e);\n",dispatch_node.target_state_ref.name));

        let mut separator = "";
        output.push_str(&"(".to_string());
        for expr in &expr_list.exprs_t {
            output.push_str(&separator.to_string());
            expr.accept_to_string(self, output);
            separator = ",";
        }
        output.push_str(&")".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node(&mut self, literal_expression_node: &LiteralExprNode) {
        match &literal_expression_node.token_t {
            TokenType::Number => self.add_code(&literal_expression_node.value.to_string()),
            TokenType::SuperString => self.add_code(&literal_expression_node.value.to_string()),
            TokenType::String => {
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
            TokenType::True => self.add_code("true"),
            TokenType::False => self.add_code("false"),
            TokenType::Null => self.add_code("null"),
            TokenType::Nil => self.add_code("null"),
            // TokenType::SuperString => {
            //     self.add_code(&format!("{}", literal_expression_node.value));
            // },
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
                output.push_str(&format!(
                    "String::from(\"{}\")",
                    literal_expression_node.value
                ));
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
                self.add_code(&format!(
                    "self.{}();",
                    self.config.code.state_stack_push_method_name
                ));
            }
            StateStackOperationType::Pop => {
                if self.generate_state_context {
                    self.add_code(&format!(
                        "let {} = self.{}();",
                        self.config.code.state_context_var_name,
                        self.config.code.state_stack_pop_method_name
                    ));
                    self.add_code(&format!(
                        "let state = {}.borrow().get_state();",
                        self.config.code.state_context_var_name
                    ));
                } else {
                    self.add_code(&format!(
                        "let state = self.{}();",
                        self.config.code.state_stack_pop_method_name
                    ));
                }
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
            FrameEventPart::Event { is_reference } => self.add_code(&format!(
                "{}{}",
                if *is_reference { "&" } else { "" },
                self.config.code.frame_event_variable_name
            )),
            FrameEventPart::Message { is_reference } => self.add_code(&format!(
                "{}{}.{}.to_string()",
                if *is_reference { "&" } else { "" },
                self.config.code.frame_event_variable_name,
                self.config.code.frame_event_message_attribute_name
            )),
            // FrameEventPart::Param {param_tok} => self.add_code(&format!("{}._parameters[\"{}\"]"
            //                                                             ,self.config.code.frame_event_variable_name
            FrameEventPart::Param {
                param_tok,
                is_reference,
            } => {
                let event_name = self.format_event_type_name(&self.current_message);
                self.add_code(&format!(
                    "{}{}.{}.{}().{}",
                    if *is_reference { "&" } else { "" },
                    self.config.code.frame_event_variable_name,
                    self.config.code.frame_event_args_attribute_name,
                    self.format_args_method_name(&event_name),
                    self.format_value_name(&param_tok.lexeme)
                ));
            }
            FrameEventPart::Return { is_reference } => {
                self.add_code(&format!(
                    "{}{}.{}.{}()",
                    if *is_reference { "&" } else { "" },
                    self.config.code.frame_event_variable_name,
                    self.config.code.frame_event_return_attribute_name,
                    self.format_param_getter(&self.current_message, "ret")
                ));
                // self.add_code(&format!("{}{}.{}"
                //                         ,if *is_reference {"&"} else {""}
                //                        ,self.config.code.frame_event_variable_name
                //                        ,self.config.code.frame_event_return_attribute_name))
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
            FrameEventPart::Event { is_reference } => output.push_str(&format!(
                "{}{}",
                if *is_reference { "&" } else { "" },
                self.config.code.frame_event_variable_name
            )),
            FrameEventPart::Message { is_reference } => output.push_str(&format!(
                "{}{}.{}.to_string()",
                if *is_reference { "&" } else { "" },
                self.config.code.frame_event_variable_name,
                self.config.code.frame_event_message_attribute_name
            )),
            FrameEventPart::Param {
                param_tok,
                is_reference,
            } => {
                let event_name = self.format_event_type_name(&self.current_message);
                output.push_str(&format!(
                    "{}{}.{}.{}().{}",
                    if *is_reference { "&" } else { "" },
                    self.config.code.frame_event_variable_name,
                    self.config.code.frame_event_args_attribute_name,
                    self.format_args_method_name(&event_name),
                    self.format_value_name(&param_tok.lexeme)
                ));
            }
            FrameEventPart::Return { is_reference } => output.push_str(&format!(
                "{}{}.{}.{}()",
                if *is_reference { "&" } else { "" },
                self.config.code.frame_event_variable_name,
                self.config.code.frame_event_return_attribute_name,
                self.format_param_getter(&self.current_message, "ret")
            )),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_decl_node(&mut self, action_decl_node: &ActionNode) {
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_impl_node(&mut self, action_node: &ActionNode) {
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
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
                    self.config.code.frame_event_variable_name,
                    self.config.code.frame_event_return_attribute_name
                ));

                self.add_code(&format!(
                    "{}::{} {{ return_value: {}}};",
                    self.config.code.frame_event_return_type_name,
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node_to_string(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
        output: &mut String,
    ) {
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
        output.push(';');
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

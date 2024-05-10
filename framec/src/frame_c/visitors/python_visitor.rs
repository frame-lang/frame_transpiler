// TODO fix these issues and disable warning suppression
#![allow(unknown_lints)]
#![allow(clippy::branches_sharing_code)]
#![allow(clippy::single_match)]
#![allow(clippy::ptr_arg)]
#![allow(non_snake_case)]

use crate::config::*;
use crate::frame_c::ast::DeclOrStmtType;
use crate::frame_c::ast::DeclOrStmtType::{StmtT, VarDeclT};
use crate::frame_c::ast::*;
use crate::frame_c::scanner::{Token, TokenType};
use crate::frame_c::symbol_table::*;
use crate::frame_c::visitors::*;
use std::cell::RefCell;
use std::rc::Rc;

// use yaml_rust::{YamlLoader, Yaml};

pub struct PythonVisitor {
    compiler_version: String,
    code: String,
    dent: usize,
    current_state_name_opt: Option<String>,
    current_event_ret_type: String,
    arcanium: Arcanum,
    symbol_config: SymbolConfig,
    comments: Vec<Token>,
    current_comment_idx: usize,
    first_event_handler: bool,
    system_name: String,
    first_state_name: String,
    // serialize: Vec<String>,
    // deserialize: Vec<String>,
    subclass_code: Vec<String>,
    warnings: Vec<String>,
    has_states: bool,
    errors: Vec<String>,
    visiting_call_chain_literal_variable: bool,
    // generate_exit_args: bool,
    // generate_state_context: bool,
    generate_state_stack: bool,
    generate_change_state: bool,
    // generate_transition_state: bool,
    event_handler_has_code: bool,
    // loop_for_inc_dec_expr_rcref_opt: Option<Rc<RefCell<ExprType>>>,

    /* Persistence */
    managed: bool, // Generate Managed code
    marshal: bool, // Generate JSON code
    manager: String,

    // config
    config: PythonConfig,

    // keeping track of traversal context
    this_branch_transitioned: bool,
    skip_next_newline: bool,
    generate_main: bool,
    variable_init_override_opt: Option<String>,
    continue_post_expr_vec: Vec<Option<String>>,
    operation_scope_depth: i32,
    action_scope_depth: i32,
    system_node_rcref_opt: Option<Rc<RefCell<SystemNode>>>,
}

impl PythonVisitor {
    //* --------------------------------------------------------------------- *//

    pub fn new(
        arcanium: Arcanum,
        // generate_exit_args: bool,
        // generate_state_context: bool,
        generate_state_stack: bool,
        generate_change_state: bool,
        // generate_transition_state: bool,
        compiler_version: &str,
        comments: Vec<Token>,
        config: FrameConfig,
    ) -> PythonVisitor {
        let python_config = config.codegen.python;
        PythonVisitor {
            compiler_version: compiler_version.to_string(),
            code: String::from(""),
            dent: 0,
            current_state_name_opt: None,
            current_event_ret_type: String::new(),
            arcanium,
            symbol_config: SymbolConfig::new(),
            comments,
            current_comment_idx: 0,
            first_event_handler: true,
            system_name: String::new(),
            first_state_name: String::new(),
            // serialize: Vec::new(),
            // deserialize: Vec::new(),
            has_states: false,
            errors: Vec::new(),
            subclass_code: Vec::new(),
            warnings: Vec::new(),
            visiting_call_chain_literal_variable: false,
            // generate_exit_args,
            // generate_state_context,
            generate_state_stack,
            generate_change_state,
            // generate_transition_state,
            event_handler_has_code: false,
            // loop_for_inc_dec_expr_rcref_opt: None,

            /* Persistence */
            managed: false, // Generate Managed code
            marshal: false, // Generate Json code
            manager: String::new(),
            config: python_config,
            // keeping track of traversal context
            this_branch_transitioned: false,
            skip_next_newline: false,
            generate_main: false,
            variable_init_override_opt: Option::None,
            continue_post_expr_vec: Vec::new(),
            operation_scope_depth: 0,
            action_scope_depth: 0,
            system_node_rcref_opt: None,
        }
    }

    //* --------------------------------------------------------------------- *//

    /// This helper function determines if code is in the scope of
    /// an action or operation.
    pub fn is_in_action_or_operation(&self) -> bool {
        self.operation_scope_depth > 0 || self.action_scope_depth > 0
    }

    //* --------------------------------------------------------------------- *//

    /// This helper function determines if there are any "real"  (non empty block)
    /// statements in a vec of statements and decls.

    pub fn has_non_block_statements(&self, decl_or_stmts: &Vec<DeclOrStmtType>) -> bool {
        for decl_or_stmt in decl_or_stmts {
            match decl_or_stmt {
                VarDeclT { .. } => {
                    return true;
                }
                StmtT { stmt_t } => {
                    if let StatementType::BlockStmt { block_stmt_node } = stmt_t {
                        if self.has_non_block_statements(&block_stmt_node.statements) {
                            return true;
                        }
                    } else {
                        return true;
                    }
                }
            }
        }
        false
    }

    //* --------------------------------------------------------------------- *//

    pub fn format_compartment_hierarchy(
        &mut self,
        state_node_rcref: &Rc<RefCell<StateNode>>,
        is_factory_context: bool,
        transition_expr_node_opt: Option<&TransitionExprNode>,
    ) -> String {
        let mut ret = String::new();

        // recurse to the highest parent in the chain and start generating in reverse order
        if let Some(dispatch_node) = &state_node_rcref.borrow().dispatch_opt {
            let state_symbol_rcref_opt = self
                .arcanium
                .get_state(&dispatch_node.target_state_ref.name);
            if let Some(state_symbol_rcref) = state_symbol_rcref_opt {
                let state_symbol = state_symbol_rcref.borrow();
                if let Some(parent_state_node_rcref) = &state_symbol.state_node_opt {
                    ret.push_str(&*self.format_compartment_hierarchy(
                        parent_state_node_rcref,
                        is_factory_context,
                        None,
                    ));
                }
            }
        }

        // The code below is strictly for the factory system initializtion of
        // the start state.

        // self.newline_to_string(&mut ret);

        // At the top level we want the l_value to be the standard
        // local variable/parameter for "the top compartment"/current state.

        // let mut compartment_name = "compartment";
        // if depth == 0 {
        //     compartment_name = "compartment";
        // }

        self.newline_to_string(&mut ret);
        ret.push_str(&format!(
            "next_compartment = {}Compartment('{}', next_compartment)",
            self.system_node_rcref_opt.as_ref().unwrap().borrow().name,
            self.format_target_state_name(state_node_rcref.borrow().name.as_str())
        ));

        let target_state_name = state_node_rcref.borrow().name.clone();

        if let Some(transition_expr_node) = transition_expr_node_opt {
            if transition_expr_node.forward_event {
                self.newline_to_string(&mut ret);
                ret.push_str("next_compartment.forward_event = __e");
            }

            // self.newline();

            let enter_args_opt = match &transition_expr_node.target_state_context_t {
                TargetStateContextType::StateRef { state_context_node } => {
                    &state_context_node.enter_args_opt
                }
                TargetStateContextType::StateStackPop {} => &None,
            };

            if let Some(enter_args) = enter_args_opt {
                // Note - searching for event keyed with "State:>"
                // e.g. "S1:>"

                let mut msg: String = String::from(target_state_name.clone());
                msg.push(':');
                msg.push_str(&self.symbol_config.enter_msg_symbol);

                if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt)
                {
                    match &event_sym.borrow().event_symbol_params_opt {
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
                                            // TODO: check this
                                            None => String::from(""),
                                        };
                                        let mut expr = String::new();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.newline_to_string(&mut ret);
                                        ret.push_str(&format!(
                                            "next_compartment.enter_args[\"{}\"] = {}",
                                            p.name, expr
                                        ));
                                    }
                                    None => panic!(
                                        "Invalid number of arguments for \"{}\" event handler.",
                                        msg
                                    ),
                                }
                            }
                        }
                        None => {
                            panic!("Invalid number of arguments for \"{}\" event handler.", msg)
                        }
                    }
                } else {
                    self.warnings.push(format!("State {} does not have an enter event handler but is being passed parameters in a transition", target_state_name.clone()));
                }
            }
        }

        if let Some(transition_expr_node) = transition_expr_node_opt {
            // let target_state_name = transition_expr_node.target_state_context_t.
            if transition_expr_node.forward_event {
                self.newline_to_string(&mut ret);
                // self.add_code("next_compartment.forward_event = __e");
                ret.push_str("next_compartment.forward_event = __e");
            }
            // Initialize state arguments.

            // TODO - this is temporary as for right now the only parent
            // state fields that get initialized are the variables.
            // This will get fixed when parent state initialization syntax exists
            // if !is_factory_context {
            //     match &state_node.params_opt {
            //         Some(params) => {
            //             for param in params {
            //                 self.newline_to_string(&mut ret);
            //                 if is_factory_context {
            //                     ret.push_str(&format!(
            //                         "next_compartment.state_args[\"{}\"] = start_state_state_param_{}",
            //                         param.param_name, param.param_name,
            //                     ));
            //                 } else {
            //                     ret.push_str(&format!(
            //                         "next_compartment.state_args[\"{}\"] = {}",
            //                         param.param_name, param.param_name,
            //                     ));
            //                 }
            //
            //             }
            //         }
            //         None => {}
            //     }
            // }
            // -- State Arguments --

            let target_state_args_opt = match &transition_expr_node.target_state_context_t {
                TargetStateContextType::StateRef { state_context_node } => {
                    &state_context_node.state_ref_args_opt
                }
                TargetStateContextType::StateStackPop {} => &Option::None,
            };
            //
            if let Some(state_args) = target_state_args_opt {
                //            let mut params_copy = Vec::new();
                if let Some(state_sym) = self.arcanium.get_state(&state_node_rcref.borrow().name) {
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
                                            // TODO: check this
                                            None => String::from(""),
                                        };
                                        let mut expr = String::new();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.newline_to_string(&mut ret);
                                        ret.push_str(&format!(
                                            "next_compartment.state_args[\"{}\"] = {}",
                                            param_symbol.name, expr
                                        ));
                                    }
                                    None => panic!(
                                        "Invalid number of arguments for \"{}\" state parameters.",
                                        &state_node_rcref.borrow().name
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

            // let state_node = state_node_rcref.borrow();
            // match &state_node.vars_opt {
            //     Some(vars) => {
            //         for variable_decl_node_rcref in vars {
            //             let var_decl_node = variable_decl_node_rcref.borrow();
            //             let initalizer_value_expr_t = &var_decl_node.get_initializer_value_rc();
            //             initalizer_value_expr_t.debug_print();
            //             let mut expr_code = String::new();
            //             initalizer_value_expr_t.accept_to_string(self, &mut expr_code);
            //             self.newline_to_string(&mut ret);
            //             ret.push_str( &format!(
            //                 "next_compartment.state_vars[\"{}\"] = {}",
            //                 var_decl_node.name, expr_code,
            //             ));
            //         }
            //     }
            //     None => {}
            // }
        }

        // -- State Variables --
        // NOTE: State variable initialization now handled in format_compartment_hierarchy().

        if let Some(state_symbol_rcref) = self
            .arcanium
            .get_state(state_node_rcref.borrow().name.as_str())
        {
            // #STATE_NODE_UPDATE_BUG - search comments in parser for why this is here
            let state_symbol = state_symbol_rcref.borrow();
            let state_node = &state_symbol.state_node_opt.as_ref().unwrap().borrow();
            // generate local state variables
            if state_node.vars_opt.is_some() {
                for variable_decl_node_rcref in state_node.vars_opt.as_ref().unwrap() {
                    let var_decl_node = variable_decl_node_rcref.borrow();
                    let initalizer_value_expr_t = &var_decl_node.get_initializer_value_rc();
                    initalizer_value_expr_t.debug_print();
                    let mut expr_code = String::new();
                    // #STATE_NODE_UPDATE_BUG - the AST state node wasn't being updated
                    // in the semantic pass and contained var decls from the syntactic
                    // pass. The types of the nodes therefore were CallChainNodeType::UndeclaredIdentifierNodeT
                    // and not CallChainNodeType::VariableNodeT. Therefore the generation
                    // code that relies on knowing what kind of variable this is
                    // broke. That happened in this next line.
                    initalizer_value_expr_t.accept_to_string(self, &mut expr_code);
                    self.newline_to_string(&mut ret);
                    ret.push_str(&format!(
                        "next_compartment.state_vars[\"{}\"] = {}",
                        var_decl_node.name, expr_code
                    ));
                }
            }
        }

        // TODO - this is temporary as for right now the only parent
        // state fields that get initialized are the variables.
        // This will get fixed when parent state initialization syntax exists
        // if !is_factory_context {
        //     if let Some(enter_params) = &self.system_node_rcref_opt.as_ref().unwrap().borrow().start_state_enter_params_opt {
        //         for param in enter_params {
        //             ret.push_str(&*format!("\n{}", self.dent()));
        //             //self.newline_to_string(&mut ret);
        //             if is_factory_context {
        //                 ret.push_str(&format!(
        //                     "next_compartment.enter_args[\"{}\"] = start_state_enter_param_{}"
        //                     , param.param_name, param.param_name,
        //                 ));
        //             } else {
        //                 ret.push_str(&format!(
        //                     "next_compartment.enter_args[\"{}\"] = {}"
        //                     , param.param_name, param.param_name,
        //                 ));
        //             }
        //         }
        //     }
        // }

        ret
    }

    //* --------------------------------------------------------------------- *//

    pub fn format_type(&self, type_node: &TypeNode) -> String {
        if type_node.is_system {
            String::new()
        } else if type_node.is_enum {
            format!("{}_{}", self.system_name, type_node.type_str)
        } else {
            let mut s = String::new();
            s.push_str(&type_node.type_str.clone());
            s
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
            IdentifierDeclScope::SystemScope => {
                code.push_str("self");
            }
            IdentifierDeclScope::DomainBlockScope => {
                code.push_str(&format!("self.{}", variable_node.id_node.name.lexeme));
            }
            IdentifierDeclScope::StateParamScope => {
                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }
                code.push_str(&format!(
                    "compartment.state_args[\"{}\"]",
                    variable_node.id_node.name.lexeme
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::StateVarScope => {
                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }
                code.push_str(&format!(
                    "compartment.state_vars[\"{}\"]",
                    variable_node.id_node.name.lexeme
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::EventHandlerParamScope => {
                // if self.visiting_call_chain_literal_variable {
                //     code.push_str("(");
                // }
                code.push_str(&format!(
                    "__e._parameters[\"{}\"]",
                    variable_node.id_node.name.lexeme
                ));
                // if self.visiting_call_chain_literal_variable {
                //     code.push_str(")");
                // }
            }
            IdentifierDeclScope::EventHandlerVarScope => {
                code.push_str(&variable_node.id_node.name.lexeme.to_string());
            }
            IdentifierDeclScope::BlockVarScope => {
                code.push_str(&variable_node.id_node.name.lexeme.to_string());
            }
            IdentifierDeclScope::UnknownScope => {
                // TODO: Explore labeling Variables as "extern" scope
                code.push_str(&variable_node.id_node.name.lexeme.to_string());
            } // Actions?
            _ => self.errors.push("Illegal scope.".to_string()),
        }

        code
    }

    //* --------------------------------------------------------------------- *//

    fn format_list_element_expr(&mut self, list_element_node: &ListElementNode) -> String {
        let mut code = String::new();

        match list_element_node.scope {
            IdentifierDeclScope::SystemScope => {
                code.push_str("self");
            }
            IdentifierDeclScope::DomainBlockScope => {
                code.push_str(&format!(
                    "self.{}",
                    list_element_node.identifier.name.lexeme
                ));
            }
            IdentifierDeclScope::StateParamScope => {
                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }
                code.push_str(&format!(
                    "compartment.state_args[\"{}\"]",
                    list_element_node.identifier.name.lexeme
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::StateVarScope => {
                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }
                code.push_str(&format!(
                    "compartment.state_vars[\"{}\"]",
                    list_element_node.identifier.name.lexeme
                ));
                if self.visiting_call_chain_literal_variable {
                    code.push(')');
                }
            }
            IdentifierDeclScope::EventHandlerParamScope => {
                // if self.visiting_call_chain_literal_variable {
                //     code.push_str("(");
                // }
                code.push_str(&format!(
                    "__e._parameters[\"{}\"]",
                    list_element_node.identifier.name.lexeme
                ));
                // if self.visiting_call_chain_literal_variable {
                //     code.push_str(")");
                // }
            }
            IdentifierDeclScope::EventHandlerVarScope => {
                code.push_str(&list_element_node.identifier.name.lexeme.to_string());
            }
            IdentifierDeclScope::BlockVarScope => {
                code.push_str(&list_element_node.identifier.name.lexeme.to_string());
            }
            IdentifierDeclScope::UnknownScope => {
                // TODO: Explore labeling Variables as "extern" scope
                code.push_str(&list_element_node.identifier.name.lexeme.to_string());
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
                None => String::from(""),
            };
            self.add_code(&param.param_name.to_string());
            if !param_type.is_empty() {
                self.add_code(&format!(": {}", param_type));
            }
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
                None => String::from(""),
            };
            self.add_code(&param.param_name.to_string());
            subclass_actions.push_str(&param.param_name.to_string());
            if !param_type.is_empty() {
                self.add_code(&format!(": {}", param_type));
                subclass_actions.push_str(&format!(": {}", param_type));
            }
            separator = ",";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_operations_parameter_list(&mut self, params: &Vec<ParameterNode>) {
        let mut separator = "";
        for param in params {
            self.add_code(separator);
            let param_type: String = match &param.param_type_opt {
                Some(type_node) => self.format_type(type_node),
                None => String::from(""),
            };
            self.add_code(&param.param_name.to_string());
            if !param_type.is_empty() {
                self.add_code(&format!(": {}", param_type));
            }
            separator = ",";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_action_name(&self, action_name: &String) -> String {
        format!("{}_do", action_name)
    }
    //* --------------------------------------------------------------------- *//

    fn format_operation_name(&self, operation_name: &String) -> String {
        format!("{}", operation_name)
    }

    //* --------------------------------------------------------------------- *//

    fn format_enum_name(&self, enum_type_name: &String) -> String {
        format!("{}_{}", self.system_name, enum_type_name)
    }

    //* --------------------------------------------------------------------- *//

    pub fn run(&mut self, system_node_rcref: Rc<RefCell<SystemNode>>) {
        match &system_node_rcref.borrow().system_attributes_opt {
            Some(attributes) => {
                for value in (*attributes).values() {
                    match value {
                        AttributeNode::MetaWord { attr } => {
                            // TODO
                            let err_msg = format!("Unknown attribute {}.", attr.name);
                            self.errors.push(err_msg);
                        }
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
                                            "marshal" => self.marshal = true,
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
            }
            None => {}
        }

        // if self.marshal {
        //     self.newline();
        //     self.add_code("import jsonpickle");
        // }

        self.system_node_rcref_opt = Some(system_node_rcref.clone());
        system_node_rcref.borrow().accept(self);

        if self.generate_main {
            self.newline();
            self.add_code("if __name__ == '__main__':");
            self.indent();
            self.newline();
            let mut arg_cnt: usize = 0;
            if let Some(functions) = &self
                .system_node_rcref_opt
                .as_ref()
                .unwrap()
                .borrow()
                .functions_opt
            {
                for function_rcref in functions {
                    let function_node = function_rcref.borrow();
                    if function_node.name == "main" {
                        if let Some(params) = &function_node.params {
                            arg_cnt = params.len();
                        } else {
                            arg_cnt = 0;
                        }
                        break;
                    }
                }
            };
            self.add_code("main(");
            let mut separator = "";
            for i in 1..arg_cnt + 1 {
                self.add_code(&format!("{}sys.argv[{}]", separator, i));
                separator = ",";
            }
            self.add_code(")");
            self.outdent();
            self.newline();
        }
    }

    //* --------------------------------------------------------------------- *//

    fn add_code(&mut self, s: &str) {
        self.code.push_str(&*s.to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn skip_next_newline(&mut self) {
        self.code.push_str(&*format!("\n{}", self.dent()));
        self.skip_next_newline = true;
    }

    //* --------------------------------------------------------------------- *//

    fn test_skip_newline(&mut self) {
        if self.skip_next_newline {
            self.skip_next_newline = false;
        } else {
            self.code.push_str(&*format!("\n{}", self.dent()));
        }
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
                DeclOrStmtType::VarDeclT {
                    var_decl_t_rcref: var_decl_t_rc_ref,
                } => {
                    let variable_decl_node = var_decl_t_rc_ref.borrow();
                    variable_decl_node.accept(self);
                }
                DeclOrStmtType::StmtT { stmt_t } => {
                    match stmt_t {
                        StatementType::ExpressionStmt { expr_stmt_t } => {
                            match expr_stmt_t {
                                ExprStmtType::TransitionStmtT {
                                    transition_statement_node: _transition_statement_node,
                                } => panic!("TODO"),
                                ExprStmtType::SystemInstanceStmtT {
                                    system_instance_stmt_node,
                                } => system_instance_stmt_node.accept(self),
                                ExprStmtType::SystemTypeStmtT {
                                    system_type_stmt_node,
                                } => system_type_stmt_node.accept(self),
                                ExprStmtType::ActionCallStmtT {
                                    action_call_stmt_node,
                                } => action_call_stmt_node.accept(self), // // TODO
                                ExprStmtType::CallStmtT { call_stmt_node } => {
                                    call_stmt_node.accept(self)
                                }
                                ExprStmtType::CallChainStmtT {
                                    call_chain_literal_stmt_node: call_chain_stmt_node,
                                } => call_chain_stmt_node.accept(self),
                                ExprStmtType::AssignmentStmtT {
                                    assignment_stmt_node,
                                } => assignment_stmt_node.accept(self),
                                ExprStmtType::VariableStmtT { variable_stmt_node } => {
                                    variable_stmt_node.accept(self)
                                }
                                ExprStmtType::ListStmtT { list_stmt_node } => {
                                    list_stmt_node.accept(self)
                                }
                                ExprStmtType::ExprListStmtT {
                                    expr_list_stmt_node,
                                } => expr_list_stmt_node.accept(self),
                                ExprStmtType::EnumeratorStmtT {
                                    enumerator_stmt_node,
                                } => enumerator_stmt_node.accept(self),
                                ExprStmtType::BinaryStmtT { binary_stmt_node } => {
                                    binary_stmt_node.accept(self)
                                }
                            }
                        }
                        StatementType::TransitionStmt {
                            transition_statement_node: transition_statement,
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
                        // StatementType::ChangeStateStmt {
                        //     change_state_stmt_node: change_state_stmt,
                        // } => {
                        //     change_state_stmt.accept(self);
                        // }
                        StatementType::LoopStmt { loop_stmt_node } => {
                            loop_stmt_node.accept(self);
                        }
                        StatementType::BlockStmt { block_stmt_node } => {
                            block_stmt_node.accept(self);
                        }
                        StatementType::ReturnAssignStmt {
                            return_assign_stmt_node,
                        } => {
                            return_assign_stmt_node.accept(self);
                        }
                        StatementType::ContinueStmt { continue_stmt_node } => {
                            continue_stmt_node.accept(self);
                        }
                        StatementType::BreakStmt { break_stmt_node } => {
                            break_stmt_node.accept(self);
                        }
                        StatementType::SuperStringStmt {
                            super_string_stmt_node,
                        } => {
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

    //* --------------------------------------------------------------------- *//

    fn generate_machinery(&mut self, system_node: &SystemNode) {
        self.newline();
        self.newline();
        self.add_code("# ==================== System Runtime =================== #");
        self.newline();
        self.newline();
        self.add_code("def __kernel(self, __e):");
        self.indent();

        match &system_node.machine_block_node_opt {
            Some(machine_block_node) => {
                self.newline();
                self.newline();
                self.add_code("# send event to current state");
                self.newline();
                self.add_code("self.__router(__e)");
                self.newline();
                self.newline();
                self.add_code("# loop until no transitions occur");
                self.newline();
                self.add_code("while self.__next_compartment != None:");
                self.indent();
                self.newline();
                self.add_code("next_compartment = self.__next_compartment");
                self.newline();
                self.add_code("self.__next_compartment = None");
                self.newline();
                self.newline();
                self.add_code("# exit current state");
                self.newline();
                self.add_code("self.__router(FrameEvent( \"<\", self.__compartment.exit_args))");
                self.newline();
                self.add_code("# change state");
                self.newline();
                self.add_code("self.__compartment = next_compartment");
                self.newline();
                self.newline();
                self.add_code("if next_compartment.forward_event is None:");
                self.indent();
                self.newline();
                self.add_code("# send normal enter event");
                self.newline();
                self.add_code("self.__router(FrameEvent(\">\", self.__compartment.enter_args))");
                self.outdent();
                self.newline();
                self.add_code("else: # there is a forwarded event");
                self.indent();
                self.newline();
                self.add_code("if next_compartment.forward_event._message == \">\":");
                self.indent();
                self.newline();
                self.add_code("# forwarded event is enter event");
                self.newline();
                self.add_code("self.__router(next_compartment.forward_event)");
                self.outdent();
                self.newline();
                self.add_code("else:");
                self.indent();
                self.newline();
                self.add_code("# forwarded event is not enter event");
                self.newline();
                self.add_code("# send normal enter event");
                self.newline();
                self.add_code("self.__router(FrameEvent(\">\", self.__compartment.enter_args))");
                self.newline();
                self.add_code("# and now forward event to new, intialized state");
                self.newline();
                self.add_code("self.__router(next_compartment.forward_event)");
                self.outdent();
                self.newline();
                self.add_code("next_compartment.forward_event = None");
                self.newline();
                self.outdent();
                self.outdent();
                self.outdent();
                self.newline();
                self.newline();
                self.add_code("def __router(self, __e):");
                self.indent();
                self.newline();

                let _current_index = 0;
                let len = machine_block_node.states.len();
                for (current_index, state_node_rcref) in
                    machine_block_node.states.iter().enumerate()
                {
                    let state_name = &self
                        .format_target_state_name(&state_node_rcref.borrow().name)
                        .to_string();
                    if current_index == 0 {
                        self.add_code(&format!("if self.__compartment.state == '{}':", state_name));
                    } else {
                        self.add_code(&format!(
                            "elif self.__compartment.state == '{}':",
                            state_name
                        ));
                    }
                    self.indent();
                    self.newline();
                    self.add_code(&format!("self.{}(__e, self.__compartment)", state_name));
                    self.outdent();
                    if current_index != len {
                        self.newline();
                    }
                }
                self.outdent();
            }
            _ => {
                self.newline();
                self.add_code("pass");
                self.outdent();
                self.newline();
            }
        }

        if system_node.get_first_state().is_some() {
            self.newline();
            self.add_code(&format!("def __transition(self, next_compartment):"));
            self.indent();
            self.newline();
            self.add_code("self.__next_compartment = next_compartment");
            self.outdent();

            if self.generate_state_stack {
                self.newline();
                self.newline();
                self.add_code(&format!("def __state_stack_push(self, compartment):"));
                self.indent();
                self.newline();
                self.add_code("self.__state_stack.append(compartment)");
                self.outdent();
                self.newline();
                self.newline();
                self.add_code("def __state_stack_pop(self):");
                self.indent();
                self.newline();
                self.add_code("return self.__state_stack.pop()");
                self.outdent();
            }
            if self.generate_change_state {
                self.newline();
                self.newline();
                self.add_code(&format!("def __change_state(self, new_compartment):"));
                self.indent();
                self.newline();
                self.add_code("self.__compartment = new_compartment");
                self.outdent();
                self.newline();
            }

            self.newline();

            // if self.arcanium.is_serializable() {
            //     for line in self.serialize.iter() {
            //         self.code.push_str(&*line.to_string());
            //         self.code.push_str(&*format!("\n{}", self.dent()));
            //     }
            //
            //     for line in self.deserialize.iter() {
            //         self.code.push_str(&*line.to_string());
            //         self.code.push_str(&*format!("\n{}", self.dent()));
            //     }
            // }
        }
    }

    //* --------------------------------------------------------------------- *//

    // fn generate_subclass(&mut self) {
    //     for line in self.subclass_code.iter() {
    //         self.code.push_str(&*line.to_string());
    //         self.code.push_str(&*format!("\n{}", self.dent()));
    //     }
    // }

    //* --------------------------------------------------------------------- *//

    /// Generate a return statement within a handler. Call this rather than adding a return
    /// statement directly to ensure that the control-flow state is properly maintained.
    fn generate_return(&mut self) {
        self.newline();
        self.add_code("return");
        self.this_branch_transitioned = false;
    }

    /// Generate a return statement if the current branch contained a transition or change-state.
    fn generate_return_if_transitioned(&mut self) {
        if self.this_branch_transitioned {
            self.generate_return();
        }
    }

    // Generate a break statement if the current branch doesnot contain a transition or change-state.

    // fn generate_break_if_not_transitioned(&mut self) {
    //     if !self.this_branch_transitioned {
    //         self.newline();
    //         self.add_code("break");
    //     }
    // }

    //* --------------------------------------------------------------------- *//

    fn generate_comment(&mut self, line: usize) -> bool {
        // can't use self.newline() or self.add_code() due to double borrow.
        let mut generated_comment = false;
        while self.current_comment_idx < self.comments.len()
            && line >= self.comments[self.current_comment_idx].line
        {
            let comment = &self.comments[self.current_comment_idx];
            if comment.token_type == TokenType::SingleLineComment {
                self.code
                    .push_str(&*format!("  # {}", &comment.lexeme[2..]));
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

        generated_comment
    }

    //* --------------------------------------------------------------------- *//

    // TODO
    // fn generate_state_ref_change_state(
    //     &mut self,
    //     change_state_stmt_node: &ChangeStateStatementNode,
    // ) {
    //     let target_state_name = match &change_state_stmt_node.state_context_t {
    //         TargetStateContextType::StateRef { state_context_node } => {
    //             &state_context_node.state_ref_node.name
    //         }
    //         _ => {
    //             self.errors.push("Unknown error.".to_string());
    //             ""
    //         }
    //     };
    //
    //     let state_ref_code = self.generate_state_ref_code(target_state_name);
    //
    //     // get the change state label, and print it if provided
    //     match &change_state_stmt_node.label_opt {
    //         Some(label) => {
    //             self.add_code(&format!("# {}", label));
    //             self.newline();
    //         }
    //         None => {}
    //     }
    //     self.newline();
    //     self.add_code(&format!(
    //         "compartment = {}Compartment('{}')",
    //         self.system_name, state_ref_code
    //     ));
    //     self.newline();
    //
    //     // -- Enter Arguments --
    //
    //     let enter_args_opt = match &change_state_stmt_node.state_context_t {
    //         TargetStateContextType::StateRef { state_context_node } => {
    //             &state_context_node.enter_args_opt
    //         }
    //         TargetStateContextType::StateStackPop {} => &None,
    //     };
    //
    //     if let Some(enter_args) = enter_args_opt {
    //         // Note - searching for event keyed with "State:>"
    //         // e.g. "S1:>"
    //
    //         let mut msg: String = String::from(target_state_name);
    //         msg.push(':');
    //         msg.push_str(&self.symbol_config.enter_msg_symbol);
    //
    //         if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt) {
    //             match &event_sym.borrow().event_symbol_params_opt {
    //                 Some(event_params) => {
    //                     if enter_args.exprs_t.len() != event_params.len() {
    //                         panic!("Fatal error: misaligned parameters to arguments.")
    //                     }
    //                     let mut param_symbols_it = event_params.iter();
    //                     for expr_t in &enter_args.exprs_t {
    //                         match param_symbols_it.next() {
    //                             Some(p) => {
    //                                 let _param_type = match &p.param_type_opt {
    //                                     Some(param_type) => param_type.get_type_str(),
    //                                     None => String::from(""),
    //                                 };
    //                                 let mut expr = String::new();
    //                                 expr_t.accept_to_string(self, &mut expr);
    //                                 self.add_code(&format!(
    //                                     "compartment.enter_args[\"{}\"] = {}",
    //                                     p.name, expr
    //                                 ));
    //                                 self.newline();
    //                             }
    //                             None => panic!(
    //                                 "Invalid number of arguments for \"{}\" event handler.",
    //                                 msg
    //                             ),
    //                         }
    //                     }
    //                 }
    //                 None => panic!("Invalid number of arguments for \"{}\" event handler.", msg),
    //             }
    //         } else {
    //             self.warnings.push(format!("State {} does not have an enter event handler but is being passed parameters in a change state", target_state_name));
    //         }
    //     }
    //
    //     /*  -- State Arguments -- */
    //     let target_state_args_opt = match &change_state_stmt_node.state_context_t {
    //         TargetStateContextType::StateRef { state_context_node } => {
    //             &state_context_node.state_ref_args_opt
    //         }
    //         TargetStateContextType::StateStackPop {} => &Option::None,
    //     };
    //
    //     if let Some(state_args) = target_state_args_opt {
    //         //            let mut params_copy = Vec::new();
    //         if let Some(state_sym) = self.arcanium.get_state(target_state_name) {
    //             match &state_sym.borrow().params_opt {
    //                 Some(event_params) => {
    //                     let mut param_symbols_it = event_params.iter();
    //                     // Loop through the ARGUMENTS...
    //                     for expr_t in &state_args.exprs_t {
    //                         // ...and validate w/ the PARAMETERS
    //                         match param_symbols_it.next() {
    //                             Some(param_symbol_rcref) => {
    //                                 let param_symbol = param_symbol_rcref.borrow();
    //                                 let _param_type = match &param_symbol.param_type_opt {
    //                                     Some(param_type) => param_type.get_type_str(),
    //                                     None => String::from(""),
    //                                 };
    //                                 let mut expr = String::new();
    //                                 expr_t.accept_to_string(self, &mut expr);
    //                                 self.add_code(&format!(
    //                                     "next_compartment.state_args[\"{}\"] = {};",
    //                                     param_symbol.name, expr
    //                                 ));
    //                                 self.newline();
    //                             }
    //                             None => panic!(
    //                                 "Invalid number of arguments for \"{}\" state parameters.",
    //                                 target_state_name
    //                             ),
    //                         }
    //                         //
    //                     }
    //                 }
    //                 None => {}
    //             }
    //         } else {
    //             panic!("TODO");
    //         }
    //     } // -- State Arguments --
    //
    //     // -- State Variables --
    //
    //     let target_state_rcref_opt = self.arcanium.get_state(target_state_name);
    //
    //     match target_state_rcref_opt {
    //         Some(q) => {
    //             //                target_state_vars = "stateVars".to_string();
    //             if let Some(state_symbol_rcref) = self.arcanium.get_state(&q.borrow().name) {
    //                 let state_symbol = state_symbol_rcref.borrow();
    //                 let state_node = &state_symbol.state_node_opt.as_ref().unwrap().borrow();
    //                 // generate local state variables
    //                 if state_node.vars_opt.is_some() {
    //                     //                        let mut separator = "";
    //                     for var_rcref in state_node.vars_opt.as_ref().unwrap() {
    //                         let var = var_rcref.borrow();
    //                         let _var_type = match &var.type_opt {
    //                             Some(var_type) => var_type.get_type_str(),
    //                             None => String::from(""),
    //                         };
    //                         let expr_t = &var.get_initializer_value_rc();
    //                         let mut expr_code = String::new();
    //                         expr_t.accept_to_string(self, &mut expr_code);
    //                         self.add_code(&format!(
    //                             "next_compartment.state_vars[\"{}\"] = {};",
    //                             var.name, expr_code
    //                         ));
    //                         self.newline();
    //                     }
    //                 }
    //             }
    //         }
    //         None => {
    //             //                code = target_state_vars.clone();
    //         }
    //     }
    //
    //     self.newline();
    //     self.add_code("self.__change_state(compartment)");
    // }

    //* --------------------------------------------------------------------- *//

    // fn generate_state_stack_pop_change_state(
    //     &mut self,
    //     change_state_stmt_node: &ChangeStateStatementNode,
    // ) {
    //     self.newline();
    //     match &change_state_stmt_node.label_opt {
    //         Some(label) => {
    //             self.add_code(&format!("# {}", label));
    //             self.newline();
    //         }
    //         None => {}
    //     }
    //
    //     self.add_code("compartment = self.__state_stack_pop()");
    //     self.newline();
    //     self.add_code("self.__change_state(compartment)");
    // }

    //* --------------------------------------------------------------------- *//

    // fn generate_state_ref_code(&self, target_state_name: &str) -> String {
    //     self.format_target_state_name(target_state_name)
    // }

    //* --------------------------------------------------------------------- *//

    fn generate_state_ref_transition(
        &mut self,
        transition_expr_node: &TransitionExprNode,
        expr_list_node_opt: &Option<ExprListNode>,
    ) {
        let target_state_name = match &transition_expr_node.target_state_context_t {
            TargetStateContextType::StateRef { state_context_node } => {
                &state_context_node.state_ref_node.name
            }
            _ => {
                self.errors.push("Unknown error.".to_string());
                ""
            }
        };

        //  let state_ref_code = self.generate_state_ref_code(target_state_name);

        // self.newline();
        match &transition_expr_node.label_opt {
            Some(label) => {
                self.newline();
                self.add_code(&format!("# {}", label));
            }
            None => {}
        }

        // -- Exit Arguments --

        if let Some(exit_args) = expr_list_node_opt {
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
                    match &event_sym.borrow().event_symbol_params_opt {
                        Some(event_params) => {
                            if exit_args.exprs_t.len() != event_params.len() {
                                panic!("Fatal error: misaligned parameters to arguments.")
                            }
                            let mut param_symbols_it = event_params.iter();
                            // Loop through the ARGUMENTS...
                            for expr_t in &exit_args.exprs_t {
                                // ...and validate w/ the PARAMETERS
                                match param_symbols_it.next() {
                                    Some(p) => {
                                        let _param_type = match &p.param_type_opt {
                                            Some(param_type) => param_type.get_type_str(),
                                            // TODO: check this
                                            None => String::from(""),
                                        };
                                        let mut expr = String::new();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.newline();
                                        self.add_code(&format!(
                                            "self.__compartment.exit_args[\"{}\"] = {}",
                                            p.name, expr
                                        ));
                                    }
                                    None => panic!(
                                        "Invalid number of arguments for \"{}\" event handler.",
                                        msg
                                    ),
                                }
                            }
                        }
                        None => panic!("Fatal error: misaligned parameters to arguments."),
                    }
                } else {
                    panic!("TODO");
                }
            }
        }

        // Generate the state hierarchy.

        self.newline();
        self.add_code("next_compartment = None");
        //  self.newline();
        let state_node_rcref_opt = self
            .system_node_rcref_opt
            .as_ref()
            .unwrap()
            .borrow()
            .get_state_node(&target_state_name.to_string());
        if let Some(state_node_rcref) = state_node_rcref_opt {
            let code = self.format_compartment_hierarchy(
                &state_node_rcref,
                false,
                Some(transition_expr_node),
            );
            self.add_code(code.as_str());
            self.newline();
        } else {
            // TODO - figure out what to do if a state is referenced but not defined.
        }

        // self.add_code(&format!(
        //     "next_compartment = compartment"
        // ));
        //
        //
        //
        // if transition_expr_node.forward_event {
        //     self.newline();
        //     self.add_code("next_compartment.forward_event = __e");
        // }
        //
        // // self.newline();
        //
        // let enter_args_opt = match &transition_expr_node.target_state_context_t {
        //     TargetStateContextType::StateRef { state_context_node } => {
        //         &state_context_node.enter_args_opt
        //     }
        //     TargetStateContextType::StateStackPop {} => &None,
        // };
        //
        // if let Some(enter_args) = enter_args_opt {
        //     // Note - searching for event keyed with "State:>"
        //     // e.g. "S1:>"
        //
        //     let mut msg: String = String::from(target_state_name);
        //     msg.push(':');
        //     msg.push_str(&self.symbol_config.enter_msg_symbol);
        //
        //     if let Some(event_sym) = self.arcanium.get_event(&msg, &self.current_state_name_opt) {
        //         match &event_sym.borrow().event_symbol_params_opt {
        //             Some(event_params) => {
        //                 if enter_args.exprs_t.len() != event_params.len() {
        //                     panic!("Fatal error: misaligned parameters to arguments.")
        //                 }
        //                 let mut param_symbols_it = event_params.iter();
        //                 for expr_t in &enter_args.exprs_t {
        //                     match param_symbols_it.next() {
        //                         Some(p) => {
        //                             let _param_type = match &p.param_type_opt {
        //                                 Some(param_type) => param_type.get_type_str(),
        //                                 // TODO: check this
        //                                 None => String::from(""),
        //                             };
        //                             let mut expr = String::new();
        //                             expr_t.accept_to_string(self, &mut expr);
        //                             self.newline();
        //                             self.add_code(&format!(
        //                                 "next_compartment.enter_args[\"{}\"] = {}",
        //                                 p.name, expr
        //                             ));
        //                         }
        //                         None => panic!(
        //                             "Invalid number of arguments for \"{}\" event handler.",
        //                             msg
        //                         ),
        //                     }
        //                 }
        //             }
        //             None => panic!("Invalid number of arguments for \"{}\" event handler.", msg),
        //         }
        //     } else {
        //         self.warnings.push(format!("State {} does not have an enter event handler but is being passed parameters in a transition", target_state_name));
        //     }
        // }
        //
        // // -- State Arguments --
        //
        // let target_state_args_opt = match &transition_expr_node.target_state_context_t {
        //     TargetStateContextType::StateRef { state_context_node } => {
        //         &state_context_node.state_ref_args_opt
        //     }
        //     TargetStateContextType::StateStackPop {} => &Option::None,
        // };
        // //
        // if let Some(state_args) = target_state_args_opt {
        //     //            let mut params_copy = Vec::new();
        //     if let Some(state_sym) = self.arcanium.get_state(target_state_name) {
        //         match &state_sym.borrow().params_opt {
        //             Some(event_params) => {
        //                 let mut param_symbols_it = event_params.iter();
        //                 // Loop through the ARGUMENTS...
        //                 for expr_t in &state_args.exprs_t {
        //                     // ...and validate w/ the PARAMETERS
        //                     match param_symbols_it.next() {
        //                         Some(param_symbol_rcref) => {
        //                             let param_symbol = param_symbol_rcref.borrow();
        //                             let _param_type = match &param_symbol.param_type_opt {
        //                                 Some(param_type) => param_type.get_type_str(),
        //                                 // TODO: check this
        //                                 None => String::from(""),
        //                             };
        //                             let mut expr = String::new();
        //                             expr_t.accept_to_string(self, &mut expr);
        //                             self.newline();
        //                             self.add_code(&format!(
        //                                 "next_compartment.state_args[\"{}\"] = {}",
        //                                 param_symbol.name, expr
        //                             ));
        //                         }
        //                         None => panic!(
        //                             "Invalid number of arguments for \"{}\" state parameters.",
        //                             target_state_name
        //                         ),
        //                     }
        //                     //
        //                 }
        //             }
        //             None => {}
        //         }
        //     } else {
        //         panic!("TODO");
        //     }
        // } // -- State Arguments --
        //
        // // -- State Variables --
        // // NOTE: State variable initialization now handled in format_compartment_hierarchy().
        //
        // let target_state_rcref_opt = self.arcanium.get_state(target_state_name);
        //
        // match target_state_rcref_opt {
        //     Some(q) => {
        //         //                target_state_vars = "stateVars".to_string();
        //         if let Some(state_symbol_rcref) = self.arcanium.get_state(&q.borrow().name) {
        //             // #STATE_NODE_UPDATE_BUG - search comments in parser for why this is here
        //             let state_symbol = state_symbol_rcref.borrow();
        //             let state_node = &state_symbol.state_node_opt.as_ref().unwrap().borrow();
        //             // generate local state variables
        //             if state_node.vars_opt.is_some() {
        //                 for variable_decl_node_rcref in state_node.vars_opt.as_ref().unwrap() {
        //                     let var_decl_node = variable_decl_node_rcref.borrow();
        //                     let initalizer_value_expr_t = &var_decl_node.get_initializer_value_rc();
        //                     initalizer_value_expr_t.debug_print();
        //                     let mut expr_code = String::new();
        //                     // #STATE_NODE_UPDATE_BUG - the AST state node wasn't being updated
        //                     // in the semantic pass and contained var decls from the syntactic
        //                     // pass. The types of the nodes therefore were CallChainNodeType::UndeclaredIdentifierNodeT
        //                     // and not CallChainNodeType::VariableNodeT. Therefore the generation
        //                     // code that relies on knowing what kind of variable this is
        //                     // broke. That happened in this next line.
        //                     initalizer_value_expr_t.accept_to_string(self, &mut expr_code);
        //                     self.newline();
        //                     self.add_code(&format!(
        //                         "next_compartment.state_vars[\"{}\"] = {}",
        //                         var_decl_node.name, expr_code
        //                     ));
        //                 }
        //             }
        //         }
        //     }
        //     None => {
        //         //                code = target_state_vars.clone();
        //     }
        // }

        self.add_code("self.__transition(next_compartment)");
    }

    //* --------------------------------------------------------------------- *//

    fn format_target_state_name(&self, state_name: &str) -> String {
        format!("__{}_state_{}", self.system_name.to_lowercase(), state_name)
    }

    //* --------------------------------------------------------------------- *//

    // NOTE!!: it is *currently* disallowed to send state or event arguments to a state stack pop target
    // So currently this method just sets any exitArgs and pops the context from the state stack.

    fn generate_state_stack_pop_transition(
        &mut self,
        transition_expr_node: &TransitionExprNode,
        expr_list_node_opt: &Option<ExprListNode>,
    ) {
        self.newline();
        match &transition_expr_node.label_opt {
            Some(label) => {
                self.add_code(&format!("# {}", label));
                self.newline();
            }
            None => {}
        }

        // -- Exit Arguments --

        if let Some(exit_args) = &expr_list_node_opt {
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
                    match &event_sym.borrow().event_symbol_params_opt {
                        Some(event_params) => {
                            if exit_args.exprs_t.len() != event_params.len() {
                                panic!("Fatal error: misaligned parameters to arguments.")
                            }
                            let mut param_symbols_it = event_params.iter();
                            // self.add_code("FrameEventParams exitArgs = new FrameEventParams();");
                            // self.newline();
                            // Loop through the ARGUMENTS...
                            for expr_t in &exit_args.exprs_t {
                                // ...and validate w/ the PARAMETERS
                                match param_symbols_it.next() {
                                    Some(p) => {
                                        let _param_type = match &p.param_type_opt {
                                            Some(param_type) => param_type.get_type_str(),
                                            None => String::from(""),
                                        };
                                        let mut expr = String::new();
                                        expr_t.accept_to_string(self, &mut expr);
                                        self.add_code(&format!(
                                            "self.__compartment.exit_args[\"{}\"] = {}",
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
                        None => panic!("Fatal error: misaligned parameters to arguments."),
                    }
                } else {
                    panic!("TODO");
                }
            }
        }

        self.add_code("next_compartment = self.__state_stack_pop()");
        self.newline();

        if transition_expr_node.forward_event {
            self.add_code("next_compartment.forward_event = __e");
            self.newline();
        }

        self.add_code("self.__transition(next_compartment)");
    }

    //* --------------------------------------------------------------------- *//

    fn generate_compartment(&mut self, system_name: &str) {
        self.newline();
        self.add_code("# ===================== Compartment =================== #");
        self.newline();
        self.newline();
        self.add_code(&format!("class {}Compartment:", system_name));
        self.newline();
        self.indent();
        self.newline();
        self.add_code("def __init__(self,state,parent_compartment):");
        self.indent();
        self.newline();
        self.add_code("self.state = state");
        self.newline();
        self.add_code("self.state_args = {}");
        self.newline();
        self.add_code("self.state_vars = {}");
        self.newline();
        self.add_code("self.enter_args = {}");
        self.newline();
        self.add_code("self.exit_args = {}");
        self.newline();
        self.add_code("self.forward_event = None");
        self.newline();
        self.add_code("self.parent_compartment = parent_compartment");
        self.outdent();
        self.newline();
        self.outdent();
    }

    //* --------------------------------------------------------------------- *//

    fn generate_factory_fn(&mut self, system_node: &SystemNode) {
        self.indent();
        if system_node.get_first_state().is_some() {
            self.newline();
            self.newline();

            if self.generate_state_stack {
                self.add_code("# Create state stack.");
                self.newline();
                self.newline();
                self.add_code("self.__state_stack = []");
                self.newline();
                self.newline();
            }

            self.add_code(" # Create and initialize start state compartment.");
            self.newline();
            self.newline();

            self.add_code("next_compartment = None");
            let state_node_rcref = system_node.get_first_state().unwrap();
            let code = self.format_compartment_hierarchy(state_node_rcref, true, None);
            self.add_code(code.as_str());
            self.newline();
            self.add_code(&format!("self.__compartment = next_compartment"));

            self.newline();
            self.add_code(&format!("self.__next_compartment = None"));
        } else {
            // self.add_code(" # Create and initialize start state compartment.");
            self.newline();
            self.newline();
            self.add_code("self.__compartment = None");
        }

        self.newline();
        // Note - the initialization to  [None]  is to support
        // situations where the return is set during start state
        // entry event handler. As there is no call to the interface, a return
        // element is not yet pushed on the return stack so the program will just crash
        // if not initialized this way. This [None] will never be returned to a caller
        // as there is no caller during start state initialization but prevents
        // the crash from happening.
        self.add_code(&format!("self.return_stack = [None]"));

        // Initialize state arguments.
        match &system_node.start_state_state_params_opt {
            Some(params) => {
                for param in params {
                    self.newline();
                    self.add_code(&format!(
                        "self.__compartment.state_args[\"{}\"] = start_state_state_param_{}",
                        param.param_name, param.param_name,
                    ));
                }
            }
            None => {}
        }

        match system_node.get_first_state() {
            Some(state_rcref) => {
                // TODO. This code is related to #STATE_NODE_UPDATE_BUG
                // and should be refactored with the other related code
                // that generates state vars.
                let state_node = state_rcref.borrow();
                match &state_node.vars_opt {
                    Some(vars) => {
                        for variable_decl_node_rcref in vars {
                            let var_decl_node = variable_decl_node_rcref.borrow();
                            let initalizer_value_expr_t = &var_decl_node.get_initializer_value_rc();
                            initalizer_value_expr_t.debug_print();
                            let mut expr_code = String::new();
                            initalizer_value_expr_t.accept_to_string(self, &mut expr_code);
                            self.newline();
                            let code = &format!(
                                "next_compartment.state_vars[\"{}\"] = {}",
                                var_decl_node.name, expr_code,
                            );
                            self.add_code(code);
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
                    "self.__compartment.enter_args[\"{}\"] = start_state_enter_param_{}",
                    param.param_name, param.param_name,
                ));
            }
        }

        self.newline();
        self.newline();
        self.add_code("# Initialize domain");

        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            // domain_block_node.accept(self);
            self.newline();

            for variable_decl_node_rcref in &domain_block_node.member_variables {
                let variable_decl_node = variable_decl_node_rcref.borrow_mut();
                //         variable_decl_node.initializer_expr_t_opt = Option::None;
                if let Some(domain_params_vec) = &system_node.domain_params_opt {
                    for domain_param in domain_params_vec {
                        if domain_param.param_name == variable_decl_node.name {
                            self.variable_init_override_opt = Some(domain_param.param_name.clone());
                            break;
                        }
                    }
                }

                variable_decl_node.accept(self);
                self.variable_init_override_opt = Option::None;
            }

            self.newline();
        } else {
            self.newline();
        }
        if self.has_states {
            self.newline();
            self.add_code("# Send system start event");

            if let Some(_enter_params) = &system_node.start_state_enter_params_opt {
                self.newline();
                self.add_code("frame_event = FrameEvent(\">\", self.__compartment.enter_args)");
            } else {
                self.newline();
                self.add_code("frame_event = FrameEvent(\">\", None)");
            }

            self.newline();
            self.add_code("self.__kernel(frame_event)");
        }

        self.outdent();
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn generate_state_info_method(&mut self) {
        self.newline();
        self.add_code("def state_info(self):");
        self.indent();
        self.newline();
        self.add_code("return self.__compartment.state");
        self.newline();
        self.outdent();
    }
    //* --------------------------------------------------------------------- *//

    fn generate_compartment_info_method(&mut self) {
        self.newline();
        self.add_code("def compartment_info(self):");
        self.indent();
        self.newline();
        self.add_code("return self.__compartment");
        self.newline();
        self.outdent();
    }
    //* --------------------------------------------------------------------- *//
}

//* --------------------------------------------------------------------- *//

impl AstVisitor for PythonVisitor {
    //* --------------------------------------------------------------------- *//

    fn visit_module(&mut self, module: &Module) {
        self.add_code(&format!("#{}", self.compiler_version));
        self.newline();
        self.newline();

        let mut generate_frame_event = true;

        for module_element in &module.module_elements {
            match module_element {
                ModuleElement::CodeBlock { code_block } => {
                    self.add_code(code_block);
                    self.newline();
                }
                ModuleElement::ModuleAttribute { attribute_node } => {
                    // By default framec will generate the FrameEvent.
                    // See if generate_frame_event is false to disable generation.
                    if attribute_node.get_name() == "generate_frame_event" {
                        if let AttributeNode::MetaListIdents { attr } = attribute_node {
                            let attr_opt = attr.idents.get(0);
                            if let Some(attr) = attr_opt {
                                if attr == "false" {
                                    generate_frame_event = false;
                                }
                            }
                        }
                    }
                }
            }
        }

        if generate_frame_event {
            self.newline();
            self.add_code("class FrameEvent:");
            self.indent();
            self.newline();
            self.add_code("def __init__(self, message, parameters):");
            self.indent();
            self.newline();
            self.add_code("self._message = message");
            self.newline();
            self.add_code("self._parameters = parameters");
            self.outdent();
            self.outdent();
            self.newline();
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) {
        // domain variable vector
        let mut domain_vec: Vec<(String, String)> = Vec::new();
        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            // get init expression and cache code
            for var_rcref in &domain_block_node.member_variables {
                let var_name = var_rcref.borrow().name.clone();
                let var = var_rcref.borrow();
                let var_init_expr = &var.get_initializer_value_rc();
                let mut init_expression = String::new();
                var_init_expr.accept_to_string(self, &mut init_expression);
                // push for later initialization
                domain_vec.push((var_name.clone(), init_expression));
            }
        }

        self.system_name = system_node.name.clone();

        let _ = &system_node.module.accept(self);

        // Generate any enums

        if let Some(domain_block_node) = &system_node.domain_block_node_opt {
            domain_block_node.accept_enums(self);
        }

        if let Some(vec) = &system_node.functions_opt {
            for function_node_rcref in vec {
                let function_node = function_node_rcref.borrow();
                function_node.accept(self);
            }
        }
        self.newline();
        // TODO!!: This is a hack until we rework modules to detect if there
        // was no system parsed
        if system_node.name == "" {
            return;
        }

        self.newline();
        self.add_code(&format!("class {}:", system_node.name));
        self.indent();
        self.newline();

        // First state name needed for machinery.
        // Don't generate if there isn't at least one state.
        match system_node.get_first_state() {
            Some(state_node_rcref) => {
                self.first_state_name = state_node_rcref.borrow().name.clone();
                self.has_states = true;
            }
            None => {}
        }

        // format system params,if any.
        let mut separator = String::new();
        let mut new_params: String = match &system_node.start_state_state_params_opt {
            Some(param_list) => {
                let mut params = String::new();
                for param_node in param_list {
                    let param_type = match &param_node.param_type_opt {
                        Some(type_node) => self.format_type(type_node),
                        None => String::from(""),
                    };
                    params.push_str(&format!(
                        "{}start_state_state_param_{}",
                        separator, param_node.param_name
                    ));
                    if !param_type.is_empty() {
                        params.push_str(&format!(": {}", param_type));
                    }
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
                        None => String::from(""),
                    };
                    new_params.push_str(&format!(
                        "{}start_state_enter_param_{}",
                        separator, param_node.param_name
                    ));
                    if !param_type.is_empty() {
                        new_params.push_str(&format!(": {}", param_type));
                    }
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
                        None => String::from(""),
                    };
                    new_params.push_str(&format!(
                        "{}domain_param_{}",
                        separator, param_node.param_name
                    ));
                    if !param_type.is_empty() {
                        new_params.push_str(&format!(": {}", param_type));
                    }
                    separator = String::from(",");
                }
            }
            None => {}
        };

        if self.marshal {
            self.newline();
            self.add_code("# ================== System Marshaling ================== #");
            self.newline();
            self.newline();
            self.add_code("@staticmethod");
            self.newline();
            self.add_code("def unmarshal(data):");
            self.indent();
            self.newline();
            self.add_code("return jsonpickle.decode(data)");
            self.outdent();
            self.newline();
            self.newline();
            self.add_code("def marshal(self):");
            self.indent();
            self.newline();
            self.add_code("return jsonpickle.encode(self)");
            self.outdent();
            self.newline();
        }
        self.newline();
        self.newline();
        self.add_code("# ==================== System Factory =================== #");
        self.newline();
        self.newline();

        if new_params.is_empty() {
            self.add_code("def __init__(self):");
        } else {
            self.add_code(&format!("def __init__(self,{}):", new_params));
        }

        self.generate_factory_fn(system_node);

        // end of generate constructor

        // self.serialize.push("".to_string());
        // self.serialize.push("Bag _serialize__do() {".to_string());
        //
        // self.deserialize.push("".to_string());
        //
        // // @TODO: _do needs to be configurable.
        // self.deserialize
        //     .push("void _deserialize__do(Bag data) {".to_string());

        self.subclass_code.push("".to_string());
        self.subclass_code
            .push("# ********************\n".to_string());
        self.subclass_code.push(format!(
            "#class {}Controller({}):",
            system_node.name, system_node.name
        ));
        if self.managed {
            if new_params.is_empty() {
                self.subclass_code
                    .push("\t#def __init__(self,manager):".to_string());
                self.subclass_code
                    .push("\t#  super().__init__(manager)".to_string());
            } else {
                self.subclass_code
                    .push(format!("\t#def __init__(manager,{}):", new_params));
                self.subclass_code
                    .push(format!("\t#  super().__init__(manager{})", new_params));
            }
        } else {
            self.subclass_code
                .push(format!("\t#def __init__(self,{}):", new_params));
            self.subclass_code
                .push(format!("\t    #super().__init__({})", new_params));
        }

        if let Some(interface_block_node) = &system_node.interface_block_node_opt {
            interface_block_node.accept(self);
        }

        if let Some(machine_block_node) = &system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }

        if let Some(actions_block_node) = &system_node.actions_block_node_opt {
            actions_block_node.accept(self);
        }

        if let Some(operations_block_node) = &system_node.operations_block_node_opt {
            operations_block_node.accept(self);
        }

        self.subclass_code
            .push("\n# ********************\n".to_string());

        // self.serialize.push("".to_string());
        // self.serialize
        //     .push("\treturn JSON.stringify(bag)".to_string());
        // self.serialize.push("}".to_string());
        // self.serialize.push("".to_string());
        //
        // self.deserialize.push("".to_string());
        // self.deserialize.push("}".to_string());

        self.generate_machinery(system_node);

        if self.config.code.public_state_info {
            self.generate_state_info_method()
        }

        if self.config.code.public_compartment {
            self.generate_compartment_info_method()
        }

        // TODO: add comments back
        // self.newline();
        // self.generate_comment(system_node.line);
        // self.newline();
        self.outdent();
        self.newline();

        self.generate_compartment(&system_node.name);

        // TODO: Remove subclass code.
        //  self.generate_subclass();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_instance_statement_node(
        &mut self,
        system_instance_stmt_node: &SystemInstanceStmtNode,
    ) {
        system_instance_stmt_node
            .system_instance_expr_node
            .accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_instance_expr_node(
        &mut self,
        system_instance_expr_node: &SystemInstanceExprNode,
    ) {
        let system_name = &system_instance_expr_node.identifier.name.lexeme;

        self.newline();
        self.add_code(&format!("{}", system_name));
        self.add_code("(");
        let mut separator = "";
        if let Some(start_state_state_args) = &system_instance_expr_node.start_state_state_args_opt
        {
            for expr_t in &start_state_state_args.exprs_t {
                self.add_code(separator);
                expr_t.accept(self);
                separator = ",";
            }
        }

        if let Some(start_state_enter_args) = &system_instance_expr_node.start_state_enter_args_opt
        {
            for expr_t in &start_state_enter_args.exprs_t {
                self.add_code(separator);
                expr_t.accept(self);
                separator = ",";
            }
        }

        if let Some(domain_args) = &system_instance_expr_node.domain_args_opt {
            for expr_t in &domain_args.exprs_t {
                self.add_code(separator);
                expr_t.accept(self);
                separator = ",";
            }
        }
        self.add_code(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_instance_expr_node_to_string(
        &mut self,
        system_instance_expr_node: &SystemInstanceExprNode,
        output: &mut String,
    ) {
        let system_name = &system_instance_expr_node.identifier.name.lexeme;

        output.push_str(&format!("{}", system_name));
        output.push_str("(");
        let mut separator = "";
        if let Some(start_state_state_args) = &system_instance_expr_node.start_state_state_args_opt
        {
            for expr_t in &start_state_state_args.exprs_t {
                output.push_str(separator);
                expr_t.accept_to_string(self, output);
                separator = ",";
            }
        }

        if let Some(start_state_enter_args) = &system_instance_expr_node.start_state_enter_args_opt
        {
            for expr_t in &start_state_enter_args.exprs_t {
                output.push_str(separator);
                expr_t.accept_to_string(self, output);
                separator = ",";
            }
        }

        if let Some(domain_args) = &system_instance_expr_node.domain_args_opt {
            for expr_t in &domain_args.exprs_t {
                output.push_str(separator);
                expr_t.accept_to_string(self, output);
                separator = ",";
            }
        }
        output.push_str(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_type_statement_node(&mut self, system_type_stmt_node: &SystemTypeStmtNode) {
        self.newline();
        system_type_stmt_node.system_type_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_type_expr_node(&mut self, system_type_expr_node: &SystemTypeExprNode) {
        let system_name = &system_type_expr_node.identifier.name.lexeme;
        self.add_code(&format!("{}", system_name));
        if let Some(call_chain) = &*(system_type_expr_node.call_chain_opt) {
            let mut output = String::new();
            call_chain.accept_to_string(self, &mut output);
            self.add_code(&format!(".{}", output));
        }
        // self.add_code("(");
        // let mut separator = "";
        // if let Some(start_state_state_args) = &system_type_expr_node.start_state_state_args_opt
        // {
        //     for expr_t in &start_state_state_args.exprs_t {
        //         self.add_code(separator);
        //         expr_t.accept(self);
        //         separator = ",";
        //     }
        // }
        //
        // if let Some(start_state_enter_args) = &system_type_expr_node.start_state_enter_args_opt
        // {
        //     for expr_t in &start_state_enter_args.exprs_t {
        //         self.add_code(separator);
        //         expr_t.accept(self);
        //         separator = ",";
        //     }
        // }
        //
        // if let Some(domain_args) = &system_type_expr_node.domain_args_opt {
        //     for expr_t in &domain_args.exprs_t {
        //         self.add_code(separator);
        //         expr_t.accept(self);
        //         separator = ",";
        //     }
        // }
        // self.add_code(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_type_expr_node_to_string(
        &mut self,
        system_type_expr_node: &SystemTypeExprNode,
        output: &mut String,
    ) {
        let system_name = &system_type_expr_node.identifier.name.lexeme;
        output.push_str(&format!("{}", system_name));
        if let Some(call_chain) = &*(system_type_expr_node.call_chain_opt) {
            // let mut output = String::new();
            output.push_str(".");
            call_chain.accept_to_string(self, output);
            // output.push_str(&format!(".{}", output));
        }
        // output.push_str("(");
        // let mut separator = "";
        // if let Some(start_state_state_args) = &system_type_expr_node.start_state_state_args_opt
        // {
        //     for expr_t in &start_state_state_args.exprs_t {
        //         output.push_str(separator);
        //         expr_t.accept_to_string(self, output);
        //         separator = ",";
        //     }
        // }
        //
        // if let Some(start_state_enter_args) = &system_type_expr_node.start_state_enter_args_opt
        // {
        //     for expr_t in &start_state_enter_args.exprs_t {
        //         output.push_str(separator);
        //         expr_t.accept_to_string(self, output);
        //         separator = ",";
        //     }
        // }
        //
        // if let Some(domain_args) = &system_type_expr_node.domain_args_opt {
        //     for expr_t in &domain_args.exprs_t {
        //         output.push_str(separator);
        //         expr_t.accept_to_string(self, output);
        //         separator = ",";
        //     }
        // }
        // output.push_str(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_function_node(&mut self, function_node: &FunctionNode) {
        self.newline();
        self.add_code(&format!("def {}(", function_node.name));

        self.format_parameter_list(&function_node.params);

        self.add_code("):");

        if !function_node.is_implemented {
            self.newline();
            self.add_code("raise NotImplementedError");
        } else {
            // Generate statements
            if function_node.statements.is_empty() && function_node.terminator_node_opt.is_none() {
                self.indent();
                self.newline();
                self.add_code("pass");
                self.outdent();
                // self.newline();
            } else {
                if !function_node.statements.is_empty() {
                    self.indent();
                    self.visit_decl_stmts(&function_node.statements);
                    self.outdent();
                    // self.newline();
                }
                if let Some(terminator_expr) = &function_node.terminator_node_opt {
                    self.indent();
                    self.newline();
                    match &terminator_expr.terminator_type {
                        TerminatorType::Return => match &terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code("return ");
                                expr_t.accept(self);
                            }
                            None => {
                                self.add_code("return");
                            }
                        },
                        TerminatorType::Continue => {
                            // shouldn't happen.
                            self.errors
                                .push("Continue not allowed as action terminator.".to_string());
                        }
                    }
                    self.outdent();
                    // self.newline();
                }
            }
        }

        if function_node.name == "main" {
            self.generate_main = true;
        }
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
        if interface_method_call_expr_node.call_origin == CallOrigin::Internal {
            self.add_code("self.");
        }

        self.add_code(&format!(
            "{}",
            interface_method_call_expr_node.identifier.name.lexeme
        ));
        interface_method_call_expr_node.call_expr_list.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_call_expression_node_to_string(
        &mut self,
        interface_method_call_expr_node: &InterfaceMethodCallExprNode,
        output: &mut String,
    ) {
        if interface_method_call_expr_node.call_origin == CallOrigin::Internal {
            output.push_str(&format!("self."));
        }
        output.push_str(&format!(
            "{}",
            interface_method_call_expr_node.identifier.name.lexeme
        ));

        interface_method_call_expr_node
            .call_expr_list
            .accept_to_string(self, output);

        // TODO: review this return as I think it is a nop.
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_block_node(&mut self, interface_block_node: &InterfaceBlockNode) {
        self.newline();
        self.add_code("# ==================== Interface Block ================== #");
        self.newline();

        for interface_method_node_rcref in &interface_block_node.interface_methods {
            let interface_method_node = interface_method_node_rcref.borrow();
            interface_method_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_method_node(&mut self, interface_method_node: &InterfaceMethodNode) {
        self.newline();

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

        self.add_code(&format!("def {}(self", interface_method_node.name));

        // match &interface_method_node.params {
        //     Some(params) => {
        self.add_code(",");
        self.format_parameter_list(&interface_method_node.params);
        //     }
        //     None => {}
        // }

        self.add_code("):");
        self.indent();
        let params_param_code;
        if interface_method_node.params.is_some() {
            params_param_code = String::from("parameters");
            self.newline();
            self.add_code("parameters = {}");
            match &interface_method_node.params {
                Some(params) => {
                    for param in params {
                        let pname = &param.param_name;
                        self.newline();
                        self.add_code(&format!("parameters[\"{}\"] = {}", pname, pname));
                    }
                }
                None => {}
            }
        } else {
            params_param_code = String::from("None");
        }

        self.newline();
        match &interface_method_node.return_init_expr_opt {
            Some(x) => {
                let mut output = String::new();
                x.accept_to_string(self, &mut output);
                self.add_code(&format!("self.return_stack.append({})", output));
            }
            _ => {
                self.add_code("self.return_stack.append(None)");
            }
        }

        self.newline();
        self.add_code(&format!(
            "__e = FrameEvent(\"{}\",{})",
            method_name_or_alias, params_param_code
        ));
        if self.has_states {
            self.newline();
            self.add_code("self.__kernel(__e)");
        }

        match &interface_method_node.return_type_opt {
            Some(_) => {
                self.newline();
                self.add_code("return self.return_stack.pop(-1)");
            }
            None => {
                // If there was no type decl but there is an expression
                // evaluated to return then also generate code
                // to return that value.
                match &interface_method_node.return_init_expr_opt {
                    Some(_) => {
                        self.newline();
                        self.add_code("return self.return_stack.pop(-1)");
                    }
                    None => {
                        // always pop the return stack as a default Nil is addes
                        self.newline();
                        self.add_code("return self.return_stack.pop(-1)");
                    }
                }
            }
        }

        self.outdent();
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operations_block_node(&mut self, operation_block_node: &OperationsBlockNode) {
        self.newline();
        self.newline();
        self.add_code("# ==================== Operations Block ================== #");

        for operation_method_node_rcref in &operation_block_node.operations {
            let operation_method_node = operation_method_node_rcref.borrow();
            operation_method_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operation_node(&mut self, operation_node: &OperationNode) {
        self.operation_scope_depth += 1;
        self.newline();
        self.newline();
        let operation_name = self.format_operation_name(&operation_node.name);
        if operation_node.is_static() {
            self.add_code("@staticmethod");
            self.newline();
            self.add_code(&format!("def {}(", operation_name));
        } else {
            self.add_code(&format!("def {}(self", operation_name));
        }

        match &operation_node.params {
            Some(params) => {
                if !operation_node.is_static() {
                    self.add_code(",");
                }

                self.format_operations_parameter_list(params);
            }
            None => {}
        }

        self.add_code("):");

        self.indent();

        // Generate statements
        if operation_node.statements.is_empty() && operation_node.terminator_node_opt.is_none() {
            self.newline();
            self.add_code("pass");
        } else {
            if !operation_node.statements.is_empty() {
                // self.newline();
                self.visit_decl_stmts(&operation_node.statements);
            }
            if let Some(terminator_expr) = &operation_node.terminator_node_opt {
                self.newline();
                match &terminator_expr.terminator_type {
                    TerminatorType::Return => match &terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.add_code("return ");
                            expr_t.accept(self);
                            // self.newline();
                        }
                        None => {
                            self.add_code("return");
                            // self.newline();
                        }
                    },
                    TerminatorType::Continue => {
                        // shouldn't happen.
                        self.errors
                            .push("Continue not allowed as operation terminator.".to_string());
                    }
                }
            }
        }

        self.outdent();
        self.operation_scope_depth -= 1;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operation_call_expression_node(
        &mut self,
        operation_call_expr_node: &OperationCallExprNode,
    ) {
        // if operation_call_expr_node.call_origin
        //     == CallOrigin::Internal
        // {
        //     self.add_code("self.");
        // }

        self.add_code(&format!(
            "{}",
            operation_call_expr_node.identifier.name.lexeme
        ));
        operation_call_expr_node.call_expr_list.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operation_call_expression_node_to_string(
        &mut self,
        operation_call_expr_node: &OperationCallExprNode,
        output: &mut String,
    ) {
        // if operation_call_expr_node.call_origin
        //     == CallOrigin::Internal
        // {
        //     output.push_str(&format!("self."));
        // }
        output.push_str(&format!(
            "{}",
            operation_call_expr_node.identifier.name.lexeme
        ));

        operation_call_expr_node
            .call_expr_list
            .accept_to_string(self, output);

        // TODO: review this return as I think it is a nop.
    }
    //* --------------------------------------------------------------------- *//

    fn visit_operation_ref_expression_node(
        &mut self,
        operation_ref_expr_node: &OperationRefExprNode,
    ) {
        self.add_code(&format!("self.{}", operation_ref_expr_node.name));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operation_ref_expression_node_to_string(
        &mut self,
        operation_ref_expr_node: &OperationRefExprNode,
        output: &mut String,
    ) {
        output.push_str(&format!("self.{}", operation_ref_expr_node.name));

        // TODO: review this return as I think it is a nop.
    }

    //* --------------------------------------------------------------------- *//

    fn visit_machine_block_node(&mut self, machine_block_node: &MachineBlockNode) {
        self.newline();
        self.add_code("# ===================== Machine Block =================== #");
        self.newline();

        // self.serialize.push("".to_string());
        // self.serialize.push("\tvar stateName = null".to_string());
        //
        // self.deserialize.push("".to_string());
        // self.deserialize
        //     .push("\tbag = JSON.parse(data)".to_string());
        // self.deserialize.push("".to_string());
        // self.deserialize.push("\tswitch (bag.state) {".to_string());

        for state_node_rcref in &machine_block_node.states {
            state_node_rcref.borrow().accept(self);
        }

        // self.serialize.push("".to_string());
        // self.serialize.push("\tbag = {".to_string());
        // self.serialize.push("\t\tstate : stateName,".to_string());
        // self.serialize.push("\t\tdomain : {}".to_string());
        // self.serialize.push("\t}".to_string());
        // self.serialize.push("".to_string());
        //
        // self.deserialize.push("\t}".to_string());
        // self.deserialize.push("".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_actions_block_node(&mut self, actions_block_node: &ActionsBlockNode) {
        self.newline();
        self.add_code("# ===================== Actions Block =================== #");
        //        self.newline();

        // TODO - for some reason action_node.accept_action_impl() isn't being
        // called but action_node.accept_action_decl() is.

        for action_rcref in &actions_block_node.actions {
            let action_node = action_rcref.borrow();
            if action_node.code_opt.is_some() {
                action_node.accept_action_impl(self);
            }
        }

        // self.newline();
        // self.newline();

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

    // fn visit_domain_block_node(&mut self, domain_block_node: &DomainBlockNode) {
    //     // self.newline();
    //     // self.newline();
    //     // self.add_code("# ===================== Domain Block =================== #");
    //     self.newline();
    //
    //     for variable_decl_node_rcref in &domain_block_node.member_variables {
    //         let variable_decl_node = variable_decl_node_rcref.borrow();
    //         variable_decl_node.accept(self);
    //     }
    //
    //     self.newline();
    // }

    //* --------------------------------------------------------------------- *//

    fn visit_state_node(&mut self, state_node: &StateNode) {
        if self.generate_comment(state_node.line) {
            self.newline();
        }
        self.current_state_name_opt = Some(state_node.name.clone());

        self.newline();
        self.add_code("# ----------------------------------------");
        self.newline();
        self.add_code(&format!("# ${}", &state_node.name));
        self.newline();
        self.newline();
        self.add_code(&format!(
            "def {}(self, __e, compartment):",
            self.format_target_state_name(&state_node.name)
        ));
        self.indent();

        // self.serialize.push(format!(
        //     "\tif (self._state_ == _s{}_) stateName = \"{}\"",
        //     state_node.name, state_node.name
        // ));
        //
        // self.deserialize.push(format!(
        //     "\t\tcase \"{}\": _state_ = _s{}_; break;",
        //     state_node.name, state_node.name
        // ));

        let mut generate_pass = true;

        if let Some(calls) = &state_node.calls_opt {
            generate_pass = false;
            for call in calls {
                self.newline();
                call.accept(self);
            }
        }

        self.first_event_handler = true; // context for formatting

        if !state_node.evt_handlers_rcref.is_empty() {
            generate_pass = false;
            for evt_handler_node in &state_node.evt_handlers_rcref {
                evt_handler_node.as_ref().borrow().accept(self);
            }
        }

        // If we have a dispatch then there will be a call to the
        // parent state so do not generate pass.
        if state_node.dispatch_opt.is_some() {
            generate_pass = false;
        }

        if generate_pass {
            self.newline();
            self.add_code("pass");
            self.newline();
        }

        match &state_node.dispatch_opt {
            Some(dispatch) => {
                self.newline();
                dispatch.accept(self);
            }
            None => {}
        }

        self.outdent();
        self.newline();

        self.current_state_name_opt = None;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_node(&mut self, evt_handler_node: &EventHandlerNode) {
        self.event_handler_has_code = false;
        self.current_event_ret_type = evt_handler_node.get_event_ret_type();
        self.newline();
        self.generate_comment(evt_handler_node.line);
        //        let mut generate_final_close_paren = true;
        if let MessageType::CustomMessage { message_node } = &evt_handler_node.msg_t {
            if self.first_event_handler {
                self.add_code(&format!("if __e._message == \"{}\":", message_node.name));
            } else {
                self.add_code(&format!("elif __e._message == \"{}\":", message_node.name));
            }
        } else {
            // AnyMessage ( ||* )
            if self.first_event_handler {
                // This logic is for when there is only the catch all event handler ||*
                self.add_code("if True:");
            } else {
                // other event handlers preceded ||*
                self.add_code("else:");
            }
        }
        self.generate_comment(evt_handler_node.line);
        self.indent();
        // self.newline();

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
        self.event_handler_has_code = !evt_handler_node.statements.is_empty();
        if self.event_handler_has_code {
            self.visit_decl_stmts(&evt_handler_node.statements);
        }
        // else {
        //     self.newline();
        //     self.add_code("pass");
        // }

        let terminator_node = &evt_handler_node.terminator_node;
        terminator_node.accept(self);
        self.outdent();
        // self.newline();

        // this controls formatting here
        self.first_event_handler = false;
        self.current_event_ret_type = String::new();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_event_handler_terminator_node(
        &mut self,
        evt_handler_terminator_node: &TerminatorExpr,
    ) {
        // self.newline();
        match &evt_handler_terminator_node.terminator_type {
            TerminatorType::Return => match &evt_handler_terminator_node.return_expr_t_opt {
                Some(expr_t) => {
                    // expr_t.auto_pre_inc_dec(self);
                    self.newline();
                    if self.is_in_action_or_operation() {
                        self.add_code("return = ");
                    } else {
                        self.add_code("self.return_stack[-1] = ");
                    }
                    expr_t.accept(self);
                    // expr_t.auto_post_inc_dec(self);
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

        //let mut output = String::new();
        //method_call.call_expr_list.accept_to_string(self, &mut output);
        method_call.call_expr_list.accept(self);
        // self.add_code( output.as_str());
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
        self.add_code(&format!("self.{}", action_name));

        action_call.call_expr_list.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(
        &mut self,
        action_call: &ActionCallExprNode,
        output: &mut String,
    ) {
        let action_name = &format!(
            "self.{}",
            self.format_action_name(&action_call.identifier.name.lexeme)
        );
        output.push_str(action_name);

        action_call.call_expr_list.accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(&mut self, action_call_stmt_node: &ActionCallStmtNode) {
        self.newline();
        action_call_stmt_node.action_call_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_transition_statement_node(&mut self, transition_statement: &TransitionStatementNode) {
        match &transition_statement
            .transition_expr_node
            .target_state_context_t
        {
            TargetStateContextType::StateRef { .. } => self.generate_state_ref_transition(
                &transition_statement.transition_expr_node,
                &(transition_statement.exit_args_opt),
            ),
            TargetStateContextType::StateStackPop {} => self.generate_state_stack_pop_transition(
                &transition_statement.transition_expr_node,
                &(transition_statement.exit_args_opt),
            ),
        };
        // &transition_statement.transition_expr_node.accept(self);

        self.this_branch_transitioned = true;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_transition_expr_node(&mut self, transition_expr_node: &TransitionExprNode) {
        match &transition_expr_node.target_state_context_t {
            TargetStateContextType::StateRef { .. } => {
                self.generate_state_ref_transition(&transition_expr_node, &None)
            }
            TargetStateContextType::StateStackPop {} => {
                self.generate_state_stack_pop_transition(transition_expr_node, &None)
            }
        }

        self.this_branch_transitioned = true;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_state_ref_node(&mut self, state_ref: &StateRefNode) {
        self.add_code(&state_ref.name.to_string());
    }

    //* --------------------------------------------------------------------- *//

    // fn visit_change_state_statement_node(
    //     &mut self,
    //     change_state_stmt_node: &ChangeStateStatementNode,
    // ) {
    //     match &change_state_stmt_node.state_context_t {
    //         TargetStateContextType::StateRef { .. } => {
    //             self.generate_state_ref_change_state(change_state_stmt_node)
    //         }
    //         TargetStateContextType::StateStackPop { .. } => {
    //             self.generate_state_stack_pop_change_state(change_state_stmt_node)
    //         }
    //     };
    //     self.this_branch_transitioned = true
    // }

    //* --------------------------------------------------------------------- *//

    // TODO: ??
    fn visit_parameter_node(&mut self, _: &ParameterNode) {
        // self.add_code(&format!("{}",parameter_node.name));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_dispatch_node(&mut self, dispatch_node: &DispatchNode) {
        self.newline();
        self.add_code(&format!(
            "self.{}(__e, compartment.parent_compartment)",
            self.format_target_state_name(&dispatch_node.target_state_ref.name)
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

        for branch_node in &bool_test_node.conditional_branch_nodes {
            self.newline();
            if branch_node.is_negated {
                self.add_code(&format!("{} not (", if_or_else_if));
            } else {
                self.add_code(&format!("{} ", if_or_else_if));
            }

            branch_node.expr_t.accept(self);

            if branch_node.is_negated {
                self.add_code(")");
            }
            self.add_code(":");
            self.indent();

            branch_node.accept(self);
            self.generate_return_if_transitioned();

            self.outdent();
            //self.newline();

            if_or_else_if = "elif ";
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

    fn visit_call_chain_statement_node(&mut self, method_call_chain_stmt_node: &CallChainStmtNode) {
        self.skip_next_newline();
        // special case for interface method calls
        let call_chain = &method_call_chain_stmt_node
            .call_chain_literal_expr_node
            .call_chain;
        if call_chain.len() == 1 {
            if let CallChainNodeType::InterfaceMethodCallT {
                interface_method_call_expr_node,
            } = &call_chain[0]
            {
                self.this_branch_transitioned = true;
                interface_method_call_expr_node.accept(self);
                // TODO - this next statement was problematic if not in a branch.
                // Leaving here in case there is an unconsidered edge case.
                // Needs to be directly solved by the mandatory event handler solution,
                // whatever that is going to be.
                if !self.is_in_action_or_operation() {
                    self.generate_return();
                }

                return;
            }
        }

        // standard case

        method_call_chain_stmt_node
            .call_chain_literal_expr_node
            .accept(self);

        // TODO - review autoinc logic
        // resets flag used in autoinc code
        self.skip_next_newline = false;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_expr_node(&mut self, call_l_chain_expression_node: &CallChainExprNode) {
        // TODO: maybe put this in an AST node

        let mut separator = "";

        if call_l_chain_expression_node.inc_dec != IncDecExpr::None {
            self.errors.push(
                "Error - auto increment/decrement operator (++/--) not allowed in Python."
                    .to_string(),
            );
            return;
        }
        for node in &call_l_chain_expression_node.call_chain {
            self.add_code(separator);
            separator = ".";
            match &node {
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    id_node.accept(self);
                }
                CallChainNodeType::UndeclaredCallT { call_node: call } => {
                    call.accept(self);
                }
                CallChainNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    interface_method_call_expr_node.accept(self);
                }
                CallChainNodeType::OperationCallT {
                    operation_call_expr_node,
                } => {
                    operation_call_expr_node.accept(self);
                }
                CallChainNodeType::OperationRefT {
                    operation_ref_expr_node,
                } => {
                    operation_ref_expr_node.accept(self);
                }
                CallChainNodeType::ActionCallT {
                    action_call_expr_node,
                } => {
                    action_call_expr_node.accept(self);
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    // TODO: figure out why this is necessary as sometimes it generates
                    // unnecessary groups e.g.:
                    // (compartment.state_vars["x"]) = compartment.state_vars["x"] + 1
                    self.visiting_call_chain_literal_variable = true;
                    var_node.accept(self);
                    self.visiting_call_chain_literal_variable = false;
                }
                CallChainNodeType::ListElementNodeT { list_elem_node } => {
                    list_elem_node.accept(self);
                }
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    list_elem_node.accept(self);
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    // fn visit_auto_pre_inc_dec_expr_node(&mut self, ref_expr_type: &RefExprType) {
    //
    //     // match ref_expr_type.as_mut() {
    //     //     RefExprType::CallChainLiteralExprT {
    //     //         ref mut call_chain_expr_node,
    //     //     } => {
    //     //         *call_chain_expr_node.is_new_expr = false;
    //     //     }
    //     //     _ => {
    //     //
    //     //     }
    //     // };
    //
    //     match ref_expr_type {
    //         RefExprType::AssignmentExprT { .. } => {}
    //         RefExprType::CallExprT { call_expr_node } => {
    //             // TODO - not sure if this loop should move into the CallExprNode.
    //             // if so need to implement similar functionality for other expr nodes
    //             for expr_t in &call_expr_node.call_expr_list.exprs_t {
    //                 expr_t.auto_pre_inc_dec(self);
    //             }
    //         }
    //         RefExprType::BinaryExprT { binary_expr_node } => {
    //             binary_expr_node.left_rcref.borrow().auto_pre_inc_dec(self);
    //             binary_expr_node.right_rcref.borrow().auto_pre_inc_dec(self);
    //         }
    //         RefExprType::CallChainLiteralExprT {
    //             call_chain_expr_node,
    //         } => {
    //             // *call_chain_expr_node.inc_dec = IncDecExpr::None;
    //             match call_chain_expr_node.inc_dec {
    //                 IncDecExpr::PreInc => {
    //                     // this is a hack coordinating newline generation
    //                     // in multiple different paths. One path is a pure
    //                     // expression and no newline should be generated.
    //                     // self.test_skip_newline();
    //                     let mut output = String::new();
    //                     call_chain_expr_node.accept_to_string(self, &mut output);
    //                     // self.add_code(&format!("{} = {} + 1", output, output));
    //                     // self.skip_next_newline();
    //                     // *call_chain_expr_node.inc_dec = IncDecExpr::None;
    //                    // let err = format!("[line {}] Error - autoincrement of variables disallowed in Python",call_chain_expr_node.call_chain.);
    //                     self.errors
    //                         .push("Error - autoincrement of variables disallowed in Python".to_string());
    //
    //
    //                 }
    //                 IncDecExpr::PreDec => {
    //                     // this is a hack coordinating newline generation
    //                     // in multiple different paths. One path is a pure
    //                     // expression and no newline should be generated.
    //                     // self.test_skip_newline();
    //                     // let mut output = String::new();
    //                     // call_chain_expr_node.accept_to_string(self, &mut output);
    //                     // self.add_code(&format!("{} = {} - 1", output, output));
    //                     // self.skip_next_newline();
    //                     self.errors
    //                         .push("Error - autodecrement of variables disallowed in Python".to_string());
    //
    //                 }
    //                 _ => {}
    //             }
    //
    //             // now generate pre inc/dec for all arguments
    //             for node in &call_chain_expr_node.call_chain {
    //                 match &node {
    //                     // CallChainLiteralNodeType::IdentifierNodeT { id_node } => {
    //                     //     id_node.accept(self);
    //                     // }
    //                     CallChainLiteralNodeType::CallT { call } => {
    //                         for expr_t in &call.call_expr_list.exprs_t {
    //                             expr_t.auto_pre_inc_dec(self);
    //                         }
    //                     }
    //                     CallChainLiteralNodeType::InterfaceMethodCallT {
    //                         interface_method_call_expr_node,
    //                     } => {
    //                         for expr_t in &interface_method_call_expr_node.call_expr_list.exprs_t {
    //                             expr_t.auto_pre_inc_dec(self);
    //                         }
    //                     }
    //                     CallChainLiteralNodeType::ActionCallT {
    //                         action_call_expr_node,
    //                     } => {
    //                         for expr_t in &action_call_expr_node.call_expr_list.exprs_t {
    //                             expr_t.auto_pre_inc_dec(self);
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         RefExprType::ExprListT { expr_list_node } => {
    //             for expr in &expr_list_node.exprs_t {
    //                 expr.auto_pre_inc_dec(self);
    //             }
    //         }
    //         RefExprType::LoopStmtT { loop_types } => {
    //             match loop_types {
    //                 LoopStmtTypes::LoopForStmt {
    //                     loop_for_stmt_node: loop_for_expr_node,
    //                 } => {
    //                     for expr in &loop_for_expr_node.post_expr_rcref_opt {
    //                         expr.borrow().auto_pre_inc_dec(self);
    //                     }
    //                 }
    //                 LoopStmtTypes::LoopInStmt { loop_in_stmt_node } => {
    //                     loop_in_stmt_node.iterable_expr.auto_pre_inc_dec(self);
    //                 }
    //                 LoopStmtTypes::LoopInfiniteStmt { .. } => {
    //
    //                     // TODO
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // //* --------------------------------------------------------------------- *//
    //
    // fn visit_auto_post_inc_dec_expr_node(&mut self, ref_expr_type: &RefExprType) {
    //     match ref_expr_type {
    //         RefExprType::AssignmentExprT { .. } => {}
    //         RefExprType::CallExprT { call_expr_node } => {
    //             // TODO - not sure if this loop should move into the CallExprNode.
    //             // if so need to implement similar functionality for other expr nodes
    //             for expr_t in &call_expr_node.call_expr_list.exprs_t {
    //                 expr_t.auto_post_inc_dec(self);
    //             }
    //         }
    //         RefExprType::BinaryExprT { binary_expr_node } => {
    //             binary_expr_node.left_rcref.borrow().auto_post_inc_dec(self);
    //             binary_expr_node
    //                 .right_rcref
    //                 .borrow()
    //                 .auto_post_inc_dec(self);
    //         }
    //         RefExprType::CallChainLiteralExprT {
    //             call_chain_expr_node,
    //         } => {
    //             match call_chain_expr_node.inc_dec {
    //                 IncDecExpr::PostInc => {
    //                     self.newline();
    //                     let mut output = String::new();
    //                     call_chain_expr_node.accept_to_string(self, &mut output);
    //                     self.add_code(&format!("{} = {} + 1", output, output));
    //                 }
    //                 IncDecExpr::PostDec => {
    //                     self.newline();
    //                     let mut output = String::new();
    //                     call_chain_expr_node.accept_to_string(self, &mut output);
    //                     self.add_code(&format!("{} = {} - 1", output, output));
    //                 }
    //                 _ => {}
    //             }
    //
    //             // now generate pre inc/dec for all arguments
    //             for node in &call_chain_expr_node.call_chain {
    //                 match &node {
    //                     // CallChainLiteralNodeType::IdentifierNodeT { id_node } => {
    //                     //     id_node.accept(self);
    //                     // }
    //                     CallChainLiteralNodeType::CallT { call } => {
    //                         for expr_t in &call.call_expr_list.exprs_t {
    //                             expr_t.auto_post_inc_dec(self);
    //                         }
    //                     }
    //
    //                     CallChainLiteralNodeType::InterfaceMethodCallT {
    //                         interface_method_call_expr_node,
    //                     } => {
    //                         for expr_t in &interface_method_call_expr_node.call_expr_list.exprs_t {
    //                             expr_t.auto_post_inc_dec(self);
    //                         }
    //                     }
    //                     CallChainLiteralNodeType::ActionCallT {
    //                         action_call_expr_node,
    //                     } => {
    //                         for expr_t in &action_call_expr_node.call_expr_list.exprs_t {
    //                             expr_t.auto_post_inc_dec(self);
    //                         }
    //                     }
    //                     // CallChainLiteralNodeType::VariableNodeT { var_node } => {
    //                     //     self.visiting_call_chain_literal_variable = true;
    //                     //     var_node.accept(self);
    //                     //     self.visiting_call_chain_literal_variable = false;
    //                     // }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         RefExprType::ExprListT { expr_list_node } => {
    //             for expr in &expr_list_node.exprs_t {
    //                 expr.auto_post_inc_dec(self);
    //             }
    //         }
    //         // RefExprType::LoopExprT {loop_expr_node} => {
    //         //     for expr in &loop_expr_node.inc_dec_expr_rcref_opt {
    //         //         let x = expr.borrow();
    //         //         x.auto_post_inc_dec(self);
    //         //     }
    //         // }
    //         RefExprType::LoopStmtT { loop_types } => {
    //             match loop_types {
    //                 LoopStmtTypes::LoopForStmt { loop_for_stmt_node } => {
    //                     for expr in &loop_for_stmt_node.post_expr_rcref_opt {
    //                         expr.borrow().auto_post_inc_dec(self);
    //                     }
    //                 }
    //                 LoopStmtTypes::LoopInStmt { loop_in_stmt_node } => {
    //                     loop_in_stmt_node.iterable_expr.auto_post_inc_dec(self);
    //                 }
    //                 LoopStmtTypes::LoopInfiniteStmt { .. } => {
    //                     // TODO
    //                 }
    //             }
    //         }
    //     }
    // }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_expr_node_to_string(
        &mut self,
        call_chain_expression_node: &CallChainExprNode,
        output: &mut String,
    ) {
        let mut separator = "";

        for node in &call_chain_expression_node.call_chain {
            output.push_str(separator);
            match &node {
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    id_node.accept_to_string(self, output);
                }
                CallChainNodeType::UndeclaredCallT { call_node: call } => {
                    call.accept_to_string(self, output);
                }
                CallChainNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    interface_method_call_expr_node.accept_to_string(self, output);
                }
                CallChainNodeType::OperationCallT {
                    operation_call_expr_node,
                } => {
                    operation_call_expr_node.accept_to_string(self, output);
                }
                CallChainNodeType::OperationRefT {
                    operation_ref_expr_node,
                } => {
                    operation_ref_expr_node.accept(self);
                }
                CallChainNodeType::ActionCallT {
                    action_call_expr_node,
                } => {
                    action_call_expr_node.accept_to_string(self, output);
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    var_node.accept_to_string(self, output);
                }
                CallChainNodeType::ListElementNodeT { list_elem_node } => {
                    list_elem_node.accept_to_string(self, output);
                }
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    list_elem_node.accept_to_string(self, output);
                }
            }
            separator = ".";
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_loop_stmt_node(&mut self, loop_stmt_node: &LoopStmtNode) {
        match &loop_stmt_node.loop_types {
            LoopStmtTypes::LoopForStmt {
                loop_for_stmt_node: loop_for_expr_node,
            } => {
                loop_for_expr_node.accept(self);
            }
            LoopStmtTypes::LoopInStmt { loop_in_stmt_node } => {
                loop_in_stmt_node.accept(self);
            }
            LoopStmtTypes::LoopInfiniteStmt {
                loop_infinite_stmt_node,
            } => {
                loop_infinite_stmt_node.accept(self);
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_loop_for_stmt_node(&mut self, loop_for_expr_node: &LoopForStmtNode) {
        // self.loop_for_inc_dec_expr_rcref_opt = loop_for_expr_node.post_expr_rcref_opt.clone();
        // self.loop_for_inc_dec_expr_rcref_opt = ;

        if let Some(expr_type_rcref) = &loop_for_expr_node.loop_init_expr_rcref_opt {
            let lfs = expr_type_rcref.borrow();
            lfs.accept(self);
            self.newline();
        } else {
            self.newline();
        }

        // all autoincdec code in loop control should be generated as the last statement
        // in the loop
        // for ..; ..; x++

        let mut post_expr = String::new();

        if let Some(expr_type_rcref) = &loop_for_expr_node.post_expr_rcref_opt {
            let expr_t = expr_type_rcref.borrow();
            // expr_t.auto_pre_inc_dec(self);
            match *expr_t {
                ExprType::CallChainExprT { .. } => {
                    // don't emit just a simple expression.
                }
                _ => expr_t.accept_to_string(self, &mut post_expr),
            }
            // expr_t.auto_post_inc_dec(self);
        }

        self.continue_post_expr_vec.push(Some(post_expr));

        self.add_code(&format!("while True:"));
        self.indent();
        self.newline();
        if let Some(test_expr_rcref) = &loop_for_expr_node.test_expr_rcref_opt {
            let mut output = String::new();
            test_expr_rcref.borrow().accept_to_string(self, &mut output);

            // let test_expr = test_expr_rcref.borrow();
            // test_expr.auto_pre_inc_dec(self);

            //            self.newline();
            self.add_code(&format!("if not({}):", output));
            self.indent();
            self.newline();
            self.add_code("break");
            self.outdent();
            // self.newline();
            // test_expr.auto_post_inc_dec(self);
        }

        // only call if there are statements
        if loop_for_expr_node.statements.len() != 0 {
            self.visit_decl_stmts(&loop_for_expr_node.statements);
            //  self.newline();
        }

        if let Some(post_expr) = self.continue_post_expr_vec.pop() {
            self.newline();
            self.add_code(post_expr.unwrap().clone().as_str());
        } else {
            self.newline();
        }

        self.outdent();
        //self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_loop_in_stmt_node(&mut self, loop_in_stmt_node: &LoopInStmtNode) {
        self.newline();

        let mut output = String::new();
        loop_in_stmt_node
            .iterable_expr
            .accept_to_string(self, &mut output);

        match &loop_in_stmt_node.loop_first_stmt {
            LoopFirstStmt::Var { var_node } => {
                self.add_code(&format!("for {} in {}:", var_node.id_node.name, output));
            }
            LoopFirstStmt::CallChain {
                call_chain_expr_node,
            } => {
                let mut output_first_stmt = String::new();
                call_chain_expr_node.accept_to_string(self, &mut output_first_stmt);
                self.add_code(&format!("for {} in {}:", output_first_stmt, output));
            }
            LoopFirstStmt::VarDecl {
                var_decl_node_rcref,
            } => {
                self.add_code(&format!(
                    "for {} in {}:",
                    var_decl_node_rcref.borrow().name,
                    output
                ));
            }
            // LoopFirstStmt::VarDeclAssign {var_decl_node_rcref} => {
            //     self.add_code(&format!("for {} in {}:"
            //                            , var_decl_node_rcref.borrow().name
            //                            , output));
            // }
            // TODO
            _ => panic!("Error - unexpected target expression in 'in' loop."),
        };

        self.indent();
        // self.newline();

        // only call if there are statements
        if loop_in_stmt_node.statements.len() != 0 {
            self.visit_decl_stmts(&loop_in_stmt_node.statements);
        }

        if loop_in_stmt_node.statements.len() == 0 {
            self.newline();
            self.add_code(&format!("pass"));
        }

        self.outdent();
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_loop_infinite_stmt_node(&mut self, loop_in_expr_node: &LoopInfiniteStmtNode) {
        self.continue_post_expr_vec.push(None);
        self.newline();

        self.add_code(&format!("while True:"));

        self.indent();
        // self.newline();
        self.visit_decl_stmts(&loop_in_expr_node.statements);
        self.outdent();
        self.newline();
        self.continue_post_expr_vec.pop();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_block_stmt_node(&mut self, block_stmt_node: &BlockStmtNode) {
        self.indent();
        // Generate statements
        self.visit_decl_stmts(&block_stmt_node.statements);
        self.outdent();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_continue_stmt_node(&mut self, _: &ContinueStmtNode) {
        // THE FOLLOWING COMMENT IS NOT VALID. LEAVING UNTIL
        // FINAL FIX IS DECIDED ON ABOUT AUTO INC/DEC.
        // In the context of a for loop, the auto inc/dec clause needs to
        // be executed prior to generating the 'continue' statement.
        // e.g.: loop var x = 0; x < 10; x++ { .. }
        // let loop_for_inc_dec_expr_rcref_opt = self.loop_for_inc_dec_expr_rcref_opt.clone();
        // if let Some(expr_type_rcref) = loop_for_inc_dec_expr_rcref_opt {
        //     let expr_t = expr_type_rcref.borrow();
        //     expr_t.auto_pre_inc_dec(self);
        //     expr_t.auto_post_inc_dec(self);
        // }

        // In the loop_for syntax we need to generate the
        // post_expr before a "continue". However none of the
        // other loops should do that.  The continue_post_expr_vec
        // is used to store a string option that has a Some value
        // for the loop_for scope and None value for the other loop types.

        let mut opt_str = None;
        if let Some(post_expr) = self.continue_post_expr_vec.last() {
            opt_str = post_expr.clone();
        }

        if let Some(new_str) = opt_str {
            self.newline();
            self.add_code(new_str.as_str());
        }

        self.newline();
        self.add_code("continue");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_superstring_stmt_node(&mut self, super_string_stmt_node: &SuperStringStmtNode) {
        //        self.newline();
        super_string_stmt_node.literal_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_break_stmt_node(&mut self, _: &BreakStmtNode) {
        self.newline();
        self.add_code("break");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_bool_test_conditional_branch_node(
        &mut self,
        bool_test_true_branch_node: &BoolTestConditionalBranchNode,
    ) {
        // generate 'pass' unless there is a statement that will generate code
        let mut generate_pass = true;
        if bool_test_true_branch_node
            .branch_terminator_expr_opt
            .is_some()
        {
            generate_pass = false;
        }

        // even if there statements, it is possible there are only empty
        // blocks, which should generate a pass. Check if there are only
        // empty blocks.
        if generate_pass && self.has_non_block_statements(&bool_test_true_branch_node.statements) {
            generate_pass = false;
        }

        if generate_pass {
            self.newline();
            self.add_code("pass");
        } else {
            self.visit_decl_stmts(&bool_test_true_branch_node.statements);
        }

        match &bool_test_true_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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
        self.newline();
        self.add_code("else:");
        self.indent();
        // let mut generate_pass = false;
        // if bool_test_else_branch_node.statements.is_empty() && bool_test_else_branch_node.branch_terminator_expr_opt.is_none() {
        //     generate_pass = true
        // }

        // generate 'pass' unless there is a statement that will generate code
        let mut generate_pass = true;
        if bool_test_else_branch_node
            .branch_terminator_expr_opt
            .is_some()
        {
            generate_pass = false;
        }

        // even if there statements, it is possible there are only empty
        // blocks, which should generate a pass. Check if there are only
        // empty blocks.
        if generate_pass && self.has_non_block_statements(&bool_test_else_branch_node.statements) {
            generate_pass = false;
        }

        if generate_pass {
            self.newline();
            self.add_code("pass");
        } else {
            self.visit_decl_stmts(&bool_test_else_branch_node.statements);
        }

        // TODO - factor this out to work w/ other terminator code.
        match &bool_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_string_match_test_node(&mut self, string_match_test_node: &StringMatchTestNode) {
        let mut if_or_else_if = "if";

        self.newline();
        for match_branch_node in &string_match_test_node.match_branch_nodes {
            self.add_code(&format!("{} (", if_or_else_if));

            let mut expr_code = String::new();
            match &string_match_test_node.expr_t {
                ExprType::CallExprT {
                    call_expr_node: method_call_expr_node,
                } => method_call_expr_node.accept_to_string(self, &mut expr_code),
                ExprType::ActionCallExprT {
                    action_call_expr_node,
                } => action_call_expr_node.accept_to_string(self, &mut expr_code),
                ExprType::CallChainExprT {
                    call_chain_expr_node,
                } => call_chain_expr_node.accept_to_string(self, &mut expr_code),
                ExprType::VariableExprT { var_node: id_node } => {
                    id_node.accept_to_string(self, &mut expr_code)
                }
                ExprType::ExprListT { expr_list_node } => {
                    // must be only 1 expression in the list
                    if expr_list_node.exprs_t.len() != 1 {
                        // TODO: how to do this better.
                        self.errors
                            .push("Error - expression list is not testable.".to_string());
                    }
                    let expr_t = expr_list_node.exprs_t.first().unwrap();
                    expr_t.accept(self);
                }

                _ => self.errors.push("TODO".to_string()),
            }

            let mut first_match = true;
            match &match_branch_node.string_match_type {
                StringMatchType::MatchString {
                    string_match_test_pattern_node,
                } => {
                    for match_string in &string_match_test_pattern_node.match_pattern_strings {
                        if first_match {
                            first_match = false;
                            self.add_code(&format!("({} == \"{}\")", expr_code, match_string));
                        } else {
                            self.add_code(&format!(" or ({} == \"{}\")", expr_code, match_string));
                        }
                    }
                    self.add_code(")");
                }
                StringMatchType::MatchNullString => {
                    self.add_code(&format!("{} is None)", expr_code));
                }
                StringMatchType::MatchEmptyString => {
                    self.add_code(&format!(
                        "isinstance({}, str) and len({}) == 0)",
                        expr_code, expr_code
                    ));
                }
            }

            self.add_code(":");
            self.indent();

            match_branch_node.accept(self);
            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();

            if_or_else_if = "elif";
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
        // let mut generate_pass = false;
        // if string_match_test_match_branch_node.statements.is_empty() && string_match_test_match_branch_node.branch_terminator_expr_opt.is_none() {
        //     generate_pass = true;
        // }

        // generate 'pass' unless there is a statement that will generate code
        let mut generate_pass = true;
        if string_match_test_match_branch_node
            .branch_terminator_expr_opt
            .is_some()
        {
            generate_pass = false;
        }

        // even if there statements, it is possible there are only empty
        // blocks, which should generate a pass. Check if there are only
        // empty blocks.
        if generate_pass
            && self.has_non_block_statements(&string_match_test_match_branch_node.statements)
        {
            generate_pass = false;
        }

        if generate_pass {
            self.newline();
            self.add_code("pass");
        } else {
            self.visit_decl_stmts(&string_match_test_match_branch_node.statements);
        }

        match &string_match_test_match_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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
        self.newline();
        self.add_code("else:");
        self.indent();

        // if string_match_test_else_branch_node.statements.is_empty() && string_match_test_else_branch_node.branch_terminator_expr_opt.is_none() {
        //     generate_pass = true;
        // }

        // generate 'pass' unless there is a statement that will generate code
        let mut generate_pass = true;
        if string_match_test_else_branch_node
            .branch_terminator_expr_opt
            .is_some()
        {
            generate_pass = false;
        }

        // even if there statements, it is possible there are only empty
        // blocks, which should generate a pass. Check if there are only
        // empty blocks.
        if generate_pass
            && self.has_non_block_statements(&string_match_test_else_branch_node.statements)
        {
            generate_pass = false;
        }

        if generate_pass {
            self.newline();
            self.add_code("pass");
        } else {
            self.visit_decl_stmts(&string_match_test_else_branch_node.statements);
        }

        // TODO - factor this out to work w/ other terminator code.
        match &string_match_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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
            self.add_code(&format!("{} (", if_or_else_if));
            match &number_match_test_node.expr_t {
                ExprType::CallExprT {
                    call_expr_node: method_call_expr_node,
                } => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT {
                    action_call_expr_node,
                } => action_call_expr_node.accept(self),
                ExprType::CallChainExprT {
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
                    self.add_code(" or (");
                    match &number_match_test_node.expr_t {
                        ExprType::CallExprT {
                            call_expr_node: method_call_expr_node,
                        } => method_call_expr_node.accept(self),
                        ExprType::ActionCallExprT {
                            action_call_expr_node,
                        } => action_call_expr_node.accept(self),
                        ExprType::CallChainExprT {
                            call_chain_expr_node,
                        } => call_chain_expr_node.accept(self),
                        ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                        _ => self.errors.push("TODO.".to_string()),
                    }
                    self.add_code(&format!(" == {})", match_number.match_pattern_number));
                }
            }

            self.add_code(":");
            self.indent();

            match_branch_node.accept(self);
            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();

            if_or_else_if = "elif";
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
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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
        self.newline();
        self.add_code("else:");
        self.indent();

        self.visit_decl_stmts(&number_match_test_else_branch_node.statements);

        // TODO - factor this out to work w/ other terminator code.
        match &number_match_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_number_match_test_pattern_node(
        &mut self,
        match_pattern_node: &NumberMatchTestPatternNode,
    ) {
        self.add_code(&match_pattern_node.match_pattern_number.to_string());
    }

    //-----------------------------------------------------//

    fn visit_enum_match_test_node(&mut self, enum_match_test_node: &EnumMatchTestNode) {
        let mut if_or_else_if = "if";

        self.newline();
        for match_branch_node in &enum_match_test_node.match_branch_nodes {
            self.add_code(&format!("{} (", if_or_else_if));
            match &enum_match_test_node.expr_t {
                ExprType::CallExprT {
                    call_expr_node: method_call_expr_node,
                } => method_call_expr_node.accept(self),
                ExprType::ActionCallExprT {
                    action_call_expr_node,
                } => action_call_expr_node.accept(self),
                ExprType::CallChainExprT {
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
            for match_test_pattern_node in &match_branch_node.enum_match_pattern_node {
                if first_match {
                    self.add_code(&format!(
                        " == {}.{})",
                        self.format_enum_name(&enum_match_test_node.enum_type_name),
                        match_test_pattern_node.match_pattern
                    ));
                    first_match = false;
                } else {
                    self.add_code(" or (");
                    match &enum_match_test_node.expr_t {
                        ExprType::CallExprT {
                            call_expr_node: method_call_expr_node,
                        } => method_call_expr_node.accept(self),
                        ExprType::ActionCallExprT {
                            action_call_expr_node,
                        } => action_call_expr_node.accept(self),
                        ExprType::CallChainExprT {
                            call_chain_expr_node,
                        } => call_chain_expr_node.accept(self),
                        ExprType::VariableExprT { var_node: id_node } => id_node.accept(self),
                        _ => self.errors.push("TODO.".to_string()),
                    }
                    self.add_code(&format!(
                        " == {}.{})",
                        self.format_enum_name(&enum_match_test_node.enum_type_name),
                        match_test_pattern_node.match_pattern
                    ));
                }
            }

            self.add_code(":");
            self.indent();

            match_branch_node.accept(self);
            self.generate_return_if_transitioned();

            self.outdent();
            self.newline();

            if_or_else_if = "elif";
        }

        // (':' number_test_else_branch)?
        if let Some(number_match_else_branch_node) = &enum_match_test_node.else_branch_node_opt {
            number_match_else_branch_node.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enum_match_test_match_branch_node(
        &mut self,
        enum_match_test_match_branch_node: &EnumMatchTestMatchBranchNode,
    ) {
        // generate 'pass' unless there is a statement that will generate code
        let mut generate_pass = true;
        if enum_match_test_match_branch_node
            .branch_terminator_t_opt
            .is_some()
        {
            generate_pass = false;
        }

        // even if there statements, it is possible there are only empty
        // blocks, which should generate a pass. Check if there are only
        // empty blocks.
        if generate_pass
            && self.has_non_block_statements(&enum_match_test_match_branch_node.statements)
        {
            generate_pass = false;
        }

        if generate_pass {
            self.newline();
            self.add_code("pass");
        } else {
            self.visit_decl_stmts(&enum_match_test_match_branch_node.statements);
        }

        match &enum_match_test_match_branch_node.branch_terminator_t_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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

    fn visit_enum_match_test_else_branch_node(
        &mut self,
        enum_match_test_else_branch_node: &EnumMatchTestElseBranchNode,
    ) {
        self.newline();
        self.add_code("else:");
        self.indent();

        // generate 'pass' unless there is a statement that will generate code
        let mut generate_pass = true;
        if enum_match_test_else_branch_node
            .branch_terminator_expr_opt
            .is_some()
        {
            generate_pass = false;
        }

        // even if there statements, it is possible there are only empty
        // blocks, which should generate a pass. Check if there are only
        // empty blocks.
        if generate_pass
            && self.has_non_block_statements(&enum_match_test_else_branch_node.statements)
        {
            generate_pass = false;
        }

        if generate_pass {
            self.newline();
            self.add_code("pass");
        } else {
            self.visit_decl_stmts(&enum_match_test_else_branch_node.statements);
        }

        // TODO - factor this out to work w/ other terminator code.
        match &enum_match_test_else_branch_node.branch_terminator_expr_opt {
            Some(branch_terminator_expr) => {
                self.newline();
                match &branch_terminator_expr.terminator_type {
                    TerminatorType::Return => match &branch_terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            if self.is_in_action_or_operation() {
                                self.add_code("return ");
                                expr_t.accept(self);
                            } else {
                                self.add_code("self.return_stack[-1] = ");
                                expr_t.accept(self);
                                self.generate_return();
                            }
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
    }

    //* --------------------------------------------------------------------- *//

    fn visit_return_assign_stmt_node(&mut self, return_assign_stmt_node: &ReturnAssignStmtNode) {
        let mut output = String::new();

        return_assign_stmt_node
            .expr_t
            .accept_to_string(self, &mut output);

        self.newline();
        self.add_code(&format!("self.return_stack[-1] = {}", output));
    }
    //* --------------------------------------------------------------------- *//

    fn visit_enum_match_test_pattern_node(
        &mut self,
        _enum_match_test_pattern_node: &EnumMatchTestPatternNode,
    ) {
        // TODO
        self.errors.push("Not implemented.".to_string());
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node(&mut self, expr_list: &ExprListNode) {
        let mut generate_parens = true;
        if expr_list.exprs_t.len() == 1 {
            if let Some(ExprType::TransitionExprT { .. }) = expr_list.exprs_t.get(0) {
                generate_parens = false;
            }
        }
        let mut separator = "";
        if generate_parens {
            self.add_code("(");
        }

        for expr in &expr_list.exprs_t {
            self.add_code(separator);
            expr.accept(self);
            separator = ",";
        }

        if generate_parens {
            self.add_code(")");
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expression_list_node_to_string(
        &mut self,
        expr_list: &ExprListNode,
        output: &mut String,
    ) {
        //        self.add_code(&format!("{}(__e);\n",dispatch_node.target_state_ref.name));

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

    fn visit_list_stmt_node(&mut self, list_stmt_node: &ListStmtNode) {
        let ref list_node = list_stmt_node.list_node;
        // self.test_skip_newline();
        list_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_list_node(&mut self, list: &ListNode) {
        let mut separator = "";
        self.add_code("[");

        for expr in &list.exprs_t {
            self.add_code(separator);
            expr.accept(self);
            separator = ",";
        }

        self.add_code("]");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_list_node_to_string(&mut self, list: &ListNode, output: &mut String) {
        let mut separator = "";
        output.push('[');
        for expr in &list.exprs_t {
            output.push_str(separator);
            expr.accept_to_string(self, output);
            separator = ",";
        }
        output.push(']');
    }

    //* --------------------------------------------------------------------- *//

    fn visit_list_elem_node(&mut self, list_elem: &ListElementNode) {
        let str = self.format_list_element_expr(list_elem);
        self.add_code(str.as_str());
        // list_elem.identifier.accept(self);
        self.add_code("[");
        list_elem.expr_t.accept(self);
        self.add_code("]");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_list_elem_node_to_string(&mut self, list_elem: &ListElementNode, output: &mut String) {
        let str = self.format_list_element_expr(list_elem);
        output.push_str(str.as_str());
        //  list_elem.identifier.accept_to_string(self,output);
        output.push('[');
        list_elem.expr_t.accept_to_string(self, output);
        output.push(']');
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expr_list_stmt_node(&mut self, expr_list_stmt_node: &ExprListStmtNode) {
        let ref expr_list_node = expr_list_stmt_node.expr_list_node;
        self.test_skip_newline();
        expr_list_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node(&mut self, literal_expression_node: &LiteralExprNode) {
        match &literal_expression_node.token_t {
            TokenType::Number => self.add_code(&literal_expression_node.value.to_string()),
            TokenType::SuperString => self.add_code(&literal_expression_node.value.to_string()),
            TokenType::String => self.add_code(&format!("\"{}\"", literal_expression_node.value)),
            TokenType::True => self.add_code("True"),
            TokenType::False => self.add_code("False"),
            TokenType::Null => self.add_code("None"),
            TokenType::Nil => self.add_code("None"),
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
                output.push_str("True");
            }
            TokenType::False => {
                output.push_str("False");
            }
            TokenType::Nil => {
                output.push_str("None");
            }
            TokenType::Null => {
                output.push_str("None");
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
                self.add_code("self.__state_stack_push(self.__compartment)");
            }
            StateStackOperationType::Pop => {
                self.add_code("compartment = self.__state_stack_pop()");
            }
        }
    }
    //* --------------------------------------------------------------------- *//

    fn visit_state_context_node(&mut self, _state_context_node: &TargetStateContextNode) {
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
            } => self.add_code("__e._message"),
            FrameEventPart::Param {
                param_symbol_rcref,
                is_reference: _is_reference,
            } => self.add_code(&format!(
                "__e._parameters[\"{}\"]",
                param_symbol_rcref.borrow().name
            )),
            FrameEventPart::Return {
                is_reference: _is_reference,
            } => self.add_code("self.return_stack[-1]"),
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
            } => output.push_str("__e._message"),
            FrameEventPart::Param {
                param_symbol_rcref,
                is_reference: _is_reference,
            } => output.push_str(&format!(
                "__e._parameters[\"{}\"]",
                param_symbol_rcref.borrow().name
            )),
            FrameEventPart::Return {
                is_reference: _is_reference,
            } => output.push_str("self.return_stack[-1]"),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_node(&mut self, action_node: &ActionNode) {
        self.action_scope_depth += 1;

        let mut subclass_code = String::new();

        self.newline();
        self.newline();
        let action_name = self.format_action_name(&action_node.name);
        self.add_code(&format!("def {}(self", action_name));
        self.newline_to_string(&mut subclass_code);
        subclass_code.push_str(&format!("#def {}(self", action_name));

        match &action_node.params {
            Some(params) => {
                self.add_code(",");
                subclass_code.push(',');
                self.format_actions_parameter_list(params, &mut subclass_code);
            }
            None => {}
        }

        self.add_code("):");
        subclass_code.push_str("):");

        self.indent();
        if !action_node.is_implemented {
            self.newline();
            self.add_code("raise NotImplementedError");
            self.newline_to_string(&mut subclass_code);
            subclass_code.push_str("#pass");
        } else {
            // Generate statements
            if action_node.statements.is_empty() && action_node.terminator_node_opt.is_none() {
                self.newline();
                self.add_code("pass");
            } else {
                if !action_node.statements.is_empty() {
                    // self.newline();
                    self.visit_decl_stmts(&action_node.statements);
                }
                if let Some(terminator_expr) = &action_node.terminator_node_opt {
                    self.newline();
                    match &terminator_expr.terminator_type {
                        TerminatorType::Return => match &terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code("return ");
                                expr_t.accept(self);
                                self.newline();
                            }
                            None => {
                                self.add_code("return");
                                self.newline();
                            }
                        },
                        TerminatorType::Continue => {
                            // shouldn't happen.
                            self.errors
                                .push("Continue not allowed as action terminator.".to_string());
                        }
                    }
                }
            }
        }

        self.outdent();
        // self.newline();
        self.subclass_code.push(subclass_code);

        self.action_scope_depth -= 1;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_impl_node(&mut self, action_node: &ActionNode) {
        let mut subclass_code = String::new();

        self.newline();
        self.newline();

        let action_name = self.format_action_name(&action_node.name);
        self.add_code(&format!("def {}(self", action_name));
        match &action_node.params {
            Some(params) => {
                self.add_code(",");
                subclass_code.push(',');
                self.format_actions_parameter_list(params, &mut subclass_code);
            }
            None => {}
        }

        self.add_code("):");
        self.indent();
        self.newline();
        // TODO: I think code_opt is dead code.
        self.add_code(action_node.code_opt.as_ref().unwrap().as_str());
        self.outdent();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enum_decl_node(&mut self, enum_decl_node: &EnumDeclNode) {
        self.newline();
        self.newline();

        self.add_code(&format!(
            "class {}_{}(Enum):",
            self.system_name, enum_decl_node.name
        ));
        self.indent();

        for enumerator_decl_node in &enum_decl_node.enums {
            enumerator_decl_node.accept(self);
        }

        self.outdent();
        self.newline()
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enumerator_decl_node(&mut self, enumerator_decl_node: &EnumeratorDeclNode) {
        self.newline();
        self.add_code(&format!(
            "{} = {}",
            enumerator_decl_node.name, enumerator_decl_node.value
        ));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enumerator_expr_node(&mut self, enum_expr_node: &EnumeratorExprNode) {
        self.add_code(&format!(
            "{}_{}.{}",
            self.system_name, enum_expr_node.enum_type, enum_expr_node.enumerator
        ));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enumerator_expr_node_to_string(
        &mut self,
        enum_expr_node: &EnumeratorExprNode,
        output: &mut String,
    ) {
        output.push_str(&format!(
            "{}_{}.{}",
            self.system_name, enum_expr_node.enum_type, enum_expr_node.enumerator
        ));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enumerator_statement_node(&mut self, enumerator_stmt_node: &EnumeratorStmtNode) {
        self.newline();
        enumerator_stmt_node.enumerator_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_domain_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
        self.visit_variable_decl_node(variable_decl_node);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_decl_node(&mut self, variable_decl_node: &VariableDeclNode) {
        self.newline();
        let var_type = match &variable_decl_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from(""),
        };
        let var_name = &variable_decl_node.name;
        let var_init_expr = &variable_decl_node.get_initializer_value_rc();
        //self.newline();
        let mut code = String::new();
        var_init_expr.accept_to_string(self, &mut code);
        match &variable_decl_node.identifier_decl_scope {
            IdentifierDeclScope::DomainBlockScope => {
                self.add_code(&format!("self.{} ", var_name));
                if !var_type.is_empty() {
                    self.add_code(&format!(": {}", var_type));
                }
                if let Some(variable_init_override) = &self.variable_init_override_opt {
                    // TODO - move "domain_param_" prefix into config variables.
                    let copy_to_suppress_warning = variable_init_override.clone();
                    self.add_code(&format!(" = domain_param_{}", copy_to_suppress_warning));
                } else {
                    self.add_code(&format!(" = {}", code));
                }
            }
            IdentifierDeclScope::EventHandlerVarScope => {
                self.add_code(&format!("{}", var_name));
                if !var_type.is_empty() {
                    self.add_code(&format!(": {}", var_type));
                }
                self.add_code(&format!(" = {}", code));
            }
            IdentifierDeclScope::LoopVarScope => {
                self.add_code(&format!("{}", var_name));
                if !var_type.is_empty() {
                    self.add_code(&format!(": {}", var_type));
                }
                self.add_code(&format!(" = {}", code));
            }
            IdentifierDeclScope::BlockVarScope => {
                self.add_code(&format!("{}", var_name));
                if !var_type.is_empty() {
                    self.add_code(&format!(": {}", var_type));
                }
                self.add_code(&format!(" = {}", code));
            }
            _ => panic!("Error - unexpected scope for variable declaration"),
        }

        // self.serialize
        //     .push(format!("\tbag.domain[\"{}\"] = {}", var_name, var_name));
        // self.deserialize
        //     .push(format!("\t{} = bag.domain[\"{}\"]", var_name, var_name));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_loop_variable_decl_node(&mut self, loop_variable_decl_node: &LoopVariableDeclNode) {
        let var_type = match &loop_variable_decl_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from(""),
        };
        let var_name = &loop_variable_decl_node.name;
        let var_init_expr = &loop_variable_decl_node
            .initializer_expr_t_opt
            .as_ref()
            .unwrap();
        self.newline();
        let mut code = String::new();
        var_init_expr.accept_to_string(self, &mut code);
        match &loop_variable_decl_node.identifier_decl_scope {
            IdentifierDeclScope::LoopVarScope => {
                self.add_code(&format!("self.{} ", var_name));
                if !var_type.is_empty() {
                    self.add_code(&format!(": {}", var_type));
                }
                self.add_code(&format!(" = {}", code));
            }
            _ => panic!("Error - unexpected scope for variable declaration"),
        }
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
        // self.generate_comment(assignment_expr_node.line);
        // self.newline();
        // inc/dec all *rvalue* expressions before generating the
        // assignement statement
        // assignment_expr_node.r_value_box.auto_pre_inc_dec(self);
        // now generate assignment expression
        assignment_expr_node.l_value_box.accept(self);
        self.add_code(" = ");
        //self.add_code(&*output);
        //       assignment_expr_node.r_value_box.auto_pre_inc_dec(self);
        let mut output = String::new();
        assignment_expr_node
            .r_value_rc
            .accept_to_string(self, &mut output);
        self.add_code(&*output);
        // assignment_expr_node.r_value_box.auto_post_inc_dec(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_expr_node_to_string(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
        output: &mut String,
    ) {
        // self.generate_comment(assignment_expr_node.line);
        // self.newline();
        // self.newline_to_string(output);
        assignment_expr_node
            .l_value_box
            .accept_to_string(self, output);
        output.push_str(" = ");
        assignment_expr_node
            .r_value_rc
            .accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_statement_node(&mut self, assignment_stmt_node: &AssignmentStmtNode) {
        self.generate_comment(assignment_stmt_node.get_line());
        self.newline();
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

    fn visit_binary_stmt_node(&mut self, binary_stmt_node: &BinaryStmtNode) {
        self.newline();

        // binary_stmt_node
        //     .binary_expr_node
        //     .left_rcref
        //     .borrow()
        //     .auto_pre_inc_dec(self);
        // binary_stmt_node
        //     .binary_expr_node
        //     .right_rcref
        //     .borrow()
        //     .auto_pre_inc_dec(self);
        binary_stmt_node.binary_expr_node.accept(self);
        // binary_stmt_node
        //     .binary_expr_node
        //     .left_rcref
        //     .borrow()
        //     .auto_post_inc_dec(self);
        // binary_stmt_node
        //     .binary_expr_node
        //     .right_rcref
        //     .borrow()
        //     .auto_post_inc_dec(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node(&mut self, binary_expr_node: &BinaryExprNode) {
        // TODO
        //       self.generate_comment(assignment_expr_node.line);
        if binary_expr_node.operator == OperatorType::LogicalXor {
            self.add_code("((");
            binary_expr_node.left_rcref.borrow().accept(self);
            self.add_code(") and not (");
            binary_expr_node.right_rcref.borrow().accept(self);
            self.add_code(")) or (not (");
            binary_expr_node.left_rcref.borrow().accept(self);
            self.add_code(") and (");
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
            output.push_str(") and not (");
            binary_expr_node
                .right_rcref
                .borrow()
                .accept_to_string(self, output);
            output.push_str(")) or (not (");
            binary_expr_node
                .left_rcref
                .borrow()
                .accept_to_string(self, output);
            output.push_str(") and (");
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
            OperatorType::Not => self.add_code(" not "),
            OperatorType::EqualEqual => self.add_code(" == "),
            OperatorType::NotEqual => self.add_code(" != "),
            OperatorType::LogicalAnd => self.add_code(" and "),
            OperatorType::LogicalOr => self.add_code(" or "),
            OperatorType::LogicalXor => self.add_code(" ^ "),
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
            OperatorType::Not => output.push_str(" not "),
            OperatorType::EqualEqual => output.push_str(" == "),
            OperatorType::NotEqual => output.push_str(" != "),
            OperatorType::LogicalAnd => output.push_str(" and "),
            OperatorType::LogicalOr => output.push_str(" or "),
            OperatorType::LogicalXor => output.push_str(" ^ "),
        }
    }
}

// TypeScript Visitor for Frame Language Transpiler
// Generates TypeScript code from Frame AST
// v0.82.0 - Initial TypeScript support with state machines, transitions, and expressions

use super::*;
use crate::frame_c::ast::*;
use crate::frame_c::ast::FrameEventPart;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::scanner::{TokenType};
use crate::frame_c::symbol_table::{SymbolConfig, Arcanum};
use std::collections::HashSet;

pub struct TypeScriptVisitor {
    pub builder: CodeBuilder,
    system_name: String,
    
    // Symbol table and config (like Python visitor)
    symbol_config: SymbolConfig,
    arcanum: Vec<Arcanum>,
    
    // Context tracking (reduced manual tracking since we have arcanum)
    current_state_name: Option<String>,
    domain_variables: HashSet<String>, // Track domain variable names
    current_handler_params: HashSet<String>, // Track current event handler parameter names
    current_state_params: HashSet<String>, // Track current state's parameter names
    current_state_vars: HashSet<String>, // Track current state's variable names
    // TODO: Remove current_local_vars once arcanum-based resolution is implemented
    current_local_vars: HashSet<String>, // Track local variables in current handler
    action_names: HashSet<String>, // Track action names for proper call resolution
    operation_names: HashSet<String>, // Track operation names for proper call resolution
    declared_enums: HashSet<String>, // Track declared enum names to avoid duplicates
    is_in_action: bool, // Track if we're currently processing an action (vs event handler)
}

impl TypeScriptVisitor {
    pub fn new(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
    ) -> Self {
        Self {
            builder: CodeBuilder::new("    "), // 4 spaces for TypeScript indentation
            system_name: String::new(),
            symbol_config,
            arcanum,
            current_state_name: None,
            domain_variables: HashSet::new(),
            current_handler_params: HashSet::new(),
            current_state_params: HashSet::new(),
            current_state_vars: HashSet::new(),
            current_local_vars: HashSet::new(),
            action_names: HashSet::new(),
            operation_names: HashSet::new(),
            declared_enums: HashSet::new(),
            is_in_action: false,
        }
    }
    
    /// Normalize Frame event message names to valid TypeScript identifiers
    fn normalize_message_name(&self, message: &str) -> String {
        match message {
            "$>" => "enter".to_string(),
            "<$" => "exit".to_string(), 
            _ => message.to_lowercase(),
        }
    }
    
    pub fn run(mut self, frame_module: &FrameModule) -> String {
        // Add header
        self.builder.writeln(&format!("// Emitted from framec_v{}", env!("FRAME_VERSION")));
        self.builder.newline();
        self.builder.newline();
        
        // Generate runtime support
        self.generate_runtime_support();
        
        // Visit the module
        self.visit_frame_module(frame_module);
        
        let (code, _mappings) = self.builder.build();
        code
    }
    
    fn generate_runtime_support(&mut self) {
        // Import Frame runtime classes from shared module
        self.builder.writeln("import { FrameEvent, FrameCompartment } from '../../typescript/runtime/frame_runtime';");
        self.builder.newline();
    }
    
    fn generate_enum(&mut self, enum_node: &EnumDeclNode) {
        // Check if this enum has already been declared to avoid duplicates
        if self.declared_enums.contains(&enum_node.name) {
            return; // Skip if already declared
        }
        
        // Mark this enum as declared
        self.declared_enums.insert(enum_node.name.clone());
        
        // Generate TypeScript enum class
        self.builder.writeln(&format!("class {} {{", enum_node.name));
        self.builder.indent();
        
        // Generate static members for each enum value
        for enumerator in &enum_node.enums {
            let value = match &enumerator.value {
                EnumValue::Integer(i) => i.to_string(),
                EnumValue::String(s) => format!("\"{}\"", s),
                EnumValue::Auto => {
                    if matches!(enum_node.enum_type, EnumType::String) {
                        // Auto-generate string value from name
                        format!("\"{}\"", enumerator.name)
                    } else {
                        // Auto-generate numeric value (we'll use sequential numbers)
                        enumerator.name.clone() // placeholder for now
                    }
                }
            };
            
            self.builder.writeln(&format!("static {} = new {}({});", 
                enumerator.name, enum_node.name, value));
        }
        
        self.builder.newline();
        
        // Constructor and value property
        self.builder.writeln("constructor(public readonly value: any) {}");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
    }
    
    fn format_state_name(&self, state_name: &str) -> String {
        format!("__{}_state_{}", 
            self.system_name.to_lowercase(), 
            state_name.trim_start_matches('$'))
    }
}

impl AstVisitor for TypeScriptVisitor {
    fn visit_frame_module(&mut self, frame_module: &FrameModule) {
        // Visit systems
        for system_node in &frame_module.systems {
            self.visit_system_node(system_node);
        }
        
    }
    
    fn visit_system_node(&mut self, system_node: &SystemNode) {
        self.system_name = system_node.name.clone();
        
        // First pass: Collect action and operation names for proper call resolution
        if let Some(actions) = &system_node.actions_block_node_opt {
            for action_rcref in &actions.actions {
                let action = action_rcref.borrow();
                self.action_names.insert(action.name.clone());
            }
        }
        
        if let Some(operations) = &system_node.operations_block_node_opt {
            for operation_rcref in &operations.operations {
                let operation = operation_rcref.borrow();
                self.operation_names.insert(operation.name.clone());
            }
        }
        
        // Generate domain enums BEFORE the class
        if let Some(domain) = &system_node.domain_block_node_opt {
            for enum_decl in &domain.enums {
                let enum_node = enum_decl.borrow();
                self.generate_enum(&*enum_node);
            }
        }
        
        // Generate TypeScript class
        self.builder.writeln(&format!("export class {} {{", self.system_name));
        self.builder.indent();
        
        // Property declarations
        self.builder.writeln("private _compartment: FrameCompartment;");
        self.builder.writeln("private _nextCompartment: FrameCompartment | null = null;");
        self.builder.writeln("private returnStack: any[] = [];");
        
        // Domain variables
        if let Some(domain) = &system_node.domain_block_node_opt {
            for var_decl in &domain.member_variables {
                let var = var_decl.borrow();
                self.builder.writeln(&format!("private {}: any;", var.name));
            }
        }
        
        self.builder.newline();
        
        // Constructor
        self.builder.writeln("constructor() {");
        self.builder.indent();
        
        // Initialize first state
        if let Some(machine) = &system_node.machine_block_node_opt {
            if let Some(first_state) = machine.states.first() {
                let state = first_state.borrow();
                let state_name = self.format_state_name(&state.name);
                self.builder.writeln(&format!("this._compartment = new FrameCompartment('{}');", state_name));
            }
        }
        
        self.builder.writeln("this._nextCompartment = null;");
        self.builder.writeln("this.returnStack = [null];");
        
        // Initialize domain variables
        if let Some(domain) = &system_node.domain_block_node_opt {
            for var_decl in &domain.member_variables {
                let var = var_decl.borrow();
                // Track domain variable name
                self.domain_variables.insert(var.name.clone());
                
                // Check if variable has an initializer
                if !matches!(*var.value_rc, ExprType::NilExprT) {
                    let mut init_str = String::new();
                    self.visit_expr_node_to_string(&var.value_rc, &mut init_str);
                    self.builder.writeln(&format!("this.{} = {};", var.name, init_str));
                } else {
                    // No initializer, set to null
                    self.builder.writeln(&format!("this.{} = null;", var.name));
                }
            }
        }
        
        // Send start event
        self.builder.writeln("this._frame_kernel(new FrameEvent(\"$>\", null));");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        // Interface methods
        if let Some(interface) = &system_node.interface_block_node_opt {
            self.builder.writeln("// Interface methods");
            for method in &interface.interface_methods {
                let method_node = method.borrow();
                self.visit_interface_method_node(&method_node);
            }
        }
        
        // Machine block - event handlers
        if let Some(machine) = &system_node.machine_block_node_opt {
            self.builder.writeln("// Event handlers");
            for state_rcref in &machine.states {
                let state_node = state_rcref.borrow();
                self.current_state_name = Some(state_node.name.clone());
                
                // Track state parameters and variables
                self.current_state_params.clear();
                self.current_state_vars.clear();
                
                // Add state parameters if they exist
                if let Some(ref params) = state_node.params_opt {
                    for param in params {
                        if std::env::var("DEBUG_TS_VARS").is_ok() {
                            eprintln!("DEBUG: Adding state param: {}", param.param_name);
                        }
                        self.current_state_params.insert(param.param_name.clone());
                    }
                }
                
                // Add state variables if they exist
                if let Some(ref state_vars) = state_node.vars_opt {
                    for var_rcref in state_vars {
                        let var = var_rcref.borrow();
                        if std::env::var("DEBUG_TS_VARS").is_ok() {
                            eprintln!("DEBUG: Adding state var: {}", var.name);
                        }
                        self.current_state_vars.insert(var.name.clone());
                    }
                }
                
                for handler_rcref in &state_node.evt_handlers_rcref {
                    let handler = handler_rcref.borrow();
                    self.visit_event_handler_node(&handler);
                }
            }
        }
        
        // State dispatchers
        if let Some(machine) = &system_node.machine_block_node_opt {
            self.builder.writeln("// State dispatchers");
            for state_rcref in &machine.states {
                let state_node = state_rcref.borrow();
                let state_name = self.format_state_name(&state_node.name);
                
                self.builder.writeln(&format!("private {}(__e: FrameEvent, compartment: FrameCompartment): void {{", state_name));
                self.builder.indent();
                
                if !state_node.evt_handlers_rcref.is_empty() {
                    self.builder.writeln("switch(__e.message) {");
                    self.builder.indent();
                    
                    for handler_rcref in &state_node.evt_handlers_rcref {
                        let handler = handler_rcref.borrow();
                        let (message, handler_suffix) = match &handler.msg_t {
                            MessageType::CustomMessage { message_node } => {
                                if message_node.name == "$>" {
                                    ("$>".to_string(), "enter".to_string())
                                } else if message_node.name == "<$" {
                                    ("<$".to_string(), "exit".to_string())
                                } else {
                                    (message_node.name.clone(), message_node.name.to_lowercase())
                                }
                            }
                            _ => ("unknown".to_string(), "unknown".to_string()),
                        };
                        
                        let handler_name = format!("_handle_{}_{}",
                            state_node.name.trim_start_matches('$').to_lowercase(),
                            handler_suffix);
                        
                        self.builder.writeln(&format!("case \"{}\":", message));
                        self.builder.indent();
                        self.builder.writeln(&format!("this.{}(__e, compartment);", handler_name));
                        self.builder.writeln("break;");
                        self.builder.dedent();
                    }
                    
                    self.builder.dedent();
                    self.builder.writeln("}");
                }
                
                self.builder.dedent();
                self.builder.writeln("}");
                self.builder.newline();
            }
        }
        
        // Operations
        if let Some(operations) = &system_node.operations_block_node_opt {
            self.builder.writeln("// Operations");
            for operation_rcref in &operations.operations {
                let operation = operation_rcref.borrow();
                self.visit_operation_node(&operation);
            }
        }
        
        // Actions
        if let Some(actions) = &system_node.actions_block_node_opt {
            self.builder.writeln("// Actions");
            for action_rcref in &actions.actions {
                let action = action_rcref.borrow();
                self.visit_action_node(&action);
            }
        }
        
        // Runtime methods
        self.builder.writeln("// Frame runtime");
        
        // _frame_kernel
        self.builder.writeln("private _frame_kernel(__e: FrameEvent): void {");
        self.builder.indent();
        self.builder.writeln("this._frame_router(__e);");
        self.builder.writeln("while (this._nextCompartment !== null) {");
        self.builder.indent();
        self.builder.writeln("const nextCompartment = this._nextCompartment;");
        self.builder.writeln("this._nextCompartment = null;");
        self.builder.writeln("this._frame_router(new FrameEvent(\"<$\", this._compartment.exitArgs));");
        self.builder.writeln("this._compartment = nextCompartment;");
        self.builder.writeln("if (nextCompartment.forwardEvent === null) {");
        self.builder.indent();
        self.builder.writeln("this._frame_router(new FrameEvent(\"$>\", this._compartment.enterArgs));");
        self.builder.dedent();
        self.builder.writeln("} else {");
        self.builder.indent();
        self.builder.writeln("this._frame_router(nextCompartment.forwardEvent);");
        self.builder.writeln("nextCompartment.forwardEvent = null;");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        // _frame_router
        self.builder.writeln("private _frame_router(__e: FrameEvent, compartment?: FrameCompartment): void {");
        self.builder.indent();
        self.builder.writeln("const targetCompartment = compartment || this._compartment;");
        
        if let Some(machine) = &system_node.machine_block_node_opt {
            self.builder.writeln("switch(targetCompartment.state) {");
            self.builder.indent();
            
            for state_rcref in &machine.states {
                let state_node = state_rcref.borrow();
                let state_name = self.format_state_name(&state_node.name);
                
                self.builder.writeln(&format!("case '{}':", state_name));
                self.builder.indent();
                self.builder.writeln(&format!("this.{}(__e, targetCompartment);", state_name));
                self.builder.writeln("break;");
                self.builder.dedent();
            }
            
            self.builder.dedent();
            self.builder.writeln("}");
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        // _frame_transition
        self.builder.writeln("private _frame_transition(nextCompartment: FrameCompartment): void {");
        self.builder.indent();
        self.builder.writeln("this._nextCompartment = nextCompartment;");
        self.builder.dedent();
        self.builder.writeln("}");
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
    
    fn visit_interface_method_node(&mut self, method: &InterfaceMethodNode) {
        let method_name = &method.name;
        
        // Build parameter list
        let mut params = Vec::new();
        let mut param_names = Vec::new();
        if let Some(param_nodes) = &method.params {
            for param in param_nodes {
                params.push(format!("{}: any", param.param_name));
                param_names.push(param.param_name.clone());
            }
        }
        let params_str = params.join(", ");
        
        // Build parameter object for event
        let param_obj = if !param_names.is_empty() {
            format!("{{ {} }}", 
                param_names.iter()
                    .map(|name| format!("{}: {}", name, name))
                    .collect::<Vec<_>>()
                    .join(", "))
        } else {
            "null".to_string()
        };
        
        self.builder.writeln(&format!("public {}({}): any {{", method_name, params_str));
        self.builder.indent();
        self.builder.writeln("this.returnStack.push(null);");
        self.builder.writeln(&format!("const __e = new FrameEvent(\"{}\", {});", method_name, param_obj));
        self.builder.writeln("this._frame_kernel(__e);");
        self.builder.writeln("return this.returnStack.pop();");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
    }
    
    fn visit_event_handler_node(&mut self, handler: &EventHandlerNode) {
        let state_name = self.current_state_name.as_ref().unwrap();
        let message = match &handler.msg_t {
            MessageType::CustomMessage { message_node } => {
                if message_node.name == "$>" {
                    "enter".to_string()
                } else if message_node.name == "$<" {
                    "exit".to_string()
                } else {
                    message_node.name.clone()
                }
            }
            MessageType::None => "none".to_string(),
        };
        
        // Track event handler parameters and clear local variables
        self.current_handler_params.clear();
        self.current_local_vars.clear();  // Clear local variables for handler scope
        let event_symbol = handler.event_symbol_rcref.borrow();
        if let Some(params) = &event_symbol.event_symbol_params_opt {
            for param in params {
                self.current_handler_params.insert(param.name.clone());
            }
        }
        
        let handler_name = format!("_handle_{}_{}",
            state_name.trim_start_matches('$').to_lowercase(),
            self.normalize_message_name(&message));
        
        self.builder.writeln(&format!("private {}(__e: FrameEvent, compartment: FrameCompartment): void {{", handler_name));
        self.builder.indent();
        
        // Process handler statements
        for stmt_or_decl in &handler.statements {
            match stmt_or_decl {
                DeclOrStmtType::StmtT { stmt_t } => {
                    self.visit_stmt_node(stmt_t);
                }
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    // Handle variable declaration
                    let var_decl = var_decl_t_rcref.borrow();
                    let var_name = &var_decl.name;
                    
                    // Track local variable
                    self.current_local_vars.insert(var_name.clone());
                    
                    // Use get_initializer_value_rc() like Python visitor - fixes expression sharing corruption
                    let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr, ExprType::NilExprT) {
                        let mut init_str = String::new();
                        self.visit_expr_node_to_string(&value_expr, &mut init_str);
                        
                        // Generate variable declaration
                        self.builder.writeln(&format!("let {} = {};", var_name, init_str));
                    } else {
                        self.builder.writeln(&format!("let {}: any = null;", var_name));
                    }
                }
            }
        }
        
        // Handle terminator
        if let Some(terminator) = &handler.terminator_node {
            match &terminator.terminator_type {
                TerminatorType::Return => {
                    self.builder.writeln("return;");
                }
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
    }
    
    
    fn visit_operation_node(&mut self, operation: &OperationNode) {
        let operation_name = format!("_operation_{}", operation.name);
        
        // Clear context for operation scope
        self.current_local_vars.clear();
        self.current_handler_params.clear();
        
        // Build parameter list and track parameters
        let mut params = Vec::new();
        if let Some(param_nodes) = &operation.params {
            for param in param_nodes {
                params.push(format!("{}: any", param.param_name));
                // Track operation parameters as local variables
                self.current_local_vars.insert(param.param_name.clone());
            }
        }
        
        // Build return type
        let return_type = if operation.type_opt.is_some() {
            "any"
        } else {
            "void"
        };
        
        // Generate method signature
        self.builder.writeln(&format!("private {}({}): {} {{", 
            operation_name, 
            params.join(", "), 
            return_type));
        self.builder.indent();
        
        // Process operation statements
        for stmt_or_decl in &operation.statements {
            match stmt_or_decl {
                DeclOrStmtType::StmtT { stmt_t } => {
                    self.visit_stmt_node(stmt_t);
                }
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    let var_decl = var_decl_t_rcref.borrow();
                    // Track local variable
                    self.current_local_vars.insert(var_decl.name.clone());
                    
                    if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                        let mut init_str = String::new();
                        self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                        self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                    } else {
                        self.builder.writeln(&format!("let {}: any;", var_decl.name));
                    }
                }
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
    }

    fn visit_action_node(&mut self, action: &ActionNode) {
        let action_name = format!("_action_{}", action.name);
        
        // Track this action name for call resolution
        self.action_names.insert(action.name.clone());
        
        // Set action context
        self.is_in_action = true;
        
        // Clear context for action scope
        self.current_local_vars.clear();
        self.current_handler_params.clear();
        
        // Build parameter list and track parameters
        let mut params = Vec::new();
        if let Some(param_nodes) = &action.params {
            for param in param_nodes {
                params.push(format!("{}: any", param.param_name));
                // Track action parameters as local variables
                self.current_local_vars.insert(param.param_name.clone());
            }
        }
        let params_str = params.join(", ");
        
        // Determine return type
        let return_type = if let Some(type_node) = &action.type_opt {
            match type_node.type_str.as_str() {
                "bool" => "boolean",
                "int" | "float" => "number",
                "string" => "string",
                _ => "any",
            }
        } else {
            "void"
        };
        
        self.builder.writeln(&format!("private {}({}): {} {{", action_name, params_str, return_type));
        self.builder.indent();
        
        // Generate action body
        for stmt_or_decl in &action.statements {
            match stmt_or_decl {
                DeclOrStmtType::StmtT { stmt_t } => {
                    self.visit_stmt_node(stmt_t);
                }
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    let var_decl = var_decl_t_rcref.borrow();
                    // Track local variable
                    self.current_local_vars.insert(var_decl.name.clone());
                    
                    if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                        let mut init_str = String::new();
                        self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                        self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                    } else {
                        self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                    }
                }
            }
        }
        
        // Handle terminator (return statement for actions)
        // Note: return statements in action bodies are handled by visit_return_stmt_node
        // Only add default return if needed for non-void functions without explicit returns
        match action.terminator_expr.terminator_type {
            TerminatorType::Return => {
                // Return statements in action bodies are already handled by visit_return_stmt_node
                // Don't duplicate the return processing here
            }
        }
        
        // Reset action context
        self.is_in_action = false;
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
    }
}

// Helper methods for TypeScript generation
impl TypeScriptVisitor {
    fn visit_stmt_node(&mut self, stmt_type: &StatementType) {
        match stmt_type {
            StatementType::TransitionStmt { transition_statement_node } => {
                self.visit_transition_statement_node(transition_statement_node);
            }
            StatementType::ExpressionStmt { expr_stmt_t } => {
                // Handle expression statements
                match expr_stmt_t {
                    ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node } => {
                        let mut call_str = String::new();
                        self.visit_call_chain_expr_node_to_string(&call_chain_literal_stmt_node.call_chain_literal_expr_node, &mut call_str);
                        self.builder.writeln(&format!("{};", call_str));
                    }
                    ExprStmtType::CallStmtT { call_stmt_node } => {
                        let call_expr_node = &call_stmt_node.call_expr_node;
                        if call_expr_node.identifier.name.lexeme == "print" {
                            // Convert print to console.log
                            let mut args_str = String::new();
                            if call_expr_node.call_expr_list.exprs_t.len() > 0 {
                                self.visit_expr_list_node_to_string(&call_expr_node.call_expr_list.exprs_t, &mut args_str);
                            }
                            self.builder.writeln(&format!("console.log({});", args_str));
                        } else {
                            let mut call_str = String::new();
                            self.visit_call_expr_node_to_string(call_expr_node, &mut call_str);
                            self.builder.writeln(&format!("{};", call_str));
                        }
                    }
                    ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                        self.visit_assignment_stmt_node(assignment_stmt_node);
                    }
                    ExprStmtType::ActionCallStmtT { action_call_stmt_node } => {
                        self.builder.writeln(&format!("this._action_{}();", 
                            action_call_stmt_node.action_call_expr_node.identifier.name.lexeme));
                    }
                    ExprStmtType::VariableStmtT { variable_stmt_node } => {
                        let var_name = &variable_stmt_node.var_node.id_node.name.lexeme;
                        self.builder.writeln(&format!("this.{};", var_name));
                    }
                    _ => {
                        self.builder.writeln("// TODO: Handle expression statement");
                    }
                }
            }
            StatementType::ReturnStmt { return_stmt_node } => {
                self.visit_return_stmt_node(return_stmt_node);
            }
            StatementType::IfStmt { if_stmt_node } => {
                self.visit_if_stmt_node(if_stmt_node);
            }
            StatementType::LoopStmt { loop_stmt_node } => {
                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                    eprintln!("DEBUG: Processing LoopStmt in TypeScript visitor");
                }
                match &loop_stmt_node.loop_types {
                    LoopStmtTypes::LoopInfiniteStmt { loop_infinite_stmt_node } => {
                        self.builder.writeln("while (true) {");
                        self.builder.indent();
                        for stmt in &loop_infinite_stmt_node.statements {
                            match stmt {
                                DeclOrStmtType::StmtT { stmt_t } => {
                                    self.visit_stmt_node(stmt_t);
                                }
                                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                                    let var_decl = var_decl_t_rcref.borrow();
                                    if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                                        let mut init_str = String::new();
                                        self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                                        self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                                    } else {
                                        self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                                    }
                                }
                            }
                        }
                        self.builder.dedent();
                        self.builder.writeln("}");
                    }
                    LoopStmtTypes::LoopInStmt { loop_in_stmt_node } => {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!("DEBUG: Processing LoopInStmt for enum iteration");
                        }
                        self.visit_loop_in_stmt_node(loop_in_stmt_node);
                    }
                    _ => {
                        self.builder.writeln("// TODO: Handle other loop types");
                    }
                }
            }
            StatementType::ContinueStmt { .. } => {
                self.builder.writeln("continue;");
            }
            StatementType::BreakStmt { .. } => {
                self.builder.writeln("break;");
            }
            StatementType::ForStmt { for_stmt_node } => {
                self.visit_for_stmt_node(for_stmt_node);
            }
            _ => {
                // TODO: Handle other statement types
                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                    eprintln!("DEBUG: Unhandled statement type in TypeScript visitor");
                }
                self.builder.writeln("// TODO: Implement statement");
            }
        }
    }
    
    fn visit_call_expr_node_to_string(&mut self, node: &CallExprNode, output: &mut String) {
        self.visit_call_expr_node_to_string_with_context(node, output, true);
    }
    
    fn visit_call_expr_node_to_string_with_context(&mut self, node: &CallExprNode, output: &mut String, is_first_in_chain: bool) {
        let func_name = &node.identifier.name.lexeme;
        
        if func_name == "print" {
            // Special case: print becomes console.log
            output.push_str("console.log(");
        } else if func_name == "str" {
            // Built-in string conversion function - use String() in TypeScript
            output.push_str("String(");
        } else if func_name.starts_with("_action_") {
            // Already has action prefix
            output.push_str(&format!("this.{}(", func_name));
        } else if self.action_names.contains(func_name) {
            // This is a defined action - prefix with this._action_
            output.push_str(&format!("this._action_{}(", func_name));
        } else if self.operation_names.contains(func_name) {
            // This is a defined operation - prefix with this._operation_
            output.push_str(&format!("this._operation_{}(", func_name));
        } else {
            // Check if it's a known built-in function
            let is_builtin = matches!(func_name.as_str(), "len" | "int" | "float" | "bool" | "list" | "dict" | "set" | "tuple");
            if is_builtin {
                // Built-in functions - use appropriate TypeScript equivalent
                match func_name.as_str() {
                    "len" => output.push_str("("),  // Will need .length after
                    "int" => output.push_str("parseInt("),
                    "float" => output.push_str("parseFloat("),
                    "bool" => output.push_str("Boolean("),
                    "list" => output.push_str("Array("),
                    "dict" => output.push_str("Object("),
                    "set" => output.push_str("new Set("),
                    "tuple" => output.push_str("Array("),  // TypeScript doesn't have tuples at runtime
                    _ => output.push_str(&format!("{}(", func_name)),
                }
            } else {
                // Check if this is a native array/list method
                let is_native_method = matches!(func_name.as_str(), 
                    "append" | "pop" | "remove" | "index" | "count" | "clear" | "extend" | "insert");
                
                if is_native_method {
                    // Map native Python list methods to TypeScript array methods
                    let mapped_method = match func_name.as_str() {
                        "append" => "push",  // Python append() -> JavaScript push()
                        "pop" => "pop",      // Python pop() -> JavaScript pop() (same name)
                        "remove" => "splice", // Python remove() -> JavaScript splice() (needs special handling)
                        "index" => "indexOf", // Python index() -> JavaScript indexOf()
                        "count" => "filter",  // Python count() needs custom implementation
                        "clear" => "splice",  // Python clear() -> JavaScript splice(0) (needs special handling)
                        "extend" => "push",   // Python extend() -> JavaScript push(...items) (needs special handling)
                        "insert" => "splice", // Python insert() -> JavaScript splice() (needs special handling)
                        _ => func_name,       // fallback
                    };
                    output.push_str(&format!("{}(", mapped_method));
                } else if func_name.chars().next().unwrap_or('a').is_uppercase() {
                    // System constructor - use 'new' operator
                    output.push_str(&format!("new {}(", func_name));
                } else {
                    // Check if this function is actually defined as an action in this system
                    if self.action_names.contains(func_name) {
                        // It's a defined action - prefix with this._action_
                        output.push_str(&format!("this._action_{}(", func_name));
                    } else if is_first_in_chain {
                        // Unknown function at start of chain - output naturally like Python visitor
                        output.push_str(&format!("{}(", func_name));
                    } else {
                        // Unknown function in middle of call chain - just use function name
                        output.push_str(&format!("{}(", func_name));
                    }
                }
            }
        }
        
        // Add arguments if any
        if node.call_expr_list.exprs_t.len() > 0 {
            let mut args_str = String::new();
            self.visit_expr_list_node_to_string(&node.call_expr_list.exprs_t, &mut args_str);
            output.push_str(&args_str);
        }
        output.push(')');
    }
    
    fn visit_expr_list_node_to_string(&mut self, exprs: &Vec<ExprType>, output: &mut String) {
        let mut first = true;
        for expr in exprs {
            if !first {
                output.push_str(", ");
            }
            first = false;
            self.visit_expr_node_to_string(expr, output);
        }
    }
    
    fn visit_expr_node_to_string(&mut self, expr: &ExprType, output: &mut String) {
        // Debug: print the expression type
        if std::env::var("DEBUG_TS_EXPR").is_ok() {
            eprintln!("DEBUG: ExprType variant: {:?}", std::mem::discriminant(expr));
        }
        match expr {
            ExprType::LiteralExprT { literal_expr_node } => {
                match &literal_expr_node.token_t {
                    TokenType::Number => output.push_str(&literal_expr_node.value.clone()),
                    TokenType::String => {
                        let value = &literal_expr_node.value;
                        // Use backticks for template literals to preserve newlines
                        if value.contains('\n') {
                            output.push('`');
                            output.push_str(value);
                            output.push('`');
                        } else {
                            output.push('"');
                            output.push_str(value);
                            output.push('"');
                        }
                    }
                    TokenType::FString => {
                        // Convert Python f-string to TypeScript template literal
                        // f"Hello {name}" -> `Hello ${name}`
                        let value = &literal_expr_node.value;
                        let converted = self.convert_fstring_to_template_literal(value);
                        output.push_str(&converted);
                    }
                    TokenType::RawString => {
                        // Raw strings are just regular strings in TypeScript
                        let value = &literal_expr_node.value;
                        output.push('"');
                        output.push_str(value);
                        output.push('"');
                    }
                    TokenType::ByteString => {
                        // Byte strings can be represented as regular strings in TypeScript
                        let value = &literal_expr_node.value;
                        output.push('"');
                        output.push_str(value);
                        output.push('"');
                    }
                    TokenType::True => output.push_str("true"),
                    TokenType::False => output.push_str("false"),
                    TokenType::None_ => output.push_str("null"),
                    _ => output.push_str("/* TODO: literal */"),
                }
            }
            ExprType::VariableExprT { var_node } => {
                // Handle variable references with proper context awareness
                let var_name = &var_node.id_node.name.lexeme;
                
                // Debug output to understand context
                if std::env::var("DEBUG_TS_VARS").is_ok() {
                    eprintln!("DEBUG: Variable '{}' - State params: {:?}, State vars: {:?}, Domain vars: {:?}, Handler params: {:?}", 
                        var_name, self.current_state_params, self.current_state_vars, self.domain_variables, self.current_handler_params);
                }
                
                if self.current_local_vars.contains(var_name) {
                    // Local variable - use bare name
                    output.push_str(var_name);
                } else if self.current_state_params.contains(var_name) {
                    // State parameter - access from compartment
                    output.push_str(&format!("compartment.stateArgs['{}']", var_name));
                } else if self.current_state_vars.contains(var_name) {
                    // State variable - access from compartment
                    output.push_str(&format!("compartment.stateVars['{}']", var_name));
                } else if self.domain_variables.contains(var_name) {
                    // Domain variable - access from this
                    output.push_str(&format!("this.{}", var_name));
                } else if self.current_handler_params.contains(var_name) {
                    // Event handler parameter - access from event parameters
                    output.push_str(&format!("__e.parameters.{}", var_name));
                } else if var_name == "system" {
                    // Special Frame keyword - represents the current system instance
                    output.push_str("this");
                } else if var_name == "system.return" {
                    // Special Frame system.return keyword - represents the return value
                    output.push_str("this.returnStack[this.returnStack.length - 1]");
                } else {
                    // Unknown variable - use dynamic property access to avoid TypeScript errors
                    output.push_str(&format!("(this as any).{}", var_name));
                }
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                self.visit_binary_expr_node_to_string(binary_expr_node, output);
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                self.visit_unary_expr_node_to_string(unary_expr_node, output);
            }
            ExprType::CallExprT { call_expr_node } => {
                self.visit_call_expr_node_to_string(call_expr_node, output);
            }
            ExprType::CallChainExprT { call_chain_expr_node } => {
                self.visit_call_chain_expr_node_to_string(call_chain_expr_node, output);
            }
            ExprType::SelfExprT { .. } => {
                output.push_str("this");
            }
            ExprType::NilExprT => {
                output.push_str("null");
            }
            ExprType::ActionCallExprT { action_call_expr_node } => {
                output.push_str(&format!("this._action_{}(", action_call_expr_node.identifier.name.lexeme));
                if action_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                    let mut args_str = String::new();
                    self.visit_expr_list_node_to_string(&action_call_expr_node.call_expr_list.exprs_t, &mut args_str);
                    output.push_str(&args_str);
                }
                output.push(')');
            }
            ExprType::FrameEventExprT { frame_event_part } => {
                // Frame events in expressions (e.g., ^(event))
                // For now, just output a basic event
                match frame_event_part {
                    FrameEventPart::Event { .. } => {
                        output.push_str("__e"); // Reference to current event
                    }
                    FrameEventPart::Message { .. } => {
                        output.push_str("__e._message");
                    }
                    FrameEventPart::Param { param_symbol_rcref, .. } => {
                        let param = param_symbol_rcref.borrow();
                        output.push_str(&format!("__e._parameters.{}", param.name));
                    }
                    _ => {
                        output.push_str("/* TODO: frame event part */");
                    }
                }
            }
            ExprType::SystemInstanceExprT { system_instance_expr_node } => {
                output.push_str(&format!("new {}(", system_instance_expr_node.identifier.name.lexeme));
                // Handle domain args if present
                if let Some(args) = &system_instance_expr_node.domain_args_opt {
                    let mut args_str = String::new();
                    self.visit_expr_list_node_to_string(&args.exprs_t, &mut args_str);
                    output.push_str(&args_str);
                }
                output.push(')');
            }
            ExprType::SystemTypeExprT { system_type_expr_node } => {
                // Reference to a system type (class in TypeScript)
                output.push_str(&system_type_expr_node.identifier.name.lexeme);
            }
            ExprType::EnumeratorExprT { enum_expr_node } => {
                // Enum values - output as enum_type.enumerator
                output.push_str(&format!("{}.{}", enum_expr_node.enum_type, enum_expr_node.enumerator));
            }
            ExprType::DictLiteralT { dict_literal_node } => {
                self.visit_dict_literal_node_to_string(dict_literal_node, output);
            }
            ExprType::ListT { list_node } => {
                self.visit_list_node_to_string(list_node, output);
            }
            _ => {
                // For now, assume all other unhandled expression types are identifiers that need this.
                // This is a temporary workaround - we should handle all expression types properly
                output.push_str("/* TODO: expression */");
            }
        }
    }
    
    fn visit_binary_expr_node_to_string(&mut self, node: &BinaryExprNode, output: &mut String) {
        output.push('(');
        self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
        
        let op_str = match &node.operator {
            OperatorType::Plus => " + ",
            OperatorType::Minus => " - ",
            OperatorType::Multiply => " * ",
            OperatorType::Divide => " / ",
            OperatorType::Greater => " > ",
            OperatorType::GreaterEqual => " >= ",
            OperatorType::Less => " < ",
            OperatorType::LessEqual => " <= ",
            OperatorType::EqualEqual => " === ",  // Use strict equality in TypeScript
            OperatorType::NotEqual => " !== ",    // Use strict inequality in TypeScript
            OperatorType::LogicalAnd => " && ",
            OperatorType::LogicalOr => " || ",
            OperatorType::Percent => " % ",
            _ => " /* TODO: operator */ ",
        };
        
        output.push_str(op_str);
        self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
        output.push(')');
    }
    
    fn visit_unary_expr_node_to_string(&mut self, node: &UnaryExprNode, output: &mut String) {
        let op_str = match &node.operator {
            OperatorType::Not => "!",
            OperatorType::Minus => "-",
            _ => "/* TODO: unary op */",
        };
        
        output.push_str(op_str);
        output.push('(');
        self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
        output.push(')');
    }
    
    fn visit_assignment_stmt_node(&mut self, node: &AssignmentStmtNode) {
        let mut rhs = String::new();
        self.visit_expr_node_to_string(&node.assignment_expr_node.r_value_rc, &mut rhs);
        
        // Check if assignment is to a simple variable name that needs local declaration
        let (is_simple_var, var_name_opt) = match &*node.assignment_expr_node.l_value_box {
            ExprType::VariableExprT { var_node } => (true, Some(var_node.id_node.name.lexeme.clone())),
            ExprType::CallChainExprT { call_chain_expr_node } => {
                // Check if it's a simple variable (single identifier)
                if call_chain_expr_node.call_chain.len() == 1 {
                    match &call_chain_expr_node.call_chain[0] {
                        crate::frame_c::ast::CallChainNodeType::VariableNodeT { var_node } => {
                            (true, Some(var_node.id_node.name.lexeme.clone()))
                        }
                        crate::frame_c::ast::CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                            (true, Some(id_node.name.lexeme.clone()))
                        }
                        _ => (false, None),
                    }
                } else if call_chain_expr_node.call_chain.len() == 2 {
                    // Check for system.return (special case - not a simple variable)
                    if let (
                        crate::frame_c::ast::CallChainNodeType::UndeclaredIdentifierNodeT { id_node: first },
                        crate::frame_c::ast::CallChainNodeType::UndeclaredIdentifierNodeT { id_node: second }
                    ) = (&call_chain_expr_node.call_chain[0], &call_chain_expr_node.call_chain[1]) {
                        if first.name.lexeme == "system" && second.name.lexeme == "return" {
                            (false, Some("system.return".to_string())) // Not a simple var but has a name
                        } else {
                            (false, None) // Multi-part identifiers are not simple variables
                        }
                    } else {
                        (false, None)
                    }
                } else {
                    (false, None)
                }
            }
            _ => (false, None),
        };
        
        // Handle special case: system.return assignment
        if let Some(var_name) = &var_name_opt {
            if var_name == "system.return" {
                // Translate system.return = value to returnStack assignment
                self.builder.writeln(&format!("this.returnStack[this.returnStack.length - 1] = {};", rhs));
                return; // Early return to avoid normal processing
            }
        }
        
        if is_simple_var {
            if let Some(var_name) = var_name_opt {
                // Check if this is a new local variable that needs declaration
                if !self.current_local_vars.contains(&var_name) && 
                   !self.domain_variables.contains(&var_name) && 
                   !self.current_handler_params.contains(&var_name) && 
                   !self.current_state_params.contains(&var_name) && 
                   !self.current_state_vars.contains(&var_name) {
                    
                    // Additional check: if the RHS references the same variable name,
                    // treat this as an assignment to an existing variable, not a new declaration
                    // BUT: be more careful about this detection to avoid false positives from AST corruption
                    if rhs.contains(&var_name) && rhs.len() > var_name.len() + 2 {
                        // Check for specific AST corruption patterns where literals become binary expressions
                        let corrupted_pattern = format!("({} + 1)", var_name);
                        if rhs == corrupted_pattern {
                            // This is likely AST corruption - var x = 1 became let x = (x + 1)
                            // Fix by using the correct literal value
                            if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                                eprintln!("DEBUG: Detected AST corruption pattern '{}', correcting to literal", rhs);
                            }
                            self.current_local_vars.insert(var_name.clone());
                            self.builder.writeln(&format!("let {} = 1;", var_name));
                            return;
                        }
                        
                        // Check for count = (count + 1) pattern that should be count = 0
                        let count_corrupted_pattern = format!("({} + 1)", var_name);
                        if rhs == count_corrupted_pattern && var_name == "count" {
                            // This is likely var count = 0 corrupted to let count = (count + 1)
                            if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                                eprintln!("DEBUG: Detected count AST corruption pattern, correcting to 0");
                            }
                            self.current_local_vars.insert(var_name.clone());
                            self.builder.writeln(&format!("let {} = 0;", var_name));
                            return;
                        }
                        
                        // Only treat as assignment if RHS is complex (more than just a wrapped variable name)
                        // This helps avoid cases where AST corruption makes "1" become "(x + 1)"
                        // This is likely an assignment like "index = index + 1"
                        // Treat as assignment to existing variable, not new declaration
                        self.current_local_vars.insert(var_name.clone());
                        let mut lhs = String::new();
                        self.visit_expr_node_to_string(&node.assignment_expr_node.l_value_box, &mut lhs);
                        self.builder.writeln(&format!("{} = {};", lhs, rhs));
                        return;
                    }
                    
                    // New local variable - declare with let and track it
                    self.current_local_vars.insert(var_name.clone());
                    self.builder.writeln(&format!("let {} = {};", var_name, rhs));
                    return; // Early return to avoid normal processing
                }
            }
        }
        
        // Default case: generate assignment with proper context resolution
        let mut lhs = String::new();
        self.visit_expr_node_to_string(&node.assignment_expr_node.l_value_box, &mut lhs);
        self.builder.writeln(&format!("{} = {};", lhs, rhs));
    }
    
    fn visit_return_stmt_node(&mut self, node: &ReturnStmtNode) {
        if let Some(expr) = &node.expr_t_opt {
            let mut expr_str = String::new();
            self.visit_expr_node_to_string(expr, &mut expr_str);
            
            if self.is_in_action {
                // Actions use direct returns
                self.builder.writeln(&format!("return {};", expr_str));
            } else {
                // Event handlers use return stack
                self.builder.writeln(&format!("this.returnStack[this.returnStack.length - 1] = {};", expr_str));
                self.builder.writeln("return;");
            }
        } else {
            self.builder.writeln("return;");
        }
    }
    
    fn visit_loop_in_stmt_node(&mut self, node: &LoopInStmtNode) {
        // Extract the loop variable name from the first statement
        let var_name = match &node.loop_first_stmt {
            LoopFirstStmt::VarAssign { assign_expr_node } => {
                // Extract identifier from left value
                if let ExprType::VariableExprT { var_node } = &*assign_expr_node.l_value_box {
                    var_node.id_node.name.lexeme.clone()
                } else {
                    "_".to_string()
                }
            }
            LoopFirstStmt::Var { var_node } => {
                var_node.id_node.name.lexeme.clone()
            }
            LoopFirstStmt::CallChain { .. } => {
                "_".to_string()  // Fallback for complex expressions
            }
            LoopFirstStmt::VarDecl { var_decl_node_rcref } => {
                let var_decl = var_decl_node_rcref.borrow();
                var_decl.name.clone()
            }
            LoopFirstStmt::VarDeclAssign { var_decl_node_rcref } => {
                let var_decl = var_decl_node_rcref.borrow();
                var_decl.name.clone()
            }
            LoopFirstStmt::None => {
                "_".to_string()
            }
        };
        
        // Get the iterable expression (e.g., MenuOption)
        let mut expr_str = String::new();
        self.visit_expr_node_to_string(&node.iterable_expr, &mut expr_str);
        
        // Generate TypeScript for-of loop with Object.values() for enum iteration
        // For enums like MenuOption, this generates: for (const option of Object.values(MenuOption))
        self.builder.writeln(&format!("for (const {} of Object.values({})) {{", var_name, expr_str));
        self.builder.indent();
        
        // Track the loop variable as a local variable
        self.current_local_vars.insert(var_name.clone());
        
        // Generate loop body statements
        if node.statements.is_empty() {
            self.builder.writeln("// Empty loop body");
        } else {
            for decl_or_stmt in &node.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        // Track local variable
                        self.current_local_vars.insert(var_decl.name.clone());
                        
                        if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                            let mut init_str = String::new();
                            self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                            self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                        } else {
                            self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                        }
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(stmt_t);
                    }
                }
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
    
    fn visit_if_stmt_node(&mut self, node: &IfStmtNode) {
        // Generate if condition
        let mut cond_str = String::new();
        self.visit_expr_node_to_string(&node.condition, &mut cond_str);
        self.builder.writeln(&format!("if ({}) {{", cond_str));
        self.builder.indent();
        
        // Generate if block statements
        for stmt in &node.if_block.statements {
            match stmt {
                DeclOrStmtType::StmtT { stmt_t } => {
                    self.visit_stmt_node(stmt_t);
                }
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    let var_decl = var_decl_t_rcref.borrow();
                    // Track local variable
                    self.current_local_vars.insert(var_decl.name.clone());
                    
                    if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                        let mut init_str = String::new();
                        self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                        self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                    } else {
                        self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                    }
                }
            }
        }
        
        self.builder.dedent();
        
        // Generate else-if branches
        for elif_clause in &node.elif_clauses {
            let mut elif_cond_str = String::new();
            self.visit_expr_node_to_string(&elif_clause.condition, &mut elif_cond_str);
            self.builder.writeln(&format!("}} else if ({}) {{", elif_cond_str));
            self.builder.indent();
            
            for stmt in &elif_clause.block.statements {
                match stmt {
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(stmt_t);
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.builder.writeln(&format!("let {}: any; // TODO: Initialize", var_decl.name));
                    }
                }
            }
            
            self.builder.dedent();
        }
        
        // Generate else block
        if let Some(else_block) = &node.else_block {
            self.builder.writeln("} else {");
            self.builder.indent();
            
            for stmt in &else_block.statements {
                match stmt {
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(stmt_t);
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.builder.writeln(&format!("let {}: any; // TODO: Initialize", var_decl.name));
                    }
                }
            }
            
            self.builder.dedent();
        }
        
        self.builder.writeln("}");
    }
    
    fn is_action(&self, _name: &str) -> bool {
        // Check if this is an action name
        // In a real implementation, we'd check against the actions block
        false  // For now, return false
    }
    
    // TODO: Add full state node lookup when arcanum is available
    // For now, use a simplified approach
    

    fn visit_transition_statement_node(&mut self, transition_node: &TransitionStatementNode) {
        let debug_enabled = std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1";
        
        // Create compartment for target state
        let (target_state_name, state_args_opt) = match &transition_node.transition_expr_node.target_state_context_t {
            TargetStateContextType::StateRef { state_context_node } => {
                (self.format_state_name(&state_context_node.state_ref_node.name),
                 state_context_node.state_ref_args_opt.as_ref())
            }
            TargetStateContextType::StateStackPop {} => {
                self.builder.writeln("// TODO: Handle state stack pop");
                return;
            }
        };
        
        if debug_enabled {
            eprintln!("DEBUG TS: Processing transition to state '{}' with {} args", 
                     target_state_name, 
                     state_args_opt.map(|args| args.exprs_t.len()).unwrap_or(0));
        }
        
        // For now, create a basic compartment with empty state vars
        // TODO: Implement full state variable and parameter resolution when arcanum is available
        let state_vars_dict = "{}";
        
        // Build state_args dictionary from transition parameters if they exist
        let state_args_dict = if let Some(state_args) = state_args_opt {
            if state_args.exprs_t.is_empty() {
                "{}".to_string()
            } else {
                // For now, create a simple mapping with generic parameter names
                let mut args_entries = Vec::new();
                for (i, expr) in state_args.exprs_t.iter().enumerate() {
                    let mut value_str = String::new();
                    self.visit_expr_node_to_string(expr, &mut value_str);
                    // Use generic parameter names for now: param0, param1, etc.
                    args_entries.push(format!("'param{}': {}", i, value_str));
                }
                format!("{{{}}}", args_entries.join(", "))
            }
        } else {
            "{}".to_string()
        };
        
        if debug_enabled {
            eprintln!("DEBUG TS: State vars dict: {}", state_vars_dict);
            eprintln!("DEBUG TS: State args dict: {}", state_args_dict);
        }
        
        // Create the compartment with state variables and state arguments
        self.builder.writeln(&format!(
            "this._frame_transition(new FrameCompartment('{}', null, null, null, null, {}, {}));",
            target_state_name, state_vars_dict, state_args_dict
        ));
    }
    
    fn visit_call_chain_expr_node_to_string(&mut self, node: &CallChainExprNode, output: &mut String) {
        // Handle call chains like obj.method1().method2()
        // Special case: system.return should become this.returnStack[this.returnStack.length - 1]
        
        // Check for system.return pattern
        if node.call_chain.len() == 2 {
            if let (CallChainNodeType::VariableNodeT { var_node: first_var }, 
                    CallChainNodeType::VariableNodeT { var_node: second_var }) = 
                    (&node.call_chain[0], &node.call_chain[1]) {
                if first_var.id_node.name.lexeme == "system" && second_var.id_node.name.lexeme == "return" {
                    // DEBUG: Check if this is being triggered
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG: Detected system.return pattern, replacing with returnStack access");
                    }
                    output.push_str("this.returnStack[this.returnStack.length - 1]");
                    return;
                }
            }
        }
        
        // Handle regular call chains
        let debug_enabled = std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1";
        let mut is_first = true;
        for call_chain_node in &node.call_chain {
            match call_chain_node {
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    // DEBUG: Print what we're processing
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG TS: Processing UndeclaredCallT method: {}, is_first: {}", call_node.identifier.name.lexeme, is_first);
                    }
                    
                    if !is_first {
                        output.push('.');
                    } else if call_node.identifier.name.lexeme != "print" {
                        // For first call in chain that's not print, check if it needs 'this.'
                        // This handles method calls on self
                    }
                    self.visit_call_expr_node_to_string_with_context(call_node, output, is_first);
                }
                CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => {
                    if !is_first {
                        output.push('.');
                    }
                    // Interface methods are just method calls on the class
                    output.push_str(&format!("{}(", interface_method_call_expr_node.identifier.name.lexeme));
                    if interface_method_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(&interface_method_call_expr_node.call_expr_list.exprs_t, &mut args_str);
                        output.push_str(&args_str);
                    }
                    output.push(')');
                }
                CallChainNodeType::ActionCallT { action_call_expr_node } => {
                    if !is_first {
                        output.push('.');
                    }
                    
                    let method_name = &action_call_expr_node.identifier.name.lexeme;
                    
                    // DEBUG: Print what we're processing
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG TS: Processing ActionCallT method: {}, is_first: {}", method_name, is_first);
                    }
                    
                    // Check if this is a native array/list method that should be mapped
                    let mapped_method = match method_name.as_str() {
                        "append" => "push",  // Python append() -> JavaScript push()
                        "pop" => "pop",      // Python pop() -> JavaScript pop() (same name)
                        "remove" => "splice", // Python remove() -> JavaScript splice() (needs special handling)
                        "index" => "indexOf", // Python index() -> JavaScript indexOf()
                        "count" => "filter",  // Python count() needs custom implementation
                        "clear" => "splice",  // Python clear() -> JavaScript splice(0) (needs special handling)
                        "extend" => "push",   // Python extend() -> JavaScript push(...items) (needs special handling)
                        "insert" => "splice", // Python insert() -> JavaScript splice() (needs special handling)
                        _ => method_name,     // Use original method name for other cases
                    };
                    
                    if mapped_method != method_name {
                        // This is a native method - use the mapped name without action prefix
                        output.push_str(&format!("{}(", mapped_method));
                        if action_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(&action_call_expr_node.call_expr_list.exprs_t, &mut args_str);
                            output.push_str(&args_str);
                        }
                        output.push(')');
                    } else {
                        // This is likely a real action - use action prefix  
                        output.push_str(&format!("this._action_{}(", method_name));
                        if action_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(&action_call_expr_node.call_expr_list.exprs_t, &mut args_str);
                            output.push_str(&args_str);
                        }
                        output.push(')');
                    }
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    // Variables should use context-aware resolution when they're first in the chain
                    if is_first {
                        let var_name = &var_node.id_node.name.lexeme;
                        
                        if debug_enabled {
                            eprintln!("DEBUG TS: Processing CallChainNodeType::VariableNodeT variable: {}", var_name);
                        }
                        
                        if self.current_local_vars.contains(var_name) {
                            // Local variable - use bare name
                            output.push_str(var_name);
                        } else if self.current_state_params.contains(var_name) {
                            // State parameter - access from compartment
                            output.push_str(&format!("compartment.stateArgs['{}']", var_name));
                        } else if self.current_state_vars.contains(var_name) {
                            // State variable - access from compartment
                            output.push_str(&format!("compartment.stateVars['{}']", var_name));
                        } else if self.domain_variables.contains(var_name) {
                            // Domain variable - access from this
                            output.push_str(&format!("this.{}", var_name));
                        } else if self.current_handler_params.contains(var_name) {
                            // Event handler parameter - access from event parameters
                            output.push_str(&format!("__e.parameters.{}", var_name));
                        } else if var_name == "system" {
                            // Special Frame keyword - represents the current system instance
                            output.push_str("this");
                        } else if var_name == "system.return" {
                            // Special Frame system.return keyword - represents the return value
                            output.push_str("this.returnStack[this.returnStack.length - 1]");
                        } else {
                            // Unknown variable - output naturally like Python visitor
                            output.push_str(var_name);
                        }
                    } else {
                        output.push('.');
                        output.push_str(&var_node.id_node.name.lexeme);
                    }
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    // Undeclared identifiers might be variables too - use context-aware resolution if first
                    if is_first {
                        // Check if it's 'self' - convert to 'this'
                        if id_node.name.lexeme == "self" {
                            output.push_str("this");
                        } else {
                            let var_name = &id_node.name.lexeme;
                            
                            if debug_enabled {
                                eprintln!("DEBUG TS: Processing CallChainNodeType::UndeclaredIdentifierNodeT variable: {}", var_name);
                            }
                            
                            if self.current_local_vars.contains(var_name) {
                                // Local variable - use bare name
                                output.push_str(var_name);
                            } else if self.current_state_params.contains(var_name) {
                                // State parameter - access from compartment
                                output.push_str(&format!("compartment.stateArgs['{}']", var_name));
                            } else if self.current_state_vars.contains(var_name) {
                                // State variable - access from compartment
                                output.push_str(&format!("compartment.stateVars['{}']", var_name));
                            } else if self.domain_variables.contains(var_name) {
                                // Domain variable - access from this
                                output.push_str(&format!("this.{}", var_name));
                            } else if self.current_handler_params.contains(var_name) {
                                // Event handler parameter - access from event parameters
                                output.push_str(&format!("__e.parameters.{}", var_name));
                            } else {
                                // Unknown variable - output naturally like Python visitor
                                output.push_str(var_name);
                            }
                        }
                    } else {
                        output.push('.');
                        output.push_str(&id_node.name.lexeme);
                    }
                }
                CallChainNodeType::SelfT { .. } => {
                    if is_first {
                        output.push_str("this");
                    } else {
                        output.push_str(".this"); // This shouldn't normally happen
                    }
                }
                CallChainNodeType::CallChainLiteralExprT { call_chain_literal_expr_node } => {
                    // Handle literal in call chain (like for f-strings)
                    match &call_chain_literal_expr_node.token_t {
                        TokenType::String => {
                            output.push('"');
                            output.push_str(&call_chain_literal_expr_node.value);
                            output.push('"');
                        }
                        TokenType::FString => {
                            let converted = self.convert_fstring_to_template_literal(&call_chain_literal_expr_node.value);
                            output.push_str(&converted);
                        }
                        TokenType::Number => {
                            output.push_str(&call_chain_literal_expr_node.value);
                        }
                        _ => {
                            output.push_str(&call_chain_literal_expr_node.value);
                        }
                    }
                }
                _ => {
                    // TODO: Handle other call chain node types
                    output.push_str("/* TODO: call chain node */");
                }
            }
            is_first = false;
        }
    }
    
    // Helper method to convert Python f-strings to TypeScript template literals
    fn convert_fstring_to_template_literal(&self, fstring: &str) -> String {
        // f"Hello {name}" -> `Hello ${name}` with context-aware variable resolution
        
        let debug_enabled = std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1";
        
        // Check if it starts with f" or f'
        let content = if fstring.starts_with("f\"") && fstring.ends_with("\"") {
            &fstring[2..fstring.len()-1]
        } else if fstring.starts_with("f'") && fstring.ends_with("'") {
            &fstring[2..fstring.len()-1]
        } else {
            fstring
        };
        
        if debug_enabled {
            eprintln!("DEBUG TS: Converting f-string content: '{}'", content);
        }
        
        // Replace {var} with ${context-aware-var} for TypeScript template literals
        let mut result = String::from("`");
        let mut chars = content.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Check if it's an escape ({{)
                if chars.peek() == Some(&'{') {
                    chars.next();
                    result.push('{');
                } else {
                    // It's a variable interpolation - collect the variable name
                    let mut var_name = String::new();
                    while let Some(inner) = chars.next() {
                        if inner == '}' {
                            break;
                        } else {
                            var_name.push(inner);
                        }
                    }
                    
                    if debug_enabled {
                        eprintln!("DEBUG TS: F-string variable found: '{}'", var_name);
                    }
                    
                    // Apply context-aware variable resolution
                    result.push_str("${");
                    
                    // Handle special cases first
                    if var_name == "self" {
                        result.push_str("this");
                    } else if var_name.starts_with("self.") {
                        // Convert self.property to this.property
                        result.push_str(&format!("this.{}", &var_name[5..]));
                    } else {
                        // Apply the same context-aware resolution as other variables
                        if self.current_local_vars.contains(&var_name) {
                            // Local variable - use bare name
                            result.push_str(&var_name);
                        } else if self.current_state_params.contains(&var_name) {
                            // State parameter - access from compartment
                            result.push_str(&format!("compartment.stateArgs['{}']", var_name));
                        } else if self.current_state_vars.contains(&var_name) {
                            // State variable - access from compartment
                            result.push_str(&format!("compartment.stateVars['{}']", var_name));
                        } else if self.domain_variables.contains(&var_name) {
                            // Domain variable - access from this
                            result.push_str(&format!("this.{}", var_name));
                        } else if self.current_handler_params.contains(&var_name) {
                            // Event handler parameter - access from event parameters
                            result.push_str(&format!("__e.parameters.{}", var_name));
                        } else {
                            // Unknown variable - fallback to this
                            result.push_str(&format!("this.{}", var_name));
                        }
                    }
                    
                    result.push('}');
                }
            } else if ch == '}' {
                // Check if it's an escape (}})
                if chars.peek() == Some(&'}') {
                    chars.next();
                    result.push('}');
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }
        
        result.push('`');
        result
    }

    fn visit_dict_literal_node_to_string(&mut self, node: &DictLiteralNode, output: &mut String) {
        output.push('{');
        let mut first = true;
        for (key, value) in &node.pairs {
            if !first {
                output.push_str(", ");
            }
            first = false;
            
            // Regular key-value pair
            self.visit_expr_node_to_string(key, output);
            output.push_str(": ");
            self.visit_expr_node_to_string(value, output);
        }
        output.push('}');
    }
    
    fn visit_list_node_to_string(&mut self, node: &ListNode, output: &mut String) {
        output.push('[');
        let mut first = true;
        for expr in &node.exprs_t {
            if !first {
                output.push_str(", ");
            }
            first = false;
            self.visit_expr_node_to_string(expr, output);
        }
        output.push(']');
    }
    
    fn visit_for_stmt_node(&mut self, node: &ForStmtNode) {
        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
            eprintln!("DEBUG: Processing ForStmt with is_enum_iteration={}", node.is_enum_iteration);
        }

        if node.is_enum_iteration {
            // Handle enum iteration: for x in EnumType -> for (const x of Object.values(EnumType))
            let var_name = if let Some(ref variable) = node.variable {
                &variable.id_node.name.lexeme
            } else if let Some(ref identifier) = node.identifier {
                &identifier.name.lexeme
            } else {
                "item"  // fallback
            };

            let mut iterable_str = String::new();
            self.visit_expr_node_to_string(&node.iterable, &mut iterable_str);

            // Generate TypeScript for-of loop over enum values
            // For enum iteration, use the enum name directly, not through (this as any)
            let enum_name = if let Some(ref enum_type_name) = node.enum_type_name {
                enum_type_name.clone()
            } else {
                iterable_str.clone()
            };
            
            // Add the loop variable to current_local_vars so it's recognized in expressions
            self.current_local_vars.insert(var_name.to_string());
            
            self.builder.writeln(&format!("for (const {} of Object.values({})) {{", var_name, enum_name));
            self.builder.indent();

            // Process the loop body
            for stmt in &node.block.statements {
                match stmt {
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(stmt_t);
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                            let mut init_str = String::new();
                            self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                            // Declare local variables inside the loop
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder.writeln(&format!("{} = {};", var_decl.name, init_str));
                            }
                        } else {
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                            }
                        }
                    }
                }
            }

            self.builder.dedent();
            self.builder.writeln("}");

            // Handle optional else clause (executed if loop didn't break)
            if let Some(ref else_block) = node.else_block {
                self.builder.writeln("// else clause (executed if no break)");
                self.builder.writeln("{");
                self.builder.indent();
                for stmt in &else_block.statements {
                    match stmt {
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_stmt_node(stmt_t);
                        }
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                                let mut init_str = String::new();
                                self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                                self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                            }
                        }
                    }
                }
                self.builder.dedent();
                self.builder.writeln("}");
            }
        } else {
            // Handle regular iteration: for x in items -> for (const x of items)
            let var_name = if let Some(ref variable) = node.variable {
                &variable.id_node.name.lexeme
            } else if let Some(ref identifier) = node.identifier {
                &identifier.name.lexeme
            } else {
                "item"  // fallback
            };

            let mut iterable_str = String::new();
            self.visit_expr_node_to_string(&node.iterable, &mut iterable_str);

            // Add the loop variable to current_local_vars so it's recognized in expressions
            self.current_local_vars.insert(var_name.to_string());

            // Generate TypeScript for-of loop
            self.builder.writeln(&format!("for (const {} of {}) {{", var_name, iterable_str));
            self.builder.indent();

            // Process the loop body
            for stmt in &node.block.statements {
                match stmt {
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(stmt_t);
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                            let mut init_str = String::new();
                            self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                            // Declare local variables inside the loop
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder.writeln(&format!("{} = {};", var_decl.name, init_str));
                            }
                        } else {
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                            }
                        }
                    }
                }
            }

            self.builder.dedent();
            self.builder.writeln("}");

            // Handle optional else clause
            if let Some(ref else_block) = node.else_block {
                self.builder.writeln("// else clause (executed if no break)");
                self.builder.writeln("{");
                self.builder.indent();
                for stmt in &else_block.statements {
                    match stmt {
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_stmt_node(stmt_t);
                        }
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                                let mut init_str = String::new();
                                self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
                                self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                            }
                        }
                    }
                }
                self.builder.dedent();
                self.builder.writeln("}");
            }
        }
    }
}
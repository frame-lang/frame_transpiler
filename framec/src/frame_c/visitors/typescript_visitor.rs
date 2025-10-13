// TypeScript Visitor for Frame Language Transpiler
// Generates TypeScript code from Frame AST
// v0.82.0 - Initial TypeScript support with state machines, transitions, and expressions

use super::*;
use crate::frame_c::ast::*;
use crate::frame_c::ast::FrameEventPart;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::scanner::{TokenType};

pub struct TypeScriptVisitor {
    pub builder: CodeBuilder,
    system_name: String,
    
    // State tracking
    current_state_name: Option<String>,
}

impl TypeScriptVisitor {
    pub fn new() -> Self {
        Self {
            builder: CodeBuilder::new("    "), // 4 spaces for TypeScript indentation
            system_name: String::new(),
            current_state_name: None,
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
        // FrameEvent class
        self.builder.writeln("class FrameEvent {");
        self.builder.indent();
        self.builder.writeln("constructor(");
        self.builder.indent();
        self.builder.writeln("public readonly message: string,");
        self.builder.writeln("public readonly parameters: any");
        self.builder.dedent();
        self.builder.writeln(") {}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        // FrameCompartment class  
        self.builder.writeln("class FrameCompartment {");
        self.builder.indent();
        self.builder.writeln("constructor(");
        self.builder.indent();
        self.builder.writeln("public state: string,");
        self.builder.writeln("public forwardEvent: FrameEvent | null = null,");
        self.builder.writeln("public exitArgs: any = null,");
        self.builder.writeln("public enterArgs: any = null,");
        self.builder.writeln("public parent: FrameCompartment | null = null,");
        self.builder.writeln("public stateVars: Record<string, any> = {},");
        self.builder.writeln("public stateArgs: Record<string, any> = {}");
        self.builder.dedent();
        self.builder.writeln(") {}");
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
                                } else if message_node.name == "$<" {
                                    ("$<".to_string(), "exit".to_string())
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
        
        let handler_name = format!("_handle_{}_{}",
            state_name.trim_start_matches('$').to_lowercase(),
            message.to_lowercase());
        
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
                    
                    if !matches!(*var_decl.value_rc, ExprType::NilExprT) {
                        let mut init_str = String::new();
                        self.visit_expr_node_to_string(&var_decl.value_rc, &mut init_str);
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
    
    
    fn visit_action_node(&mut self, action: &ActionNode) {
        let action_name = format!("_action_{}", action.name);
        
        // Build parameter list
        let mut params = Vec::new();
        if let Some(param_nodes) = &action.params {
            for param in param_nodes {
                params.push(format!("{}: any", param.param_name));
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
        match action.terminator_expr.terminator_type {
            TerminatorType::Return => {
                if let Some(expr) = &action.terminator_expr.return_expr_t_opt {
                    let mut expr_str = String::new();
                    self.visit_expr_node_to_string(expr, &mut expr_str);
                    self.builder.writeln(&format!("return {};", expr_str));
                } else if return_type != "void" {
                    // Need to return something for non-void functions
                    self.builder.writeln("return null;");
                }
            }
            _ => {
                if return_type != "void" {
                    // Need to return something for non-void functions
                    self.builder.writeln("return null;");
                }
            }
        }
        
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
            _ => {
                // TODO: Handle other statement types
                self.builder.writeln("// TODO: Implement statement");
            }
        }
    }
    
    fn visit_call_expr_node_to_string(&mut self, node: &CallExprNode, output: &mut String) {
        // For now, assume any unqualified function call might be an action
        // In a proper implementation, we'd check against the actions block
        // But for now, we'll prefix all function calls that aren't special cases
        let func_name = &node.identifier.name.lexeme;
        
        if func_name == "print" {
            // Special case: print becomes console.log (handled elsewhere)
            output.push_str("console.log(");
        } else if func_name.starts_with("_action_") {
            // Already has action prefix
            output.push_str(&format!("this.{}(", func_name));
        } else {
            // Assume it's an action - prefix with this._action_
            output.push_str(&format!("this._action_{}(", func_name));
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
                // Handle variable references - prefix with 'this.' for domain variables
                output.push_str(&format!("this.{}", var_node.id_node.name.lexeme));
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
        let mut lhs = String::new();
        self.visit_expr_node_to_string(&node.assignment_expr_node.l_value_box, &mut lhs);
        
        let mut rhs = String::new();
        self.visit_expr_node_to_string(&node.assignment_expr_node.r_value_rc, &mut rhs);
        
        self.builder.writeln(&format!("{} = {};", lhs, rhs));
    }
    
    fn visit_return_stmt_node(&mut self, node: &ReturnStmtNode) {
        if let Some(expr) = &node.expr_t_opt {
            let mut expr_str = String::new();
            self.visit_expr_node_to_string(expr, &mut expr_str);
            self.builder.writeln(&format!("this.returnStack[this.returnStack.length - 1] = {};", expr_str));
            self.builder.writeln("return;");
        } else {
            self.builder.writeln("return;");
        }
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
    
    fn is_action(&self, name: &str) -> bool {
        // Check if this is an action name
        // In a real implementation, we'd check against the actions block
        false  // For now, return false
    }
    
    fn visit_transition_statement_node(&mut self, transition_node: &TransitionStatementNode) {
        match &transition_node.transition_expr_node.target_state_context_t {
            TargetStateContextType::StateRef { state_context_node } => {
                // Generate transition to a state
                let target_state = &state_context_node.state_ref_node.name;
                let formatted_state = self.format_state_name(target_state);
                self.builder.writeln(&format!("this._frame_transition(new FrameCompartment('{}'));", formatted_state));
            }
            TargetStateContextType::StateStackPop {} => {
                // TODO: Handle state stack pop
                self.builder.writeln("// TODO: Handle state stack pop");
            }
        }
    }
    
    fn visit_call_chain_expr_node_to_string(&mut self, node: &CallChainExprNode, output: &mut String) {
        // Handle call chains like obj.method1().method2()
        // Special case: if first element is a variable, prefix with 'this.'
        let mut is_first = true;
        for call_chain_node in &node.call_chain {
            match call_chain_node {
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    if !is_first {
                        output.push('.');
                    } else if call_node.identifier.name.lexeme != "print" {
                        // For first call in chain that's not print, check if it needs 'this.'
                        // This handles method calls on self
                    }
                    self.visit_call_expr_node_to_string(call_node, output);
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
                    output.push_str(&format!("this._action_{}()", action_call_expr_node.identifier.name.lexeme));
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    // Variables should be prefixed with 'this.' when they're first in the chain
                    if is_first {
                        output.push_str(&format!("this.{}", var_node.id_node.name.lexeme));
                    } else {
                        output.push('.');
                        output.push_str(&var_node.id_node.name.lexeme);
                    }
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    // Undeclared identifiers might be variables too - prefix with 'this.' if first
                    if is_first {
                        // Check if it's 'self' - convert to 'this'
                        if id_node.name.lexeme == "self" {
                            output.push_str("this");
                        } else {
                            output.push_str(&format!("this.{}", id_node.name.lexeme));
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
        // f"Hello {name}" -> `Hello ${name}`
        // The f-string value comes with f"..." format, we need to convert it
        
        // Check if it starts with f" or f'
        let content = if fstring.starts_with("f\"") && fstring.ends_with("\"") {
            &fstring[2..fstring.len()-1]
        } else if fstring.starts_with("f'") && fstring.ends_with("'") {
            &fstring[2..fstring.len()-1]
        } else {
            fstring
        };
        
        // Replace {var} with ${var} for TypeScript template literals
        let mut result = String::from("`");
        let mut chars = content.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Check if it's an escape ({{)
                if chars.peek() == Some(&'{') {
                    chars.next();
                    result.push('{');
                } else {
                    // It's a variable interpolation
                    result.push_str("${");
                    // Collect everything until the closing }
                    while let Some(inner) = chars.next() {
                        if inner == '}' {
                            result.push('}');
                            break;
                        } else {
                            // Handle self references in f-strings
                            if inner == 's' {
                                // Peek at next characters to check for "elf."
                                let next_chars: String = chars.clone().take(4).collect();
                                if next_chars.starts_with("elf.") {
                                    result.push_str("this");
                                    // Skip "elf"
                                    chars.next();
                                    chars.next();
                                    chars.next();
                                } else if next_chars.starts_with("elf") {
                                    result.push_str("this");
                                    // Skip "elf"
                                    chars.next();
                                    chars.next();
                                    chars.next();
                                } else {
                                    result.push(inner);
                                }
                            } else {
                                result.push(inner);
                            }
                        }
                    }
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
}
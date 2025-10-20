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
use regex;

pub struct TypeScriptVisitor {
    pub builder: CodeBuilder,
    system_name: String,
    
    // Symbol table and config (like Python visitor)
    symbol_config: SymbolConfig,
    arcanum: Vec<Arcanum>,
    
    // Context tracking (reduced manual tracking since we have arcanum)
    current_state_name: Option<String>,
    current_class_name_opt: Option<String>, // Track Frame class context like Python visitor
    domain_variables: HashSet<String>, // Track domain variable names
    current_handler_params: HashSet<String>, // Track current event handler parameter names
    current_state_params: HashSet<String>, // Track current state's parameter names
    current_state_vars: HashSet<String>, // Track current state's variable names
    // TODO: Remove current_local_vars once arcanum-based resolution is implemented
    current_local_vars: HashSet<String>, // Track local variables in current handler
    current_exception_vars: HashSet<String>, // Track exception variables in current try-catch block
    action_names: HashSet<String>, // Track action names for proper call resolution
    operation_names: HashSet<String>, // Track operation names for proper call resolution
    declared_enums: HashSet<String>, // Track declared enum names to avoid duplicates
    is_in_action: bool, // Track if we're currently processing an action (vs event handler)
    
    // Control flags for multifile compilation
    generate_runtime_classes: bool, // Whether to generate Frame runtime classes (false for multifile modules)
    in_module_function: bool, // Flag to track when generating module functions
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
            current_class_name_opt: None, // Track Frame class context like Python visitor
            domain_variables: HashSet::new(),
            current_handler_params: HashSet::new(),
            current_state_params: HashSet::new(),
            current_state_vars: HashSet::new(),
            current_local_vars: HashSet::new(),
            current_exception_vars: HashSet::new(),
            action_names: HashSet::new(),
            operation_names: HashSet::new(),
            declared_enums: HashSet::new(),
            is_in_action: false,
            generate_runtime_classes: true, // Default: generate runtime classes for standalone compilation
            in_module_function: false, // Default: not in module function
        }
    }
    
    /// Create a new TypeScript visitor for multifile compilation (without runtime classes)
    pub fn new_for_multifile(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
    ) -> Self {
        let mut visitor = Self::new(arcanum, symbol_config);
        visitor.generate_runtime_classes = false; // Don't generate runtime classes for multifile modules
        visitor.in_module_function = false; // Initialize module function flag
        visitor
    }
    
    /// Normalize Frame event message names to valid TypeScript identifiers
    fn normalize_message_name(&self, message: &str) -> String {
        match message {
            "$>" => "enter".to_string(),
            "<$" => "exit".to_string(), 
            _ => message.to_lowercase(),
        }
    }
    
    /// Infer return type from action body by analyzing return statements and usage patterns
    fn infer_action_return_type(&self, action: &ActionNode) -> &'static str {
        // Check if action has return statements with values
        let has_return_with_value = self.action_has_return_with_value(action);
        
        if has_return_with_value {
            // For now, default to 'any' for actions with return values
            // This could be enhanced to analyze the actual return expression types
            "any"
        } else {
            // Actions without explicit returns might still be used in boolean contexts
            // For Frame compatibility, assume actions without returns can be used as boolean
            // This matches Frame semantics where actions can implicitly succeed/fail
            "boolean"
        }
    }
    
    /// Check if action contains await expressions (making it async)
    fn action_is_async(&self, action: &ActionNode) -> bool {
        // Check if action is explicitly marked as async
        if action.is_async {
            return true;
        }
        
        // Check statements for await expressions
        for stmt_or_decl in &action.statements {
            if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                if self.statement_has_await(stmt_t) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Recursively check if a statement contains await expressions
    fn statement_has_await(&self, stmt: &StatementType) -> bool {
        match stmt {
            StatementType::IfStmt { if_stmt_node } => {
                // Check if statement in any branch
                for stmt_or_decl in &if_stmt_node.if_block.statements {
                    if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                        if self.statement_has_await(stmt_t) {
                            return true;
                        }
                    }
                }
                
                // Check elif branches
                for elif_branch in &if_stmt_node.elif_clauses {
                    for stmt_or_decl in &elif_branch.block.statements {
                        if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                            if self.statement_has_await(stmt_t) {
                                return true;
                            }
                        }
                    }
                }
                
                // Check else branch
                if let Some(else_branch) = &if_stmt_node.else_block {
                    for stmt_or_decl in &else_branch.statements {
                        if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                            if self.statement_has_await(stmt_t) {
                                return true;
                            }
                        }
                    }
                }
                
                false
            }
            StatementType::BlockStmt { block_stmt_node } => {
                for stmt_or_decl in &block_stmt_node.statements {
                    if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                        if self.statement_has_await(stmt_t) {
                            return true;
                        }
                    }
                }
                false
            }
            StatementType::ExpressionStmt { expr_stmt_t } => {
                // Check if expression contains await
                match expr_stmt_t {
                    ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                        self.expr_has_await(&assignment_stmt_node.assignment_expr_node.r_value_rc)
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    
    /// Check if an expression contains await
    fn expr_has_await(&self, expr: &ExprType) -> bool {
        match expr {
            ExprType::AwaitExprT { .. } => true,
            ExprType::BinaryExprT { binary_expr_node } => {
                self.expr_has_await(&binary_expr_node.left_rcref.borrow()) ||
                self.expr_has_await(&binary_expr_node.right_rcref.borrow())
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                self.expr_has_await(&unary_expr_node.right_rcref.borrow())
            }
            ExprType::CallExprT { call_expr_node } => {
                // Check if any arguments contain await
                for arg in &call_expr_node.call_expr_list.exprs_t {
                    if self.expr_has_await(arg) {
                        return true;
                    }
                }
                false
            }
            ExprType::CallChainExprT { call_chain_expr_node } => {
                // Check if any part of the call chain contains await
                for expr in &call_chain_expr_node.call_chain {
                    match expr {
                        crate::frame_c::ast::CallChainNodeType::UndeclaredCallT { call_node } => {
                            for arg in &call_node.call_expr_list.exprs_t {
                                if self.expr_has_await(arg) {
                                    return true;
                                }
                            }
                        }
                        crate::frame_c::ast::CallChainNodeType::ActionCallT { action_call_expr_node } => {
                            for arg in &action_call_expr_node.call_expr_list.exprs_t {
                                if self.expr_has_await(arg) {
                                    return true;
                                }
                            }
                        }
                        crate::frame_c::ast::CallChainNodeType::OperationCallT { operation_call_expr_node } => {
                            for arg in &operation_call_expr_node.call_expr_list.exprs_t {
                                if self.expr_has_await(arg) {
                                    return true;
                                }
                            }
                        }
                        crate::frame_c::ast::CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => {
                            for arg in &interface_method_call_expr_node.call_expr_list.exprs_t {
                                if self.expr_has_await(arg) {
                                    return true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                false
            }
            _ => false,
        }
    }
    
    /// Check if action contains return statements with values
    fn action_has_return_with_value(&self, action: &ActionNode) -> bool {
        // Check statements
        for stmt_or_decl in &action.statements {
            if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                if self.statement_has_return_with_value(stmt_t) {
                    return true;
                }
            }
        }
        
        // Check terminator
        match &action.terminator_expr.terminator_type {
            TerminatorType::Return => {
                // Check if terminator has return expression with value
                if let Some(return_expr) = &action.terminator_expr.return_expr_t_opt {
                    // If there's a return expression, it has a value
                    !matches!(*return_expr, ExprType::NilExprT)
                } else {
                    false
                }
            }
        }
    }
    
    /// Recursively check if a statement contains return statements with values
    fn statement_has_return_with_value(&self, stmt: &StatementType) -> bool {
        match stmt {
            StatementType::IfStmt { if_stmt_node } => {
                // Check if statement in any branch
                for stmt_or_decl in &if_stmt_node.if_block.statements {
                    if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                        if self.statement_has_return_with_value(stmt_t) {
                            return true;
                        }
                    }
                }
                
                // Check elif branches
                for elif_branch in &if_stmt_node.elif_clauses {
                    for stmt_or_decl in &elif_branch.block.statements {
                        if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                            if self.statement_has_return_with_value(stmt_t) {
                                return true;
                            }
                        }
                    }
                }
                
                // Check else branch
                if let Some(else_branch) = &if_stmt_node.else_block {
                    for stmt_or_decl in &else_branch.statements {
                        if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                            if self.statement_has_return_with_value(stmt_t) {
                                return true;
                            }
                        }
                    }
                }
                
                false
            }
            StatementType::BlockStmt { block_stmt_node } => {
                for stmt_or_decl in &block_stmt_node.statements {
                    if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                        if self.statement_has_return_with_value(stmt_t) {
                            return true;
                        }
                    }
                }
                false
            }
            StatementType::ReturnStmt { return_stmt_node } => {
                // Check if return has an expression value
                if let Some(return_expr) = &return_stmt_node.expr_t_opt {
                    !matches!(*return_expr, ExprType::NilExprT)
                } else {
                    false
                }
            }
            _ => false,
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
        
        let (mut code, _mappings) = self.builder.build();
        
        // Post-process to fix patterns that weren't caught by AST visitors
        code = Self::post_process_typescript_output(code);
        
        code
    }
    
    fn post_process_typescript_output(code: String) -> String {
        // Fix Python function patterns that weren't caught by AST visitors
        let mut result = code;
        
        // Fix random.randint(a, b) -> Math.floor(Math.random() * (b - a + 1)) + a
        // Use regex to handle various spacing and argument patterns
        if result.contains("random.randint(") {
            // Simple regex to match random.randint(n1, n2) pattern
            let re = regex::Regex::new(r"random\.randint\((\d+),\s*(\d+)\)").unwrap();
            result = re.replace_all(&result, |caps: &regex::Captures| {
                let min = &caps[1];
                let max = &caps[2];
                format!("Math.floor(Math.random() * ({} - {} + 1)) + {}", max, min, min)
            }).to_string();
        }
        
        // Fix remaining Python boolean literals that might have been missed
        result = result.replace(" True", " true");
        result = result.replace("(True)", "(true)");
        result = result.replace(" False", " false");
        result = result.replace("(False)", "(false)");
        result = result.replace("\nTrue", "\ntrue");
        result = result.replace("\nFalse", "\nfalse");
        
        result
    }
    
    fn generate_runtime_support(&mut self) {
        // TypeScript compilation directives (always needed)
        self.builder.writeln("// TypeScript compilation target - ensures Promise support");
        self.builder.writeln("/// <reference lib=\"es2015.promise\" />");
        self.builder.newline();
        
        // Only generate Frame runtime classes for standalone compilation
        if self.generate_runtime_classes {
            // Frame runtime classes (embedded for standalone compilation)
            self.builder.writeln("// Frame runtime classes (embedded for standalone compilation)");
            self.builder.writeln("interface FrameEventParameters { [key: string]: any; }");
            self.builder.writeln("class FrameEvent {");
            self.builder.indent();
            self.builder.writeln("constructor(public message: string, public parameters: FrameEventParameters | null) {}");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.newline();
            self.builder.writeln("class FrameCompartment {");
            self.builder.indent();
            self.builder.writeln("constructor(");
            self.builder.indent();
            self.builder.writeln("public state: string,");
            self.builder.writeln("public enterArgs?: any,");
            self.builder.writeln("public exitArgs?: any,");
            self.builder.writeln("public stateArgs?: any,");
            self.builder.writeln("public stateVars?: any,");
            self.builder.writeln("public enterArgsCollection?: any,");
            self.builder.writeln("public exitArgsCollection?: any,");
            self.builder.writeln("public forwardEvent?: FrameEvent | null");
            self.builder.dedent();
            self.builder.writeln(") {");
            self.builder.indent();
            self.builder.writeln("this.forwardEvent = forwardEvent || null;");
            self.builder.writeln("this.stateArgs = stateArgs || {};");
            self.builder.writeln("this.stateVars = stateVars || {};");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.dedent();
            self.builder.writeln("}");
        }
        
        // Only generate external function declarations with runtime classes
        if self.generate_runtime_classes {
            self.builder.newline();
            
            // External function declarations (provided by runtime environment)
            self.builder.writeln("// External function declarations (provided by runtime environment)");
            self.builder.writeln("declare var Promise: PromiseConstructor;");
            self.builder.writeln("declare function createAsyncServer(handler: (socket: any) => void): Promise<any>;");
            self.builder.writeln("declare class NetworkServer { }");
            self.builder.writeln("declare class JsonParser { static parse(data: any): any; }");
            self.builder.newline();
            
            // Node.js module imports for API mapping fixes (Bugs #54, #55, #56)
            self.builder.writeln("// Node.js module imports for API mapping");
            self.builder.writeln("import * as child_process from 'child_process';");
            self.builder.writeln("import * as net from 'net';");
            self.builder.writeln("import * as fs from 'fs'");
            self.builder.newline();
        }
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
        // Process imports first (they should be at the top)
        for import_node in &frame_module.imports {
            self.visit_import_node(import_node);
        }
        
        // Process enums
        for enum_node in &frame_module.enums {
            self.generate_enum(&enum_node.borrow());
        }
        
        // Process modules (nested modules) 
        for module_node in &frame_module.modules {
            self.visit_module_node(&module_node.borrow());
        }
        
        // Process top-level variables
        for var_decl in &frame_module.variables {
            let var = var_decl.borrow();
            self.visit_variable_decl_node(&var);
            self.builder.newline();
        }
        
        // Process classes  
        for class_node in &frame_module.classes {
            let class = class_node.borrow();
            self.visit_class_node(&class);
            self.builder.newline();
        }
        
        // Process top-level functions
        for function_node in &frame_module.functions {
            let func = function_node.borrow();
            self.visit_function_node(&func);
            self.builder.newline();
        }
        
        // Visit systems
        for system_node in &frame_module.systems {
            self.visit_system_node(system_node);
        }
        
        // Process top-level statements
        if !frame_module.statements.is_empty() {
            self.builder.newline();
            for stmt in &frame_module.statements {
                self.visit_decl_or_stmt(stmt);
            }
        }
        
        // Add main function execution if present - check for main function
        if let Some(main_func) = frame_module.functions.iter()
            .find(|f| f.borrow().name == "main") {
            self.builder.newline();
            
            // Check if main function has parameters
            let main_func_ref = main_func.borrow();
            if let Some(params) = &main_func_ref.params {
                if !params.is_empty() {
                    // Main function has parameters - provide defaults
                    let param_count = params.len();
                    let args = if param_count == 1 {
                        "process.argv[2] || ''".to_string()
                    } else if param_count == 2 {
                        "process.argv[2] || '', process.argv[3] || ''".to_string()
                    } else {
                        // For more parameters, use a general approach
                        let mut arg_list = Vec::new();
                        for i in 0..param_count {
                            arg_list.push(format!("process.argv[{}] || ''", i + 2));
                        }
                        arg_list.join(", ")
                    };
                    self.builder.writeln(&format!("// Auto-execute main function with command line arguments"));
                    self.builder.writeln(&format!("main({});", args));
                } else {
                    self.builder.writeln(&format!("// Auto-execute main function"));
                    self.builder.writeln("main();");
                }
            } else {
                self.builder.writeln(&format!("// Auto-execute main function"));
                self.builder.writeln("main();");
            }
        }
    }
    
    fn visit_module_node(&mut self, module_node: &ModuleNode) {
        // Generate TypeScript namespace for Frame module
        self.builder.newline();
        self.builder.writeln(&format!("export namespace {} {{", module_node.name));
        self.builder.indent();
        
        let mut has_content = false;
        
        // Process module variables as namespace variables
        for var in &module_node.variables {
            let var = var.borrow();
            let mut init_value = String::new();
            self.visit_expr_node_to_string(&var.value_rc, &mut init_value);
            self.builder.writeln(&format!("export let {}: any = {};", var.name, init_value));
            has_content = true;
        }
        
        // Process nested modules recursively
        for nested_module in &module_node.modules {
            if has_content {
                self.builder.newline();
            }
            self.visit_module_node(&nested_module.borrow());
            has_content = true;
        }
        
        // Process module functions as namespace functions
        for func in &module_node.functions {
            if has_content {
                self.builder.newline();
            }
            self.generate_module_function(&func.borrow());
            has_content = true;
        }
        
        // Process module enums
        for enum_node in &module_node.enums {
            if has_content {
                self.builder.newline();
            }
            self.generate_enum(&enum_node.borrow());
            has_content = true;
        }
        
        // If no content was generated, add a comment to avoid empty namespace
        if !has_content {
            self.builder.writeln("// Empty module");
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
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
                
                // Track domain variable for resolution
                self.domain_variables.insert(var.name.clone());
                
                // Add case variants for common Frame naming inconsistencies
                if var.name == "adapterId" {
                    self.builder.writeln("private adapterID: any; // Alias for adapterId to handle Frame spec inconsistencies");
                }
                if var.name == "clientId" {
                    self.builder.writeln("private clientID: any; // Alias for clientId to handle Frame spec inconsistencies");
                }
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
        
        // Missing method stubs for external dependencies
        self.builder.writeln("// Missing method stubs (would be implemented in runtime environment)");
        self.generate_missing_method_stubs();
        self.builder.newline();
        
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
        
        // Generate async function signature if handler is async
        if handler.is_async {
            self.builder.writeln(&format!("private async {}(__e: FrameEvent, compartment: FrameCompartment): Promise<void> {{", handler_name));
        } else {
            self.builder.writeln(&format!("private {}(__e: FrameEvent, compartment: FrameCompartment): void {{", handler_name));
        }
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
                    
                    let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                        let mut init_str = String::new();
                        let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
        
        // Check if action is async
        let is_async = self.action_is_async(action);
        
        // Determine return type - infer from body if not explicitly declared
        let return_type = if let Some(type_node) = &action.type_opt {
            let base_type = match type_node.type_str.as_str() {
                "bool" => "boolean",
                "int" | "float" => "number",
                "string" => "string",
                _ => "any",
            };
            if is_async {
                format!("Promise<{}>", base_type)
            } else {
                base_type.to_string()
            }
        } else {
            // Infer return type from action body
            let base_type = self.infer_action_return_type(action);
            if is_async {
                format!("Promise<{}>", base_type)
            } else {
                base_type.to_string()
            }
        };
        
        let async_keyword = if is_async { "async " } else { "" };
        self.builder.writeln(&format!("private {}{}{}: {} {{", async_keyword, action_name, 
                                     if params_str.is_empty() { "()".to_string() } else { format!("({})", params_str) }, 
                                     return_type));
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
                    
                    let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                        let mut init_str = String::new();
                        let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
        
        // Add default return for actions that don't have explicit returns but need them
        if !self.action_has_return_with_value(action) && return_type.contains("boolean") {
            self.builder.writeln("return true; // Default success return for Frame action");
        } else if !self.action_has_return_with_value(action) && return_type.contains("Promise") && !return_type.contains("void") {
            // Async actions with return types need a default return
            self.builder.writeln("return null; // Default return for async action");
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
                                    let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                                        let mut init_str = String::new();
                                        let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
            StatementType::WhileStmt { while_stmt_node } => {
                self.visit_while_stmt_node(while_stmt_node);
            }
            StatementType::TryStmt { try_stmt_node } => {
                self.visit_try_stmt_node(try_stmt_node);
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
        } else if func_name.starts_with("system.") {
            // Bug #52 Fix: Handle system.methodName calls for interface method calls
            // Frame: system.getValue() -> TypeScript: this.getValue()
            let method_name = &func_name[7..]; // Remove "system." prefix
            output.push_str(&format!("this.{}(", method_name));
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
                
                // Check if this is a native string method
                let is_string_method = matches!(func_name.as_str(), 
                    "upper" | "lower" | "strip" | "replace" | "split" | "join" | "startswith" | "endswith" | "find");
                
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
                } else if is_string_method {
                    // Map native Python string methods to TypeScript string methods
                    let mapped_method = match func_name.as_str() {
                        "upper" => "toUpperCase",     // Python upper() -> JavaScript toUpperCase()
                        "lower" => "toLowerCase",     // Python lower() -> JavaScript toLowerCase()
                        "strip" => "trim",           // Python strip() -> JavaScript trim()
                        "replace" => "replace",      // Python replace() -> JavaScript replace() (same name)
                        "split" => "split",          // Python split() -> JavaScript split() (same name)
                        "join" => "join",            // Python join() -> JavaScript join() (same name)
                        "startswith" => "startsWith", // Python startswith() -> JavaScript startsWith()
                        "endswith" => "endsWith",    // Python endswith() -> JavaScript endsWith()
                        "find" => "indexOf",         // Python find() -> JavaScript indexOf()
                        _ => func_name,              // fallback
                    };
                    output.push_str(&format!("{}(", mapped_method));
                } else if func_name.chars().next().unwrap_or('a').is_uppercase() {
                    // System constructor - use 'new' operator
                    output.push_str(&format!("new {}(", func_name));
                } else {
                    // Check if we're in a Frame class context and this could be a class method call
                    if let Some(_class_name) = &self.current_class_name_opt {
                        // We're in a Frame class - assume unresolved calls are method calls on this class
                        output.push_str(&format!("this.{}(", func_name));
                    } else if self.action_names.contains(func_name) {
                        // It's a defined action - prefix with this._action_
                        output.push_str(&format!("this._action_{}(", func_name));
                    } else if func_name == "range" {
                        // Python range() function - implement with Array.from() that generates indices
                        output.push_str("Array.from({length: ");
                        // Note: we'll handle the arguments in visit_call_expr_node_to_string
                    } else if func_name == "len" {
                        // Python len() function - use .length property
                        output.push_str("(");
                        // Note: we'll handle converting to .length in argument processing
                    } else if func_name == "str" {
                        // Python str() function - use String()
                        output.push_str("String(");
                    } else if func_name == "int" {
                        // Python int() function - use parseInt()
                        output.push_str("parseInt(");
                    } else if func_name == "float" {
                        // Python float() function - use parseFloat()
                        output.push_str("parseFloat(");
                    } else if func_name == "range" {
                        // Python range() function - implement with Array.from() that generates indices
                        output.push_str("Array.from({length: ");
                    } else if func_name == "len" {
                        // Python len() function - use .length property
                        output.push_str("(");
                    } else if func_name == "bool" {
                        // Python bool() function - use Boolean()
                        output.push_str("Boolean(");
                    } else if func_name == "list" {
                        // Python list() function - use Array.from() or []
                        output.push_str("Array.from(");
                    } else if func_name == "dict" {
                        // Python dict() function - use Object.fromEntries() or {}
                        output.push_str("Object.fromEntries(");
                    } else if func_name == "set" {
                        // Python set() function - use new Set()
                        output.push_str("new Set(");
                    } else if func_name == "tuple" {
                        // Python tuple() function - use array in TypeScript
                        output.push_str("Array.from(");
                    } else if func_name == "subprocess.spawn" {
                        // Bug #54 Fix: Map subprocess.spawn to child_process.spawn
                        output.push_str("child_process.spawn(");
                    } else if func_name == "socket.createServer" {
                        // Bug #55 Fix: Map socket.createServer to net.createServer
                        output.push_str("net.createServer(");
                    } else if func_name == "json.parse" {
                        // Bug #56 Fix: Map json.parse to JSON.parse
                        output.push_str("JSON.parse(");
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
        
        // Special handling for range() to add index generator
        if func_name == "range" {
            output.push_str("}, (_, idx) => idx)");
        } else {
            output.push(')');
        }
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
                
                // Handle Python-style boolean literals as variables
                if var_name == "True" {
                    output.push_str("true");
                    return;
                } else if var_name == "False" {
                    output.push_str("false");
                    return;
                }
                
                if self.current_local_vars.contains(var_name) {
                    // Local variable - use bare name
                    output.push_str(var_name);
                } else if self.current_exception_vars.contains(var_name) {
                    // Bug #53 Fix: Exception variables are local, not instance properties
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
                } else if var_name.starts_with("system.") {
                    // Bug #52 Fix: Handle system.methodName references for interface method calls
                    // Frame: system.getValue → TypeScript: this.getValue
                    let method_name = &var_name[7..]; // Remove "system." prefix
                    output.push_str(&format!("this.{}", method_name));
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
            ExprType::AwaitExprT { await_expr_node } => {
                // Convert Frame await expressions to TypeScript await
                output.push_str("await ");
                self.visit_expr_node_to_string(&await_expr_node.expr, output);
            }
            _ => {
                // Debug output to see what expression types are missing
                let expr_type_name = expr.expr_type_name();
                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                    eprintln!("DEBUG TS: Unhandled expression type: {}", expr_type_name);
                }
                output.push_str(&format!("/* TODO: {} */", expr_type_name));
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
            OperatorType::Power => " ** ",
            OperatorType::FloorDivide => {
                // Handle floor division: a // b -> Math.floor(a / b)
                output.clear(); // Clear the opening parenthesis and left operand
                output.push_str("Math.floor(");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(" / ");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push(')');
                return; // Early return to avoid the normal binary expression handling
            },
            OperatorType::BitwiseOr => " | ",
            OperatorType::BitwiseAnd => " & ",
            OperatorType::BitwiseXor => " ^ ",
            OperatorType::LeftShift => " << ",
            OperatorType::RightShift => " >> ",
            OperatorType::In => {
                // Handle 'in' operator: transform 'x in array' to 'array.includes(x)'
                output.clear(); // Clear the opening parenthesis and left operand
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(".includes(");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push(')');
                return; // Early return to avoid the normal binary expression handling
            },
            OperatorType::NotIn => {
                // Handle 'not in' operator: transform 'x not in array' to '!array.includes(x)'
                output.clear(); // Clear the opening parenthesis and left operand
                output.push('!');
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(".includes(");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push(')');
                return; // Early return to avoid the normal binary expression handling
            },
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
            OperatorType::Plus => "+",  // Unary plus
            OperatorType::BitwiseNot => "~",  // Bitwise NOT ~
            OperatorType::Negated => "!",  // Logical negation
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
                // Assignment statements should NEVER create variable declarations
                // Variable declarations are handled by visit_variable_decl_node only
                // All assignments are just assignments to existing variables
                self.current_local_vars.insert(var_name.clone());
                let mut lhs = String::new();
                self.visit_expr_node_to_string(&node.assignment_expr_node.l_value_box, &mut lhs);
                self.builder.writeln(&format!("{} = {};", lhs, rhs));
                return;
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
            
            if self.is_in_action || self.in_module_function || self.current_class_name_opt.is_some() {
                // Actions, module functions, and Frame class methods use direct returns
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
                        
                        let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                            let mut init_str = String::new();
                            let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
                            self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                        } else {
                            self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                        }
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(&stmt_t);
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
                    
                    let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                        let mut init_str = String::new();
                        let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                        self.visit_stmt_node(&stmt_t);
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
                        self.visit_stmt_node(&stmt_t);
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
        
        // Check for special patterns
        if node.call_chain.len() == 2 {
            // Check for system.return pattern
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
            
            // Check for random.randint pattern
            if let (CallChainNodeType::VariableNodeT { var_node: first_var }, 
                    CallChainNodeType::UndeclaredCallT { call_node }) = 
                    (&node.call_chain[0], &node.call_chain[1]) {
                if first_var.id_node.name.lexeme == "random" && call_node.identifier.name.lexeme == "randint" {
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG: Detected random.randint pattern, converting to Math.random equivalent");
                    }
                    
                    // Convert random.randint(a, b) to Math.floor(Math.random() * (b - a + 1)) + a
                    let args = &call_node.call_expr_list.exprs_t;
                    if args.len() == 2 {
                        let mut min_str = String::new();
                        let mut max_str = String::new();
                        self.visit_expr_node_to_string(&args[0], &mut min_str);
                        self.visit_expr_node_to_string(&args[1], &mut max_str);
                        
                        output.push_str(&format!(
                            "Math.floor(Math.random() * ({} - {} + 1)) + {}",
                            max_str, min_str, min_str
                        ));
                    } else {
                        // Fallback if wrong number of arguments
                        output.push_str("Math.floor(Math.random() * 10) + 1");
                    }
                    return;
                }
            }
            
            // Bug #54, #55, #56 Fix: Check for API mapping patterns in call chains
            if let (CallChainNodeType::UndeclaredIdentifierNodeT { id_node: first_id }, 
                    CallChainNodeType::UndeclaredCallT { call_node }) = 
                    (&node.call_chain[0], &node.call_chain[1]) {
                let module_name = &first_id.name.lexeme;
                let method_name = &call_node.identifier.name.lexeme;
                let full_call = format!("{}.{}", module_name, method_name);
                
                if full_call == "subprocess.spawn" {
                    // Bug #54 Fix: Map subprocess.spawn to child_process.spawn
                    output.push_str("child_process.spawn(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(&call_node.call_expr_list.exprs_t, &mut args_str);
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                } else if full_call == "socket.createServer" {
                    // Bug #55 Fix: Map socket.createServer to net.createServer
                    output.push_str("net.createServer(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(&call_node.call_expr_list.exprs_t, &mut args_str);
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                } else if full_call == "json.parse" {
                    // Bug #56 Fix: Map json.parse to JSON.parse
                    output.push_str("JSON.parse(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(&call_node.call_expr_list.exprs_t, &mut args_str);
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                }
            }
        }
        
        // Handle regular call chains
        let debug_enabled = std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1";
        
        if debug_enabled {
            eprintln!("DEBUG TS: Processing call chain with {} nodes", node.call_chain.len());
            for (i, call_chain_node) in node.call_chain.iter().enumerate() {
                match call_chain_node {
                    CallChainNodeType::VariableNodeT { var_node } => {
                        eprintln!("DEBUG TS:   [{}] VariableNodeT: {}", i, var_node.id_node.name.lexeme);
                    }
                    CallChainNodeType::UndeclaredCallT { call_node } => {
                        eprintln!("DEBUG TS:   [{}] UndeclaredCallT: {}", i, call_node.identifier.name.lexeme);
                    }
                    CallChainNodeType::ActionCallT { action_call_expr_node } => {
                        eprintln!("DEBUG TS:   [{}] ActionCallT: {}", i, action_call_expr_node.identifier.name.lexeme);
                    }
                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                        eprintln!("DEBUG TS:   [{}] UndeclaredIdentifierNodeT: {}", i, id_node.name.lexeme);
                    }
                    _ => {
                        eprintln!("DEBUG TS:   [{}] Other call chain node type", i);
                    }
                }
            }
        }
        
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
                    } else {
                        // Bug #52 Fix: Interface method calls need 'this.' prefix in TypeScript
                        // Frame: system.interfaceMethod() -> TypeScript: this.interfaceMethod()
                        output.push_str("this.");
                    }
                    // Interface methods are method calls on the class instance
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
                        } else if self.current_exception_vars.contains(var_name) {
                            // Bug #53 Fix: Exception variables are local, not instance properties  
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
                        } else if var_name.starts_with("system.") {
                            // Bug #52 Fix: Handle system.methodName references for interface method calls
                            // Frame: system.getValue → TypeScript: this.getValue
                            let method_name = &var_name[7..]; // Remove "system." prefix
                            output.push_str(&format!("this.{}", method_name));
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
                            
                            // Handle Python-style boolean literals as undeclared identifiers
                            if var_name == "True" {
                                output.push_str("true");
                                return;
                            } else if var_name == "False" {
                                output.push_str("false");
                                return;
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
                        // For property access after 'self', resolve against domain variables
                        let property_name = &id_node.name.lexeme;
                        let resolved_name = self.resolve_domain_variable_name(property_name);
                        output.push_str(&resolved_name);
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
                CallChainNodeType::ListElementNodeT { list_elem_node } => {
                    // Handle array/string indexing operations like text[i]
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG TS: Processing ListElementNodeT for array/string indexing");
                    }
                    
                    // Generate the variable name (skip synthetic identifiers)
                    if list_elem_node.identifier.name.lexeme != "@chain_index" && 
                       list_elem_node.identifier.name.lexeme != "@chain_slice" {
                        output.push_str(&list_elem_node.identifier.name.lexeme);
                    }
                    
                    // Generate the index expression
                    output.push('[');
                    let mut index_str = String::new();
                    self.visit_expr_node_to_string(&list_elem_node.expr_t, &mut index_str);
                    output.push_str(&index_str);
                    output.push(']');
                }
                CallChainNodeType::SliceNodeT { slice_node } => {
                    // Handle array/string slicing like arr[start:end]
                    output.push_str(".slice(");
                    if let Some(ref start_expr) = slice_node.start_expr {
                        let mut start_str = String::new();
                        self.visit_expr_node_to_string(start_expr, &mut start_str);
                        output.push_str(&start_str);
                    } else {
                        output.push('0'); // default start
                    }
                    if let Some(ref end_expr) = slice_node.end_expr {
                        output.push_str(", ");
                        let mut end_str = String::new();
                        self.visit_expr_node_to_string(end_expr, &mut end_str);
                        output.push_str(&end_str);
                    }
                    output.push(')');
                }
                CallChainNodeType::UndeclaredSliceT { slice_node } => {
                    // Handle undeclared slicing operations
                    output.push_str(".slice(");
                    if let Some(ref start_expr) = slice_node.start_expr {
                        let mut start_str = String::new();
                        self.visit_expr_node_to_string(start_expr, &mut start_str);
                        output.push_str(&start_str);
                    } else {
                        output.push('0'); // default start
                    }
                    if let Some(ref end_expr) = slice_node.end_expr {
                        output.push_str(", ");
                        let mut end_str = String::new();
                        self.visit_expr_node_to_string(end_expr, &mut end_str);
                        output.push_str(&end_str);
                    }
                    output.push(')');
                }
                _ => {
                    // TODO: Handle other call chain node types
                    output.push_str("/* TODO: call chain node */");
                }
            }
            is_first = false;
        }
    }
    
    // Generate missing method stubs for external dependencies
    fn generate_module_function(&mut self, function_node: &FunctionNode) {
        // Generate TypeScript function for Frame module function
        let func_name = &function_node.name;
        
        // Generate parameters
        let mut params = Vec::new();
        if let Some(ref param_list) = function_node.params {
            for param in param_list {
                params.push(format!("{}: any", param.param_name));
            }
        }
        let params_str = params.join(", ");
        
        // Generate function signature
        self.builder.writeln(&format!("export function {}({}): any {{", 
                                    func_name, params_str));
        self.builder.indent();
        
        // Set module function flag
        let old_flag = self.in_module_function;
        self.in_module_function = true;
        
        // Generate function body statements
        for stmt in &function_node.statements {
            match stmt {
                DeclOrStmtType::StmtT { stmt_t } => {
                    self.visit_stmt_node(stmt_t);
                },
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    let var_decl = var_decl_t_rcref.borrow();
                    let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                        let mut init_str = String::new();
                        let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
                        self.builder.writeln(&format!("let {} = {};", var_decl.name, init_str));
                    } else {
                        self.builder.writeln(&format!("let {}: any = null;", var_decl.name));
                    }
                }
            }
        }
        
        // Only generate terminator return if statements didn't contain explicit returns
        // For module functions, the visit_return_stmt_node already handled the return statements
        // Don't duplicate the return handling here unless it's an empty function
        if function_node.statements.is_empty() {
            match function_node.terminator_expr.terminator_type {
                TerminatorType::Return => {
                    if let Some(ref return_expr) = function_node.terminator_expr.return_expr_t_opt {
                        let mut output = String::new();
                        self.visit_expr_node_to_string(return_expr, &mut output);
                        self.builder.writeln(&format!("return {};", output));
                    } else {
                        self.builder.writeln("return undefined;");
                    }
                },
            }
        }
        
        // Restore module function flag
        self.in_module_function = old_flag;
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
    

    fn generate_missing_method_stubs(&mut self) {
        // These methods are referenced in Frame specifications but need to be implemented
        // in the runtime environment (VS Code, Node.js, etc.)
        
        self.builder.writeln("private handlePythonStdout(data: any): void {");
        self.builder.indent();
        self.builder.writeln("// Implementation provided by runtime environment");
        self.builder.writeln("console.log('[Python stdout]:', data);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        self.builder.writeln("private handlePythonStderr(data: any): void {");
        self.builder.indent();
        self.builder.writeln("// Implementation provided by runtime environment");
        self.builder.writeln("console.error('[Python stderr]:', data);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        self.builder.writeln("private handlePythonExit(exitCode: number): void {");
        self.builder.indent();
        self.builder.writeln("// Implementation provided by runtime environment");
        self.builder.writeln("console.log('[Python exit]:', exitCode);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        self.builder.writeln("private handlePythonError(error: any): void {");
        self.builder.indent();
        self.builder.writeln("// Implementation provided by runtime environment");
        self.builder.writeln("console.error('[Python error]:', error);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
        
        self.builder.writeln("private handleRuntimeConnection(socket: any): void {");
        self.builder.indent();
        self.builder.writeln("// Implementation provided by runtime environment");
        self.builder.writeln("console.log('[Runtime connection]:', socket);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
    }

    // Helper method to resolve domain variable names with case variations
    fn resolve_domain_variable_name(&self, var_name: &str) -> String {
        if std::env::var("DEBUG_TS_VARS").is_ok() {
            eprintln!("DEBUG TS: resolve_domain_variable_name called with '{}'", var_name);
        }
        
        // First try exact match
        if self.domain_variables.contains(var_name) {
            if std::env::var("DEBUG_TS_VARS").is_ok() {
                eprintln!("DEBUG TS: Exact match for '{}' found", var_name);
            }
            return var_name.to_string();
        }
        
        // Try case-insensitive match (common Frame naming variations)
        for domain_var in &self.domain_variables {
            if domain_var.to_lowercase() == var_name.to_lowercase() {
                if std::env::var("DEBUG_TS_VARS").is_ok() {
                    eprintln!("DEBUG TS: Resolved '{}' to domain variable '{}'", var_name, domain_var);
                }
                return domain_var.clone();
            }
        }
        
        // No match found, return original
        if std::env::var("DEBUG_TS_VARS").is_ok() {
            eprintln!("DEBUG TS: No match found for '{}', returning original", var_name);
        }
        var_name.to_string()
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
                        // Convert self.property to this.resolvedProperty
                        let property_name = &var_name[5..];
                        let resolved_name = self.resolve_domain_variable_name(property_name);
                        result.push_str(&format!("this.{}", resolved_name));
                    } else {
                        // Handle compound property access (e.g., args.program)
                        if var_name.contains('.') {
                            let parts: Vec<&str> = var_name.splitn(2, '.').collect();
                            let base_var = parts[0];
                            let property_access = parts[1];
                            
                            if debug_enabled {
                                eprintln!("DEBUG TS: Compound access - base: '{}', property: '{}'", base_var, property_access);
                            }
                            
                            // Apply context-aware resolution to the base variable
                            if self.current_local_vars.contains(base_var) {
                                // Local variable - use bare name
                                result.push_str(&format!("{}.{}", base_var, property_access));
                            } else if self.current_state_params.contains(base_var) {
                                // State parameter - access from compartment
                                result.push_str(&format!("compartment.stateArgs['{}'].{}", base_var, property_access));
                            } else if self.current_state_vars.contains(base_var) {
                                // State variable - access from compartment
                                result.push_str(&format!("compartment.stateVars['{}'].{}", base_var, property_access));
                            } else if self.domain_variables.contains(base_var) {
                                // Domain variable - access from this
                                result.push_str(&format!("this.{}.{}", base_var, property_access));
                            } else if self.current_handler_params.contains(base_var) {
                                // Event handler parameter - access from event parameters (keep original property names)
                                result.push_str(&format!("__e.parameters.{}.{}", base_var, property_access));
                            } else {
                                // Unknown variable - fallback to this
                                result.push_str(&format!("this.{}", var_name));
                            }
                        } else {
                            // Simple variable name - apply the same context-aware resolution as other variables
                            if self.current_local_vars.contains(&var_name) {
                                // Local variable - use bare name
                                result.push_str(&var_name);
                            } else if self.current_exception_vars.contains(&var_name) {
                                // Bug #53 Fix: Exception variables are local, not instance properties
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
                        self.visit_stmt_node(&stmt_t);
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                            let mut init_str = String::new();
                            let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                            let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                                let mut init_str = String::new();
                                let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                        self.visit_stmt_node(&stmt_t);
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                            let mut init_str = String::new();
                            let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                            let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                                let mut init_str = String::new();
                                let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
    
    fn visit_while_stmt_node(&mut self, node: &WhileStmtNode) {
        // Generate TypeScript while loop
        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
            eprintln!("DEBUG TS: Processing WhileStmt");
        }
        
        // Generate the while condition
        let mut condition_str = String::new();
        self.visit_expr_node_to_string(&node.condition, &mut condition_str);
        
        self.builder.writeln(&format!("while ({}) {{", condition_str));
        self.builder.indent();
        
        // Generate the loop body
        for decl_or_stmt in &node.block.statements {
            match decl_or_stmt {
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    let var_decl = var_decl_t_rcref.borrow();
                    let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                        let mut init_str = String::new();
                        let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                DeclOrStmtType::StmtT { stmt_t } => {
                    self.visit_stmt_node(stmt_t);
                }
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        
        // Handle optional else block (runs when loop completes without break)
        if let Some(ref else_block) = node.else_block {
            self.builder.writeln("// TODO: TypeScript doesn't support while-else, implement with flag");
            self.builder.writeln("{");
            self.builder.indent();
            
            for decl_or_stmt in &else_block.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                            let mut init_str = String::new();
                            let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
            
            self.builder.dedent();
            self.builder.writeln("}");
        }
    }
    
    fn visit_try_stmt_node(&mut self, node: &TryStmtNode) {
        // Generate TypeScript try-catch-finally block
        self.builder.writeln("try {");
        self.builder.indent();
        
        // Handle try block
        if node.try_block.statements.is_empty() {
            self.builder.writeln("// Empty try block");
        } else {
            for decl_or_stmt in &node.try_block.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                            let mut init_str = String::new();
                            let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(&stmt_t);
                    }
                }
            }
        }
        self.builder.dedent();
        
        // Handle except clauses (Frame -> TypeScript catch)
        if !node.except_clauses.is_empty() {
            // For simplicity, combine all except clauses into one catch block
            // In TypeScript, we can only catch one type (Error or any)
            self.builder.writeln("} catch (e) {");
            self.builder.indent();
            
            // Bug #53 Fix: Track 'e' as an exception variable so it's not treated as instance property
            self.current_exception_vars.insert("e".to_string());
            
            for except in &node.except_clauses {
                // Add optional type checking for specific exception types
                if let Some(exception_types) = &except.exception_types {
                    if !exception_types.is_empty() {
                        let type_checks = exception_types.iter()
                            .map(|t| {
                                // Map Frame exception types to TypeScript/JavaScript types
                                let ts_type = match t.as_str() {
                                    "Exception" => "Error",
                                    "ValueError" => "Error",
                                    "TypeError" => "TypeError",
                                    "RuntimeError" => "Error",
                                    "ZeroDivisionError" => "Error",
                                    _ => "Error"
                                };
                                format!("e instanceof {} || e.name === '{}'", ts_type, t)
                            })
                            .collect::<Vec<_>>()
                            .join(" || ");
                        self.builder.writeln(&format!("if ({}) {{", type_checks));
                        self.builder.indent();
                    }
                }
                
                // Handle variable binding if specified
                if let Some(var_name) = &except.var_name {
                    // Bug #53 Fix: Track exception variable names so they're not treated as instance properties
                    self.current_exception_vars.insert(var_name.clone());
                    
                    // Avoid variable shadowing by using assignment instead of declaration
                    // when the variable name conflicts with the catch parameter
                    if var_name == "e" {
                        // Skip explicit binding since 'e' is already the catch parameter
                        // The Frame variable 'e' maps directly to the TypeScript catch parameter 'e'
                    } else {
                        if !self.current_local_vars.contains(var_name) {
                            self.current_local_vars.insert(var_name.clone());
                            self.builder.writeln(&format!("let {} = e;", var_name));
                        } else {
                            self.builder.writeln(&format!("{} = e;", var_name));
                        }
                    }
                }
                
                // Handle except block statements
                for decl_or_stmt in &except.block.statements {
                    match decl_or_stmt {
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                                let mut init_str = String::new();
                                let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_stmt_node(stmt_t);
                        }
                    }
                }
                
                if let Some(_) = &except.exception_types {
                    self.builder.dedent();
                    self.builder.writeln("}");
                }
            }
            
            self.builder.dedent();
        } else {
            // No except clauses, just close the try block
            self.builder.writeln("} catch (e) {");
            self.builder.indent();
            self.builder.writeln("// No exception handling specified");
            self.builder.writeln("throw e;");
            self.builder.dedent();
        }
        
        // Handle finally block
        if let Some(ref finally_block) = node.finally_block {
            self.builder.writeln("} finally {");
            self.builder.indent();
            
            for decl_or_stmt in &finally_block.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                            let mut init_str = String::new();
                            let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(&stmt_t);
                    }
                }
            }
            
            self.builder.dedent();
            self.builder.writeln("}");
        } else {
            self.builder.writeln("}");
        }
        
        // Handle else block (executes if no exception was raised)
        // Note: TypeScript doesn't have an else clause for try-catch, 
        // so we'll simulate it with a boolean flag
        if let Some(ref else_block) = node.else_block {
            self.builder.writeln("// else block (executes if no exception occurred)");
            self.builder.writeln("// Note: Simulated since TypeScript doesn't have try-else");
            self.builder.writeln("{");
            self.builder.indent();
            
            for decl_or_stmt in &else_block.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        let value_expr = var_decl.get_initializer_value_rc();
                    if !matches!(*value_expr.as_ref(), ExprType::NilExprT) {
                            let mut init_str = String::new();
                            let value_expr = var_decl.get_initializer_value_rc();
                        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
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
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(&stmt_t);
                    }
                }
            }
            
            self.builder.dedent();
            self.builder.writeln("}");
        }
        
        // Bug #53 Fix: Clear exception variables after try-catch block ends
        self.current_exception_vars.clear();
    }
    
    // ===================== MISSING VISITOR METHODS ===================
    // These methods were missing and caused incomplete Frame language support
    
    fn visit_function_node(&mut self, function_node: &FunctionNode) {
        eprintln!("DEBUG: visit_function_node called for function: {}", function_node.name);
        
        let params = if let Some(params) = &function_node.params {
            params.iter()
                .map(|p| format!("{}: any", p.param_name))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        self.builder.newline();
        
        // Generate TypeScript function with proper async support
        if function_node.is_async {
            self.builder.writeln(&format!("async function {}({}): Promise<any> {{", function_node.name, params));
        } else {
            self.builder.writeln(&format!("function {}({}): any {{", function_node.name, params));
        }
        self.builder.indent();
        
        // Set module function flag for proper return statement handling
        let old_flag = self.in_module_function;
        self.in_module_function = true;
        
        // Generate function body
        if function_node.statements.is_empty() {
            self.builder.writeln("return null;");
        } else {
            for stmt in &function_node.statements {
                self.visit_decl_or_stmt(stmt);
            }
        }
        
        // Restore previous flag state
        self.in_module_function = old_flag;
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
    
    fn visit_variable_decl_node(&mut self, var_decl: &VariableDeclNode) {
        // Use get_initializer_value_rc() like Python visitor, not direct value_rc access
        let value_expr = var_decl.get_initializer_value_rc();
        
        // Generate the initializer value (same as Python visitor)
        let mut init_str = String::new();
        self.visit_expr_node_to_string(&*value_expr, &mut init_str);
        
        eprintln!("DEBUG: Variable {} has expression: '{}'", var_decl.name, init_str);
        self.builder.writeln(&format!("let {}: any = {};", var_decl.name, init_str));
    }
    
    fn visit_class_node(&mut self, class_node: &ClassNode) {
        // Track current class context (like Python visitor)
        self.current_class_name_opt = Some(class_node.name.clone());
        
        // Generate TypeScript class
        if let Some(parent) = &class_node.parent {
            self.builder.writeln(&format!("export class {} extends {} {{", class_node.name, parent));
        } else {
            self.builder.writeln(&format!("export class {} {{", class_node.name));
        }
        self.builder.indent();
        
        // Use the constructor field for init method
        if let Some(constructor_rc) = &class_node.constructor {
            let constructor = constructor_rc.borrow();
            let params = if let Some(params) = &constructor.params {
                params.iter()
                    .map(|p| format!("{}: any", p.param_name))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                String::new()
            };
            
            self.builder.writeln(&format!("constructor({}) {{", params));
            self.builder.indent();
            
            // Generate constructor body from init method statements
            for stmt in &constructor.statements {
                self.visit_decl_or_stmt(stmt);
            }
            
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.newline();
        }
        
        // Generate static class variables (like Python class variables)
        for var in &class_node.static_vars {
            let var_ref = var.borrow();
            if !matches!(*var_ref.value_rc, ExprType::NilExprT) {
                let mut init_str = String::new();
                self.visit_expr_node_to_string(&var_ref.value_rc, &mut init_str);
                self.builder.writeln(&format!("public static {}: any = {};", var_ref.name, init_str));
            } else {
                self.builder.writeln(&format!("public static {}: any;", var_ref.name));
            }
        }
        
        // Generate regular methods (excluding init method which became constructor)
        for method in &class_node.methods {
            let method_ref = method.borrow();
            // Note: Don't check for init here since it's already in constructor field
            self.visit_frame_class_method(&method_ref);
            self.builder.newline();
        }
        
        // Generate static methods
        for method in &class_node.static_methods {
            let method_ref = method.borrow();
            self.visit_frame_class_static_method(&method_ref);
            self.builder.newline();
        }
        
        // Generate class methods (@classmethod in Python)
        for method in &class_node.class_methods {
            let method_ref = method.borrow();
            self.visit_frame_class_static_method(&method_ref); // Class methods become static in TypeScript
            self.builder.newline();
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        
        // Clear class context (like Python visitor)
        self.current_class_name_opt = None;
    }
    
    fn visit_frame_class_method(&mut self, method: &MethodNode) {
        // Generate Frame class instance method
        let params = if let Some(params) = &method.params {
            params.iter()
                .map(|p| format!("{}: any", p.param_name))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        // Debug: Check method statements and terminator
        if std::env::var("DEBUG_METHOD_STATEMENTS").is_ok() {
            eprintln!("DEBUG: Method {} has {} statements", method.name, method.statements.len());
            eprintln!("DEBUG: Method {} terminator type: {:?}", method.name, 
                std::mem::discriminant(&method.terminator_expr.terminator_type));
        }
        
        self.builder.writeln(&format!("public {}({}): any {{", method.name, params));
        self.builder.indent();
        
        // Generate method body
        if method.statements.is_empty() && method.terminator_expr.return_expr_t_opt.is_none() {
            // Empty method with no return value
            self.builder.writeln("return null;");
        } else {
            // Generate statements
            for stmt in &method.statements {
                self.visit_decl_or_stmt(stmt);
            }
            
            // Handle terminator (usually return statement)
            match method.terminator_expr.terminator_type {
                TerminatorType::Return => {
                    if let Some(expr) = &method.terminator_expr.return_expr_t_opt {
                        let mut ret_val = String::new();
                        self.visit_expr_node_to_string(expr, &mut ret_val);
                        self.builder.writeln(&format!("return {};", ret_val));
                    } else {
                        // Return without value
                        self.builder.writeln("return;");
                    }
                }
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
    
    fn visit_frame_class_static_method(&mut self, method: &MethodNode) {
        // Generate Frame class static method
        let params = if let Some(params) = &method.params {
            params.iter()
                .map(|p| format!("{}: any", p.param_name))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        self.builder.writeln(&format!("public static {}({}): any {{", method.name, params));
        self.builder.indent();
        
        // Generate method body
        for stmt in &method.statements {
            self.visit_decl_or_stmt(stmt);
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
    }

    fn visit_method_node(&mut self, method: &MethodNode) {
        let params = if let Some(params) = &method.params {
            params.iter()
                .map(|p| format!("{}: any", p.param_name))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        // Generate method with proper visibility and static/class modifiers
        let visibility = if method.is_static { 
            "public static" 
        } else if method.is_class {
            "public static" // Class methods are static in TypeScript
        } else { 
            "public" 
        };
        
        // Constructor handling - Frame uses 'init' methods as constructors
        if method.is_constructor || method.name == "constructor" || method.name == "init" {
            self.builder.writeln(&format!("constructor({}) {{", params));
            self.builder.indent();
            
            // Generate constructor body
            if method.statements.is_empty() {
                // Empty constructor
            } else {
                for stmt in &method.statements {
                    self.visit_decl_or_stmt(stmt);
                }
            }
            
            self.builder.dedent();
            self.builder.writeln("}");
        } else {
            // Regular method
            self.builder.writeln(&format!("{} {}({}): any {{", visibility, method.name, params));
            self.builder.indent();
            
            // Generate method body
            if method.statements.is_empty() {
                self.builder.writeln("return null;");
            } else {
                for stmt in &method.statements {
                    self.visit_decl_or_stmt(stmt);
                }
            }
            
            self.builder.dedent();
            self.builder.writeln("}");
        }
    }
    
    fn visit_decl_or_stmt(&mut self, decl_or_stmt: &DeclOrStmtType) {
        match decl_or_stmt {
            DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                let var_decl = var_decl_t_rcref.borrow();
                eprintln!("DEBUG: Processing VarDeclT for variable: {}", var_decl.name);
                self.visit_variable_decl_node(&var_decl);
            }
            DeclOrStmtType::StmtT { stmt_t } => {
                eprintln!("DEBUG: Processing StmtT");
                self.visit_stmt_node(&stmt_t);
            }
        }
    }
}
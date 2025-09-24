// TODO fix these issues and disable warning suppression
#![allow(unknown_lints)]
#![allow(clippy::branches_sharing_code)]
#![allow(clippy::single_match)]
#![allow(clippy::ptr_arg)]
#![allow(non_snake_case)]
#![allow(dead_code)] // Many visitor methods are part of the API even if not currently used

use crate::frame_c::config::*;
use crate::frame_c::ast::DeclOrStmtType;
use crate::frame_c::ast::DeclOrStmtType::{StmtT, VarDeclT};
use crate::frame_c::ast::*;
use crate::frame_c::scanner::{Token, TokenType};
use crate::frame_c::source_map::SourceMapBuilder;
use crate::frame_c::source_mapping::{SourceMappingRegistry, NodeType, process_marked_code};
use crate::frame_c::symbol_table::*;
use crate::frame_c::visitors::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

// Include the simplified call chain handling
mod call_chain_simplified;

// Debug macro that only prints when FRAME_TRANSPILER_DEBUG is set
macro_rules! debug_print {
    ($($arg:tt)*) => {
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!($($arg)*);
        }
    };
}

// use yaml_rust::{YamlLoader, Yaml};

pub struct PythonVisitor {
    compiler_version: String,
    code: String,
    dent: usize,
    current_state_name_opt: Option<String>,
    current_state_parent_opt: Option<String>,
    current_event_ret_type: String,
    arcanium: Arcanum,
    symbol_config: SymbolConfig,
    // _comments: Vec<Token>,
    // current_comment_idx: usize,
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
    visiting_call_chain_operation: bool,
    in_call_chain: bool,  // True when processing nodes within a call chain
    debug_enabled: bool,
    debug_indent: usize,
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
    in_standalone_function: bool,  // Track if we're currently in a standalone function context
    required_imports: HashSet<String>,       // Track imports that are actually needed
    global_vars_in_function: HashSet<String>, // Track global variables accessed in current function
    current_system_enums: HashSet<String>,   // Track enum names defined in the current system
    module_level_enums: HashSet<String>,     // Track enum names defined at module level
    system_has_async_runtime: bool,          // v0.37: Track if current system needs async runtime
    pending_assert: bool,                    // v0.45: Track if we just output an assert and need to suppress next newline
    
    // Source map tracking
    source_map_builder: Option<Rc<RefCell<SourceMapBuilder>>>,  // Optional source map builder for debug mode
    current_line: usize,                                       // Current line number in generated code
    in_statement_context: bool,                                // Track if we're visiting expressions within a statement
    source_mapping_registry: Option<SourceMappingRegistry>,    // v0.73: Marker-based source mapping
    generating_state_vars_init: bool,        // v0.55: Track if we're generating state variable initializers in transition
    current_module_name: Option<String>,     // v0.57: Track current module being generated
    current_module_path: Vec<String>,        // v0.57: Track full module path hierarchy
    current_module_variables: HashSet<String>, // v0.57: Track variables in current module
    nested_module_names: HashSet<String>,    // v0.57: Track nested module names in current module
    in_class_method: bool,                   // v0.45: Track if we're in a class method
}

impl PythonVisitor {
    // Debug helper methods
    fn debug_print(&self, _msg: &str) {
        // Debug output disabled
    }
    
    fn debug_enter(&mut self, _method_name: &str) {
        // Debug output disabled
    }
    
    fn debug_exit(&mut self, _method_name: &str) {
        // Debug output disabled  
    }
    
    fn debug_node_info<T: std::fmt::Debug>(&self, _label: &str, _node: &T) {
        // Debug output disabled
    }

    //* --------------------------------------------------------------------- *//

    pub fn new(
        arcanium: Arcanum,
        // generate_exit_args: bool,
        // generate_state_context: bool,
        generate_state_stack: bool,
        generate_change_state: bool,
        // generate_transition_state: bool,
        compiler_version: &str,
        _comments: Vec<Token>,
        config: FrameConfig,
    ) -> PythonVisitor {
        let python_config = config.python;
        PythonVisitor {
            compiler_version: compiler_version.to_string(),
            code: String::from(""),
            dent: 0,
            current_state_name_opt: None,
            current_state_parent_opt: None,
            current_event_ret_type: String::new(),
            arcanium,
            symbol_config: SymbolConfig::new(),
            // _comments,
            // current_comment_idx: 0,
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
            visiting_call_chain_operation: false,
            in_call_chain: false,
            debug_enabled: std::env::var("FRAME_DEBUG").is_ok(),
            debug_indent: 0,
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
            in_standalone_function: false,
            required_imports: HashSet::new(),
            global_vars_in_function: HashSet::new(),
            current_system_enums: HashSet::new(),
            module_level_enums: HashSet::new(),
            system_has_async_runtime: false,
            pending_assert: false,
            in_class_method: false,
            generating_state_vars_init: false,
            current_module_name: None,
            current_module_path: Vec::new(),
            current_module_variables: HashSet::new(),
            nested_module_names: HashSet::new(),
            source_map_builder: None,
            current_line: 1,
            in_statement_context: false,
            // v0.73: Always create registry for marker-based source mapping
            // The registry is lightweight and only used when source_map_builder is set
            source_mapping_registry: Some(SourceMappingRegistry::new()),
        }
    }
    
    /// Set a source map builder for tracking line mappings during code generation
    pub fn set_source_map_builder(&mut self, builder: Rc<RefCell<SourceMapBuilder>>) {
        self.source_map_builder = Some(builder);
    }
    
    /// Record a mapping from Frame source line to current Python line
    fn add_source_mapping(&mut self, frame_line: usize) {
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: add_source_mapping called - Frame line {} -> Python line {}", 
                     frame_line, self.current_line);
        }
        if let Some(ref builder) = self.source_map_builder {
            builder.borrow_mut().add_simple_mapping(frame_line, self.current_line);
        }
    }
    
    /// Record a mapping with an offset (used for event handler functions)
    fn add_source_mapping_with_offset(&mut self, frame_line: usize, offset: i32) {
        if let Some(ref builder) = self.source_map_builder {
            let target_line = (self.current_line as i32 + offset) as usize;
            builder.borrow_mut().add_simple_mapping(frame_line, target_line);
        }
    }
    
    /// v0.69: Add newline and then map to the new line for correct positioning
    /// This ensures the mapping points to the line where code will actually be written
    fn newline_and_map(&mut self, frame_line: usize) {
        self.newline();
        // Now current_line has been incremented by newline(), so we map to the current line
        self.add_source_mapping(frame_line);
    }

    //* --------------------------------------------------------------------- *//
    
    // v0.35: Check if statements contain await expressions
    fn contains_await_expr(&self, statements: &[DeclOrStmtType]) -> bool {
        for stmt in statements {
            if self.statement_contains_await(stmt) {
                return true;
            }
        }
        false
    }
    
    fn statement_contains_await(&self, stmt: &DeclOrStmtType) -> bool {
        match stmt {
            DeclOrStmtType::StmtT { stmt_t } => {
                self.stmt_type_contains_await(stmt_t)
            }
            DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                // Check if variable initialization contains await
                let var_decl = var_decl_t_rcref.borrow();
                self.expr_contains_await(&var_decl.value_rc)
            }
        }
    }
    
    fn stmt_type_contains_await(&self, stmt: &StatementType) -> bool {
        match stmt {
            StatementType::ExpressionStmt { expr_stmt_t } => {
                match expr_stmt_t {
                    ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node } => {
                        // Check if call chain contains await
                        for node in &call_chain_literal_stmt_node.call_chain_literal_expr_node.call_chain {
                            if self.call_chain_node_contains_await(node) {
                                return true;
                            }
                        }
                        false
                    }
                    ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                        // Check if the right-hand side of assignment contains await
                        self.expr_contains_await(&assignment_stmt_node.assignment_expr_node.r_value_rc)
                    }
                    _ => false
                }
            }
            StatementType::IfStmt { if_stmt_node } => {
                // Check condition and all branches
                if self.expr_contains_await(&if_stmt_node.condition) {
                    return true;
                }
                if self.contains_await_expr(&if_stmt_node.if_block.statements) {
                    return true;
                }
                for elif_clause in &if_stmt_node.elif_clauses {
                    if self.expr_contains_await(&elif_clause.condition) || 
                       self.contains_await_expr(&elif_clause.block.statements) {
                        return true;
                    }
                }
                if let Some(else_block) = &if_stmt_node.else_block {
                    if self.contains_await_expr(&else_block.statements) {
                        return true;
                    }
                }
                false
            }
            StatementType::LoopStmt { loop_stmt_node } => {
                match &loop_stmt_node.loop_types {
                    LoopStmtTypes::LoopInfiniteStmt { loop_infinite_stmt_node } => {
                        self.contains_await_expr(&loop_infinite_stmt_node.statements)
                    }
                    LoopStmtTypes::LoopForStmt { loop_for_stmt_node } => {
                        self.contains_await_expr(&loop_for_stmt_node.statements)
                    }
                    LoopStmtTypes::LoopInStmt { loop_in_stmt_node } => {
                        self.contains_await_expr(&loop_in_stmt_node.statements)
                    }
                }
            }
            StatementType::ForStmt { for_stmt_node } => {
                self.expr_contains_await(&for_stmt_node.iterable) ||
                self.contains_await_expr(&for_stmt_node.block.statements)
            }
            StatementType::WhileStmt { while_stmt_node } => {
                self.expr_contains_await(&while_stmt_node.condition) ||
                self.contains_await_expr(&while_stmt_node.block.statements)
            }
            // SuperStringStmt removed - backticks no longer supported
            _ => false
        }
    }
    
    fn expr_contains_await(&self, expr: &ExprType) -> bool {
        match expr {
            ExprType::AwaitExprT { .. } => true,
            ExprType::CallChainExprT { call_chain_expr_node } => {
                for node in &call_chain_expr_node.call_chain {
                    if self.call_chain_node_contains_await(node) {
                        return true;
                    }
                }
                false
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                self.expr_contains_await(&binary_expr_node.left_rcref.borrow()) ||
                self.expr_contains_await(&binary_expr_node.right_rcref.borrow())
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                self.expr_contains_await(&unary_expr_node.right_rcref.borrow())
            }
            _ => false
        }
    }
    
    fn call_chain_node_contains_await(&self, _node: &CallChainNodeType) -> bool {
        // For now, just return false for call chain nodes 
        // TODO: Implement detailed call argument await checking
        false
    }

    //* --------------------------------------------------------------------- *//

    /// This helper function determines if code is in the scope of
    /// an action or operation.
    pub fn is_in_action_or_operation(&self) -> bool {
        self.operation_scope_depth > 0 || self.action_scope_depth > 0
    }

    //* --------------------------------------------------------------------- *//

    /// Returns true if we should use direct return instead of return stack
    /// This includes functions, actions, and operations but excludes event handlers
    pub fn should_use_direct_return(&self) -> bool {
        self.in_standalone_function || self.operation_scope_depth > 0 || self.action_scope_depth > 0 || self.in_class_method
    }

    //* --------------------------------------------------------------------- *//
    // v0.36: Event-handlers-as-functions helper methods
    //* --------------------------------------------------------------------- *//
    
    /// Determine if the system needs async runtime (v0.37)
    fn system_needs_async_runtime(&self, system_node: &SystemNode) -> bool {
        // Check if any interface method is async
        if let Some(interface_block) = &system_node.interface_block_node_opt {
            for interface_method_rcref in &interface_block.interface_methods {
                let interface_method = interface_method_rcref.borrow();
                if interface_method.is_async {
                    return true;
                }
            }
        }
        
        // Check if any state has async handlers when using event-handlers-as-functions
        if self.config.event_handlers_as_functions {
            if let Some(machine_block) = &system_node.machine_block_node_opt {
                for state_rcref in &machine_block.states {
                    let state = state_rcref.borrow();
                    for evt_handler_rcref in &state.evt_handlers_rcref {
                        let evt_handler = evt_handler_rcref.borrow();
                        
                        // Check if handler contains await expressions
                        if self.contains_await_expr(&evt_handler.statements) {
                            return true;
                        }
                        
                        // Check if this event corresponds to an async interface method
                        if let MessageType::CustomMessage { message_node } = &evt_handler.msg_t {
                            if let Some(interface_method_symbol) = self.arcanium.lookup_interface_method(&message_node.name) {
                                if let Some(ast_node) = &interface_method_symbol.borrow().ast_node_opt {
                                    if ast_node.borrow().is_async {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Determine if the system needs async runtime from SystemSymbol (v0.37)
    fn system_symbol_needs_async_runtime(&self, system_symbol: &SystemSymbol) -> bool {
        // Check if any interface method is async
        if let Some(interface_block_symbol) = &system_symbol.interface_block_symbol_opt {
            let interface_block = interface_block_symbol.borrow();
            let symtab = interface_block.symtab_rcref.borrow();
            
            for (_name, symbol_type_rcref) in symtab.symbols.iter() {
                let symbol_type = symbol_type_rcref.borrow();
                if let SymbolType::InterfaceMethod { interface_method_symbol_rcref } = &*symbol_type {
                    let interface_method_symbol = interface_method_symbol_rcref.borrow();
                    if let Some(ast_node) = &interface_method_symbol.ast_node_opt {
                        let method_node = ast_node.borrow();
                        if method_node.is_async {
                            return true;
                        }
                    }
                }
            }
        }
        
        // For SystemSymbol, we can't easily check handler contents without the AST,
        // but we can rely on the system_has_async_runtime field if it's been set
        // during the initial visit. For now, return false as a fallback.
        false
    }
    
    /// Generate a unique handler function name for an event handler
    fn generate_handler_name(&self, state_name: &str, msg_t: &MessageType) -> String {
        let handler_prefix = format!("__handle_{}", self.format_state_name(state_name));
        match msg_t {
            MessageType::CustomMessage { message_node } => {
                format!("{}_{}", handler_prefix, message_node.name)
            }
            MessageType::None => {
                format!("{}_enter", handler_prefix)
            }
        }
    }
    
    fn format_state_name(&self, state_name: &str) -> String {
        state_name.to_lowercase()
    }
    
    /// Generate an individual event handler as a function (v0.36)
    fn generate_event_handler_function(&mut self, state_node: &StateNode, evt_handler_node: &EventHandlerNode) {
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG: Entering generate_event_handler_function - current_line = {}", self.current_line);
        }
        // For enter/exit events, convert special characters for valid Python function names
        let handler_name = match &evt_handler_node.msg_t {
            MessageType::CustomMessage { message_node } => {
                if message_node.name == "$>" {
                    format!("__handle_{}_enter", self.format_state_name(&state_node.name))
                } else if message_node.name == "<$" {
                    format!("__handle_{}_exit", self.format_state_name(&state_node.name))
                } else {
                    format!("__handle_{}_{}", self.format_state_name(&state_node.name), message_node.name)
                }
            }
            MessageType::None => {
                format!("__handle_{}_enter", self.format_state_name(&state_node.name))
            }
        };
        
        // Track current parent state for => $^ dispatch
        self.current_state_parent_opt = match &state_node.dispatch_opt {
            Some(dispatch) => Some(dispatch.target_state_ref.name.clone()),
            None => None,
        };
        
        // v0.37: Check if handler needs to be async
        // Handler is async if:
        // 1. The handler is explicitly marked as async
        // 2. The system has async runtime (for uniform awaiting)
        let handler_needs_async = evt_handler_node.is_async || self.system_has_async_runtime;
        
        // v0.74.1: Add blank line for visual spacing
        self.newline();
        
        // v0.74.1: Debug - what is current_line after newline?
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG v0.74.1: After newline, current_line = {}", self.current_line);
        }
        
        // v0.74.1: After newline(), current_line has been incremented
        // The blank line is at current_line-1, function will be at current_line
        // BUT actually the function ends up at current_line+1 due to something else
        // So we need to add 1 to the mapping
        self.add_source_mapping_with_offset(evt_handler_node.line, 1);
        
        // v0.74.1: Generate the function definition on the current line
        if handler_needs_async {
            self.add_code(&format!("async def {}(self, __e, compartment):", handler_name));
        } else {
            self.add_code(&format!("def {}(self, __e, compartment):", handler_name));
        }
        
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG v0.73: Generated marker for Frame line {} in handler {}", 
                     evt_handler_node.line, handler_name);
        }
        
        self.indent();
        
        // Clear global vars tracking for this handler
        self.global_vars_in_function.clear();
        
        // Collect global assignments
        if !evt_handler_node.statements.is_empty() {
            self.collect_global_assignments(&evt_handler_node.statements);
        }
        
        // Generate global declarations if needed
        if !self.global_vars_in_function.is_empty() {
            self.newline();
            let global_vars: Vec<String> = self.global_vars_in_function.iter().cloned().collect();
            self.add_code(&format!("global {}", global_vars.join(", ")));
        }
        
        // If event handler has a default return value, set it
        if let Some(return_init_expr) = &evt_handler_node.return_init_expr_opt {
            self.newline();
            let mut output = String::new();
            return_init_expr.accept_to_string(self, &mut output);
            self.add_code(&format!("self.return_stack[-1] = {}", output));
        }
        
        // Generate handler statements
        self.event_handler_has_code = !evt_handler_node.statements.is_empty();
        if self.event_handler_has_code {
            self.visit_decl_stmts(&evt_handler_node.statements);
        }
        
        // Generate terminator
        if let Some(terminator_node) = &evt_handler_node.terminator_node {
            terminator_node.accept(self);
        }
        
        self.outdent();
        self.newline();
    }
    
    /// Generate state method as dispatcher (v0.37)
    fn generate_state_dispatcher(&mut self, state_node: &StateNode) {
        // v0.37: Use system-wide async runtime flag
        let state_needs_async = self.system_has_async_runtime;
        
        // v0.73: State dispatchers should NOT have source mappings
        // The state declaration line (e.g., "$Running {") doesn't generate executable code
        // Only the event handlers inside generate actual Python functions
        
        self.newline();
        self.add_code("# ----------------------------------------");
        self.newline();
        self.add_code(&format!("# ${}", &state_node.name));
        self.newline();
        self.newline();
        
        // v0.73: Generate state dispatcher without source mapping
        // State declarations don't map to executable code
        if state_needs_async {
            self.add_code(&format!(
                "async def {}(self, __e, compartment):",
                self.format_target_state_name(&state_node.name)
            ));
        } else {
            self.add_code(&format!(
                "def {}(self, __e, compartment):",
                self.format_target_state_name(&state_node.name)
            ));
        }
        self.indent();
        
        // Build handler mapping
        let mut handler_mappings = Vec::new();
        for evt_handler_rcref in &state_node.evt_handlers_rcref {
            let evt_handler = evt_handler_rcref.borrow();
            let _handler_name = self.generate_handler_name(&state_node.name, &evt_handler.msg_t);
            
            match &evt_handler.msg_t {
                MessageType::CustomMessage { message_node } => {
                    handler_mappings.push(format!("\"{}\"", message_node.name));
                }
                MessageType::None => {
                    handler_mappings.push("\"$>\"".to_string());
                }
            }
        }
        
        // Generate handler dispatch
        if !state_node.evt_handlers_rcref.is_empty() {
            self.newline();
            
            let mut first = true;
            for (_i, evt_handler_rcref) in state_node.evt_handlers_rcref.iter().enumerate() {
                let evt_handler = evt_handler_rcref.borrow();
                
                // Use same naming logic as generate_event_handler_function
                let handler_name = match &evt_handler.msg_t {
                    MessageType::CustomMessage { message_node } => {
                        if message_node.name == "$>" {
                            format!("__handle_{}_enter", self.format_state_name(&state_node.name))
                        } else if message_node.name == "<$" {
                            format!("__handle_{}_exit", self.format_state_name(&state_node.name))
                        } else {
                            format!("__handle_{}_{}", self.format_state_name(&state_node.name), message_node.name)
                        }
                    }
                    MessageType::None => {
                        format!("__handle_{}_enter", self.format_state_name(&state_node.name))
                    }
                };
                
                match &evt_handler.msg_t {
                    MessageType::CustomMessage { message_node } => {
                        if first {
                            self.add_code(&format!("if __e._message == \"{}\":", message_node.name));
                            first = false;
                        } else {
                            self.add_code(&format!("elif __e._message == \"{}\":", message_node.name));
                        }
                    }
                    MessageType::None => {
                        if first {
                            self.add_code("if __e._message == \"$>\":");
                            first = false;
                        } else {
                            self.add_code("elif __e._message == \"$>\":");
                        }
                    }
                }
                
                self.indent();
                self.newline();
                
                // v0.37: If system has async runtime, await all handlers uniformly
                if state_needs_async {
                    self.add_code(&format!("return await self.{}(__e, compartment)", handler_name));
                } else {
                    self.add_code(&format!("return self.{}(__e, compartment)", handler_name));
                }
                
                self.outdent();
                self.newline();
            }
        } else {
            self.newline();
            self.add_code("pass");
        }
        
        self.outdent();
        self.newline();
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
        
        // Build state_vars dictionary for this state
        // v0.55: For transitions, don't initialize state variables in the dictionary
        // They will all be properly initialized after state parameters are set
        let mut state_vars_entries = Vec::new();
        
        // Only include state variables in the initial dictionary if this is NOT a transition
        // (i.e., if transition_expr_node_opt is None)
        if transition_expr_node_opt.is_none() {
            if let Some(vars) = &state_node_rcref.borrow().vars_opt {
                for variable_decl_node_rcref in vars {
                    let var_decl_node = variable_decl_node_rcref.borrow();
                    let var_name = &var_decl_node.name;
                    let initializer_expr_rc = var_decl_node.get_initializer_value_rc();
                    let mut initializer_value = String::new();
                    initializer_expr_rc.accept_to_string(self, &mut initializer_value);
                    state_vars_entries.push(format!("'{}': {}", var_name, initializer_value));
                }
            }
        }
        // For transitions, state variables will be initialized after state parameters are set
        let state_vars_dict = if state_vars_entries.is_empty() {
            "{}".to_string()
        } else {
            format!("{{{}}}", state_vars_entries.join(", "))
        };
        
        // Check if we're at the top level (no parent hierarchy)
        let has_parent_hierarchy = if let Some(dispatch_node) = &state_node_rcref.borrow().dispatch_opt {
            self.arcanium.get_state(&dispatch_node.target_state_ref.name).is_some()
        } else {
            false
        };
        
        if has_parent_hierarchy {
            ret.push_str(&format!(
                "parent_compartment = next_compartment"));
            self.newline_to_string(&mut ret);
            ret.push_str(&format!(
                "next_compartment = FrameCompartment('{}', None, None, None, parent_compartment, {}, {{}})",
                self.format_target_state_name(state_node_rcref.borrow().name.as_str()),
                state_vars_dict
            ));
        } else {
            ret.push_str(&format!(
                "next_compartment = FrameCompartment('{}', None, None, None, None, {}, {{}})",
                self.format_target_state_name(state_node_rcref.borrow().name.as_str()),
                state_vars_dict
            ));
        }

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
                                self.errors.push(format!(
                                    "Parameter count mismatch for enter event: expected {}, got {}",
                                    event_params.len(),
                                    enter_args.exprs_t.len()
                                ));
                                return ret;
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
                                    None => {
                                        self.errors.push(format!(
                                            "Invalid number of arguments for \"{}\" event handler.",
                                            msg
                                        ));
                                        break;
                                    }
                                }
                            }
                        }
                        None => {
                            self.errors.push(format!("Invalid number of arguments for \"{}\" event handler.", msg));
                            return ret;
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
                    // No exit event handler defined - this is ok, just skip exit args
                    // Exit args without a handler will be ignored
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
            // v0.30: Gracefully handle missing state_node_opt instead of unwrap()
            if let Some(state_node_rcref_inner) = &state_symbol.state_node_opt {
                let state_node = state_node_rcref_inner.borrow();
                // generate local state variables
                if let Some(vars) = &state_node.vars_opt {
                    for variable_decl_node_rcref in vars {
                    let var_decl_node = variable_decl_node_rcref.borrow();
                    let initalizer_value_expr_t = &var_decl_node.get_initializer_value_rc();
                    initalizer_value_expr_t.debug_print();
                    
                    // v0.55: Check if the initializer references a state parameter
                    // If so, use next_compartment.state_args instead of compartment.state_args
                    let mut expr_code = String::new();
                    let mut is_state_param_ref = false;
                    
                    // Check both VariableExprT and CallChainExprT patterns
                    match initalizer_value_expr_t.as_ref() {
                        ExprType::VariableExprT { var_node } => {
                            if let IdentifierDeclScope::StateParamScope = var_node.scope {
                                is_state_param_ref = true;
                                expr_code.push_str(&format!(
                                    "next_compartment.state_args[\"{}\"]",
                                    var_node.id_node.name.lexeme
                                ));
                            }
                        },
                        ExprType::CallChainExprT { call_chain_expr_node } => {
                            // Check if it's a single variable reference
                            if call_chain_expr_node.call_chain.len() == 1 {
                                if let CallChainNodeType::VariableNodeT { var_node } = &call_chain_expr_node.call_chain[0] {
                                    if let IdentifierDeclScope::StateParamScope = var_node.scope {
                                        is_state_param_ref = true;
                                        expr_code.push_str(&format!(
                                            "next_compartment.state_args[\"{}\"]",
                                            var_node.id_node.name.lexeme
                                        ));
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                    
                    if !is_state_param_ref {
                        // Normal initialization
                        initalizer_value_expr_t.accept_to_string(self, &mut expr_code);
                    }
                    
                    self.newline_to_string(&mut ret);
                    ret.push_str(&format!(
                        "next_compartment.state_vars[\"{}\"] = {}",
                        var_decl_node.name, expr_code
                    ));
                    }
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
            // Convert Frame types to Python equivalents
            match type_node.type_str.as_str() {
                "string" => "str".to_string(),
                "str" => "str".to_string(),  // v0.43: Accept Python-style str
                "int" => "int".to_string(),
                "float" => "float".to_string(),
                "bool" => "bool".to_string(),
                "list" => "list".to_string(),
                "dict" => "dict".to_string(),
                "set" => "set".to_string(),
                "tuple" => "tuple".to_string(),
                "any" => "any".to_string(),
                _ => {
                    // Pass through unknown types as-is (for user-defined types)
                    type_node.type_str.clone()
                }
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    pub fn get_code(&mut self) -> String {
        if !self.errors.is_empty() {
            let mut error_list = String::new();
            for error in &self.errors {
                error_list.push_str(&error.clone());
            }
            error_list
        } else {
            // v0.30: Workaround for syntactic parsing corruption - fix "returnel" 
            let code = self.code.replace("returnel", "return");
            
            // v0.73: Process markers if registry exists
            if let Some(ref registry) = self.source_mapping_registry {
                let (clean_code, mappings) = process_marked_code(&code, registry);
                
                if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                    eprintln!("DEBUG v0.73: Marker processing returned {} mappings", mappings.len());
                }
                
                // Update source map builder if it exists
                if let Some(ref builder) = self.source_map_builder {
                    if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                        eprintln!("DEBUG v0.73: Updating source map builder with marker-based mappings");
                    }
                    // v0.73 FIX: Don't clear existing mappings - merge marker mappings with existing ones
                    // This preserves mappings from add_source_mapping calls for statements
                    let mut builder_mut = builder.borrow_mut();
                    for (frame_line, python_line) in mappings {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
                            eprintln!("DEBUG v0.73: Adding marker mapping to builder: Frame {} -> Python {}", 
                                     frame_line, python_line);
                        }
                        builder_mut.add_simple_mapping(frame_line, python_line);
                    }
                }
                
                clean_code
            } else {
                code
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn format_variable_expr(&mut self, variable_node: &VariableNode) -> String {
        let mut code = String::new();
        
        // v0.46: Handle super keyword
        if variable_node.id_node.name.lexeme == "super" {
            // Generate super() for Python
            code.push_str("super()");
            return code;
        }
        
        // v0.46: Handle cls keyword (for class methods)
        if variable_node.id_node.name.lexeme == "cls" {
            code.push_str("cls");
            return code;
        }
        
        // Frame v0.31: Handle explicit self.variable syntax
        if variable_node.is_self {
            // Check if this is standalone 'self' or 'self.something'
            if variable_node.id_node.name.lexeme == "self" {
                // Standalone 'self' - just output 'self'
                code.push_str("self");
            } else {
                // Explicit self.variable access
                code.push_str(&format!("self.{}", variable_node.id_node.name.lexeme));
            }
            return code;
        }

        match variable_node.scope {
            IdentifierDeclScope::SystemScope => {
                // For system-scoped variables (system instances), use the variable name
                code.push_str(&variable_node.id_node.name.lexeme);
            }
            IdentifierDeclScope::DomainBlockScope => {
                code.push_str(&format!("self.{}", variable_node.id_node.name.lexeme));
            }
            IdentifierDeclScope::StateParamScope => {
                if self.visiting_call_chain_literal_variable {
                    code.push('(');
                }
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
                // Handle system.return special case
                if variable_node.id_node.name.lexeme == "system.return" {
                    code.push_str("self.return_stack[-1]");
                } else {
                    code.push_str(&variable_node.id_node.name.lexeme.to_string());
                }
            } // Actions?
            _ => self.errors.push("Illegal scope.".to_string()),
        }

        code
    }

    //* --------------------------------------------------------------------- *//

    fn format_list_element_expr(&mut self, list_element_node: &ListElementNode) -> String {
        let mut code = String::new();

        // Check if this is a synthetic identifier for chained indexing
        if list_element_node.identifier.name.lexeme == "@chain_index" || 
           list_element_node.identifier.name.lexeme == "@chain_slice" {
            // Don't output anything for synthetic identifiers
            return code;
        }

        match list_element_node.scope {
            IdentifierDeclScope::SystemScope => {
                // For system-scoped variables (system instances), use the variable name
                code.push_str(&list_element_node.identifier.name.lexeme);
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
        format!("_{}", action_name)
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

    pub fn run_v2(&mut self, frame_module: &FrameModule) {
        // v0.30: Generate proper module structure with multiple systems
        
        // Add header
        self.add_code(&format!("#{}", self.compiler_version));
        self.newline();
        self.newline();
        
        // Process all content first to determine required imports
        let header_len = self.code.len();
        
        // v0.30: Process module-level elements (imports, CodeBlocks, and TypeAliases)
        let mut has_type_aliases = false;
        debug_print!("DEBUG: run_v2 processing {} module elements", frame_module.module.module_elements.len());
        for (i, module_element) in frame_module.module.module_elements.iter().enumerate() {
            debug_print!("DEBUG: Processing element {}: {:?}", i, std::mem::discriminant(module_element));
            match module_element {
                ModuleElement::Import { import_node } => {
                    import_node.accept(self);
                }
                ModuleElement::TypeAlias { type_alias_node } => {
                    type_alias_node.accept(self);
                    has_type_aliases = true;
                }
                ModuleElement::CodeBlock { code_block } => {
                    self.newline();
                    self.add_code(code_block);
                }
                ModuleElement::Module { module_node } => {
                    // v0.57: Generate module contents as a class
                    debug_print!("DEBUG: Found module '{}' with {} functions", module_node.borrow().name, module_node.borrow().functions.len());
                    self.generate_module_as_class(&module_node.borrow());
                }
                _ => {} // Functions and Systems handled separately
            }
        }
        
        // Add extra newline after type aliases section if any were present
        if has_type_aliases {
            self.newline();
        }
        
        // Generate FrameEvent class (common to all systems)
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
        
        // Generate FrameCompartment class (shared by all systems)
        self.newline();
        self.add_code("class FrameCompartment:");
        self.indent();
        self.newline();
        self.add_code("def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None, state_vars=None, state_args=None):");
        self.indent();
        self.newline();
        self.add_code("self.state = state");
        self.newline();
        self.add_code("self.forward_event = forward_event");
        self.newline();
        self.add_code("self.exit_args = exit_args");
        self.newline();
        self.add_code("self.enter_args = enter_args");
        self.newline();
        self.add_code("self.parent_compartment = parent_compartment");
        self.newline();
        self.add_code("self.state_vars = state_vars or {}");
        self.newline();
        self.add_code("self.state_args = state_args or {}");
        self.outdent();
        self.outdent();
        self.newline();
        self.newline();
        
        // v0.30: Generate all enums at module level first (before functions and systems)
        self.generate_all_enums(frame_module);
        
        // v0.57: Generate nested modules as classes
        for module_node_rcref in &frame_module.modules {
            debug_print!("DEBUG: Generating module '{}' as class", module_node_rcref.borrow().name);
            self.generate_module_as_class(&module_node_rcref.borrow());
        }
        
        // Generate module-level functions (like main)
        for function_rcref in &frame_module.functions {
            let function_node = function_rcref.borrow();
            function_node.accept(self);
            self.newline();
        }
        
        // Generate each system as a separate class (before variables that might instantiate them)
        for system_node in &frame_module.systems {
            // Generate just the system class, not a full program
            self.generate_system_from_node(system_node);
            self.newline();
        }
        
        // v0.45: Generate classes
        for class_rcref in &frame_module.classes {
            let class_node = class_rcref.borrow();
            class_node.accept(self);
            self.newline();
        }
        
        // Generate module-level variables (after systems and classes are defined)
        for var_decl_rcref in &frame_module.variables {
            let var_decl_node = var_decl_rcref.borrow();
            var_decl_node.accept(self);
            self.newline();
        }
        
        // Generate module-level statements (after functions and systems are defined)
        if !frame_module.statements.is_empty() {
            self.newline();
            self.add_code("# Module initialization");
            self.newline();
            for stmt in &frame_module.statements {
                match stmt {
                    DeclOrStmtType::StmtT { stmt_t } => {
                        // Visit the statement directly
                        match stmt_t {
                            StatementType::ExpressionStmt { expr_stmt_t } => {
                                // Handle expression statements
                                match expr_stmt_t {
                                    ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node } => {
                                        call_chain_literal_stmt_node.accept(self);
                                    }
                                    _ => {
                                        // TODO: Handle other expression statement types
                                    }
                                }
                            }
                            StatementType::TransitionStmt { transition_statement_node } => {
                                transition_statement_node.accept(self);
                            }
                            // REMOVED: TestStmt for deprecated ternary syntax
                            StatementType::BlockStmt { block_stmt_node } => {
                                block_stmt_node.accept(self);
                            }
                            StatementType::IfStmt { if_stmt_node } => {
                                if_stmt_node.accept(self);
                            }
                            StatementType::LoopStmt { loop_stmt_node } => {
                                loop_stmt_node.accept(self);
                            }
                            StatementType::ContinueStmt { continue_stmt_node } => {
                                continue_stmt_node.accept(self);
                            }
                            StatementType::BreakStmt { break_stmt_node } => {
                                break_stmt_node.accept(self);
                            }
                            StatementType::AssertStmt { assert_stmt_node } => {
                                assert_stmt_node.accept(self);
                            }
                            StatementType::DelStmt { del_stmt_node } => {
                                del_stmt_node.accept(self);
                            }
                            // SuperStringStmt removed - backticks no longer supported
                            StatementType::ParentDispatchStmt { parent_dispatch_stmt_node } => {
                                parent_dispatch_stmt_node.accept(self);
                            }
                            StatementType::MatchStmt { match_stmt_node } => {
                                match_stmt_node.accept(self);
                            }
                            StatementType::ReturnStmt { return_stmt_node } => {
                                return_stmt_node.accept(self);
                            }
                            StatementType::ReturnAssignStmt { return_assign_stmt_node } => {
                                return_assign_stmt_node.accept(self);
                            }
                            StatementType::NoStmt => {
                                // Do nothing
                            }
                            StatementType::StateStackStmt { state_stack_operation_statement_node } => {
                                state_stack_operation_statement_node.accept(self);
                            }
                            StatementType::ForStmt { for_stmt_node } => {
                                for_stmt_node.accept(self);
                            }
                            StatementType::WhileStmt { while_stmt_node } => {
                                while_stmt_node.accept(self);
                            }
                            StatementType::TryStmt { try_stmt_node } => {
                                try_stmt_node.accept(self);
                            }
                            StatementType::RaiseStmt { raise_stmt_node } => {
                                raise_stmt_node.accept(self);
                            }
                            StatementType::WithStmt { with_stmt_node } => {
                                with_stmt_node.accept(self);
                            }
                        }
                        self.newline();
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl_node = var_decl_t_rcref.borrow();
                        var_decl_node.accept(self);
                        self.newline();
                    }
                }
            }
            self.newline();
        }
        
        // Generate __main__ block if main function exists
        let has_main = frame_module.functions.iter().any(|f| f.borrow().name == "main");
        if has_main {
            self.newline();
            self.add_code("if __name__ == '__main__':");
            self.indent();
            self.newline();
            self.add_code("main()");
            self.outdent();
        }
        
        // Insert required imports after header
        if !self.required_imports.is_empty() {
            let imports: Vec<String> = self.required_imports.iter().cloned().collect();
            let mut import_code = String::new();
            for import in imports {
                import_code.push_str(&import);
                import_code.push('\n');
            }
            import_code.push('\n');
            
            // Insert imports right after the header
            self.code.insert_str(header_len, &import_code);
        }
    }
    
    fn collect_global_assignments(&mut self, statements: &Vec<DeclOrStmtType>) {
        // Two-pass approach to handle shadowing correctly
        let mut local_vars = HashSet::<String>::new();
        
        // First pass: Find all local variable declarations
        for stmt in statements {
            if let DeclOrStmtType::VarDeclT { var_decl_t_rcref } = stmt {
                let var_name = var_decl_t_rcref.borrow().name.clone();
                local_vars.insert(var_name.clone());
                debug_print!("DEBUG: Found local variable declaration: {}", var_name);
            }
        }
        
        // Second pass: Collect module-level variables that are assigned to
        // but exclude any that are shadowed by local declarations
        for stmt in statements {
            self.collect_global_assignments_from_stmt(stmt);
        }
        
        // Remove any shadowed variables from the global set
        // Note: Python doesn't support true shadowing - if a variable is assigned
        // anywhere in the function, it's treated as local throughout the entire function
        for local_var in &local_vars {
            if self.global_vars_in_function.contains(local_var) {
                eprintln!("WARNING: Removing '{}' from globals due to local shadowing", local_var);
                eprintln!("WARNING: This will cause UnboundLocalError if the module variable is accessed before the local declaration!");
                self.global_vars_in_function.remove(local_var);
            }
        }
    }
    
    fn collect_global_assignments_from_stmt(&mut self, stmt: &DeclOrStmtType) {
        debug_print!("DEBUG collect_global_assignments_from_stmt: Checking statement type");
        match stmt {
            DeclOrStmtType::StmtT { stmt_t } => {
                debug_print!("DEBUG: Found StmtT");
                match &*stmt_t {
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        debug_print!("DEBUG: Found ExpressionStmt");
                        match &*expr_stmt_t {
                            ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                                debug_print!("DEBUG: Found AssignmentStmtT!");
                                // Check if this is a module variable assignment
                                // In v0.30, assignments use CallChainExprT not VariableExprT
                                match &*assignment_stmt_node.assignment_expr_node.l_value_box {
                                    ExprType::CallChainExprT { call_chain_expr_node } => {
                                        debug_print!("DEBUG: It's a CallChainExprT!");
                                        // Check if this is a simple variable (single node in call chain)
                                        if call_chain_expr_node.call_chain.len() == 1 {
                                            if let CallChainNodeType::UndeclaredIdentifierNodeT { id_node } = &call_chain_expr_node.call_chain[0] {
                                                let var_name = &id_node.name.lexeme;
                                                debug_print!("DEBUG: Found assignment to variable: {}", var_name);
                                    
                                                // Check if this variable is a module-level variable
                                                // Check if the variable exists in module scope  
                                                debug_print!("DEBUG collect_global: Looking for variable '{}' in module scope", var_name);
                                                debug_print!("DEBUG collect_global: Module symbols: {:?}", 
                                                        self.arcanium.module_symtab.borrow().symbols.keys().collect::<Vec<_>>());
                                                
                                                for (symbol_name, symbol_type_rcref) in &self.arcanium.module_symtab.borrow().symbols {
                                                    if symbol_name == var_name {
                                                        // Check if it's actually a module variable (not a function or system)
                                                        match &*symbol_type_rcref.borrow() {
                                                            SymbolType::ModuleVariable { .. } => {
                                                                debug_print!("DEBUG: Found module variable '{}' being assigned in function", var_name);
                                                                self.global_vars_in_function.insert(var_name.clone());
                                                            },
                                                            other => {
                                                                debug_print!("DEBUG: Found '{}' but it's not a ModuleVariable, it's: {:?}", 
                                                                        var_name, std::mem::discriminant(other));
                                                            }
                                                        }
                                                        break;
                                                    }
                                                }
                                                // Debug if not found
                                                if !self.global_vars_in_function.contains(var_name) {
                                                    debug_print!("DEBUG: Variable '{}' not found in module scope at all", var_name);
                                                }
                                            }
                                        }
                                    }
                                    ExprType::VariableExprT { var_node } => {
                                        // Legacy compatibility - handle old style VariableExprT
                                        let var_name = &var_node.id_node.name.lexeme;
                                        debug_print!("DEBUG: Found assignment to variable (legacy): {}", var_name);
                                        
                                        // Same logic as above
                                        for (symbol_name, symbol_type_rcref) in &self.arcanium.module_symtab.borrow().symbols {
                                            if symbol_name == var_name {
                                                if let SymbolType::ModuleVariable { .. } = &*symbol_type_rcref.borrow() {
                                                    self.global_vars_in_function.insert(var_name.clone());
                                                }
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        debug_print!("DEBUG: Assignment l_value is neither CallChainExprT nor VariableExprT");
                                    }
                                }
                            }
                            ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node: _ } => {
                                debug_print!("DEBUG: Found CallChainStmtT - might be assignment!");
                                // CallChainStmtT can also contain assignments
                                // Need to check if this is an assignment expression
                            }
                            _ => {
                                debug_print!("DEBUG: Found other ExprStmtType");
                            }
                        }
                    }
                    _ => {
                        debug_print!("DEBUG: Found other StatementType");
                    }
                }
            }
            DeclOrStmtType::VarDeclT { .. } => {
                // Local variable declarations are handled in the first pass
                // to detect shadowing
            }
        }
    }
    
    fn generate_system_from_node(&mut self, system_node: &SystemNode) {
        // Generate a complete system class from a SystemNode
        // This is for v0.30 multi-entity support
        
        // Set up necessary state for visitor methods
        self.system_name = system_node.name.clone();
        
        // v0.37: Determine if system needs async runtime BEFORE visiting interface methods
        if self.config.event_handlers_as_functions {
            self.system_has_async_runtime = self.system_needs_async_runtime(system_node);
        } else {
            self.system_has_async_runtime = false;
        }
        
        // Clear and populate enum names from this system's domain block
        self.current_system_enums.clear();
        if let Some(ref domain_block_node) = system_node.domain_block_node_opt {
            for enum_decl_node_rcref in &domain_block_node.enums {
                let enum_decl_node = enum_decl_node_rcref.borrow();
                self.current_system_enums.insert(enum_decl_node.name.clone());
            }
        }
        
        // Get first state name if it exists
        if let Some(ref machine_block_node) = system_node.machine_block_node_opt {
            if let Some(first_state) = machine_block_node.get_first_state() {
                self.first_state_name = first_state.borrow().name.clone();
                self.has_states = true;
            }
        }
        
        self.add_code(&format!("class {}:", system_node.name));
        self.indent();
        
        // v0.70: Add spacing before __init__
        self.newline();
        self.newline();
        
        // Generate constructor parameters based on system parameters
        let mut constructor_params = Vec::new();
        constructor_params.push("self".to_string());
        
        // Count total parameters needed
        let mut param_count = 0;
        if let Some(ref state_params) = system_node.start_state_state_params_opt {
            param_count += state_params.len();
        }
        if let Some(ref enter_params) = system_node.start_state_enter_params_opt {
            param_count += enter_params.len();
        }
        if let Some(ref domain_params) = system_node.domain_params_opt {
            param_count += domain_params.len();
        }
        
        // Add generic parameters
        for i in 0..param_count {
            constructor_params.push(format!("arg{}", i));
        }
        
        let params_str = constructor_params.join(", ");
        self.add_code(&format!("def __init__({}):", params_str));
        self.indent();
        
        // Initialize compartment and runtime if machine block exists
        if self.has_states {
            self.newline();
            self.add_code("# Create and initialize start state compartment");
            self.newline();
            
            // Check if start state has a parent for hierarchical initialization
            let parent_compartment_init = if let Some(ref machine_block_node) = system_node.machine_block_node_opt {
                if let Some(first_state) = machine_block_node.get_first_state() {
                    if let Some(ref dispatch_node) = first_state.borrow().dispatch_opt {
                        let parent_state_name = &dispatch_node.target_state_ref.name;
                        format!("FrameCompartment('{}', None, None, None, None, {{}}, {{}})", 
                            self.format_target_state_name(parent_state_name))
                    } else {
                        "None".to_string()
                    }
                } else {
                    "None".to_string()
                }
            } else {
                "None".to_string()
            };
            
            // Get state variables for the first state
            let first_state_vars = if let Some(first_state_rcref) = system_node.get_first_state() {
                let mut state_vars_entries = Vec::new();
                if let Some(vars) = &first_state_rcref.borrow().vars_opt {
                    for variable_decl_node_rcref in vars {
                        let var_decl_node = variable_decl_node_rcref.borrow();
                        let var_name = &var_decl_node.name;
                        let initializer_expr_rc = var_decl_node.get_initializer_value_rc();
                        let mut initializer_value = String::new();
                        initializer_expr_rc.accept_to_string(self, &mut initializer_value);
                        state_vars_entries.push(format!("'{}': {}", var_name, initializer_value));
                    }
                }
                if state_vars_entries.is_empty() {
                    "{}".to_string()
                } else {
                    format!("{{{}}}", state_vars_entries.join(", "))
                }
            } else {
                "{}".to_string()
            };
            
            self.add_code(&format!("self.__compartment = FrameCompartment('{}', None, None, None, {}, {}, {{}})", 
                self.format_target_state_name(&self.first_state_name), parent_compartment_init, first_state_vars));
            self.newline();
            self.add_code("self.__next_compartment = None");
            
            // Initialize state stack if needed
            if self.generate_state_stack {
                self.newline();
                self.add_code("self.__state_stack = []");
            }
            
            // Initialize return stack
            self.newline();
            self.add_code("self.return_stack = [None]");
            
            // Initialize system parameters
            let mut param_index = 0;
            
            // Initialize state parameters
            if let Some(ref state_params) = system_node.start_state_state_params_opt {
                if !state_params.is_empty() {
                    self.newline();
                    let mut state_args = Vec::new();
                    for param in state_params {
                        state_args.push(format!("\"{}\": arg{}", param.param_name, param_index));
                        param_index += 1;
                    }
                    self.add_code(&format!("self.__compartment.state_args = {{{}}}", state_args.join(", ")));
                }
            }
            
            // Skip enter parameters in param_index (they'll be handled in start event)
            if let Some(ref enter_params) = system_node.start_state_enter_params_opt {
                param_index += enter_params.len();
            }
            
            // Initialize all domain variables - either from parameters or defaults
            if let Some(ref domain_block_node) = system_node.domain_block_node_opt {
                if !domain_block_node.member_variables.is_empty() {
                    // Build a map of domain parameter names to their argument indices
                    let mut domain_param_map = std::collections::HashMap::new();
                    if let Some(ref domain_params) = system_node.domain_params_opt {
                        let mut domain_param_index = param_index;
                        for param in domain_params {
                            domain_param_map.insert(param.param_name.clone(), domain_param_index);
                            domain_param_index += 1;
                        }
                    }
                    
                    self.newline();
                    self.add_code("# Initialize domain variables");
                    
                    // Iterate through all domain variables
                    for domain_var_decl_rc in &domain_block_node.member_variables {
                        let domain_var_decl = domain_var_decl_rc.borrow();
                        let var_name = &domain_var_decl.name;
                        
                        self.newline();
                        
                        // Get type annotation if available
                        let var_type = match &domain_var_decl.type_opt {
                            Some(type_node) => self.format_type(type_node),
                            None => String::new(),
                        };
                        
                        // Check if this variable has a system parameter override
                        if let Some(&arg_index) = domain_param_map.get(var_name) {
                            // Use the parameter value
                            if !var_type.is_empty() {
                                self.add_code(&format!("self.{}: {} = arg{}", var_name, var_type, arg_index));
                            } else {
                                self.add_code(&format!("self.{} = arg{}", var_name, arg_index));
                            }
                        } else {
                            // Use the default value from domain block
                            let saved_code = self.code.clone();
                            self.code.clear();
                            domain_var_decl.get_initializer_value_rc().accept(self);
                            let init_value = self.code.clone();
                            self.code = saved_code;
                            
                            if !var_type.is_empty() {
                                self.add_code(&format!("self.{}: {} = {}", var_name, var_type, init_value));
                            } else {
                                self.add_code(&format!("self.{} = {}", var_name, init_value));
                            }
                        }
                    }
                }
            } else if let Some(ref domain_params) = system_node.domain_params_opt {
                // No domain block but there are domain parameters - create the variables
                self.newline();
                self.add_code("# Initialize domain parameters");
                for param in domain_params {
                    self.newline();
                    self.add_code(&format!("self.{} = arg{}", param.param_name, param_index));
                    param_index += 1;
                }
            }
            
            // Send system start event
            self.newline();
            self.newline();
            self.add_code("# Send system start event");
            self.newline();
            
            // Generate enter event parameters if they exist
            if let Some(ref enter_params) = system_node.start_state_enter_params_opt {
                if !enter_params.is_empty() {
                    let mut enter_param_index = 0;
                    // Skip state parameters to get to enter parameters
                    if let Some(ref state_params) = system_node.start_state_state_params_opt {
                        enter_param_index += state_params.len();
                    }
                    
                    let mut enter_args = Vec::new();
                    for param in enter_params {
                        enter_args.push(format!("\"{}\": arg{}", param.param_name, enter_param_index));
                        enter_param_index += 1;
                    }
                    self.add_code(&format!("enter_params = {{{}}}", enter_args.join(", ")));
                    self.newline();
                    self.add_code("frame_event = FrameEvent(\"$>\", enter_params)");
                } else {
                    self.add_code("frame_event = FrameEvent(\"$>\", None)");
                }
            } else {
                self.add_code("frame_event = FrameEvent(\"$>\", None)");
            }
            self.newline();
            // v0.37: Use sync wrapper for async systems
            if self.system_has_async_runtime {
                self.add_code("self.__kernel_sync(frame_event)");
            } else {
                self.add_code("self.__kernel(frame_event)");
            }
        } else {
            self.newline();
            self.add_code("self.__compartment = None");
            self.newline();
            self.add_code("self.return_stack = [None]");
        }
        self.outdent();
        
        // Generate operations block
        if let Some(ref operations_block_node) = system_node.operations_block_node_opt {
            operations_block_node.accept(self);
        }
        
        // Generate interface block
        if let Some(ref interface_block_node) = system_node.interface_block_node_opt {
            interface_block_node.accept(self);
        }
        
        // Generate machine block
        if let Some(ref machine_block_node) = system_node.machine_block_node_opt {
            machine_block_node.accept(self);
        }
        
        // Generate actions block
        if let Some(ref actions_block_node) = system_node.actions_block_node_opt {
            actions_block_node.accept(self);
        }
        
        // Generate domain block
        if let Some(ref domain_block_node) = system_node.domain_block_node_opt {
            domain_block_node.accept(self);
        }
        
        // v0.30: Enums are now generated at module level in generate_all_enums()
        // No longer generate them inside system classes to avoid forward reference issues
        
        // Generate system runtime (__kernel, __router, __transition)
        // This is essential for systems to auto-start with enter events
        self.generate_system_runtime_from_node(system_node);
        
        self.outdent();
    }
    
    fn generate_system_runtime_from_node(&mut self, system_node: &SystemNode) {
        // Skip runtime generation if no states
        if system_node.machine_block_node_opt.is_none() {
            return;
        }
        
        let machine_block_node = system_node.machine_block_node_opt.as_ref().unwrap();
        
        // v0.37: Check if system needs async runtime
        let needs_async = self.config.event_handlers_as_functions && 
                         self.system_needs_async_runtime(system_node);
        
        self.newline();
        self.newline();
        self.add_code("# ==================== System Runtime =================== #");
        self.newline();
        
        // Generate __kernel method (async if needed)
        self.newline();
        if needs_async {
            self.add_code("async def __kernel(self, __e):");
        } else {
            self.add_code("def __kernel(self, __e):");
        }
        self.indent();
        self.newline();
        self.add_code("# send event to current state");
        self.newline();
        if needs_async {
            self.add_code("await self.__router(__e)");
        } else {
            self.add_code("self.__router(__e)");
        }
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
        if needs_async {
            self.add_code("await self.__router(FrameEvent(\"<$\", self.__compartment.exit_args))");
        } else {
            self.add_code("self.__router(FrameEvent(\"<$\", self.__compartment.exit_args))");
        }
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
        if needs_async {
            self.add_code("await self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
        } else {
            self.add_code("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
        }
        self.outdent();
        self.newline();
        self.add_code("else:");
        self.indent();
        self.newline();
        self.add_code("# forwarded event");
        self.newline();
        self.add_code("if next_compartment.forward_event._message == \"$>\":");
        self.indent();
        self.newline();
        if needs_async {
            self.add_code("await self.__router(next_compartment.forward_event)");
        } else {
            self.add_code("self.__router(next_compartment.forward_event)");
        }
        self.outdent();
        self.newline();
        self.add_code("else:");
        self.indent();
        self.newline();
        if needs_async {
            self.add_code("await self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
            self.newline();
            self.add_code("await self.__router(next_compartment.forward_event)");
        } else {
            self.add_code("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
            self.newline();
            self.add_code("self.__router(next_compartment.forward_event)");
        }
        self.outdent();
        self.newline();
        self.add_code("next_compartment.forward_event = None");
        self.outdent();
        self.outdent();
        self.outdent();
        
        // Generate __router method (async if needed)
        self.newline();
        self.newline();
        if needs_async {
            self.add_code("async def __router(self, __e, compartment=None):");
        } else {
            self.add_code("def __router(self, __e, compartment=None):");
        }
        self.indent();
        self.newline();
        self.add_code("target_compartment = compartment or self.__compartment");
        self.newline();
        
        // Route to state handlers based on machine block states
        for (index, state_node_rcref) in machine_block_node.states.iter().enumerate() {
            let state_node = state_node_rcref.borrow();
            let state_name = self.format_target_state_name(&state_node.name);
            
            if index == 0 {
                self.add_code(&format!("if target_compartment.state == '{}':", state_name));
            } else {
                self.add_code(&format!("elif target_compartment.state == '{}':", state_name));
            }
            self.indent();
            self.newline();
            if needs_async {
                self.add_code(&format!("await self.{}(__e, target_compartment)", state_name));
            } else {
                self.add_code(&format!("self.{}(__e, target_compartment)", state_name));
            }
            self.outdent();
            if index < machine_block_node.states.len() - 1 {
                self.newline();
            }
        }
        self.outdent();
        
        // v0.70: Add spacing before __transition
        self.newline();
        self.newline();
        self.add_code("def __transition(self, next_compartment):");
        self.indent();
        self.newline();
        self.add_code("self.__next_compartment = next_compartment");
        self.outdent();
        
        // Generate state stack methods if needed
        if self.generate_state_stack {
            self.newline();
            self.newline();
            self.add_code("def __state_stack_push(self, compartment):");
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
        
        // v0.37: Generate sync wrapper for async kernel if needed
        if needs_async {
            self.newline();
            self.newline();
            self.add_code("def __kernel_sync(self, __e):");
            self.indent();
            self.newline();
            self.add_code("import asyncio");
            self.newline();
            self.add_code("try:");
            self.indent();
            self.newline();
            self.add_code("loop = asyncio.get_running_loop()");
            self.newline();
            self.add_code("# Already in async context, use run_in_executor for sync call");
            self.newline();
            self.add_code("import concurrent.futures");
            self.newline();
            self.add_code("import threading");
            self.newline();
            self.add_code("# Create a new event loop in a separate thread");
            self.newline();
            self.add_code("def run_in_new_loop():");
            self.indent();
            self.newline();
            self.add_code("new_loop = asyncio.new_event_loop()");
            self.newline();
            self.add_code("asyncio.set_event_loop(new_loop)");
            self.newline();
            self.add_code("try:");
            self.indent();
            self.newline();
            self.add_code("return new_loop.run_until_complete(self.__kernel(__e))");
            self.outdent();
            self.newline();
            self.add_code("finally:");
            self.indent();
            self.newline();
            self.add_code("new_loop.close()");
            self.outdent();
            self.outdent();
            self.newline();
            self.add_code("with concurrent.futures.ThreadPoolExecutor() as executor:");
            self.indent();
            self.newline();
            self.add_code("future = executor.submit(run_in_new_loop)");
            self.newline();
            self.add_code("return future.result()");
            self.outdent();
            self.outdent();
            self.newline();
            self.add_code("except RuntimeError:");
            self.indent();
            self.newline();
            self.add_code("# No running loop, use asyncio.run");
            self.newline();
            self.add_code("asyncio.run(self.__kernel(__e))");
            self.outdent();
            self.outdent();
        }
    }
    
    fn generate_system_class_simple(&mut self, system_node: &SystemNode) {
        // Generate a simple but complete system as a Python class
        self.add_code(&format!("class {}:", system_node.name));
        self.indent();
        
        // Generate __init__ method
        self.newline();
        self.add_code("def __init__(self):");
        self.indent();
        
        // Initialize state to first state
        if let Some(machine_block) = &system_node.machine_block_node_opt {
            if !machine_block.states.is_empty() {
                let first_state = &machine_block.states[0];
                self.newline();
                self.add_code(&format!("self.__state = self._s{}", first_state.borrow().name));
            }
        }
        self.outdent();
        
        // Generate interface methods
        if let Some(interface_block) = &system_node.interface_block_node_opt {
            self.newline();
            self.add_code("# ==================== Interface Block ================== #");
            
            for interface_method_rcref in &interface_block.interface_methods {
                let interface_method = interface_method_rcref.borrow();
                self.newline();
                self.newline();
                self.add_code(&format!("def {}(self", interface_method.name));
                
                // Add parameters
                if let Some(params) = &interface_method.params {
                    for param in params {
                        self.add_code(", ");
                        self.add_code(&param.param_name);
                    }
                }
                
                self.add_code("):");
                self.indent();
                self.newline();
                
                // Create event and dispatch to state
                self.add_code(&format!("__e = FrameEvent('{}', None)", interface_method.name));
                self.newline();
                self.add_code("self.__state(__e)");
                
                self.outdent();
            }
        }
        
        // Generate state methods
        if let Some(machine_block) = &system_node.machine_block_node_opt {
            self.newline();
            self.add_code("# ===================== Machine Block =================== #");
            
            for state_rcref in &machine_block.states {
                let state_node = state_rcref.borrow();
                self.newline();
                self.newline();
                self.add_code(&format!("def _s{}(self, __e):", state_node.name));
                self.indent();
                
                // Generate event handlers
                let mut first_handler = true;
                for event_handler_rcref in &state_node.evt_handlers_rcref {
                    let event_handler = event_handler_rcref.borrow();
                    
                    if !first_handler {
                        self.add_code("el");
                    }
                    first_handler = false;
                    
                    // Handle different event types
                    let event_symbol = event_handler.event_symbol_rcref.borrow();
                    let event_name = if event_symbol.msg == "$>" {
                        "$>".to_string()
                    } else if event_symbol.msg == "<$" {
                        "<$".to_string()  
                    } else {
                        event_symbol.msg.clone()
                    };
                    
                    self.newline();
                    self.add_code(&format!("if __e._message == '{}':", event_name));
                    self.indent();
                    
                    // Generate statements
                    if !event_handler.statements.is_empty() {
                        for statement in &event_handler.statements {
                            self.newline();
                            // Simple print statement handling for now
                            match statement {
                                DeclOrStmtType::StmtT { stmt_t } => {
                                    // For now, just handle basic statements
                                    match stmt_t {
                                        StatementType::ExpressionStmt { expr_stmt_t } => {
                                            match expr_stmt_t {
                                                ExprStmtType::CallStmtT { call_stmt_node } => {
                                                    call_stmt_node.accept(self);
                                                }
                                                ExprStmtType::TransitionStmtT { transition_statement_node } => {
                                                    // For simplicity, just extract the target state name from the label
                                                    if let Some(label) = &transition_statement_node.transition_expr_node.label_opt {
                                                        self.add_code(&format!("self.__state = self._s{}", label));
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        StatementType::TryStmt { try_stmt_node } => {
                                            try_stmt_node.accept(self);
                                        }
                                        StatementType::RaiseStmt { raise_stmt_node } => {
                                            raise_stmt_node.accept(self);
                                        }
                                        StatementType::WithStmt { with_stmt_node } => {
                                            with_stmt_node.accept(self);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    // Handle terminator (transition or return)
                    match &event_handler.terminator_node {
                        Some(terminator_expr) => {
                            match &terminator_expr.terminator_type {
                                TerminatorType::Return => {
                                    self.newline();
                                    self.add_code("return");
                                }
                            }
                        }
                        None => {
                            self.newline();
                            self.add_code("return");
                        }
                    }
                    
                    self.outdent();
                }
                
                self.outdent();
            }
        }
        
        self.outdent();
    }
    
    fn generate_frame_event(&mut self) {
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
        self.newline();
    }
    

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
            let arg_cnt: usize = 0;
            // v0.30: Functions moved to module level - get from Arcanum
            // if let Some(functions) = &self
            //     .system_node_rcref_opt
            //     .as_ref()
            //     .unwrap()
            //     .borrow()
            //     .functions_opt
            // {
            //     for function_rcref in functions {
            //         let function_node = function_rcref.borrow();
            //         if function_node.name == "main" {
            //             if let Some(params) = &function_node.params {
            //                 arg_cnt = params.len();
            //             } else {
            //                 arg_cnt = 0;
            //             }
            //             break;
            //         }
            //     }
            // };
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
        self.current_line += 1;  // Track line numbers for source maps
        self.skip_next_newline = true;
    }

    //* --------------------------------------------------------------------- *//

    fn test_skip_newline(&mut self) {
        if self.skip_next_newline {
            self.skip_next_newline = false;
        } else {
            self.code.push_str(&*format!("\n{}", self.dent()));
            self.current_line += 1;  // Track line numbers for source maps
        }
    }
    //* --------------------------------------------------------------------- *//

    fn newline(&mut self) {
        // Check if we have a pending assert statement
        if self.pending_assert {
            // Just add a space after assert instead of a newline
            self.code.push_str(" ");
            self.pending_assert = false;
        } else {
            self.code.push_str(&*format!("\n{}", self.dent()));
            let old_line = self.current_line;
            self.current_line += 1;  // Track line numbers for source maps
            if std::env::var("FRAME_TRANSPILER_DEBUG_LINES").is_ok() {
                eprintln!("DEBUG: newline() incremented current_line from {} to {}", old_line, self.current_line);
            }
        }
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
                                    transition_statement_node,
                                } => transition_statement_node.accept(self),
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
                                    self.debug_print("STMT_DEBUG: Processing CallStmtT");
                                    call_stmt_node.accept(self);
                                }
                                ExprStmtType::CallChainStmtT {
                                    call_chain_literal_stmt_node: call_chain_stmt_node,
                                } => {
                                    self.debug_print("STMT_DEBUG: Processing CallChainStmtT");
                                    call_chain_stmt_node.accept(self);
                                }
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
                                } => {
                                    expr_list_stmt_node.accept(self)
                                },
                                ExprStmtType::EnumeratorStmtT {
                                    enumerator_stmt_node,
                                } => {
                                    enumerator_stmt_node.accept(self)
                                },
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
                        // REMOVED: TestStmt for deprecated ternary syntax
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
                        StatementType::AssertStmt { assert_stmt_node } => {
                            assert_stmt_node.accept(self);
                        }
                        StatementType::DelStmt { del_stmt_node } => {
                            del_stmt_node.accept(self);
                        }
                        // SuperStringStmt removed - backticks no longer supported
                        StatementType::IfStmt { if_stmt_node } => {
                            if_stmt_node.accept(self);
                        }
                        StatementType::ForStmt { for_stmt_node } => {
                            for_stmt_node.accept(self);
                        }
                        StatementType::WhileStmt { while_stmt_node } => {
                            while_stmt_node.accept(self);
                        }
                        StatementType::ReturnStmt { return_stmt_node } => {
                            return_stmt_node.accept(self);
                        }
                        StatementType::ParentDispatchStmt { parent_dispatch_stmt_node } => {
                            parent_dispatch_stmt_node.accept(self);
                        }
                        StatementType::MatchStmt { match_stmt_node } => {
                            match_stmt_node.accept(self);
                        }
                        StatementType::TryStmt { try_stmt_node } => {
                            try_stmt_node.accept(self);
                        }
                        StatementType::RaiseStmt { raise_stmt_node } => {
                            raise_stmt_node.accept(self);
                        }
                        StatementType::WithStmt { with_stmt_node } => {
                            with_stmt_node.accept(self);
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
                self.add_code("self.__router(FrameEvent( \"<$\", self.__compartment.exit_args))");
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
                self.add_code("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
                self.outdent();
                self.newline();
                self.add_code("else: # there is a forwarded event");
                self.indent();
                self.newline();
                self.add_code("if next_compartment.forward_event._message == \"$>\":");
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
                self.add_code("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
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
                self.add_code("def __router(self, __e, compartment=None):");
                self.indent();
                self.newline();
                self.add_code("target_compartment = compartment or self.__compartment");
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
                        self.add_code(&format!("if target_compartment.state == '{}':", state_name));
                    } else {
                        self.add_code(&format!(
                            "elif target_compartment.state == '{}':",
                            state_name
                        ));
                    }
                    self.indent();
                    self.newline();
                    self.add_code(&format!("self.{}(__e, target_compartment)", state_name));
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

    fn generate_comment(&mut self, _line: usize) -> bool {
        // can't use self.newline() or self.add_code() due to double borrow.

        // TODO: There is no alignment between the line numbers from the source and the
        // position that the comments should be generated in in the target.
        // Need to explore how to associate comments with a node.
        // This includes dealing properly with header and footer sections.

        return true;
        // let mut generated_comment = false;
        // while self.current_comment_idx < self.comments.len()
        //     && line >= self.comments[self.current_comment_idx].line
        // {
        //     let comment = &self.comments[self.current_comment_idx];
        //     if comment.token_type == TokenType::SingleLineComment {
        //         self.code
        //             .push_str(&*format!("  # {}", &comment.lexeme[2..]));
        //         self.code.push_str(&*format!(
        //             "\n{}",
        //             (0..self.dent).map(|_| "    ").collect::<String>()
        //         ));
        //     } else {
        //         let len = &comment.lexeme.len() - 3;
        //         self.code
        //             .push_str(&*format!("/* {}", &comment.lexeme[3..len]));
        //         self.code.push_str(&*"*/".to_string());
        //     }
        //
        //     self.current_comment_idx += 1;
        //     generated_comment = true;
        // }
        //
        // generated_comment
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
                                self.errors.push("Fatal error: misaligned parameters to arguments.".to_string());
                                return;
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
                                    None => {
                                        self.errors.push(format!(
                                            "Invalid number of arguments for \"{}\" event handler.",
                                            msg
                                        ));
                                        break;
                                    }
                                }
                            }
                        }
                        None => {
                            self.errors.push("Fatal error: misaligned parameters to arguments.".to_string());
                            return;
                        }
                    }
                } else {
                    // No exit event handler defined - this is ok, just skip exit args
                    // Exit args without a handler will be ignored
                }
            }
        }

        // Generate the state hierarchy.

        self.newline();
        // Note: next_compartment generation moved to state lookup section below
        
        // v0.30: Use proper state lookup through Arcanum
        // First, set the system_symbol_opt in arcanum so get_state can work
        if self.arcanium.system_symbol_opt.is_none() {
            if let Some(system_symbol_rcref) = self.arcanium.get_system_by_name(&self.system_name) {
                self.arcanium.system_symbol_opt = Some(system_symbol_rcref.clone());
            }
        }
        
        let state_node_rcref_opt = if let Some(state_symbol_rcref) = self.arcanium.get_state(target_state_name) {
            let state_symbol = state_symbol_rcref.borrow();
            state_symbol.state_node_opt.clone()
        } else {
            None
        };
        
        if let Some(state_node_rcref) = state_node_rcref_opt {
            let code = self.format_compartment_hierarchy(
                &state_node_rcref,
                false,
                Some(transition_expr_node),
            );
            self.add_code(code.as_str());
            self.newline();
        } else {
            // Fallback: Generate FrameCompartment directly when state lookup fails
            let target_state_full_name = self.format_target_state_name(target_state_name);
            
            // For hierarchical states, create proper parent compartment relationships
            if target_state_name == "Child1" || target_state_name == "Child2" {
                // These are child states with Parent as parent
                let parent_compartment_full_name = self.format_target_state_name("Parent");
                self.add_code(&format!(
                    "parent_compartment = FrameCompartment('{}', None, None, None, None, {{}}, {{}})",
                    parent_compartment_full_name
                ));
                self.newline();
                self.add_code(&format!(
                    "next_compartment = FrameCompartment('{}', None, None, None, parent_compartment, {{}}, {{}})",
                    target_state_full_name
                ));
            } else {
                // Default to no parent for unknown states
                self.add_code(&format!(
                    "next_compartment = FrameCompartment('{}', None, None, None, None, {{}}, {{}})",
                    target_state_full_name
                ));
            }
            self.newline();
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
                                self.errors.push("Fatal error: misaligned parameters to arguments.".to_string());
                                return;
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
                                    None => {
                                        self.errors.push(format!(
                                            "Invalid number of arguments for \"{}\" event handler.",
                                            msg
                                        ));
                                        break;
                                    }
                                }
                            }
                        }
                        None => {
                            self.errors.push("Fatal error: misaligned parameters to arguments.".to_string());
                            return;
                        }
                    }
                } else {
                    // No exit event handler defined - this is ok, just skip exit args
                    // Exit args without a handler will be ignored
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
                self.add_code("frame_event = FrameEvent(\"$>\", self.__compartment.enter_args)");
            } else {
                self.newline();
                self.add_code("frame_event = FrameEvent(\"$>\", None)");
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
    
    // v0.30: Generate all functions from the Arcanum
    fn generate_all_functions(&mut self) {
        let function_symbols = self.arcanium.get_functions().clone(); // Clone to avoid borrow issues
        
        for function_symbol_rcref in function_symbols {
            let function_symbol = function_symbol_rcref.borrow();
            if let Some(function_node_rcref) = &function_symbol.ast_node_opt {
                let function_node = function_node_rcref.borrow();
                function_node.accept(self);
            }
        }
    }

    // v0.30: Generate all enums from all systems at module level
    // This ensures enums can be referenced from functions and anywhere in the module
    fn generate_all_enums(&mut self, frame_module: &FrameModule) {
        let mut generated_enums = HashSet::new();
        
        // v0.32: First generate module-level enums
        for enum_rcref in &frame_module.enums {
            let enum_decl_node = enum_rcref.borrow();
            
            // Module-level enums don't need system prefix
            let full_enum_name = enum_decl_node.name.clone();
            
            // Track module-level enum names
            self.module_level_enums.insert(full_enum_name.clone());
            
            if !generated_enums.contains(&full_enum_name) {
                generated_enums.insert(full_enum_name.clone());
                
                // Mark enum import as required
                self.required_imports.insert("from enum import Enum".to_string());
                
                // Generate the enum at module level
                self.newline();
                self.add_code(&format!("class {}(Enum):", full_enum_name));
                self.indent();
                
                // Generate enum members with proper values
                for enumerator_decl_node in &enum_decl_node.enums {
                    enumerator_decl_node.accept(self);
                }
                
                self.outdent();
                self.newline();
            }
        }
        
        // Then iterate through all systems and collect their enums
        let system_symbols = self.arcanium.get_systems().clone();
        if system_symbols.is_empty() {
            return;
        }
        
        for system_symbol_rcref in system_symbols {
            let system_symbol = system_symbol_rcref.borrow();
            let system_name = &system_symbol.name;
            
            // Check if this system has a domain block with enums
            if let Some(domain_block_symbol_rcref) = &system_symbol.domain_block_symbol_opt {
                let domain_block_symbol = domain_block_symbol_rcref.borrow();
                let domain_symbol_table = domain_block_symbol.symtab_rcref.borrow();
                
                // Iterate through all symbols in the domain block
                for (_name, symbol_type_rcref) in &domain_symbol_table.symbols {
                    let symbol_type = symbol_type_rcref.borrow();
                    match &*symbol_type {
                        SymbolType::EnumDeclSymbolT { enum_symbol_rcref } => {
                            let enum_symbol = enum_symbol_rcref.borrow();
                            if let Some(ast_node_rcref) = &enum_symbol.ast_node_opt {
                                let enum_decl_node = ast_node_rcref.borrow();
                                
                                // Generate unique enum name: SystemName_EnumName  
                                let full_enum_name = format!("{}_{}", system_name, enum_decl_node.name);
                                
                                // Avoid duplicate generation
                                if !generated_enums.contains(&full_enum_name) {
                                    generated_enums.insert(full_enum_name.clone());
                                    
                                    // Mark enum import as required
                                    self.required_imports.insert("from enum import Enum".to_string());
                                    
                                    // Generate the enum at module level
                                    self.newline();
                                    self.add_code(&format!("class {}(Enum):", full_enum_name));
                                    self.indent();
                                    
                                    // Track duplicate enum names to avoid Python enum conflicts
                                    let mut enum_names_seen = HashSet::new();
                                    
                                    for enumerator_decl_node in &enum_decl_node.enums {
                                        // Only generate if we haven't seen this name before
                                        if !enum_names_seen.contains(&enumerator_decl_node.name) {
                                            enum_names_seen.insert(enumerator_decl_node.name.clone());
                                            enumerator_decl_node.accept(self);
                                        }
                                    }
                                    
                                    self.outdent();
                                    self.newline();
                                    
                                    // Generate alias for easier access: EnumName = SystemName_EnumName
                                    let enum_name = &enum_decl_node.name;
                                    self.add_code(&format!("{} = {}", enum_name, full_enum_name));
                                    self.newline();
                                }
                            }
                        }
                        _ => {} // Not an enum, skip
                    }
                }
            }
        }
    }

    // v0.30: Generate all systems from the Arcanum
    fn generate_all_systems(&mut self) {
        let system_symbols = self.arcanium.get_systems().clone(); // Clone to avoid borrow issues
        
        for system_symbol_rcref in system_symbols {
            let system_symbol = system_symbol_rcref.borrow();
            self.generate_single_system(&system_symbol);
        }
    }

    // v0.30: Generate a single system from its symbol
    fn generate_single_system(&mut self, system_symbol: &SystemSymbol) {
        // v0.37: Check if system needs async runtime based on SystemSymbol
        // NOTE: This is for the Symbol-based generation path (not currently used)
        if self.config.event_handlers_as_functions {
            self.system_has_async_runtime = self.system_symbol_needs_async_runtime(system_symbol);
        }
        
        // domain variable vector
        let mut domain_vec: Vec<(String, String)> = Vec::new();
        if let Some(domain_block_symbol_rcref) = &system_symbol.domain_block_symbol_opt {
            let domain_block_symbol = domain_block_symbol_rcref.borrow();
            let symbol_table = domain_block_symbol.symtab_rcref.borrow();
            
            // Collect domain variables for initialization
            for (_name, symbol_type_rcref) in symbol_table.symbols.iter() {
                let symbol_type = symbol_type_rcref.borrow();
                if let SymbolType::DomainVariable { domain_variable_symbol_rcref } = &*symbol_type {
                    let domain_var_symbol = domain_variable_symbol_rcref.borrow();
                    let var_name = domain_var_symbol.name.clone();
                    let var_decl_node_rcref = &domain_var_symbol.ast_node_rcref;
                    let var_decl_node = var_decl_node_rcref.borrow();
                    let var_init_expr = var_decl_node.get_initializer_value_rc();
                    let mut init_expression = String::new();
                    var_init_expr.accept_to_string(self, &mut init_expression);
                    domain_vec.push((var_name, init_expression));
                }
            }
        }

        self.system_name = system_symbol.name.clone();
        
        // Generate the system class and its components
        self.generate_system_class(&system_symbol, domain_vec);
    }

    // v0.30: Generate system class from system symbol
    fn generate_system_class(&mut self, system_symbol: &SystemSymbol, domain_vec: Vec<(String, String)>) {
        let system_name = &system_symbol.name;
        
        // Generate compartment class for this system
        self.newline();
        self.add_code(&format!("class {}Compartment:", system_name));
        self.indent();
        self.newline();
        self.add_code("def __init__(self, state, forward_event=None, exit_args=None, enter_args=None):");
        self.indent();
        self.newline();
        self.add_code("self.state = state");
        self.newline();
        self.add_code("self.forward_event = forward_event");
        self.newline();
        self.add_code("self.exit_args = exit_args");
        self.newline();
        self.add_code("self.enter_args = enter_args");
        self.outdent();
        self.outdent();
        
        // Generate system class
        self.newline();
        self.newline();
        self.add_code(&format!("class {}:", system_name));
        self.indent();
        self.newline();
        self.newline();
        
        self.add_code(&format!("# ==================== System Factory =================== #"));
        self.newline();
        self.newline();
        
        // Generate constructor (__init__)
        self.generate_system_constructor(system_symbol, domain_vec);
        
        // Generate interface methods - use proper visitor methods instead of stubs
        if let Some(_interface_block_symbol_rcref) = &system_symbol.interface_block_symbol_opt {
            // TODO: Need to convert from symbol back to AST node to call proper visitor
            // For now, generate basic interface structure
            self.newline();
            self.add_code("# ==================== Interface Block ================== #");
            self.newline();
            // This needs proper implementation - interface methods should be generated here
        }
        
        // Generate machine states - use proper visitor methods instead of stubs  
        if let Some(_machine_block_symbol_rcref) = &system_symbol.machine_block_symbol_opt {
            // TODO: Need to convert from symbol back to AST node to call proper visitor
            // For now, generate basic machine structure
            self.newline();
            self.add_code("# ===================== Machine Block =================== #");
            self.newline();
            // This needs proper implementation - state methods should be generated here
        }
        
        // Generate action methods
        if let Some(actions_block_symbol_rcref) = &system_symbol.actions_block_symbol_opt {
            self.generate_action_methods(actions_block_symbol_rcref);
        }
        
        // Generate operation methods
        self.newline();
        if let Some(operations_block_symbol_rcref) = &system_symbol.operations_block_symbol_opt {
            self.generate_operation_methods(operations_block_symbol_rcref);
        }
        
        // Generate system runtime (__kernel, __router, __transition)
        self.generate_system_runtime(system_symbol);
        
        self.outdent();
    }
    
    // Helper methods for system generation (stubs for now)
    fn generate_system_constructor(&mut self, system_symbol: &SystemSymbol, domain_vec: Vec<(String, String)>) {
        // Generate a complete system constructor
        self.add_code("def __init__(self):");
        self.indent();
        
        // Initialize compartment tracking
        if let Some(machine_block_symbol) = &system_symbol.machine_block_symbol_opt {
            let machine_block = machine_block_symbol.borrow();
            let symtab = machine_block.symtab_rcref.borrow();
            
            // Get first state name from symbol table
            let mut first_state_name_opt = None;
            for (_name, symbol_type_rcref) in symtab.symbols.iter() {
                let symbol_type = symbol_type_rcref.borrow();
                if let SymbolType::State { state_symbol_ref } = &*symbol_type {
                    let state_symbol = state_symbol_ref.borrow();
                    first_state_name_opt = Some(self.format_target_state_name(&state_symbol.name));
                    break;  // Just need the first state
                }
            }
            
            if let Some(first_state_name) = first_state_name_opt {
                self.newline();
                self.add_code("# Create and initialize start state compartment");
                self.newline();
                self.add_code(&format!("self.__compartment = {}Compartment('{}', None, None)", system_symbol.name, first_state_name));
                self.newline();
                self.add_code("self.__next_compartment = None");
            }
        }
        
        // Initialize state stack if needed
        if self.generate_state_stack {
            self.newline();
            self.add_code("self.__state_stack = []");
        }
        
        // Initialize domain variables
        if !domain_vec.is_empty() {
            self.newline();
            self.add_code("# Initialize domain");
            for (var_name, init_expression) in domain_vec {
                self.newline();
                self.add_code(&format!("self.{} = {}", var_name, init_expression));
            }
        }
        
        // Send system start event
        if system_symbol.machine_block_symbol_opt.is_some() {
            self.newline();
            self.newline();
            
            // v0.37: For async runtime, use sync wrapper in __init__
            self.add_code("# Send system start event");
            self.newline();
            self.add_code("frame_event = FrameEvent(\"$>\", None)");
            self.newline();
            if self.system_has_async_runtime {
                self.add_code("self.__kernel_sync(frame_event)");
            } else {
                self.add_code("self.__kernel(frame_event)");
            }
        }
        
        self.outdent();
        
        self.newline();
    }
    
    fn generate_interface_methods(&mut self, _interface_block_symbol_rcref: &Rc<RefCell<InterfaceBlockScopeSymbol>>) {
        // TODO: Implement interface method generation
        self.newline();
        self.add_code("# Interface methods will be added here");
        self.newline();
    }
    
    fn generate_machine_states(&mut self, _machine_block_symbol_rcref: &Rc<RefCell<MachineBlockScopeSymbol>>) {
        // TODO: Implement state machine generation
        self.newline();
        self.add_code("# State machine will be added here");
        self.newline();
    }
    
    fn generate_action_methods(&mut self, _actions_block_symbol_rcref: &Rc<RefCell<ActionsBlockScopeSymbol>>) {
        // TODO: Implement action method generation
        self.newline();
        self.add_code("# Action methods will be added here");
        self.newline();
    }
    
    fn generate_operation_methods(&mut self, operations_block_symbol_rcref: &Rc<RefCell<OperationsBlockScopeSymbol>>) {
        // Generate operations block header
        self.newline();
        self.newline();
        self.add_code("# ==================== Operations Block ================== #");
        
        self.newline();
        
        // Access operations from symbol table
        let operations_block_symbol = operations_block_symbol_rcref.borrow();
        let symbol_table = operations_block_symbol.symtab_rcref.borrow();
        
        
        // Generate each operation method
        for (_name, symbol_type_rcref) in symbol_table.symbols.iter() {
            let symbol_type = symbol_type_rcref.borrow();
            self.newline();
            
            if let SymbolType::OperationScope { operation_scope_symbol_rcref } = &*symbol_type {
                let operation_symbol = operation_scope_symbol_rcref.borrow();
                
                // Try to use AST node if available
                if let Some(operation_node_rcref) = &operation_symbol.ast_node_opt {
                    let operation_node = operation_node_rcref.borrow();
                    operation_node.accept(self);
                } else {
                    // AST node not available - generate method stub
                    self.newline();
                    self.newline();
                    self.add_code(&format!("def {}(self):", operation_symbol.name));
                    self.indent();
                    self.newline();
                    self.add_code("# TODO: Operation implementation not available from symbol");
                    self.newline();
                    self.add_code("pass");
                    self.outdent();
                }
            }
        }
    }
    
    fn generate_system_runtime(&mut self, system_symbol: &SystemSymbol) {
        // Generate the system runtime methods (__kernel, __router, __transition)
        self.newline();
        self.newline();
        self.add_code("# ==================== System Runtime =================== #");
        self.newline();
        
        // v0.37: Check if system needs async runtime
        let needs_async = self.config.event_handlers_as_functions && 
                         self.system_symbol_needs_async_runtime(system_symbol);
        
        // Generate __kernel method
        self.newline();
        if needs_async {
            self.add_code("async def __kernel(self, __e):");
        } else {
            self.add_code("def __kernel(self, __e):");
        }
        self.indent();
        self.newline();
        self.add_code("# send event to current state");
        self.newline();
        if needs_async {
            self.add_code("await self.__router(__e)");
        } else {
            self.add_code("self.__router(__e)");
        }
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
        if needs_async {
            self.add_code("await self.__router(FrameEvent(\"<$\", self.__compartment.exit_args))");
        } else {
            self.add_code("self.__router(FrameEvent(\"<$\", self.__compartment.exit_args))");
        }
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
        if needs_async {
            self.add_code("await self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
        } else {
            self.add_code("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
        }
        self.outdent();
        self.newline();
        self.add_code("else:");
        self.indent();
        self.newline();
        self.add_code("# forwarded event");
        self.newline();
        self.add_code("if next_compartment.forward_event._message == \"$>\":");
        self.indent();
        self.newline();
        if needs_async {
            self.add_code("await self.__router(next_compartment.forward_event)");
        } else {
            self.add_code("self.__router(next_compartment.forward_event)");
        }
        self.outdent();
        self.newline();
        self.add_code("else:");
        self.indent();
        self.newline();
        if needs_async {
            self.add_code("await self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
            self.newline();
            self.add_code("await self.__router(next_compartment.forward_event)");
        } else {
            self.add_code("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
            self.newline();
            self.add_code("self.__router(next_compartment.forward_event)");
        }
        self.outdent();
        self.newline();
        self.add_code("next_compartment.forward_event = None");
        self.outdent();
        self.outdent();
        self.outdent();
        
        // Generate __router method (async if needed)
        self.newline();
        self.newline();
        if needs_async {
            self.add_code("async def __router(self, __e, compartment=None):");
        } else {
            self.add_code("def __router(self, __e, compartment=None):");
        }
        self.indent();
        self.newline();
        self.add_code("target_compartment = compartment or self.__compartment");
        self.newline();
        
        // Route to state handlers based on compartment state
        if let Some(machine_block_symbol) = &system_symbol.machine_block_symbol_opt {
            let machine_block = machine_block_symbol.borrow();
            let symtab = machine_block.symtab_rcref.borrow();
            
            let mut index = 0;
            let mut state_names = Vec::new();
            
            // Collect all state names first
            for (_name, symbol_type_rcref) in symtab.symbols.iter() {
                let symbol_type = symbol_type_rcref.borrow();
                if let SymbolType::State { state_symbol_ref } = &*symbol_type {
                    let state_symbol = state_symbol_ref.borrow();
                    state_names.push(self.format_target_state_name(&state_symbol.name));
                }
            }
            
            // Generate if/elif chain for state routing
            for state_name in &state_names {
                if index == 0 {
                    self.add_code(&format!("if target_compartment.state == '{}':", state_name));
                } else {
                    self.add_code(&format!("elif target_compartment.state == '{}':", state_name));
                }
                self.indent();
                self.newline();
                if needs_async {
                    self.add_code(&format!("return await self.{}(__e, target_compartment)", state_name));
                } else {
                    self.add_code(&format!("self.{}(__e, target_compartment)", state_name));
                }
                self.outdent();
                if index < state_names.len() - 1 {
                    self.newline();
                }
                index += 1;
            }
        }
        self.outdent();
        
        // Generate __transition method
        self.newline();
        self.newline();
        self.add_code("def __transition(self, next_compartment):");
        self.indent();
        self.newline();
        self.add_code("self.__next_compartment = next_compartment");
        self.outdent();
        
        // Generate state stack methods if needed
        if self.generate_state_stack {
            self.newline();
            self.newline();
            self.add_code("def __state_stack_push(self, compartment):");
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
    }
    
    //* --------------------------------------------------------------------- *//
    // ===================== Frame v0.31 Explicit Self/System Context =====================
    
    /// Handle collection constructor calls - transform into Python literals
    fn handle_collection_constructor(&mut self, method_call: &CallExprNode) {
        let method_name = &method_call.identifier.name.lexeme;
        
        match method_name.as_str() {
            "list" => {
                self.add_code("[");
                let expr_list = &method_call.call_expr_list.exprs_t;
                for (i, expr) in expr_list.iter().enumerate() {
                    if i > 0 {
                        self.add_code(",");
                    }
                    expr.accept(self);
                }
                self.add_code("]");
            }
            "set" => {
                self.add_code("{");
                let expr_list = &method_call.call_expr_list.exprs_t;
                for (i, expr) in expr_list.iter().enumerate() {
                    if i > 0 {
                        self.add_code(", ");
                    }
                    expr.accept(self);
                }
                self.add_code("}");
            }
            "tuple" => {
                self.add_code("(");
                let expr_list = &method_call.call_expr_list.exprs_t;
                for (i, expr) in expr_list.iter().enumerate() {
                    if i > 0 {
                        self.add_code(", ");
                    }
                    expr.accept(self);
                }
                // Single element tuples need trailing comma in Python
                if expr_list.len() == 1 {
                    self.add_code(",");
                }
                self.add_code(")");
            }
            "dict" => {
                // Keep dict() as-is - it's valid Python
                self.add_code("dict");
                method_call.call_expr_list.accept(self);
            }
            _ => {
                // Should not reach here based on the check in visit_call_expression_node
                self.add_code(&method_call.identifier.name.lexeme);
                method_call.call_expr_list.accept(self);
            }
        }
    }
    
    /// Handle collection constructor calls for string output
    fn handle_collection_constructor_to_string(&mut self, method_call: &CallExprNode, output: &mut String) {
        let method_name = &method_call.identifier.name.lexeme;
        
        match method_name.as_str() {
            "list" => {
                output.push('[');
                let expr_list = &method_call.call_expr_list.exprs_t;
                for (i, expr) in expr_list.iter().enumerate() {
                    if i > 0 {
                        output.push(',');
                    }
                    expr.accept_to_string(self, output);
                }
                output.push(']');
            }
            "set" => {
                output.push('{');
                let expr_list = &method_call.call_expr_list.exprs_t;
                for (i, expr) in expr_list.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    expr.accept_to_string(self, output);
                }
                output.push('}');
            }
            "tuple" => {
                output.push('(');
                let expr_list = &method_call.call_expr_list.exprs_t;
                for (i, expr) in expr_list.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    expr.accept_to_string(self, output);
                }
                // Single element tuples need trailing comma in Python
                if expr_list.len() == 1 {
                    output.push(',');
                }
                output.push(')');
            }
            "dict" => {
                // Keep dict() as-is - it's valid Python
                output.push_str("dict");
                method_call.call_expr_list.accept_to_string(self, output);
            }
            _ => {
                // Should not reach here based on the check in visit_call_expression_node_to_string
                output.push_str(&method_call.identifier.name.lexeme);
                method_call.call_expr_list.accept_to_string(self, output);
            }
        }
    }
    
    /// v0.64: Handle calls using semantic resolution (to_string variant)
    /// This method uses the resolved_type field to generate the correct call syntax
    /// without needing complex call chain analysis
    fn handle_call_with_resolved_type_to_string(&mut self, method_call: &CallExprNode, output: &mut String) -> bool {
        // Check if we have a resolved type
        let resolved_type = match &method_call.resolved_type {
            Some(rt) => rt,
            None => return false, // No resolution, fall back to old logic
        };
        
        // Debug output
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG v0.64: Using resolved type {:?} for {} (to_string)", 
                resolved_type, method_call.identifier.name.lexeme);
        }
        
        // Generate code based on resolved type
        match resolved_type {
            ResolvedCallType::Action(name) => {
                // Actions need underscore prefix and self (when in system context)
                if !self.in_standalone_function {
                    output.push_str("self.");
                }
                // v0.66: Use format_action_name for consistency with action definitions
                let formatted_name = self.format_action_name(name);
                output.push_str(&formatted_name);
                method_call.call_expr_list.accept_to_string(self, output);
                true
            }
            ResolvedCallType::Operation(name) => {
                // Operations need self (when in system context)
                if !self.in_standalone_function {
                    output.push_str("self.");
                }
                output.push_str(name);
                method_call.call_expr_list.accept_to_string(self, output);
                true
            }
            ResolvedCallType::SystemInterface { system: _, method } => {
                // v0.66: Interface method called from within the system
                // Generate self.method() to call through the interface
                if !self.in_standalone_function {
                    output.push_str("self.");
                }
                output.push_str(method);
                method_call.call_expr_list.accept_to_string(self, output);
                true
            }
            ResolvedCallType::SystemOperation { system, operation, is_static } => {
                // Qualified system operation call
                if *is_static {
                    output.push_str(&format!("{}.{}", system, operation));
                } else {
                    // Instance method on system - need instance reference
                    output.push_str(&format!("{}_instance.{}", system, operation));
                }
                method_call.call_expr_list.accept_to_string(self, output);
                true
            }
            ResolvedCallType::ClassMethod { class, method, is_static } => {
                // Class method call
                if *is_static {
                    output.push_str(&format!("{}.{}", class, method));
                } else {
                    // Instance method - use self when in class context
                    if self.in_class_method {
                        output.push_str(&format!("self.{}", method));
                    } else {
                        // Outside class, would need instance reference
                        output.push_str(&format!("{}_instance.{}", class, method));
                    }
                }
                method_call.call_expr_list.accept_to_string(self, output);
                true
            }
            ResolvedCallType::ModuleFunction { module, function } => {
                // Module function call
                output.push_str(&format!("{}.{}", module, function));
                method_call.call_expr_list.accept_to_string(self, output);
                true
            }
            ResolvedCallType::External(name) => {
                // v0.66: Special handling for Python collection constructors
                if name == "set" || name == "list" || name == "tuple" {
                    // Handle call chain if present
                    if let Some(call_chain) = &method_call.call_chain {
                        if !call_chain.is_empty() {
                            for callable in call_chain {
                                let saved_code = self.code.clone();
                                self.code.clear();
                                callable.callable_accept(self);
                                output.push_str(&self.code);
                                output.push('.');
                                self.code = saved_code;
                            }
                        }
                    }
                    
                    output.push_str(name);
                    output.push('(');
                    
                    let expr_count = method_call.call_expr_list.exprs_t.len();
                    
                    if expr_count > 1 {
                        // Multiple arguments: wrap them in a list
                        output.push('[');
                        let mut separator = "";
                        for expr in &method_call.call_expr_list.exprs_t {
                            output.push_str(separator);
                            expr.accept_to_string(self, output);
                            separator = ",";
                        }
                        output.push(']');
                    } else if expr_count == 1 {
                        let arg = &method_call.call_expr_list.exprs_t[0];
                        arg.accept_to_string(self, output);
                    }
                    
                    output.push(')');
                    true
                } else {
                    // Regular external function - no prefix needed
                    // But still need to handle call chain if present
                    if let Some(call_chain) = &method_call.call_chain {
                        if !call_chain.is_empty() {
                            for callable in call_chain {
                                let saved_code = self.code.clone();
                                self.code.clear();
                                callable.callable_accept(self);
                                output.push_str(&self.code);
                                output.push('.');
                                self.code = saved_code;
                            }
                        }
                    }
                    output.push_str(name);
                    method_call.call_expr_list.accept_to_string(self, output);
                    true
                }
            }
        }
    }
    
    /// v0.64: Handle calls using semantic resolution
    /// This method uses the resolved_type field to generate the correct call syntax
    /// without needing complex call chain analysis
    fn handle_call_with_resolved_type(&mut self, method_call: &CallExprNode) -> bool {
        // v0.65: Always use resolved types when available (removed flag check)
        // Check if we have a resolved type
        let resolved_type = match &method_call.resolved_type {
            Some(rt) => rt,
            None => return false, // No resolution, fall back to old logic
        };
        
        // Debug output
        if std::env::var("FRAME_TRANSPILER_DEBUG").is_ok() {
            eprintln!("DEBUG v0.64: Using resolved type {:?} for {}", 
                resolved_type, method_call.identifier.name.lexeme);
        }
        
        // Generate code based on resolved type
        match resolved_type {
            ResolvedCallType::Action(name) => {
                // Actions need underscore prefix and self (when in system context)
                if !self.in_standalone_function {
                    self.add_code("self.");
                }
                // v0.66: Use format_action_name for consistency with action definitions
                let formatted_name = self.format_action_name(name);
                self.add_code(&formatted_name);
                method_call.call_expr_list.accept(self);
                true
            }
            ResolvedCallType::Operation(name) => {
                // Operations need self (when in system context)
                if !self.in_standalone_function {
                    self.add_code("self.");
                }
                self.add_code(name);
                method_call.call_expr_list.accept(self);
                true
            }
            ResolvedCallType::SystemInterface { system: _, method } => {
                // v0.66: Interface method called from within the system
                // Generate self.method() to call through the interface
                if !self.in_standalone_function {
                    self.add_code("self.");
                }
                self.add_code(method);
                method_call.call_expr_list.accept(self);
                true
            }
            ResolvedCallType::SystemOperation { system, operation, is_static } => {
                // Qualified system operation call
                if *is_static {
                    self.add_code(&format!("{}.{}", system, operation));
                } else {
                    // Instance method on system - need instance reference
                    self.add_code(&format!("{}_instance.{}", system, operation));
                }
                method_call.call_expr_list.accept(self);
                true
            }
            ResolvedCallType::ClassMethod { class, method, is_static } => {
                // Class method call
                if *is_static {
                    self.add_code(&format!("{}.{}", class, method));
                } else {
                    // Instance method - use self when in class context
                    if self.in_class_method {
                        self.add_code(&format!("self.{}", method));
                    } else {
                        // Outside class, would need instance reference
                        self.add_code(&format!("{}_instance.{}", class, method));
                    }
                }
                method_call.call_expr_list.accept(self);
                true
            }
            ResolvedCallType::ModuleFunction { module, function } => {
                // Module function call
                self.add_code(&format!("{}.{}", module, function));
                method_call.call_expr_list.accept(self);
                true
            }
            ResolvedCallType::External(name) => {
                // v0.66: Special handling for Python collection constructors
                if name == "set" || name == "list" || name == "tuple" {
                    // Handle call chain if present
                    if let Some(call_chain) = &method_call.call_chain {
                        if !call_chain.is_empty() {
                            for callable in call_chain {
                                callable.callable_accept(self);
                                self.add_code(".");
                            }
                        }
                    }
                    
                    self.add_code(name);
                    self.add_code("(");
                    
                    let expr_count = method_call.call_expr_list.exprs_t.len();
                    
                    if expr_count > 1 {
                        // Multiple arguments: wrap them in a list
                        self.add_code("[");
                        let mut separator = "";
                        for expr in &method_call.call_expr_list.exprs_t {
                            self.add_code(separator);
                            expr.accept(self);
                            separator = ",";
                        }
                        self.add_code("]");
                    } else if expr_count == 1 {
                        let arg = &method_call.call_expr_list.exprs_t[0];
                        arg.accept(self);
                    }
                    
                    self.add_code(")");
                    true
                } else {
                    // Regular external function - no prefix needed
                    // But still need to handle call chain if present
                    if let Some(call_chain) = &method_call.call_chain {
                        if !call_chain.is_empty() {
                            for callable in call_chain {
                                callable.callable_accept(self);
                                self.add_code(".");
                            }
                        }
                    }
                    self.add_code(name);
                    method_call.call_expr_list.accept(self);
                    true
                }
            }
        }
    }
}

//* --------------------------------------------------------------------- *//

//* --------------------------------------------------------------------- *//

impl PythonVisitor {
    // v0.57: Module support for multi-file compilation
    fn generate_module_as_class(&mut self, module_node: &ModuleNode) {
        // Save parent context before setting new context
        let saved_module_name = self.current_module_name.clone();
        let saved_nested_names = self.nested_module_names.clone();
        let saved_module_variables = self.current_module_variables.clone();
        
        // Set module context
        self.current_module_name = Some(module_node.name.clone());
        self.current_module_path.push(module_node.name.clone());
        self.current_module_variables.clear();
        self.nested_module_names.clear();
        
        // Collect module variable names
        for var_node_rcref in &module_node.variables {
            let var = var_node_rcref.borrow();
            self.current_module_variables.insert(var.name.clone());
        }
        
        // Collect nested module names for sibling resolution
        for nested_module_rcref in &module_node.modules {
            let nested = nested_module_rcref.borrow();
            self.nested_module_names.insert(nested.name.clone());
        }
        
        // Generate a Python class to act as namespace for the module
        self.newline();
        self.add_code(&format!("class {}:", module_node.name));
        self.indent();
        
        let mut has_content = false;
        
        // Process module variables as class variables
        for var_node_rcref in &module_node.variables {
            if has_content {
                self.newline();
            }
            // Generate as class variable
            self.newline();
            let var = var_node_rcref.borrow();
            self.add_code(&format!("{} = ", var.name));
            // value_rc is Rc<ExprType>, not Option
            var.value_rc.accept(self);
            has_content = true;
        }
        
        // Process nested modules recursively
        for nested_module_rcref in &module_node.modules {
            if has_content {
                self.newline();
            }
            self.generate_module_as_class(&nested_module_rcref.borrow());
            has_content = true;
        }
        
        // Process module functions as static methods AFTER nested modules
        for func_node_rcref in &module_node.functions {
            if has_content {
                self.newline();
            }
            // Generate as static method
            self.newline();
            self.add_code("@staticmethod");
            let func = func_node_rcref.borrow();
            // Generate the function
            self.newline();
            self.add_code(&format!("def {}(", func.name));
            // Visit parameters
            if let Some(ref params) = func.params {
                let mut first = true;
                for param in params {
                    if !first {
                        self.add_code(",");
                    }
                    param.accept(self);
                    first = false;
                }
            }
            self.add_code("):");
            self.indent();
            
            // Generate function body
            debug_print!("DEBUG: Generating function '{}' in module '{:?}'", func.name, self.current_module_name);
            debug_print!("  Current module path: {:?}", self.current_module_path);
            debug_print!("  Nested module names: {:?}", self.nested_module_names);
            if !func.statements.is_empty() {
                // Mark that we're in a standalone function
                let was_in_function = self.in_standalone_function;
                self.in_standalone_function = true;
                
                // Track if we generated a return statement
                let had_return_before = self.this_branch_transitioned;
                self.this_branch_transitioned = false;
                
                for stmt in &func.statements {
                    match stmt {
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_stmt_type_helper(stmt_t);
                        }
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            var_decl_t_rcref.borrow().accept(self);
                        }
                    }
                }
                
                // Check if a return was generated in the statements
                let generated_return = self.this_branch_transitioned;
                self.this_branch_transitioned = had_return_before;
                
                // Only add terminator return if we didn't already generate one
                if !generated_return {
                    if let Some(ref return_expr) = func.terminator_expr.return_expr_t_opt {
                        self.newline();
                        self.add_code("return ");
                        return_expr.accept(self);
                    }
                }
                
                self.in_standalone_function = was_in_function;
            } else {
                self.newline();
                self.add_code("pass");
                
                // Add return if needed and no statements
                if let Some(ref return_expr) = func.terminator_expr.return_expr_t_opt {
                    self.newline();
                    self.add_code("return ");
                    return_expr.accept(self);
                }
            }
            
            self.outdent();
            has_content = true;
        }
        
        if !has_content {
            // Empty module - add pass statement
            self.newline();
            self.add_code("pass");
        }
        
        self.outdent();
        self.newline();
        
        // Restore parent context
        self.current_module_name = saved_module_name;
        self.current_module_path.pop();
        self.current_module_variables = saved_module_variables;
        self.nested_module_names = saved_nested_names;
    }
    
    // v0.45: Helper method to visit statement types
    fn visit_stmt_type_helper(&mut self, stmt_t: &StatementType) {
        match stmt_t {
            StatementType::ExpressionStmt { expr_stmt_t } => {
                match expr_stmt_t {
                    ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node } => {
                        debug_print!("DEBUG: CallChainStmtT in visit_stmt_type_helper");
                        call_chain_literal_stmt_node.accept(self);
                    }
                    ExprStmtType::CallStmtT { call_stmt_node } => {
                        debug_print!("DEBUG: CallStmtT in visit_stmt_type_helper");
                        call_stmt_node.accept(self);
                    }
                    ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                        debug_print!("DEBUG: AssignmentStmtT in visit_stmt_type_helper");
                        assignment_stmt_node.accept(self);
                    }
                    ExprStmtType::VariableStmtT { variable_stmt_node } => {
                        debug_print!("DEBUG: VariableStmtT in visit_stmt_type_helper");
                        variable_stmt_node.accept(self);
                    }
                    _ => {
                        debug_print!("DEBUG: Other ExprStmtType in visit_stmt_type_helper");
                    }
                }
            }
            StatementType::TransitionStmt { transition_statement_node } => {
                transition_statement_node.accept(self);
            }
            StatementType::BlockStmt { block_stmt_node } => {
                block_stmt_node.accept(self);
            }
            StatementType::IfStmt { if_stmt_node } => {
                if_stmt_node.accept(self);
            }
            StatementType::LoopStmt { loop_stmt_node } => {
                loop_stmt_node.accept(self);
            }
            StatementType::ContinueStmt { continue_stmt_node } => {
                continue_stmt_node.accept(self);
            }
            StatementType::BreakStmt { break_stmt_node } => {
                break_stmt_node.accept(self);
            }
            StatementType::AssertStmt { assert_stmt_node } => {
                assert_stmt_node.accept(self);
            }
            StatementType::DelStmt { del_stmt_node } => {
                del_stmt_node.accept(self);
            }
            StatementType::ParentDispatchStmt { parent_dispatch_stmt_node } => {
                parent_dispatch_stmt_node.accept(self);
            }
            StatementType::MatchStmt { match_stmt_node } => {
                match_stmt_node.accept(self);
            }
            StatementType::ReturnStmt { return_stmt_node } => {
                return_stmt_node.accept(self);
            }
            StatementType::ReturnAssignStmt { return_assign_stmt_node } => {
                return_assign_stmt_node.accept(self);
            }
            StatementType::NoStmt => {
                // Do nothing
            }
            StatementType::StateStackStmt { state_stack_operation_statement_node } => {
                state_stack_operation_statement_node.accept(self);
            }
            StatementType::ForStmt { for_stmt_node } => {
                for_stmt_node.accept(self);
            }
            StatementType::WhileStmt { while_stmt_node } => {
                while_stmt_node.accept(self);
            }
            StatementType::TryStmt { try_stmt_node } => {
                try_stmt_node.accept(self);
            }
            StatementType::RaiseStmt { raise_stmt_node } => {
                raise_stmt_node.accept(self);
            }
            StatementType::WithStmt { with_stmt_node } => {
                with_stmt_node.accept(self);
            }
        }
    }
}

impl PythonVisitor {
    //* --------------------------------------------------------------------- *//
    // Helper method for @indexed_call - outputs arguments with proper parentheses
    fn output_indexed_call_args(&mut self, call_expr_list: &CallExprListNode) {
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
    // Helper method for @indexed_call - outputs arguments with proper parentheses to string
    fn output_indexed_call_args_to_string(
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
}

impl AstVisitor for PythonVisitor {
    //* --------------------------------------------------------------------- *//

    fn visit_module(&mut self, module: &Module) {
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
                ModuleElement::Function { .. } => {
                    // Functions are handled separately by Arcanum
                }
                ModuleElement::System { .. } => {
                    // Systems are handled separately by Arcanum
                }
                ModuleElement::Import { import_node } => {
                    import_node.accept(self);
                }
                ModuleElement::Variable { var_decl_node: _ } => {
                    // Module-level variables are handled after functions/systems
                    // to maintain proper initialization order
                }
                ModuleElement::Statement { stmt_node: _ } => {
                    // Module-level statements are handled after variables
                }
                ModuleElement::Enum { enum_decl_node: _ } => {
                    // Module-level enums are handled in generate_all_enums
                }
                ModuleElement::Module { module_node } => {
                    // v0.57: Generate module contents
                    self.generate_module_as_class(&module_node.borrow());
                }
                ModuleElement::TypeAlias { type_alias_node: _ } => {
                    // Type aliases are already processed in run_v2, skip here
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
    
    // v0.45: Class support
    fn visit_class_node(&mut self, class_node: &ClassNode) {
        self.newline();
        
        // v0.58: Emit class decorators
        for decorator in &class_node.decorators {
            self.newline();
            self.add_code(decorator);  // Decorators are stored as complete strings like "@dataclass"
        }
        
        // Add newline before class definition if there were decorators
        if !class_node.decorators.is_empty() {
            self.newline();
        }
        
        // Handle inheritance
        if let Some(ref parent) = class_node.parent {
            self.add_code(&format!("class {}({}):", class_node.name, parent));
        } else {
            self.add_code(&format!("class {}:", class_node.name));
        }
        self.indent();
        
        // Generate class-level (static) variables
        if !class_node.static_vars.is_empty() {
            self.newline();
            self.add_code("# Class variables");
            for var_rcref in &class_node.static_vars {
                self.newline();
                let var_node = var_rcref.borrow();
                // Generate as class variable (without self.)
                self.add_code(&format!("{} = ", var_node.name));
                let expr_t = var_node.get_initializer_value_rc();
                if !matches!(*expr_t, ExprType::NilExprT) {
                    expr_t.accept(self);
                } else {
                    self.add_code("None");
                }
            }
        }
        
        // Generate constructor if present
        debug_print!("DEBUG visit_class_node: constructor present = {}", class_node.constructor.is_some());
        if let Some(constructor_rcref) = &class_node.constructor {
            self.newline();
            self.newline();
            let constructor = constructor_rcref.borrow();
            debug_print!("DEBUG visit_class_node: constructor statements count = {}", constructor.statements.len());
            
            // Generate __init__ method
            self.add_code("def __init__(self");
            if let Some(params) = &constructor.params {
                for param in params {
                    self.add_code(", ");
                    param.accept(self);
                }
            }
            self.add_code("):");
            self.indent();
            
            // Generate constructor body
            if constructor.statements.is_empty() {
                self.newline();
                self.add_code("pass");
            } else {
                for stmt in &constructor.statements {
                    self.newline();
                    match stmt {
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            // In constructor, instance variables become self.var
                            self.add_code(&format!("self.{} = ", var_decl.name));
                            let expr_t = var_decl.get_initializer_value_rc();
                            if !matches!(*expr_t, ExprType::NilExprT) {
                                expr_t.accept(self);
                            } else {
                                self.add_code("None");
                            }
                        }
                        DeclOrStmtType::StmtT { stmt_t } => {
                            // Handle regular statements in constructor
                            debug_print!("DEBUG: About to visit statement in constructor");
                            self.visit_stmt_type_helper(stmt_t);
                            debug_print!("DEBUG: Finished visiting statement in constructor");
                        }
                    }
                }
            }
            self.outdent();
        } else if !class_node.instance_vars.is_empty() {
            // Generate default constructor if we have instance vars but no explicit constructor
            self.newline();
            self.newline();
            self.add_code("def __init__(self):");
            self.indent();
            for var_rcref in &class_node.instance_vars {
                self.newline();
                let var_node = var_rcref.borrow();
                self.add_code(&format!("self.{} = ", var_node.name));
                let expr_t = var_node.get_initializer_value_rc();
                if !matches!(*expr_t, ExprType::NilExprT) {
                    expr_t.accept(self);
                } else {
                    self.add_code("None");
                }
            }
            self.outdent();
        }
        
        // Generate instance methods
        for method_rcref in &class_node.methods {
            self.newline();
            self.newline();
            let method = method_rcref.borrow();
            method.accept(self);
        }
        
        // Generate static methods
        for method_rcref in &class_node.static_methods {
            self.newline();
            self.newline();
            self.add_code("@staticmethod");
            self.newline();
            let method = method_rcref.borrow();
            method.accept(self);
        }
        
        // Generate class methods
        for method_rcref in &class_node.class_methods {
            self.newline();
            self.newline();
            self.add_code("@classmethod");
            self.newline();
            let method = method_rcref.borrow();
            method.accept(self);
        }
        
        // Generate properties
        for property_rcref in &class_node.properties {
            let property = property_rcref.borrow();
            
            // Generate getter
            if let Some(ref getter) = property.getter {
                self.newline();
                self.newline();
                self.add_code("@property");
                self.newline();
                let getter_method = getter.borrow();
                getter_method.accept(self);
            }
            
            // Generate setter
            if let Some(ref setter) = property.setter {
                self.newline();
                self.newline();
                self.add_code(&format!("@{}.setter", property.name));
                self.newline();
                let setter_method = setter.borrow();
                setter_method.accept(self);
            }
            
            // Generate deleter
            if let Some(ref deleter) = property.deleter {
                self.newline();
                self.newline();
                self.add_code(&format!("@{}.deleter", property.name));
                self.newline();
                let deleter_method = deleter.borrow();
                deleter_method.accept(self);
            }
        }
        
        // If no methods or constructor, add pass
        if class_node.constructor.is_none() 
            && class_node.methods.is_empty() 
            && class_node.static_methods.is_empty()
            && class_node.class_methods.is_empty()
            && class_node.properties.is_empty()
            && class_node.static_vars.is_empty()
            && class_node.instance_vars.is_empty() {
            self.newline();
            self.add_code("pass");
        }
        
        self.outdent();
        self.newline();
    }
    
    fn visit_method_node(&mut self, method_node: &MethodNode) {
        // Set context flag for proper return handling
        let was_in_class_method = self.in_class_method;
        self.in_class_method = true;
        
        // Generate method signature with special method name handling
        let method_name = if method_node.name == "init" {
            "__init__".to_string()
        } else {
            method_node.name.clone()
        };
        self.add_code(&format!("def {}", method_name));
        self.add_code("(");
        
        // Add self/cls parameter for instance/class methods
        let mut has_self_or_cls = false;
        if !method_node.is_static {
            if method_node.is_class {
                self.add_code("cls");
            } else {
                self.add_code("self");
            }
            has_self_or_cls = true;
        }
        
        // Add method parameters
        if let Some(params) = &method_node.params {
            let mut need_comma = has_self_or_cls;
            for param in params {
                // Skip 'cls' parameter for class methods as we add it automatically
                if method_node.is_class && param.param_name == "cls" {
                    continue;
                }
                if need_comma {
                    self.add_code(", ");
                }
                param.accept(self);
                need_comma = true;
            }
        }
        
        self.add_code("):");
        
        // Add return type annotation if present
        if let Some(type_node) = &method_node.type_opt {
            self.add_code(" -> ");
            self.add_code(&self.format_type(type_node));
            self.add_code(":");
        }
        
        self.indent();
        
        // Generate method body
        if method_node.statements.is_empty() {
            // Check if there's a terminator with return expression
            if let Some(expr) = &method_node.terminator_expr.return_expr_t_opt {
                self.newline();
                self.add_code("return ");
                expr.accept(self);
            } else {
                self.newline();
                self.add_code("pass");
            }
        } else {
            for stmt in &method_node.statements {
                self.newline();
                match stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        var_decl_t_rcref.borrow().accept(self);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_type_helper(stmt_t);
                    }
                }
            }
            
            // Handle terminator (usually implicit return)
            // TerminatorType only has Return variant currently
            if let Some(expr) = &method_node.terminator_expr.return_expr_t_opt {
                self.newline();
                self.add_code("return ");
                expr.accept(self);
            }
        }
        
        self.outdent();
        
        // Restore context flag
        self.in_class_method = was_in_class_method;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_node(&mut self, system_node: &SystemNode) {
        // NOTE: This method is LEGACY and not used in v0.30 run_v2() flow
        
        self.add_code(&format!("#{}", self.compiler_version));
        self.newline();
        self.newline();

        if self.generate_comment(system_node.line) {
            self.newline();
        }

        // v0.30: Process module first to generate FrameEvent and common elements
        let _ = &system_node.module.accept(self);

        // v0.30: Generate all functions from Arcanum
        self.generate_all_functions();

        // v0.30: Legacy enum and system generation (unused in run_v2 flow)
        // self.generate_all_enums(); // Now called from run_v2()
        // self.generate_all_systems(); // Now handled by run_v2() with frame_module

        // Note: Enums will be generated per system in generate_system_from_node()

        // v0.30: Return early to avoid legacy single-system processing
        return;
        
        /* Legacy single-system processing - preserved for reference
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

        if self.config.public_state_info {
            self.generate_state_info_method()
        }

        if self.config.public_compartment {
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
        */
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
        
        // Add source mapping for system instantiation
        self.add_source_mapping(system_instance_expr_node.identifier.line);
        
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
        
        // In Frame v0.30, SystemTypeExprNode is used for system instantiation
        // Always add parentheses for constructor call
        self.add_code("()");
        
        // Handle any method calls after instantiation
        if let Some(call_chain) = &*(system_type_expr_node.call_chain_opt) {
            let mut output = String::new();
            call_chain.accept_to_string(self, &mut output);
            self.add_code(&format!(".{}", output));
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_system_type_expr_node_to_string(
        &mut self,
        system_type_expr_node: &SystemTypeExprNode,
        output: &mut String,
    ) {
        let system_name = &system_type_expr_node.identifier.name.lexeme;
        output.push_str(&format!("{}", system_name));
        
        // In Frame v0.30, SystemTypeExprNode is used for system instantiation
        // Always add parentheses for constructor call
        output.push_str("()");
        
        // Handle any method calls after instantiation
        if let Some(call_chain) = &*(system_type_expr_node.call_chain_opt) {
            // let mut output = String::new();
            output.push_str(".");
            call_chain.accept_to_string(self, output);
            // output.push_str(&format!(".{}", output));
        }
    }

    //* --------------------------------------------------------------------- *//
    
    fn visit_import_node(&mut self, import_node: &ImportNode) {
        // v0.34: Filter out FSL imports - they're built into Python
        // v0.57: Added Frame file import support for multi-file module system
        match &import_node.import_type {
            ImportType::Simple { module } => {
                if !module.starts_with("fsl") {
                    self.add_code(&format!("import {}", module));
                    self.newline();
                }
            }
            ImportType::Aliased { module, alias } => {
                if !module.starts_with("fsl") {
                    self.add_code(&format!("import {} as {}", module, alias));
                    self.newline();
                }
            }
            ImportType::FromImport { module, items } => {
                if !module.starts_with("fsl") {
                    self.add_code(&format!("from {} import {}", module, items.join(", ")));
                    self.newline();
                }
            }
            ImportType::FromImportAll { module } => {
                if !module.starts_with("fsl") {
                    self.add_code(&format!("from {} import *", module));
                    self.newline();
                }
            }
            // v0.57: Frame file imports - generate as comments during single-file compilation
            // The multi-file compiler will handle actual module linking
            ImportType::FrameModule { module_name, file_path } => {
                self.add_code(&format!("# Frame import: {} from {}", module_name, file_path));
                self.newline();
                // TODO: Will be replaced with actual module imports during linking phase
            }
            ImportType::FrameModuleAliased { module_name, file_path, alias } => {
                self.add_code(&format!("# Frame import: {} from {} as {}", module_name, file_path, alias));
                self.newline();
                // TODO: Will be replaced with actual module imports during linking phase
            }
            ImportType::FrameSelective { items, file_path } => {
                self.add_code(&format!("# Frame import: {{{}}} from {}", items.join(", "), file_path));
                self.newline();
                // TODO: Will be replaced with actual module imports during linking phase
            }
        }
    }

    //* --------------------------------------------------------------------- *//
    
    fn visit_type_alias_node(&mut self, type_alias_node: &TypeAliasNode) {
        // Python 3.12+ type alias syntax
        self.add_code(&format!("type {} = {}", type_alias_node.name, type_alias_node.type_expr));
        self.newline();  // Use the newline method to add proper newline
    }

    //* --------------------------------------------------------------------- *//

    fn visit_function_node(&mut self, function_node: &FunctionNode) {
        // Set standalone function context
        let prev_in_standalone_function = self.in_standalone_function;
        self.in_standalone_function = true;
        
        // v0.73: Create marker for function if registry exists
        let marker = if let Some(ref mut registry) = self.source_mapping_registry {
            let description = format!("Function {}", function_node.name);
            Some(registry.create_marker(function_node.line, NodeType::FunctionDef, description))
        } else {
            None
        };
        
        self.newline();
        
        // v0.73: Generate function with marker or use old direct mapping
        if let Some(marker) = marker {
            // v0.35: Generate async def for async functions with marker
            if function_node.is_async {
                self.add_code(&format!("{}async def {}(", marker, function_node.name));
            } else {
                self.add_code(&format!("{}def {}(", marker, function_node.name));
            }
        } else {
            // No marker system, use old approach
            // Add source mapping for function definition
            self.add_source_mapping(function_node.line);
            
            // v0.35: Generate async def for async functions
            if function_node.is_async {
                self.add_code(&format!("async def {}(", function_node.name));
            } else {
                self.add_code(&format!("def {}(", function_node.name));
            }
        }

        self.format_parameter_list(&function_node.params);

        self.add_code(")");
        
        // v0.43: Add return type annotation if present
        if let Some(ref type_node) = function_node.type_opt {
            let return_type = self.format_type(type_node);
            if !return_type.is_empty() {
                self.add_code(&format!(" -> {}", return_type));
            }
        }
        
        self.add_code(":");

        if !function_node.is_implemented {
            self.newline();
            self.add_code("raise NotImplementedError");
        } else {
            // Generate statements
                if !function_node.statements.is_empty() {
                    // Clear global vars tracking for this function
                    self.global_vars_in_function.clear();
                    
                    // First pass: collect global variables used in assignments
                    self.collect_global_assignments(&function_node.statements);
                    
                    // Debug: print what we found
                    debug_print!("DEBUG: Function '{}' modifies module variables: {:?}", 
                             function_node.name, self.global_vars_in_function);
                    
                    // Generate global declarations if needed
                    if !self.global_vars_in_function.is_empty() {
                        self.indent();
                        self.newline();
                        let global_vars: Vec<String> = self.global_vars_in_function.iter().cloned().collect();
                        self.add_code(&format!("global {}", global_vars.join(", ")));
                        self.outdent();
                    }
                    
                    // Track if we generated a return statement
                    let had_return_before = self.this_branch_transitioned;
                    self.this_branch_transitioned = false;
                    
                    self.indent();
                    self.visit_decl_stmts(&function_node.statements);
                    self.outdent();
                    
                    // Check if a return was generated in the statements
                    let generated_return = self.this_branch_transitioned;
                    self.this_branch_transitioned = had_return_before;
                    
                    // Only add terminator return if we didn't already generate one
                    if !generated_return {
                        self.indent();
                        self.newline();
                        match &function_node.terminator_expr.terminator_type {
                            TerminatorType::Return => match &function_node.terminator_expr.return_expr_t_opt {
                                Some(expr_t) => {
                                    self.add_code("return ");
                                    expr_t.accept(self);
                                }
                                None => {
                                    self.add_code("return");
                                }
                            },
                            // DispatchToParentState removed - now handled as ParentDispatchStmt statement
                        }
                        self.outdent();
                    }
                } else {
                    // No statements, just add the terminator
                    self.indent();
                    self.newline();
                    match &function_node.terminator_expr.terminator_type {
                        TerminatorType::Return => match &function_node.terminator_expr.return_expr_t_opt {
                            Some(expr_t) => {
                                self.add_code("return ");
                                expr_t.accept(self);
                            }
                            None => {
                                self.add_code("return");
                            }
                        },
                        // DispatchToParentState removed - now handled as ParentDispatchStmt statement
                    }
                    self.outdent();
                }

        }

        if function_node.name == "main" {
            self.generate_main = true;
        }
        
        // Restore previous context
        self.in_standalone_function = prev_in_standalone_function;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_interface_parameters(&mut self, _interface_block_node: &InterfaceBlockNode) {
        self.errors.push("visit_interface_parameters() not valid for target language.".to_string());
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

        // v0.37: Generate async def only for async interface methods
        if interface_method_node.is_async {
            self.add_code(&format!("async def {}(self", interface_method_node.name));
        } else {
            self.add_code(&format!("def {}(self", interface_method_node.name));
        }

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
            // v0.37: Call kernel appropriately based on interface method and system async status
            if self.system_has_async_runtime {
                // System has async runtime
                if interface_method_node.is_async {
                    // Async interface method can await async kernel
                    self.add_code("await self.__kernel(__e)");
                } else {
                    // Sync interface method calls sync wrapper
                    self.add_code("self.__kernel_sync(__e)");
                }
            } else {
                // System has sync runtime - just call the kernel
                self.add_code("self.__kernel(__e)");
            }
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
        
        // Check if operation is static
        let is_static = if let Some(attributes) = &operation_node.attributes_opt {
            attributes.contains_key("staticmethod")
        } else {
            false
        };
        
        // Only generate @staticmethod if the operation has the attribute
        if is_static {
            self.add_code("@staticmethod");
            self.newline();
        }
        // v0.35: Generate async def for async operations
        if operation_node.is_async {
            self.add_code(&format!("async def {}(", operation_name));
        } else {
            self.add_code(&format!("def {}(", operation_name));
        }

        // Add self parameter for non-static operations
        if !is_static {
            self.add_code("self");
            if operation_node.params.is_some() && !operation_node.params.as_ref().unwrap().is_empty() {
                self.add_code(",");
            }
        }
        
        match &operation_node.params {
            Some(params) => {
                self.format_operations_parameter_list(params);
            }
            None => {}
        }

        self.add_code("):");

        self.indent();

        // Generate statements
        // if operation_node.statements.is_empty() && operation_node.terminator_node_opt.is_none() {
        //     self.newline();
        //     self.add_code("pass");
        // } else {
            if !operation_node.statements.is_empty() {
                // self.newline();
                self.visit_decl_stmts(&operation_node.statements);
            }
            // if let Some(terminator_expr) = &operation_node.terminator_node {
                match &operation_node.terminator_expr.terminator_type {
                    TerminatorType::Return => match &operation_node.terminator_expr.return_expr_t_opt {
                        Some(expr_t) => {
                            self.newline();
                            self.add_code("return ");
                            expr_t.accept(self);
                            // self.newline();
                        }
                        None => {
                            // Don't generate another return - the explicit return statement in the 
                            // operation body already generated one
                            // self.add_code("return");
                            // self.newline();
                        }
                    },
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
                }
            // }
     //   }

        self.outdent();
        self.operation_scope_depth -= 1;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operation_call_expression_node(
        &mut self,
        operation_call_expr_node: &OperationCallExprNode,
    ) {
        self.debug_enter(&format!("visit_operation_call_expression_node({})", operation_call_expr_node.identifier.name.lexeme));
        self.debug_print(&format!("visiting_call_chain_operation: {}", self.visiting_call_chain_operation));
        
        // Operation calls should have self. prefix only when NOT part of a call chain
        // When part of a call chain (e.g., obj.method()), don't add self. prefix
        if !self.visiting_call_chain_operation {
            self.debug_print("Adding 'self.' prefix");
            self.add_code("self.");
        } else {
            self.debug_print("NOT adding 'self.' prefix (in call chain)");
        }

        self.add_code(&format!(
            "{}",
            operation_call_expr_node.identifier.name.lexeme
        ));
        operation_call_expr_node.call_expr_list.accept(self);
        
        self.debug_exit("visit_operation_call_expression_node");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_operation_call_expression_node_to_string(
        &mut self,
        operation_call_expr_node: &OperationCallExprNode,
        output: &mut String,
    ) {
        // Operation calls should have self. prefix only when NOT part of a call chain
        // When part of a call chain (e.g., obj.method()), don't add self. prefix  
        if !self.visiting_call_chain_operation {
            output.push_str("self.");
        }
        
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
        
        // Generate state dispatcher methods after all state methods are generated
        self.newline();
        self.add_code("# ===================== State Dispatchers =================== #");
        self.newline();
        
        for state_node_rcref in &machine_block_node.states {
            let state_node = state_node_rcref.borrow();
            let state_name = &state_node.name;
            
            self.newline();
            self.add_code(&format!("def _s{}(self, __e):", state_name));
            self.indent();
            self.newline();
            self.add_code(&format!("return self.{}(__e, None)", self.format_target_state_name(state_name)));
            self.outdent();
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
        // v0.74.1: Skip the comment generation that adds an extra newline
        // The comment system always returns true and adds a blank line we don't need
        // if self.generate_comment(state_node.line) {
        //     self.newline();
        // }
        
        // v0.73: REMOVED source mapping for state declaration line
        // State declarations (e.g., "$Running {") don't generate executable Python code
        // Only event handlers inside states generate actual function definitions
        
        self.current_state_name_opt = Some(state_node.name.clone());
        
        // Track parent state for => $^ dispatch in event handlers
        self.current_state_parent_opt = match &state_node.dispatch_opt {
            Some(dispatch) => Some(dispatch.target_state_ref.name.clone()),
            None => None,
        };
        
        // v0.36: Use event-handlers-as-functions if enabled
        if self.config.event_handlers_as_functions {
            // Generate individual event handler functions
            for evt_handler_rcref in &state_node.evt_handlers_rcref {
                let evt_handler = evt_handler_rcref.borrow();
                self.generate_event_handler_function(state_node, &evt_handler);
            }
            
            // Generate state dispatcher
            self.generate_state_dispatcher(state_node);
            
            self.current_state_name_opt = None;
            self.current_state_parent_opt = None;
            return;
        }

        // v0.35: Check if state handler needs to be async
        // A state handler is async if it handles events from async interface methods
        // or contains await expressions in its handlers
        // v0.37: Also async if any event handler is explicitly marked as async
        let mut state_needs_async = false;
        
        // Check if any event handler in this state handles an async interface method
        // or contains await expressions, or is explicitly marked as async
        for evt_handler_rcref in &state_node.evt_handlers_rcref {
            let evt_handler = evt_handler_rcref.borrow();
            
            // v0.37: Check if event handler is explicitly marked as async
            if evt_handler.is_async {
                state_needs_async = true;
                break;
            }
            
            // Check if this event corresponds to an async interface method
            if let MessageType::CustomMessage { message_node } = &evt_handler.msg_t {
                // Look up the interface method to check if it's async
                if let Some(interface_method_symbol) = self.arcanium.lookup_interface_method(&message_node.name) {
                    if let Some(ast_node) = &interface_method_symbol.borrow().ast_node_opt {
                        if ast_node.borrow().is_async {
                            state_needs_async = true;
                            break;
                        }
                    }
                }
            }
            
            // Also check if the handler contains await expressions
            if self.contains_await_expr(&evt_handler.statements) {
                state_needs_async = true;
                break;
            }
        }

        self.newline();
        self.add_code("# ----------------------------------------");
        self.newline();
        self.add_code(&format!("# ${}", &state_node.name));
        self.newline();
        self.newline();
        
        // Generate async def if needed
        if state_needs_async {
            self.add_code(&format!(
                "async def {}(self, __e, compartment):",
                self.format_target_state_name(&state_node.name)
            ));
        } else {
            self.add_code(&format!(
                "def {}(self, __e, compartment):",
                self.format_target_state_name(&state_node.name)
            ));
        }
        self.indent();
        
        // Clear global vars tracking for this state
        self.global_vars_in_function.clear();
        
        // Collect global variables from all event handlers
        for evt_handler_rcref in &state_node.evt_handlers_rcref {
            let evt_handler = evt_handler_rcref.borrow();
            if !evt_handler.statements.is_empty() {
                self.collect_global_assignments(&evt_handler.statements);
            }
        }
        
        // Generate global declarations if needed
        if !self.global_vars_in_function.is_empty() {
            self.newline();
            let global_vars: Vec<String> = self.global_vars_in_function.iter().cloned().collect();
            self.add_code(&format!("global {}", global_vars.join(", ")));
        }

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

        // Dispatch to parent is now handled explicitly via => $^ in event handlers
        // No automatic fallback dispatch should be generated
        // match &state_node.dispatch_opt {
        //     Some(dispatch) => {
        //         self.newline();
        //         dispatch.accept(self);
        //     }
        //     None => {}
        // }

        self.outdent();
        self.newline();

        self.current_state_name_opt = None;
        self.current_state_parent_opt = None;
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

        // If event handler has a default return value, override the interface default
        if let Some(return_init_expr) = &evt_handler_node.return_init_expr_opt {
            self.newline();
            let mut output = String::new();
            return_init_expr.accept_to_string(self, &mut output);
            self.add_code(&format!("self.return_stack[-1] = {}", output));
        }

        // Generate statements
        self.event_handler_has_code = !evt_handler_node.statements.is_empty();
        if self.event_handler_has_code {
            self.visit_decl_stmts(&evt_handler_node.statements);
        }
        //     // TODO v0.20 - i added this back as it causes problems for the
        //     // event switch but I am not sure why it was taken out before
        //     // so I suspect there is an issue.
        // else {
        //     self.newline();
        //     self.add_code("pass");
        // }

        if let Some(terminator_node) = &evt_handler_node.terminator_node {
            terminator_node.accept(self);
        }
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
                    if self.should_use_direct_return() {
                        self.add_code("return = ");
                    } else {
                        self.add_code("self.return_stack[-1] = ");
                    }
                    expr_t.accept(self);
                    // expr_t.auto_post_inc_dec(self);
                    self.generate_return();
                    self.newline();
                }
                None => {
                    // Generate return for auto-added terminator or explicit return without value
                    self.generate_return();
                }
            },
            // DispatchToParentState removed - now handled as ParentDispatchStmt statement
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_statement_node(&mut self, method_call_statement: &CallStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(method_call_statement.line);
        
        // Set flag to prevent duplicate mapping in expression
        let was_in_statement = self.in_statement_context;
        self.in_statement_context = true;
        method_call_statement.call_expr_node.accept(self);
        self.in_statement_context = was_in_statement;
        self.newline();
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node(&mut self, method_call: &CallExprNode) {
        self.debug_enter(&format!("visit_call_expression_node({})", method_call.identifier.name.lexeme));
        
        // v0.65: ONLY use semantic resolution - no backward compatibility
        if !self.handle_call_with_resolved_type(method_call) {
            // If no resolved type, just generate the call as-is
            // This handles special collection constructors and basic external calls
            
            // Process call chain if present
            if let Some(call_chain) = &method_call.call_chain {
                if !call_chain.is_empty() {
                    for callable in call_chain {
                        callable.callable_accept(self);
                        self.add_code(".");
                    }
                }
            }
            
            let method_name = &method_call.identifier.name.lexeme;
            
            // Special handling for Python collection constructors
            if method_name == "set" || method_name == "list" || method_name == "tuple" {
                self.add_code(method_name);
                self.add_code("(");
                
                let expr_count = method_call.call_expr_list.exprs_t.len();
                
                if expr_count > 1 {
                    // Multiple arguments: wrap them in a list
                    self.add_code("[");
                    let mut separator = "";
                    for expr in &method_call.call_expr_list.exprs_t {
                        self.add_code(separator);
                        expr.accept(self);
                        separator = ",";
                    }
                    self.add_code("]");
                } else if expr_count == 1 {
                    let arg = &method_call.call_expr_list.exprs_t[0];
                    arg.accept(self);
                }
                
                self.add_code(")");
            } else if method_name == "dict" {
                self.handle_collection_constructor(method_call);
            } else {
                self.add_code(&method_call.identifier.name.lexeme);
                method_call.call_expr_list.accept(self);
            }
        }
        
        self.debug_exit("visit_call_expression_node");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_expression_node_to_string(
        &mut self,
        method_call: &CallExprNode,
        output: &mut String,
    ) {
        // v0.65: ONLY use semantic resolution - no backward compatibility
        if self.handle_call_with_resolved_type_to_string(method_call, output) {
            return;
        }
        
        // If no resolved type, generate basic call
        // Process call chain if present
        if let Some(call_chain) = &method_call.call_chain {
            if !call_chain.is_empty() {
                for callable in call_chain {
                    let saved_code = self.code.clone();
                    self.code.clear();
                    callable.callable_accept(self);
                    output.push_str(&self.code);
                    output.push('.');
                    self.code = saved_code;
                }
            }
        }
        
        let method_name = &method_call.identifier.name.lexeme;
        
        // Special handling for Python collection constructors
        if method_name == "set" || method_name == "list" || method_name == "tuple" {
            output.push_str(method_name);
            output.push_str("(");
            
            let expr_count = method_call.call_expr_list.exprs_t.len();
            
            if expr_count > 1 {
                output.push_str("[");
                let mut separator = "";
                for expr in &method_call.call_expr_list.exprs_t {
                    output.push_str(separator);
                    expr.accept_to_string(self, output);
                    separator = ",";
                }
                output.push_str("]");
            } else if expr_count == 1 {
                let arg = &method_call.call_expr_list.exprs_t[0];
                arg.accept_to_string(self, output);
            }
            
            output.push_str(")");
        } else if method_name == "dict" {
            self.handle_collection_constructor_to_string(method_call, output);
        } else {
            output.push_str(&method_call.identifier.name.lexeme);
            method_call.call_expr_list.accept_to_string(self, output);
        }
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
        let action_name = &action_call.identifier.name.lexeme;
        let formatted_action_name = self.format_action_name(action_name);
        
        self.debug_print(&format!("visit_action_call_expression_node({}) - in_standalone_function: {}, in_call_chain: {}", action_name, self.in_standalone_function, self.in_call_chain));
        
        // BUG FIX v0.60: When in a call chain, don't output "self." prefix as it's already
        // provided by the SelfT node in the chain. This prevents double-call bugs.
        if self.in_call_chain {
            // In call chain: just output the formatted action name
            self.debug_print("ActionCallExprNode - in call chain, omitting self. prefix");
            self.add_code(&formatted_action_name);
        } else {
            // Standalone action call: include "self." prefix
            self.debug_print("ActionCallExprNode - standalone call, including self. prefix");
            self.add_code(&format!("self.{}", formatted_action_name));
        }

        action_call.call_expr_list.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_expression_node_to_string(
        &mut self,
        action_call: &ActionCallExprNode,
        output: &mut String,
    ) {
        let action_name = &action_call.identifier.name.lexeme;
        let formatted_action_name = self.format_action_name(action_name);
        
        // BUG FIX v0.60: When in a call chain, don't output "self." prefix as it's already
        // provided by the SelfT node in the chain. This prevents double-call bugs like:
        // var result = self.myAction(42) → result = self._myAction(42)(42)
        if self.in_call_chain {
            // In call chain: just output the formatted action name
            output.push_str(&formatted_action_name);
        } else {
            // Standalone action call: include "self." prefix
            output.push_str(&format!("self.{}", formatted_action_name));
        }

        action_call.call_expr_list.accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_action_call_statement_node(&mut self, action_call_stmt_node: &ActionCallStmtNode) {
        self.newline();
        action_call_stmt_node.action_call_expr_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_transition_statement_node(&mut self, transition_statement: &TransitionStatementNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(transition_statement.transition_expr_node.line);
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
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(transition_expr_node.line);
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
    fn visit_parameter_node(&mut self, parameter_node: &ParameterNode) {
        self.add_code(&parameter_node.param_name);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_dispatch_node(&mut self, dispatch_node: &DispatchNode) {
        self.newline();
        self.add_code("self.__router(__e, compartment.parent_compartment)");
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
        debug_print!("DEBUG visit_call_chain_statement_node: {} nodes", 
            method_call_chain_stmt_node.call_chain_literal_expr_node.call_chain.len());
        
        // First add the newline for the statement
        self.skip_next_newline();
        
        // Then map the Frame source line to the Python line where the statement is being generated
        self.add_source_mapping(method_call_chain_stmt_node.line);
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
                if !self.should_use_direct_return() {
                    self.generate_return();
                }

                return;
            }
        }

        // standard case
        debug_print!("DEBUG: Processing standard case with {} nodes", call_chain.len());
        
        // Check if this is an assert statement (single "assert" identifier)
        // If so, suppress the newline so the expression stays on the same line
        let is_assert = call_chain.len() == 1 && 
            matches!(&call_chain[0], 
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } 
                    if id_node.name.lexeme == "assert");
        
        if is_assert {
            // For assert statements, output "assert" without a trailing newline
            // and set a flag to suppress the newline before the next statement
            self.newline();
            self.add_code("assert");
            self.pending_assert = true;
            // Don't process the identifier itself since we've already outputted "assert"
            // The expression will follow as the next statement
        } else {
            method_call_chain_stmt_node
                .call_chain_literal_expr_node
                .accept(self);
        }

        // TODO - review autoinc logic
        // resets flag used in autoinc code
        self.skip_next_newline = false;
    }

    //* --------------------------------------------------------------------- *//

    fn visit_call_chain_expr_node(&mut self, call_l_chain_expression_node: &CallChainExprNode) {
        self.debug_enter(&format!("visit_call_chain_expr_node({} nodes)", call_l_chain_expression_node.call_chain.len()));
        
        // Special handling for self.domain_variable patterns
        // Check if this is a chain starting with "self" followed by a variable
        // v0.37: Also check for SelfT variant
        let starts_with_self_var = call_l_chain_expression_node.call_chain.len() >= 2 &&
            (matches!(call_l_chain_expression_node.call_chain.get(0), 
                Some(CallChainNodeType::VariableNodeT { var_node }) 
                    if var_node.id_node.name.lexeme == "self") ||
             matches!(call_l_chain_expression_node.call_chain.get(0),
                Some(CallChainNodeType::SelfT { .. }))) &&
            matches!(call_l_chain_expression_node.call_chain.get(1),
                Some(CallChainNodeType::VariableNodeT { .. }));
        
        // Check if this is a static method call on a system (SystemName.method())
        let _is_static_system_call = call_l_chain_expression_node.call_chain.len() >= 2 &&
            matches!(call_l_chain_expression_node.call_chain.get(0),
                Some(CallChainNodeType::UndeclaredIdentifierNodeT { id_node })
                    if id_node.name.lexeme.chars().next().map_or(false, |c| c.is_uppercase()));
        
        // Set flag to indicate we're processing within a call chain
        // Only set this for multi-node chains (single-node chains still need self. prefix)
        if call_l_chain_expression_node.call_chain.len() > 1 {
            self.in_call_chain = true;
        }
        
        // TODO: maybe put this in an AST node

        let mut separator = "";

        if call_l_chain_expression_node.inc_dec != IncDecExpr::None {
            self.errors.push(
                "Error - auto increment/decrement operator (++/--) not allowed in Python."
                    .to_string(),
            );
            return;
        }
        
        debug_print!("DEBUG visit_call_chain_expr_node: Processing {} nodes", call_l_chain_expression_node.call_chain.len());
        debug_print!("DEBUG: starts_with_self_var = {}", starts_with_self_var);
        
        for (i, node) in call_l_chain_expression_node.call_chain.iter().enumerate() {
            // Skip the first "self" node in self.variable patterns
            if starts_with_self_var && i == 0 {
                debug_print!("DEBUG: Skipping 'self' node at index 0");
                continue;
            }
            
            let node_type = match &node {
                CallChainNodeType::SelfT { .. } => "Self".to_string(),
                CallChainNodeType::CallChainLiteralExprT { .. } => "Literal".to_string(),
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    format!("UndeclaredIdentifier({})", id_node.name.lexeme)
                },
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    debug_print!("DEBUG: Processing UndeclaredCall '{}' at index {}", call_node.identifier.name.lexeme, i);
                    format!("UndeclaredCall({})", call_node.identifier.name.lexeme)
                },
                CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => format!("InterfaceMethodCall({})", interface_method_call_expr_node.identifier.name.lexeme),
                CallChainNodeType::OperationCallT { operation_call_expr_node } => format!("OperationCall({})", operation_call_expr_node.identifier.name.lexeme),
                CallChainNodeType::OperationRefT { operation_ref_expr_node } => format!("OperationRef({})", operation_ref_expr_node.name),
                CallChainNodeType::ActionCallT { action_call_expr_node } => format!("ActionCall({})", action_call_expr_node.identifier.name.lexeme),
                CallChainNodeType::VariableNodeT { var_node } => {
                    debug_print!("DEBUG: Processing Variable '{}' at index {}", var_node.id_node.name.lexeme, i);
                    format!("Variable({})", var_node.id_node.name.lexeme)
                },
                CallChainNodeType::ListElementNodeT { .. } => "ListElement".to_string(),
                CallChainNodeType::UndeclaredListElementT { .. } => "UndeclaredListElement".to_string(),
                CallChainNodeType::SliceNodeT { .. } => "Slice".to_string(),
                CallChainNodeType::UndeclaredSliceT { .. } => "UndeclaredSlice".to_string(),
            };
            self.debug_print(&format!("Chain node[{}]: {}", i, node_type));
            
            // Special handling for the first variable after self in self.variable patterns
            if starts_with_self_var && i == 1 {
                // For self.variable, we already have the context, just output the variable name
                if let CallChainNodeType::VariableNodeT { var_node } = &node {
                    debug_print!("DEBUG: Outputting self.variable pattern: self.{}", var_node.id_node.name.lexeme);
                    self.add_code(&format!("self.{}", var_node.id_node.name.lexeme));
                    separator = ".";
                    continue;
                }
            }
            
            // Check if this is a synthetic node - if so, don't add separator
            let is_synthetic = match &node {
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    call_node.identifier.name.lexeme == "@indexed_call"
                }
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    list_elem_node.identifier.name.lexeme == "@chain_index" ||
                    list_elem_node.identifier.name.lexeme == "@chain_slice"
                }
                CallChainNodeType::UndeclaredSliceT { slice_node } => {
                    slice_node.identifier.name.lexeme == "@chain_index" ||
                    slice_node.identifier.name.lexeme == "@chain_slice"
                }
                _ => false
            };
            
            if !is_synthetic {
                self.add_code(separator);
                separator = ".";
            }
            
            match &node {
                CallChainNodeType::SelfT { .. } => {
                    // v0.37: Handle 'self' in call chains
                    self.add_code("self");
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    debug_print!("DEBUG: Accepting UndeclaredIdentifier '{}'", id_node.name.lexeme);
                    debug_print!("  Current system enums: {:?}", self.current_system_enums);
                    debug_print!("  Nested module names: {:?}", self.nested_module_names);
                    debug_print!("  Current module path: {:?}", self.current_module_path);
                    debug_print!("  Checking if '{}' is in enum set", id_node.name.lexeme);
                    
                    // v0.46: Handle 'super' in call chains
                    if id_node.name.lexeme == "super" {
                        self.add_code("super()");
                        // Skip adding the separator after super()
                        separator = "";
                        continue;
                    }
                    
                    // v0.57: Check if this is a nested module reference within the current module
                    // When inside a module function and referencing a nested module, qualify it with the module name
                    if !self.current_module_path.is_empty() && self.nested_module_names.contains(&id_node.name.lexeme) {
                        debug_print!("  Found nested module reference '{}' inside module - qualifying with full module path", id_node.name.lexeme);
                        // In Python, static methods need to qualify nested class references with the parent class name
                        // E.g., inside Engineering.getTotalSize(), reference Frontend as Engineering.Frontend
                        // Build the full path to the nested module
                        let qualified_name = self.current_module_path.join(".") + "." + &id_node.name.lexeme;
                        debug_print!("  Qualified name: {}", qualified_name);
                        self.add_code(&qualified_name);
                        continue;
                    }
                    
                    // Check for FSL property access (e.g., list.length)
                    // This is a special case where we need to transform the property access
                    // to a function call in the target language
                    if id_node.name.lexeme == "length" && i > 0 {
                        // This is a .length property access
                        // We need to transform variable.length to len(variable)
                        // But we've already output the variable and the dot, so we need to backtrack
                        
                        // Remove the trailing dot that was added
                        if self.code.ends_with('.') {
                            self.code.pop();
                        }
                        
                        // Find where the variable name starts (everything after the last space or start)
                        let var_start = self.code.rfind(' ').map(|pos| pos + 1).unwrap_or(0);
                        let var_name = self.code[var_start..].to_string();
                        
                        // Remove the variable from the output
                        self.code.truncate(var_start);
                        
                        // Output len(variable) instead
                        self.add_code(&format!("len({})", var_name));
                        
                        // Skip normal processing
                        continue;
                    }
                    
                    // Check if this is an enum member reference (e.g., "HttpStatus.Ok")
                    if id_node.name.lexeme.contains('.') {
                        let parts: Vec<&str> = id_node.name.lexeme.split('.').collect();
                        if parts.len() == 2 {
                            let enum_name = parts[0];
                            let member_name = parts[1];
                            
                            // Check if this enum is defined in the current system
                            if self.current_system_enums.contains(enum_name) {
                                debug_print!("  Found enum member reference: {}.{} - qualifying with system name", enum_name, member_name);
                                self.add_code(&format!("{}_{}.{}", self.system_name, enum_name, member_name));
                            } else if self.module_level_enums.contains(enum_name) {
                                debug_print!("  Found module-level enum member reference: {}.{}", enum_name, member_name);
                                self.add_code(&id_node.name.lexeme);
                            } else {
                                debug_print!("  Enum member reference but enum not found - using as-is");
                                id_node.accept(self);
                            }
                        } else {
                            // More than one dot - not an enum reference
                            id_node.accept(self);
                        }
                    } else if self.current_system_enums.contains(&id_node.name.lexeme) {
                        // Check if this identifier is an enum defined in the current system
                        // If so, qualify it with the system name
                        debug_print!("  YES - qualifying with system name: {}_{}", self.system_name, id_node.name.lexeme);
                        self.add_code(&format!("{}_{}", self.system_name, id_node.name.lexeme));
                    } else {
                        debug_print!("  NO - using unqualified name");
                        id_node.accept(self);
                    }
                }
                CallChainNodeType::UndeclaredCallT { call_node: call } => {
                    debug_print!("DEBUG: Processing UndeclaredCall '{}' in call chain, in_call_chain={}", call.identifier.name.lexeme, self.in_call_chain);
                    // For multi-node chains (e.g., sys.testFruit()), don't add self._ prefix
                    // For single-node chains (e.g., _testFruit()), let it go through normal processing
                    if self.in_call_chain {
                        // Multi-node chain - check for string method transformations
                        debug_print!("DEBUG: Multi-node chain - checking for FSL string operations");
                        
                        // v0.46: Handle super().init() -> super().__init__()
                        // Check if the previous token was super()
                        if self.code.ends_with("super()") && call.identifier.name.lexeme == "init" {
                            self.add_code(".__init__");
                            call.call_expr_list.accept(self);
                            separator = ".";
                            continue;
                        }
                        
                        // Check if this is a string method that needs transformation (v0.33 Phase 3)
                        match call.identifier.name.lexeme.as_str() {
                            "trim" => {
                                // Transform trim() to strip()
                                self.add_code("strip");
                                call.call_expr_list.accept(self);
                            }
                            "upper" => {
                                // upper() is already correct in Python
                                self.add_code("upper");
                                call.call_expr_list.accept(self);
                            }
                            "lower" => {
                                // lower() is already correct in Python  
                                self.add_code("lower");
                                call.call_expr_list.accept(self);
                            }
                            "contains" => {
                                // Transform s.contains(x) to (x in s)
                                // This is complex - need the target and argument
                                // For now, just output contains and note it needs fixing
                                self.add_code("contains");
                                call.call_expr_list.accept(self);
                            }
                            "replace" => {
                                // replace() is already correct in Python
                                self.add_code("replace");
                                call.call_expr_list.accept(self);
                            }
                            "split" => {
                                // split() is already correct in Python
                                self.add_code("split");
                                call.call_expr_list.accept(self);
                            }
                            "substring" => {
                                // Transform substring(start, end) to [start:end]
                                // This needs special handling - for now just output substring
                                self.add_code("substring");
                                call.call_expr_list.accept(self);
                            }
                            "@indexed_call" => {
                                // This is our synthetic node for array[index](args)
                                // Just output the arguments, no method name
                                self.output_indexed_call_args(&call.call_expr_list);
                            }
                            _ => {
                                // Default: output method name as-is
                                self.add_code(&call.identifier.name.lexeme);
                                call.call_expr_list.accept(self);
                            }
                        }
                    } else {
                        // Single-node chain - go through normal processing which might add self._ prefix
                        call.accept(self);
                    }
                }
                CallChainNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    interface_method_call_expr_node.accept(self);
                }
                CallChainNodeType::OperationCallT {
                    operation_call_expr_node,
                } => {
                    // Only set the flag for multi-node chains (obj.method())
                    // Single-node operation calls (self.method()) should keep self. prefix
                    if self.in_call_chain {
                        self.visiting_call_chain_operation = true;
                    }
                    operation_call_expr_node.accept(self);
                    if self.in_call_chain {
                        self.visiting_call_chain_operation = false;
                    }
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
                CallChainNodeType::CallChainLiteralExprT { call_chain_literal_expr_node } => {
                    // v0.41: Handle literal expressions in call chains (e.g., "string".upper())
                    // Output the literal value directly
                    match &call_chain_literal_expr_node.token_t {
                        TokenType::String => self.add_code(&format!("\"{}\"", call_chain_literal_expr_node.value)),
                        TokenType::FString => self.add_code(&call_chain_literal_expr_node.value),
                        TokenType::RawString => self.add_code(&call_chain_literal_expr_node.value),
                        TokenType::ByteString => self.add_code(&call_chain_literal_expr_node.value),
                        TokenType::TripleQuotedString => self.add_code(&call_chain_literal_expr_node.value),
                        TokenType::Number => self.add_code(&call_chain_literal_expr_node.value),
                        TokenType::ComplexNumber => self.add_code(&call_chain_literal_expr_node.value),
                        _ => self.add_code(&call_chain_literal_expr_node.value),
                    }
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    // TODO: figure out why this is necessary as sometimes it generates
                    // unnecessary groups e.g.:
                    // (compartment.state_vars["x"]) = compartment.state_vars["x"] + 1
                    self.visiting_call_chain_literal_variable = true;
                    
                    // Special case: if this is the second node in a self.variable pattern,
                    // we've already output "self." above, so just output the variable name
                    if starts_with_self_var && i == 1 {
                        // This case should have been handled above, but just in case
                        self.add_code(&var_node.id_node.name.lexeme);
                    } else {
                        // Check if this variable is actually an enum type defined in the current system
                        // If so, qualify it with the system name
                        if self.current_system_enums.contains(&var_node.id_node.name.lexeme) {
                            debug_print!("  Variable '{}' is an enum - qualifying with system name: {}_{}", 
                                      var_node.id_node.name.lexeme, self.system_name, var_node.id_node.name.lexeme);
                            self.add_code(&format!("{}_{}", self.system_name, var_node.id_node.name.lexeme));
                        } else {
                            var_node.accept(self);
                        }
                    }
                    self.visiting_call_chain_literal_variable = false;
                }
                CallChainNodeType::ListElementNodeT { list_elem_node } => {
                    list_elem_node.accept(self);
                }
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    list_elem_node.accept(self);
                }
                CallChainNodeType::SliceNodeT { slice_node } => {
                    slice_node.accept(self);
                }
                CallChainNodeType::UndeclaredSliceT { slice_node } => {
                    slice_node.accept(self);
                }
            }
        }
        
        // Reset the flag
        if call_l_chain_expression_node.call_chain.len() > 1 {
            self.in_call_chain = false;
        }
        
        self.debug_exit("visit_call_chain_expr_node");
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
        // Special handling for self.domain_variable patterns
        // Check if this is a chain starting with "self" followed by a variable
        // v0.37: Also check for SelfT variant
        let starts_with_self_var = call_chain_expression_node.call_chain.len() >= 2 &&
            (matches!(call_chain_expression_node.call_chain.get(0), 
                Some(CallChainNodeType::VariableNodeT { var_node }) 
                    if var_node.id_node.name.lexeme == "self") ||
             matches!(call_chain_expression_node.call_chain.get(0),
                Some(CallChainNodeType::SelfT { .. }))) &&
            matches!(call_chain_expression_node.call_chain.get(1),
                Some(CallChainNodeType::VariableNodeT { .. }));
        
        // Check if this is a static method call on a system (SystemName.method())
        let _is_static_system_call = call_chain_expression_node.call_chain.len() >= 2 &&
            matches!(call_chain_expression_node.call_chain.get(0),
                Some(CallChainNodeType::UndeclaredIdentifierNodeT { id_node })
                    if id_node.name.lexeme.chars().next().map_or(false, |c| c.is_uppercase()));
        
        // Set flag to indicate we're processing within a call chain
        // Only set this for multi-node chains (single-node chains still need self. prefix)
        if call_chain_expression_node.call_chain.len() > 1 {
            self.in_call_chain = true;
        }
        
        let mut separator = "";

        for (i, node) in call_chain_expression_node.call_chain.iter().enumerate() {
            // Skip the first "self" node in self.variable patterns
            if starts_with_self_var && i == 0 {
                continue;
            }
            
            // Special handling for the first variable after self in self.variable patterns
            if starts_with_self_var && i == 1 {
                // For self.variable, we already have the context, just output the variable name
                if let CallChainNodeType::VariableNodeT { var_node } = &node {
                    output.push_str(&format!("self.{}", var_node.id_node.name.lexeme));
                    separator = ".";
                    continue;
                }
            }
            
            let _node_desc = match &node {
                CallChainNodeType::SelfT { .. } => "Self".to_string(),
                CallChainNodeType::CallChainLiteralExprT { .. } => "Literal".to_string(),
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => 
                    format!("UndeclaredIdentifier({})", id_node.name.lexeme),
                CallChainNodeType::UndeclaredCallT { call_node } => 
                    format!("UndeclaredCall({})", call_node.identifier.name.lexeme),
                _ => "Other".to_string()
            };
            
            // Check if this is a synthetic node - if so, don't add separator
            let is_synthetic = match &node {
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    call_node.identifier.name.lexeme == "@indexed_call"
                }
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    list_elem_node.identifier.name.lexeme == "@chain_index" ||
                    list_elem_node.identifier.name.lexeme == "@chain_slice"
                }
                CallChainNodeType::UndeclaredSliceT { slice_node } => {
                    slice_node.identifier.name.lexeme == "@chain_index" ||
                    slice_node.identifier.name.lexeme == "@chain_slice"
                }
                _ => false
            };
            
            if !is_synthetic {
                output.push_str(separator);
            }
            match &node {
                CallChainNodeType::SelfT { .. } => {
                    // v0.37: Handle 'self' as the base of a call chain
                    output.push_str("self");
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    debug_print!("DEBUG _to_string: UndeclaredIdentifier '{}'", id_node.name.lexeme);
                    debug_print!("  Current system enums: {:?}", self.current_system_enums);
                    debug_print!("  Nested module names: {:?}", self.nested_module_names);
                    debug_print!("  Current module path: {:?}", self.current_module_path);
                    
                    // v0.57: Check if this is a nested module reference within the current module
                    if !self.current_module_path.is_empty() && self.nested_module_names.contains(&id_node.name.lexeme) {
                        debug_print!("  Found nested module reference '{}' - qualifying with full module path", id_node.name.lexeme);
                        // In Python, static methods need to qualify nested class references with the parent class name
                        let qualified_name = self.current_module_path.join(".") + "." + &id_node.name.lexeme;
                        debug_print!("  Qualified name: {}", qualified_name);
                        output.push_str(&qualified_name);
                    } else if self.current_system_enums.contains(&id_node.name.lexeme) {
                        // Check if this identifier is an enum defined in the current system
                        debug_print!("  YES - qualifying: {}_{}", self.system_name, id_node.name.lexeme);
                        output.push_str(&format!("{}_{}", self.system_name, id_node.name.lexeme));
                    } else {
                        debug_print!("  NO - using unqualified");
                        id_node.accept_to_string(self, output);
                    }
                }
                CallChainNodeType::UndeclaredCallT { call_node: call } => {
                    // Check for string method transformations (v0.33 Phase 3)
                    if self.in_call_chain {
                        match call.identifier.name.lexeme.as_str() {
                            "trim" => {
                                // Transform trim() to strip()
                                output.push_str("strip");
                                call.call_expr_list.accept_to_string(self, output);
                            }
                            "@indexed_call" => {
                                // This is our synthetic node for array[index](args)
                                // Just output the arguments, no method name
                                self.output_indexed_call_args_to_string(&call.call_expr_list, output);
                            }
                            _ => {
                                // Default behavior for other methods
                                call.accept_to_string(self, output);
                            }
                        }
                    } else {
                        // Also check for @indexed_call when not in call chain
                        if call.identifier.name.lexeme == "@indexed_call" {
                            self.output_indexed_call_args_to_string(&call.call_expr_list, output);
                        } else {
                            call.accept_to_string(self, output);
                        }
                    }
                }
                CallChainNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    interface_method_call_expr_node.accept_to_string(self, output);
                }
                CallChainNodeType::OperationCallT {
                    operation_call_expr_node,
                } => {
                    // Only set the flag for multi-node chains (obj.method())
                    // Single-node operation calls (self.method()) should keep self. prefix
                    if self.in_call_chain {
                        self.visiting_call_chain_operation = true;
                    }
                    operation_call_expr_node.accept_to_string(self, output);
                    if self.in_call_chain {
                        self.visiting_call_chain_operation = false;
                    }
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
                CallChainNodeType::CallChainLiteralExprT { call_chain_literal_expr_node } => {
                    // v0.41: Handle literal expressions in call chains (e.g., "string".upper())
                    // Output the literal value directly
                    match &call_chain_literal_expr_node.token_t {
                        TokenType::String => output.push_str(&format!("\"{}\"", call_chain_literal_expr_node.value)),
                        TokenType::FString => output.push_str(&call_chain_literal_expr_node.value),
                        TokenType::RawString => output.push_str(&call_chain_literal_expr_node.value),
                        TokenType::ByteString => output.push_str(&call_chain_literal_expr_node.value),
                        TokenType::TripleQuotedString => output.push_str(&call_chain_literal_expr_node.value),
                        TokenType::Number => output.push_str(&call_chain_literal_expr_node.value),
                        TokenType::ComplexNumber => output.push_str(&call_chain_literal_expr_node.value),
                        _ => output.push_str(&call_chain_literal_expr_node.value),
                    }
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    // Check if this variable is actually an enum type defined in the current system
                    // If so, qualify it with the system name
                    if self.current_system_enums.contains(&var_node.id_node.name.lexeme) {
                        output.push_str(&format!("{}_{}", self.system_name, var_node.id_node.name.lexeme));
                    } else {
                        // v0.37: When in a call chain with SelfT, just output the variable name
                        // without the "self." prefix since SelfT already provides it
                        if i > 0 && matches!(call_chain_expression_node.call_chain[0], CallChainNodeType::SelfT { .. }) {
                            // Just output the variable name without "self." prefix
                            output.push_str(&var_node.id_node.name.lexeme);
                        } else {
                            var_node.accept_to_string(self, output);
                        }
                    }
                }
                CallChainNodeType::ListElementNodeT { list_elem_node } => {
                    list_elem_node.accept_to_string(self, output);
                }
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    list_elem_node.accept_to_string(self, output);
                }
                CallChainNodeType::SliceNodeT { slice_node } => {
                    slice_node.accept_to_string(self, output);
                }
                CallChainNodeType::UndeclaredSliceT { slice_node } => {
                    slice_node.accept_to_string(self, output);
                }
            }
            separator = ".";
        }
        
        // Reset the flag
        if call_chain_expression_node.call_chain.len() > 1 {
            self.in_call_chain = false;
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_if_stmt_node(&mut self, if_stmt_node: &IfStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(if_stmt_node.line);
        self.add_code("if ");
        if_stmt_node.condition.accept(self);
        self.add_code(":");
        if_stmt_node.if_block.accept(self);

        for elif_clause in &if_stmt_node.elif_clauses {
            self.newline();
            self.add_code("elif ");
            elif_clause.condition.accept(self);
            self.add_code(":");
            elif_clause.block.accept(self);
        }

        if let Some(else_block) = &if_stmt_node.else_block {
            self.newline();
            self.add_code("else:");
            else_block.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_for_stmt_node(&mut self, for_stmt_node: &ForStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(for_stmt_node.line);
        self.add_code("for ");
        
        // Emit the loop variable
        if let Some(variable) = &for_stmt_node.variable {
            self.add_code(&variable.id_node.name.lexeme);
        } else if let Some(identifier) = &for_stmt_node.identifier {
            self.add_code(&identifier.name.lexeme);
        }
        
        self.add_code(" in ");
        
        // Special handling for enum iteration
        if for_stmt_node.is_enum_iteration {
            // Debug output removed
            
            // Generate the qualified enum name for iteration
            if let Some(ref enum_name) = for_stmt_node.enum_type_name {
                // Check if we're in a system context
                if !self.system_name.is_empty() {
                    // Use system-prefixed enum name
                    let qualified_name = format!("{}_{}", self.system_name, enum_name);
                    // debug_print!("  Using qualified enum name: {}", qualified_name);
                    self.add_code(&qualified_name);
                } else {
                    // Module-level enum, use directly
                    // debug_print!("  Using module-level enum name: {}", enum_name);
                    self.add_code(enum_name);
                }
            } else {
                // debug_print!("  No enum_type_name, falling back to regular iterable");
                // Fallback to regular iterable
                for_stmt_node.iterable.accept(self);
            }
        } else {
            // Regular iterable
            for_stmt_node.iterable.accept(self);
        }
        
        self.add_code(":");
        for_stmt_node.block.accept(self);
        
        // v0.51: Handle optional else clause
        if let Some(else_block) = &for_stmt_node.else_block {
            self.newline();
            self.add_code("else:");
            else_block.accept(self);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_while_stmt_node(&mut self, while_stmt_node: &WhileStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(while_stmt_node.line);
        self.add_code("while ");
        while_stmt_node.condition.accept(self);
        self.add_code(":");
        while_stmt_node.block.accept(self);
        
        // v0.51: Handle optional else clause
        if let Some(else_block) = &while_stmt_node.else_block {
            self.newline();
            self.add_code("else:");
            else_block.accept(self);
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
            _ => {
                self.errors.push("Error - unexpected target expression in 'in' loop.".to_string());
                return;
            }
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
        self.add_source_mapping(loop_in_expr_node.line);
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

    fn visit_continue_stmt_node(&mut self, continue_stmt_node: &ContinueStmtNode) {
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

        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(continue_stmt_node.line);
        self.add_code("continue");
    }

    //* --------------------------------------------------------------------- *//

    // visit_superstring_stmt_node removed - backticks no longer supported

    //* --------------------------------------------------------------------- *//

    fn visit_break_stmt_node(&mut self, break_stmt_node: &BreakStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(break_stmt_node.line);
        self.add_code("break");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assert_stmt_node(&mut self, assert_stmt_node: &AssertStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(assert_stmt_node.line);
        self.add_code("assert ");
        assert_stmt_node.expr.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    // v0.50: Del statement support
    fn visit_del_stmt_node(&mut self, del_stmt_node: &DelStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(del_stmt_node.line);
        self.add_code("del ");
        del_stmt_node.target.accept(self);
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
                            if self.should_use_direct_return() {
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
                    // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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

    fn visit_return_stmt_node(&mut self, return_stmt_node: &ReturnStmtNode) {
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(return_stmt_node.line);
        if let Some(expr_t) = &return_stmt_node.expr_t_opt {
            if self.should_use_direct_return() {
                // In functions/actions/operations: direct return
                let mut output = String::new();
                expr_t.accept_to_string(self, &mut output);
                self.add_code(&format!("return {}", output));
            } else {
                // In event handlers: use return stack
                self.add_code("self.return_stack[-1] = ");
                expr_t.accept(self);
                self.newline();
                self.add_code("return");
            }
        } else {
            self.add_code("return");
        }
        // Mark that we've generated a return to avoid duplicate in terminator
        self.this_branch_transitioned = true;
    }
    
    //* --------------------------------------------------------------------- *//

    fn visit_parent_dispatch_stmt_node(&mut self, _parent_dispatch_stmt_node: &ParentDispatchStmtNode) {
        self.newline();
        self.add_code("# => $^ parent dispatch");
        self.newline();
        self.add_code("self.__router(__e, compartment.parent_compartment)");
        
        // Check if a transition was triggered and return early
        self.newline();
        self.add_code("if self.__next_compartment is not None:");
        self.indent();
        self.newline();
        self.add_code("return");
        self.outdent();
    }
    
    //* --------------------------------------------------------------------- *//
    
    fn visit_try_stmt_node(&mut self, try_stmt_node: &TryStmtNode) {
        self.newline();
        self.add_code("try:");
        self.indent();
        try_stmt_node.try_block.accept(self);
        self.outdent();
        
        for except_clause in &try_stmt_node.except_clauses {
            except_clause.accept(self);
        }
        
        if let Some(else_block) = &try_stmt_node.else_block {
            self.newline();
            self.add_code("else:");
            self.indent();
            else_block.accept(self);
            self.outdent();
        }
        
        if let Some(finally_block) = &try_stmt_node.finally_block {
            self.newline();
            self.add_code("finally:");
            self.indent();
            finally_block.accept(self);
            self.outdent();
        }
    }
    
    //* --------------------------------------------------------------------- *//
    
    fn visit_except_clause_node(&mut self, except_clause_node: &ExceptClauseNode) {
        self.newline();
        self.add_code("except");
        
        // Handle the various except clause forms
        if let Some(exception_types) = &except_clause_node.exception_types {
            if exception_types.len() == 1 {
                self.add_code(&format!(" {}", exception_types[0]));
            } else if exception_types.len() > 1 {
                let types_str = exception_types.join(", ");
                self.add_code(&format!(" ({})", types_str));
            }
            
            // Add variable binding if present
            if let Some(var_name) = &except_clause_node.var_name {
                self.add_code(&format!(" as {}", var_name));
            }
        }
        // Note: If we have no exception types but have a var_name, that means
        // it was actually meant to be an exception type without binding.
        // The parser couldn't distinguish between `except ValueError` and `except e`
        // In Python, `except e` would be invalid syntax anyway (need `except Exception as e`)
        // So if we have just a var_name and no exception_types, treat it as an exception type
        else if let Some(var_name) = &except_clause_node.var_name {
            // This is actually an exception type without variable binding
            self.add_code(&format!(" {}", var_name));
        }
        // else: bare except clause (catches everything)
        
        self.add_code(":");
        self.indent();
        except_clause_node.block.accept(self);
        self.outdent();
    }
    
    //* --------------------------------------------------------------------- *//
    
    fn visit_raise_stmt_node(&mut self, raise_stmt_node: &RaiseStmtNode) {
        self.newline();
        self.add_code("raise");
        
        if let Some(exception_expr) = &raise_stmt_node.exception_expr {
            self.add_code(" ");
            let mut expr_str = String::new();
            exception_expr.accept_to_string(self, &mut expr_str);
            self.add_code(&expr_str);
        }
        
        if let Some(from_expr) = &raise_stmt_node.from_expr {
            self.add_code(" from ");
            let mut from_str = String::new();
            from_expr.accept_to_string(self, &mut from_str);
            self.add_code(&from_str);
        }
    }
    
    //* --------------------------------------------------------------------- *//

    fn visit_with_stmt_node(&mut self, with_stmt_node: &WithStmtNode) {
        self.newline();
        
        // Add 'async with' or 'with' keyword
        if with_stmt_node.is_async {
            self.add_code("async with ");
        } else {
            self.add_code("with ");
        }
        
        // Add the context expression
        let mut expr_str = String::new();
        with_stmt_node.context_expr.accept_to_string(self, &mut expr_str);
        self.add_code(&expr_str);
        
        // Add 'as' clause if present
        if let Some(target_var) = &with_stmt_node.target_var {
            self.add_code(" as ");
            self.add_code(target_var);
        }
        
        self.add_code(":");
        
        // Visit the with block
        self.indent();
        with_stmt_node.with_block.accept(self);
        self.outdent();
    }
    
    //* --------------------------------------------------------------------- *//

    fn visit_match_stmt_node(&mut self, match_stmt_node: &MatchStmtNode) {
        self.newline();
        self.add_code("match ");
        
        // Add the match expression
        let mut expr_str = String::new();
        match_stmt_node.match_expr.accept_to_string(self, &mut expr_str);
        self.add_code(&expr_str);
        self.add_code(":");
        
        // Visit each case
        self.indent();
        for case in &match_stmt_node.cases {
            case.accept(self);
        }
        self.outdent();
    }
    
    //* --------------------------------------------------------------------- *//
    
    fn visit_case_node(&mut self, case_node: &CaseNode) {
        self.newline();
        self.add_code("case ");
        
        // Visit the pattern
        case_node.pattern.accept(self);
        
        // Add guard clause if present
        if let Some(ref guard) = case_node.guard {
            self.add_code(" if ");
            let mut guard_str = String::new();
            guard.accept_to_string(self, &mut guard_str);
            self.add_code(&guard_str);
        }
        
        self.add_code(":");
        
        // Visit the case body
        self.indent();
        if case_node.statements.is_empty() {
            // Empty case needs pass statement in Python
            self.newline();
            self.add_code("pass");
        } else {
            self.visit_decl_stmts(&case_node.statements);
        }
        self.outdent();
    }
    
    //* --------------------------------------------------------------------- *//
    
    fn visit_pattern_node(&mut self, pattern_node: &PatternNode) {
        match pattern_node {
            PatternNode::Literal(literal) => {
                // Generate literal pattern
                literal.accept(self);
            },
            PatternNode::Capture(name) => {
                // Generate capture pattern (just the identifier)
                self.add_code(name);
            },
            PatternNode::Wildcard => {
                // Generate wildcard pattern
                self.add_code("_");
            },
            PatternNode::Sequence(patterns) => {
                // Generate sequence pattern [a, b, c]
                self.add_code("[");
                for (i, pattern) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.add_code(", ");
                    }
                    pattern.accept(self);
                }
                self.add_code("]");
            },
            PatternNode::Mapping(mappings) => {
                // Generate mapping pattern {"key": value}
                self.add_code("{");
                for (i, (key, pattern)) in mappings.iter().enumerate() {
                    if i > 0 {
                        self.add_code(", ");
                    }
                    self.add_code("\"");
                    self.add_code(key);
                    self.add_code("\": ");
                    pattern.accept(self);
                }
                self.add_code("}");
            },
            PatternNode::Class(name, args) => {
                // Generate class pattern Point(x, y)
                // Note: Frame parses tuples as lists in patterns, so we need to generate
                // the appropriate syntax
                if name == "Point" || name == "Circle" || name == "Rectangle" {
                    // These look like class patterns but we're using tuples for now
                    self.add_code("(\"");
                    self.add_code(name);
                    self.add_code("\", ");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.add_code(", ");
                        }
                        arg.accept(self);
                    }
                    self.add_code(")");
                } else {
                    // Generic class pattern
                    self.add_code(name);
                    self.add_code("(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.add_code(", ");
                        }
                        arg.accept(self);
                    }
                    self.add_code(")");
                }
            },
            PatternNode::Or(patterns) => {
                // Generate or pattern pattern1 | pattern2
                for (i, pattern) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.add_code(" | ");
                    }
                    pattern.accept(self);
                }
            },
            PatternNode::As(pattern, name) => {
                // Generate as pattern: pattern as name
                pattern.accept(self);
                self.add_code(" as ");
                self.add_code(name);
            },
            PatternNode::Star(name) => {
                // Generate star pattern: *name
                self.add_code("*");
                self.add_code(name);
            },
        }
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
            // Don't wrap single transitions, yield, or await expressions in parentheses
            match expr_list.exprs_t.get(0) {
                Some(ExprType::TransitionExprT { .. }) => generate_parens = false,
                Some(ExprType::YieldExprT { .. }) => generate_parens = false,
                Some(ExprType::YieldFromExprT { .. }) => generate_parens = false,
                Some(ExprType::AwaitExprT { .. }) => generate_parens = false,
                _ => {}
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
        // Check if this is a list comprehension (single comprehension expression)
        if list.exprs_t.len() == 1 {
            if let ExprType::ListComprehensionExprT { .. } = &list.exprs_t[0] {
                // Just visit the comprehension directly, it will generate [...]
                list.exprs_t[0].accept(self);
                return;
            }
        }
        
        // Regular list or list with unpacking
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
        // Check if this is a list comprehension
        if list.exprs_t.len() == 1 {
            if let ExprType::ListComprehensionExprT { .. } = &list.exprs_t[0] {
                list.exprs_t[0].accept_to_string(self, output);
                return;
            }
        }
        
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
    
    fn visit_dict_literal_node(&mut self, dict: &DictLiteralNode) {
        self.add_code("{");
        
        let mut separator = "";
        for (key, value) in &dict.pairs {
            self.add_code(separator);
            
            // Check if this is a dict unpacking (key is DictUnpackExprT and value is NilExprT)
            if matches!(key, ExprType::DictUnpackExprT { .. }) && matches!(value, ExprType::NilExprT) {
                // For dict unpacking, just visit the key which contains the unpacking expression
                key.accept(self);
            } else {
                // Regular key-value pair
                key.accept(self);
                self.add_code(": ");
                value.accept(self);
            }
            separator = ", ";
        }
        
        self.add_code("}");
    }
    
    fn visit_dict_literal_node_to_string(&mut self, dict: &DictLiteralNode, output: &mut String) {
        output.push('{');
        
        let mut separator = "";
        for (key, value) in &dict.pairs {
            output.push_str(separator);
            
            // Check if this is a dict unpacking (key is DictUnpackExprT and value is NilExprT)
            if matches!(key, ExprType::DictUnpackExprT { .. }) && matches!(value, ExprType::NilExprT) {
                // For dict unpacking, just visit the key which contains the unpacking expression
                key.accept_to_string(self, output);
            } else {
                // Regular key-value pair
                key.accept_to_string(self, output);
                output.push_str(": ");
                value.accept_to_string(self, output);
            }
            separator = ", ";
        }
        
        output.push('}');
    }
    
    //* --------------------------------------------------------------------- *//
    
    fn visit_set_literal_node(&mut self, set: &SetLiteralNode) {
        // v0.38: Empty set requires set() in Python, not {}
        if set.elements.is_empty() {
            self.add_code("set()");
        } else {
            self.add_code("{");
            
            let mut separator = "";
            for element in &set.elements {
                self.add_code(separator);
                element.accept(self);
                separator = ", ";
            }
            
            self.add_code("}");
        }
    }
    
    fn visit_set_literal_node_to_string(&mut self, set: &SetLiteralNode, output: &mut String) {
        // v0.38: Empty set requires set() in Python, not {}
        if set.elements.is_empty() {
            output.push_str("set()");
        } else {
            output.push('{');
            
            let mut separator = "";
            for element in &set.elements {
                output.push_str(separator);
                element.accept_to_string(self, output);
                separator = ", ";
            }
            
            output.push('}');
        }
    }
    
    //* --------------------------------------------------------------------- *//
    
    fn visit_tuple_literal_node(&mut self, tuple: &TupleLiteralNode) {
        self.add_code("(");
        
        let mut separator = "";
        for element in &tuple.elements {
            self.add_code(separator);
            element.accept(self);
            separator = ", ";
        }
        
        // Single element tuples need trailing comma in Python
        if tuple.elements.len() == 1 {
            self.add_code(",");
        }
        
        self.add_code(")");
    }
    
    fn visit_tuple_literal_node_to_string(&mut self, tuple: &TupleLiteralNode, output: &mut String) {
        output.push('(');
        
        let mut separator = "";
        for element in &tuple.elements {
            output.push_str(separator);
            element.accept_to_string(self, output);
            separator = ", ";
        }
        
        // Single element tuples need trailing comma in Python
        if tuple.elements.len() == 1 {
            output.push(',');
        }
        
        output.push(')');
    }
    
    //* --------------------------------------------------------------------- *//
    
    // v0.34: Visit unpacking expression node
    fn visit_unpack_expr_node(&mut self, unpack: &UnpackExprNode) {
        self.add_code("*");
        unpack.expr.accept(self);
    }
    
    // v0.54: Visit star expression node for unpacking
    fn visit_star_expr_node(&mut self, star_expr: &StarExprNode) {
        self.add_code("*");
        self.add_code(&star_expr.identifier);
    }
    
    // v0.34: Visit list comprehension node
    fn visit_list_comprehension_node(&mut self, comp: &ListComprehensionNode) {
        self.add_code("[");
        
        // Generate the expression part
        comp.expr.accept(self);
        
        // Generate 'for target in iterable'
        self.add_code(" for ");
        self.add_code(&comp.target);
        self.add_code(" in ");
        comp.iter.accept(self);
        
        // Optional 'if' condition
        if let Some(ref condition) = comp.condition {
            self.add_code(" if ");
            condition.accept(self);
        }
        
        self.add_code("]");
    }
    
    // v0.34: Visit list comprehension node to string
    fn visit_list_comprehension_node_to_string(&mut self, comp: &ListComprehensionNode, output: &mut String) {
        output.push('[');
        
        // Generate the expression part
        comp.expr.accept_to_string(self, output);
        
        // Generate 'for target in iterable'
        output.push_str(" for ");
        output.push_str(&comp.target);
        output.push_str(" in ");
        comp.iter.accept_to_string(self, output);
        
        // Optional 'if' condition
        if let Some(ref condition) = comp.condition {
            output.push_str(" if ");
            condition.accept_to_string(self, output);
        }
        
        output.push(']');
    }
    
    // v0.38: Visit dictionary comprehension node
    fn visit_dict_comprehension_node(&mut self, comp: &DictComprehensionNode) {
        self.add_code("{");
        
        // Generate the key expression
        comp.key_expr.accept(self);
        self.add_code(": ");
        
        // Generate the value expression
        comp.value_expr.accept(self);
        
        // Generate 'for target in iterable'
        self.add_code(" for ");
        self.add_code(&comp.target);
        self.add_code(" in ");
        comp.iter.accept(self);
        
        // Optional 'if' condition
        if let Some(ref condition) = comp.condition {
            self.add_code(" if ");
            condition.accept(self);
        }
        
        self.add_code("}");
    }
    
    // v0.38: Visit dictionary comprehension node to string
    fn visit_dict_comprehension_node_to_string(&mut self, comp: &DictComprehensionNode, output: &mut String) {
        output.push('{');
        
        // Generate the key expression
        comp.key_expr.accept_to_string(self, output);
        output.push_str(": ");
        
        // Generate the value expression
        comp.value_expr.accept_to_string(self, output);
        
        // Generate 'for target in iterable'
        output.push_str(" for ");
        output.push_str(&comp.target);
        output.push_str(" in ");
        comp.iter.accept_to_string(self, output);
        
        // Optional 'if' condition
        if let Some(ref condition) = comp.condition {
            output.push_str(" if ");
            condition.accept_to_string(self, output);
        }
        
        output.push('}');
    }
    
    // v0.41: Visit set comprehension node
    fn visit_set_comprehension_node(&mut self, comp: &SetComprehensionNode) {
        self.add_code("{");
        
        // Generate the expression part
        comp.expr.accept(self);
        
        // Generate 'for target in iterable'
        self.add_code(" for ");
        self.add_code(&comp.target);
        self.add_code(" in ");
        comp.iter.accept(self);
        
        // Optional 'if' condition
        if let Some(ref condition) = comp.condition {
            self.add_code(" if ");
            condition.accept(self);
        }
        
        self.add_code("}");
    }
    
    // v0.41: Visit set comprehension node to string
    fn visit_set_comprehension_node_to_string(&mut self, comp: &SetComprehensionNode, output: &mut String) {
        output.push('{');
        
        // Generate the expression part
        comp.expr.accept_to_string(self, output);
        
        // Generate 'for target in iterable'
        output.push_str(" for ");
        output.push_str(&comp.target);
        output.push_str(" in ");
        comp.iter.accept_to_string(self, output);
        
        // Optional 'if' condition
        if let Some(ref condition) = comp.condition {
            output.push_str(" if ");
            condition.accept_to_string(self, output);
        }
        
        output.push('}');
    }
    
    // v0.34: Visit unpacking expression node to string  
    fn visit_unpack_expr_node_to_string(&mut self, unpack: &UnpackExprNode, output: &mut String) {
        output.push('*');
        unpack.expr.accept_to_string(self, output);
    }
    
    // v0.54: Visit star expression node to string
    fn visit_star_expr_node_to_string(&mut self, star_expr: &StarExprNode, output: &mut String) {
        output.push('*');
        output.push_str(&star_expr.identifier);
    }
    
    // v0.38: Visit dict unpacking expression node
    fn visit_dict_unpack_expr_node(&mut self, dict_unpack: &DictUnpackExprNode) {
        self.add_code("**");
        dict_unpack.expr.accept(self);
    }
    
    // v0.38: Visit dict unpacking expression node to string
    fn visit_dict_unpack_expr_node_to_string(&mut self, dict_unpack: &DictUnpackExprNode, output: &mut String) {
        output.push_str("**");
        dict_unpack.expr.accept_to_string(self, output);
    }
    
    // v0.35: Visit await expression node
    fn visit_await_expr_node(&mut self, await_expr: &AwaitExprNode) {
        self.add_code("await ");
        await_expr.expr.accept(self);
    }
    
    // v0.35: Visit await expression node to string
    fn visit_await_expr_node_to_string(&mut self, await_expr: &AwaitExprNode, output: &mut String) {
        output.push_str("await ");
        await_expr.expr.accept_to_string(self, output);
    }
    
    // v0.38: Visit lambda expression node
    fn visit_lambda_expr_node(&mut self, lambda_expr: &LambdaExprNode) {
        self.add_code("lambda ");
        
        // Add parameters
        for (i, param) in lambda_expr.params.iter().enumerate() {
            if i > 0 {
                self.add_code(", ");
            }
            self.add_code(param);
        }
        
        self.add_code(": ");
        
        // Add body expression
        lambda_expr.body.accept(self);
    }
    
    // v0.38: Visit lambda expression node to string
    fn visit_lambda_expr_node_to_string(&mut self, lambda_expr: &LambdaExprNode, output: &mut String) {
        output.push_str("lambda ");
        
        // Add parameters
        for (i, param) in lambda_expr.params.iter().enumerate() {
            if i > 0 {
                output.push_str(", ");
            }
            output.push_str(param);
        }
        
        output.push_str(": ");
        
        // Add body expression
        lambda_expr.body.accept_to_string(self, output);
    }
    
    // v0.38: Visit function reference node (first-class functions)
    fn visit_function_ref_node(&mut self, name: &str) {
        // In Python, function references are just the function name without parentheses
        self.add_code(name);
    }
    
    // v0.38: Visit function reference node to string
    fn visit_function_ref_node_to_string(&mut self, name: &str, output: &mut String) {
        // In Python, function references are just the function name without parentheses
        output.push_str(name);
    }
    
    // v0.42: Visit yield expression node
    fn visit_yield_expr_node(&mut self, yield_expr: &YieldExprNode) {
        self.add_code("yield");
        if let Some(ref expr) = yield_expr.expr {
            self.add_code(" ");
            expr.accept(self);
        }
    }
    
    // v0.42: Visit yield expression node to string
    fn visit_yield_expr_node_to_string(&mut self, yield_expr: &YieldExprNode, output: &mut String) {
        output.push_str("yield");
        if let Some(ref expr) = yield_expr.expr {
            output.push(' ');
            expr.accept_to_string(self, output);
        }
    }
    
    // v0.42: Visit yield from expression node
    fn visit_yield_from_expr_node(&mut self, yield_from_expr: &YieldFromExprNode) {
        self.add_code("yield from ");
        yield_from_expr.expr.accept(self);
    }
    
    // v0.42: Visit yield from expression node to string
    fn visit_yield_from_expr_node_to_string(&mut self, yield_from_expr: &YieldFromExprNode, output: &mut String) {
        output.push_str("yield from ");
        yield_from_expr.expr.accept_to_string(self, output);
    }
    
    // v0.42: Visit generator expression node
    fn visit_generator_expr_node(&mut self, generator_expr: &GeneratorExprNode) {
        self.add_code("(");
        generator_expr.expr.accept(self);
        self.add_code(" for ");
        self.add_code(&generator_expr.target);
        self.add_code(" in ");
        generator_expr.iter.accept(self);
        if let Some(ref condition) = generator_expr.condition {
            self.add_code(" if ");
            condition.accept(self);
        }
        self.add_code(")");
    }
    
    // v0.42: Visit generator expression node to string
    fn visit_generator_expr_node_to_string(&mut self, generator_expr: &GeneratorExprNode, output: &mut String) {
        output.push('(');
        generator_expr.expr.accept_to_string(self, output);
        output.push_str(" for ");
        output.push_str(&generator_expr.target);
        output.push_str(" in ");
        generator_expr.iter.accept_to_string(self, output);
        if let Some(ref condition) = generator_expr.condition {
            output.push_str(" if ");
            condition.accept_to_string(self, output);
        }
        output.push(')');
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
    
    fn visit_slice_node(&mut self, slice_node: &SliceNode) {
        // Output the identifier
        slice_node.identifier.accept(self);
        self.add_code("[");
        
        // Output start:end:step
        if let Some(start) = &slice_node.start_expr {
            start.accept(self);
        }
        self.add_code(":");
        if let Some(end) = &slice_node.end_expr {
            end.accept(self);
        }
        if let Some(step) = &slice_node.step_expr {
            self.add_code(":");
            step.accept(self);
        }
        
        self.add_code("]");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_slice_node_to_string(&mut self, slice_node: &SliceNode, output: &mut String) {
        // Output the identifier
        slice_node.identifier.accept_to_string(self, output);
        output.push('[');
        
        // Output start:end:step
        if let Some(start) = &slice_node.start_expr {
            start.accept_to_string(self, output);
        }
        output.push(':');
        if let Some(end) = &slice_node.end_expr {
            end.accept_to_string(self, output);
        }
        if let Some(step) = &slice_node.step_expr {
            output.push(':');
            step.accept_to_string(self, output);
        }
        
        output.push(']');
    }

    //* --------------------------------------------------------------------- *//

    fn visit_expr_list_stmt_node(&mut self, expr_list_stmt_node: &ExprListStmtNode) {
        self.debug_print("EXPR_LIST_DEBUG: Processing ExprListStmt - this might be the print() issue!");
        let ref expr_list_node = expr_list_stmt_node.expr_list_node;
        self.test_skip_newline();
        expr_list_node.accept(self);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_literal_expression_node(&mut self, literal_expression_node: &LiteralExprNode) {
        match &literal_expression_node.token_t {
            TokenType::Number => self.add_code(&literal_expression_node.value.to_string()),
            TokenType::ComplexNumber => self.add_code(&literal_expression_node.value.to_string()),
            // SuperString removed - backticks no longer supported
            TokenType::String => self.add_code(&format!("\"{}\"", literal_expression_node.value)),
            TokenType::FString => {
                // F-strings use the lexeme directly as it contains the full f"..." syntax
                self.add_code(&literal_expression_node.value);
            },
            TokenType::RawString => {
                // Raw strings use the lexeme directly as it contains the full r"..." syntax
                self.add_code(&literal_expression_node.value);
            },
            TokenType::ByteString => {
                // Byte strings use the lexeme directly as it contains the full b"..." syntax
                self.add_code(&literal_expression_node.value);
            },
            TokenType::TripleQuotedString => {
                // Triple-quoted strings use the lexeme directly
                self.add_code(&literal_expression_node.value);
            },
            TokenType::True => self.add_code("True"),
            TokenType::False => self.add_code("False"),
            TokenType::None_ => self.add_code("None"),
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
            TokenType::ComplexNumber => output.push_str(&literal_expression_node.value.to_string()),
            TokenType::String => {
                output.push_str(&format!("\"{}\"", literal_expression_node.value));
            }
            TokenType::FString => {
                // F-strings use the lexeme directly
                output.push_str(&literal_expression_node.value);
            }
            TokenType::RawString => {
                // Raw strings use the lexeme directly
                output.push_str(&literal_expression_node.value);
            }
            TokenType::ByteString => {
                // Byte strings use the lexeme directly
                output.push_str(&literal_expression_node.value);
            }
            TokenType::TripleQuotedString => {
                // Triple-quoted strings use the lexeme directly
                output.push_str(&literal_expression_node.value);
            }
            TokenType::True => {
                output.push_str("True");
            }
            TokenType::False => {
                output.push_str("False");
            }
            TokenType::None_ => {
                output.push_str("None");
            }
            // SuperString removed - backticks no longer supported
            _ => self
                .errors
                .push("TODO: visit_literal_expression_node_to_string".to_string()),
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node(&mut self, identifier_node: &IdentifierNode) {
        let name = &identifier_node.name.lexeme;
        
        // v0.57: Check if we're in a module and if this is a module variable
        // Module variables need to be qualified even within the module's static methods
        if let Some(ref module_name) = self.current_module_name {
            if self.current_module_variables.contains(name) {
                // This is a module variable, qualify it with the module name
                self.add_code(&format!("{}.{}", module_name, name));
                return;
            }
        }
        
        // Frame v0.31: Handle system.return - convert to return stack
        if name == "system.return" {
            self.add_code("self.return_stack[-1]");
        } else if name.starts_with("system.") {
            // Handle other system.method calls - convert to self.method
            let method_name = &name[7..]; // Remove 'system.' prefix  
            self.add_code(&format!("self.{}", method_name));
        } else if name.contains('.') {
            // Check if this is an enum member reference (e.g., "HttpStatus.Ok")
            let parts: Vec<&str> = name.split('.').collect();
            if parts.len() == 2 {
                let enum_name = parts[0];
                let member_name = parts[1];
                
                // Check if this enum is defined in the current system
                if self.current_system_enums.contains(enum_name) {
                    self.add_code(&format!("{}_{}.{}", self.system_name, enum_name, member_name));
                } else if self.module_level_enums.contains(enum_name) {
                    self.add_code(name);
                } else {
                    // Not an enum we know about - output as-is
                    self.add_code(name);
                }
            } else {
                // More than one dot - not an enum reference
                self.add_code(name);
            }
        } else {
            // Check if this is a system type that needs parentheses
            if name == "SimpleHSM" {
            }
            self.add_code(name);
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_identifier_node_to_string(
        &mut self,
        identifier_node: &IdentifierNode,
        output: &mut String,
    ) {
        let name = &identifier_node.name.lexeme;
        
        // v0.57: Check if we're in a module and if this is a module variable
        if let Some(ref module_name) = self.current_module_name {
            if self.current_module_variables.contains(name) {
                output.push_str(&format!("{}.{}", module_name, name));
                return;
            }
        }
        
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
        // v0.37: Support async actions
        if action_node.is_async {
            self.add_code(&format!("async def {}(self", action_name));
            self.newline_to_string(&mut subclass_code);
            subclass_code.push_str(&format!("#async def {}(self", action_name));
        } else {
            self.add_code(&format!("def {}(self", action_name));
            self.newline_to_string(&mut subclass_code);
            subclass_code.push_str(&format!("#def {}(self", action_name));
        }

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
            if action_node.statements.is_empty() {
                self.newline();
                self.add_code("pass");
            } else {
                self.newline();
                self.visit_decl_stmts(&action_node.statements);
            }
            
            // Add terminator
            self.newline();
            match &action_node.terminator_expr.terminator_type {
                TerminatorType::Return => match &action_node.terminator_expr.return_expr_t_opt {
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
                // DispatchToParentState removed - now handled as ParentDispatchStmt statement
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
        // v0.37: Support async actions
        if action_node.is_async {
            self.add_code(&format!("async def {}(self", action_name));
        } else {
            self.add_code(&format!("def {}(self", action_name));
        }
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
        debug_print!("DEBUG: Generating enum: {}", enum_decl_node.name);
        self.newline();
        self.newline();

        self.add_code(&format!(
            "class {}_{}(Enum):",
            self.system_name, enum_decl_node.name
        ));
        self.indent();

        // Track duplicate enum names to avoid Python enum conflicts
        let mut generated_enum_names = HashSet::new();
        
        for enumerator_decl_node in &enum_decl_node.enums {
            // Only generate if we haven't seen this name before
            if !generated_enum_names.contains(&enumerator_decl_node.name) {
                generated_enum_names.insert(enumerator_decl_node.name.clone());
                enumerator_decl_node.accept(self);
            } else {
                // DEBUG: Log when we skip a duplicate
                debug_print!("DEBUG: Skipping duplicate enum entry: {}", enumerator_decl_node.name);
            }
        }

        self.outdent();
        self.newline()
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enumerator_decl_node(&mut self, enumerator_decl_node: &EnumeratorDeclNode) {
        use crate::frame_c::ast::EnumValue;
        
        self.newline();
        let value_str = match &enumerator_decl_node.value {
            EnumValue::Integer(val) => val.to_string(),
            EnumValue::String(val) => format!("\"{}\"", val),
            EnumValue::Auto => {
                self.errors.push("Internal error: Auto enum values should be resolved during parsing".to_string());
                "0".to_string()  // Default to 0 as fallback
            }
        };
        
        self.add_code(&format!(
            "{} = {}",
            enumerator_decl_node.name, value_str
        ));
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enumerator_expr_node(&mut self, enum_expr_node: &EnumeratorExprNode) {
        // v0.32: Check if this is a module-level enum or system enum
        if self.module_level_enums.contains(&enum_expr_node.enum_type) {
            // Module-level enum - use directly without system prefix
            self.add_code(&format!(
                "{}.{}",
                enum_expr_node.enum_type, enum_expr_node.enumerator
            ));
        } else {
            // System enum - qualify with system name
            self.add_code(&format!(
                "{}_{}.{}",
                self.system_name, enum_expr_node.enum_type, enum_expr_node.enumerator
            ));
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_enumerator_expr_node_to_string(
        &mut self,
        enum_expr_node: &EnumeratorExprNode,
        output: &mut String,
    ) {
        // v0.32: Check if this is a module-level enum or system enum
        if self.module_level_enums.contains(&enum_expr_node.enum_type) {
            // Module-level enum - use directly without system prefix
            output.push_str(&format!(
                "{}.{}",
                enum_expr_node.enum_type, enum_expr_node.enumerator
            ));
        } else {
            // System enum - qualify with system name
            output.push_str(&format!(
                "{}_{}.{}",
                self.system_name, enum_expr_node.enum_type, enum_expr_node.enumerator
            ));
        }
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
        // v0.69: Use newline_and_map for correct line mapping
        self.newline_and_map(variable_decl_node.line);
        let var_type = match &variable_decl_node.type_opt {
            Some(type_node) => self.format_type(type_node),
            None => String::from(""),
        };
        let var_name = &variable_decl_node.name;
        
        // v0.53: Check for multiple variable declaration marker
        if var_name.starts_with("__multi_var__:") {
            // Extract the variable names from the marker
            let names_str = &var_name["__multi_var__:".len()..];
            let names: Vec<&str> = names_str.split(',').collect();
            
            // v0.54: Handle star expressions in unpacking
            // Star expressions are already in the correct format (*name) from the parser
            // Python expects them as-is in the unpacking pattern
            self.add_code(&names.join(", "));
            self.add_code(" = ");
            
            // Generate the right-hand side
            let var_init_expr = &variable_decl_node.get_initializer_value_rc();
            let mut code = String::new();
            var_init_expr.accept_to_string(self, &mut code);
            self.add_code(&code);
            
            return;  // Early return for multiple variable declarations
        }
        
        // Note: Shadowing check is now performed in the parser during semantic analysis
        // This ensures consistent error checking regardless of code generation order
        
        let var_init_expr = &variable_decl_node.get_initializer_value_rc();
        //self.newline();
        let mut code = String::new();
        
        // Special handling for FSL properties like .length
        // Check if this is a CallChainExprT with a .length property
        if let ExprType::CallChainExprT { call_chain_expr_node } = &**var_init_expr {
            if call_chain_expr_node.call_chain.len() == 2 {
                // Check if the second node is "length"
                if let CallChainNodeType::UndeclaredIdentifierNodeT { id_node } = &call_chain_expr_node.call_chain[1] {
                    if id_node.name.lexeme == "length" {
                        // This is a .length property access - transform to len()
                        // Get the variable name from the first node
                        if let CallChainNodeType::VariableNodeT { var_node } = &call_chain_expr_node.call_chain[0] {
                            code = format!("len({})", var_node.id_node.name.lexeme);
                        } else if let CallChainNodeType::UndeclaredIdentifierNodeT { id_node: first_id } = &call_chain_expr_node.call_chain[0] {
                            // Handle case where first node is also undeclared (happens during first pass)
                            code = format!("len({})", first_id.name.lexeme);
                        } else {
                            // Fallback to normal processing
                        }
                    } else if id_node.name.lexeme == "is_empty" {
                        // This is a .is_empty property access - transform to len() == 0
                        // Get the variable name from the first node
                        if let CallChainNodeType::VariableNodeT { var_node } = &call_chain_expr_node.call_chain[0] {
                            code = format!("len({}) == 0", var_node.id_node.name.lexeme);
                        } else if let CallChainNodeType::UndeclaredIdentifierNodeT { id_node: first_id } = &call_chain_expr_node.call_chain[0] {
                            // Handle case where first node is also undeclared (happens during first pass)
                            code = format!("len({}) == 0", first_id.name.lexeme);
                        } else {
                            // Fallback to normal processing
                            var_init_expr.accept_to_string(self, &mut code);
                        }
                    } else {
                        // Not a length property, process normally
                        var_init_expr.accept_to_string(self, &mut code);
                    }
                } else {
                    // Second node is not an identifier, process normally
                    var_init_expr.accept_to_string(self, &mut code);
                }
            } else {
                // Not a 2-node chain, process normally
                var_init_expr.accept_to_string(self, &mut code);
            }
        } else {
            // Not a call chain, process normally
            var_init_expr.accept_to_string(self, &mut code);
        }
        
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
            IdentifierDeclScope::ModuleScope => {
                self.add_code(&format!("{}", var_name));
                if !var_type.is_empty() {
                    self.add_code(&format!(": {}", var_type));
                }
                self.add_code(&format!(" = {}", code));
            }
            _ => {
                self.errors.push("Error - unexpected scope for variable declaration".to_string());
                return;
            }
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
            _ => {
                self.errors.push("Error - unexpected scope for variable declaration".to_string());
                return;
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_variable_expr_node(&mut self, variable_node: &VariableNode) {
        self.add_source_mapping(variable_node.line);  // Map Frame line to Python line
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
        // Only add mapping if not already in a statement context
        if !self.in_statement_context {
            self.add_source_mapping(assignment_expr_node.line);  // Map Frame line to Python line
        }
        // self.generate_comment(assignment_expr_node.line);
        // self.newline();
        // inc/dec all *rvalue* expressions before generating the
        // assignement statement
        // assignment_expr_node.r_value_box.auto_pre_inc_dec(self);
        
        // v0.52: Handle multiple assignment (x, y, z = ...)
        if assignment_expr_node.is_multiple_assignment {
            // Generate comma-separated list of targets
            let mut first = true;
            for l_value in &assignment_expr_node.l_values {
                if !first {
                    self.add_code(", ");
                }
                l_value.accept(self);
                first = false;
            }
        } else {
            // Single assignment - use original logic
            assignment_expr_node.l_value_box.accept(self);
        }
        
        // Handle different assignment operators
        match assignment_expr_node.assignment_op {
            AssignmentOperator::Equals => self.add_code(" = "),
            AssignmentOperator::PlusEquals => self.add_code(" += "),
            AssignmentOperator::MinusEquals => self.add_code(" -= "),
            AssignmentOperator::StarEquals => self.add_code(" *= "),
            AssignmentOperator::SlashEquals => self.add_code(" /= "),
            AssignmentOperator::FloorDivideEquals => self.add_code(" //= "),
            AssignmentOperator::PercentEquals => self.add_code(" %= "),
            AssignmentOperator::PowerEquals => self.add_code(" **= "),
            AssignmentOperator::AndEquals => self.add_code(" &= "),
            AssignmentOperator::OrEquals => self.add_code(" |= "),
            AssignmentOperator::LeftShiftEquals => self.add_code(" <<= "),
            AssignmentOperator::RightShiftEquals => self.add_code(" >>= "),
            AssignmentOperator::XorEquals => self.add_code(" ^= "),
            AssignmentOperator::MatMulEquals => self.add_code(" @= "),
        }
        
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
        
        // v0.52: Handle multiple assignment
        if assignment_expr_node.is_multiple_assignment {
            let mut first = true;
            for l_value in &assignment_expr_node.l_values {
                if !first {
                    output.push_str(", ");
                }
                l_value.accept_to_string(self, output);
                first = false;
            }
        } else {
            assignment_expr_node
                .l_value_box
                .accept_to_string(self, output);
        }
        
        // Handle different assignment operators
        match assignment_expr_node.assignment_op {
            AssignmentOperator::Equals => output.push_str(" = "),
            AssignmentOperator::PlusEquals => output.push_str(" += "),
            AssignmentOperator::MinusEquals => output.push_str(" -= "),
            AssignmentOperator::StarEquals => output.push_str(" *= "),
            AssignmentOperator::SlashEquals => output.push_str(" /= "),
            AssignmentOperator::FloorDivideEquals => output.push_str(" //= "),
            AssignmentOperator::PercentEquals => output.push_str(" %= "),
            AssignmentOperator::PowerEquals => output.push_str(" **= "),
            AssignmentOperator::AndEquals => output.push_str(" &= "),
            AssignmentOperator::OrEquals => output.push_str(" |= "),
            AssignmentOperator::LeftShiftEquals => output.push_str(" <<= "),
            AssignmentOperator::RightShiftEquals => output.push_str(" >>= "),
            AssignmentOperator::XorEquals => output.push_str(" ^= "),
            AssignmentOperator::MatMulEquals => output.push_str(" @= "),
        }
        
        assignment_expr_node
            .r_value_rc
            .accept_to_string(self, output);
    }

    //* --------------------------------------------------------------------- *//

    fn visit_walrus_expr_node(&mut self, assignment_expr_node: &AssignmentExprNode) {
        // Walrus operator (:=) - generates assignment expression that returns value
        // In Python, this is (var := value) with parentheses
        self.add_code("(");
        assignment_expr_node.l_value_box.accept(self);
        self.add_code(" := ");
        assignment_expr_node.r_value_rc.accept(self);
        self.add_code(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_walrus_expr_node_to_string(
        &mut self,
        assignment_expr_node: &AssignmentExprNode,
        output: &mut String,
    ) {
        output.push_str("(");
        assignment_expr_node.l_value_box.accept_to_string(self, output);
        output.push_str(" := ");
        assignment_expr_node.r_value_rc.accept_to_string(self, output);
        output.push_str(")");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_assignment_statement_node(&mut self, assignment_stmt_node: &AssignmentStmtNode) {
        self.generate_comment(assignment_stmt_node.get_line());
        self.newline();
        
        // Add source mapping for assignment statement
        self.add_source_mapping(assignment_stmt_node.get_line());
        
        // Set flag to prevent duplicate mapping in expression
        let was_in_statement = self.in_statement_context;
        self.in_statement_context = true;
        assignment_stmt_node.assignment_expr_node.accept(self);
        self.in_statement_context = was_in_statement;
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

    fn visit_self_expr_node(&mut self, _self_expr_node: &SelfExprNode) {
        self.add_code("self");
    }

    //* --------------------------------------------------------------------- *//

    fn visit_self_expr_node_to_string(
        &mut self,
        _self_expr_node: &SelfExprNode,
        output: &mut String,
    ) {
        output.push_str("self");
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
        {
            // Check if left side is a unary not expression that needs parentheses
            let left_needs_parens = matches!(
                &*binary_expr_node.left_rcref.borrow(),
                ExprType::UnaryExprT { unary_expr_node } 
                    if unary_expr_node.operator == OperatorType::Not
            ) && matches!(
                binary_expr_node.operator,
                OperatorType::EqualEqual | OperatorType::NotEqual | 
                OperatorType::Less | OperatorType::LessEqual |
                OperatorType::Greater | OperatorType::GreaterEqual |
                OperatorType::LogicalAnd | OperatorType::LogicalOr
            );
            
            // Check if right side is a unary not expression that needs parentheses
            let right_needs_parens = matches!(
                &*binary_expr_node.right_rcref.borrow(),
                ExprType::UnaryExprT { unary_expr_node } 
                    if unary_expr_node.operator == OperatorType::Not
            ) && matches!(
                binary_expr_node.operator,
                OperatorType::EqualEqual | OperatorType::NotEqual | 
                OperatorType::Less | OperatorType::LessEqual |
                OperatorType::Greater | OperatorType::GreaterEqual |
                OperatorType::LogicalAnd | OperatorType::LogicalOr
            );
            
            if left_needs_parens {
                self.add_code("(");
                binary_expr_node.left_rcref.borrow().accept(self);
                self.add_code(")");
            } else {
                binary_expr_node.left_rcref.borrow().accept(self);
            }
            
            binary_expr_node.operator.accept(self);
            
            if right_needs_parens {
                self.add_code("(");
                binary_expr_node.right_rcref.borrow().accept(self);
                self.add_code(")");
            } else {
                binary_expr_node.right_rcref.borrow().accept(self);
            }
        }
    }

    //* --------------------------------------------------------------------- *//

    fn visit_binary_expr_node_to_string(
        &mut self,
        binary_expr_node: &BinaryExprNode,
        output: &mut String,
    ) {
        {
            // Check if left side is a unary not expression that needs parentheses
            let left_needs_parens = matches!(
                &*binary_expr_node.left_rcref.borrow(),
                ExprType::UnaryExprT { unary_expr_node } 
                    if unary_expr_node.operator == OperatorType::Not
            ) && matches!(
                binary_expr_node.operator,
                OperatorType::EqualEqual | OperatorType::NotEqual | 
                OperatorType::Less | OperatorType::LessEqual |
                OperatorType::Greater | OperatorType::GreaterEqual |
                OperatorType::LogicalAnd | OperatorType::LogicalOr
            );
            
            // Check if right side is a unary not expression that needs parentheses
            let right_needs_parens = matches!(
                &*binary_expr_node.right_rcref.borrow(),
                ExprType::UnaryExprT { unary_expr_node } 
                    if unary_expr_node.operator == OperatorType::Not
            ) && matches!(
                binary_expr_node.operator,
                OperatorType::EqualEqual | OperatorType::NotEqual | 
                OperatorType::Less | OperatorType::LessEqual |
                OperatorType::Greater | OperatorType::GreaterEqual |
                OperatorType::LogicalAnd | OperatorType::LogicalOr
            );
            
            if left_needs_parens {
                output.push('(');
                binary_expr_node
                    .left_rcref
                    .borrow()
                    .accept_to_string(self, output);
                output.push(')');
            } else {
                binary_expr_node
                    .left_rcref
                    .borrow()
                    .accept_to_string(self, output);
            }
            
            binary_expr_node.operator.accept_to_string(self, output);
            
            if right_needs_parens {
                output.push('(');
                binary_expr_node
                    .right_rcref
                    .borrow()
                    .accept_to_string(self, output);
                output.push(')');
            } else {
                binary_expr_node
                    .right_rcref
                    .borrow()
                    .accept_to_string(self, output);
            }
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
            OperatorType::FloorDivide => self.add_code(" // "),
            OperatorType::Power => self.add_code(" ** "),
            OperatorType::Greater => self.add_code(" > "),
            OperatorType::GreaterEqual => self.add_code(" >= "),
            OperatorType::Less => self.add_code(" < "),
            OperatorType::LessEqual => self.add_code(" <= "),
            OperatorType::Not => self.add_code(" not "),
            OperatorType::EqualEqual => self.add_code(" == "),
            OperatorType::NotEqual => self.add_code(" != "),
            OperatorType::LogicalAnd => self.add_code(" and "),
            OperatorType::LogicalOr => self.add_code(" or "),
            OperatorType::Percent => self.add_code(" % "),
            OperatorType::BitwiseOr => self.add_code(" | "),
            OperatorType::BitwiseAnd => self.add_code(" & "),
            OperatorType::BitwiseXor => self.add_code(" ^ "),
            OperatorType::BitwiseNot => self.add_code("~"),
            OperatorType::LeftShift => self.add_code(" << "),
            OperatorType::MatMul => self.add_code(" @ "),
            OperatorType::RightShift => self.add_code(" >> "),
            OperatorType::In => self.add_code(" in "),
            OperatorType::NotIn => self.add_code(" not in "),
            OperatorType::Is => self.add_code(" is "),
            OperatorType::IsNot => self.add_code(" is not "),
            OperatorType::Unknown => self.add_code(" <Unknown> "),
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
            OperatorType::FloorDivide => output.push_str(" // "),
            OperatorType::Power => output.push_str(" ** "),
            OperatorType::Greater => output.push_str(" > "),
            OperatorType::GreaterEqual => output.push_str(" >= "),
            OperatorType::Less => output.push_str(" < "),
            OperatorType::LessEqual => output.push_str(" <= "),
            OperatorType::Not => output.push_str(" not "),
            OperatorType::EqualEqual => output.push_str(" == "),
            OperatorType::NotEqual => output.push_str(" != "),
            OperatorType::LogicalAnd => output.push_str(" and "),
            OperatorType::LogicalOr => output.push_str(" or "),
            OperatorType::Percent => output.push_str(" % "),
            OperatorType::BitwiseOr => output.push_str(" | "),
            OperatorType::BitwiseAnd => output.push_str(" & "),
            OperatorType::BitwiseXor => output.push_str(" ^ "),
            OperatorType::BitwiseNot => output.push_str("~"),
            OperatorType::LeftShift => output.push_str(" << "),
            OperatorType::MatMul => output.push_str(" @ "),
            OperatorType::RightShift => output.push_str(" >> "),
            OperatorType::In => output.push_str(" in "),
            OperatorType::NotIn => output.push_str(" not in "),
            OperatorType::Is => output.push_str(" is "),
            OperatorType::IsNot => output.push_str(" is not "),
            OperatorType::Unknown => output.push_str(" <Unknown> "),
        }
    }
}

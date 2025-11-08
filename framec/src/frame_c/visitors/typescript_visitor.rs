// TypeScript Visitor for Frame Language Transpiler
// Generates TypeScript code from Frame AST
// v0.82.0 - Initial TypeScript support with state machines, transitions, and expressions

use super::*;
use crate::frame_c::ast::FrameEventPart;
use crate::frame_c::ast::*;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::native_region_segmenter::BodySegment;
use crate::frame_c::scanner::{TargetRegion, TokenType};
use crate::frame_c::symbol_table::{Arcanum, SymbolConfig};
use crate::frame_c::target_parsers::typescript::{TypeScriptTargetAst, TypeScriptTargetElement};
use crate::frame_c::target_parsers::ParsedTargetBlock;
use convert_case::{Case, Casing};
use regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

// B2 MixedBody/MIR directive emission (offline fallback: textual composition)

// SWC codegen imports for B2 emission of MIR
// (B2 planned) SWC codegen can be enabled later for MIR emission
// SWC B2 emission will be enabled after we pin codegen deps.

#[derive(Clone, Debug)]
struct TypeScriptNativeBinding {
    identifier: Option<String>,
    import_entries: Vec<String>,
    segments: Vec<String>,
}

const DEFAULT_RUNTIME_IMPORTS: &[&str] = &[
    "FrameRuntime",
    "FrameCollections",
    "FrameCounter",
    "FrameDict",
    "FrameMath",
    "FrameString",
    "FrameEvent",
    "FrameCompartment",
    "OrderedDict",
    "ChainMap",
    "FrameSocketClient",
    "configparser",
    "json",
    "numpy",
    "os",
    "random",
    "signal",
    "sys",
    "time",
    "open",
];

pub struct TypeScriptVisitor {
    pub builder: CodeBuilder,
    system_name: String,

    // Symbol table and config (like Python visitor)
    symbol_config: SymbolConfig,
    arcanum: Vec<Arcanum>,

    // Context tracking (reduced manual tracking since we have arcanum)
    current_state_name: Option<String>,
    current_class_name_opt: Option<String>, // Track Frame class context like Python visitor
    domain_variables: HashSet<String>,      // Track domain variable names
    current_handler_params: HashSet<String>, // Track current event handler parameter names
    current_state_params: HashSet<String>,  // Track current state's parameter names
    current_state_vars: HashSet<String>,    // Track current state's variable names
    // TODO: Remove current_local_vars once arcanum-based resolution is implemented
    current_local_vars: HashSet<String>, // Track local variables in current handler
    current_exception_vars: HashSet<String>, // Track exception variables in current try-catch block
    counter_variables: HashSet<String>,  // Track variables referencing Counter instances
    action_names: HashSet<String>,       // Track action names for proper call resolution
    operation_names: HashSet<String>,    // Track operation names for proper call resolution
    declared_enums: HashSet<String>,     // Track declared enum names to avoid duplicates
    is_in_action: bool, // Track if we're currently processing an action (vs event handler)
    current_event_handler_default_return_value: Option<String>, // Track event handler default return value
    node_module_imports: HashSet<String>, // Track required Node.js module imports

    // Control flags for multifile compilation
    generate_runtime_classes: bool, // Whether to generate Frame runtime classes (false for multifile modules)
    in_module_function: bool,       // Flag to track when generating module functions
    state_var_initializers: HashMap<String, String>, // Cached state variable initializers per state
    pending_frame_math_property: bool,
    pending_super_call: bool,
    unpack_temp_counter: usize,
    module_variables: HashSet<String>, // Track module-level variables for proper scoping
    pending_class_method: bool,
    pending_class_method_param: Option<String>,
    state_param_names: HashMap<String, Vec<String>>,
    in_state_var_initializer: bool,
    class_method_names: HashMap<String, HashSet<String>>,
    walrus_declared_locals: HashSet<String>,
    pending_walrus_decls: Vec<String>,
    in_generator_function: bool,
    system_has_async_runtime: bool,
    // Bug 53 fix: Track dynamic properties and runtime function calls
    dynamic_properties: HashSet<String>, // Track properties assigned with self.propertyName
    runtime_function_calls: HashSet<String>, // Track frameRuntime* function calls

    // Target-specific region metadata
    target_regions: Arc<Vec<TargetRegion>>,
    native_module_bindings: HashMap<String, TypeScriptNativeBinding>,
    runtime_imports: BTreeSet<String>,
    // Parent lookup: formatted state name -> formatted parent state name (if any)
    parent_state_map: HashMap<String, Option<String>>,
}

struct NodeApiActionMapping {
    return_type: &'static str,
    force_async: Option<bool>,
    statements: Vec<String>,
    return_expression: Option<String>,
}

impl TypeScriptVisitor {
    fn state_param_names_for(&self, raw_state_name: &str) -> Vec<String> {
        if let Some(v) = self.state_param_names.get(raw_state_name) { return v.clone(); }
        let prefixed = format!("${}", raw_state_name);
        if let Some(v) = self.state_param_names.get(&prefixed) { return v.clone(); }
        let formatted = self.format_state_name(raw_state_name);
        if let Some(v) = self.state_param_names.get(&formatted) { return v.clone(); }
        Vec::new()
    }
    pub fn new(arcanum: Vec<Arcanum>, symbol_config: SymbolConfig) -> Self {
        let runtime_imports: BTreeSet<String> = DEFAULT_RUNTIME_IMPORTS
            .iter()
            .map(|s| s.to_string())
            .collect();

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
            counter_variables: HashSet::new(),
            action_names: HashSet::new(),
            operation_names: HashSet::new(),
            declared_enums: HashSet::new(),
            is_in_action: false,
            current_event_handler_default_return_value: None,
            node_module_imports: HashSet::new(),
            // Default: do not embed runtime; import shared runtime instead
            generate_runtime_classes: false,
            in_module_function: false,      // Default: not in module function
            state_var_initializers: HashMap::new(),
            pending_frame_math_property: false,
            pending_super_call: false,
            unpack_temp_counter: 0,
            module_variables: HashSet::new(),
            pending_class_method: false,
            pending_class_method_param: None,
            state_param_names: HashMap::new(),
            in_state_var_initializer: false,
            class_method_names: HashMap::new(),
            walrus_declared_locals: HashSet::new(),
            pending_walrus_decls: Vec::new(),
            in_generator_function: false,
            system_has_async_runtime: false,
            // Bug 53 fix: Initialize property and function tracking
            dynamic_properties: HashSet::new(),
            runtime_function_calls: HashSet::new(),
            target_regions: Arc::new(Vec::new()),
            native_module_bindings: HashMap::new(),
            runtime_imports,
            parent_state_map: HashMap::new(),
        }
    }

    fn needs_computed_static(name: &str) -> bool {
        matches!(
            name,
            "name" | "length" | "prototype" | "caller" | "arguments" | "apply" | "bind" | "call"
        )
    }

    /// Create a new TypeScript visitor for multifile compilation (without runtime classes)
    pub fn new_for_multifile(arcanum: Vec<Arcanum>, symbol_config: SymbolConfig) -> Self {
        let mut visitor = Self::new(arcanum, symbol_config);
        // For multifile, do not embed runtime in each module; linker/main will import it once
        visitor.generate_runtime_classes = false;
        visitor.in_module_function = false; // Initialize module function flag
        visitor.pending_super_call = false;
        visitor
    }

    fn register_native_modules(&mut self, frame_module: &FrameModule) {
        for module_rcref in &frame_module.native_modules {
            let module = module_rcref.borrow();
            if module.qualified_name.is_empty() {
                continue;
            }

            let key = module.path();
            if self.native_module_bindings.contains_key(&key) {
                continue;
            }

            if let Some(binding) = self.resolve_native_binding(&module.qualified_name) {
                for entry in &binding.import_entries {
                    self.runtime_imports.insert(entry.clone());
                }
                self.native_module_bindings.insert(key, binding);
            }
        }
    }

    fn resolve_native_binding(&self, segments: &[String]) -> Option<TypeScriptNativeBinding> {
        if segments.is_empty() {
            return None;
        }

        let path = segments.join("/");
        match path.as_str() {
            "runtime/socket" | "runtime_socket" => Some(TypeScriptNativeBinding {
                identifier: None,
                import_entries: vec![
                    "FrameSocketClient".to_string(),
                    "frame_socket_client_connect".to_string(),
                    "frame_socket_client_read_line".to_string(),
                    "frame_socket_client_write_line".to_string(),
                    "frame_socket_client_close".to_string(),
                ],
                segments: segments.to_vec(),
            }),
            "runtime/json" => Some(TypeScriptNativeBinding {
                identifier: Some("json".to_string()),
                import_entries: vec!["json".to_string()],
                segments: segments.to_vec(),
            }),
            "runtime/os" => Some(TypeScriptNativeBinding {
                identifier: Some("os".to_string()),
                import_entries: vec!["os".to_string()],
                segments: segments.to_vec(),
            }),
            "runtime/random" => Some(TypeScriptNativeBinding {
                identifier: Some("random".to_string()),
                import_entries: vec!["random".to_string()],
                segments: segments.to_vec(),
            }),
            "runtime/signal" => Some(TypeScriptNativeBinding {
                identifier: Some("signal".to_string()),
                import_entries: vec!["signal".to_string()],
                segments: segments.to_vec(),
            }),
            "runtime/sys" => Some(TypeScriptNativeBinding {
                identifier: Some("sys".to_string()),
                import_entries: vec!["sys".to_string()],
                segments: segments.to_vec(),
            }),
            "runtime/time" => Some(TypeScriptNativeBinding {
                identifier: Some("time".to_string()),
                import_entries: vec!["time".to_string()],
                segments: segments.to_vec(),
            }),
            "runtime/configparser" => Some(TypeScriptNativeBinding {
                identifier: Some("configparser".to_string()),
                import_entries: vec!["configparser".to_string()],
                segments: segments.to_vec(),
            }),
            "runtime/open" => Some(TypeScriptNativeBinding {
                identifier: Some("open".to_string()),
                import_entries: vec!["open".to_string()],
                segments: segments.to_vec(),
            }),
            _ => {
                let identifier = segments
                    .last()
                    .map(|s| s.to_case(Case::UpperCamel))
                    .unwrap_or_else(|| "FrameRuntime".to_string());
                Some(TypeScriptNativeBinding {
                    identifier: Some(identifier.clone()),
                    import_entries: vec![identifier],
                    segments: segments.to_vec(),
                })
            }
        }
    }

    fn match_native_module_binding(
        &self,
        node: &CallChainExprNode,
        start_index: usize,
    ) -> Option<(&TypeScriptNativeBinding, usize)> {
        for binding in self.native_module_bindings.values() {
            if node.call_chain.len() < start_index + binding.segments.len() {
                continue;
            }

            let mut matches = true;
            for (offset, segment) in binding.segments.iter().enumerate() {
                let chain_node = &node.call_chain[start_index + offset];
                let name = match chain_node {
                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                        &id_node.name.lexeme
                    }
                    CallChainNodeType::VariableNodeT { var_node } => &var_node.id_node.name.lexeme,
                    _ => {
                        matches = false;
                        break;
                    }
                };
                if name != segment {
                    matches = false;
                    break;
                }
            }

            if matches {
                return Some((binding, binding.segments.len()));
            }
        }
        None
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

    /// Embed Frame TypeScript runtime library at the beginning of generated files
    fn embed_frame_runtime(&mut self) {
        let runtime_code = include_str!("../../../../frame_runtime_ts/index.ts");
        self.builder.writeln("// Frame TypeScript Runtime Library");
        self.builder
            .writeln("// Provides Frame-semantic implementations for consistent behavior");
        self.builder.newline();
        self.builder.write(runtime_code);
        self.builder.newline();
        self.builder.newline();
    }
    fn build_state_vars_dict(&mut self, state_node: &StateNode) -> String {
        let param_names = if let Some(params) = &state_node.params_opt {
            params.iter().map(|p| p.param_name.clone()).collect()
        } else {
            Vec::new()
        };
        if !param_names.is_empty() {
            self.state_param_names
                .insert(state_node.name.clone(), param_names.clone());
            let trimmed_key = state_node.name.trim_start_matches('$').to_string();
            self.state_param_names
                .insert(trimmed_key, param_names.clone());
            let formatted_key = self.format_state_name(&state_node.name);
            self.state_param_names
                .insert(formatted_key, param_names.clone());
            if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                eprintln!(
                    "DEBUG TS: Registered state params for '{}' -> {:?}",
                    state_node.name, param_names
                );
            }
        }

        let result = if let Some(state_vars) = &state_node.vars_opt {
            if state_vars.is_empty() {
                return "{}".to_string();
            }

            let saved_local = self.current_local_vars.clone();
            let saved_state_vars = self.current_state_vars.clone();
            let saved_state_params = self.current_state_params.clone();
            let saved_handler_params = self.current_handler_params.clone();
            let saved_exception_vars = self.current_exception_vars.clone();

            self.current_local_vars.clear();
            self.current_state_vars.clear();
            self.current_state_params.clear();
            self.current_handler_params.clear();
            self.current_exception_vars.clear();

            let saved_state_flag = self.in_state_var_initializer;
            self.in_state_var_initializer = true;

            if let Some(params) = &state_node.params_opt {
                for param in params {
                    self.current_state_params.insert(param.param_name.clone());
                }
            }

            let mut entries = Vec::new();
            for var_rcref in state_vars {
                let var = var_rcref.borrow();
                let mut value_str = String::new();
                self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                entries.push(format!("'{}': {}", var.name, value_str));
            }

            self.current_local_vars = saved_local;
            self.current_state_vars = saved_state_vars;
            self.current_state_params = saved_state_params;
            self.current_handler_params = saved_handler_params;
            self.current_exception_vars = saved_exception_vars;
            self.in_state_var_initializer = saved_state_flag;
            format!("{{{}}}", entries.join(", "))
        } else {
            "{}".to_string()
        };
        result
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
                    ExprStmtType::AssignmentStmtT {
                        assignment_stmt_node,
                    } => self.expr_has_await(&assignment_stmt_node.assignment_expr_node.r_value_rc),
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
                self.expr_has_await(&binary_expr_node.left_rcref.borrow())
                    || self.expr_has_await(&binary_expr_node.right_rcref.borrow())
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
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
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
                        crate::frame_c::ast::CallChainNodeType::ActionCallT {
                            action_call_expr_node,
                        } => {
                            for arg in &action_call_expr_node.call_expr_list.exprs_t {
                                if self.expr_has_await(arg) {
                                    return true;
                                }
                            }
                        }
                        crate::frame_c::ast::CallChainNodeType::OperationCallT {
                            operation_call_expr_node,
                        } => {
                            for arg in &operation_call_expr_node.call_expr_list.exprs_t {
                                if self.expr_has_await(arg) {
                                    return true;
                                }
                            }
                        }
                        crate::frame_c::ast::CallChainNodeType::InterfaceMethodCallT {
                            interface_method_call_expr_node,
                        } => {
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

    fn system_needs_async_runtime(&self, system_node: &SystemNode) -> bool {
        if let Some(interface_block) = &system_node.interface_block_node_opt {
            for method_rcref in &interface_block.interface_methods {
                if method_rcref.borrow().is_async {
                    return true;
                }
            }
        }

        if let Some(machine_block) = &system_node.machine_block_node_opt {
            for state_rcref in &machine_block.states {
                let state = state_rcref.borrow();
                for handler_rcref in &state.evt_handlers_rcref {
                    let handler = handler_rcref.borrow();
                    if handler.is_async {
                        return true;
                    }
                    for stmt_or_decl in &handler.statements {
                        if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                            if self.statement_has_await(stmt_t) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Bug 53 fix: Analyze system AST to collect dynamic properties and runtime function calls
    fn analyze_system_for_dynamic_members(&mut self, system_node: &SystemNode) {
        // Reset collections
        self.dynamic_properties.clear();
        self.runtime_function_calls.clear();

        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
            eprintln!(
                "Bug 53: Analyzing system '{}' for dynamic members",
                system_node.name
            );
        }

        // Analyze machine block for property assignments and function calls
        if let Some(machine_block) = &system_node.machine_block_node_opt {
            for state_rcref in &machine_block.states {
                let state = state_rcref.borrow();

                // Analyze event handlers
                for handler_rcref in &state.evt_handlers_rcref {
                    let handler = handler_rcref.borrow();
                    for stmt_or_decl in &handler.statements {
                        self.analyze_statement_for_dynamic_members(stmt_or_decl);
                    }
                }
            }
        }

        // Analyze actions block
        if let Some(actions_block) = &system_node.actions_block_node_opt {
            for action_rcref in &actions_block.actions {
                let action = action_rcref.borrow();
                for stmt_or_decl in &action.statements {
                    self.analyze_statement_for_dynamic_members(stmt_or_decl);
                }
            }
        }
    }

    /// Recursively analyze statements for self.property assignments and frameRuntime* calls
    fn analyze_statement_for_dynamic_members(&mut self, stmt_or_decl: &DeclOrStmtType) {
        match stmt_or_decl {
            DeclOrStmtType::StmtT { stmt_t } => {
                match stmt_t {
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!("Bug 53: Analyzing ExpressionStmt");
                        }
                        match expr_stmt_t {
                            ExprStmtType::AssignmentStmtT {
                                assignment_stmt_node,
                            } => {
                                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default()
                                    == "1"
                                {
                                    eprintln!("Bug 53: Found AssignmentStmtT");
                                }
                                // Check for self.property assignments
                                self.analyze_assignment_for_properties(
                                    &assignment_stmt_node.assignment_expr_node,
                                );
                                // Check for frameRuntime calls in RHS
                                self.analyze_expression_for_runtime_calls(
                                    &assignment_stmt_node.assignment_expr_node.r_value_rc,
                                );
                            }
                            ExprStmtType::CallStmtT { call_stmt_node } => {
                                // Check if it's a frameRuntime function call
                                let method_name =
                                    &call_stmt_node.call_expr_node.identifier.name.lexeme;
                                if method_name.starts_with("frameRuntime") {
                                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default()
                                        == "1"
                                    {
                                        eprintln!(
                                            "Bug 53: Found frameRuntime function call: {}",
                                            method_name
                                        );
                                    }
                                    self.runtime_function_calls.insert(method_name.clone());
                                }
                            }
                            _ => {
                                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default()
                                    == "1"
                                {
                                    eprintln!(
                                        "Bug 53: Found other expr_stmt_t type: {:?}",
                                        std::mem::discriminant(expr_stmt_t)
                                    );
                                }
                                // For other expression statement types
                            }
                        }
                    }
                    _ => {
                        // For other statement types, we could add more specific analysis if needed
                    }
                }
            }
            _ => {
                // Handle other types if needed
            }
        }
    }

    /// Analyze assignment expressions for self.property patterns
    fn analyze_assignment_for_properties(&mut self, assignment_expr: &AssignmentExprNode) {
        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
            eprintln!("Bug 53: Analyzing assignment l_value");
        }
        if let ExprType::CallChainExprT {
            call_chain_expr_node,
        } = &*assignment_expr.l_value_box
        {
            if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                eprintln!(
                    "Bug 53: Found CallChainExprT with {} nodes",
                    call_chain_expr_node.call_chain.len()
                );
            }
            if call_chain_expr_node.call_chain.len() == 2 {
                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                    eprintln!("Bug 53: Checking 2-node call chain for self.property pattern");
                }
                // Check for self.property pattern - try different combinations
                let first_is_self = match &call_chain_expr_node.call_chain[0] {
                    CallChainNodeType::SelfT { .. } => {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!("Bug 53: Found SelfT node");
                        }
                        true
                    }
                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!(
                                "Bug 53: Found UndeclaredIdentifierNodeT: {}",
                                id_node.name.lexeme
                            );
                        }
                        id_node.name.lexeme == "self"
                    }
                    CallChainNodeType::VariableNodeT { var_node } => {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!(
                                "Bug 53: Found VariableNodeT: {}",
                                var_node.id_node.name.lexeme
                            );
                        }
                        var_node.id_node.name.lexeme == "self"
                    }
                    _ => {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!(
                                "Bug 53: First node is not self-related: {:?}",
                                std::mem::discriminant(&call_chain_expr_node.call_chain[0])
                            );
                        }
                        false
                    }
                };

                if first_is_self {
                    // Try to get property name from second node
                    let property_name = match &call_chain_expr_node.call_chain[1] {
                        CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                            Some(id_node.name.lexeme.clone())
                        }
                        CallChainNodeType::VariableNodeT { var_node } => {
                            Some(var_node.id_node.name.lexeme.clone())
                        }
                        _ => None,
                    };

                    if let Some(prop_name) = property_name {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!(
                                "Bug 53: Found dynamic property assignment: self.{}",
                                prop_name
                            );
                        }
                        self.dynamic_properties.insert(prop_name);
                    }
                }
            }
        } else {
            if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                eprintln!(
                    "Bug 53: l_value is not CallChainExprT, it's: {:?}",
                    std::mem::discriminant(&*assignment_expr.l_value_box)
                );
            }
        }
    }

    /// Recursively analyze expressions for frameRuntime* function calls
    fn analyze_expression_for_runtime_calls(&mut self, expr: &ExprType) {
        match expr {
            ExprType::CallExprT { call_expr_node } => {
                let method_name = &call_expr_node.identifier.name.lexeme;
                if method_name.starts_with("frameRuntime") {
                    self.runtime_function_calls.insert(method_name.clone());
                }
                // Analyze arguments recursively
                for arg in &call_expr_node.call_expr_list.exprs_t {
                    self.analyze_expression_for_runtime_calls(arg);
                }
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                for call_chain_node in &call_chain_expr_node.call_chain {
                    if let CallChainNodeType::UndeclaredCallT { call_node } = call_chain_node {
                        let method_name = &call_node.identifier.name.lexeme;
                        if method_name.starts_with("frameRuntime") {
                            self.runtime_function_calls.insert(method_name.clone());
                        }
                        // Analyze arguments recursively
                        for arg in &call_node.call_expr_list.exprs_t {
                            self.analyze_expression_for_runtime_calls(arg);
                        }
                    }
                }
            }
            _ => {
                // For other expression types, we could add more analysis if needed
            }
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

    fn action_has_any_return(&self, action: &ActionNode) -> bool {
        for stmt_or_decl in &action.statements {
            if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                if self.statement_has_any_return(stmt_t) {
                    return true;
                }
            }
        }
        false
    }

    fn statement_has_any_return(&self, stmt: &StatementType) -> bool {
        match stmt {
            StatementType::IfStmt { if_stmt_node } => {
                for stmt_or_decl in &if_stmt_node.if_block.statements {
                    if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                        if self.statement_has_any_return(stmt_t) {
                            return true;
                        }
                    }
                }
                for elif_branch in &if_stmt_node.elif_clauses {
                    for stmt_or_decl in &elif_branch.block.statements {
                        if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                            if self.statement_has_any_return(stmt_t) {
                                return true;
                            }
                        }
                    }
                }
                if let Some(else_branch) = &if_stmt_node.else_block {
                    for stmt_or_decl in &else_branch.statements {
                        if let DeclOrStmtType::StmtT { stmt_t } = stmt_or_decl {
                            if self.statement_has_any_return(stmt_t) {
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
                        if self.statement_has_any_return(stmt_t) {
                            return true;
                        }
                    }
                }
                false
            }
            StatementType::ReturnStmt { .. } => true,
            _ => false,
        }
    }

    fn format_function_ref(&self, name: &str) -> String {
        let parts: Vec<&str> = name.split('.').collect();
        if parts.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        for (index, part) in parts.iter().enumerate() {
            if index > 0 {
                result.push('.');
            }

            match *part {
                "self" | "system" => {
                    if index == 0 {
                        result.push_str("this");
                    } else {
                        result.push_str("this");
                    }
                }
                _ => result.push_str(part),
            }
        }

        result
    }

    fn require_node_module(&mut self, module: &str) {
        let m = module.trim();
        if m.is_empty() {
            return;
        }
        self.node_module_imports.insert(m.to_string());
    }

    fn sanitize_identifier(identifier: &str) -> String {
        if identifier.is_empty() {
            return "_imported".to_string();
        }

        let mut result = identifier
            .chars()
            .map(|ch| {
                if ch.is_alphanumeric() || ch == '_' {
                    ch
                } else {
                    '_'
                }
            })
            .collect::<String>();

        if result
            .chars()
            .next()
            .map(|ch| ch.is_ascii_digit())
            .unwrap_or(false)
        {
            result.insert(0, '_');
        }

        if result.is_empty() {
            "_imported".to_string()
        } else {
            result
        }
    }

    fn frame_path_to_typescript_module(file_path: &str) -> String {
        let path = Path::new(file_path);
        let without_ext = path.with_extension("");
        let mut module_path = without_ext.to_string_lossy().replace('\\', "/");

        if module_path.is_empty() {
            return ".".to_string();
        }

        if !module_path.starts_with('.') && !module_path.starts_with('/') {
            module_path = format!("./{}", module_path);
        }

        module_path
    }

    /// Map well-known Python-style imports onto Frame TypeScript runtime helpers.
    fn map_builtin_simple(module: &str, alias_opt: Option<&str>) -> Option<String> {
        let sanitized_module = Self::sanitize_identifier(module);
        let alias_string = alias_opt.map(Self::sanitize_identifier);

        match module {
            "json" => match alias_string {
                Some(ref alias) if alias != &sanitized_module => {
                    Some(format!("const {} = FrameRuntime.json;", alias))
                }
                _ => Some("// FrameRuntime.json provides 'json' module bindings".to_string()),
            },
            "random" => match alias_string {
                Some(ref alias) if alias != &sanitized_module => {
                    Some(format!("const {} = random;", alias))
                }
                _ => Some("// Frame runtime exposes 'random' module helpers".to_string()),
            },
            "time" => match alias_string {
                Some(ref alias) if alias != &sanitized_module => {
                    Some(format!("const {} = time;", alias))
                }
                _ => Some("// Frame runtime exposes 'time' module helpers".to_string()),
            },
            "sys" => match alias_string {
                Some(ref alias) if alias != &sanitized_module => {
                    Some(format!("const {} = sys;", alias))
                }
                _ => Some("// Frame runtime exposes 'sys' module helpers".to_string()),
            },
            "signal" => match alias_string {
                Some(ref alias) if alias != &sanitized_module => {
                    Some(format!("const {} = signal;", alias))
                }
                _ => Some("// Frame runtime exposes 'signal' module helpers".to_string()),
            },
            "configparser" => match alias_string {
                Some(ref alias) if alias != &sanitized_module => {
                    Some(format!("const {} = configparser;", alias))
                }
                _ => Some("// Frame runtime exposes 'configparser' module helpers".to_string()),
            },
            "numpy" => match alias_string {
                Some(ref alias) if alias != &sanitized_module => {
                    Some(format!("const {} = numpy;", alias))
                }
                _ => Some("// Frame runtime exposes 'numpy' helpers".to_string()),
            },
            "collections" => {
                let target_alias = alias_string
                    .clone()
                    .unwrap_or_else(|| sanitized_module.clone());
                let mut lines = Vec::new();
                lines.push(format!("const {} = {{", target_alias));
                lines.push("    defaultdict: FrameCollections.defaultdict,".to_string());
                lines.push("    dictFromKeys: FrameCollections.dictFromKeys,".to_string());
                lines.push("    dictUpdate: FrameCollections.dictUpdate,".to_string());
                lines.push("    dictSetDefault: FrameCollections.dictSetDefault,".to_string());
                lines.push("    setUnion: FrameCollections.setUnion,".to_string());
                lines.push("    setIntersection: FrameCollections.setIntersection,".to_string());
                lines.push("    setDifference: FrameCollections.setDifference,".to_string());
                lines.push("    listExtend: FrameCollections.listExtend,".to_string());
                lines.push("    listInsert: FrameCollections.listInsert,".to_string());
                lines.push("    listRemove: FrameCollections.listRemove,".to_string());
                lines.push("    listClear: FrameCollections.listClear,".to_string());
                lines.push("    listPop: FrameCollections.listPop,".to_string());
                lines.push("    listCopy: FrameCollections.listCopy,".to_string());
                lines.push("    setdefault: FrameCollections.dictSetDefault,".to_string());
                lines.push("    OrderedDict,".to_string());
                lines.push("    Counter: FrameCounter,".to_string());
                lines.push("    ChainMap,".to_string());
                lines.push("    FrameDict,".to_string());
                lines.push("    FrameCollections,".to_string());
                lines.push("};".to_string());
                Some(lines.join("\n"))
            }
            "ast" => Some(format!(
                "const {} = {{ literal_eval: (value: any) => FrameRuntime.literalEval(value) }};",
                alias_string.unwrap_or_else(|| sanitized_module.clone())
            )),
            "math" => Some(format!(
                "const {} = FrameMath;",
                alias_string.unwrap_or_else(|| sanitized_module.clone())
            )),
            _ => None,
        }
    }

    /// Support `from module import name` style imports for builtins that the runtime exposes.
    fn map_builtin_from(module: &str, items: &[String]) -> Option<String> {
        if items.is_empty() {
            return Self::map_builtin_simple(module, None);
        }

        let sanitized_items: Vec<String> = items
            .iter()
            .map(|item| Self::sanitize_identifier(item))
            .collect();

        match module {
            "json" => Some(format!(
                "const {{ {} }} = FrameRuntime.json;",
                sanitized_items.join(", ")
            )),
            "ast" => {
                if sanitized_items.len() == 1 && sanitized_items[0] == "literal_eval" {
                    Some(
                        "const { literal_eval } = { literal_eval: (value: any) => FrameRuntime.literalEval(value) };"
                            .to_string(),
                    )
                } else {
                    Some(format!(
                        "const {{ {} }} = {{ literal_eval: (value: any) => FrameRuntime.literalEval(value) }};",
                        sanitized_items.join(", ")
                    ))
                }
            }
            "math" => Some(format!(
                "const {{ {} }} = FrameMath;",
                sanitized_items.join(", ")
            )),
            "collections" => {
                let mut statements: Vec<String> = Vec::new();
                for item in sanitized_items {
                    match item.as_str() {
                        "defaultdict" => {
                            statements.push(
                                "const defaultdict = FrameCollections.defaultdict;".to_string(),
                            );
                        }
                        "dictFromKeys" => {
                            statements.push(
                                "const dictFromKeys = FrameCollections.dictFromKeys;".to_string(),
                            );
                        }
                        "dictUpdate" => {
                            statements.push(
                                "const dictUpdate = FrameCollections.dictUpdate;".to_string(),
                            );
                        }
                        "dictSetDefault" | "setdefault" => {
                            statements.push(
                                "const setdefault = FrameCollections.dictSetDefault;".to_string(),
                            );
                        }
                        "Counter" => {
                            statements.push("const Counter = FrameCounter;".to_string());
                        }
                        "setUnion" => {
                            statements
                                .push("const setUnion = FrameCollections.setUnion;".to_string());
                        }
                        "setIntersection" => {
                            statements.push(
                                "const setIntersection = FrameCollections.setIntersection;"
                                    .to_string(),
                            );
                        }
                        "setDifference" => {
                            statements.push(
                                "const setDifference = FrameCollections.setDifference;".to_string(),
                            );
                        }
                        "listExtend" => {
                            statements.push(
                                "const listExtend = FrameCollections.listExtend;".to_string(),
                            );
                        }
                        "listInsert" => {
                            statements.push(
                                "const listInsert = FrameCollections.listInsert;".to_string(),
                            );
                        }
                        "listRemove" => {
                            statements.push(
                                "const listRemove = FrameCollections.listRemove;".to_string(),
                            );
                        }
                        "listClear" => {
                            statements
                                .push("const listClear = FrameCollections.listClear;".to_string());
                        }
                        "listPop" => {
                            statements
                                .push("const listPop = FrameCollections.listPop;".to_string());
                        }
                        "listCopy" => {
                            statements
                                .push("const listCopy = FrameCollections.listCopy;".to_string());
                        }
                        "OrderedDict" | "ChainMap" | "FrameDict" | "FrameCollections" => {
                            // Already provided by runtime classes; no alias required
                        }
                        other => {
                            statements
                                .push(format!("const {0} = (FrameCollections as any).{0};", other));
                        }
                    }
                }

                if statements.is_empty() {
                    Some(
                        "// Frame runtime collections already expose requested symbols".to_string(),
                    )
                } else {
                    Some(statements.join("\n"))
                }
            }
            "random" | "time" | "sys" | "signal" | "numpy" | "configparser" => {
                if sanitized_items.is_empty() {
                    return Some("// Frame runtime already exposes requested module".to_string());
                }
                let target = match module {
                    "random" => "random",
                    "time" => "time",
                    "sys" => "sys",
                    "signal" => "signal",
                    "numpy" => "numpy",
                    "configparser" => "configparser",
                    _ => unreachable!(),
                };
                Some(format!(
                    "const {{ {} }} = {};",
                    sanitized_items.join(", "),
                    target
                ))
            }
            _ => None,
        }
    }

    fn is_comparison_operator(op: &OperatorType) -> bool {
        matches!(
            op,
            OperatorType::Less
                | OperatorType::LessEqual
                | OperatorType::Greater
                | OperatorType::GreaterEqual
                | OperatorType::EqualEqual
                | OperatorType::NotEqual
                | OperatorType::Is
                | OperatorType::IsNot
        )
    }

    fn operator_to_string(op: &OperatorType) -> &'static str {
        match op {
            OperatorType::Less => "<",
            OperatorType::LessEqual => "<=",
            OperatorType::Greater => ">",
            OperatorType::GreaterEqual => ">=",
            OperatorType::EqualEqual => "===",
            OperatorType::NotEqual => "!==",
            OperatorType::LogicalAnd => "&&",
            OperatorType::LogicalOr => "||",
            OperatorType::Percent => "%",
            OperatorType::Multiply => "*",
            OperatorType::Divide => "/",
            OperatorType::Plus => "+",
            OperatorType::Minus => "-",
            OperatorType::Power => "**",
            OperatorType::BitwiseAnd => "&",
            OperatorType::BitwiseOr => "|",
            OperatorType::BitwiseXor => "^",
            OperatorType::LeftShift => "<<",
            OperatorType::RightShift => ">>",
            _ => "",
        }
    }

    fn collect_comparison_chain(
        &mut self,
        node: &BinaryExprNode,
        operands: &mut Vec<String>,
        operators: &mut Vec<OperatorType>,
    ) -> bool {
        if !Self::is_comparison_operator(&node.operator) {
            return false;
        }

        let left_expr_ref = node.left_rcref.borrow();
        let mut left_handled = false;
        if let ExprType::BinaryExprT {
            binary_expr_node: left_node,
        } = &*left_expr_ref
        {
            if Self::is_comparison_operator(&left_node.operator) {
                left_handled = self.collect_comparison_chain(left_node, operands, operators);
            }
        }

        if !left_handled {
            let mut left_str = String::new();
            self.visit_expr_node_to_string(&left_expr_ref, &mut left_str);
            operands.push(left_str);
        }

        operators.push(node.operator.clone());

        let right_expr_ref = node.right_rcref.borrow();
        let mut right_str = String::new();
        self.visit_expr_node_to_string(&right_expr_ref, &mut right_str);
        operands.push(right_str);

        true
    }

    fn try_emit_comparison_chain(&mut self, node: &BinaryExprNode, output: &mut String) -> bool {
        let mut operands: Vec<String> = Vec::new();
        let mut operators: Vec<OperatorType> = Vec::new();

        if !self.collect_comparison_chain(node, &mut operands, &mut operators) {
            return false;
        }

        if operators.len() < 2 {
            return false;
        }

        let mut parts: Vec<String> = Vec::new();
        for i in 0..operators.len() {
            let left = &operands[i];
            let right = &operands[i + 1];
            let part = match &operators[i] {
                OperatorType::EqualEqual => {
                    format!("FrameRuntime.equals({}, {})", left, right)
                }
                OperatorType::NotEqual => {
                    format!("FrameRuntime.notEquals({}, {})", left, right)
                }
                OperatorType::Is => format!("FrameRuntime.is({}, {})", left, right),
                OperatorType::IsNot => {
                    format!("FrameRuntime.isNot({}, {})", left, right)
                }
                OperatorType::Less
                | OperatorType::LessEqual
                | OperatorType::Greater
                | OperatorType::GreaterEqual => {
                    format!(
                        "({} {} {})",
                        left,
                        Self::operator_to_string(&operators[i]),
                        right
                    )
                }
                _ => format!(
                    "({} {} {})",
                    left,
                    Self::operator_to_string(&operators[i]),
                    right
                ),
            };
            parts.push(part);
        }

        output.push('(');
        output.push_str(&parts.join(" && "));
        output.push(')');
        true
    }

    fn dict_key_identifier(&self, key: &ExprType) -> Option<String> {
        match key {
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                if call_chain_expr_node.call_chain.len() == 1 {
                    match &call_chain_expr_node.call_chain[0] {
                        CallChainNodeType::VariableNodeT { var_node } => {
                            Some(var_node.id_node.name.lexeme.clone())
                        }
                        CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                            Some(id_node.name.lexeme.clone())
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_node_api_action_mapping(&mut self, action: &ActionNode) -> Option<NodeApiActionMapping> {
        if self.action_has_any_return(action) {
            return None;
        }

        let param_names: Vec<String> = action
            .params
            .as_ref()
            .map(|params| params.iter().map(|p| p.param_name.clone()).collect())
            .unwrap_or_default();

        match action.name.as_str() {
            "spawn" => {
                if param_names.is_empty() {
                    return None;
                }

                let command_expr = param_names[0].clone();
                let args_expr = if param_names.len() > 1 {
                    param_names[1].clone()
                } else {
                    "[]".to_string()
                };
                let extra_args = if param_names.len() > 2 {
                    format!(", {}", param_names[2..].join(", "))
                } else {
                    String::new()
                };

                let return_expr = format!(
                    "child_process.spawn({}, {}{})",
                    command_expr, args_expr, extra_args
                );

                self.require_node_module("child_process");

                Some(NodeApiActionMapping {
                    return_type: "any",
                    force_async: None,
                    statements: Vec::new(),
                    return_expression: Some(return_expr),
                })
            }
            "createTcpServer" => {
                let mut create_args: Vec<String> = Vec::new();
                let mut listen_args: Vec<String> = Vec::new();

                for name in &param_names {
                    let lowered = name.to_lowercase();
                    if lowered.contains("port")
                        || lowered.contains("host")
                        || lowered.contains("path")
                        || lowered.contains("backlog")
                    {
                        listen_args.push(name.clone());
                    } else {
                        create_args.push(name.clone());
                    }
                }

                let create_call = if create_args.is_empty() {
                    "net.createServer()".to_string()
                } else {
                    format!("net.createServer({})", create_args.join(", "))
                };

                self.require_node_module("net");

                if listen_args.is_empty() {
                    Some(NodeApiActionMapping {
                        return_type: "any",
                        force_async: None,
                        statements: Vec::new(),
                        return_expression: Some(create_call),
                    })
                } else {
                    let listen_call = listen_args.join(", ");
                    let mut statements = Vec::new();
                    statements.push(format!("const server = {};", create_call));
                    statements.push(format!("server.listen({});", listen_call));
                    self.require_node_module("net");
                    Some(NodeApiActionMapping {
                        return_type: "any",
                        force_async: None,
                        statements,
                        return_expression: Some("server".to_string()),
                    })
                }
            }
            "readFile" => {
                if param_names.is_empty() {
                    return None;
                }

                let path_expr = param_names[0].clone();
                let extra_args = if param_names.len() > 1 {
                    param_names[1..].join(", ")
                } else {
                    "'utf8'".to_string()
                };

                let return_expr = format!("fs.readFileSync({}, {})", path_expr, extra_args);

                self.require_node_module("fs");

                Some(NodeApiActionMapping {
                    return_type: "any",
                    force_async: None,
                    statements: Vec::new(),
                    return_expression: Some(return_expr),
                })
            }
            _ => None,
        }
    }

    pub fn run(mut self, frame_module: &FrameModule) -> String {
        // Add header
        self.builder.writeln(&format!(
            "// Emitted from framec_v{}",
            env!("FRAME_VERSION")
        ));
        self.builder.newline();
        self.builder.newline();

        // Capture target-specific regions for native block emission
        self.target_regions = Arc::clone(&frame_module.target_regions);
        self.register_native_modules(frame_module);

        // Generate runtime support
        self.generate_runtime_support();

        // Visit the module
        self.visit_frame_module(frame_module);

        // Bug 53 fix: Generate runtime function declarations after analysis
        self.emit_runtime_function_declarations();

        let (mut code, mappings) = self.builder.build();

        // Insert any required Node.js module imports detected during visitation
        code = Self::insert_node_module_imports(code, &self.node_module_imports);

        // Post-process to fix patterns that weren't caught by AST visitors
        code = Self::post_process_typescript_output(code);

        // Optional mapping dump (debug): append mapping comments when enabled
        if std::env::var("FRAME_TS_MAP_COMMENTS").is_ok() {
            use std::fmt::Write as _;
            let mut trailer = String::new();
            trailer.push_str("\n// __frame_map_begin__\n");
            for m in &mappings {
                let _ = write!(
                    &mut trailer,
                    "// map frame:{} -> ts:{}\n",
                    m.frame_line, m.python_line
                );
            }
            trailer.push_str("// __frame_map_end__\n");
            code.push_str(&trailer);
        }

        // Optional mapping dump as JSON (debug): append a JSON block inside comments
        if std::env::var("FRAME_TS_MAP_JSON").is_ok() {
            let mut entries: Vec<String> = Vec::new();
            for m in &mappings {
                // We expose python_line as tsLine for clarity on the TS target
                let ty = m
                    .mapping_type
                    .as_ref()
                    .map(|t| format!("\"{:?}\"", t))
                    .unwrap_or("null".to_string());
                entries.push(format!(
                    "{{\"frameLine\":{},\"tsLine\":{},\"type\":{}}}",
                    m.frame_line, m.python_line, ty
                ));
            }
            let json = format!("[{}]", entries.join(","));
            let json_block = format!(
                "\n// __frame_map_json_begin__\n// {}\n// __frame_map_json_end__\n",
                json
            );
            code.push_str(&json_block);
        }

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
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    let min = &caps[1];
                    let max = &caps[2];
                    format!(
                        "Math.floor(Math.random() * ({} - {} + 1)) + {}",
                        max, min, min
                    )
                })
                .to_string();
        }

        // All post-processing hacks removed - visitor should generate correct code directly
        result
    }

    fn generate_runtime_support(&mut self) {
        // TypeScript compilation directives (always needed)
        self.builder
            .writeln("// TypeScript compilation target - ensures Promise support");
        self.builder
            .writeln("/// <reference lib=\"es2015.promise\" />");
        self.builder.newline();

        if self.generate_runtime_classes {
            self.embed_frame_runtime();
            self.builder.newline();
            self.emit_runtime_scaffolding();
        } else {
            self.emit_runtime_imports();
        }
    }

    fn emit_runtime_imports(&mut self) {
        self.builder.writeln(
            "import { FrameRuntime, FrameCollections, FrameCounter, FrameDict, FrameMath, FrameString, FrameEvent, FrameCompartment, OrderedDict, ChainMap, FrameSocketClient, configparser, json, numpy, os, random, signal, sys, time, open } from '../../typescript/runtime/frame_runtime';",
        );
        self.builder.newline();
    }

    fn emit_runtime_scaffolding(&mut self) {
        // External function declarations (provided by runtime environment)
        self.builder
            .writeln("// External function declarations (provided by runtime environment)");
        self.builder
            .writeln("declare var Promise: PromiseConstructor;");
        self.builder.writeln(
            "declare function createAsyncServer(handler: (socket: any) => void): Promise<any>;",
        );
        self.builder.writeln("declare class NetworkServer { }");
        self.builder
            .writeln("declare class JsonParser { static parse(data: any): any; }");

        self.builder.newline();
    }

    fn emit_runtime_function_declarations(&mut self) {
        // Bug 53 fix: Generate runtime function declarations after analysis
        if !self.runtime_function_calls.is_empty() {
            self.builder.writeln("");
            self.builder
                .writeln("// Dynamic runtime function declarations (auto-generated)");
            for func_name in &self.runtime_function_calls {
                self.builder.writeln(&format!(
                    "declare function {}(...args: any[]): any;",
                    func_name
                ));
            }
            self.builder.writeln("");
        }
    }

    fn insert_node_module_imports(code: String, node_modules: &HashSet<String>) -> String {
        if node_modules.is_empty() {
            return code;
        }

        let mut existing_imports: HashSet<String> = HashSet::new();
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") || trimmed.starts_with("const ") {
                existing_imports.insert(trimmed.to_string());
            }
        }

        let mut modules: Vec<String> = node_modules.iter().cloned().collect();
        modules.sort();

        let mut new_imports: Vec<String> = Vec::new();
        for module in modules {
            let module = module.trim();
            if module.is_empty() {
                continue;
            }
            if matches!(
                module,
                "random" | "time" | "sys" | "signal" | "configparser" | "numpy"
            ) {
                continue;
            }
            let alias = Self::sanitize_identifier(module);
            let import_line = format!("const {} = require('{}');", alias, module);
            if !existing_imports.contains(&import_line) {
                existing_imports.insert(import_line.clone());
                new_imports.push(import_line);
            }
        }

        if new_imports.is_empty() {
            return code;
        }

        let mut lines: Vec<String> = code.lines().map(|s| s.to_string()).collect();
        let mut in_block_comment = false;
        let mut last_import_index: Option<usize> = None;
        let mut insertion_index: usize = 0;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();

            if in_block_comment {
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }

            if trimmed.starts_with("/*") {
                in_block_comment = !trimmed.contains("*/");
                continue;
            }

            if trimmed.starts_with("import ") {
                last_import_index = Some(idx);
                insertion_index = idx + 1;
                continue;
            }

            if trimmed.starts_with("///") || trimmed.starts_with("//") || trimmed.is_empty() {
                continue;
            }

            // Found first non-import, non-comment code line
            if last_import_index.is_none() {
                insertion_index = idx;
            }
            break;
        }

        if let Some(last_index) = last_import_index {
            insertion_index = last_index + 1;
        }

        if insertion_index > lines.len() {
            insertion_index = lines.len();
        }

        for (offset, import_line) in new_imports.iter().enumerate() {
            lines.insert(insertion_index + offset, import_line.to_string());
        }

        let mut new_code = lines.join("\n");
        if code.ends_with('\n') {
            new_code.push('\n');
        }
        new_code
    }

    fn generate_enum(&mut self, enum_node: &EnumDeclNode) {
        // Check if this enum has already been declared to avoid duplicates
        if self.declared_enums.contains(&enum_node.name) {
            return; // Skip if already declared
        }

        // Mark this enum as declared
        self.declared_enums.insert(enum_node.name.clone());

        let saved_walrus = self.walrus_declared_locals.clone();
        let saved_pending_walrus = self.pending_walrus_decls.clone();
        self.walrus_declared_locals.clear();
        self.pending_walrus_decls.clear();

        // Generate TypeScript enum class
        self.builder
            .writeln(&format!("class {} {{", enum_node.name));
        self.builder.indent();

        let mut enumerator_names: Vec<String> = Vec::new();
        let mut next_auto_value: i32 = 0;

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
                        let value_str = next_auto_value.to_string();
                        next_auto_value += 1;
                        value_str
                    }
                }
            };

            if let EnumValue::Integer(i) = &enumerator.value {
                next_auto_value = i + 1;
            } else if !matches!(enum_node.enum_type, EnumType::String)
                && !matches!(enumerator.value, EnumValue::Auto)
            {
                // For explicit numeric/string values that aren't integers, reset auto counter
                next_auto_value = 0;
            }

            enumerator_names.push(enumerator.name.clone());

            self.builder.writeln(&format!(
                "static {} = new {}(\"{}\", {});",
                enumerator.name, enum_node.name, enumerator.name, value
            ));
        }

        self.builder.newline();

        // Constructor and value property
        self.builder.writeln(
            "private constructor(public readonly name: string, public readonly value: any) {}",
        );

        // values() helper
        self.builder
            .writeln(&format!("static values(): {}[] {{", enum_node.name));
        self.builder.indent();
        let values_list = enumerator_names
            .iter()
            .map(|name| format!("{}.{}", enum_node.name, name))
            .collect::<Vec<_>>()
            .join(", ");
        self.builder.writeln(&format!("return [{}];", values_list));
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        // Iterator support
        self.builder.writeln(&format!(
            "static [Symbol.iterator](): IterableIterator<{}> {{",
            enum_node.name
        ));
        self.builder.indent();
        self.builder
            .writeln("return this.values()[Symbol.iterator]();");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        // toString helper
        self.builder
            .writeln("toString(): string { return this.name; }");

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        self.walrus_declared_locals = saved_walrus;
        self.pending_walrus_decls = saved_pending_walrus;
    }

    fn format_state_name(&self, state_name: &str) -> String {
        format!(
            "__{}_state_{}",
            self.system_name.to_lowercase(),
            state_name.trim_start_matches('$')
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::visitors::TargetLanguage;

    #[test]
    fn mixedbody_maps_directive_line() {
        let mut v = TypeScriptVisitor::new(Vec::new(), SymbolConfig::default());
        // Simulate a simple mixed body with one native line and one directive at Frame line 123
        let items = vec![
            MixedBodyItem::NativeText {
                target: TargetLanguage::TypeScript,
                text: "const x = 1;\n".to_string(),
                start_line: 10,
                end_line: 10,
            },
            MixedBodyItem::Frame {
                frame_line: 123,
                stmt: MirStatement::StackPush,
            },
        ];
        let (_generated, _ignored) =
            v.emit_target_specific_body(ActionBody::Mixed, &[], &[], &[], None, Some(&items));

        let (_out, mappings) = v.builder.build();
        let has_directive_mapping = mappings.iter().any(|m| m.frame_line == 123);
        assert!(
            has_directive_mapping,
            "expected mapping for directive frame line 123"
        );
    }

    #[test]
    fn mixedbody_maps_native_start_line() {
        let mut v = TypeScriptVisitor::new(Vec::new(), SymbolConfig::default());
        let items = vec![MixedBodyItem::NativeText {
            target: TargetLanguage::TypeScript,
            text: "console.log('x');\n".to_string(),
            start_line: 77,
            end_line: 77,
        }];
        let (_generated, _ignored) =
            v.emit_target_specific_body(ActionBody::Mixed, &[], &[], &[], None, Some(&items));

        let (_out, mappings) = v.builder.build();
        let has_native_mapping = mappings.iter().any(|m| m.frame_line == 77);
        assert!(
            has_native_mapping,
            "expected mapping for native start frame line 77"
        );
    }

    #[test]
    fn mixedbody_maps_transition_line_and_warns_unreachable() {
        let mut v = TypeScriptVisitor::new(Vec::new(), SymbolConfig::default());
        let items = vec![
            MixedBodyItem::NativeText {
                target: TargetLanguage::TypeScript,
                text: "const a = 1;\n".to_string(),
                start_line: 40,
                end_line: 40,
            },
            MixedBodyItem::Frame {
                frame_line: 41,
                stmt: MirStatement::Transition {
                    state: "Next".to_string(),
                    args: vec![],
                },
            },
            // Following native code should be warned as unreachable in the emitted output
            MixedBodyItem::NativeText {
                target: TargetLanguage::TypeScript,
                text: "const b = 2;\n".to_string(),
                start_line: 42,
                end_line: 42,
            },
        ];
        let (_generated, _ignored) =
            v.emit_target_specific_body(ActionBody::Mixed, &[], &[], &[], None, Some(&items));

        let (out, mappings) = v.builder.build();
        let has_transition_mapping = mappings.iter().any(|m| m.frame_line == 41);
        assert!(has_transition_mapping, "expected mapping for transition frame line 41");
        assert!(
            out.contains("WARNING: Unreachable code after transition/forward/stack op"),
            "expected unreachable warning after transition"
        );
    }
}

impl AstVisitor for TypeScriptVisitor {
    fn visit_frame_module(&mut self, frame_module: &FrameModule) {
        self.module_variables.clear();

        // Pre-register module-level variables so systems and functions can reference them
        for var_decl in &frame_module.variables {
            let var = var_decl.borrow();
            self.module_variables.insert(var.name.clone());
        }

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

        // Process classes
        for class_node in &frame_module.classes {
            let class = class_node.borrow();
            self.visit_class_node(&class);
            self.builder.newline();
        }

        // Visit systems before top-level variables so declarations can reference them safely
        for system_node in &frame_module.systems {
            self.visit_system_node(system_node);
        }

        // Process top-level variables (after systems/classes to avoid TDZ issues)
        for var_decl in &frame_module.variables {
            let var = var_decl.borrow();
            self.visit_variable_decl_node(&var);
            self.builder.newline();
        }

        // Process top-level functions
        for function_node in &frame_module.functions {
            let func = function_node.borrow();
            self.visit_function_node(&func);
            self.builder.newline();
        }

        // Process top-level statements
        if !frame_module.statements.is_empty() {
            self.builder.newline();
            for stmt in &frame_module.statements {
                self.visit_decl_or_stmt(stmt);
            }
        }

        // Add main function execution if present - check for main function
        if let Some(main_func) = frame_module
            .functions
            .iter()
            .find(|f| f.borrow().name == "main")
        {
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
                    self.builder.writeln(&format!(
                        "// Auto-execute main function with command line arguments"
                    ));
                    self.builder.writeln(&format!("main({});", args));
                } else {
                    self.builder
                        .writeln(&format!("// Auto-execute main function"));
                    self.builder.writeln("main();");
                }
            } else {
                self.builder
                    .writeln(&format!("// Auto-execute main function"));
                self.builder.writeln("main();");
            }
        }
    }

    fn visit_import_node(&mut self, import_node: &ImportNode) {
        self.builder.map_next(import_node.line);

        let import_stmt = match &import_node.import_type {
            ImportType::Simple { module } => {
                if let Some(mapped) = Self::map_builtin_simple(module, None) {
                    mapped
                } else {
                    let alias = Self::sanitize_identifier(module);
                    format!("const {} = require('{}');", alias, module)
                }
            }
            ImportType::Aliased { module, alias } => {
                if let Some(mapped) = Self::map_builtin_simple(module, Some(alias.as_str())) {
                    mapped
                } else {
                    let sanitized_alias = Self::sanitize_identifier(alias);
                    format!("const {} = require('{}');", sanitized_alias, module)
                }
            }
            ImportType::FromImport { module, items } => {
                if let Some(mapped) = Self::map_builtin_from(module, items) {
                    mapped
                } else {
                    let imports = items.join(", ");
                    format!("const {{ {} }} = require('{}');", imports, module)
                }
            }
            ImportType::FromImportAll { module } => {
                if let Some(mapped) = Self::map_builtin_simple(module, None) {
                    mapped
                } else {
                    let alias = Self::sanitize_identifier(module);
                    format!("const {} = require('{}');", alias, module)
                }
            }
            ImportType::FrameModule {
                module_name,
                file_path,
            } => {
                let ts_module = Self::frame_path_to_typescript_module(file_path);
                format!("import {{ {} }} from '{}';", module_name, ts_module)
            }
            ImportType::FrameModuleAliased {
                module_name,
                file_path,
                alias,
            } => {
                let ts_module = Self::frame_path_to_typescript_module(file_path);
                format!(
                    "import {{ {} as {} }} from '{}';",
                    module_name, alias, ts_module
                )
            }
            ImportType::FrameSelective { items, file_path } => {
                let ts_module = Self::frame_path_to_typescript_module(file_path);
                let items_str = items.join(", ");
                format!("import {{ {} }} from '{}';", items_str, ts_module)
            }
            ImportType::Native { target, code } => {
                if target == &TargetLanguage::TypeScript {
                    code.clone()
                } else {
                    format!("// Unsupported native import for {:?}: {}", target, code)
                }
            }
        };

        self.builder.writeln(&import_stmt);
    }

    fn visit_module_node(&mut self, module_node: &ModuleNode) {
        // Generate TypeScript namespace for Frame module
        self.builder.newline();
        self.builder
            .writeln(&format!("export namespace {} {{", module_node.name));
        self.builder.indent();

        let mut has_content = false;

        // Process module variables as namespace variables
        for var in &module_node.variables {
            let var = var.borrow();
            let mut init_value = String::new();
            self.visit_expr_node_to_string(&var.value_rc, &mut init_value);
            self.builder
                .writeln(&format!("export let {}: any = {};", var.name, init_value));
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
        let previous_async_runtime = self.system_has_async_runtime;
        self.system_has_async_runtime = self.system_needs_async_runtime(system_node);

        // Build parent state map for this system for quick parent-forward lookup
        self.parent_state_map.clear();
        if let Some(machine) = &system_node.machine_block_node_opt {
            for state_rcref in &machine.states {
                let state = state_rcref.borrow();
                let formatted = self.format_state_name(&state.name);
                let parent_formatted = state
                    .dispatch_opt
                    .as_ref()
                    .map(|d| self.format_state_name(&d.target_state_ref.name));
                self.parent_state_map.insert(formatted, parent_formatted);
            }
        }

        // Prepare state variable initializers for this system
        self.state_var_initializers.clear();
        self.state_param_names.clear();
        if let Some(machine) = &system_node.machine_block_node_opt {
            for state_rcref in &machine.states {
                let state = state_rcref.borrow();
                let dict = self.build_state_vars_dict(&state);
                self.state_var_initializers.insert(state.name.clone(), dict);
            }
        }

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

        // Bug 53 fix: Pre-analyze system to collect dynamic properties and runtime function calls
        self.analyze_system_for_dynamic_members(system_node);

        // Generate domain enums BEFORE the class
        if let Some(domain) = &system_node.domain_block_node_opt {
            for enum_decl in &domain.enums {
                let enum_node = enum_decl.borrow();
                self.generate_enum(&*enum_node);
                let alias_name = format!("{}_{}", self.system_name, enum_node.name);
                self.builder
                    .writeln(&format!("const {} = {};", alias_name, enum_node.name));
                self.builder
                    .writeln(&format!("exports.{} = {};", alias_name, alias_name));
                self.builder.newline();
            }
        }

        // Generate TypeScript class
        self.builder
            .writeln(&format!("export class {} {{", self.system_name));
        self.builder.indent();

        // Property declarations
        self.builder
            .writeln("private _compartment: FrameCompartment;");
        self.builder
            .writeln("private _nextCompartment: FrameCompartment | null = null;");
        self.builder.writeln("private returnStack: any[] = [];");
        if self.system_has_async_runtime {
            self.builder
                .writeln("private _startupEvent: FrameEvent | null = null;");
        }

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

        // Bug 53 fix: Generate dynamic property declarations
        if !self.dynamic_properties.is_empty() {
            self.builder
                .writeln("// Dynamic properties (assigned with self.propertyName)");
            for property in &self.dynamic_properties {
                self.builder.writeln(&format!("private {}: any;", property));
            }
        }

        self.builder.newline();

        // Constructor with system parameters
        let mut constructor_params = Vec::new();

        // Collect all system parameters
        if let Some(state_params) = &system_node.start_state_state_params_opt {
            for param in state_params {
                constructor_params.push(format!("{}: any", param.param_name));
            }
        }
        if let Some(enter_params) = &system_node.start_state_enter_params_opt {
            for param in enter_params {
                constructor_params.push(format!("{}: any", param.param_name));
            }
        }
        if let Some(domain_params) = &system_node.domain_params_opt {
            for param in domain_params {
                constructor_params.push(format!("{}: any", param.param_name));
            }
        }

        let params_str = constructor_params.join(", ");
        self.builder
            .writeln(&format!("constructor({}) {{", params_str));
        self.builder.indent();

        // Initialize first state with parameters
        if let Some(machine) = &system_node.machine_block_node_opt {
            if let Some(first_state) = machine.states.first() {
                let state = first_state.borrow();
                let state_name = self.format_state_name(&state.name);

                // Build state args and enter args objects
                let mut state_args = Vec::new();
                let mut enter_args = Vec::new();

                if let Some(state_params) = &system_node.start_state_state_params_opt {
                    for param in state_params {
                        state_args.push(format!("'{}': {}", param.param_name, param.param_name));
                    }
                }

                if let Some(enter_params) = &system_node.start_state_enter_params_opt {
                    for param in enter_params {
                        enter_args.push(format!("'{}': {}", param.param_name, param.param_name));
                    }
                }

                let state_args_str = if state_args.is_empty() {
                    "{}".to_string()
                } else {
                    format!("{{{}}}", state_args.join(", "))
                };

                let enter_args_str = if enter_args.is_empty() {
                    "null".to_string()
                } else {
                    format!("{{{}}}", enter_args.join(", "))
                };

                let state_vars_str = self
                    .state_var_initializers
                    .get(&state.name)
                    .cloned()
                    .unwrap_or_else(|| "{}".to_string());

                self.builder.writeln(&format!(
                    "this._compartment = new FrameCompartment('{}', {}, null, {}, {});",
                    state_name, enter_args_str, state_args_str, state_vars_str
                ));
            }
        }

        self.builder.writeln("this._nextCompartment = null;");
        self.builder.writeln("this.returnStack = [null];");

        // Initialize domain variables
        if let Some(domain) = &system_node.domain_block_node_opt {
            // Check if we have domain parameters that match domain variables
            let domain_param_names: Vec<String> =
                if let Some(domain_params) = &system_node.domain_params_opt {
                    domain_params.iter().map(|p| p.param_name.clone()).collect()
                } else {
                    Vec::new()
                };

            let mut param_index = 0;
            for var_decl in &domain.member_variables {
                let var = var_decl.borrow();
                // Track domain variable name
                self.domain_variables.insert(var.name.clone());

                // Check if this domain variable has a corresponding domain parameter
                if param_index < domain_param_names.len() {
                    // Use domain parameter value
                    self.builder.writeln(&format!(
                        "this.{} = {};",
                        var.name, domain_param_names[param_index]
                    ));
                    param_index += 1;
                } else if !matches!(*var.value_rc, ExprType::NilExprT) {
                    // Use explicit initializer
                    let mut init_str = String::new();
                    self.visit_expr_node_to_string(&var.value_rc, &mut init_str);
                    self.builder
                        .writeln(&format!("this.{} = {};", var.name, init_str));
                } else {
                    // No initializer, set to null
                    self.builder.writeln(&format!("this.{} = null;", var.name));
                }
            }
        }

        // Send start event with enter parameters
        let enter_params_obj = if let Some(enter_params) = &system_node.start_state_enter_params_opt
        {
            let params: Vec<String> = enter_params
                .iter()
                .map(|p| format!("'{}': {}", p.param_name, p.param_name))
                .collect();
            if params.is_empty() {
                "null".to_string()
            } else {
                format!("{{{}}}", params.join(", "))
            }
        } else {
            "null".to_string()
        };

        if self.system_has_async_runtime {
            self.builder.writeln(&format!(
                "this._startupEvent = new FrameEvent(\"$>\", {});",
                enter_params_obj
            ));
        } else {
            self.builder.writeln(&format!(
                "this._frame_kernel(new FrameEvent(\"$>\", {}));",
                enter_params_obj
            ));
        }

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

        if self.system_has_async_runtime {
            self.builder
                .writeln("public async async_start(): Promise<void> {");
            self.builder.indent();
            self.builder.writeln("if (this._startupEvent !== null) {");
            self.builder.indent();
            self.builder
                .writeln("await this._frame_kernel(this._startupEvent);");
            self.builder.writeln("this._startupEvent = null;");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.newline();
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

                if self.system_has_async_runtime {
                    self.builder.writeln(&format!(
                        "private async {}(__e: FrameEvent, compartment: FrameCompartment): Promise<void> {{",
                        state_name
                    ));
                } else {
                    self.builder.writeln(&format!(
                        "private {}(__e: FrameEvent, compartment: FrameCompartment): void {{",
                        state_name
                    ));
                }
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

                        let handler_name = format!(
                            "_handle_{}_{}",
                            state_node.name.trim_start_matches('$').to_lowercase(),
                            handler_suffix
                        );

                        self.builder.writeln(&format!("case \"{}\":", message));
                        self.builder.indent();
                        if self.system_has_async_runtime {
                            self.builder.writeln(&format!(
                                "await this.{}(__e, compartment);",
                                handler_name
                            ));
                        } else {
                            self.builder
                                .writeln(&format!("this.{}(__e, compartment);", handler_name));
                        }
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

        // Public state methods
        if let Some(machine) = &system_node.machine_block_node_opt {
            self.builder.writeln("// Public state methods");
            for state_rcref in &machine.states {
                let state_node = state_rcref.borrow();
                let method_name = format!("_s{}", state_node.name.trim_start_matches('$'));
                let dispatcher_name = self.format_state_name(&state_node.name);
                if self.system_has_async_runtime {
                    self.builder.writeln(&format!(
                        "public async {}(__e: FrameEvent): Promise<void> {{",
                        method_name
                    ));
                } else {
                    self.builder
                        .writeln(&format!("public {}(__e: FrameEvent): void {{", method_name));
                }
                self.builder.indent();
                if self.system_has_async_runtime {
                    self.builder.writeln(&format!(
                        "return await this.{}(__e, this._compartment);",
                        dispatcher_name
                    ));
                } else {
                    self.builder.writeln(&format!(
                        "this.{}(__e, this._compartment);",
                        dispatcher_name
                    ));
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
        self.builder
            .writeln("// Missing method stubs (would be implemented in runtime environment)");
        self.generate_missing_method_stubs();
        self.builder.newline();

        // Runtime methods
        self.builder.writeln("// Frame runtime");

        // _frame_kernel
        if self.system_has_async_runtime {
            self.builder
                .writeln("private async _frame_kernel(__e: FrameEvent): Promise<void> {");
            self.builder.indent();
            self.builder.writeln("await this._frame_router(__e);");
            self.builder
                .writeln("while (this._nextCompartment !== null) {");
            self.builder.indent();
            self.builder
                .writeln("const nextCompartment = this._nextCompartment;");
            self.builder.writeln("this._nextCompartment = null;");
            self.builder.writeln(
                "await this._frame_router(new FrameEvent(\"<$\", this._compartment.exitArgs));",
            );
            self.builder.writeln("this._compartment = nextCompartment;");
            self.builder
                .writeln("if (nextCompartment.forwardEvent === null) {");
            self.builder.indent();
            self.builder.writeln(
                "await this._frame_router(new FrameEvent(\"$>\", this._compartment.enterArgs));",
            );
            self.builder.dedent();
            self.builder.writeln("} else {");
            self.builder.indent();
            self.builder
                .writeln("await this._frame_router(nextCompartment.forwardEvent);");
            self.builder.writeln("nextCompartment.forwardEvent = null;");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.dedent();
            self.builder.writeln("}");
        } else {
            self.builder
                .writeln("private _frame_kernel(__e: FrameEvent): void {");
            self.builder.indent();
            self.builder.writeln("this._frame_router(__e);");
            self.builder
                .writeln("while (this._nextCompartment !== null) {");
            self.builder.indent();
            self.builder
                .writeln("const nextCompartment = this._nextCompartment;");
            self.builder.writeln("this._nextCompartment = null;");
            self.builder
                .writeln("this._frame_router(new FrameEvent(\"<$\", this._compartment.exitArgs));");
            self.builder.writeln("this._compartment = nextCompartment;");
            self.builder
                .writeln("if (nextCompartment.forwardEvent === null) {");
            self.builder.indent();
            self.builder.writeln(
                "this._frame_router(new FrameEvent(\"$>\", this._compartment.enterArgs));",
            );
            self.builder.dedent();
            self.builder.writeln("} else {");
            self.builder.indent();
            self.builder
                .writeln("this._frame_router(nextCompartment.forwardEvent);");
            self.builder.writeln("nextCompartment.forwardEvent = null;");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.dedent();
            self.builder.writeln("}");
        }
        self.builder.newline();

        // _frame_router
        if self.system_has_async_runtime {
            self.builder.writeln(
                "private async _frame_router(__e: FrameEvent, compartment?: FrameCompartment): Promise<void> {",
            );
        } else {
            self.builder.writeln(
                "private _frame_router(__e: FrameEvent, compartment?: FrameCompartment): void {",
            );
        }
        self.builder.indent();
        self.builder
            .writeln("const targetCompartment = compartment || this._compartment;");

        if let Some(machine) = &system_node.machine_block_node_opt {
            self.builder.writeln("switch(targetCompartment.state) {");
            self.builder.indent();

            for state_rcref in &machine.states {
                let state_node = state_rcref.borrow();
                let state_name = self.format_state_name(&state_node.name);

                self.builder.writeln(&format!("case '{}':", state_name));
                self.builder.indent();
                if self.system_has_async_runtime {
                    self.builder.writeln(&format!(
                        "await this.{}(__e, targetCompartment);",
                        state_name
                    ));
                } else {
                    self.builder
                        .writeln(&format!("this.{}(__e, targetCompartment);", state_name));
                }
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
        self.builder
            .writeln("private _frame_transition(nextCompartment: FrameCompartment): void {");
        self.builder.indent();
        self.builder
            .writeln("this._nextCompartment = nextCompartment;");
        self.builder.dedent();
        self.builder.writeln("}");

        self.builder.dedent();
        self.builder.writeln("}");
        self.system_has_async_runtime = previous_async_runtime;
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

        let saved_walrus = self.walrus_declared_locals.clone();
        let saved_pending_walrus = self.pending_walrus_decls.clone();
        self.walrus_declared_locals.clear();
        self.pending_walrus_decls.clear();

        // Build parameter object for event
        let param_obj = if !param_names.is_empty() {
            format!(
                "{{ {} }}",
                param_names
                    .iter()
                    .map(|name| format!("{}: {}", name, name))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            "null".to_string()
        };

        let method_is_async = method.is_async || self.system_has_async_runtime;
        if method_is_async {
            self.builder.writeln(&format!(
                "public async {}({}): Promise<any> {{",
                method_name, params_str
            ));
        } else {
            self.builder
                .writeln(&format!("public {}({}): any {{", method_name, params_str));
        }
        self.builder.indent();

        // Use interface method default value if available, otherwise null
        if let Some(return_init_expr) = &method.return_init_expr_opt {
            let mut default_value = String::new();
            self.visit_expr_node_to_string(return_init_expr, &mut default_value);
            self.builder
                .writeln(&format!("this.returnStack.push({});", default_value));
        } else {
            self.builder.writeln("this.returnStack.push(null);");
        }

        self.builder.writeln(&format!(
            "const __e = new FrameEvent(\"{}\", {});",
            method_name, param_obj
        ));
        if method_is_async {
            self.builder.writeln("await this._frame_kernel(__e);");
        } else {
            self.builder.writeln("this._frame_kernel(__e);");
        }
        self.builder.writeln("return this.returnStack.pop();");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        self.walrus_declared_locals = saved_walrus;
        self.pending_walrus_decls = saved_pending_walrus;
    }

    fn visit_event_handler_node(&mut self, handler: &EventHandlerNode) {
        let state_name = self.current_state_name.as_ref().unwrap().clone();
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
        self.current_local_vars.clear(); // Clear local variables for handler scope
        let saved_walrus = self.walrus_declared_locals.clone();
        let saved_pending_walrus = self.pending_walrus_decls.clone();
        self.walrus_declared_locals.clear();
        self.pending_walrus_decls.clear();
        let event_symbol = handler.event_symbol_rcref.borrow();
        if let Some(params) = &event_symbol.event_symbol_params_opt {
            for param in params {
                self.current_handler_params.insert(param.name.clone());
            }
        }
        drop(event_symbol); // Explicitly drop borrow

        // Set handler default value if available (for any event handler)
        if let Some(return_init_expr) = &handler.return_init_expr_opt {
            let mut default_value = String::new();
            self.visit_expr_node_to_string(return_init_expr, &mut default_value);
            // Handler has default value for empty returns
            self.current_event_handler_default_return_value = Some(default_value);
        } else {
            self.current_event_handler_default_return_value = None;
        }

        let handler_name = format!(
            "_handle_{}_{}",
            state_name.trim_start_matches('$').to_lowercase(),
            self.normalize_message_name(&message)
        );

        // Generate async function signature if handler is async
        if handler.is_async {
            self.builder.writeln(&format!("private async {}(__e: FrameEvent, compartment: FrameCompartment): Promise<void> {{", handler_name));
        } else {
            self.builder.writeln(&format!(
                "private {}(__e: FrameEvent, compartment: FrameCompartment): void {{",
                handler_name
            ));
        }
        self.builder.indent();

        let (generated_target_specific, ignored_targets) = self.emit_target_specific_body(
            handler.body,
            &handler.parsed_target_blocks,
            &handler.target_specific_regions,
            &handler.unrecognized_statements,
            handler.segmented_body.as_ref().map(|v| v.as_slice()),
            handler.mixed_body.as_deref(),
        );

        if generated_target_specific
            && matches!(handler.body, ActionBody::Mixed)
            && !handler.statements.is_empty()
        {
            self.builder.writeln(
                "// NOTE: Frame statements ignored because native TypeScript block was provided",
            );
        }

        if !generated_target_specific {
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
                            self.builder
                                .writeln(&format!("var {} = {};", var_name, init_str));
                        } else {
                            self.builder
                                .writeln(&format!("var {}: any = null;", var_name));
                        }
                    }
                }
            }

            // Handle terminator
            if let Some(terminator) = &handler.terminator_node {
                match &terminator.terminator_type {
                    TerminatorType::Return => {
                        // Use handler default value if available
                        if let Some(handler_default) =
                            &self.current_event_handler_default_return_value
                        {
                            self.builder.writeln(&format!(
                                "this.returnStack[this.returnStack.length - 1] = {};",
                                handler_default
                            ));
                        }
                        self.builder.writeln("return;");
                    }
                }
            }
        }

        // Clear handler default return value
        self.current_event_handler_default_return_value = None;

        self.builder.dedent();
        if !ignored_targets.is_empty() {
            let ignored_list: Vec<String> = ignored_targets.into_iter().collect();
            self.builder.writeln(&format!(
                "// NOTE: target-specific block(s) for {:?} ignored by TypeScript backend",
                ignored_list
            ));
        }
        self.builder.writeln("}");
        self.builder.newline();

        self.walrus_declared_locals = saved_walrus;
        self.pending_walrus_decls = saved_pending_walrus;
    }

    fn visit_operation_node(&mut self, operation: &OperationNode) {
        let operation_name = format!("_operation_{}", operation.name);

        // Clear context for operation scope
        self.current_local_vars.clear();
        self.current_handler_params.clear();
        let saved_walrus = self.walrus_declared_locals.clone();
        let saved_pending_walrus = self.pending_walrus_decls.clone();
        self.walrus_declared_locals.clear();
        self.pending_walrus_decls.clear();

        // Build parameter list and track parameters
        let mut params = Vec::new();
        let mut param_names = Vec::new();
        if let Some(param_nodes) = &operation.params {
            for param in param_nodes {
                params.push(format!("{}: any", param.param_name));
                param_names.push(param.param_name.clone());
                // Track operation parameters as local variables
                self.current_local_vars.insert(param.param_name.clone());
            }
        }
        let params_str = params.join(", ");
        let call_args = if param_names.is_empty() {
            String::new()
        } else {
            param_names.join(", ")
        };

        // Build return type
        let return_type = "any";

        let is_static = operation.is_static();

        if is_static {
            self.builder.writeln(&format!(
                "public static {}({}): {} {{",
                operation.name, params_str, return_type
            ));
        } else {
            self.builder.writeln(&format!(
                "public {}({}): {} {{",
                operation.name, params_str, return_type
            ));
        }
        self.builder.indent();

        let (caller_prefix, receiver) = if is_static {
            (
                format!("const sys = new {}();", self.system_name),
                "sys".to_string(),
            )
        } else {
            (
                "this.returnStack.push(null);".to_string(),
                "this".to_string(),
            )
        };

        if is_static {
            self.builder.writeln(&caller_prefix);
            self.builder.writeln("sys.returnStack.push(null);");
        } else {
            self.builder.writeln(&caller_prefix);
        }

        let call_stmt = if call_args.is_empty() {
            format!("{}.{}();", receiver, operation_name)
        } else {
            format!("{}.{}({});", receiver, operation_name, call_args)
        };
        self.builder.writeln(&call_stmt);

        if is_static {
            self.builder
                .writeln("const result = sys.returnStack.pop();");
            self.builder.writeln("return result;");
        } else {
            self.builder
                .writeln("const result = this.returnStack.pop();");
            self.builder.writeln("return result;");
        }

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        // Generate method signature
        self.builder.writeln(&format!(
            "private {}({}): {} {{",
            operation_name, params_str, return_type
        ));
        self.builder.indent();

        let (generated_target_specific, ignored_targets) = self.emit_target_specific_body(
            operation.body,
            &operation.parsed_target_blocks,
            &operation.target_specific_regions,
            &operation.unrecognized_statements,
            operation.segmented_body.as_deref(),
            operation.mixed_body.as_deref(),
        );

        if generated_target_specific
            && matches!(operation.body, ActionBody::Mixed)
            && !operation.statements.is_empty()
        {
            self.builder.writeln(
                "// NOTE: Frame statements ignored because native TypeScript block was provided",
            );
        }

        if !generated_target_specific {
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
                            self.builder
                                .writeln(&format!("var {} = {};", var_decl.name, init_str));
                        } else {
                            self.builder
                                .writeln(&format!("var {}: any;", var_decl.name));
                        }
                    }
                }
            }
        }

        self.builder.dedent();
        if !ignored_targets.is_empty() {
            let ignored_list: Vec<String> = ignored_targets.into_iter().collect();
            self.builder.writeln(&format!(
                "// NOTE: target-specific block(s) for {:?} ignored by TypeScript backend",
                ignored_list
            ));
        }
        self.builder.writeln("}");
        self.builder.newline();

        self.walrus_declared_locals = saved_walrus;
        self.pending_walrus_decls = saved_pending_walrus;
    }

    fn visit_action_node(&mut self, action: &ActionNode) {
        let action_name = format!("_action_{}", action.name);
        let mapping_opt = self.get_node_api_action_mapping(action);

        // Track this action name for call resolution
        self.action_names.insert(action.name.clone());

        // Set action context
        self.is_in_action = true;

        // Clear context for action scope
        self.current_local_vars.clear();
        self.current_handler_params.clear();
        let saved_walrus = self.walrus_declared_locals.clone();
        let saved_pending_walrus = self.pending_walrus_decls.clone();
        self.walrus_declared_locals.clear();
        self.pending_walrus_decls.clear();

        // Build parameter list and track parameters
        let mut params = Vec::new();
        let mut param_names = Vec::new();
        if let Some(param_nodes) = &action.params {
            for param in param_nodes {
                params.push(format!("{}: any", param.param_name));
                param_names.push(param.param_name.clone());
                // Track action parameters as local variables
                self.current_local_vars.insert(param.param_name.clone());
            }
        }
        let params_str = params.join(", ");

        // Check if action is async
        let mut is_async = self.action_is_async(action);
        if let Some(mapping) = &mapping_opt {
            if let Some(force_async) = mapping.force_async {
                is_async = force_async;
            }
        }

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
            let base_type = if let Some(mapping) = &mapping_opt {
                mapping.return_type
            } else {
                self.infer_action_return_type(action)
            };
            if is_async {
                format!("Promise<{}>", base_type)
            } else {
                base_type.to_string()
            }
        };

        let async_keyword = if is_async { "async " } else { "" };
        self.builder.writeln(&format!(
            "private {}{}{}: {} {{",
            async_keyword,
            action_name,
            if params_str.is_empty() {
                "()".to_string()
            } else {
                format!("({})", params_str)
            },
            return_type
        ));
        self.builder.indent();

        let (generated_target_specific, ignored_targets) = self.emit_target_specific_body(
            action.body,
            &action.parsed_target_blocks,
            &action.target_specific_regions,
            &action.unrecognized_statements,
            action.segmented_body.as_deref(),
            action.mixed_body.as_deref(),
        );

        if generated_target_specific
            && matches!(action.body, ActionBody::Mixed)
            && !action.statements.is_empty()
        {
            self.builder.writeln(
                "// NOTE: Frame statements ignored because native TypeScript block was provided",
            );
        }

        if !generated_target_specific {
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
                            self.builder
                                .writeln(&format!("var {} = {};", var_decl.name, init_str));
                        } else {
                            self.builder
                                .writeln(&format!("var {}: any = null;", var_decl.name));
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
            let mut mapping_applied = false;
            if let Some(mapping) = &mapping_opt {
                for stmt in &mapping.statements {
                    self.builder.writeln(stmt);
                }
                if let Some(return_expr) = &mapping.return_expression {
                    self.builder.writeln(&format!("return {};", return_expr));
                }
                mapping_applied = true;
            }

            if !mapping_applied {
                if !self.action_has_return_with_value(action) && return_type.contains("boolean") {
                    self.builder
                        .writeln("return true; // Default success return for Frame action");
                } else if !self.action_has_return_with_value(action)
                    && return_type.contains("Promise")
                    && !return_type.contains("void")
                {
                    // Async actions with return types need a default return
                    self.builder
                        .writeln("return null; // Default return for async action");
                }
            }
        }

        // Reset action context
        self.walrus_declared_locals = saved_walrus;
        self.pending_walrus_decls = saved_pending_walrus;
        self.is_in_action = false;

        self.builder.dedent();

        if !ignored_targets.is_empty() {
            let ignored_list: Vec<String> = ignored_targets.into_iter().collect();
            self.builder.writeln(&format!(
                "// NOTE: target-specific block(s) for {:?} ignored by TypeScript backend",
                ignored_list
            ));
        }

        self.builder.writeln("}");
        self.builder.newline();

        // Public wrapper for action invocation
        let wrapper_async_keyword = if is_async { "async " } else { "" };
        self.builder.writeln(&format!(
            "public {}{}({}): {} {{",
            wrapper_async_keyword, action.name, params_str, return_type
        ));
        self.builder.indent();
        let call_target = format!("this.{}", action_name);
        let call_args = if param_names.is_empty() {
            String::new()
        } else {
            param_names.join(", ")
        };
        let needs_return = return_type != "void";
        let return_prefix = if needs_return { "return " } else { "" };
        if call_args.is_empty() {
            self.builder
                .writeln(&format!("{}{}();", return_prefix, call_target));
        } else {
            self.builder
                .writeln(&format!("{}{}({});", return_prefix, call_target, call_args));
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
            StatementType::TransitionStmt {
                transition_statement_node,
            } => {
                self.visit_transition_statement_node(transition_statement_node);
            }
            StatementType::ExpressionStmt { expr_stmt_t } => {
                // Handle expression statements
                match expr_stmt_t {
                    ExprStmtType::CallChainStmtT {
                        call_chain_literal_stmt_node,
                    } => {
                        let mut call_str = String::new();
                        self.visit_call_chain_expr_node_to_string(
                            &call_chain_literal_stmt_node.call_chain_literal_expr_node,
                            &mut call_str,
                        );
                        self.flush_pending_walrus_decls();
                        // Don't add semicolon to comments (like // pass)
                        if call_str.starts_with("//") {
                            self.builder.writeln(&call_str);
                        } else {
                            self.builder.writeln(&format!("{};", call_str));
                        }
                    }
                    ExprStmtType::CallStmtT { call_stmt_node } => {
                        let call_expr_node = &call_stmt_node.call_expr_node;
                        if call_expr_node.identifier.name.lexeme == "print" {
                            // Convert print to console.log
                            let mut args_str = String::new();
                            if call_expr_node.call_expr_list.exprs_t.len() > 0 {
                                self.visit_expr_list_node_to_string(
                                    &call_expr_node.call_expr_list.exprs_t,
                                    &mut args_str,
                                );
                            }
                            self.flush_pending_walrus_decls();
                            self.builder.writeln(&format!("console.log({});", args_str));
                        } else {
                            let mut call_str = String::new();
                            self.visit_call_expr_node_to_string(call_expr_node, &mut call_str);
                            self.flush_pending_walrus_decls();
                            self.builder.writeln(&format!("{};", call_str));
                        }
                    }
                    ExprStmtType::AssignmentStmtT {
                        assignment_stmt_node,
                    } => {
                        self.visit_assignment_stmt_node(assignment_stmt_node);
                    }
                    ExprStmtType::ActionCallStmtT {
                        action_call_stmt_node,
                    } => {
                        self.builder.writeln(&format!(
                            "this.{}();",
                            action_call_stmt_node
                                .action_call_expr_node
                                .identifier
                                .name
                                .lexeme
                        ));
                    }
                    ExprStmtType::VariableStmtT { variable_stmt_node } => {
                        let var_name = &variable_stmt_node.var_node.id_node.name.lexeme;
                        // Handle special case of Python's pass statement
                        if var_name == "pass" {
                            // In TypeScript, we can use an empty statement or comment
                            self.builder.writeln("// pass");
                        } else {
                            self.builder.writeln(&format!("this.{};", var_name));
                        }
                    }
                    ExprStmtType::ExprListStmtT {
                        expr_list_stmt_node,
                    } => {
                        for expr in &expr_list_stmt_node.expr_list_node.exprs_t {
                            match expr {
                                ExprType::YieldExprT { yield_expr_node } => {
                                    self.flush_pending_walrus_decls();
                                    self.builder.write("yield");
                                    if let Some(value_expr) = &yield_expr_node.expr {
                                        let mut value_str = String::new();
                                        self.visit_expr_node_to_string(value_expr, &mut value_str);
                                        if !value_str.is_empty() {
                                            self.builder.write(" ");
                                            self.builder.write(&value_str);
                                        }
                                    }
                                    self.builder.writeln(";");
                                }
                                ExprType::YieldFromExprT {
                                    yield_from_expr_node,
                                } => {
                                    self.flush_pending_walrus_decls();
                                    let mut iter_str = String::new();
                                    self.visit_expr_node_to_string(
                                        &yield_from_expr_node.expr,
                                        &mut iter_str,
                                    );
                                    self.builder.writeln(&format!(
                                        "yield* FrameRuntime.iterable({});",
                                        iter_str
                                    ));
                                }
                                _ => {
                                    let mut expr_str = String::new();
                                    self.visit_expr_node_to_string(expr, &mut expr_str);
                                    self.flush_pending_walrus_decls();
                                    self.builder.writeln(&format!("{};", expr_str));
                                }
                            }
                        }
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
                    LoopStmtTypes::LoopInfiniteStmt {
                        loop_infinite_stmt_node,
                    } => {
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
                                        self.builder.writeln(&format!(
                                            "let {} = {};",
                                            var_decl.name, init_str
                                        ));
                                    } else {
                                        self.builder.writeln(&format!(
                                            "let {}: any = null;",
                                            var_decl.name
                                        ));
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
            StatementType::RaiseStmt { raise_stmt_node } => {
                // Handle throw/raise statements
                if let Some(ref expr) = &raise_stmt_node.exception_expr {
                    let mut expr_str = String::new();
                    self.visit_expr_node_to_string(expr, &mut expr_str);
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG TS Raise: expr = {}", expr_str);
                    }
                    // If the expression is already an Error object or string literal, use it directly
                    // Otherwise wrap it in new Error()
                    if expr_str.starts_with("new Error") || expr_str.starts_with("Error") {
                        self.builder.writeln(&format!("throw {};", expr_str));
                    } else {
                        self.builder
                            .writeln(&format!("throw new Error({});", expr_str));
                    }
                } else {
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG TS Raise: no expression");
                    }
                    // Parser bug workaround: bare throw statements from "throw variable" syntax
                    // The parser splits "throw error" into a bare throw and a separate expression
                    // For now, we output a placeholder that won't break TypeScript compilation
                    self.builder.writeln("throw new Error('Exception');");
                }
            }
            StatementType::DelStmt { .. } => {
                // JavaScript doesn't have a direct equivalent to Python's del
                // Could use delete for object properties, but for now just comment
                self.builder
                    .writeln("// TODO: del statement not directly supported in JavaScript");
            }
            StatementType::AssertStmt { assert_stmt_node } => {
                // Convert assert to a runtime check
                let mut condition = String::new();
                self.visit_expr_node_to_string(&assert_stmt_node.expr, &mut condition);

                // Assert statements in Frame don't have a message field, just an expression
                self.builder.writeln(&format!(
                    "if (!({condition})) {{ throw new Error('Assertion failed') }}"
                ));
            }
            _ => {
                // TODO: Handle other statement types (ParentDispatchStmt, StateStackStmt, etc.)
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

    fn visit_call_expr_node_to_string_with_context(
        &mut self,
        node: &CallExprNode,
        output: &mut String,
        is_first_in_chain: bool,
    ) {
        let func_name = &node.identifier.name.lexeme;

        if func_name == "print" {
            // Special case: print becomes console.log
            output.push_str("console.log(");
        } else if func_name == "str" {
            output.push_str("FrameRuntime.str(");
        } else if func_name == "type" {
            // Python type() function - use Frame runtime
            output.push_str("FrameRuntime.getType(");
        } else if func_name == "round" {
            output.push_str("FrameMath.round(");
        } else if func_name == "min" {
            output.push_str("FrameMath.min(");
        } else if func_name == "max" {
            output.push_str("FrameMath.max(");
        } else if self.pending_class_method
            && self
                .pending_class_method_param
                .as_ref()
                .map(|name| name == func_name)
                .unwrap_or(func_name == "cls")
        {
            output.push_str(&format!("new {}(", func_name));
        } else if func_name.starts_with("system.") {
            // Bug #52 Fix: Handle system.methodName calls for interface method calls
            // Frame: system.getValue() -> TypeScript: this.getValue()
            let method_name = &func_name[7..]; // Remove "system." prefix
            output.push_str(&format!("this.{}(", method_name));
        } else if func_name.starts_with("_action_") {
            // Already has action prefix
            let public_name = func_name.trim_start_matches("_action_");
            if is_first_in_chain {
                output.push_str(&format!("this.{}(", public_name));
            } else {
                output.push_str(&format!("{}(", public_name));
            }
        } else if self.action_names.contains(func_name) {
            // Route through public action wrapper; prefix with this. only when call starts the chain
            if is_first_in_chain {
                output.push_str(&format!("this.{}(", func_name));
            } else {
                output.push_str(&format!("{}(", func_name));
            }
        } else if self.operation_names.contains(func_name) {
            // Bug #54-56 Fix: Only add operation prefix if this is the first in the chain
            if is_first_in_chain {
                output.push_str(&format!("this.{}(", func_name));
            } else {
                output.push_str(&format!("{}(", func_name));
            }
        } else if func_name == "defaultdict" {
            let factory_str = if let Some(factory_expr) = node.call_expr_list.exprs_t.get(0) {
                self.render_defaultdict_factory(factory_expr)
            } else {
                "() => undefined".to_string()
            };
            output.push_str("FrameCollections.defaultdict(");
            output.push_str(&factory_str);
            output.push(')');
            return;
        } else {
            // Check if it's a known built-in function
            let is_builtin = matches!(
                func_name.as_str(),
                "int"
                    | "float"
                    | "bool"
                    | "list"
                    | "dict"
                    | "set"
                    | "tuple"
                    | "sum"
                    | "any"
                    | "all"
                    | "len"
                    | "range"
                    | "rfind"
            );
            if is_builtin {
                // Built-in functions - use appropriate TypeScript equivalent
                match func_name.as_str() {
                    "int" => output.push_str("parseInt("),
                    "float" => output.push_str("parseFloat("),
                    "bool" => output.push_str("Boolean("),
                    "sum" => output.push_str("FrameRuntime.sum("),
                    "any" => output.push_str("FrameRuntime.any("),
                    "all" => output.push_str("FrameRuntime.all("),
                    "len" => output.push_str("FrameRuntime.len("),
                    "range" => output.push_str("FrameRuntime.range("),
                    "rfind" => output.push_str("FrameString.rfind("),
                    "list" => {
                        // Check if arguments are provided
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("[]");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("Array.from(");
                        }
                    }
                    "dict" => {
                        // Check if arguments are provided
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("{}");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("FrameDict.fromEntries(");
                        }
                    }
                    "set" => {
                        // Check if arguments are provided
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("new Set()");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("new Set(");
                        }
                    }
                    "tuple" => {
                        // Tuples are just arrays in TypeScript
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("[]");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("Array.from(");
                        }
                    }
                    _ => output.push_str(&format!("{}(", func_name)),
                }
            } else {
                // Check if this is a native array/list method
                let is_native_method = matches!(
                    func_name.as_str(),
                    "append"
                        | "pop"
                        | "remove"
                        | "index"
                        | "count"
                        | "clear"
                        | "extend"
                        | "insert"
                        | "copy"
                );

                // Check if this is a native string method
                let is_string_method = matches!(
                    func_name.as_str(),
                    "upper"
                        | "lower"
                        | "strip"
                        | "replace"
                        | "split"
                        | "join"
                        | "startswith"
                        | "endswith"
                        | "find"
                );

                // Check if this is a native dict method
                let is_dict_method = matches!(func_name.as_str(), "get");

                if is_native_method {
                    // Map native Python list methods to TypeScript array methods
                    let mapped_method = match func_name.as_str() {
                        "append" => "push",   // Python append() -> JavaScript push()
                        "pop" => "pop",       // Python pop() -> JavaScript pop() (same name)
                        "remove" => "splice", // Python remove() -> JavaScript splice() (needs special handling)
                        "index" => "indexOf", // Python index() -> JavaScript indexOf()
                        "count" => "filter",  // Python count() needs custom implementation
                        "clear" => "splice", // Python clear() -> JavaScript splice(0) (needs special handling)
                        "extend" => "push", // Python extend() -> JavaScript push(...items) (needs special handling)
                        "insert" => "splice", // Python insert() -> JavaScript splice() (needs special handling)
                        _ => func_name,       // fallback
                    };
                    output.push_str(&format!("{}(", mapped_method));
                } else if is_dict_method {
                    // Handle dict.get() method: now handled in call chain processing
                    // This path should not be reached for chained .get() calls
                    if func_name == "get" {
                        // Simple .get() call (not in chain) - still use the simple conversion
                        output.push_str("GET_METHOD_PLACEHOLDER(");
                    } else {
                        output.push_str(&format!("{}(", func_name));
                    }
                } else if is_string_method {
                    // Map native Python string methods to TypeScript string methods
                    let mapped_method = match func_name.as_str() {
                        "upper" => "toUpperCase",     // Python upper() -> JavaScript toUpperCase()
                        "lower" => "toLowerCase",     // Python lower() -> JavaScript toLowerCase()
                        "strip" => "trim",            // Python strip() -> JavaScript trim()
                        "replace" => "replace", // Python replace() -> JavaScript replace() (same name)
                        "split" => "split",     // Python split() -> JavaScript split() (same name)
                        "join" => "join",       // Python join() -> JavaScript join() (same name)
                        "startswith" => "startsWith", // Python startswith() -> JavaScript startsWith()
                        "endswith" => "endsWith",     // Python endswith() -> JavaScript endsWith()
                        "find" => "indexOf",          // Python find() -> JavaScript indexOf()
                        _ => func_name,               // fallback
                    };
                    output.push_str(&format!("{}(", mapped_method));
                } else if is_first_in_chain
                    && func_name.chars().next().unwrap_or('a').is_uppercase()
                {
                    // Top-level constructor call - use 'new' operator
                    output.push_str(&format!("new {}(", func_name));
                } else {
                    // Check if we're in a Frame class context and this could be a class method call
                    if let Some(_class_name) = &self.current_class_name_opt {
                        // We're in a Frame class - treat unresolved calls on the current instance when this is the chain root
                        if is_first_in_chain {
                            output.push_str(&format!("this.{}(", func_name));
                        } else {
                            output.push_str(&format!("{}(", func_name));
                        }
                    } else if self.action_names.contains(func_name) {
                        // It's a defined action - route through public wrapper
                        if is_first_in_chain {
                            output.push_str(&format!("this.{}(", func_name));
                        } else {
                            output.push_str(&format!("{}(", func_name));
                        }
                    } else if func_name == "range" {
                        // Python range() function - use Frame runtime
                        output.push_str("FrameRuntime.range(");
                    } else if func_name == "len" {
                        // Python len() function - use Frame runtime
                        output.push_str("FrameRuntime.len(");
                    } else if func_name == "str" {
                        // Python str() function - use JSON.stringify for objects, String for primitives
                        output.push_str("((x) => typeof x === 'object' && x !== null ? JSON.stringify(x) : String(x))(");
                    } else if func_name == "type" {
                        // Python type() function - use Frame runtime
                        output.push_str("FrameRuntime.getType(");
                    } else if func_name == "int" {
                        // Python int() function - use parseInt()
                        output.push_str("parseInt(");
                    } else if func_name == "float" {
                        // Python float() function - use parseFloat()
                        output.push_str("parseFloat(");
                    } else if func_name == "range" {
                        // Python range() function - use Frame runtime
                        output.push_str("FrameRuntime.range(");
                    } else if func_name == "len" {
                        // Python len() function - use Frame runtime
                        output.push_str("FrameRuntime.len(");
                    } else if func_name == "bool" {
                        // Python bool() function - use Boolean()
                        output.push_str("Boolean(");
                    } else if func_name == "list" {
                        // Python list() function - use Array.from() or []
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("[]");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("Array.from(");
                        }
                    } else if func_name == "dict" {
                        // Python dict() function - use FrameDict helper
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("{}");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("FrameDict.fromEntries(");
                        }
                    } else if func_name == "set" {
                        // Python set() function - use new Set()
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("new Set()");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("new Set(");
                        }
                    } else if func_name == "tuple" {
                        // Python tuple() function - use array in TypeScript
                        if node.call_expr_list.exprs_t.is_empty() {
                            output.push_str("[]");
                            return; // Early return to avoid adding arguments
                        } else {
                            output.push_str("Array.from(");
                        }
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

        // Special handling for range() is now handled by Frame runtime

        // Special handling for len() is now handled by Frame runtime in earlier function setup

        if func_name == "get" && output.contains("GET_METHOD_PLACEHOLDER") {
            // Convert obj.get(key, default) to (obj[key] || default)
            // Find the position where "GET_METHOD_PLACEHOLDER(" was added
            if let Some(placeholder_pos) = output.rfind("GET_METHOD_PLACEHOLDER(") {
                // Get everything before the placeholder and arguments
                let before_placeholder = output[..placeholder_pos].to_string();
                let start_pos = placeholder_pos + "GET_METHOD_PLACEHOLDER(".len();
                let args_part = output[start_pos..output.len() - 1].to_string(); // Remove closing paren

                // Parse arguments to get key and default
                let args: Vec<&str> = args_part.split(", ").collect();
                let key = args.get(0).unwrap_or(&"undefined");
                let default = args.get(1).unwrap_or(&"undefined");

                // Rebuild as (obj[key] || default)
                output.clear();
                output.push('(');
                output.push_str(&before_placeholder);
                output.push('[');
                output.push_str(key);
                output.push_str("] || ");
                output.push_str(default);
                output.push(')');

                return; // Early return to skip normal closing paren
            }
        } else if func_name == "set" {
            // Convert set([1, 2, 3]) to new Set([1, 2, 3]) by preserving the array
            // Find the position where "new Set(" was added
            if let Some(set_pos) = output.rfind("new Set(") {
                let start_pos = set_pos + "new Set(".len();

                // Extract the arguments (everything after "new Set(")
                let args_part = output[start_pos..].to_string();

                // Remove everything from "new Set(" onwards
                output.truncate(set_pos);

                // For set([1,2,3]) we want new Set([1,2,3]), not new Set([[1,2,3]])
                let trimmed_args = args_part.trim();
                if trimmed_args.starts_with('[') && trimmed_args.ends_with(')') {
                    // Remove the trailing ) and keep the array as-is
                    let array_part = &trimmed_args[..trimmed_args.len() - 1];
                    output.push_str("new Set(");
                    output.push_str(array_part);
                    output.push(')');
                } else {
                    // Fallback: wrap individual arguments in array
                    output.push_str("new Set([");
                    output.push_str(&args_part);
                    output.push_str("])");
                }
            } else {
                // Fallback if pattern not found
                output.push(')');
            }
        } else if func_name == "list" {
            // Convert list([1, 2, 3]) to [1, 2, 3] by rewriting the output
            // Find the position where "Array.from(" was added
            if let Some(array_pos) = output.rfind("Array.from(") {
                let start_pos = array_pos + "Array.from(".len();

                // Extract the arguments (everything after "Array.from(")
                let args_part = output[start_pos..].to_string();

                // Remove everything from "Array.from(" onwards
                output.truncate(array_pos);

                // For list([1,2,3]) we want just [1,2,3], so remove outer parens from args
                let trimmed_args = args_part.trim();
                if trimmed_args.starts_with('[') && trimmed_args.ends_with(')') {
                    // Remove the trailing ) and just use the array
                    let array_part = &trimmed_args[..trimmed_args.len() - 1];
                    output.push_str(array_part);
                } else {
                    // Fallback: just wrap in brackets
                    output.push('[');
                    output.push_str(&args_part);
                    output.push(']');
                }
            } else {
                // Fallback if pattern not found
                output.push(')');
            }
        } else if func_name == "tuple" {
            // Convert tuple([1, 2, 3]) to [1, 2, 3] by rewriting the output
            // Find the position where "Array.from(" was added
            if let Some(array_pos) = output.rfind("Array.from(") {
                let start_pos = array_pos + "Array.from(".len();

                // Extract the arguments (everything after "Array.from(")
                let args_part = output[start_pos..].to_string();

                // Remove everything from "Array.from(" onwards
                output.truncate(array_pos);

                // For tuple([1,2,3]) we want just [1,2,3], so remove outer parens from args
                let trimmed_args = args_part.trim();
                if trimmed_args.starts_with('[') && trimmed_args.ends_with(')') {
                    // Remove the trailing ) and just use the array
                    let array_part = &trimmed_args[..trimmed_args.len() - 1];
                    output.push_str(array_part);
                } else {
                    // Fallback: just wrap in brackets
                    output.push('[');
                    output.push_str(&args_part);
                    output.push(']');
                }
            } else {
                // Fallback if pattern not found
                output.push(')');
            }
        } else if func_name == "dict" {
            // Convert dict([...]) to FrameDict.fromEntries([...]) for proper dict construction
            // Find the position where "FrameDict(" was added
            if let Some(dict_pos) = output.rfind("FrameDict(") {
                let start_pos = dict_pos + "FrameDict(".len();

                // Extract the arguments (everything after "Object(")
                let args_part = output[start_pos..].to_string();

                // Remove everything from "Object(" onwards
                output.truncate(dict_pos);

                // Rebuild as FrameDict.fromEntries(arguments)
                output.push_str("FrameDict.fromEntries(");
                output.push_str(&args_part);
                output.push(')');
            } else {
                // Fallback if pattern not found
                output.push(')');
            }
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
            eprintln!(
                "DEBUG: ExprType variant: {:?}",
                std::mem::discriminant(expr)
            );
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
                        // Raw strings: convert r"text" to "text" or r"""text""" to `text`
                        let value = &literal_expr_node.value;

                        // Extract content from raw string format
                        let content = if value.starts_with("r\"\"\"") && value.ends_with("\"\"\"") {
                            // Raw triple quoted: r"""content""" -> content (multiline)
                            &value[4..value.len() - 3]
                        } else if value.starts_with("r'''") && value.ends_with("'''") {
                            // Raw triple quoted single quotes: r'''content''' -> content (multiline)
                            &value[4..value.len() - 3]
                        } else if value.starts_with("r\"") && value.ends_with("\"") {
                            // Raw double quoted: r"content" -> content (single line)
                            &value[2..value.len() - 1]
                        } else if value.starts_with("r'") && value.ends_with("'") {
                            // Raw single quoted: r'content' -> content (single line)
                            &value[2..value.len() - 1]
                        } else {
                            // Fallback if format is unexpected
                            value
                        };

                        // Use template literals for multiline raw strings, regular quotes for single line
                        if value.contains("\"\"\"") || value.contains("'''") {
                            // Multiline raw string - use template literal
                            output.push('`');
                            output.push_str(content);
                            output.push('`');
                        } else {
                            // Single line raw string - use regular quotes
                            output.push('"');
                            output.push_str(content);
                            output.push('"');
                        }
                    }
                    TokenType::ByteString => {
                        // Byte strings can be represented as regular strings in TypeScript
                        let value = &literal_expr_node.value;
                        // Handle b"content" format - extract content between quotes
                        let content = if value.starts_with("b\"") && value.ends_with("\"") {
                            &value[2..value.len() - 1]
                        } else if value.starts_with("b'") && value.ends_with("'") {
                            &value[2..value.len() - 1]
                        } else {
                            value
                        };
                        output.push('"');
                        output.push_str(content);
                        output.push('"');
                    }
                    TokenType::TripleQuotedString => {
                        // Triple quoted strings: convert """content""" to `content` (template literal for multiline)
                        let value = &literal_expr_node.value;
                        let content = if value.starts_with("r\"\"\"") && value.ends_with("\"\"\"") {
                            // Raw triple quoted: r"""content""" -> content (without escaping)
                            &value[4..value.len() - 3]
                        } else if value.starts_with("f\"\"\"") && value.ends_with("\"\"\"") {
                            // F-string triple quoted: f"""content""" -> content (will be processed as f-string)
                            &value[4..value.len() - 3]
                        } else if value.starts_with("\"\"\"") && value.ends_with("\"\"\"") {
                            // Regular triple quoted: """content""" -> content
                            &value[3..value.len() - 3]
                        } else if value.starts_with("r'''") && value.ends_with("'''") {
                            // Raw triple quoted single quotes: r'''content''' -> content
                            &value[4..value.len() - 3]
                        } else if value.starts_with("f'''") && value.ends_with("'''") {
                            // F-string triple quoted single quotes: f'''content''' -> content
                            &value[4..value.len() - 3]
                        } else if value.starts_with("'''") && value.ends_with("'''") {
                            // Regular triple quoted single quotes: '''content''' -> content
                            &value[3..value.len() - 3]
                        } else {
                            value
                        };
                        // Use template literals for multiline strings
                        output.push('`');
                        output.push_str(content);
                        output.push('`');
                    }
                    TokenType::ComplexNumber => {
                        // Complex numbers: convert 3+4j to {real: 3, imag: 4} object
                        let value = &literal_expr_node.value;
                        // For now, convert to string representation - full complex number support would need a complex number library
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

                // Handle Python-style keywords as variables
                if var_name == "True" {
                    output.push_str("true");
                    return;
                } else if var_name == "False" {
                    output.push_str("false");
                    return;
                } else if var_name == "pass" {
                    // Python pass statement - in TypeScript we use an empty comment
                    output.push_str("// pass");
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
                    if self.in_state_var_initializer {
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!(
                                "DEBUG TS: Rewriting state param '{}' as placeholder",
                                var_name
                            );
                        }
                        output.push_str(&format!("__frameStateArg_{}", var_name));
                    } else {
                        output.push_str(&format!("compartment.stateArgs['{}']", var_name));
                    }
                } else if self.current_state_vars.contains(var_name) {
                    // State variable - access from compartment
                    output.push_str(&format!("compartment.stateVars['{}']", var_name));
                } else if self.domain_variables.contains(var_name) {
                    // Domain variable - access from this
                    output.push_str(&format!("this.{}", var_name));
                } else if self.current_handler_params.contains(var_name) {
                    // Event handler parameter - access from event parameters
                    output.push_str(&format!("__e.parameters.{}", var_name));
                } else if var_name == "self" {
                    output.push_str("this");
                } else if var_name.starts_with("self.") {
                    output.push_str(&format!("this.{}", &var_name[5..]));
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
                    if var_name == "self" {
                        output.push_str("this");
                    } else {
                        // Unknown variable - use dynamic property access to avoid TypeScript errors
                        output.push_str(&format!("(this as any).{}", var_name));
                    }
                }
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                // Check for equality operations that should use runtime
                if matches!(
                    &binary_expr_node.operator,
                    OperatorType::EqualEqual | OperatorType::NotEqual
                ) {
                    let mut left_str = String::new();
                    let mut right_str = String::new();
                    self.visit_expr_node_to_string(
                        &*binary_expr_node.left_rcref.borrow(),
                        &mut left_str,
                    );
                    self.visit_expr_node_to_string(
                        &*binary_expr_node.right_rcref.borrow(),
                        &mut right_str,
                    );

                    match &binary_expr_node.operator {
                        OperatorType::EqualEqual => {
                            output.push_str(&format!(
                                "FrameRuntime.equals({}, {})",
                                left_str, right_str
                            ));
                        }
                        OperatorType::NotEqual => {
                            output.push_str(&format!(
                                "FrameRuntime.notEquals({}, {})",
                                left_str, right_str
                            ));
                        }
                        _ => unreachable!(),
                    }
                } else {
                    self.visit_binary_expr_node_to_string(binary_expr_node, output);
                }
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                self.visit_unary_expr_node_to_string(unary_expr_node, output);
            }
            ExprType::CallExprT { call_expr_node } => {
                self.visit_call_expr_node_to_string(call_expr_node, output);
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                self.visit_call_chain_expr_node_to_string(call_chain_expr_node, output);
            }
            ExprType::YieldExprT { yield_expr_node } => {
                output.push_str("yield");
                if let Some(value_expr) = &yield_expr_node.expr {
                    let mut value_str = String::new();
                    self.visit_expr_node_to_string(value_expr, &mut value_str);
                    if !value_str.is_empty() {
                        output.push(' ');
                        output.push_str(&value_str);
                    }
                }
            }
            ExprType::YieldFromExprT {
                yield_from_expr_node,
            } => {
                output.push_str("yield* FrameRuntime.iterable(");
                self.visit_expr_node_to_string(&yield_from_expr_node.expr, output);
                output.push(')');
            }
            ExprType::GeneratorExprT {
                generator_expr_node,
            } => {
                output.push_str("(function* () {\n");
                output.push_str("    for (const ");
                output.push_str(&generator_expr_node.target);
                output.push_str(" of FrameRuntime.iterable(");
                self.visit_expr_node_to_string(&generator_expr_node.iter, output);
                output.push_str(")) {\n");
                if let Some(condition) = &generator_expr_node.condition {
                    output.push_str("        if (");
                    self.visit_expr_node_to_string(condition, output);
                    output.push_str(") {\n");
                    output.push_str("            yield ");
                    self.visit_expr_node_to_string(&generator_expr_node.expr, output);
                    output.push_str(";\n");
                    output.push_str("        }\n");
                } else {
                    output.push_str("        yield ");
                    self.visit_expr_node_to_string(&generator_expr_node.expr, output);
                    output.push_str(";\n");
                }
                output.push_str("    }\n");
                output.push_str("})()");
            }
            ExprType::SelfExprT { .. } => {
                output.push_str("this");
            }
            ExprType::NilExprT => {
                output.push_str("null");
            }
            ExprType::ActionCallExprT {
                action_call_expr_node,
            } => {
                output.push_str(&format!(
                    "this.{}(",
                    action_call_expr_node.identifier.name.lexeme
                ));
                if action_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                    let mut args_str = String::new();
                    self.visit_expr_list_node_to_string(
                        &action_call_expr_node.call_expr_list.exprs_t,
                        &mut args_str,
                    );
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
                    FrameEventPart::Param {
                        param_symbol_rcref, ..
                    } => {
                        let param = param_symbol_rcref.borrow();
                        output.push_str(&format!("__e._parameters.{}", param.name));
                    }
                    _ => {
                        output.push_str("/* TODO: frame event part */");
                    }
                }
            }
            ExprType::SystemInstanceExprT {
                system_instance_expr_node,
            } => {
                output.push_str(&format!(
                    "new {}(",
                    system_instance_expr_node.identifier.name.lexeme
                ));
                // Handle domain args if present
                if let Some(args) = &system_instance_expr_node.domain_args_opt {
                    let mut args_str = String::new();
                    self.visit_expr_list_node_to_string(&args.exprs_t, &mut args_str);
                    output.push_str(&args_str);
                }
                output.push(')');
            }
            ExprType::SystemTypeExprT {
                system_type_expr_node,
            } => {
                // Reference to a system type (class in TypeScript)
                output.push_str(&system_type_expr_node.identifier.name.lexeme);
            }
            ExprType::EnumeratorExprT { enum_expr_node } => {
                // Enum values - output as enum_type.enumerator
                output.push_str(&format!(
                    "{}.{}",
                    enum_expr_node.enum_type, enum_expr_node.enumerator
                ));
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
            ExprType::SetLiteralT { set_literal_node } => {
                // Convert Frame set literals to TypeScript Set
                output.push_str("new Set([");
                let mut first = true;
                for expr in &set_literal_node.elements {
                    if !first {
                        output.push_str(", ");
                    }
                    first = false;
                    self.visit_expr_node_to_string(expr, output);
                }
                output.push_str("])");
            }
            ExprType::TupleLiteralT { tuple_literal_node } => {
                // Convert Frame tuple literals to TypeScript arrays
                output.push('[');
                let mut first = true;
                for expr in &tuple_literal_node.elements {
                    if !first {
                        output.push_str(", ");
                    }
                    first = false;
                    self.visit_expr_node_to_string(expr, output);
                }
                output.push(']');
            }
            ExprType::DictComprehensionExprT {
                dict_comprehension_node,
            } => {
                // Convert Frame dict comprehensions to TypeScript using FrameDict.fromEntries
                let (binding, locals) =
                    TypeScriptVisitor::comprehension_binding_parts(&dict_comprehension_node.target);

                for local in &locals {
                    if !local.is_empty() {
                        self.current_local_vars.insert(local.clone());
                    }
                }

                output.push_str("FrameDict.fromEntries(FrameRuntime.iterable(");
                self.visit_expr_node_to_string(&dict_comprehension_node.iter, output);
                output.push(')');

                if let Some(ref condition) = dict_comprehension_node.condition {
                    output.push_str(&format!(".filter({} => ", binding));
                    self.visit_expr_node_to_string(condition, output);
                    output.push(')');
                }

                output.push_str(&format!(".map({} => [", binding));
                self.visit_expr_node_to_string(&dict_comprehension_node.key_expr, output);
                output.push_str(", ");
                self.visit_expr_node_to_string(&dict_comprehension_node.value_expr, output);
                output.push_str("]))");

                for local in locals {
                    self.current_local_vars.remove(&local);
                }
            }
            ExprType::DictUnpackExprT {
                dict_unpack_expr_node,
            } => {
                // Convert Frame dict unpacking (**dict) to TypeScript spread operator (...dict)
                output.push_str("...");
                self.visit_expr_node_to_string(&dict_unpack_expr_node.expr, output);
            }
            ExprType::LambdaExprT { lambda_expr_node } => {
                // Convert Frame lambda expressions to TypeScript arrow functions
                // lambda x, y: x + y -> (x, y) => x + y
                if lambda_expr_node.params.len() == 1 {
                    // Single parameter doesn't need parentheses
                    output.push_str(&lambda_expr_node.params[0]);
                } else {
                    // Multiple parameters need parentheses
                    output.push('(');
                    for (i, param) in lambda_expr_node.params.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        output.push_str(param);
                    }
                    output.push(')');
                }
                output.push_str(" => ");
                self.visit_expr_node_to_string(&lambda_expr_node.body, output);
            }
            ExprType::ListComprehensionExprT {
                list_comprehension_node,
            } => {
                // Convert Frame list comprehensions to TypeScript
                // [expr for var in iterable if condition] -> iterable.filter(var => condition).map(var => expr)
                let (binding, locals) =
                    TypeScriptVisitor::comprehension_binding_parts(&list_comprehension_node.target);

                for local in &locals {
                    if !local.is_empty() {
                        self.current_local_vars.insert(local.clone());
                    }
                }

                output.push_str("FrameRuntime.iterable(");
                self.visit_expr_node_to_string(&list_comprehension_node.iter, output);
                output.push(')');

                if let Some(ref condition) = list_comprehension_node.condition {
                    output.push_str(&format!(".filter({} => ", binding));
                    self.visit_expr_node_to_string(condition, output);
                    output.push(')');
                }

                // Add map to transform elements
                output.push_str(&format!(".map({} => ", binding));
                self.visit_expr_node_to_string(&list_comprehension_node.expr, output);
                output.push(')');

                for local in locals {
                    self.current_local_vars.remove(&local);
                }
            }
            ExprType::SetComprehensionExprT {
                set_comprehension_node,
            } => {
                // Convert Frame set comprehensions to TypeScript
                // {expr for var in iterable if condition} -> new Set(iterable.filter(var => condition).map(var => expr))
                let (binding, locals) =
                    TypeScriptVisitor::comprehension_binding_parts(&set_comprehension_node.target);

                for local in &locals {
                    if !local.is_empty() {
                        self.current_local_vars.insert(local.clone());
                    }
                }

                output.push_str("new Set(FrameRuntime.iterable(");
                self.visit_expr_node_to_string(&set_comprehension_node.iter, output);
                output.push(')');

                if let Some(ref condition) = set_comprehension_node.condition {
                    output.push_str(&format!(".filter({} => ", binding));
                    self.visit_expr_node_to_string(condition, output);
                    output.push(')');
                }

                // Add map to transform elements
                output.push_str(&format!(".map({} => ", binding));
                self.visit_expr_node_to_string(&set_comprehension_node.expr, output);
                output.push_str("))");

                for local in locals {
                    self.current_local_vars.remove(&local);
                }
            }
            ExprType::FunctionRefT { name } => {
                let ref_expr = self.format_function_ref(name);
                output.push_str(&ref_expr);
            }
            ExprType::AssignmentExprT {
                assignment_expr_node,
            } => {
                let mut rhs = String::new();
                self.visit_expr_node_to_string(&assignment_expr_node.r_value_rc, &mut rhs);
                output.push_str(&rhs);
            }
            ExprType::WalrusExprT {
                assignment_expr_node,
            } => {
                if let ExprType::VariableExprT { var_node } = &*assignment_expr_node.l_value_box {
                    self.record_walrus_local(&var_node.id_node.name.lexeme);
                }

                let mut lhs = String::new();
                self.visit_expr_node_to_string(&assignment_expr_node.l_value_box, &mut lhs);

                let mut rhs = String::new();
                self.visit_expr_node_to_string(&assignment_expr_node.r_value_rc, &mut rhs);

                output.push('(');
                output.push_str(&lhs);
                output.push_str(" = ");
                output.push_str(&rhs);
                output.push(')');
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
        // Handle operators that require specialized runtime support before generic formatting.
        if matches!(&node.operator, OperatorType::Plus) {
            let mut left_str = String::new();
            let mut right_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);
            self.visit_expr_node_to_string(&*node.right_rcref.borrow(), &mut right_str);

            output.push_str(&format!("FrameRuntime.add({}, {})", left_str, right_str));
            return;
        }

        if matches!(&node.operator, OperatorType::Is | OperatorType::IsNot) {
            let mut left_str = String::new();
            let mut right_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);
            self.visit_expr_node_to_string(&*node.right_rcref.borrow(), &mut right_str);

            match &node.operator {
                OperatorType::Is => {
                    output.push_str(&format!("FrameRuntime.is({}, {})", left_str, right_str));
                }
                OperatorType::IsNot => {
                    output.push_str(&format!("FrameRuntime.isNot({}, {})", left_str, right_str));
                }
                _ => {}
            }
            return;
        }

        if matches!(
            &node.operator,
            OperatorType::Minus
                | OperatorType::Multiply
                | OperatorType::Divide
                | OperatorType::FloorDivide
                | OperatorType::Power
        ) {
            let mut left_str = String::new();
            let mut right_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);
            self.visit_expr_node_to_string(&*node.right_rcref.borrow(), &mut right_str);

            let helper = match &node.operator {
                OperatorType::Minus => "FrameRuntime.subtract",
                OperatorType::Multiply => "FrameRuntime.multiply",
                OperatorType::Divide => "FrameRuntime.divide",
                OperatorType::FloorDivide => "FrameRuntime.floorDivide",
                OperatorType::Percent => "FrameRuntime.modulo",
                OperatorType::Power => "FrameRuntime.power",
                _ => unreachable!(),
            };

            output.push_str(&format!("{}({}, {})", helper, left_str, right_str));
            return;
        }

        if matches!(&node.operator, OperatorType::BitwiseOr) {
            let mut left_str = String::new();
            let mut right_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);
            self.visit_expr_node_to_string(&*node.right_rcref.borrow(), &mut right_str);
            output.push_str(&format!(
                "FrameDict.unionDynamic({}, {})",
                left_str, right_str
            ));
            return;
        }

        // Special handling for percent formatting - check if it's string formatting vs modulo
        if matches!(&node.operator, OperatorType::Percent) {
            // Check if left operand is a string literal containing format specifiers
            let mut left_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);

            // If left side is a string containing %s, %d, %f, etc., treat as string formatting
            if (left_str.starts_with('"') && left_str.ends_with('"'))
                && (left_str.contains("%s")
                    || left_str.contains("%d")
                    || left_str.contains("%f")
                    || left_str.contains("%.")
                    || left_str.contains("%("))
            {
                // This is string formatting - convert to simple template literals for now
                // Full implementation would need a proper printf-style formatter
                output.push_str("/* TODO: String formatting - ");
                output.push_str(&left_str);
                output.push_str(" % ");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(" */");

                // For now, just return the string without formatting
                output.push_str(&left_str);
                return;
            }
            // If not string formatting, fall through to runtime modulo handling
            let mut left_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);
            let mut right_str = String::new();
            self.visit_expr_node_to_string(&*node.right_rcref.borrow(), &mut right_str);
            output.push_str(&format!("FrameRuntime.modulo({}, {})", left_str, right_str));
            return;
        }

        if matches!(&node.operator, OperatorType::Plus) {
            let mut left_str = String::new();
            let mut right_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);
            self.visit_expr_node_to_string(&*node.right_rcref.borrow(), &mut right_str);

            if self.is_counter_reference(&left_str) || self.is_counter_reference(&right_str) {
                output.push_str(&format!("Counter.add({}, {})", left_str, right_str));
                return;
            }
        }

        // Special handling for string multiplication - check before adding parenthesis
        if matches!(&node.operator, OperatorType::Multiply) {
            // Check if this is string multiplication
            let mut left_str = String::new();
            let mut right_str = String::new();
            self.visit_expr_node_to_string(&*node.left_rcref.borrow(), &mut left_str);
            self.visit_expr_node_to_string(&*node.right_rcref.borrow(), &mut right_str);

            // Check if left is a string literal
            let left_is_string_literal = left_str.starts_with('"') && left_str.ends_with('"');
            // Check if right is a string literal
            let right_is_string_literal = right_str.starts_with('"') && right_str.ends_with('"');

            // Check if operands are numeric literals
            let right_is_number = right_str.chars().all(|c| c.is_ascii_digit());
            let left_is_number = left_str.chars().all(|c| c.is_ascii_digit());

            // Only apply string repeat when we're CERTAIN it's a string (literal or known string var)
            // Be conservative - only when one operand is definitely a string literal
            if left_is_string_literal && right_is_number {
                // "string" * number -> "string".repeat(number)
                output.push_str(&format!("({}).repeat({})", left_str, right_str));
                return; // Early return - don't add parentheses or closing
            } else if right_is_string_literal && left_is_number {
                // number * "string" -> "string".repeat(number)
                output.push_str(&format!("({}).repeat({})", right_str, left_str));
                return; // Early return - don't add parentheses or closing
            }

            // Check for known string variables (like from string operations)
            // For now, we'll be conservative and not guess variable types
            // If not string multiplication, fall through to normal handling
        }

        output.push('(');
        self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);

        let op_str = match &node.operator {
            OperatorType::Minus => " - ",
            OperatorType::Multiply => " * ", // Normal numeric multiplication
            OperatorType::Divide => " / ",
            OperatorType::Greater => " > ",
            OperatorType::GreaterEqual => " >= ",
            OperatorType::Less => " < ",
            OperatorType::LessEqual => " <= ",
            OperatorType::EqualEqual => " === ",
            OperatorType::NotEqual => " !== ",
            OperatorType::LogicalAnd => " && ",
            OperatorType::LogicalOr => " || ",
            OperatorType::Percent => " % ",
            OperatorType::Power => " ** ",
            OperatorType::BitwiseOr => " | ",
            OperatorType::BitwiseAnd => " & ",
            OperatorType::BitwiseXor => " ^ ",
            OperatorType::LeftShift => " << ",
            OperatorType::RightShift => " >> ",
            OperatorType::In => {
                // Handle 'in' operator for TypeScript
                // Since TypeScript 'in' only works for object properties, we need special handling for arrays/sets
                output.clear(); // Clear the opening parenthesis and left operand

                // For simplicity, generate a runtime check that handles all collection types
                // This uses JavaScript's runtime checking to handle objects, arrays, and sets
                output.push_str("((");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").includes ? (");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").includes(");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(") : (");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").has ? (");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").has(");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(") : (");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(" in ");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str("))");
                return; // Early return to avoid the normal binary expression handling
            }
            OperatorType::NotIn => {
                // Handle 'not in' operator: negate the comprehensive 'in' check
                output.clear(); // Clear the opening parenthesis and left operand
                output.push_str("!((");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").includes ? (");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").includes(");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(") : (");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").has ? (");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str(").has(");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(") : (");
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(" in ");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push_str("))");
                return; // Early return to avoid the normal binary expression handling
            }
            OperatorType::MatMul => {
                // Handle matrix multiplication: a @ b -> a.matmul(b)
                // Note: TypeScript/JavaScript doesn't have built-in matrix multiplication
                // This generates a method call that would need to be provided by a math library
                output.clear(); // Clear the opening parenthesis and left operand
                self.visit_expr_node_to_string(&*node.left_rcref.borrow(), output);
                output.push_str(".matmul(");
                self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
                output.push(')');
                return; // Early return to avoid the normal binary expression handling
            }
            _ => " /* TODO: operator */ ",
        };

        output.push_str(op_str);
        self.visit_expr_node_to_string(&*node.right_rcref.borrow(), output);
        output.push(')');
    }

    fn visit_unary_expr_node_to_string(&mut self, node: &UnaryExprNode, output: &mut String) {
        let op_str = match &node.operator {
            OperatorType::Not => "!",                           // Logical NOT: !true
            OperatorType::Minus | OperatorType::Negated => "-", // Arithmetic negation: -2, (-2)
            OperatorType::Plus => "+",                          // Unary plus: +2
            OperatorType::BitwiseNot => "~",                    // Bitwise NOT: ~5
            _ => "/* TODO: unary op */",
        };

        output.push_str(op_str);

        // Add parentheses around operand only when needed (like Python visitor)
        let right_expr = node.right_rcref.borrow();
        let needs_parens = matches!(
            &*right_expr,
            ExprType::BinaryExprT { .. } | ExprType::UnaryExprT { .. }
        );

        if needs_parens {
            output.push('(');
        }
        self.visit_expr_node_to_string(&*right_expr, output);
        if needs_parens {
            output.push(')');
        }
    }

    fn visit_assignment_stmt_node(&mut self, node: &AssignmentStmtNode) {
        let mut rhs = String::new();
        self.visit_expr_node_to_string(&node.assignment_expr_node.r_value_rc, &mut rhs);
        self.flush_pending_walrus_decls();

        // Check if assignment is to a simple variable name that needs local declaration
        let (is_simple_var, var_name_opt) = match &*node.assignment_expr_node.l_value_box {
            ExprType::VariableExprT { var_node } => {
                (true, Some(var_node.id_node.name.lexeme.clone()))
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                // Check if it's a simple variable (single identifier)
                if call_chain_expr_node.call_chain.len() == 1 {
                    match &call_chain_expr_node.call_chain[0] {
                        crate::frame_c::ast::CallChainNodeType::VariableNodeT { var_node } => {
                            (true, Some(var_node.id_node.name.lexeme.clone()))
                        }
                        crate::frame_c::ast::CallChainNodeType::UndeclaredIdentifierNodeT {
                            id_node,
                        } => (true, Some(id_node.name.lexeme.clone())),
                        _ => (false, None),
                    }
                } else if call_chain_expr_node.call_chain.len() == 2 {
                    // Check for system.return (special case - not a simple variable)
                    if let (
                        crate::frame_c::ast::CallChainNodeType::UndeclaredIdentifierNodeT {
                            id_node: first,
                        },
                        crate::frame_c::ast::CallChainNodeType::UndeclaredIdentifierNodeT {
                            id_node: second,
                        },
                    ) = (
                        &call_chain_expr_node.call_chain[0],
                        &call_chain_expr_node.call_chain[1],
                    ) {
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
                self.builder.writeln(&format!(
                    "this.returnStack[this.returnStack.length - 1] = {};",
                    rhs
                ));
                return; // Early return to avoid normal processing
            }
        }

        if is_simple_var {
            if let Some(var_name) = var_name_opt {
                // Assignment statements should NEVER create variable declarations
                // Variable declarations are handled by visit_variable_decl_node only
                // All assignments are just assignments to existing variables

                if self.module_variables.contains(&var_name) {
                    let mut lhs = String::new();
                    self.visit_expr_node_to_string(
                        &node.assignment_expr_node.l_value_box,
                        &mut lhs,
                    );
                    self.builder.writeln(&format!("{} = {};", lhs, rhs));
                    return;
                }

                // Don't add domain variables to local vars - they should be resolved as "this.var"
                if !self.domain_variables.contains(&var_name)
                    && !self.current_state_vars.contains(&var_name)
                    && !self.current_state_params.contains(&var_name)
                    && !self.current_handler_params.contains(&var_name)
                    && !self.current_exception_vars.contains(&var_name)
                {
                    let is_new = self.current_local_vars.insert(var_name.clone());

                    let mut lhs = String::new();
                    self.visit_expr_node_to_string(
                        &node.assignment_expr_node.l_value_box,
                        &mut lhs,
                    );

                    if is_new {
                        self.builder.writeln(&format!("var {} = {};", lhs, rhs));
                    } else {
                        self.builder.writeln(&format!("{} = {};", lhs, rhs));
                    }
                    return;
                }

                let mut lhs = String::new();
                self.visit_expr_node_to_string(&node.assignment_expr_node.l_value_box, &mut lhs);
                self.builder.writeln(&format!("{} = {};", lhs, rhs));

                let rhs_trimmed = rhs.trim_start();
                if rhs_trimmed.starts_with("new Counter(")
                    || rhs_trimmed.starts_with("Counter.add(")
                {
                    self.counter_variables.insert(var_name.clone());
                }
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
            self.flush_pending_walrus_decls();

            if self.is_in_action || self.in_module_function || self.current_class_name_opt.is_some()
            {
                // Actions, module functions, and Frame class methods use direct returns
                self.builder.writeln(&format!("return {};", expr_str));
            } else {
                // Event handlers use return stack
                self.builder.writeln(&format!(
                    "this.returnStack[this.returnStack.length - 1] = {};",
                    expr_str
                ));
                self.builder.writeln("return;");
            }
        } else {
            // Return with no expression - use handler default value if available
            if !self.is_in_action
                && !self.in_module_function
                && self.current_class_name_opt.is_none()
            {
                // Event handlers - check for default return value
                if let Some(handler_default) = &self.current_event_handler_default_return_value {
                    self.builder.writeln(&format!(
                        "this.returnStack[this.returnStack.length - 1] = {};",
                        handler_default
                    ));
                }
            }
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
            LoopFirstStmt::Var { var_node } => var_node.id_node.name.lexeme.clone(),
            LoopFirstStmt::CallChain { .. } => {
                "_".to_string() // Fallback for complex expressions
            }
            LoopFirstStmt::VarDecl {
                var_decl_node_rcref,
            } => {
                let var_decl = var_decl_node_rcref.borrow();
                var_decl.name.clone()
            }
            LoopFirstStmt::VarDeclAssign {
                var_decl_node_rcref,
            } => {
                let var_decl = var_decl_node_rcref.borrow();
                var_decl.name.clone()
            }
            LoopFirstStmt::None => "_".to_string(),
        };

        // Get the iterable expression (e.g., MenuOption)
        let mut expr_str = String::new();
        self.visit_expr_node_to_string(&node.iterable_expr, &mut expr_str);

        // Generate TypeScript for-of loop with Object.values() for enum iteration
        // For enums like MenuOption, this generates: for (const option of Object.values(MenuOption))
        self.builder.writeln(&format!(
            "for (const {} of Object.values({})) {{",
            var_name, expr_str
        ));
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
                            self.builder
                                .writeln(&format!("var {} = {};", var_decl.name, init_str));
                        } else {
                            self.builder
                                .writeln(&format!("var {}: any = null;", var_decl.name));
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
        self.flush_pending_walrus_decls();
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
                        self.builder
                            .writeln(&format!("var {} = {};", var_decl.name, init_str));
                    } else {
                        self.builder
                            .writeln(&format!("var {}: any = null;", var_decl.name));
                    }
                }
            }
        }

        self.builder.dedent();

        // Generate else-if branches
        for elif_clause in &node.elif_clauses {
            let mut elif_cond_str = String::new();
            self.visit_expr_node_to_string(&elif_clause.condition, &mut elif_cond_str);
            self.flush_pending_walrus_decls();
            self.builder
                .writeln(&format!("}} else if ({}) {{", elif_cond_str));
            self.builder.indent();

            for stmt in &elif_clause.block.statements {
                match stmt {
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_stmt_node(&stmt_t);
                    }
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.builder
                            .writeln(&format!("var {}: any; // TODO: Initialize", var_decl.name));
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
                        self.builder
                            .writeln(&format!("var {}: any; // TODO: Initialize", var_decl.name));
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
        false // For now, return false
    }

    // TODO: Add full state node lookup when arcanum is available
    // For now, use a simplified approach

    fn visit_transition_statement_node(&mut self, transition_node: &TransitionStatementNode) {
        let debug_enabled = std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1";

        // Create compartment for target state
        let (target_state_name, raw_state_name, state_args_opt) =
            match &transition_node.transition_expr_node.target_state_context_t {
                TargetStateContextType::StateRef { state_context_node } => {
                    let raw_name = state_context_node.state_ref_node.name.clone();
                    (
                        self.format_state_name(&raw_name),
                        raw_name,
                        state_context_node.state_ref_args_opt.as_ref(),
                    )
                }
                TargetStateContextType::StateStackPop {} => {
                    // Pop previously pushed value/state and return from handler
                    self.builder
                        .writeln("const __popped = this.returnStack.pop();");
                    self.builder
                        .writeln("this.returnStack[this.returnStack.length - 1] = __popped;");
                    self.builder.writeln("return;");
                    return;
                }
                TargetStateContextType::StateStackPush {} => {
                    // Push placeholder onto return stack and return
                    self.builder.writeln("this.returnStack.push({});");
                    self.builder.writeln("return;");
                    return;
                }
            };

        if debug_enabled {
            eprintln!(
                "DEBUG TS: Processing transition to state '{}' with {} args",
                target_state_name,
                state_args_opt.map(|args| args.exprs_t.len()).unwrap_or(0)
            );
        }

        let state_vars_dict = self
            .state_var_initializers
            .get(&raw_state_name)
            .cloned()
            .unwrap_or_else(|| "{}".to_string());

        let param_names = self
            .state_param_names
            .get(&raw_state_name)
            .cloned()
            .or_else(|| {
                let prefixed = format!("${}", raw_state_name);
                self.state_param_names.get(&prefixed).cloned()
            })
            .or_else(|| {
                let formatted = self.format_state_name(&raw_state_name);
                self.state_param_names.get(&formatted).cloned()
            })
            .unwrap_or_default();

        let mut state_vars_expr_output = state_vars_dict.clone();
        let mut binding_order: Vec<(String, String)> = Vec::new();

        if debug_enabled {
            eprintln!(
                "DEBUG TS: Transitioning to '{}' with param names {:?}",
                raw_state_name, param_names
            );
            if param_names.is_empty() {
                let keys: Vec<String> = self.state_param_names.keys().cloned().collect();
                eprintln!("DEBUG TS: Available state param map keys: {:?}", keys);
            }
        }

        if let Some(state_args) = state_args_opt {
            if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                eprintln!(
                    "DEBUG TS: Transition state args count {}, param_names {:?}",
                    state_args.exprs_t.len(),
                    param_names
                );
            }
            for (idx, expr) in state_args.exprs_t.iter().enumerate() {
                let mut value_str = String::new();
                self.visit_expr_node_to_string(expr, &mut value_str);
                let mut resolved_name = param_names.get(idx).cloned();
                if resolved_name.is_none() {
                    resolved_name = self
                        .state_param_names
                        .get(&raw_state_name)
                        .and_then(|names| names.get(idx).cloned())
                        .or_else(|| {
                            let prefixed = format!("${}", raw_state_name);
                            self.state_param_names
                                .get(&prefixed)
                                .and_then(|names| names.get(idx).cloned())
                        })
                        .or_else(|| {
                            let formatted = self.format_state_name(&raw_state_name);
                            self.state_param_names
                                .get(&formatted)
                                .and_then(|names| names.get(idx).cloned())
                        });
                }
                let param_name = resolved_name.unwrap_or_else(|| format!("param{}", idx));
                let arg_var_name = format!("__frameStateArg{}{}", self.unpack_temp_counter, idx);
                self.unpack_temp_counter += 1;
                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                    eprintln!(
                        "DEBUG TS: Binding transition arg {} to temp {}",
                        param_name, arg_var_name
                    );
                }
                self.builder
                    .writeln(&format!("const {} = {};", arg_var_name, value_str));
                binding_order.push((param_name, arg_var_name));
            }
        }

        let mut state_args_argument = "{}".to_string();
        if !binding_order.is_empty() {
            let state_args_var_name = format!("__frameStateArgs{}", self.unpack_temp_counter);
            self.unpack_temp_counter += 1;
            let mut entries = Vec::new();
            for (param_name, arg_var_name) in &binding_order {
                entries.push(format!("'{}': {}", param_name, arg_var_name));
            }
            self.builder.writeln(&format!(
                "const {} = {{ {} }};",
                state_args_var_name,
                entries.join(", ")
            ));
            state_args_argument = state_args_var_name.clone();

            for (param_name, arg_var_name) in &binding_order {
                let placeholder = format!("__frameStateArg_{}", param_name);
                state_vars_expr_output = state_vars_expr_output.replace(&placeholder, arg_var_name);

                let single_quote_access = format!("compartment.stateArgs['{}']", param_name);
                if state_vars_expr_output.contains(&single_quote_access) {
                    let replacement = format!("{}['{}']", state_args_argument, param_name);
                    state_vars_expr_output =
                        state_vars_expr_output.replace(&single_quote_access, &replacement);
                }
                let double_quote_access = format!("compartment.stateArgs[\"{}\"]", param_name);
                if state_vars_expr_output.contains(&double_quote_access) {
                    let replacement = format!("{}['{}']", state_args_argument, param_name);
                    state_vars_expr_output =
                        state_vars_expr_output.replace(&double_quote_access, &replacement);
                }
            }
        }

        let mut state_vars_argument = "{}".to_string();
        if state_vars_expr_output.trim() != "{}" {
            let state_vars_var_name = format!("__frameStateVars{}", self.unpack_temp_counter);
            self.unpack_temp_counter += 1;
            self.builder.writeln(&format!(
                "const {} = {};",
                state_vars_var_name, state_vars_expr_output
            ));
            state_vars_argument = state_vars_var_name;
        }

        if debug_enabled {
            eprintln!("DEBUG TS: State vars dict: {}", state_vars_dict);
            eprintln!(
                "DEBUG TS: Transition stateArgs binding count: {}",
                binding_order.len()
            );
        }

        self.builder.writeln(&format!(
            "this._frame_transition(new FrameCompartment('{}', null, null, {}, {}));",
            target_state_name, state_args_argument, state_vars_argument
        ));
    }

    fn visit_call_chain_expr_node_to_string(
        &mut self,
        node: &CallChainExprNode,
        output: &mut String,
    ) {
        // Handle call chains like obj.method1().method2()
        // Special case: system.return should become this.returnStack[this.returnStack.length - 1]

        // Check for special patterns
        if node.call_chain.len() == 2 {
            // Check for system.return pattern
            if let (
                CallChainNodeType::VariableNodeT {
                    var_node: first_var,
                },
                CallChainNodeType::VariableNodeT {
                    var_node: second_var,
                },
            ) = (&node.call_chain[0], &node.call_chain[1])
            {
                if first_var.id_node.name.lexeme == "system"
                    && second_var.id_node.name.lexeme == "return"
                {
                    // DEBUG: Check if this is being triggered
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG: Detected system.return pattern, replacing with returnStack access");
                    }
                    output.push_str("this.returnStack[this.returnStack.length - 1]");
                    return;
                }
            }

            // Check for random.randint pattern
            if let (
                CallChainNodeType::VariableNodeT {
                    var_node: first_var,
                },
                CallChainNodeType::UndeclaredCallT { call_node },
            ) = (&node.call_chain[0], &node.call_chain[1])
            {
                if first_var.id_node.name.lexeme == "random"
                    && call_node.identifier.name.lexeme == "randint"
                {
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

                if first_var.id_node.name.lexeme == "json" {
                    let method = &call_node.identifier.name.lexeme;
                    if method == "loads" {
                        output.push_str("FrameRuntime.jsonLoads(");
                        if call_node.call_expr_list.exprs_t.len() > 0 {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(
                                &call_node.call_expr_list.exprs_t,
                                &mut args_str,
                            );
                            output.push_str(&args_str);
                        }
                        output.push(')');
                        return;
                    } else if method == "dumps" {
                        output.push_str("FrameRuntime.jsonDumps(");
                        if call_node.call_expr_list.exprs_t.len() > 0 {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(
                                &call_node.call_expr_list.exprs_t,
                                &mut args_str,
                            );
                            output.push_str(&args_str);
                        }
                        output.push(')');
                        return;
                    }
                }

                if first_var.id_node.name.lexeme == "ast"
                    && call_node.identifier.name.lexeme == "literal_eval"
                {
                    output.push_str("FrameRuntime.literalEval(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                }
            }

            // Bug #54, #55, #56 Fix: Check for API mapping patterns in call chains
            if let (
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node: first_id },
                CallChainNodeType::UndeclaredCallT { call_node },
            ) = (&node.call_chain[0], &node.call_chain[1])
            {
                let module_name = &first_id.name.lexeme;
                let method_name = &call_node.identifier.name.lexeme;
                let full_call = format!("{}.{}", module_name, method_name);

                if full_call == "subprocess.spawn" {
                    // Bug #54 Fix: Map subprocess.spawn to child_process.spawn
                    output.push_str("child_process.spawn(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                } else if full_call == "socket.createServer" {
                    // Bug #55 Fix: Map socket.createServer to net.createServer
                    output.push_str("net.createServer(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                } else if full_call == "json.parse" {
                    // Bug #56 Fix: Map json.parse to JSON.parse
                    output.push_str("JSON.parse(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                } else if full_call == "json.loads" {
                    output.push_str("FrameRuntime.jsonLoads(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                } else if full_call == "json.dumps" {
                    output.push_str("FrameRuntime.jsonDumps(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                } else if full_call == "ast.literal_eval" {
                    output.push_str("FrameRuntime.literalEval(");
                    if call_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                    return;
                }
            }
        }

        if node.call_chain.len() == 3 {
            if let (
                CallChainNodeType::UndeclaredIdentifierNodeT {
                    id_node: module_node,
                },
                CallChainNodeType::UndeclaredIdentifierNodeT {
                    id_node: submodule_node,
                },
                CallChainNodeType::UndeclaredCallT { call_node },
            ) = (
                &node.call_chain[0],
                &node.call_chain[1],
                &node.call_chain[2],
            ) {
                if module_node.name.lexeme == "os"
                    && submodule_node.name.lexeme == "path"
                    && call_node.identifier.name.lexeme == "join"
                {
                    output.push_str("pathLib.join(");
                    if !call_node.call_expr_list.exprs_t.is_empty() {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &call_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
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
            eprintln!(
                "DEBUG TS: Processing call chain with {} nodes",
                node.call_chain.len()
            );
            for (i, call_chain_node) in node.call_chain.iter().enumerate() {
                match call_chain_node {
                    CallChainNodeType::VariableNodeT { var_node } => {
                        eprintln!(
                            "DEBUG TS:   [{}] VariableNodeT: {}",
                            i, var_node.id_node.name.lexeme
                        );
                    }
                    CallChainNodeType::UndeclaredCallT { call_node } => {
                        eprintln!(
                            "DEBUG TS:   [{}] UndeclaredCallT: {}",
                            i, call_node.identifier.name.lexeme
                        );
                    }
                    CallChainNodeType::ActionCallT {
                        action_call_expr_node,
                    } => {
                        eprintln!(
                            "DEBUG TS:   [{}] ActionCallT: {}",
                            i, action_call_expr_node.identifier.name.lexeme
                        );
                    }
                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                        eprintln!(
                            "DEBUG TS:   [{}] UndeclaredIdentifierNodeT: {}",
                            i, id_node.name.lexeme
                        );
                    }
                    CallChainNodeType::SliceNodeT { slice_node } => {
                        eprintln!(
                            "DEBUG TS:   [{}] SliceNodeT: {} [{}:{}]",
                            i,
                            slice_node.identifier.name.lexeme,
                            slice_node
                                .start_expr
                                .as_ref()
                                .map(|_| "start")
                                .unwrap_or(""),
                            slice_node.end_expr.as_ref().map(|_| "end").unwrap_or("")
                        );
                    }
                    CallChainNodeType::UndeclaredSliceT { slice_node } => {
                        eprintln!(
                            "DEBUG TS:   [{}] UndeclaredSliceT: {} [{}:{}]",
                            i,
                            slice_node.identifier.name.lexeme,
                            slice_node
                                .start_expr
                                .as_ref()
                                .map(|_| "start")
                                .unwrap_or(""),
                            slice_node.end_expr.as_ref().map(|_| "end").unwrap_or("")
                        );
                    }
                    _ => {
                        eprintln!("DEBUG TS:   [{}] Other call chain node type", i);
                    }
                }
            }
        }

        let mut is_first = true;
        let mut needs_any_wrap = false;
        let mut skip_segments: usize = 0;
        for (index, call_chain_node) in node.call_chain.iter().enumerate() {
            if skip_segments > 0 {
                skip_segments -= 1;
                continue;
            }

            if let Some((binding, consumed)) = self.match_native_module_binding(node, index) {
                if let Some(identifier) = &binding.identifier {
                    if !is_first {
                        if !output.trim_end().ends_with('.') {
                            output.push('.');
                        }
                    }
                    output.push_str(identifier);
                    is_first = false;
                }
                skip_segments = consumed.saturating_sub(1);
                continue;
            }

            let prev_node = if index > 0 {
                Some(&node.call_chain[index - 1])
            } else {
                None
            };
            match call_chain_node {
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    if self.pending_super_call {
                        self.pending_super_call = false;
                        let method_name = &call_node.identifier.name.lexeme;
                        if method_name == "init"
                            || method_name == "__init__"
                            || method_name == "constructor"
                        {
                            output.push_str("super(");
                            if !call_node.call_expr_list.exprs_t.is_empty() {
                                let mut args_str = String::new();
                                self.visit_expr_list_node_to_string(
                                    &call_node.call_expr_list.exprs_t,
                                    &mut args_str,
                                );
                                output.push_str(&args_str);
                            }
                            output.push(')');
                            continue;
                        } else {
                            output.push_str("super.");
                            output.push_str(method_name);
                            output.push('(');
                            if !call_node.call_expr_list.exprs_t.is_empty() {
                                let mut args_str = String::new();
                                self.visit_expr_list_node_to_string(
                                    &call_node.call_expr_list.exprs_t,
                                    &mut args_str,
                                );
                                output.push_str(&args_str);
                            }
                            output.push(')');
                            continue;
                        }
                    }
                    // DEBUG: Print what we're processing
                    let debug_env =
                        std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1";
                    if debug_env {
                        eprintln!(
                            "DEBUG TS: Processing UndeclaredCallT method: {}, is_first: {}",
                            call_node.identifier.name.lexeme, is_first
                        );
                    }

                    let method_name = &call_node.identifier.name.lexeme;
                    let method_base = method_name.split('.').last().unwrap_or(method_name);

                    // Bug 53 fix: Track frameRuntime* function calls
                    if method_name.starts_with("frameRuntime") {
                        self.runtime_function_calls.insert(method_name.clone());
                    }

                    if method_name.starts_with("_action_") {
                        let wrapper_name = method_name.trim_start_matches("_action_");
                        if is_first {
                            output.push_str(&format!("this.{}(", wrapper_name));
                        } else {
                            if !output.trim_end().ends_with('.') {
                                output.push('.');
                            }
                            output.push_str(&format!("{}(", wrapper_name));
                        }
                        if !call_node.call_expr_list.exprs_t.is_empty() {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(
                                &call_node.call_expr_list.exprs_t,
                                &mut args_str,
                            );
                            output.push_str(&args_str);
                        }
                        output.push(')');
                        needs_any_wrap = true;
                        continue;
                    }

                    if self.action_names.contains(method_name) {
                        if is_first {
                            output.push_str(&format!("this.{}(", method_name));
                        } else {
                            if !output.trim_end().ends_with('.') {
                                output.push('.');
                            }
                            output.push_str(&format!("{}(", method_name));
                        }
                        if !call_node.call_expr_list.exprs_t.is_empty() {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(
                                &call_node.call_expr_list.exprs_t,
                                &mut args_str,
                            );
                            output.push_str(&args_str);
                        }
                        output.push(')');
                        needs_any_wrap = true;
                        continue;
                    }

                    // Check for special @indexed_call node
                    if method_name == "@indexed_call" {
                        // This is a synthetic node for array[index](args) or dict[key](args)
                        // Just output the arguments without any method name
                        output.push('(');
                        let mut first_arg = true;
                        for arg in &call_node.call_expr_list.exprs_t {
                            if !first_arg {
                                output.push_str(", ");
                            }
                            self.visit_expr_node_to_string(arg, output);
                            first_arg = false;
                        }
                        output.push(')');
                        return; // Don't process further
                    }

                    // Special handling for dictionary helpers using FrameDict runtime
                    if method_name == "fromkeys" {
                        let is_dict_static_call = prev_node.map_or(false, |prev| match prev {
                            CallChainNodeType::VariableNodeT { var_node } => {
                                var_node.id_node.name.lexeme == "dict"
                            }
                            CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                                id_node.name.lexeme == "dict"
                            }
                            _ => false,
                        });

                        if is_dict_static_call {
                            output.clear();
                            output.push_str("FrameDict.fromKeys(");
                            if let Some(keys_arg) = call_node.call_expr_list.exprs_t.get(0) {
                                self.visit_expr_node_to_string(keys_arg, output);
                            } else {
                                output.push_str("[]");
                            }
                            if let Some(value_arg) = call_node.call_expr_list.exprs_t.get(1) {
                                output.push_str(", ");
                                self.visit_expr_node_to_string(value_arg, output);
                            }
                            output.push(')');
                            continue;
                        }
                    }

                    if !is_first {
                        let target_expr = output.clone();
                        if debug_env && method_name.contains("rfind") {
                            eprintln!("DEBUG TS: target_expr: {}", target_expr);
                            eprintln!("DEBUG TS: method_name raw: {}", method_name);
                            eprintln!("DEBUG TS: method_base: {}", method_base);
                        }
                        let target_trimmed = target_expr.trim().to_string();
                        let args = &call_node.call_expr_list.exprs_t;

                        let is_string_method = matches!(
                            method_base,
                            "upper"
                                | "lower"
                                | "strip"
                                | "lstrip"
                                | "rstrip"
                                | "replace"
                                | "split"
                                | "splitlines"
                                | "join"
                                | "startswith"
                                | "endswith"
                                | "find"
                                | "rfind"
                                | "rindex"
                                | "partition"
                                | "rpartition"
                                | "rsplit"
                                | "center"
                                | "ljust"
                                | "rjust"
                                | "zfill"
                                | "format"
                                | "format_map"
                                | "encode"
                                | "expandtabs"
                        );

                        if is_string_method {
                            output.clear();
                            output.push_str(&target_expr);
                            output.push('.');
                            output.push_str(method_base);
                            output.push('(');
                            for (idx, arg) in args.iter().enumerate() {
                                if idx > 0 {
                                    output.push_str(", ");
                                }
                                self.visit_expr_node_to_string(arg, output);
                            }
                            output.push(')');
                            continue;
                        }

                        if self.is_counter_reference(&target_trimmed) {
                            match method_base {
                                "update" => {
                                    output.clear();
                                    output.push_str("Counter.update(");
                                    output.push_str(&target_expr);
                                    if let Some(arg) = args.get(0) {
                                        output.push_str(", ");
                                        self.visit_expr_node_to_string(arg, output);
                                    } else {
                                        output.push_str(", []");
                                    }
                                    output.push(')');
                                    continue;
                                }
                                "most_common" => {
                                    output.clear();
                                    output.push_str("Counter.mostCommon(");
                                    output.push_str(&target_expr);
                                    if let Some(arg) = args.get(0) {
                                        output.push_str(", ");
                                        self.visit_expr_node_to_string(arg, output);
                                    }
                                    output.push(')');
                                    continue;
                                }
                                _ => {}
                            }
                        }

                        match method_base {
                            "append" => {
                                output.clear();
                                output.push_str(&target_expr);
                                output.push_str(".push(");
                                if let Some(arg) = args.get(0) {
                                    self.visit_expr_node_to_string(arg, output);
                                }
                                output.push(')');
                                continue;
                            }
                            "extend" => {
                                output.clear();
                                output.push_str("FrameCollections.listExtend(");
                                output.push_str(&target_expr);
                                output.push_str(", ");
                                if let Some(arg) = args.get(0) {
                                    self.visit_expr_node_to_string(arg, output);
                                } else {
                                    output.push_str("[]");
                                }
                                output.push(')');
                                continue;
                            }
                            "insert" => {
                                output.clear();
                                output.push_str("FrameCollections.listInsert(");
                                output.push_str(&target_expr);
                                output.push_str(", ");
                                if let Some(index_arg) = args.get(0) {
                                    self.visit_expr_node_to_string(index_arg, output);
                                } else {
                                    output.push_str("0");
                                }
                                output.push_str(", ");
                                if let Some(value_arg) = args.get(1) {
                                    self.visit_expr_node_to_string(value_arg, output);
                                } else {
                                    output.push_str("undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "remove" => {
                                output.clear();
                                output.push_str("FrameCollections.listRemove(");
                                output.push_str(&target_expr);
                                output.push_str(", ");
                                if let Some(value_arg) = args.get(0) {
                                    self.visit_expr_node_to_string(value_arg, output);
                                } else {
                                    output.push_str("undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "clear" => {
                                output.clear();
                                output.push_str("(FrameCollections.listClear(");
                                output.push_str(&target_expr);
                                output.push_str("), undefined)");
                                continue;
                            }
                            "pop" => {
                                output.clear();
                                output.push_str("FrameCollections.listPop(");
                                output.push_str(&target_expr);
                                if let Some(index_arg) = args.get(0) {
                                    output.push_str(", ");
                                    self.visit_expr_node_to_string(index_arg, output);
                                }
                                output.push(')');
                                continue;
                            }
                            "count" => {
                                output.clear();
                                output.push_str("FrameCollections.listCount(");
                                output.push_str(&target_expr);
                                output.push_str(", ");
                                if let Some(value_arg) = args.get(0) {
                                    self.visit_expr_node_to_string(value_arg, output);
                                } else {
                                    output.push_str("undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "copy" => {
                                output.clear();
                                output.push_str("FrameCollections.listCopy(");
                                output.push_str(&target_expr);
                                output.push(')');
                                continue;
                            }
                            _ => {}
                        }

                        let dict_expr = target_expr.clone();
                        match method_name.as_str() {
                            "get" => {
                                output.clear();
                                output.push_str("FrameDict.get(");
                                output.push_str(&dict_expr);
                                if let Some(key_arg) = call_node.call_expr_list.exprs_t.get(0) {
                                    output.push_str(", ");
                                    self.visit_expr_node_to_string(key_arg, output);
                                    if let Some(default_arg) =
                                        call_node.call_expr_list.exprs_t.get(1)
                                    {
                                        output.push_str(", ");
                                        self.visit_expr_node_to_string(default_arg, output);
                                    }
                                } else {
                                    output.push_str(", undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "setdefault" => {
                                output.clear();
                                output.push_str("FrameDict.setdefault(");
                                output.push_str(&dict_expr);
                                if let Some(key_arg) = call_node.call_expr_list.exprs_t.get(0) {
                                    output.push_str(", ");
                                    self.visit_expr_node_to_string(key_arg, output);
                                    if let Some(default_arg) =
                                        call_node.call_expr_list.exprs_t.get(1)
                                    {
                                        output.push_str(", ");
                                        self.visit_expr_node_to_string(default_arg, output);
                                    }
                                } else {
                                    output.push_str(", undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "update" => {
                                output.clear();
                                output.push_str("FrameDict.update(");
                                output.push_str(&dict_expr);
                                if !call_node.call_expr_list.exprs_t.is_empty() {
                                    output.push_str(", ");
                                    let mut args_str = String::new();
                                    self.visit_expr_list_node_to_string(
                                        &call_node.call_expr_list.exprs_t,
                                        &mut args_str,
                                    );
                                    output.push_str(&args_str);
                                }
                                output.push(')');
                                continue;
                            }
                            _ => {}
                        }
                    }

                    // Regular method call handling
                    if !is_first {
                        output.push('.');
                    } else if call_node.identifier.name.lexeme != "print" {
                        // For first call in chain that's not print, check if it needs 'this.'
                        // This handles method calls on self
                    }
                    self.visit_call_expr_node_to_string_with_context(call_node, output, is_first);
                    needs_any_wrap = true;
                }
                CallChainNodeType::InterfaceMethodCallT {
                    interface_method_call_expr_node,
                } => {
                    if !is_first {
                        output.push('.');
                    } else {
                        // Bug #52 Fix: Interface method calls need 'this.' prefix in TypeScript
                        // Frame: system.interfaceMethod() -> TypeScript: this.interfaceMethod()
                        output.push_str("this.");
                    }
                    // Interface methods are method calls on the class instance
                    output.push_str(&format!(
                        "{}(",
                        interface_method_call_expr_node.identifier.name.lexeme
                    ));
                    if interface_method_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                        let mut args_str = String::new();
                        self.visit_expr_list_node_to_string(
                            &interface_method_call_expr_node.call_expr_list.exprs_t,
                            &mut args_str,
                        );
                        output.push_str(&args_str);
                    }
                    output.push(')');
                }
                CallChainNodeType::ActionCallT {
                    action_call_expr_node,
                } => {
                    if !is_first {
                        output.push('.');
                    }

                    let method_name = &action_call_expr_node.identifier.name.lexeme;

                    // DEBUG: Print what we're processing
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!(
                            "DEBUG TS: Processing ActionCallT method: {}, is_first: {}",
                            method_name, is_first
                        );
                    }

                    // Check if this is a native array/list method that should be mapped
                    let mapped_method = match method_name.as_str() {
                        "append" => "push",   // Python append() -> JavaScript push()
                        "pop" => "pop",       // handled separately for index support
                        "remove" => "splice", // handled via runtime helper
                        "index" => "indexOf", // Python index() -> JavaScript indexOf()
                        "count" => "filter",  // handled via runtime helper
                        "clear" => "splice",  // handled via runtime helper
                        "extend" => "push",   // handled via runtime helper
                        "insert" => "splice", // handled via runtime helper
                        _ => method_name,     // Use original method name for other cases
                    };

                    if mapped_method != method_name {
                        let args = &action_call_expr_node.call_expr_list.exprs_t;
                        match method_name.as_str() {
                            "remove" => {
                                let list_expr = output.clone();
                                output.clear();
                                output.push_str("FrameCollections.listRemove(");
                                output.push_str(&list_expr);
                                output.push_str(", ");
                                if let Some(arg) = args.get(0) {
                                    self.visit_expr_node_to_string(arg, output);
                                } else {
                                    output.push_str("undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "copy" => {
                                let list_expr = output.clone();
                                output.clear();
                                output.push_str("FrameCollections.listCopy(");
                                output.push_str(&list_expr);
                                output.push(')');
                                continue;
                            }
                            "extend" => {
                                let list_expr = output.clone();
                                output.clear();
                                output.push_str("FrameCollections.listExtend(");
                                output.push_str(&list_expr);
                                output.push_str(", ");
                                if let Some(arg) = args.get(0) {
                                    self.visit_expr_node_to_string(arg, output);
                                } else {
                                    output.push_str("[]");
                                }
                                output.push(')');
                                continue;
                            }
                            "insert" => {
                                let list_expr = output.clone();
                                output.clear();
                                output.push_str("FrameCollections.listInsert(");
                                output.push_str(&list_expr);
                                output.push_str(", ");
                                if let Some(index_arg) = args.get(0) {
                                    self.visit_expr_node_to_string(index_arg, output);
                                } else {
                                    output.push_str("0");
                                }
                                output.push_str(", ");
                                if let Some(value_arg) = args.get(1) {
                                    self.visit_expr_node_to_string(value_arg, output);
                                } else {
                                    output.push_str("undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "count" => {
                                let list_expr = output.clone();
                                output.clear();
                                output.push_str("FrameCollections.listCount(");
                                output.push_str(&list_expr);
                                output.push_str(", ");
                                if let Some(arg) = args.get(0) {
                                    self.visit_expr_node_to_string(arg, output);
                                } else {
                                    output.push_str("undefined");
                                }
                                output.push(')');
                                continue;
                            }
                            "clear" => {
                                let list_expr = output.clone();
                                output.clear();
                                output.push_str("(FrameCollections.listClear(");
                                output.push_str(&list_expr);
                                output.push_str("), undefined)");
                                continue;
                            }
                            "pop" => {
                                let list_expr = output.clone();
                                output.clear();
                                output.push_str("FrameCollections.listPop(");
                                output.push_str(&list_expr);
                                if let Some(arg) = args.get(0) {
                                    output.push_str(", ");
                                    self.visit_expr_node_to_string(arg, output);
                                }
                                output.push(')');
                                continue;
                            }
                            _ => {}
                        }

                        // This is a native method - use the mapped name without action prefix
                        output.push_str(&format!("{}(", mapped_method));
                        if action_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(
                                &action_call_expr_node.call_expr_list.exprs_t,
                                &mut args_str,
                            );
                            output.push_str(&args_str);
                        }
                        output.push(')');
                    } else {
                        // Check if this is a real action call or an external API call
                        // If it's not the first in the chain, it's likely an external API method
                        if self.action_names.contains(method_name) {
                            if is_first {
                                // This is a real action - call through wrapper on current instance
                                output.push_str(&format!("this.{}(", method_name));
                            } else {
                                // Action invoked on explicit target - ensure property access and call wrapper
                                if !output.trim_end().ends_with('.') {
                                    output.push('.');
                                }
                                output.push_str(&format!("{}(", method_name));
                            }
                        } else {
                            // This is an external API method call (e.g., child_process.spawn)
                            // Don't add the action prefix but ensure proper property access
                            if !output.trim_end().ends_with('.') {
                                output.push('.');
                            }
                            output.push_str(&format!("{}(", method_name));
                        }
                        if action_call_expr_node.call_expr_list.exprs_t.len() > 0 {
                            let mut args_str = String::new();
                            self.visit_expr_list_node_to_string(
                                &action_call_expr_node.call_expr_list.exprs_t,
                                &mut args_str,
                            );
                            output.push_str(&args_str);
                        }
                        output.push(')');
                        needs_any_wrap = true;
                    }
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    // Variables should use context-aware resolution when they're first in the chain
                    if is_first {
                        let var_name = &var_node.id_node.name.lexeme;

                        if debug_enabled {
                            eprintln!("DEBUG TS: Processing CallChainNodeType::VariableNodeT variable: {}", var_name);
                        }

                        // Handle Python-style keywords
                        if var_name == "pass" {
                            output.push_str("// pass");
                            break; // Don't process further
                        } else if var_name == "True" {
                            output.push_str("true");
                            break;
                        } else if var_name == "False" {
                            output.push_str("false");
                            break;
                        }

                        if self.current_local_vars.contains(var_name) {
                            // Local variable - use bare name
                            output.push_str(var_name);
                        } else if self.current_exception_vars.contains(var_name) {
                            // Bug #53 Fix: Exception variables are local, not instance properties
                            output.push_str(var_name);
                        } else if self.current_state_params.contains(var_name) {
                            // State parameter - access from compartment
                            if self.in_state_var_initializer {
                                output.push_str(&format!("__frameStateArg_{}", var_name));
                            } else {
                                output.push_str(&format!("compartment.stateArgs['{}']", var_name));
                            }
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
                        } else if var_name == "math" {
                            output.push_str("FrameMath");
                            self.pending_frame_math_property = true;
                        } else if var_name == "round" {
                            output.push_str("FrameMath.round");
                            return;
                        } else if var_name == "min" {
                            output.push_str("FrameMath.min");
                            return;
                        } else if var_name == "max" {
                            output.push_str("FrameMath.max");
                            return;
                        } else {
                            // Unknown variable - output naturally like Python visitor
                            output.push_str(var_name);
                        }
                    } else {
                        let property = &var_node.id_node.name.lexeme;
                        if needs_any_wrap {
                            let current = output.clone();
                            output.clear();
                            output.push_str("((");
                            output.push_str(&current);
                            output.push_str(") as any)");
                        }
                        if self.pending_frame_math_property {
                            output.push('.');
                            output.push_str(property);
                            self.pending_frame_math_property = false;
                        } else {
                            output.push('.');
                            output.push_str(property);
                        }
                        needs_any_wrap = true;
                    }
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    // Undeclared identifiers might be variables too - use context-aware resolution if first
                    if is_first {
                        // Check if it's 'self' or 'super'
                        if id_node.name.lexeme == "self" {
                            output.push_str("this");
                        } else if id_node.name.lexeme == "super" {
                            self.pending_super_call = true;
                            continue;
                        } else {
                            let var_name = &id_node.name.lexeme;

                            if debug_enabled {
                                eprintln!("DEBUG TS: Processing CallChainNodeType::UndeclaredIdentifierNodeT variable: {}", var_name);
                            }

                            // Handle Python-style keywords as undeclared identifiers
                            if var_name == "True" {
                                output.push_str("true");
                                return;
                            } else if var_name == "False" {
                                output.push_str("false");
                                return;
                            } else if var_name == "pass" {
                                output.push_str("// pass");
                                return;
                            }

                            if self.current_local_vars.contains(var_name) {
                                // Local variable - use bare name
                                output.push_str(var_name);
                            } else if self.current_exception_vars.contains(var_name) {
                                output.push_str(var_name);
                            } else if self.current_state_params.contains(var_name) {
                                // State parameter - access from compartment
                                if self.in_state_var_initializer {
                                    output.push_str(&format!("__frameStateArg_{}", var_name));
                                } else {
                                    output.push_str(&format!(
                                        "compartment.stateArgs['{}']",
                                        var_name
                                    ));
                                }
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
                                output.push_str("this");
                            } else if var_name == "system.return" {
                                output.push_str("this.returnStack[this.returnStack.length - 1]");
                            } else if var_name.starts_with("system.") {
                                let method_name = &var_name[7..];
                                output.push_str(&format!("this.{}", method_name));
                            } else if var_name == "math" {
                                output.push_str("FrameMath");
                                self.pending_frame_math_property = true;
                            } else if var_name == "round" {
                                output.push_str("FrameMath.round");
                                return;
                            } else if var_name == "min" {
                                output.push_str("FrameMath.min");
                                return;
                            } else if var_name == "max" {
                                output.push_str("FrameMath.max");
                                return;
                            } else {
                                // Unknown variable - output naturally like Python visitor
                                output.push_str(var_name);
                            }
                        }
                    } else {
                        let property_name = &id_node.name.lexeme;
                        if self.pending_frame_math_property {
                            if needs_any_wrap {
                                let current = output.clone();
                                output.clear();
                                output.push_str("((");
                                output.push_str(&current);
                                output.push_str(") as any)");
                            }
                            output.push('.');
                            output.push_str(property_name);
                            self.pending_frame_math_property = false;
                        } else {
                            if needs_any_wrap {
                                let current = output.clone();
                                output.clear();
                                output.push_str("((");
                                output.push_str(&current);
                                output.push_str(") as any)");
                            }
                            output.push('.');
                            let resolved_name = self.resolve_domain_variable_name(property_name);
                            output.push_str(&resolved_name);

                            // Bug 53 fix: Track dynamic property assignments for this.propertyName patterns
                            // Check if we're in an assignment context and the output so far ends with "this"
                            if output.ends_with(&format!("this.{}", resolved_name)) {
                                self.dynamic_properties.insert(resolved_name.clone());
                            }
                        }
                        needs_any_wrap = true;
                    }
                }
                CallChainNodeType::SelfT { .. } => {
                    if is_first {
                        output.push_str("this");
                    }
                }
                CallChainNodeType::CallChainLiteralExprT {
                    call_chain_literal_expr_node,
                } => {
                    // Handle literal in call chain - use comprehensive literal processing
                    // Create a temporary ExprType to reuse the enhanced literal handling
                    let temp_literal_node = LiteralExprNode {
                        line: 0,
                        token_t: call_chain_literal_expr_node.token_t.clone(),
                        value: call_chain_literal_expr_node.value.clone(),
                        is_reference: false,
                        inc_dec: IncDecExpr::None,
                    };

                    let temp_expr = ExprType::LiteralExprT {
                        literal_expr_node: temp_literal_node,
                    };

                    // Use the enhanced expression handler that supports all string types
                    self.visit_expr_node_to_string(&temp_expr, output);
                }
                CallChainNodeType::ListElementNodeT { list_elem_node } => {
                    // Handle array/string indexing operations like text[i]
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!(
                            "DEBUG TS: Processing ListElementNodeT for array/string indexing"
                        );
                    }

                    // Generate the variable name (skip synthetic identifiers)
                    if list_elem_node.identifier.name.lexeme != "@chain_index"
                        && list_elem_node.identifier.name.lexeme != "@chain_slice"
                    {
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
                    // Generate the variable name if this is the first node in chain
                    if is_first {
                        let var_name = &slice_node.identifier.name.lexeme;
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!("DEBUG TS: SliceNodeT is first - generating variable '{}' before .slice()", var_name);
                        }

                        // Use context-aware variable resolution
                        if self.current_local_vars.contains(var_name) {
                            output.push_str(var_name);
                        } else if self.current_exception_vars.contains(var_name) {
                            output.push_str(var_name);
                        } else if self.current_state_params.contains(var_name) {
                            output.push_str(&format!("compartment.stateArgs['{}']", var_name));
                        } else if self.current_state_vars.contains(var_name) {
                            output.push_str(&format!("compartment.stateVars['{}']", var_name));
                        } else if self.domain_variables.contains(var_name) {
                            output.push_str(&format!("this.{}", var_name));
                        } else if self.current_handler_params.contains(var_name) {
                            output.push_str(&format!("__e.parameters.{}", var_name));
                        } else {
                            // Unknown variable - output naturally
                            output.push_str(var_name);
                        }
                    }

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
                    // Generate the variable name if this is the first node in chain
                    if is_first {
                        let var_name = &slice_node.identifier.name.lexeme;
                        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                            eprintln!("DEBUG TS: UndeclaredSliceT is first - generating variable '{}' before .slice()", var_name);
                        }

                        // Use context-aware variable resolution
                        if self.current_local_vars.contains(var_name) {
                            output.push_str(var_name);
                        } else if self.current_exception_vars.contains(var_name) {
                            output.push_str(var_name);
                        } else if self.current_state_params.contains(var_name) {
                            output.push_str(&format!("compartment.stateArgs['{}']", var_name));
                        } else if self.current_state_vars.contains(var_name) {
                            output.push_str(&format!("compartment.stateVars['{}']", var_name));
                        } else if self.domain_variables.contains(var_name) {
                            output.push_str(&format!("this.{}", var_name));
                        } else if self.current_handler_params.contains(var_name) {
                            output.push_str(&format!("__e.parameters.{}", var_name));
                        } else {
                            // Unknown variable - output naturally
                            output.push_str(var_name);
                        }
                    }

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
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    // Handle undeclared list element access (chained indexing)
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!(
                            "DEBUG TS: Processing UndeclaredListElementT for chained indexing"
                        );
                    }

                    // Generate the variable name (skip synthetic identifiers)
                    if list_elem_node.identifier.name.lexeme != "@chain_index"
                        && list_elem_node.identifier.name.lexeme != "@chain_slice"
                    {
                        output.push_str(&list_elem_node.identifier.name.lexeme);
                    }

                    // Generate the index expression
                    output.push('[');
                    let mut index_str = String::new();
                    self.visit_expr_node_to_string(&list_elem_node.expr_t, &mut index_str);
                    output.push_str(&index_str);
                    output.push(']');
                }
                _ => {
                    // TODO: Handle other call chain node types
                    if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                        eprintln!("DEBUG TS: Unhandled call chain node type - adding TODO comment");
                        eprintln!(
                            "DEBUG TS: Unknown node type: {:?}",
                            std::mem::discriminant(call_chain_node)
                        );
                    }
                    output.push_str("/* TODO: call chain node */");
                }
            }
            is_first = false;
        }

        self.pending_frame_math_property = false;
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

        // Generate function signature with async support
        if function_node.is_async {
            self.builder.writeln(&format!(
                "export async function {}({}): Promise<any> {{",
                func_name, params_str
            ));
        } else {
            self.builder.writeln(&format!(
                "export function {}({}): any {{",
                func_name, params_str
            ));
        }
        self.builder.indent();

        // Set module function flag
        let old_flag = self.in_module_function;
        self.in_module_function = true;

        // Generate function body statements
        for stmt in &function_node.statements {
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
                        self.builder
                            .writeln(&format!("var {} = {};", var_decl.name, init_str));
                    } else {
                        self.builder
                            .writeln(&format!("var {}: any = null;", var_decl.name));
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
                }
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

        self.builder
            .writeln("private handlePythonStdout(data: any): void {");
        self.builder.indent();
        self.builder
            .writeln("// Implementation provided by runtime environment");
        self.builder
            .writeln("console.log('[Python stdout]:', data);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        self.builder
            .writeln("private handlePythonStderr(data: any): void {");
        self.builder.indent();
        self.builder
            .writeln("// Implementation provided by runtime environment");
        self.builder
            .writeln("console.error('[Python stderr]:', data);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        self.builder
            .writeln("private handlePythonExit(exitCode: number): void {");
        self.builder.indent();
        self.builder
            .writeln("// Implementation provided by runtime environment");
        self.builder
            .writeln("console.log('[Python exit]:', exitCode);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        self.builder
            .writeln("private handlePythonError(error: any): void {");
        self.builder.indent();
        self.builder
            .writeln("// Implementation provided by runtime environment");
        self.builder
            .writeln("console.error('[Python error]:', error);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();

        self.builder
            .writeln("private handleRuntimeConnection(socket: any): void {");
        self.builder.indent();
        self.builder
            .writeln("// Implementation provided by runtime environment");
        self.builder
            .writeln("console.log('[Runtime connection]:', socket);");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.newline();
    }

    // Helper method to resolve domain variable names with case variations
    fn resolve_domain_variable_name(&self, var_name: &str) -> String {
        if std::env::var("DEBUG_TS_VARS").is_ok() {
            eprintln!(
                "DEBUG TS: resolve_domain_variable_name called with '{}'",
                var_name
            );
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
                    eprintln!(
                        "DEBUG TS: Resolved '{}' to domain variable '{}'",
                        var_name, domain_var
                    );
                }
                return domain_var.clone();
            }
        }

        // No match found, return original
        if std::env::var("DEBUG_TS_VARS").is_ok() {
            eprintln!(
                "DEBUG TS: No match found for '{}', returning original",
                var_name
            );
        }
        var_name.to_string()
    }

    fn comprehension_binding_parts(target: &str) -> (String, Vec<String>) {
        let trimmed = target.trim();
        let inner = if trimmed.starts_with('(') && trimmed.ends_with(')') && trimmed.len() > 1 {
            trimmed[1..trimmed.len() - 1].trim()
        } else {
            trimmed
        };

        let parts: Vec<String> = inner
            .split(',')
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .map(|part| part.to_string())
            .collect();

        if parts.len() > 1 {
            let destructured = format!("[{}]", parts.join(", "));
            (format!("({})", destructured), parts)
        } else {
            let single = parts
                .into_iter()
                .next()
                .unwrap_or_else(|| inner.to_string());
            (single.clone(), vec![single])
        }
    }

    fn is_simple_dict_key(&self, key: &ExprType) -> bool {
        match key {
            ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t {
                TokenType::String | TokenType::Number => true,
                _ => false,
            },
            _ => false,
        }
    }

    // Helper method to convert Python f-strings to TypeScript template literals
    fn convert_fstring_to_template_literal(&self, fstring: &str) -> String {
        // f"Hello {name}" -> `Hello ${name}` with context-aware variable resolution

        let debug_enabled = std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1";

        // Check if it starts with f" or f'
        let content = if fstring.starts_with("f\"") && fstring.ends_with("\"") {
            &fstring[2..fstring.len() - 1]
        } else if fstring.starts_with("f'") && fstring.ends_with("'") {
            &fstring[2..fstring.len() - 1]
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

                    // Handle expressions that are not simple variables
                    if var_name.contains(&['+', '-', '*', '/', '%', '(', ')', ':', '='][..]) {
                        // This looks like an expression, not a simple variable
                        // Clean up obvious errors like "this.2" -> "2", "this.3.14159:.2f" -> "3.14159"
                        let cleaned_expr = var_name
                            .replace("this.", "") // Remove erroneous "this." prefixes
                            .split(':')
                            .next()
                            .unwrap_or(&var_name) // Remove format specifiers like :.2f
                            .to_string();
                        result.push_str(&cleaned_expr);
                    } else if var_name == "self" {
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
                                eprintln!(
                                    "DEBUG TS: Compound access - base: '{}', property: '{}'",
                                    base_var, property_access
                                );
                            }

                            // Apply context-aware resolution to the base variable
                            if self.current_local_vars.contains(base_var) {
                                // Local variable - use bare name
                                result.push_str(&format!("{}.{}", base_var, property_access));
                            } else if self.current_state_params.contains(base_var) {
                                // State parameter - access from compartment
                                result.push_str(&format!(
                                    "compartment.stateArgs['{}'].{}",
                                    base_var, property_access
                                ));
                            } else if self.current_state_vars.contains(base_var) {
                                // State variable - access from compartment
                                result.push_str(&format!(
                                    "compartment.stateVars['{}'].{}",
                                    base_var, property_access
                                ));
                            } else if self.domain_variables.contains(base_var) {
                                // Domain variable - access from this
                                result.push_str(&format!("this.{}.{}", base_var, property_access));
                            } else if self.current_handler_params.contains(base_var) {
                                // Event handler parameter - access from event parameters (keep original property names)
                                result.push_str(&format!(
                                    "__e.parameters.{}.{}",
                                    base_var, property_access
                                ));
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

    fn dedup_imports(code: String) -> String {
        let mut seen: HashSet<String> = HashSet::new();
        let mut deduped_lines: Vec<String> = Vec::new();

        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ")
                || (trimmed.starts_with("const ") && trimmed.contains("require("))
            {
                if seen.insert(trimmed.to_string()) {
                    deduped_lines.push(line.to_string());
                } else {
                    continue;
                }
            } else {
                deduped_lines.push(line.to_string());
            }
        }

        let mut deduped_code = deduped_lines.join("\n");
        if code.ends_with('\n') {
            deduped_code.push('\n');
        }
        deduped_code
    }

    fn visit_dict_literal_node_to_string(&mut self, node: &DictLiteralNode, output: &mut String) {
        output.push('{');
        let mut first = true;
        for (key, value) in &node.pairs {
            if !first {
                output.push_str(", ");
            }
            first = false;

            // Check if this is a dict unpacking (key is DictUnpackExprT, value is NilExprT)
            if matches!(*key, ExprType::DictUnpackExprT { .. }) {
                // This is a dict unpacking, just output the spread key (not value)
                self.visit_expr_node_to_string(key, output);
            } else {
                if let Some(identifier_key) = self.dict_key_identifier(key) {
                    output.push('\'');
                    output.push_str(&identifier_key);
                    output.push('\'');
                    output.push_str(": ");
                    self.visit_expr_node_to_string(value, output);
                    continue;
                }

                // Regular key-value pair
                if !self.is_simple_dict_key(key) {
                    // Use computed property syntax with FrameDict key normalization
                    output.push('[');
                    output.push_str("FrameDict.normalizeKey(");
                    self.visit_expr_node_to_string(key, output);
                    output.push_str(")]");
                } else {
                    // Simple key - output directly
                    self.visit_expr_node_to_string(key, output);
                }
                output.push_str(": ");
                self.visit_expr_node_to_string(value, output);
            }
        }
        output.push('}');
    }

    fn visit_list_node_to_string(&mut self, node: &ListNode, output: &mut String) {
        // Special case: if the list contains a single list comprehension,
        // don't wrap it in brackets as the comprehension already produces an array
        if node.exprs_t.len() == 1 {
            if let ExprType::ListComprehensionExprT { .. } = &node.exprs_t[0] {
                // Just output the comprehension without wrapping brackets
                self.visit_expr_node_to_string(&node.exprs_t[0], output);
                return;
            }
        }

        // Normal list literal
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
            eprintln!(
                "DEBUG: Processing ForStmt with is_enum_iteration={}",
                node.is_enum_iteration
            );
        }

        if node.is_enum_iteration {
            // Handle enum iteration: for x in EnumType -> for (const x of Object.values(EnumType))
            let var_name = if let Some(ref variable) = node.variable {
                &variable.id_node.name.lexeme
            } else if let Some(ref identifier) = node.identifier {
                &identifier.name.lexeme
            } else {
                "item" // fallback
            };

            let mut iterable_str = String::new();
            self.visit_expr_node_to_string(&node.iterable, &mut iterable_str);
            self.flush_pending_walrus_decls();

            // Generate TypeScript for-of loop over enum values
            // For enum iteration, use the enum name directly, not through (this as any)
            let enum_name = if let Some(ref enum_type_name) = node.enum_type_name {
                enum_type_name.clone()
            } else {
                iterable_str.clone()
            };

            // Add the loop variable to current_local_vars so it's recognized in expressions
            self.current_local_vars.insert(var_name.to_string());

            self.builder.writeln(&format!(
                "for (const {} of Object.values({})) {{",
                var_name, enum_name
            ));
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
                                self.builder
                                    .writeln(&format!("var {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder
                                    .writeln(&format!("{} = {};", var_decl.name, init_str));
                            }
                        } else {
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder
                                    .writeln(&format!("var {}: any = null;", var_decl.name));
                            }
                        }
                    }
                }
            }

            self.builder.dedent();
            self.builder.writeln("}");

            // Handle optional else clause (executed if loop didn't break)
            if let Some(ref else_block) = node.else_block {
                self.builder
                    .writeln("// else clause (executed if no break)");
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
                                self.builder
                                    .writeln(&format!("var {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder
                                    .writeln(&format!("var {}: any = null;", var_decl.name));
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
                "item" // fallback
            };

            let mut iterable_str = String::new();
            self.visit_expr_node_to_string(&node.iterable, &mut iterable_str);
            self.flush_pending_walrus_decls();

            // Add the loop variable to current_local_vars so it's recognized in expressions
            self.current_local_vars.insert(var_name.to_string());

            // Generate TypeScript for-of loop
            self.builder.writeln(&format!(
                "for (const {} of FrameRuntime.iterable({})) {{",
                var_name, iterable_str
            ));
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
                                self.builder
                                    .writeln(&format!("var {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder
                                    .writeln(&format!("{} = {};", var_decl.name, init_str));
                            }
                        } else {
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder
                                    .writeln(&format!("var {}: any = null;", var_decl.name));
                            }
                        }
                    }
                }
            }

            self.builder.dedent();
            self.builder.writeln("}");

            // Handle optional else clause
            if let Some(ref else_block) = node.else_block {
                self.builder
                    .writeln("// else clause (executed if no break)");
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
                                self.builder
                                    .writeln(&format!("var {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder
                                    .writeln(&format!("var {}: any = null;", var_decl.name));
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
        self.flush_pending_walrus_decls();

        self.builder
            .writeln(&format!("while ({}) {{", condition_str));
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
                            self.builder
                                .writeln(&format!("var {} = {};", var_decl.name, init_str));
                        } else {
                            self.builder
                                .writeln(&format!("{} = {};", var_decl.name, init_str));
                        }
                    } else {
                        if !self.current_local_vars.contains(&var_decl.name) {
                            self.current_local_vars.insert(var_decl.name.clone());
                            self.builder
                                .writeln(&format!("var {}: any = null;", var_decl.name));
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
            self.builder
                .writeln("// TODO: TypeScript doesn't support while-else, implement with flag");
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
                            self.builder
                                .writeln(&format!("var {} = {};", var_decl.name, init_str));
                        } else {
                            self.builder
                                .writeln(&format!("var {}: any = null;", var_decl.name));
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
                                self.builder
                                    .writeln(&format!("var {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder
                                    .writeln(&format!("{} = {};", var_decl.name, init_str));
                            }
                        } else {
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder
                                    .writeln(&format!("var {}: any = null;", var_decl.name));
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
                        let type_checks = exception_types
                            .iter()
                            .map(|t| {
                                // Map Frame exception types to TypeScript/JavaScript types
                                let ts_type = match t.as_str() {
                                    "Exception" => "Error",
                                    "ValueError" => "Error",
                                    "TypeError" => "TypeError",
                                    "RuntimeError" => "Error",
                                    "ZeroDivisionError" => "Error",
                                    _ => "Error",
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
                            self.builder.writeln(&format!("var {} = e;", var_name));
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
                                    self.builder
                                        .writeln(&format!("var {} = {};", var_decl.name, init_str));
                                } else {
                                    self.builder
                                        .writeln(&format!("{} = {};", var_decl.name, init_str));
                                }
                            } else {
                                if !self.current_local_vars.contains(&var_decl.name) {
                                    self.current_local_vars.insert(var_decl.name.clone());
                                    self.builder
                                        .writeln(&format!("var {}: any = null;", var_decl.name));
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
                                self.builder
                                    .writeln(&format!("var {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder
                                    .writeln(&format!("{} = {};", var_decl.name, init_str));
                            }
                        } else {
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder
                                    .writeln(&format!("var {}: any = null;", var_decl.name));
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
            self.builder
                .writeln("// else block (executes if no exception occurred)");
            self.builder
                .writeln("// Note: Simulated since TypeScript doesn't have try-else");
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
                                self.builder
                                    .writeln(&format!("var {} = {};", var_decl.name, init_str));
                            } else {
                                self.builder
                                    .writeln(&format!("{} = {};", var_decl.name, init_str));
                            }
                        } else {
                            if !self.current_local_vars.contains(&var_decl.name) {
                                self.current_local_vars.insert(var_decl.name.clone());
                                self.builder
                                    .writeln(&format!("var {}: any = null;", var_decl.name));
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
        let is_generator = Self::function_contains_yield(function_node);

        let prev_generator_flag = self.in_generator_function;
        self.in_generator_function = is_generator;

        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
            eprintln!(
                "DEBUG: visit_function_node called for function: {}",
                function_node.name
            );
        }

        let params = if let Some(params) = &function_node.params {
            params
                .iter()
                .map(|p| format!("{}: any", p.param_name))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };

        self.builder.newline();

        let saved_locals = self.current_local_vars.clone();
        let saved_counters = self.counter_variables.clone();
        let saved_walrus = self.walrus_declared_locals.clone();
        let saved_pending_walrus = self.pending_walrus_decls.clone();
        self.current_local_vars.clear();
        self.counter_variables.clear();
        self.walrus_declared_locals.clear();
        self.pending_walrus_decls.clear();
        if let Some(params) = &function_node.params {
            for param in params {
                self.current_local_vars.insert(param.param_name.clone());
            }
        }

        // Generate TypeScript function with proper async/generator support
        let mut function_prefix = String::new();
        if function_node.is_async {
            function_prefix.push_str("async ");
        }
        if is_generator {
            function_prefix.push_str("function*");
        } else {
            function_prefix.push_str("function");
        }

        if function_node.is_async {
            self.builder.writeln(&format!(
                "{} {}({}): Promise<any> {{",
                function_prefix, function_node.name, params
            ));
        } else {
            self.builder.writeln(&format!(
                "{} {}({}): any {{",
                function_prefix, function_node.name, params
            ));
        }
        self.builder.indent();

        // Set module function flag for proper return statement handling
        let old_flag = self.in_module_function;
        self.in_module_function = true;

        let (generated_target_specific, ignored_targets) = self.emit_target_specific_body(
            function_node.body,
            &function_node.parsed_target_blocks,
            &function_node.target_specific_regions,
            &function_node.unrecognized_statements,
            None,
            None,
        );

        if generated_target_specific
            && matches!(function_node.body, ActionBody::Mixed)
            && !function_node.statements.is_empty()
        {
            self.builder.writeln(
                "// NOTE: Frame statements ignored because native TypeScript block was provided",
            );
        }

        // Generate function body
        if !generated_target_specific {
            if function_node.statements.is_empty() {
                if !is_generator {
                    self.builder.writeln("return null;");
                }
            } else {
                for stmt in &function_node.statements {
                    self.visit_decl_or_stmt(stmt);
                }
            }
        }

        // Restore previous flag state
        self.in_module_function = old_flag;

        self.current_local_vars = saved_locals;
        self.counter_variables = saved_counters;
        self.walrus_declared_locals = saved_walrus;
        self.pending_walrus_decls = saved_pending_walrus;

        self.in_generator_function = prev_generator_flag;

        self.builder.dedent();

        if !ignored_targets.is_empty() {
            let ignored_list: Vec<String> = ignored_targets.into_iter().collect();
            self.builder.writeln(&format!(
                "// NOTE: target-specific block(s) for {:?} ignored by TypeScript backend",
                ignored_list
            ));
        }

        self.builder.writeln("}");
    }

    fn build_variable_decl_lines(&mut self, var_decl: &VariableDeclNode) -> Vec<String> {
        let mut lines = Vec::new();

        let value_expr = var_decl.get_initializer_value_rc();
        let mut init_str = String::new();
        self.visit_expr_node_to_string(&*value_expr, &mut init_str);

        if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
            eprintln!(
                "DEBUG: Variable {} has expression: '{}'",
                var_decl.name, init_str
            );
        }

        let init_trimmed = init_str.trim_start();
        if init_trimmed.starts_with("new Counter(") {
            self.counter_variables.insert(var_decl.name.clone());
        }

        let unpack_candidate = var_decl
            .name
            .split(':')
            .next()
            .unwrap_or(&var_decl.name)
            .trim();

        if unpack_candidate.starts_with('[')
            && unpack_candidate.ends_with(']')
            && unpack_candidate.contains('*')
        {
            lines.extend(self.render_star_unpack(unpack_candidate, &init_str));
            return lines;
        }

        if var_decl.name.starts_with("__multi_var__:") {
            let names = var_decl
                .name
                .strip_prefix("__multi_var__:")
                .unwrap_or(&var_decl.name);
            let var_names: Vec<String> = names
                .split(',')
                .map(|name| name.trim().to_string())
                .collect();

            if var_names.iter().any(|name| name.starts_with('*')) {
                let pattern = format!("[{}]", var_names.join(", "));
                lines.extend(self.render_star_unpack(&pattern, &init_str));
            } else {
                for name in &var_names {
                    if !name.is_empty() {
                        self.current_local_vars.insert(name.clone());
                    }
                }

                lines.push(format!(
                    "var [{}]: any[] = {};",
                    var_names.join(", "),
                    init_str
                ));
            }
        } else {
            self.current_local_vars.insert(var_decl.name.clone());
            lines.push(format!("var {}: any = {};", var_decl.name, init_str));
        }

        lines
    }

    fn render_star_unpack(&mut self, pattern: &str, initializer: &str) -> Vec<String> {
        let pattern = pattern.trim();
        let (pattern_core, _type_annotation_opt) = if let Some(colon_index) = pattern.find(':') {
            pattern.split_at(colon_index)
        } else {
            (pattern, "")
        };

        let inner = pattern_core
            .trim_start_matches('[')
            .trim_end_matches(']')
            .trim();

        let parts: Vec<String> = inner
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();

        if parts.is_empty() {
            return vec![format!(
                "const _ignored = Array.from(FrameRuntime.iterable({}));",
                initializer
            )];
        }

        self.unpack_temp_counter += 1;
        let temp_name = format!("__unpack{}_tmp", self.unpack_temp_counter);
        let mut lines = Vec::new();
        lines.push(format!(
            "const {}: any[] = Array.from(FrameRuntime.iterable({}));",
            temp_name, initializer
        ));

        if let Some(star_idx) = parts.iter().position(|p| p.starts_with('*')) {
            let before_count = star_idx;
            let after_count = parts.len() - star_idx - 1;

            for (idx, part) in parts.iter().enumerate().take(before_count) {
                let name = part.trim();
                if name.is_empty() || name == "_" {
                    continue;
                }
                self.current_local_vars.insert(name.to_string());
                lines.push(format!(
                    "var {name}: any = {tmp}[{index}];",
                    name = name,
                    tmp = temp_name,
                    index = idx
                ));
            }

            let star_name = parts[star_idx].trim_start_matches('*').trim();
            if !star_name.is_empty() && star_name != "_" {
                self.current_local_vars.insert(star_name.to_string());
                let start_index = before_count;
                let end_expr = if after_count == 0 {
                    format!("{}.length", temp_name)
                } else {
                    format!("{}.length - {}", temp_name, after_count)
                };
                lines.push(format!(
                    "var {name}: any[] = {tmp}.slice({start}, {end});",
                    name = star_name,
                    tmp = temp_name,
                    start = start_index,
                    end = end_expr
                ));
            }

            for (offset, part) in parts.iter().enumerate().skip(star_idx + 1) {
                let name = part.trim();
                if name.is_empty() || name == "_" {
                    continue;
                }
                self.current_local_vars.insert(name.to_string());
                let relative = offset - (star_idx + 1);
                lines.push(format!(
                    "var {name}: any = {tmp}[{tmp}.length - {after} + {rel}];",
                    name = name,
                    tmp = temp_name,
                    after = after_count,
                    rel = relative
                ));
            }

            lines
        } else {
            // No starred target; fall back to simple destructuring semantics
            for part in &parts {
                let name = part.trim();
                if name.is_empty() || name == "_" {
                    continue;
                }
                self.current_local_vars.insert(name.to_string());
            }
            let destructured = parts.join(", ");
            lines.push(format!(
                "var [{pattern}] = Array.from(FrameRuntime.iterable({initializer}));",
                pattern = destructured,
                initializer = initializer
            ));
            lines
        }
    }

    fn visit_variable_decl_node(&mut self, var_decl: &VariableDeclNode) {
        let lines = self.build_variable_decl_lines(var_decl);
        self.flush_pending_walrus_decls();
        for line in lines {
            self.builder.writeln(&line);
        }
    }

    fn visit_class_node(&mut self, class_node: &ClassNode) {
        // Track current class context (like Python visitor)
        self.current_class_name_opt = Some(class_node.name.clone());

        // Generate TypeScript class
        if let Some(parent) = &class_node.parent {
            self.builder.writeln(&format!(
                "export class {} extends {} {{",
                class_node.name, parent
            ));
        } else {
            self.builder
                .writeln(&format!("export class {} {{", class_node.name));
        }
        self.builder.indent();
        let mut reserved_static_inits: Vec<(String, Option<String>)> = Vec::new();
        self.builder.writeln("[key: string]: any;");
        self.builder.newline();

        self.class_method_names
            .entry(class_node.name.clone())
            .or_insert_with(HashSet::new);

        let instance_method_names: HashSet<String> = class_node
            .methods
            .iter()
            .map(|method| method.borrow().name.clone())
            .collect();
        let static_method_names: Vec<String> = class_node
            .static_methods
            .iter()
            .map(|method| method.borrow().name.clone())
            .collect();

        // Use the constructor field for init method
        if let Some(constructor_rc) = &class_node.constructor {
            let constructor = constructor_rc.borrow();
            let params = if let Some(params) = &constructor.params {
                params
                    .iter()
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
            if Self::needs_computed_static(&var_ref.name) {
                if !matches!(*var_ref.value_rc, ExprType::NilExprT) {
                    let mut init_str = String::new();
                    self.visit_expr_node_to_string(&var_ref.value_rc, &mut init_str);
                    reserved_static_inits.push((var_ref.name.clone(), Some(init_str)));
                } else {
                    reserved_static_inits.push((var_ref.name.clone(), None));
                }
                continue;
            }

            if !matches!(*var_ref.value_rc, ExprType::NilExprT) {
                let mut init_str = String::new();
                self.visit_expr_node_to_string(&var_ref.value_rc, &mut init_str);
                self.builder.writeln(&format!(
                    "public static {}: any = {};",
                    var_ref.name, init_str
                ));
            } else {
                self.builder
                    .writeln(&format!("public static {}: any;", var_ref.name));
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
            self.pending_class_method = true;
            self.pending_class_method_param = method_ref
                .params
                .as_ref()
                .and_then(|params| params.first().map(|p| p.param_name.clone()));
            if let Some(entry) = self.class_method_names.get_mut(&class_node.name) {
                entry.insert(method_ref.name.clone());
            }
            self.visit_frame_class_static_method(&method_ref); // Class methods become static in TypeScript
            self.pending_class_method = false;
            self.pending_class_method_param = None;
            self.builder.newline();
        }

        self.builder.dedent();
        self.builder.writeln("}");

        // Clear class context (like Python visitor)
        self.current_class_name_opt = None;

        for (name, init_opt) in reserved_static_inits {
            let init_value = init_opt.unwrap_or_else(|| "undefined".to_string());
            self.builder.writeln(&format!(
                "Object.defineProperty({0}, \"{1}\", {{ configurable: true, writable: true, enumerable: true, value: {2} }});",
                class_node.name, name, init_value
            ));
        }

        for static_name in &static_method_names {
            if instance_method_names.contains(static_name) {
                continue;
            }
            self.builder.writeln(&format!(
                "if (!Object.prototype.hasOwnProperty.call({0}.prototype, \"{1}\")) {{",
                class_node.name, static_name
            ));
            self.builder.indent();
            self.builder.writeln(&format!(
                "Object.defineProperty({0}.prototype, \"{1}\", {{",
                class_node.name, static_name
            ));
            self.builder.indent();
            self.builder.writeln("configurable: true,");
            self.builder.writeln("writable: true,");
            self.builder.writeln("enumerable: false,");
            self.builder.writeln(&format!(
                "value: function (...args: any[]) {{ return {0}.{1}.apply({0}, args); }}",
                class_node.name, static_name
            ));
            self.builder.dedent();
            self.builder.writeln("});");
            self.builder.dedent();
            self.builder.writeln("}");
        }
    }

    fn visit_frame_class_method(&mut self, method: &MethodNode) {
        // Generate Frame class instance method
        let params = if let Some(params) = &method.params {
            params
                .iter()
                .map(|p| format!("{}: any", p.param_name))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };

        // Debug: Check method statements and terminator
        if std::env::var("DEBUG_METHOD_STATEMENTS").is_ok() {
            eprintln!(
                "DEBUG: Method {} has {} statements",
                method.name,
                method.statements.len()
            );
            eprintln!(
                "DEBUG: Method {} terminator type: {:?}",
                method.name,
                std::mem::discriminant(&method.terminator_expr.terminator_type)
            );
        }

        self.builder
            .writeln(&format!("public {}({}): any {{", method.name, params));
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
            params
                .iter()
                .map(|p| format!("{}: any", p.param_name))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };

        self.builder.writeln(&format!(
            "public static {}({}): any {{",
            method.name, params
        ));
        self.builder.indent();

        let saved_locals = self.current_local_vars.clone();
        self.current_local_vars.clear();
        let saved_walrus = self.walrus_declared_locals.clone();
        let saved_pending_walrus = self.pending_walrus_decls.clone();
        self.walrus_declared_locals.clear();
        self.pending_walrus_decls.clear();

        if let Some(param_list) = &method.params {
            for param in param_list {
                self.current_local_vars.insert(param.param_name.clone());
            }
        }

        if self.pending_class_method {
            self.builder
                .writeln("const __frameArgs = Array.prototype.slice.call(arguments);");

            if let Some(param_list) = &method.params {
                if let Some(first_param) = param_list.first() {
                    let remaining_params: Vec<_> = param_list.iter().skip(1).collect();
                    self.builder.writeln(&format!(
                        "const __frameClassCandidate = {};",
                        first_param.param_name
                    ));
                    self.builder.writeln(
                        "if (__frameClassCandidate === undefined || (__frameClassCandidate !== this && typeof __frameClassCandidate !== 'function')) {",
                    );
                    self.builder.indent();
                    self.builder
                        .writeln(&format!("{} = this;", first_param.param_name));
                    if !remaining_params.is_empty() {
                        self.builder
                            .writeln("const __frameDataArgs = __frameArgs.slice();");
                        self.builder
                            .writeln("const __frameFirstArg = __frameDataArgs.shift();");
                        let first_remaining = &remaining_params[0];
                        self.builder.writeln(&format!(
                            "{} = __frameFirstArg;",
                            first_remaining.param_name
                        ));
                        for (idx, param) in remaining_params.iter().enumerate().skip(1) {
                            self.builder.writeln(&format!(
                                "{} = __frameDataArgs[{}];",
                                param.param_name,
                                idx - 1
                            ));
                        }
                    }
                    self.builder.dedent();
                    self.builder.writeln("} else {");
                    self.builder.indent();
                    self.builder.writeln(&format!(
                        "{} = __frameClassCandidate;",
                        first_param.param_name
                    ));
                    if !remaining_params.is_empty() {
                        for (idx, param) in remaining_params.iter().enumerate() {
                            self.builder.writeln(&format!(
                                "{} = __frameArgs[{}];",
                                param.param_name,
                                idx + 1
                            ));
                        }
                    }
                    self.builder.dedent();
                    self.builder.writeln("}");
                }
            }
        }

        // Generate method body
        for stmt in &method.statements {
            self.visit_decl_or_stmt(stmt);
        }

        let has_return_expr = method.terminator_expr.return_expr_t_opt.is_some();

        if method.statements.is_empty() && !has_return_expr {
            self.builder.writeln("return null;");
        } else {
            // Handle explicit return
            match method.terminator_expr.terminator_type {
                TerminatorType::Return => {
                    if let Some(expr) = &method.terminator_expr.return_expr_t_opt {
                        let mut ret_val = String::new();
                        self.visit_expr_node_to_string(expr, &mut ret_val);
                        self.builder.writeln(&format!("return {};", ret_val));
                    } else if !method.statements.is_empty() {
                        self.builder.writeln("return;");
                    }
                }
            }
        }

        self.current_local_vars = saved_locals;
        self.walrus_declared_locals = saved_walrus;
        self.pending_walrus_decls = saved_pending_walrus;
        self.builder.dedent();
        self.builder.writeln("}");
    }

    fn visit_method_node(&mut self, method: &MethodNode) {
        let params = if let Some(params) = &method.params {
            params
                .iter()
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
            self.builder.writeln(&format!(
                "{} {}({}): any {{",
                visibility, method.name, params
            ));
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
                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                    eprintln!("DEBUG: Processing VarDeclT for variable: {}", var_decl.name);
                }
                self.visit_variable_decl_node(&var_decl);
            }
            DeclOrStmtType::StmtT { stmt_t } => {
                if std::env::var("FRAME_TRANSPILER_DEBUG").unwrap_or_default() == "1" {
                    eprintln!("DEBUG: Processing StmtT");
                }
                self.visit_stmt_node(&stmt_t);
            }
        }
    }

    fn record_walrus_local(&mut self, var_name: &str) {
        if self.current_local_vars.contains(var_name)
            || self.domain_variables.contains(var_name)
            || self.current_state_vars.contains(var_name)
            || self.current_state_params.contains(var_name)
            || self.current_handler_params.contains(var_name)
            || self.current_exception_vars.contains(var_name)
        {
            return;
        }

        let owned = var_name.to_string();
        self.current_local_vars.insert(owned.clone());
        if self.walrus_declared_locals.insert(owned.clone()) {
            self.pending_walrus_decls.push(owned);
        }
    }

    fn flush_pending_walrus_decls(&mut self) {
        if self.pending_walrus_decls.is_empty() {
            return;
        }

        for var_name in self.pending_walrus_decls.drain(..) {
            self.builder.writeln(&format!("var {}: any;", var_name));
        }
    }

    fn render_defaultdict_factory(&mut self, expr: &ExprType) -> String {
        let mut expr_str = String::new();
        self.visit_expr_node_to_string(expr, &mut expr_str);

        match expr_str.as_str() {
            "int" | "float" => "() => 0".to_string(),
            "list" => "() => []".to_string(),
            "dict" => "() => ({})".to_string(),
            "set" => "() => new Set()".to_string(),
            _ => {
                if expr_str.is_empty() {
                    "(() => undefined)".to_string()
                } else {
                    format!(
                        "(() => (typeof {0} === 'function' ? {0}() : {0}))",
                        expr_str
                    )
                }
            }
        }
    }

    fn function_contains_yield(function_node: &FunctionNode) -> bool {
        function_node
            .statements
            .iter()
            .any(Self::decl_or_stmt_contains_yield)
    }

    fn decl_or_stmt_contains_yield(item: &DeclOrStmtType) -> bool {
        match item {
            DeclOrStmtType::StmtT { stmt_t } => Self::statement_contains_yield(stmt_t),
            DeclOrStmtType::VarDeclT { .. } => false,
        }
    }

    fn block_contains_yield(block: &BlockStmtNode) -> bool {
        block
            .statements
            .iter()
            .any(Self::decl_or_stmt_contains_yield)
    }

    fn statement_contains_yield(stmt: &StatementType) -> bool {
        match stmt {
            StatementType::ExpressionStmt { expr_stmt_t } => {
                Self::expr_stmt_contains_yield(expr_stmt_t)
            }
            StatementType::IfStmt { if_stmt_node } => {
                if Self::block_contains_yield(&if_stmt_node.if_block) {
                    true
                } else if if_stmt_node
                    .elif_clauses
                    .iter()
                    .any(|clause| Self::block_contains_yield(&clause.block))
                {
                    true
                } else if let Some(else_block) = &if_stmt_node.else_block {
                    Self::block_contains_yield(else_block)
                } else {
                    false
                }
            }
            StatementType::LoopStmt { loop_stmt_node } => match &loop_stmt_node.loop_types {
                LoopStmtTypes::LoopInfiniteStmt {
                    loop_infinite_stmt_node,
                } => loop_infinite_stmt_node
                    .statements
                    .iter()
                    .any(Self::decl_or_stmt_contains_yield),
                LoopStmtTypes::LoopInStmt { loop_in_stmt_node } => loop_in_stmt_node
                    .statements
                    .iter()
                    .any(Self::decl_or_stmt_contains_yield),
                LoopStmtTypes::LoopForStmt { loop_for_stmt_node } => loop_for_stmt_node
                    .statements
                    .iter()
                    .any(Self::decl_or_stmt_contains_yield),
            },
            StatementType::WhileStmt { while_stmt_node } => {
                if Self::block_contains_yield(&while_stmt_node.block) {
                    true
                } else if let Some(else_block) = &while_stmt_node.else_block {
                    Self::block_contains_yield(else_block)
                } else {
                    false
                }
            }
            StatementType::ForStmt { for_stmt_node } => {
                if Self::block_contains_yield(&for_stmt_node.block) {
                    true
                } else if let Some(else_block) = &for_stmt_node.else_block {
                    Self::block_contains_yield(else_block)
                } else {
                    false
                }
            }
            StatementType::BlockStmt { block_stmt_node } => {
                Self::block_contains_yield(block_stmt_node)
            }
            _ => false,
        }
    }

    fn emit_target_specific_body(
        &mut self,
        body_kind: ActionBody,
        parsed_blocks: &[ParsedTargetBlock],
        region_refs: &[TargetSpecificRegionRef],
        unrecognized: &[UnrecognizedStatementNode],
        segments: Option<&[BodySegment]>,
        mixed: Option<&[MixedBodyItem]>,
    ) -> (bool, BTreeSet<String>) {
        let mut generated = false;
        let mut ignored: BTreeSet<String> = unrecognized
            .iter()
            .map(|entry| format!("{:?}", entry.target))
            .collect();

        // Prefer MixedBody if provided (B2 path); fallback to segmented native text
        if let Some(items) = mixed {
            let mut after_terminal_dir = false; // transition/forward/stack ops imply early return
            let mut warned_unreachable = false; // emit warning once per body
            for it in items {
                match it {
                    MixedBodyItem::NativeText { text, start_line, .. } => {
                        // Map the first output position of this native span to its frame line
                        self.builder.map_next(*start_line);
                        if !text.trim().is_empty() {
                            if after_terminal_dir && !warned_unreachable {
                                // Emit a non-fatal warning comment to flag unreachable native code
                                self.builder.writeln("// WARNING: Unreachable code after transition/forward/stack op");
                                warned_unreachable = true;
                            }
                            let rewritten = self.rewrite_typescript_target_source(text);
                            self.builder.write(&rewritten);
                            if !rewritten.ends_with('\n') {
                                self.builder.newline();
                            }
                        }
                    }
                    MixedBodyItem::NativeAst {
                        start_line, ast, ..
                    } => {
                        self.builder.map_next(*start_line);
                        let code_src = ast.to_source();
                        let code = self.rewrite_typescript_target_source(&code_src);
                        if !code.trim().is_empty() {
                            if after_terminal_dir && !warned_unreachable {
                                self.builder.writeln("// WARNING: Unreachable code after transition/forward/stack op");
                                warned_unreachable = true;
                            }
                            self.builder.write(&code);
                            if !code.ends_with('\n') {
                                self.builder.newline();
                            }
                        }
                    }
                    MixedBodyItem::Frame { frame_line, indent: _indent, stmt } => {
                        // Map directive glue to the directive's frame line
                        self.builder.map_next(*frame_line);
                        let code = self.emit_mir_statement_as_swc(stmt);
                        if !code.is_empty() {
                            self.builder.write(&code);
                            if !code.ends_with('\n') {
                                self.builder.newline();
                            }
                        }
                        // Mark terminal directives to warn on following code
                        match stmt {
                            MirStatement::Transition { .. }
                            | MirStatement::ParentForward
                            | MirStatement::StackPush
                            | MirStatement::StackPop => after_terminal_dir = true,
                            _ => {}
                        }
                    }
                }
            }
            return (true, ignored);
        } else if let Some(segs) = segments {
            for seg in segs {
                match seg {
                    BodySegment::Native { text, .. } => {
                        if !text.trim().is_empty() {
                            self.builder.write(text);
                            if !text.ends_with('\n') {
                                self.builder.newline();
                            }
                        }
                    }
                    BodySegment::FrameStmt {
                        kind, line_text, ..
                    } => {
                        match kind {
                            crate::frame_c::native_region_segmenter::FrameStmtKind::Transition => {
                                // Very simple parse: look for "$Name" after ->
                                if let Some(dollar) = line_text.find('$') {
                                    let tail = &line_text[dollar + 1..];
                                    let mut name = String::new();
                                    for ch in tail.chars() {
                                        if ch.is_alphanumeric() || ch == '_' {
                                            name.push(ch);
                                        } else {
                                            break;
                                        }
                                    }
                                    if !name.is_empty() {
                                        // Emit basic transition glue without args
                                        self.builder.writeln(&format!(
                                            "this._frame_transition(new FrameCompartment('{}', null, null, {{}}, {{}}));",
                                            name
                                        ));
                                        self.builder.writeln("return;");
                                    } else {
                                        self.builder.writeln("// TODO: transition parse failed");
                                    }
                                }
                            }
                            crate::frame_c::native_region_segmenter::FrameStmtKind::Forward => {
                                // Parent forward: set nextCompartment to parent state and forward current event
                                // Determine current state and its parent from the AST if possible.
                                if let Some(curr_state_name) = &self.current_state_name {
                                    // Resolve parent via precomputed map
                                    let curr_formatted = self.format_state_name(curr_state_name);
                                    let parent_name_opt = self
                                        .parent_state_map
                                        .get(&curr_formatted)
                                        .and_then(|p| p.clone());
                                    if let Some(parent_name) = parent_name_opt {
                                        self.builder.writeln(&format!(
                                            "this._nextCompartment = new FrameCompartment('{}', null, null, {{}}, {{}});",
                                            parent_name
                                        ));
                                        self.builder
                                            .writeln("this._nextCompartment.forwardEvent = __e;");
                                        self.builder.writeln("return;");
                                    } else {
                                        // If parent not found, fall back to forwarding to current state
                                        self.builder.writeln("// WARN: parent state not found; forwarding to current state");
                                        let curr = curr_formatted;
                                        self.builder.writeln(&format!(
                                            "this._nextCompartment = new FrameCompartment('{}', null, null, {{}}, {{}});",
                                            curr
                                        ));
                                        self.builder
                                            .writeln("this._nextCompartment.forwardEvent = __e;");
                                        self.builder.writeln("return;");
                                    }
                                } else {
                                    self.builder.writeln("// TODO: unable to resolve current state for parent forward");
                                }
                            }
                            crate::frame_c::native_region_segmenter::FrameStmtKind::StackPush => {
                                // Push current state onto return stack (or dedicated state stack) and return
                                // For TS backend we use returnStack as general stack storage.
                                self.builder.writeln("this.returnStack.push({});");
                                self.builder.writeln("return;");
                            }
                            crate::frame_c::native_region_segmenter::FrameStmtKind::StackPop => {
                                // Pop a previously pushed value/state and return it as handler result
                                self.builder
                                    .writeln("const __popped = this.returnStack.pop();");
                                self.builder.writeln(
                                    "this.returnStack[this.returnStack.length - 1] = __popped;",
                                );
                                self.builder.writeln("return;");
                            }
                            crate::frame_c::native_region_segmenter::FrameStmtKind::Return => {
                                // Minimal fallback: treat as bare return in TS mixed bodies
                                self.builder.writeln("return;");
                            }
                        }
                    }
                }
            }
            return (true, ignored);
        }

        if !matches!(body_kind, ActionBody::TargetSpecific | ActionBody::Mixed) {
            return (generated, ignored);
        }

        for block in parsed_blocks {
            if self.emit_typescript_target_block(block) {
                generated = true;
            } else {
                ignored.insert(format!("{:?}", block.ast.target_language()));
            }
        }

        if !generated {
            for region_ref in region_refs {
                if region_ref.target == TargetLanguage::TypeScript {
                    if let Some(region) = self.target_regions.get(region_ref.region_index).cloned()
                    {
                        self.emit_target_region_lines(&region);
                        generated = true;
                    }
                } else {
                    ignored.insert(format!("{:?}", region_ref.target));
                }
            }
        } else {
            for region_ref in region_refs {
                if region_ref.target != TargetLanguage::TypeScript {
                    ignored.insert(format!("{:?}", region_ref.target));
                }
            }
        }

        (generated, ignored)
    }

    fn emit_typescript_target_block(&mut self, block: &ParsedTargetBlock) -> bool {
        if block.ast.target_language() != TargetLanguage::TypeScript {
            return false;
        }

        if let Some(ast) = block.ast.as_any().downcast_ref::<TypeScriptTargetAst>() {
            let mut emitted_any = false;
            for element in ast.elements() {
                if self.emit_typescript_target_element(block, element) {
                    emitted_any = true;
                }
            }

            if emitted_any {
                return true;
            }
        }

        let source = block.ast.to_code();
        if source.trim().is_empty() {
            return false;
        }

        self.emit_target_source_with_metadata(
            &source,
            block.frame_start_line,
            block.frame_end_line,
            TargetLanguage::TypeScript,
        );
        true
    }

    fn emit_typescript_target_element(
        &mut self,
        block: &ParsedTargetBlock,
        element: &TypeScriptTargetElement,
    ) -> bool {
        let segment = match element {
            TypeScriptTargetElement::Statement(stmt) => stmt,
            TypeScriptTargetElement::RawSegment(stmt) => stmt,
        };

        if segment.code.trim().is_empty() {
            let line_count = segment
                .end_line
                .saturating_sub(segment.start_line)
                .saturating_add(1);
            for _ in 0..line_count {
                self.builder.newline();
            }
            return line_count > 0;
        }

        let rewritten_code = self.rewrite_typescript_target_source(&segment.code);

        let frame_start = block
            .frame_start_line
            .saturating_add(segment.start_line.saturating_sub(1));
        let frame_end = block
            .frame_start_line
            .saturating_add(segment.end_line.saturating_sub(1));

        self.emit_target_source_with_metadata(
            &rewritten_code,
            frame_start,
            frame_end,
            TargetLanguage::TypeScript,
        );
        true
    }

    fn rewrite_typescript_target_source(&self, source: &str) -> String {
        if source.is_empty() {
            return String::new();
        }

        let mut rewritten = String::with_capacity(source.len());
        for line in source.split_inclusive('\n') {
            let (content, suffix_newline) = if line.ends_with('\n') {
                (&line[..line.len() - 1], true)
            } else {
                (line, false)
            };

            let indent_len = content
                .chars()
                .take_while(|c| *c == ' ' || *c == '\t')
                .count();
            let (indent, body) = content.split_at(indent_len);

            // Rewrite pseudo-symbols and runtime shims
            let transformed = body
                // system.return (read/write) → this.returnStack[top]
                .replace(
                    "system.return",
                    "this.returnStack[this.returnStack.length - 1]",
                )
                // Historical runtime path alias cleanup
                .replace("runtime/socket.", "")
                .replace("runtime_socket.", "");

            rewritten.push_str(indent);
            rewritten.push_str(&transformed);

            if suffix_newline {
                rewritten.push('\n');
            }
        }

        rewritten
    }

    fn emit_target_source_with_metadata(
        &mut self,
        source: &str,
        frame_start_line: usize,
        frame_end_line: usize,
        target_language: TargetLanguage,
    ) {
        let lines: Vec<&str> = if source.is_empty() {
            Vec::new()
        } else {
            source.lines().collect()
        };

        if lines.is_empty() {
            self.builder.writeln(&format!(
                "// [target {:?} block | frame line {}]",
                target_language, frame_start_line
            ));
            return;
        }

        let comment = if frame_start_line == frame_end_line {
            format!(
                "// [target {:?} line 1 -> frame line {}]",
                target_language, frame_start_line
            )
        } else {
            let line_count = lines.len();
            format!(
                "// [target {:?} lines 1-{} -> frame lines {}-{}]",
                target_language, line_count, frame_start_line, frame_end_line
            )
        };
        self.builder.writeln(&comment);

        for line in lines {
            if line.trim().is_empty() {
                self.builder.newline();
            } else {
                self.builder.writeln(line);
            }
        }
    }

    fn emit_mir_statement_as_swc(&self, mir: &MirStatement) -> String {
        match mir {
            MirStatement::Transition { state, args } => {
                // Build stateArgs object if args are present; use recorded param names when available
                let mut state_args_code = String::from("{}");
                if !args.is_empty() {
                    let param_names = self.state_param_names_for(state);
                    let mut entries: Vec<String> = Vec::new();
                    for (i, a) in args.iter().enumerate() {
                        let key = param_names.get(i).cloned().unwrap_or_else(|| format!("arg_{}", i));
                        entries.push(format!("'{}': {}", key, a));
                    }
                    state_args_code = format!("{{ {} }}", entries.join(", "));
                }
                format!(
                    "this._frame_transition(new FrameCompartment(\"{}\", null, null, {}, {{}}));\nreturn;\n",
                    state, state_args_code
                )
            }
            MirStatement::ParentForward => {
                let mut target = String::new();
                if let Some(curr) = &self.current_state_name {
                    let curr_formatted = self.format_state_name(curr);
                    if let Some(parent) = self.parent_state_map.get(&curr_formatted).and_then(|p| p.clone()) {
                        target = parent;
                    } else {
                        target = curr_formatted;
                    }
                }
                if target.is_empty() { target = "".to_string(); }
                format!(
                    "this._nextCompartment = new FrameCompartment(\"{}\", null, null, {{}}, {{}});\nthis._nextCompartment.forwardEvent = __e;\nreturn;\n",
                    target
                )
            }
            MirStatement::StackPush => {
                "this.returnStack.push({});\nreturn;\n".to_string()
            }
            MirStatement::StackPop => {
                "const __popped = this.returnStack.pop();\nthis.returnStack[this.returnStack.length - 1] = __popped;\nreturn;\n".to_string()
            }
            MirStatement::Return(_expr) => {
                "return;\n".to_string()
            }
        }
    }

    fn emit_target_region_lines(&mut self, region: &TargetRegion) {
        if region.raw_content.trim().is_empty() {
            return;
        }

        let lines: Vec<&str> = region.raw_content.lines().collect();
        let mut min_indent = usize::MAX;

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let indent = line.chars().take_while(|c| *c == ' ' || *c == '\t').count();
            if indent < min_indent {
                min_indent = indent;
            }
        }

        if min_indent == usize::MAX {
            min_indent = 0;
        }

        let mut dedented_lines: Vec<String> = Vec::new();
        for line in &lines {
            if line.trim().is_empty() {
                dedented_lines.push(String::new());
            } else if line.len() > min_indent {
                dedented_lines.push(line[min_indent..].to_string());
            } else {
                dedented_lines.push(line.to_string());
            }
        }

        let mut dedented_source = dedented_lines.join("\n");
        if region.raw_content.ends_with('\n') && !dedented_source.ends_with('\n') {
            dedented_source.push('\n');
        }

        let frame_end_line = if dedented_lines.is_empty() {
            region.source_map.frame_start_line
        } else {
            region
                .source_map
                .frame_start_line
                .saturating_add(dedented_lines.len().saturating_sub(1))
        };

        self.emit_target_source_with_metadata(
            &dedented_source,
            region.source_map.frame_start_line,
            frame_end_line,
            region.target,
        );
    }

    fn expr_stmt_contains_yield(expr_stmt: &ExprStmtType) -> bool {
        match expr_stmt {
            ExprStmtType::ExprListStmtT {
                expr_list_stmt_node,
            } => expr_list_stmt_node
                .expr_list_node
                .exprs_t
                .iter()
                .any(|expr| {
                    matches!(
                        expr,
                        ExprType::YieldExprT { .. } | ExprType::YieldFromExprT { .. }
                    )
                }),
            _ => false,
        }
    }

    fn is_counter_reference(&self, expr: &str) -> bool {
        let trimmed = expr.trim();
        self.counter_variables.contains(trimmed)
    }
}

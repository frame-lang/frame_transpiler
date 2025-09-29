// Python Visitor v2 - Complete implementation using CodeBuilder for robust source mapping
//
// This visitor uses the CodeBuilder architecture for automatic line tracking and perfect
// source mappings without manual offsets.

use crate::frame_c::ast::*;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::config::FrameConfig;
use crate::frame_c::scanner::{Token, TokenType};
use crate::frame_c::source_map::SourceMapBuilder;
use crate::frame_c::symbol_table::{SymbolConfig, Arcanum, SymbolType, SymbolTable};
use crate::frame_c::visitors::AstVisitor;

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PythonVisitorV2 {
    // Core configuration
    config: FrameConfig,
    
    // Code generation
    builder: CodeBuilder,
    
    // External source map builder (for --debug-output integration)
    external_source_map_builder: Option<Rc<RefCell<SourceMapBuilder>>>,
    
    // Symbol tracking
    symbol_config: SymbolConfig,
    arcanum: Vec<Arcanum>,
    
    // Current context
    current_state_name_opt: Option<String>,
    current_state_parent_opt: Option<String>, // HSM parent state tracking
    current_event_ret_type: String,
    current_class_name_opt: Option<String>,
    module_context: Vec<String>, // Track nested module context
    current_module_variables: HashSet<String>, // Track variables in current module
    current_module_name_opt: Option<String>, // Track the current module name for qualification
    
    // System metadata
    system_name: String,
    system_has_async_runtime: bool,
    interface_methods: HashMap<String, Vec<String>>, // method_name -> parameter_names
    domain_variables: HashSet<String>, // Track domain variable names
    current_handler_params: HashSet<String>, // Track current event handler parameter names
    current_handler_locals: HashSet<String>, // Track local variables in current handler
    current_state_params: HashSet<String>, // Track current state's parameter names
    domain_enums: HashSet<String>, // Track domain enum names (without prefix)
    action_names: HashSet<String>, // Track action names for proper call resolution
    operation_names: HashSet<String>, // Track operation names for proper call resolution
    
    // Import tracking
    imports: Vec<String>,
    used_modules: HashSet<String>,
    
    // Global variable tracking
    global_vars: HashSet<String>,
    
    // Comments (for future use)
    _comments: Vec<Token>,
}

impl PythonVisitorV2 {
    pub fn new(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
        config: FrameConfig,
        comments: Vec<Token>,
    ) -> PythonVisitorV2 {
        PythonVisitorV2 {
            config,
            builder: CodeBuilder::new("    "), // 4-space indent for Python
            external_source_map_builder: None,
            symbol_config,
            arcanum,
            current_state_name_opt: None,
            current_state_parent_opt: None,
            current_event_ret_type: String::new(),
            current_class_name_opt: None,
            module_context: Vec::new(),
            current_module_variables: HashSet::new(),
            current_module_name_opt: None,
            system_name: String::new(),
            system_has_async_runtime: false,
            interface_methods: HashMap::new(),
            domain_variables: HashSet::new(),
            current_handler_params: HashSet::new(),
            current_handler_locals: HashSet::new(),
            current_state_params: HashSet::new(),
            domain_enums: HashSet::new(),
            action_names: HashSet::new(),
            operation_names: HashSet::new(),
            imports: Vec::new(),
            used_modules: HashSet::new(),
            global_vars: HashSet::new(),
            _comments: comments,
        }
    }
    
    /// Set an external source map builder for --debug-output integration
    pub fn set_source_map_builder(&mut self, builder: Rc<RefCell<SourceMapBuilder>>) {
        self.external_source_map_builder = Some(builder);
    }
    
    // Helper method to generate enum with a specific name  
    fn generate_enum_with_name(&mut self, enum_node: &EnumDeclNode, enum_name: &str) {
        self.builder.newline();
        
        // Check if we need Enum import
        if !self.imports.contains(&"from enum import Enum".to_string()) {
            self.imports.push("from enum import Enum".to_string());
        }
        
        // Determine base class
        let base_class = match enum_node.enum_type {
            EnumType::String => "(str, Enum)",
            _ => "(Enum)",
        };
        
        // v0.78.8: Map enum declarations to source
        self.builder.writeln_mapped(&format!("class {}{}:", enum_name, base_class), enum_node.line);
        self.builder.indent();
        
        // Generate enum members
        if enum_node.enums.is_empty() {
            self.builder.writeln("pass");
        } else {
            // Track seen names to handle duplicates
            let mut seen_names = std::collections::HashSet::new();
            
            for enumerator in &enum_node.enums {
                // Skip duplicates - Python Enum doesn't allow duplicate names
                if seen_names.contains(&enumerator.name) {
                    // Optionally, we could generate a comment about the duplicate
                    self.builder.writeln(&format!("# Duplicate enum member '{}' skipped", enumerator.name));
                    continue;
                }
                seen_names.insert(enumerator.name.clone());
                
                let value = match &enumerator.value {
                    EnumValue::Integer(i) => i.to_string(),
                    EnumValue::String(s) => format!("\"{}\"", s),
                    EnumValue::Auto => {
                        if matches!(enum_node.enum_type, EnumType::String) {
                            // Auto-generate string value from name
                            format!("\"{}\"", enumerator.name)
                        } else {
                            // Auto-generate numeric value
                            "auto()".to_string()
                        }
                    }
                };
                
                // v0.78.9: Map enum member to source line
                self.builder.writeln_mapped(&format!("{} = {}", enumerator.name, value), enumerator.line);
            }
        }
        
        self.builder.dedent();
    }
    
    pub fn run(&mut self, frame_module: &FrameModule) -> String {
        // Add header
        self.builder.write_comment(&format!("Emitted from framec_v{}", env!("FRAME_VERSION")));
        self.builder.newline();
        self.builder.newline();
        
        // Generate Frame runtime classes
        self.generate_frame_runtime();
        
        // Visit the module
        self.visit_frame_module(frame_module);
        
        // Build the final output - need to move builder out
        let builder = std::mem::replace(&mut self.builder, CodeBuilder::new("    "));
        let (mut code, mappings) = builder.build();
        
        // If we have an external source map builder (--debug-output mode), sync our mappings
        if let Some(ref external_builder) = self.external_source_map_builder {
            use crate::frame_c::source_map::MappingType;
            for mapping in &mappings {
                // Set the Python line and add the mapping
                external_builder.borrow_mut().set_python_line(mapping.python_line);
                external_builder.borrow_mut().add_mapping(
                    mapping.frame_line,
                    MappingType::FunctionDef,
                    None
                );
            }
        }
        
        // Generate source map if debug output is enabled (internal mode)
        let debug_output = std::env::var("FRAME_TRANSPILER_DEBUG").is_ok();
        if debug_output && self.external_source_map_builder.is_none() {
            let source_map = self.generate_source_map("unknown.frm", mappings);
            code = self.wrap_with_source_map(code, source_map);
        }
        
        // Add imports at the top if needed
        if !self.imports.is_empty() {
            let import_code = self.imports.join("\n");
            if debug_output {
                code = self.insert_imports_in_json(code, import_code);
            } else {
                code = format!("{}\n\n{}", import_code, code);
            }
        }
        
        code
    }
    
    fn generate_source_map(&self, source_path: &str, mappings: Vec<crate::frame_c::code_builder::SourceMapping>) -> String {
        let source_file = source_path.split('/').last().unwrap_or("unknown.frm");
        let mut builder = SourceMapBuilder::new(
            source_file.to_string(),
            format!("{}.py", source_file.trim_end_matches(".frm"))
        );
        
        // Add each mapping with the correct Python line
        use crate::frame_c::source_map::MappingType;
        for mapping in mappings {
            // Set the Python line from CodeBuilder (already 1-based)
            builder.set_python_line(mapping.python_line);
            // Add the mapping
            builder.add_mapping(
                mapping.frame_line,
                MappingType::FunctionDef,
                None
            );
        }
        
        let source_map = builder.build();
        serde_json::to_string(&source_map).unwrap_or_else(|_| "null".to_string())
    }
    
    fn wrap_with_source_map(&self, python_code: String, source_map: String) -> String {
        format!(
            r#"{{
  "python": {},
  "sourceMap": {}
}}"#,
            serde_json::to_string(&python_code).unwrap_or_else(|_| "null".to_string()),
            source_map
        )
    }
    
    fn insert_imports_in_json(&self, json_code: String, import_code: String) -> String {
        if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&json_code) {
            if let Some(python) = value.get_mut("python") {
                if let Some(python_str) = python.as_str() {
                    let new_python = format!("{}\n\n{}", import_code, python_str);
                    *python = serde_json::Value::String(new_python);
                    return serde_json::to_string_pretty(&value).unwrap_or(json_code);
                }
            }
        }
        json_code
    }
    
    fn generate_frame_runtime(&mut self) {
        self.builder.writeln("class FrameEvent:");
        self.builder.indent();
        self.builder.writeln("def __init__(self, message, parameters):");
        self.builder.indent();
        self.builder.writeln("self._message = message");
        self.builder.writeln("self._parameters = parameters");
        self.builder.dedent();
        self.builder.dedent();
        
        self.builder.newline();
        self.builder.writeln("class FrameCompartment:");
        self.builder.indent();
        self.builder.writeln("def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None, state_vars=None, state_args=None):");
        self.builder.indent();
        self.builder.writeln("self.state = state");
        self.builder.writeln("self.forward_event = forward_event");
        self.builder.writeln("self.exit_args = exit_args");
        self.builder.writeln("self.enter_args = enter_args");
        self.builder.writeln("self.parent_compartment = parent_compartment");
        self.builder.writeln("self.state_vars = state_vars or {}");
        self.builder.writeln("self.state_args = state_args or {}");
        self.builder.dedent();
        self.builder.dedent();
        self.builder.newline();
    }
    
    fn format_state_name(&self, state_name: &str) -> String {
        format!("__{}_state_{}", self.system_name.to_lowercase(), state_name)
    }
}

// Main visitor trait implementation
impl AstVisitor for PythonVisitorV2 {
    fn visit_frame_module(&mut self, frame_module: &FrameModule) {
        // Visit all module elements - using correct FrameModule structure
        
        // Process imports first (they should be at the top)
        for import in &frame_module.imports {
            self.visit_import_node(import);
        }
        
        // Process enums
        for enum_decl in &frame_module.enums {
            self.visit_enum_decl_node(&enum_decl.borrow());
        }
        
        // First pass: Collect all global variable names BEFORE processing functions
        // This is needed so functions can generate proper global declarations
        for var_decl in &frame_module.variables {
            let var = var_decl.borrow();
            self.global_vars.insert(var.name.clone());
        }
        
        // Process modules (nested modules)
        for module_node in &frame_module.modules {
            self.visit_module_node(&module_node.borrow());
        }
        
        // Process classes
        for class_node in &frame_module.classes {
            self.visit_class_node(&class_node.borrow());
        }
        
        // Process functions (now with global_vars populated)
        for function in &frame_module.functions {
            function.borrow().accept(self);
        }
        
        // Process systems
        for system in &frame_module.systems {
            self.visit_system_node(system);
        }
        
        // Process module-level variables (actual generation)
        for var_decl in &frame_module.variables {
            let var = var_decl.borrow();
            var.accept(self);
            self.builder.newline(); // V1 adds a newline after each variable
        }
        
        // Process module-level statements (e.g., print statements, function calls at top level)
        // This must come LAST so all functions, systems, classes are defined before execution
        if !frame_module.statements.is_empty() {
            self.builder.newline();
            self.builder.newline();
            for stmt in &frame_module.statements {
                self.visit_decl_or_stmt(stmt);
            }
        }
        
        // Skip the old element processing loop
        /*for element in &frame_module.module_elements {
            match element {
                ModuleElement::Function { function_node } => {
                    function_node.borrow().accept(self);
                }
                ModuleElement::System { system_node } => {
                    system_node.accept_system(self);
                }
                ModuleElement::Import { import_node } => {
                    import_node.accept(self);
                }
                ModuleElement::Enum { enum_decl_node } => {
                    enum_decl_node.borrow().accept_enum_decl(self);
                }
                ModuleElement::Module { module_node } => {
                    module_node.borrow().accept_module(self);
                }
                ModuleElement::TypeAlias { type_alias_node } => {
                    type_alias_node.borrow().accept_type_alias(self);
                }
                ModuleElement::Class { class_node } => {
                    class_node.borrow().accept_class(self);
                }
                ModuleElement::Variable { var_decl_node } => {
                    let var_decl = var_decl_node.borrow();
                    self.builder.writeln_mapped(
                        &format!("{} = None", var_decl.name),
                        var_decl.line
                    );
                    self.global_vars.insert(var_decl.name.clone());
                }
                _ => {
                    // Skip other elements for now
                }
            }
        }*/
        
        // Add main block if present - check for main function
        let has_main = frame_module.functions.iter()
            .any(|f| f.borrow().name == "main");
        if has_main {
            self.builder.newline();
            self.builder.writeln("if __name__ == '__main__':");
            self.builder.indent();
            self.builder.writeln("main()");
            self.builder.dedent();
        }
    }
    
    fn visit_variable_decl_node(&mut self, var_decl: &VariableDeclNode) {
        // V1 uses get_initializer_value_rc() not value_rc - this is the key difference!
        let value_expr = var_decl.get_initializer_value_rc();
        
        // Generate the initializer value
        let mut init_value = String::new();
        self.visit_expr_node_to_string(&*value_expr, &mut init_value);
        
        // Don't default to None if we get TODO - that's a sign something is wrong
        if init_value.contains("TODO") {
            eprintln!("WARNING: Got TODO for variable '{}' initialization", var_decl.name);
            init_value = "None".to_string();
        }
        
        // Handle multi-variable declaration
        let var_name = if var_decl.name.starts_with("__multi_var__:") {
            // Extract the variable names after the prefix
            let names = var_decl.name.strip_prefix("__multi_var__:").unwrap_or(&var_decl.name);
            // Only track as local if NOT module scope
            if var_decl.identifier_decl_scope != IdentifierDeclScope::ModuleScope {
                for name in names.split(',') {
                    self.current_handler_locals.insert(name.trim().to_string());
                }
            }
            names.replace(",", ", ")
        } else {
            // Only track as local if NOT module scope
            // Module variables are tracked in global_vars separately
            if var_decl.identifier_decl_scope != IdentifierDeclScope::ModuleScope {
                self.current_handler_locals.insert(var_decl.name.clone());
            }
            var_decl.name.clone()
        };
        
        let assignment = format!("{} = {}", var_name, init_value);
        self.builder.writeln_mapped(&assignment, var_decl.line);
    }
    
    fn visit_function_node(&mut self, function_node: &FunctionNode) {
        let params = if let Some(params) = &function_node.params {
            params.iter()
                .map(|p| p.param_name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        self.builder.newline();
        
        // Map the function definition to its Frame source line
        self.builder.map_next(function_node.line);
        self.builder.write_function(
            &function_node.name,
            &params,
            function_node.is_async,
            function_node.line
        );
        
        // Add global declarations if needed
        let mut needs_globals = Vec::new();
        for stmt in &function_node.statements {
            self.collect_global_vars_in_stmt(stmt, &mut needs_globals);
        }
        
        if !needs_globals.is_empty() {
            let globals = needs_globals.join(", ");
            self.builder.writeln(&format!("global {}", globals));
        }
        
        // Generate function body
        if function_node.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for stmt in &function_node.statements {
                self.visit_decl_or_stmt(stmt);
            }
        }
        
        self.builder.end_function();
    }
} // impl AstVisitor for PythonVisitorV2

impl PythonVisitorV2 {
    fn visit_system_node(&mut self, system_node: &SystemNode) {
        self.system_name = system_node.name.clone();
        self.system_has_async_runtime = self.check_system_async(system_node);
        
        // Clear previous system's data
        self.interface_methods.clear();
        self.domain_variables.clear();
        self.domain_enums.clear();
        self.action_names.clear();
        
        // Store interface method parameters for later use in event handlers
        if let Some(interface) = &system_node.interface_block_node_opt {
            for method in &interface.interface_methods {
                let method_borrow = method.borrow();
                let param_names = if let Some(params) = &method_borrow.params {
                    params.iter()
                        .map(|p| p.param_name.clone())
                        .collect()
                } else {
                    Vec::new()
                };
                self.interface_methods.insert(method_borrow.name.clone(), param_names);
            }
        }
        
        // Store domain variable names for later use
        if let Some(domain_block) = &system_node.domain_block_node_opt {
            for var_rcref in &domain_block.member_variables {
                let var = var_rcref.borrow();
                self.domain_variables.insert(var.name.clone());
            }
            
            // Generate domain enums BEFORE the class
            // They need to be defined at module level, not inside the class
            // Domain enums should be prefixed with system name
            for enum_rcref in &domain_block.enums {
                let enum_node = enum_rcref.borrow();
                // Track this as a domain enum
                self.domain_enums.insert(enum_node.name.clone());
                // Generate the enum with a prefixed name
                let prefixed_name = format!("{}_{}", self.system_name, enum_node.name);
                self.generate_enum_with_name(&enum_node, &prefixed_name);
                
                // Create module-level alias for the enum so it can be accessed without prefix
                self.builder.newline();
                self.builder.writeln(&format!("{} = {}", enum_node.name, prefixed_name));
            }
        }
        
        // Generate class
        self.builder.write_class(&system_node.name, None, Some(system_node.line));
        self.builder.newline();
        
        // Generate __init__
        self.generate_system_init(system_node);
        
        // Generate interface methods
        if let Some(interface) = &system_node.interface_block_node_opt {
            self.visit_interface_block_node(interface);
        }
        
        // Collect action and operation names BEFORE processing machine block (for proper call resolution)
        if let Some(actions) = &system_node.actions_block_node_opt {
            for action_rcref in &actions.actions {
                let action_node = action_rcref.borrow();
                self.action_names.insert(action_node.name.clone());
            }
        }
        if let Some(operations) = &system_node.operations_block_node_opt {
            for operation_rcref in &operations.operations {
                let operation_node = operation_rcref.borrow();
                self.operation_names.insert(operation_node.name.clone());
            }
        }
        
        // Generate machine block
        if let Some(machine) = &system_node.machine_block_node_opt {
            self.builder.newline();
            self.builder.write_comment("===================== Machine Block ===================");
            self.visit_machine_block_node(machine);
            
            // Generate state dispatchers
            self.builder.newline();
            self.builder.write_comment("===================== State Dispatchers ===================");
            self.generate_state_dispatchers(machine);
        }
        
        // Generate actions
        if let Some(actions) = &system_node.actions_block_node_opt {
            self.builder.newline();
            self.builder.write_comment("===================== Actions Block ===================");
            self.visit_actions_block_node(actions);
        }
        
        // Generate operations
        if let Some(operations) = &system_node.operations_block_node_opt {
            self.builder.newline();
            self.builder.write_comment("==================== Operations Block =================");
            self.visit_operations_block_node(operations);
        }
        
        // Generate system runtime
        self.generate_system_runtime(system_node);
        
        // Add async start method if needed
        if self.system_has_async_runtime {
            self.builder.newline();
            self.builder.write_function("async_start", "self", true, 0);
            self.builder.write_comment("Send startup event for async systems");
            self.builder.writeln("if hasattr(self, '__startup_event'):");
            self.builder.indent();
            self.builder.writeln("await self.__kernel(self.__startup_event)");
            self.builder.writeln("del self.__startup_event");
            self.builder.dedent();
            self.builder.dedent();
        }
        
        self.builder.end_class();
        
        // Generate module-level wrappers for actions if they exist
        // This allows module-level functions to call system actions
        if let Some(actions) = &system_node.actions_block_node_opt {
            // Create a singleton instance of the system
            self.builder.newline();
            self.builder.writeln(&format!("# Module-level singleton instance for {}", system_node.name));
            self.builder.writeln(&format!("_{}_instance = None", system_node.name.to_lowercase()));
            self.builder.newline();
            
            // Generate wrapper function for each action
            for action_rcref in &actions.actions {
                let action = action_rcref.borrow();
                self.generate_module_level_action_wrapper(&system_node.name, &action);
            }
        }
    }
    
    fn generate_module_level_action_wrapper(&mut self, system_name: &str, action: &ActionNode) {
        // Generate module-level wrapper function for action
        let params = if let Some(params) = &action.params {
            params.iter()
                .map(|p| p.param_name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        self.builder.newline();
        self.builder.write_function(&action.name, &params, action.is_async, 0);
        
        // Get or create singleton instance
        self.builder.writeln(&format!("global _{}_instance", system_name.to_lowercase()));
        self.builder.writeln(&format!("if _{}_instance is None:", system_name.to_lowercase()));
        self.builder.indent();
        self.builder.writeln(&format!("_{}_instance = {}()", system_name.to_lowercase(), system_name));
        self.builder.dedent();
        
        // Call the action on the singleton (method already has system prefix in its definition)
        let action_call = if params.is_empty() {
            format!("_{}_instance._{}__{}__{}()", system_name.to_lowercase(), system_name, system_name, action.name)
        } else {
            format!("_{}_instance._{}__{}__{}({})", system_name.to_lowercase(), system_name, system_name, action.name, params)
        };
        
        if action.is_async {
            self.builder.writeln(&format!("return await {}", action_call));
        } else {
            self.builder.writeln(&format!("return {}", action_call));
        }
        
        self.builder.dedent();
    }
    
    fn visit_state_node(&mut self, state_node: &StateNode) {
        self.current_state_name_opt = Some(state_node.name.clone());
        
        // Track parent state for => $^ dispatch in event handlers (HSM support)
        self.current_state_parent_opt = match &state_node.dispatch_opt {
            Some(dispatch) => Some(dispatch.target_state_ref.name.clone()),
            None => None,
        };
        
        // Track state parameters
        self.current_state_params.clear();
        if let Some(params) = &state_node.params_opt {
            for param in params {
                self.current_state_params.insert(param.param_name.clone());
            }
        }
        
        // Generate event handler functions
        for evt_handler_rcref in &state_node.evt_handlers_rcref {
            let evt_handler = evt_handler_rcref.borrow();
            self.generate_event_handler(&state_node.name, &evt_handler);
        }
        
        self.current_state_name_opt = None;
        self.current_state_parent_opt = None;
        self.current_state_params.clear();
    }
    
    fn visit_interface_block_node(&mut self, interface_block: &InterfaceBlockNode) {
        self.builder.newline();
        self.builder.write_comment("==================== Interface Block ==================");
        
        for method in &interface_block.interface_methods {
            let method = method.borrow();
            self.generate_interface_method(&method);
        }
    }
    
    fn visit_actions_block_node(&mut self, actions_block: &ActionsBlockNode) {
        // Action names already collected before machine block processing
        // Generate each action in the actions block
        for action_rcref in &actions_block.actions {
            let action_node = action_rcref.borrow();
            self.visit_action_node(&*action_node);
        }
    }
    
    fn visit_operations_block_node(&mut self, operations_block: &OperationsBlockNode) {
        // Track operation names for call resolution
        for operation_rcref in &operations_block.operations {
            let operation_node = operation_rcref.borrow();
            self.operation_names.insert(operation_node.name.clone());
        }
        
        // Generate each operation in the operations block
        for operation_rcref in &operations_block.operations {
            let operation_node = operation_rcref.borrow();
            self.visit_operation_node(&*operation_node);
        }
    }
    
    fn visit_class_node(&mut self, class_node: &ClassNode) {
        // Track current class context
        self.current_class_name_opt = Some(class_node.name.clone());
        
        self.builder.newline();
        // Pass parent class for inheritance
        let parent_class = class_node.parent.as_deref();
        self.builder.write_class(&class_node.name, parent_class, Some(class_node.line));
        self.builder.newline();
        
        // Generate static/class variables
        for var in &class_node.static_vars {
            let var = var.borrow();
            let mut init_value = String::new();
            self.visit_expr_node_to_string(&var.value_rc, &mut init_value);
            self.builder.writeln(&format!("{} = {}", var.name, init_value));
        }
        
        // Generate constructor if present
        if let Some(constructor) = &class_node.constructor {
            let method = constructor.borrow();
            self.visit_method_node(&*method);
        }
        
        // Generate regular methods
        for method_rcref in &class_node.methods {
            let method = method_rcref.borrow();
            self.visit_method_node(&*method);
        }
        
        // Generate static methods
        for method_rcref in &class_node.static_methods {
            let method = method_rcref.borrow();
            self.builder.newline();
            // Map decorator to the method line
            self.builder.writeln_mapped("@staticmethod", method.line);
            self.visit_method_node(&*method);
        }
        
        // Generate class methods
        for method_rcref in &class_node.class_methods {
            let method = method_rcref.borrow();
            self.builder.newline();
            self.builder.writeln("@classmethod");
            self.visit_method_node(&*method);
        }
        
        // Generate properties
        for property_rcref in &class_node.properties {
            let property = property_rcref.borrow();
            
            // Generate getter if present
            if let Some(getter) = &property.getter {
                let method = getter.borrow();
                self.builder.newline();
                self.builder.writeln("@property");
                self.visit_method_node(&*method);
            }
            
            // Generate setter if present
            if let Some(setter) = &property.setter {
                let method = setter.borrow();
                self.builder.newline();
                self.builder.writeln(&format!("@{}.setter", property.name));
                self.visit_method_node(&*method);
            }
            
            // Generate deleter if present (rarely used)
            if let Some(deleter) = &property.deleter {
                let method = deleter.borrow();
                self.builder.newline();
                self.builder.writeln(&format!("@{}.deleter", property.name));
                self.visit_method_node(&*method);
            }
        }
        
        self.builder.end_class();
        
        // Clear class context
        self.current_class_name_opt = None;
    }
    
    fn visit_module_node(&mut self, module_node: &ModuleNode) {
        // eprintln!("visit_module_node: {}", module_node.name);
        // Only save parent state if we're in a nested module (module_context is not empty)
        let should_restore = !self.module_context.is_empty();
        let saved_module_name = if should_restore { self.current_module_name_opt.clone() } else { None };
        let saved_module_variables = if should_restore { self.current_module_variables.clone() } else { HashSet::new() };
        
        // Push module name to context stack
        self.module_context.push(module_node.name.clone());
        
        // Set current module name to the full path
        self.current_module_name_opt = Some(self.module_context.join("."));
        
        // Clear and set current module's variables (don't inherit parent's)
        self.current_module_variables.clear();
        for var in &module_node.variables {
            let var_name = var.borrow().name.clone();
            self.current_module_variables.insert(var_name);
        }
        
        
        // Generate module as a Python class to act as namespace
        self.builder.newline();
        self.builder.writeln(&format!("class {}:", module_node.name));
        self.builder.indent();
        
        let mut has_content = false;
        
        // Process module variables as class variables
        for var in &module_node.variables {
            let var = var.borrow();
            let mut init_value = String::new();
            self.visit_expr_node_to_string(&var.value_rc, &mut init_value);
            self.builder.writeln(&format!("{} = {}", var.name, init_value));
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
        
        // Process module functions as static methods
        for func in &module_node.functions {
            if has_content {
                self.builder.newline();
            }
            let func = func.borrow();
            // Map decorator to the function line
            self.builder.writeln_mapped("@staticmethod", func.line);
            self.builder.write(&format!("def {}(", func.name));
            
            // Generate parameters
            if let Some(params) = &func.params {
                let param_names: Vec<String> = params.iter()
                    .map(|p| p.param_name.clone())
                    .collect();
                self.builder.write(&param_names.join(", "));
            }
            self.builder.writeln("):"); // Use writeln to add newline after :
            self.builder.indent();
            
            // Generate function body
            if func.statements.is_empty() {
                // Check for terminator expression (return statement in function signature)
                if let Some(ref return_expr) = func.terminator_expr.return_expr_t_opt {
                        let mut output = String::new();
                        self.visit_expr_node_to_string(return_expr, &mut output);
                    self.builder.writeln(&format!("return {}", output));
                } else {
                    self.builder.writeln("pass");
                }
            } else {
                for stmt in &func.statements {
                    match stmt {
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_statement(stmt_t);
                        }
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            var_decl_t_rcref.borrow().accept(self);
                        }
                    }
                }
            }
            self.builder.dedent();
            self.builder.newline(); // Add newline after function
            has_content = true;
        }
        
        // If module is empty, add pass
        if !has_content {
            self.builder.writeln("pass");
        }
        
        self.builder.dedent();
        
        // Pop module name from context stack
        self.module_context.pop();
        
        // Only restore parent module state if we saved it (i.e., this was a nested module)
        if should_restore {
            self.current_module_name_opt = saved_module_name;
            self.current_module_variables = saved_module_variables;
        }
    }
    
    fn visit_enum_decl_node(&mut self, enum_node: &EnumDeclNode) {
        // Module-level enums use their original name
        self.generate_enum_with_name(enum_node, &enum_node.name);
        
        // Add auto import if needed
        if enum_node.enums.iter().any(|e| matches!(e.value, EnumValue::Auto) && !matches!(enum_node.enum_type, EnumType::String)) {
            if !self.imports.contains(&"from enum import auto".to_string()) {
                self.imports.push("from enum import auto".to_string());
            }
        }
    }
    
    fn visit_method_node(&mut self, method: &MethodNode) {
        // For class methods, filter out 'cls' parameter if present (it's implicit in Python)
        let params = if let Some(params) = &method.params {
            let param_list: Vec<_> = if method.is_class && !params.is_empty() && params[0].param_name == "cls" {
                // Skip the first parameter if it's 'cls' in a class method
                params.iter().skip(1).map(|p| p.param_name.clone()).collect()
            } else {
                params.iter().map(|p| p.param_name.clone()).collect()
            };
            param_list.join(", ")
        } else {
            String::new()
        };
        
        // Check if this is a constructor 
        let is_constructor = method.is_constructor || method.name == "init";
        let method_name = if is_constructor {
            "__init__".to_string()
        } else {
            method.name.clone()
        };
        
        // Use method's own is_static and is_class flags
        let is_static = method.is_static;
        let is_class = method.is_class;
        
        let full_params = if is_static {
            params.clone()
        } else if is_class {
            // Class methods get 'cls' as the first parameter
            if params.is_empty() {
                "cls".to_string()
            } else {
                format!("cls, {}", params)
            }
        } else if params.is_empty() {
            "self".to_string()
        } else {
            format!("self, {}", params)
        };
        
        self.builder.newline();
        
        // Static decorator is already added in visit_class_node for static methods
        
        self.builder.write_function(
            &method_name,
            &full_params,
            false,  // MethodNode doesn't have is_async field
            method.line
        );
        
        // Generate method body
        if method.statements.is_empty() && method.terminator_expr.return_expr_t_opt.is_none() {
            // Empty method with no return value - use pass
            self.builder.writeln("pass");
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
                        self.builder.writeln_mapped(&format!("return {}", ret_val), method.terminator_expr.line);
                    } else {
                        // Return without value
                        self.builder.writeln_mapped("return", method.terminator_expr.line);
                    }
                }
            }
        }
        
        self.builder.end_function();
    }
    
    fn visit_action_node(&mut self, action_node: &ActionNode) {
        let params = if let Some(params) = &action_node.params {
            params.iter()
                .map(|p| p.param_name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        let full_params = if params.is_empty() {
            "self".to_string()
        } else {
            format!("self, {}", params)
        };
        
        self.builder.newline();
        self.builder.write_function(
            &format!("__{}__{}", self.system_name, action_node.name),
            &full_params,
            action_node.is_async,
            action_node.line  // v0.78.7: now has line field for source mapping
        );
        
        // Generate the action body
        if let Some(code) = &action_node.code_opt {
            // If there's code_opt, use it
            self.builder.writeln(&format!("# TODO: Python code opt not implemented"));
            self.builder.writeln(&code);
        } else if !action_node.statements.is_empty() {
            // Otherwise generate from statements
            let statements = &action_node.statements;
            for stmt in statements {
                self.visit_decl_or_stmt(stmt);
            }
        } else {
            self.builder.writeln("pass");
        }
        
        self.builder.end_function();
    }
    
    fn visit_operation_node(&mut self, operation_node: &OperationNode) {
        let params = if let Some(params) = &operation_node.params {
            params.iter()
                .map(|p| p.param_name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        let is_static = operation_node.attributes_opt.as_ref()
            .map(|attrs| attrs.contains_key("static") || attrs.contains_key("staticmethod"))
            .unwrap_or(false);
        
        let full_params = if is_static {
            params.clone()
        } else if params.is_empty() {
            "self".to_string()
        } else {
            format!("self, {}", params)
        };
        
        self.builder.newline();
        
        if is_static {
            // Map decorator to the operation line
            self.builder.writeln_mapped("@staticmethod", operation_node.line);
        }
        
        self.builder.write_function(
            &operation_node.name,
            &full_params,
            operation_node.is_async,
            operation_node.line  // v0.78.2: Use actual line from Frame source
        );
        
        if !operation_node.statements.is_empty() {
            let statements = &operation_node.statements;
            for stmt in statements {
                self.visit_decl_or_stmt(stmt);
            }
        } else {
            self.builder.writeln("pass");
        }
        
        self.builder.end_function();
    }
    
    fn visit_import_node(&mut self, import_node: &ImportNode) {
        let import_stmt = match &import_node.import_type {
            ImportType::Simple { module } => {
                format!("import {}", module)
            }
            ImportType::Aliased { module, alias } => {
                format!("import {} as {}", module, alias)
            }
            ImportType::FromImport { module, items } => {
                let imports = items.join(", ");
                format!("from {} import {}", module, imports)
            }
            ImportType::FromImportAll { module } => {
                format!("from {} import *", module)
            }
            ImportType::FrameModule { module_name, .. } => {
                // Frame module imports - just import as Python module
                format!("import {}", module_name)
            }
            ImportType::FrameModuleAliased { module_name, alias, .. } => {
                format!("import {} as {}", module_name, alias)
            }
            ImportType::FrameSelective { .. } => {
                // For now, ignore Frame selective imports
                return;
            }
        };
        
        self.imports.push(import_stmt);
    }
}

// Helper methods
impl PythonVisitorV2 {
    // Helper method to find a state node by name in the current system
    fn get_state_node(&self, state_name: &str) -> Option<Rc<RefCell<StateNode>>> {
        // Search through the system symbols to find the state
        for arc in &self.arcanum {
            for system_symbol_rcref in &arc.system_symbols {
                let system_symbol = system_symbol_rcref.borrow();
                if let Some(machine_block_symbol_opt) = &system_symbol.machine_block_symbol_opt {
                    let machine_symbol = machine_block_symbol_opt.borrow();
                    let states_symtab = machine_symbol.symtab_rcref.borrow();
                    if let Some(symbol) = states_symtab.symbols.get(state_name) {
                        let symbol_type = symbol.borrow();
                        if let SymbolType::State { state_symbol_ref } = &*symbol_type {
                            let state_symbol = state_symbol_ref.borrow();
                            if let Some(state_node) = &state_symbol.state_node_opt {
                                return Some(state_node.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    
    fn check_system_async(&self, system_node: &SystemNode) -> bool {
        // Check runtime info
        if let Some(runtime) = &system_node.runtime_info {
            if runtime.kernel.is_async || runtime.router.is_async {
                return true;
            }
        }
        
        // Check interface methods
        if let Some(interface) = &system_node.interface_block_node_opt {
            for method in &interface.interface_methods {
                if method.borrow().is_async {
                    return true;
                }
            }
        }
        
        // Check machine states
        if let Some(machine) = &system_node.machine_block_node_opt {
            for state_rcref in &machine.states {
                let state = state_rcref.borrow();
                for evt_handler_rcref in &state.evt_handlers_rcref {
                    if evt_handler_rcref.borrow().is_async {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    fn check_handler_has_async_operations(&self, evt_handler: &EventHandlerNode) -> bool {
        // Check all statements in the handler for async operations
        for stmt in &evt_handler.statements {
            if self.statement_contains_async(stmt) {
                return true;
            }
        }
        false
    }
    
    fn get_statement_line(&self, stmt: &StatementType) -> usize {
        match stmt {
            StatementType::ExpressionStmt { expr_stmt_t } => {
                match expr_stmt_t {
                    ExprStmtType::CallStmtT { call_stmt_node } => call_stmt_node.line,
                    ExprStmtType::AssignmentStmtT { assignment_stmt_node } => assignment_stmt_node.line,
                    ExprStmtType::VariableStmtT { variable_stmt_node } => variable_stmt_node.line,
                    ExprStmtType::ExprListStmtT { expr_list_stmt_node } => expr_list_stmt_node.line,
                    ExprStmtType::BinaryStmtT { binary_stmt_node } => binary_stmt_node.line,
                    _ => 0,
                }
            }
            StatementType::TransitionStmt { transition_statement_node } => transition_statement_node.line,
            StatementType::StateStackStmt { .. } => 0,
            StatementType::IfStmt { if_stmt_node } => if_stmt_node.line,
            StatementType::ForStmt { for_stmt_node } => for_stmt_node.line,
            StatementType::WhileStmt { while_stmt_node } => while_stmt_node.line,
            StatementType::LoopStmt { loop_stmt_node } => loop_stmt_node.line,
            StatementType::ContinueStmt { continue_stmt_node } => continue_stmt_node.line,
            StatementType::BreakStmt { break_stmt_node } => break_stmt_node.line,
            StatementType::ReturnStmt { return_stmt_node } => return_stmt_node.line,
            StatementType::ParentDispatchStmt { parent_dispatch_stmt_node } => parent_dispatch_stmt_node.line,
            StatementType::WithStmt { with_stmt_node } => with_stmt_node.line,
            StatementType::MatchStmt { match_stmt_node } => match_stmt_node.line,
            StatementType::TryStmt { try_stmt_node } => try_stmt_node.line,
            StatementType::AssertStmt { assert_stmt_node } => assert_stmt_node.line,
            StatementType::DelStmt { del_stmt_node } => del_stmt_node.line,
            StatementType::RaiseStmt { raise_stmt_node } => raise_stmt_node.line,
            _ => 0,
        }
    }
    
    fn statement_contains_async(&self, stmt: &DeclOrStmtType) -> bool {
        match stmt {
            DeclOrStmtType::StmtT { stmt_t } => {
                match stmt_t {
                    StatementType::WithStmt { with_stmt_node } => {
                        // With statements can be async
                        // NOTE: Parser bug - is_async is not being set correctly for "async with"
                        // Workaround: Check if any statement within the with block contains await
                        if with_stmt_node.is_async {
                            return true;
                        }
                        // Also check statements inside the with block for await/async
                        for inner_stmt in &with_stmt_node.with_block.statements {
                            if self.statement_contains_async(inner_stmt) {
                                return true;
                            }
                        }
                        // Additional workaround: Check if with context expression suggests async usage
                        // Common async context managers: aiohttp.ClientSession, session.get, etc.
                        // This is a temporary fix until the parser correctly sets is_async
                        if self.expression_contains_async(&with_stmt_node.context_expr) {
                            return true;
                        }
                        
                        // Check if the context expression looks like an async pattern
                        if let ExprType::CallExprT { call_expr_node } = &with_stmt_node.context_expr {
                            // If there's a call chain, it might be an async context manager
                            if call_expr_node.call_chain.is_some() {
                                // For now, assume any call chain in a with statement might be async
                                // This is overly broad but ensures we don't miss async cases
                                return true;
                            }
                        }
                        false
                    }
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        // Check if expression statement contains await
                        match expr_stmt_t {
                            ExprStmtType::CallStmtT { call_stmt_node: _ } => {
                                // Check the call expression for await
                                false  // For now, we'll focus on async with statements
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
            DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                // Check if variable initialization contains await
                let var_decl = var_decl_t_rcref.borrow();
                self.expression_contains_async(&*var_decl.value_rc)
            }
        }
    }
    
    fn expression_contains_async(&self, expr: &ExprType) -> bool {
        match expr {
            ExprType::AwaitExprT { .. } => true,
            ExprType::CallExprT { call_expr_node } => {
                // Check arguments for await
                for arg_expr in &call_expr_node.call_expr_list.exprs_t {
                    if self.expression_contains_async(arg_expr) {
                        return true;
                    }
                }
                false
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                let left = binary_expr_node.left_rcref.borrow();
                let right = binary_expr_node.right_rcref.borrow();
                self.expression_contains_async(&*left) ||
                self.expression_contains_async(&*right)
            }
            _ => false,
        }
    }
    
    fn looks_like_async_context(&self, expr: &ExprType) -> bool {
        // Check if this looks like an async context manager
        // Common patterns: aiohttp.ClientSession(), session.get(), etc.
        if let ExprType::CallExprT { call_expr_node } = expr {
            // Check for call chains that typically indicate async (e.g., session.get())
            if call_expr_node.call_chain.is_some() {
                return true;
            }
        }
        // For now, this simple check should suffice
        false
    }
    
    fn generate_system_init(&mut self, system_node: &SystemNode) {
        // Generate __init__ with system parameters
        let params = if let Some(domain_params) = &system_node.domain_params_opt {
            let param_names: Vec<String> = domain_params.iter()
                .map(|p| p.param_name.clone())
                .collect();
            if param_names.is_empty() {
                "self".to_string()
            } else {
                format!("self, {}", param_names.join(", "))
            }
        } else {
            "self".to_string()
        };
        
        // Map __init__ to the system declaration line
        self.builder.writeln_mapped(&format!("def __init__({}):", params), system_node.line);
        self.builder.indent();
        
        self.builder.write_comment("Create and initialize start state compartment");
        
        if let Some(machine) = &system_node.machine_block_node_opt {
            if let Some(first_state) = machine.states.first() {
                let state = first_state.borrow();
                let state_name = self.format_state_name(&state.name);
                
                // Build state_vars dictionary for the initial state
                let mut state_vars_entries = Vec::new();
                if let Some(vars) = &state.vars_opt {
                    for var_rcref in vars {
                        let var = var_rcref.borrow();
                        let var_name = &var.name;
                        
                        // Get the initializer value
                        let mut value_str = String::new();
                        self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                        
                        // TEMPORARY WORKAROUND: If initializer references the variable itself,
                        // it's likely a parser bug. Use a default value instead.
                        let initializer_value = if value_str.contains(var_name) {
                            eprintln!("WARNING: State var '{}' initializer '{}' references itself - using 0", var_name, value_str);
                            "0".to_string()  // Use 0 as default for numeric operations
                        } else {
                            value_str
                        };
                        
                        state_vars_entries.push(format!("'{}': {}", var_name, initializer_value));
                    }
                }
                
                let state_vars_dict = if state_vars_entries.is_empty() {
                    "{}".to_string()
                } else {
                    format!("{{{}}}", state_vars_entries.join(", "))
                };
                
                // Check if initial state has a parent (for HSM support)
                if let Some(dispatch) = &state.dispatch_opt {
                    // Initial state has a parent - create parent compartment first
                    let parent_state_name = self.format_state_name(&dispatch.target_state_ref.name);
                    self.builder.writeln_mapped(&format!(
                        "parent_compartment = FrameCompartment('{}', None, None, None, None, {{}}, {{}})",
                        parent_state_name
                    ), system_node.line);
                    self.builder.writeln_mapped(&format!(
                        "self.__compartment = FrameCompartment('{}', None, None, None, parent_compartment, {}, {{}})",
                        state_name, state_vars_dict
                    ), system_node.line);
                } else {
                    // No parent - create compartment normally
                    self.builder.writeln_mapped(&format!(
                        "self.__compartment = FrameCompartment('{}', None, None, None, None, {}, {{}})",
                        state_name, state_vars_dict
                    ), system_node.line);
                }
            } else {
                self.builder.writeln_mapped("self.__compartment = None", system_node.line);
            }
        } else {
            self.builder.writeln_mapped("self.__compartment = None", system_node.line);
        }
        
        self.builder.writeln_mapped("self.__next_compartment = None", system_node.line);
        self.builder.writeln_mapped("self.return_stack = [None]", system_node.line);
        
        // Initialize domain variables
        if let Some(domain_block) = &system_node.domain_block_node_opt {
            if !domain_block.member_variables.is_empty() {
                self.builder.newline();
                self.builder.write_comment("Initialize domain variables");
                
                // Get list of parameter names for checking
                let param_names: HashSet<String> = if let Some(domain_params) = &system_node.domain_params_opt {
                    domain_params.iter().map(|p| p.param_name.clone()).collect()
                } else {
                    HashSet::new()
                };
                
                for var_rcref in &domain_block.member_variables {
                    let var = var_rcref.borrow();
                    
                    // If this variable has the same name as a parameter, use the parameter
                    // Otherwise use the default value
                    if param_names.contains(&var.name) {
                        self.builder.writeln_mapped(&format!("self.{} = {}", var.name, var.name), var.line);
                    } else {
                        let mut value_str = String::new();
                        self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                        self.builder.writeln_mapped(&format!("self.{} = {}", var.name, value_str), var.line);
                    }
                }
            }
        }
        
        // Send start event
        if system_node.machine_block_node_opt.is_some() {
            self.builder.newline();
            
            // Check if system has async runtime
            let has_async = self.check_system_async(system_node);
            if has_async {
                self.builder.write_comment("System has async runtime - start event must be sent asynchronously");
                self.builder.writeln_mapped("self.__startup_event = FrameEvent(\"$>\", None)", system_node.line);
            } else {
                self.builder.write_comment("Send system start event");
                self.builder.writeln_mapped("frame_event = FrameEvent(\"$>\", None)", system_node.line);
                self.builder.writeln_mapped("self.__kernel(frame_event)", system_node.line);
            }
        }
        
        self.builder.dedent();
    }
    
    fn generate_interface_method(&mut self, method: &InterfaceMethodNode) {
        let params = if let Some(params) = &method.params {
            params.iter()
                .map(|p| p.param_name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };
        
        let full_params = if params.is_empty() {
            "self,".to_string()
        } else {
            format!("self, {}", params)
        };
        
        // Make the method async if:
        // 1. It's explicitly marked as async, OR
        // 2. The system has async runtime (kernel is async)
        let needs_async = method.is_async || self.system_has_async_runtime;
        
        self.builder.newline();
        self.builder.write_function(
            &method.name,
            &full_params,
            needs_async,
            method.line  // v0.77: use line from InterfaceMethodNode for source mapping
        );
        
        // Map interface method internals to the method line
        self.builder.writeln_mapped("self.return_stack.append(None)", method.line);
        
        // Create event and send to kernel
        if params.is_empty() {
            self.builder.writeln_mapped(&format!("__e = FrameEvent(\"{}\", None)", method.name), method.line);
        } else {
            // Pack parameters into a dictionary
            let param_names: Vec<String> = if let Some(params) = &method.params {
                params.iter().map(|p| p.param_name.clone()).collect()
            } else {
                vec![]
            };
            
            let param_dict = param_names.iter()
                .map(|name| format!("\"{}\": {}", name, name))
                .collect::<Vec<_>>()
                .join(", ");
            
            self.builder.writeln_mapped(&format!("__e = FrameEvent(\"{}\", {{{}}})", method.name, param_dict), method.line);
        }
        
        if needs_async {
            self.builder.writeln_mapped("await self.__kernel(__e)", method.line);
        } else {
            self.builder.writeln_mapped("self.__kernel(__e)", method.line);
        }
        
        self.builder.writeln_mapped("return self.return_stack.pop(-1)", method.line);
        
        self.builder.dedent();
    }
    
    fn collect_module_vars_from_stmt(&self, stmt: &DeclOrStmtType, global_vars: &mut Vec<String>, module_symtab: &SymbolTable) {
        match stmt {
            DeclOrStmtType::StmtT { stmt_t } => {
                match stmt_t {
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        match expr_stmt_t {
                            ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                                // Check if assigning to a module variable
                                if let ExprType::CallChainExprT { call_chain_expr_node } = &*assignment_stmt_node.assignment_expr_node.l_value_box {
                                    if call_chain_expr_node.call_chain.len() == 1 {
                                        if let Some(first) = call_chain_expr_node.call_chain.front() {
                                            let var_name = match first {
                                                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => Some(&id_node.name.lexeme),
                                                CallChainNodeType::VariableNodeT { var_node } => Some(&var_node.id_node.name.lexeme),
                                                _ => None,
                                            };
                                            
                                            if let Some(name) = var_name {
                                                // Check if it's a module variable
                                                if let Some(symbol) = module_symtab.symbols.get(name) {
                                                    if matches!(&*symbol.borrow(), SymbolType::ModuleVariable { .. }) {
                                                        global_vars.push(name.clone());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    StatementType::IfStmt { if_stmt_node } => {
                        for stmt in &if_stmt_node.if_block.statements {
                            self.collect_module_vars_from_stmt(stmt, global_vars, module_symtab);
                        }
                        for elif in &if_stmt_node.elif_clauses {
                            for stmt in &elif.block.statements {
                                self.collect_module_vars_from_stmt(stmt, global_vars, module_symtab);
                            }
                        }
                        if let Some(else_block) = &if_stmt_node.else_block {
                            for stmt in &else_block.statements {
                                self.collect_module_vars_from_stmt(stmt, global_vars, module_symtab);
                            }
                        }
                    }
                    StatementType::ForStmt { for_stmt_node } => {
                        for stmt in &for_stmt_node.block.statements {
                            self.collect_module_vars_from_stmt(stmt, global_vars, module_symtab);
                        }
                    }
                    StatementType::WhileStmt { while_stmt_node } => {
                        for stmt in &while_stmt_node.block.statements {
                            self.collect_module_vars_from_stmt(stmt, global_vars, module_symtab);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    fn collect_modified_module_variables(&self, statements: &Vec<DeclOrStmtType>) -> Vec<String> {
        // eprintln!("DEBUG: collect_modified_module_variables called with {} statements", statements.len());
        let mut modified_vars = Vec::new();
        let mut local_vars = HashSet::<String>::new();
        
        // First pass: Find all local variable declarations
        for stmt in statements {
            if let DeclOrStmtType::VarDeclT { var_decl_t_rcref } = stmt {
                let var_name = var_decl_t_rcref.borrow().name.clone();
                // eprintln!("DEBUG: Found local var declaration: {}", var_name);
                local_vars.insert(var_name);
            }
        }
        
        // Second pass: Find module variables that are modified
        // Only collect module variables if we have a symbol table
        if !self.arcanum.is_empty() {
            let module_symtab = self.arcanum[0].module_symtab.borrow();
            for stmt in statements {
                self.collect_module_vars_from_stmt(stmt, &mut modified_vars, &module_symtab);
            }
        }
        
        // eprintln!("DEBUG: Before filtering, modified_vars = {:?}", modified_vars);
        
        // Remove duplicates and filter to only module variables
        // In V2, we need to check the module symbol table to see what's a module variable
        let mut seen = HashSet::<String>::new();
        let module_symtab = self.arcanum[0].module_symtab.borrow();
        modified_vars.retain(|var: &String| {
            // Only keep if it's a module variable and not locally shadowed
            if local_vars.contains(var) || !seen.insert(var.clone()) {
                return false;
            }
            
            // Check if it's a module variable in the symbol table
            if let Some(symbol) = module_symtab.symbols.get(var) {
                matches!(&*symbol.borrow(), SymbolType::ModuleVariable { .. })
            } else {
                false
            }
        });
        
        modified_vars
    }
    
    fn generate_event_handler(&mut self, state_name: &str, evt_handler: &EventHandlerNode) {
        // eprintln!("DEBUG generate_event_handler: state={}", state_name);
        let handler_name = self.format_handler_name(state_name, &evt_handler.msg_t);
        
        // Check if handler needs to be async
        let contains_async_ops = self.check_handler_has_async_operations(evt_handler);
        let is_async = evt_handler.is_async || self.system_has_async_runtime || contains_async_ops;
        
        self.builder.newline();
        self.builder.write_function(
            &handler_name,
            "self, __e, compartment",
            is_async,
            evt_handler.line
        );
        
        // Collect and generate global declarations for module variables
        let global_vars = self.collect_modified_module_variables(&evt_handler.statements);
        
        // Generate global declarations
        if !global_vars.is_empty() {
            // eprintln!("DEBUG: Generating global declaration for: {:?}", global_vars);
            self.builder.writeln(&format!("global {}", global_vars.join(", ")));
        }
        
        // Extract parameters from event if present
        // First try the event_symbol_params_opt (which the parser should populate but doesn't)
        let event_symbol = evt_handler.event_symbol_rcref.borrow();
        let params_extracted = if let Some(params) = &event_symbol.event_symbol_params_opt {
            if !params.is_empty() {
                for param in params {
                    // Map parameter extraction to the handler line
                    self.builder.writeln_mapped(&format!(
                        "{} = __e._parameters.get(\"{}\") if __e._parameters else None",
                        param.name, param.name
                    ), evt_handler.line);
                }
                true
            } else {
                false
            }
        } else {
            false
        };
        
        // If no parameters were extracted from event_symbol, try to get them from interface method
        if !params_extracted {
            if let MessageType::CustomMessage { message_node } = &evt_handler.msg_t {
                if let Some(param_names) = self.interface_methods.get(&message_node.name) {
                    for param_name in param_names {
                        // Map parameter extraction to the handler line
                        self.builder.writeln_mapped(&format!(
                            "{} = __e._parameters.get(\"{}\") if __e._parameters else None",
                            param_name, param_name
                        ), evt_handler.line);
                    }
                }
            }
        }
        
        // Generate statements
        for stmt in &evt_handler.statements {
            self.visit_decl_or_stmt(stmt);
        }
        
        // Handle terminator
        if let Some(terminator) = &evt_handler.terminator_node {
            self.visit_event_handler_terminator_node(&terminator);
        } else {
            // Map implicit return to the handler line
            self.builder.writeln_mapped("return", evt_handler.line);
        }
        
        self.builder.dedent();
    }
    
    fn format_handler_name(&self, state_name: &str, msg_type: &MessageType) -> String {
        let state_part = state_name.to_lowercase();
        let msg_part = match msg_type {
            MessageType::CustomMessage { message_node } => message_node.name.clone(),
            MessageType::None => "none".to_string(),
        };
        // Handle special enter/exit messages
        if msg_part == "$>" {
            format!("__handle_{}_enter", state_part)
        } else if msg_part == "<$" {
            format!("__handle_{}_exit", state_part)
        } else {
            format!("__handle_{}_{}", state_part, msg_part)
        }
    }
    
    fn generate_state_dispatchers(&mut self, machine: &MachineBlockNode) {
        // Just generate the state dispatchers
        // Event handlers are already generated by visit_machine_block_node
        for state_rcref in &machine.states {
            let state = state_rcref.borrow();
            self.generate_state_dispatcher(&state);
        }
    }
    
    fn generate_state_dispatcher(&mut self, state: &StateNode) {
        let state_method = self.format_state_name(&state.name);
        let needs_async = state.evt_handlers_rcref.iter().any(|h| {
            let handler = h.borrow();
            handler.is_async || self.system_has_async_runtime || self.check_handler_has_async_operations(&handler)
        });
        
        self.builder.newline();
        self.builder.write_comment("----------------------------------------");
        self.builder.write_comment(&format!("${}", &state.name));
        self.builder.newline();
        
        self.builder.write_function(
            &state_method,
            "self, __e, compartment",
            needs_async,
            state.line
        );
        
        let mut first = true;
        for evt_handler_rcref in &state.evt_handlers_rcref {
            let evt_handler = evt_handler_rcref.borrow();
            let handler_name = self.format_handler_name(&state.name, &evt_handler.msg_t);
            
            let condition = match &evt_handler.msg_t {
                MessageType::CustomMessage { message_node } => {
                    if message_node.name == "$>" {
                        "__e._message == \"$>\"".to_string()
                    } else if message_node.name == "<$" {
                        "__e._message == \"<$\"".to_string()
                    } else {
                        format!("__e._message == \"{}\"", message_node.name)
                    }
                }
                MessageType::None => "False".to_string(),
            };
            
            // Map the conditional to the event handler line
            if first {
                self.builder.write_mapped("if ", evt_handler.line);
                first = false;
            } else {
                self.builder.write_mapped("elif ", evt_handler.line);
            }
            
            self.builder.writeln(&format!("{}:", condition));
            self.builder.indent();
            
            let call = if needs_async {
                format!("return await self.{}(__e, compartment)", handler_name)
            } else {
                format!("return self.{}(__e, compartment)", handler_name)
            };
            // Map the return statement to the handler line too
            self.builder.writeln_mapped(&call, evt_handler.line);
            self.builder.dedent();
        }
        
        // If state has no event handlers, add a pass statement
        if state.evt_handlers_rcref.is_empty() {
            self.builder.writeln("pass");
        } else {
            self.builder.newline();
        }
        
        self.builder.dedent();
    }
    
    fn generate_system_runtime(&mut self, system_node: &SystemNode) {
        self.builder.newline();
        self.builder.write_comment("==================== System Runtime ===================");
        self.builder.newline();
        
        // Generate __kernel
        let is_async = self.system_has_async_runtime;
        
        self.builder.write_function("__kernel", "self, __e", is_async, 0);
        self.builder.write_comment("send event to current state");
        if is_async {
            self.builder.writeln("await self.__router(__e)");
        } else {
            self.builder.writeln("self.__router(__e)");
        }
        
        self.builder.newline();
        self.builder.write_comment("loop until no transitions occur");
        self.builder.writeln("while self.__next_compartment != None:");
        self.builder.indent();
        
        self.builder.writeln("next_compartment = self.__next_compartment");
        self.builder.writeln("self.__next_compartment = None");
        self.builder.newline();
        
        self.builder.write_comment("exit current state");
        if is_async {
            self.builder.writeln("await self.__router(FrameEvent(\"<$\", self.__compartment.exit_args))");
        } else {
            self.builder.writeln("self.__router(FrameEvent(\"<$\", self.__compartment.exit_args))");
        }
        self.builder.write_comment("change state");
        self.builder.writeln("self.__compartment = next_compartment");
        self.builder.newline();
        
        self.builder.writeln("if next_compartment.forward_event is None:");
        self.builder.indent();
        self.builder.write_comment("send normal enter event");
        if is_async {
            self.builder.writeln("await self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
        } else {
            self.builder.writeln("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
        }
        self.builder.dedent();
        
        self.builder.writeln("else:");
        self.builder.indent();
        self.builder.write_comment("forwarded event");
        self.builder.writeln("if next_compartment.forward_event._message == \"$>\":");
        self.builder.indent();
        if is_async {
            self.builder.writeln("await self.__router(next_compartment.forward_event)");
        } else {
            self.builder.writeln("self.__router(next_compartment.forward_event)");
        }
        self.builder.dedent();
        self.builder.writeln("else:");
        self.builder.indent();
        if is_async {
            self.builder.writeln("await self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
            self.builder.writeln("await self.__router(next_compartment.forward_event)");
        } else {
            self.builder.writeln("self.__router(FrameEvent(\"$>\", self.__compartment.enter_args))");
            self.builder.writeln("self.__router(next_compartment.forward_event)");
        }
        self.builder.dedent();
        self.builder.writeln("next_compartment.forward_event = None");
        self.builder.dedent();
        
        self.builder.dedent();
        self.builder.dedent();
        
        // Generate __router
        self.builder.newline();
        self.builder.write_function("__router", "self, __e, compartment=None", is_async, 0);
        
        self.builder.writeln("target_compartment = compartment or self.__compartment");
        
        if let Some(machine) = &system_node.machine_block_node_opt {
            let mut first = true;
            for state_rcref in &machine.states {
                let state = state_rcref.borrow();
                let state_name = self.format_state_name(&state.name);
                let state_method = self.format_state_name(&state.name);
                
                if first {
                    self.builder.write("if ");
                    first = false;
                } else {
                    self.builder.write("elif ");
                }
                
                self.builder.writeln(&format!("target_compartment.state == '{}':", state_name));
                self.builder.indent();
                
                if is_async {
                    self.builder.writeln(&format!("await self.{}(__e, target_compartment)", state_method));
                } else {
                    self.builder.writeln(&format!("self.{}(__e, target_compartment)", state_method));
                }
                
                self.builder.dedent();
            }
        }
        
        self.builder.dedent();
        
        // Generate __transition
        self.builder.newline();
        self.builder.write_function("__transition", "self, next_compartment", false, 0);
        self.builder.writeln("self.__next_compartment = next_compartment");
        self.builder.dedent();
    }
    
    fn visit_decl_or_stmt(&mut self, decl_or_stmt: &DeclOrStmtType) {
        match decl_or_stmt {
            DeclOrStmtType::StmtT { stmt_t } => {
                self.visit_statement(stmt_t);
            }
            DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                let var_decl = var_decl_t_rcref.borrow();
                self.visit_variable_decl_node(&var_decl);
            }
        }
    }
    
    fn visit_statement(&mut self, stmt: &StatementType) {
        match stmt {
            StatementType::ExpressionStmt { expr_stmt_t } => {
                self.visit_expression_stmt(expr_stmt_t);
            }
            StatementType::TransitionStmt { transition_statement_node } => {
                self.visit_transition_statement_node(transition_statement_node);
            }
            StatementType::StateStackStmt { state_stack_operation_statement_node } => {
                self.visit_state_stack_operation_statement_node(state_stack_operation_statement_node);
            }
            StatementType::IfStmt { if_stmt_node } => {
                self.visit_if_stmt_node(if_stmt_node);
            }
            StatementType::ForStmt { for_stmt_node } => {
                self.visit_for_stmt_node(for_stmt_node);
            }
            StatementType::WhileStmt { while_stmt_node } => {
                self.visit_while_stmt_node(while_stmt_node);
            }
            StatementType::LoopStmt { loop_stmt_node } => {
                self.visit_loop_stmt_node(loop_stmt_node);
            }
            StatementType::ContinueStmt { continue_stmt_node } => {
                self.builder.map_next(continue_stmt_node.line);
                self.builder.writeln("continue");
            }
            StatementType::BreakStmt { break_stmt_node } => {
                self.builder.map_next(break_stmt_node.line);
                self.builder.writeln("break");
            }
            StatementType::BlockStmt { block_stmt_node } => {
                self.visit_block_stmt_node(block_stmt_node);
            }
            StatementType::ReturnStmt { return_stmt_node } => {
                self.visit_return_stmt_node(return_stmt_node);
            }
            StatementType::ReturnAssignStmt { return_assign_stmt_node } => {
                self.visit_return_assign_stmt_node(return_assign_stmt_node);
            }
            StatementType::ParentDispatchStmt { parent_dispatch_stmt_node } => {
                self.visit_parent_dispatch_stmt_node(parent_dispatch_stmt_node);
            }
            StatementType::AssertStmt { assert_stmt_node } => {
                self.visit_assert_stmt_node(assert_stmt_node);
            }
            StatementType::DelStmt { del_stmt_node } => {
                self.visit_del_stmt_node(del_stmt_node);
            }
            StatementType::TryStmt { try_stmt_node } => {
                self.visit_try_stmt_node(try_stmt_node);
            }
            StatementType::RaiseStmt { raise_stmt_node } => {
                self.visit_raise_stmt_node(raise_stmt_node);
            }
            StatementType::WithStmt { with_stmt_node } => {
                self.visit_with_stmt_node(with_stmt_node);
            }
            StatementType::MatchStmt { match_stmt_node } => {
                self.visit_match_stmt_node(match_stmt_node);
            }
            _ => {
                eprintln!("WARNING: Unimplemented statement type in python_visitor_v2");
                self.builder.writeln("# Unimplemented statement type");
            }
        }
    }
    
    fn visit_event_handler_terminator_node(&mut self, terminator: &TerminatorExpr) {
        match &terminator.terminator_type {
            TerminatorType::Return => {
                // Only add return if there's no explicit return value
                // (explicit returns are handled as statements)
                if terminator.return_expr_t_opt.is_none() {
                    // Map return to the terminator line
                    self.builder.writeln_mapped("return", terminator.line);
                }
            }
        }
    }
    
    fn collect_global_vars_in_stmt(&self, stmt: &DeclOrStmtType, globals: &mut Vec<String>) {
        // Collect global variables that are modified in the statement
        match stmt {
            DeclOrStmtType::StmtT { stmt_t } => {
                self.collect_global_vars_in_statement_type(stmt_t, globals);
            }
            DeclOrStmtType::VarDeclT { .. } => {
                // Variable declarations don't need global declarations
            }
        }
    }
    
    fn collect_global_vars_in_statement_type(&self, stmt: &StatementType, globals: &mut Vec<String>) {
        match stmt {
            StatementType::ExpressionStmt { expr_stmt_t } => {
                self.collect_global_vars_in_expr_stmt(expr_stmt_t, globals);
            }
            StatementType::IfStmt { if_stmt_node } => {
                // Check if block
                for stmt in &if_stmt_node.if_block.statements {
                    self.collect_global_vars_in_stmt(stmt, globals);
                }
                // Check elif branches
                for elif_clause in &if_stmt_node.elif_clauses {
                    for stmt in &elif_clause.block.statements {
                        self.collect_global_vars_in_stmt(stmt, globals);
                    }
                }
                // Check else block
                if let Some(else_block) = &if_stmt_node.else_block {
                    for stmt in &else_block.statements {
                        self.collect_global_vars_in_stmt(stmt, globals);
                    }
                }
            }
            StatementType::ForStmt { for_stmt_node } => {
                for stmt in &for_stmt_node.block.statements {
                    self.collect_global_vars_in_stmt(stmt, globals);
                }
            }
            StatementType::WhileStmt { while_stmt_node } => {
                for stmt in &while_stmt_node.block.statements {
                    self.collect_global_vars_in_stmt(stmt, globals);
                }
            }
            _ => {
                // Other statement types
            }
        }
    }
    
    fn collect_global_vars_in_expr_stmt(&self, expr_stmt: &ExprStmtType, globals: &mut Vec<String>) {
        match expr_stmt {
            ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                // Check if we're assigning to a global variable
                match &*assignment_stmt_node.assignment_expr_node.l_value_box {
                    ExprType::VariableExprT { var_node } => {
                        if self.global_vars.contains(&var_node.id_node.name.lexeme) {
                            if !globals.contains(&var_node.id_node.name.lexeme) {
                                globals.push(var_node.id_node.name.lexeme.clone());
                            }
                        }
                    }
                    ExprType::CallChainExprT { call_chain_expr_node } => {
                        // Handle simple variables represented as call chains (common in parser)
                        if call_chain_expr_node.call_chain.len() == 1 {
                            if let Some(node) = call_chain_expr_node.call_chain.front() {
                                match node {
                                    CallChainNodeType::VariableNodeT { var_node } => {
                                        if self.global_vars.contains(&var_node.id_node.name.lexeme) {
                                            if !globals.contains(&var_node.id_node.name.lexeme) {
                                                globals.push(var_node.id_node.name.lexeme.clone());
                                            }
                                        }
                                    }
                                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                                        // Handle undeclared identifiers which might be global variables
                                        if self.global_vars.contains(&id_node.name.lexeme) {
                                            if !globals.contains(&id_node.name.lexeme) {
                                                globals.push(id_node.name.lexeme.clone());
                                            }
                                        }
                                    }
                                    _ => {
                                        // Other node types aren't simple variable references
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        // Other l_value types (e.g., list indexing, attribute access)
                    }
                }
                // Also check if we're reading from global variables in the RHS
                self.collect_global_vars_in_expr(&assignment_stmt_node.assignment_expr_node.r_value_rc, globals);
            }
            _ => {
                // Other expression statement types
            }
        }
    }
    
    fn collect_global_vars_in_expr(&self, expr: &ExprType, globals: &mut Vec<String>) {
        // Check if any global variables are modified in this expression
        match expr {
            ExprType::VariableExprT { var_node: _ } => {
                // We don't add variables just for reading them, only if they're being modified
                // The modification check is done in the assignment handler above
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                self.collect_global_vars_in_expr(&binary_expr_node.left_rcref.borrow(), globals);
                self.collect_global_vars_in_expr(&binary_expr_node.right_rcref.borrow(), globals);
            }
            ExprType::CallExprT { call_expr_node } => {
                // Check arguments
                for arg in &call_expr_node.call_expr_list.exprs_t {
                    self.collect_global_vars_in_expr(arg, globals);
                }
            }
            ExprType::AssignmentExprT { assignment_expr_node } => {
                // Handle nested assignments
                if let ExprType::VariableExprT { var_node } = &*assignment_expr_node.l_value_box {
                    if self.global_vars.contains(&var_node.id_node.name.lexeme) {
                        if !globals.contains(&var_node.id_node.name.lexeme) {
                            globals.push(var_node.id_node.name.lexeme.clone());
                        }
                    }
                }
                self.collect_global_vars_in_expr(&assignment_expr_node.r_value_rc, globals);
            }
            _ => {
                // Other expression types don't typically modify globals
            }
        }
    }
    
    // Expression statement visitor
    fn visit_expression_stmt(&mut self, expr_stmt: &ExprStmtType) {
        match expr_stmt {
            ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node } => {
                self.visit_call_chain_statement_node(&call_chain_literal_stmt_node);
            }
            ExprStmtType::CallStmtT { call_stmt_node } => {
                self.visit_call_statement_node(call_stmt_node);
            }
            ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                self.visit_assignment_statement_node(assignment_stmt_node);
            }
            ExprStmtType::ExprListStmtT { expr_list_stmt_node } => {
                self.visit_expr_list_stmt_node(expr_list_stmt_node);
            }
            ExprStmtType::VariableStmtT { variable_stmt_node } => {
                self.visit_variable_stmt_node(variable_stmt_node);
            }
            ExprStmtType::BinaryStmtT { binary_stmt_node } => {
                self.visit_binary_stmt_node(binary_stmt_node);
            }
            // Return assign and parent dispatch are handled as StatementType variants, not ExprStmtType
            _ => {
                self.builder.writeln("# Unimplemented expression statement");
            }
        }
    }
    
    // Assignment statement
    fn visit_assignment_statement_node(&mut self, node: &AssignmentStmtNode) {
        // Map the statement line before writing
        self.builder.map_next(node.line);
        let mut output = String::new();
        self.visit_assignment_expr_node_to_string(&node.assignment_expr_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Assignment expression to string
    fn visit_assignment_expr_node_to_string(&mut self, node: &AssignmentExprNode, output: &mut String) {
        // Generate LHS
        // Special handling: Check if LHS is a simple identifier that's a domain variable
        // If so, add self. prefix
        let lhs_start = output.len();
        self.visit_expr_node_to_string(&node.l_value_box, output);
        
        // Check if we just generated a simple identifier that's a domain variable
        let generated_lhs = output[lhs_start..].to_string();
        if self.domain_variables.contains(&generated_lhs) && !generated_lhs.starts_with("self.") {
            // Replace with self.prefixed version
            output.truncate(lhs_start);
            output.push_str(&format!("self.{}", generated_lhs));
        }
        
        // Generate assignment operator
        match &node.assignment_op {
            AssignmentOperator::Equals => output.push_str(" = "),
            AssignmentOperator::PlusEquals => output.push_str(" += "),
            AssignmentOperator::MinusEquals => output.push_str(" -= "),
            AssignmentOperator::StarEquals => output.push_str(" *= "),
            AssignmentOperator::SlashEquals => output.push_str(" /= "),
            AssignmentOperator::PercentEquals => output.push_str(" %= "),
            AssignmentOperator::PowerEquals => output.push_str(" **= "),
            AssignmentOperator::FloorDivideEquals => output.push_str(" //= "),
            AssignmentOperator::AndEquals => output.push_str(" &= "),
            AssignmentOperator::OrEquals => output.push_str(" |= "),
            AssignmentOperator::XorEquals => output.push_str(" ^= "),
            AssignmentOperator::LeftShiftEquals => output.push_str(" <<= "),
            AssignmentOperator::RightShiftEquals => output.push_str(" >>= "),
            AssignmentOperator::MatMulEquals => output.push_str(" @= "),
        }
        
        // Generate RHS
        self.visit_expr_node_to_string(&node.r_value_rc, output);
    }
    
    // Expression node to string
    fn visit_expr_node_to_string(&mut self, expr_t: &ExprType, output: &mut String) {
        // Debug output for module variable tracking
        if let ExprType::CallChainExprT { call_chain_expr_node } = expr_t {
            if call_chain_expr_node.call_chain.len() == 1 {
                if let Some(first) = call_chain_expr_node.call_chain.front() {
                    if let CallChainNodeType::UndeclaredIdentifierNodeT { id_node } = first {
                        if self.current_module_variables.contains(&id_node.name.lexeme) {
                        }
                    }
                }
            }
        }
        
        if std::env::var("DEBUG_NEG").is_ok() {
        }
        match expr_t {
            ExprType::LiteralExprT { literal_expr_node } => {
                if std::env::var("DEBUG_NEG").is_ok() {
                }
                self.visit_literal_expression_node_to_string(literal_expr_node, output);
            }
            ExprType::VariableExprT { var_node } => {
                self.visit_variable_node_to_string(var_node, output);
            }
            ExprType::CallExprT { call_expr_node } => {
                self.visit_call_expression_node_to_string(call_expr_node, output);
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                self.visit_binary_expr_node_to_string(binary_expr_node, output);
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                self.visit_unary_expr_node_to_string(unary_expr_node, output);
            }
            ExprType::ListT { list_node } => {
                self.visit_list_node_to_string(list_node, output);
            }
            ExprType::DictLiteralT { dict_literal_node } => {
                self.visit_dict_literal_node_to_string(dict_literal_node, output);
            }
            ExprType::SetLiteralT { set_literal_node } => {
                self.visit_set_literal_node_to_string(set_literal_node, output);
            }
            ExprType::TupleLiteralT { tuple_literal_node } => {
                self.visit_tuple_literal_node_to_string(tuple_literal_node, output);
            }
            ExprType::SelfExprT { .. } => {
                output.push_str("self");
            }
            ExprType::CallChainExprT { call_chain_expr_node } => {
                // eprintln!("DEBUG V2: CallChainExprT in visit_expr_node_to_string");
                self.visit_call_chain_expr_node_to_string(call_chain_expr_node, output);
            }
            ExprType::AssignmentExprT { assignment_expr_node } => {
                self.visit_assignment_expr_node_to_string(assignment_expr_node, output);
            }
            ExprType::ExprListT { expr_list_node } => {
                self.visit_expr_list_node_to_string(&expr_list_node.exprs_t, output);
            }
            ExprType::NilExprT => {
                output.push_str("None");
            }
            ExprType::EnumeratorExprT { enum_expr_node } => {
                // Check if this is a domain enum that needs prefixing
                let enum_name = if self.domain_enums.contains(&enum_expr_node.enum_type) {
                    format!("{}_{}", self.system_name, enum_expr_node.enum_type)
                } else {
                    enum_expr_node.enum_type.clone()
                };
                output.push_str(&format!("{}.{}", enum_name, enum_expr_node.enumerator));
            }
            ExprType::WalrusExprT { assignment_expr_node } => {
                // Assignment expression: (var := expr)
                output.push('(');
                self.visit_expr_node_to_string(&assignment_expr_node.l_value_box, output);
                output.push_str(" := ");
                self.visit_expr_node_to_string(&assignment_expr_node.r_value_rc, output);
                output.push(')');
            }
            ExprType::LambdaExprT { lambda_expr_node } => {
                // Lambda expression: lambda params: expr
                output.push_str("lambda ");
                if !lambda_expr_node.params.is_empty() {
                    let mut first = true;
                    for param in &lambda_expr_node.params {
                        if !first {
                            output.push_str(", ");
                        }
                        output.push_str(param);
                        first = false;
                    }
                }
                output.push_str(": ");
                self.visit_expr_node_to_string(&lambda_expr_node.body, output);
            }
            ExprType::FunctionRefT { name } => {
                // Function reference (just the name, no call)
                output.push_str(name);
            }
            ExprType::ListComprehensionExprT { list_comprehension_node } => {
                // List comprehension: [expr for var in iter if cond]
                output.push('[');
                self.visit_expr_node_to_string(&list_comprehension_node.expr, output);
                output.push_str(" for ");
                output.push_str(&list_comprehension_node.target);
                output.push_str(" in ");
                self.visit_expr_node_to_string(&list_comprehension_node.iter, output);
                if let Some(cond) = &list_comprehension_node.condition {
                    output.push_str(" if ");
                    self.visit_expr_node_to_string(cond, output);
                }
                output.push(']');
            }
            ExprType::SetComprehensionExprT { set_comprehension_node } => {
                // Set comprehension: {expr for var in iter if cond}
                output.push('{');
                self.visit_expr_node_to_string(&set_comprehension_node.expr, output);
                output.push_str(" for ");
                output.push_str(&set_comprehension_node.target);
                output.push_str(" in ");
                self.visit_expr_node_to_string(&set_comprehension_node.iter, output);
                if let Some(cond) = &set_comprehension_node.condition {
                    output.push_str(" if ");
                    self.visit_expr_node_to_string(cond, output);
                }
                output.push('}');
            }
            ExprType::DictComprehensionExprT { dict_comprehension_node } => {
                // Dict comprehension: {key: value for var in iter if cond}
                output.push('{');
                self.visit_expr_node_to_string(&dict_comprehension_node.key_expr, output);
                output.push_str(": ");
                self.visit_expr_node_to_string(&dict_comprehension_node.value_expr, output);
                output.push_str(" for ");
                output.push_str(&dict_comprehension_node.target);
                output.push_str(" in ");
                self.visit_expr_node_to_string(&dict_comprehension_node.iter, output);
                if let Some(cond) = &dict_comprehension_node.condition {
                    output.push_str(" if ");
                    self.visit_expr_node_to_string(cond, output);
                }
                output.push('}');
            }
            ExprType::GeneratorExprT { generator_expr_node } => {
                // Generator expression: (expr for var in iter if cond)
                output.push('(');
                self.visit_expr_node_to_string(&generator_expr_node.expr, output);
                output.push_str(" for ");
                output.push_str(&generator_expr_node.target);
                output.push_str(" in ");
                self.visit_expr_node_to_string(&generator_expr_node.iter, output);
                if let Some(cond) = &generator_expr_node.condition {
                    output.push_str(" if ");
                    self.visit_expr_node_to_string(cond, output);
                }
                output.push(')');
            }
            ExprType::AwaitExprT { await_expr_node } => {
                // Await expression
                output.push_str("await ");
                self.visit_expr_node_to_string(&await_expr_node.expr, output);
            }
            ExprType::StarExprT { star_expr_node } => {
                // Unpacking operator: *expr
                output.push('*');
                output.push_str(&star_expr_node.identifier);
            }
            ExprType::UnpackExprT { unpack_expr_node } => {
                // Unpacking operator: *expr (alternative form)
                output.push('*');
                self.visit_expr_node_to_string(&unpack_expr_node.expr, output);
            }
            ExprType::DictUnpackExprT { dict_unpack_expr_node } => {
                // Dictionary unpacking: **expr
                output.push_str("**");
                self.visit_expr_node_to_string(&dict_unpack_expr_node.expr, output);
            }
            ExprType::YieldExprT { yield_expr_node } => {
                // Yield expression: yield or yield value
                output.push_str("yield");
                if let Some(ref expr) = yield_expr_node.expr {
                    output.push(' ');
                    self.visit_expr_node_to_string(expr, output);
                }
            }
            ExprType::YieldFromExprT { yield_from_expr_node } => {
                // Yield from expression: yield from iterable
                output.push_str("yield from ");
                self.visit_expr_node_to_string(&yield_from_expr_node.expr, output);
            }
            _ => {
                // Handle other expression types as needed
                eprintln!("WARNING: Unhandled expression type in visit_expr_node_to_string: {:?}", std::mem::discriminant(expr_t));
                output.push_str("# TODO: expr type");
            }
        }
    }
    
    // Literal expression to string
    fn visit_literal_expression_node_to_string(&mut self, node: &LiteralExprNode, output: &mut String) {
        match &node.token_t {
            TokenType::Number => {
                output.push_str(&node.value.to_string());
            }
            TokenType::String => {
                // Add quotes around string literals
                output.push('"');
                output.push_str(&node.value.to_string());
                output.push('"');
            }
            TokenType::FString | TokenType::RawString | TokenType::ByteString => {
                // These already include their prefix and quotes
                output.push_str(&node.value.to_string());
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
            _ => {
                output.push_str(&node.value.to_string());
            }
        }
    }
    
    // Identifier node to string
    fn visit_identifier_node_to_string(&mut self, node: &IdentifierNode, output: &mut String) {
        // Check if we're in a module and if this is a module variable
        // Module variables need to be qualified even within the module's static methods
        if let Some(ref module_name) = self.current_module_name_opt {
            if self.current_module_variables.contains(&node.name.lexeme) {
                // This is a module variable, qualify it with the module name
                output.push_str(&format!("{}.{}", module_name, node.name.lexeme));
                return;
            }
        }
        
        // IdentifierNode just has a name token, no scope field
        output.push_str(&node.name.lexeme);
    }
    
    // If statement
    fn visit_if_stmt_node(&mut self, node: &IfStmtNode) {
        // Map the if statement line
        self.builder.map_next(node.line);
        // If condition
        let mut cond_str = String::new();
        self.visit_expr_node_to_string(&node.condition, &mut cond_str);
        self.builder.writeln(&format!("if {}:", cond_str));
        self.builder.indent();
        
        // If block
        self.visit_block_stmt_node(&node.if_block);
        
        self.builder.dedent();
        
        // Elif blocks
        for elif in &node.elif_clauses {
            // Map the elif to its source line
            self.builder.map_next(elif.line);
            let mut elif_cond = String::new();
            self.visit_expr_node_to_string(&elif.condition, &mut elif_cond);
            self.builder.writeln(&format!("elif {}:", elif_cond));
            self.builder.indent();
            
            self.visit_block_stmt_node(&elif.block);
            
            self.builder.dedent();
        }
        
        // Else block
        if let Some(else_block) = &node.else_block {
            // v0.78.10: Use BlockStmtNode's line field for accurate mapping
            self.builder.writeln_mapped("else:", else_block.line);
            self.builder.indent();
            
            self.visit_block_stmt_node(else_block);
            
            self.builder.dedent();
        }
    }
    
    // Block statement visitor
    fn visit_block_stmt_node(&mut self, node: &BlockStmtNode) {
        if node.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for stmt in &node.statements {
                self.visit_decl_or_stmt(stmt);
            }
        }
    }
    
    // Call expression to string
    fn visit_call_expression_node_to_string(&mut self, node: &CallExprNode, output: &mut String) {
        let func_name = &node.identifier.name.lexeme;
        
        
        // Special handling for collection constructors with multiple arguments
        // Python's set(), frozenset() constructors need a single iterable, not multiple args
        // Check this BEFORE writing function name to output
        let is_special_collection = (func_name == "set" || func_name == "frozenset") && node.call_expr_list.exprs_t.len() > 1;
        
        // Check for resolved call type
        if let Some(resolved_type) = &node.resolved_type {
            
            match resolved_type {
                ResolvedCallType::Action(_) => {
                    output.push_str(&format!("self.__{}__{}", self.system_name, node.identifier.name.lexeme));
                }
                ResolvedCallType::Operation(_) => {
                    output.push_str("self.");
                    output.push_str(&node.identifier.name.lexeme);
                }
                ResolvedCallType::SystemInterface { method, .. } => {
                    output.push_str("self.");
                    output.push_str(method);
                }
                ResolvedCallType::SystemOperation { system, operation, .. } => {
                    output.push_str(system);
                    output.push('.');
                    output.push_str(operation);
                }
                ResolvedCallType::ClassMethod { class, method, is_static } => {
                    // Check if we're calling a method from within the same class
                    if let Some(ref current_class) = self.current_class_name_opt {
                        if current_class == class && !is_static {
                            // Within same class, use self.method() for instance methods
                            output.push_str("self.");
                            output.push_str(method);
                        } else {
                            // Different class or static method, use ClassName.method()
                            output.push_str(class);
                            output.push('.');
                            output.push_str(method);
                        }
                    } else {
                        // Not in a class context, use ClassName.method()
                        output.push_str(class);
                        output.push('.');
                        output.push_str(method);
                    }
                }
                ResolvedCallType::ModuleFunction { module, function } => {
                    output.push_str(module);
                    output.push('.');
                    output.push_str(function);
                }
                ResolvedCallType::External(_name) => {
                    // Check if this might be a module reference
                    // When we're inside a module and reference another nested module,
                    // we need to qualify it with the parent module path
                    // For now, just use the name as-is
                    output.push_str(&node.identifier.name.lexeme);
                }
            }
        } else {
            // Fallback: check if this is an action or operation call
            if self.action_names.contains(&node.identifier.name.lexeme) {
                // Generate action call: self.__SystemName__actionName
                output.push_str(&format!("self.__{}__{}",
                    self.system_name, node.identifier.name.lexeme));
            } else if self.operation_names.contains(&node.identifier.name.lexeme) {
                // Generate operation call: self.operationName
                output.push_str("self.");
                output.push_str(&node.identifier.name.lexeme);
            } else {
                // Regular function name
                output.push_str(&node.identifier.name.lexeme);
            }
        }
        
        // Apply special collection constructor handling
        if is_special_collection {
            // Convert set(1, 2, 3) to set([1, 2, 3])
            output.push_str("([");
            self.visit_expr_list_node_to_string(&node.call_expr_list.exprs_t, output);
            output.push_str("])");
        } else {
            // Normal function call
            output.push('(');
            self.visit_expr_list_node_to_string(&node.call_expr_list.exprs_t, output);
            output.push(')');
        }
    }
    
    // Expression list to string
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
    
    // Binary expression to string
    fn visit_binary_expr_node_to_string(&mut self, node: &BinaryExprNode, output: &mut String) {
        // Left operand
        let left_expr = node.left_rcref.borrow();
        let needs_parens = self.needs_parentheses(&*left_expr);
        if needs_parens {
            output.push('(');
        }
        self.visit_expr_node_to_string(&*left_expr, output);
        if needs_parens {
            output.push(')');
        }
        
        // Operator
        output.push(' ');
        self.visit_operator_type_to_string(&node.operator, output);
        output.push(' ');
        
        // Right operand
        let right_expr = node.right_rcref.borrow();
        let needs_parens = self.needs_parentheses(&*right_expr);
        if needs_parens {
            output.push('(');
        }
        self.visit_expr_node_to_string(&*right_expr, output);
        if needs_parens {
            output.push(')');
        }
    }
    
    // Operator to string
    fn visit_operator_type_to_string(&mut self, op: &OperatorType, output: &mut String) {
        match op {
            OperatorType::Plus => output.push('+'),
            OperatorType::Minus => output.push('-'),
            OperatorType::Multiply => output.push('*'),
            OperatorType::Divide => output.push('/'),
            OperatorType::Percent => output.push('%'),
            OperatorType::Power => output.push_str("**"),
            OperatorType::FloorDivide => output.push_str("//"),
            OperatorType::MatMul => output.push('@'),
            OperatorType::EqualEqual => output.push_str("=="),
            OperatorType::NotEqual => output.push_str("!="),
            OperatorType::Less => output.push('<'),
            OperatorType::LessEqual => output.push_str("<="),
            OperatorType::Greater => output.push('>'),
            OperatorType::GreaterEqual => output.push_str(">="),
            OperatorType::LogicalAnd => output.push_str("and"),
            OperatorType::LogicalOr => output.push_str("or"),
            OperatorType::BitwiseAnd => output.push('&'),
            OperatorType::BitwiseOr => output.push('|'),
            OperatorType::BitwiseXor => output.push('^'),
            OperatorType::LeftShift => output.push_str("<<"),
            OperatorType::RightShift => output.push_str(">>"),
            OperatorType::In => output.push_str("in"),
            OperatorType::NotIn => output.push_str("not in"),
            OperatorType::Is => output.push_str("is"),
            OperatorType::IsNot => output.push_str("is not"),
            OperatorType::Unknown => output.push_str("# Unknown operator"),
            _ => {
                // These operators are not used in binary expressions
                output.push_str("# Unexpected binary operator");
            }
        }
    }
    
    // Unary expression to string
    fn visit_unary_expr_node_to_string(&mut self, node: &UnaryExprNode, output: &mut String) {
        match &node.operator {
            OperatorType::Minus | OperatorType::Negated => output.push('-'),
            OperatorType::Plus => output.push('+'),
            OperatorType::Not => output.push_str("not "),
            OperatorType::BitwiseNot => output.push('~'),
            _ => {} // Other operators are not unary
        }
        
        let right_expr = node.right_rcref.borrow();
        let needs_parens = matches!(&*right_expr, 
            ExprType::BinaryExprT { .. } | 
            ExprType::UnaryExprT { .. }
        );
        
        if needs_parens {
            output.push('(');
        }
        self.visit_expr_node_to_string(&*right_expr, output);
        if needs_parens {
            output.push(')');
        }
    }
    
    // List literal to string
    fn visit_list_node_to_string(&mut self, node: &ListNode, output: &mut String) {
        output.push('[');
        let mut first = true;
        for element in &node.exprs_t {
            if !first {
                output.push_str(", ");
            }
            first = false;
            // List elements are directly ExprType in the AST
            self.visit_expr_node_to_string(element, output);
        }
        output.push(']');
    }
    
    // Dict literal to string
    fn visit_dict_literal_node_to_string(&mut self, node: &DictLiteralNode, output: &mut String) {
        output.push('{');
        let mut first = true;
        for (key, value) in &node.pairs {
            if !first {
                output.push_str(", ");
            }
            first = false;
            
            // Check if this is a dictionary unpacking expression
            if matches!(key, ExprType::DictUnpackExprT { .. }) {
                // For unpacking expressions, just output the unpacking without ": value"
                self.visit_expr_node_to_string(key, output);
            } else {
                // Regular key-value pair
                self.visit_expr_node_to_string(key, output);
                output.push_str(": ");
                self.visit_expr_node_to_string(value, output);
            }
        }
        output.push('}');
    }
    
    // Set literal to string
    fn visit_set_literal_node_to_string(&mut self, node: &SetLiteralNode, output: &mut String) {
        if node.elements.is_empty() {
            output.push_str("set()");
        } else {
            output.push('{');
            let mut first = true;
            for element in &node.elements {
                if !first {
                    output.push_str(", ");
                }
                first = false;
                self.visit_expr_node_to_string(element, output);
            }
            output.push('}');
        }
    }
    
    // Tuple literal to string
    fn visit_tuple_literal_node_to_string(&mut self, node: &TupleLiteralNode, output: &mut String) {
        output.push('(');
        let mut first = true;
        for element in &node.elements {
            if !first {
                output.push_str(", ");
            }
            first = false;
            self.visit_expr_node_to_string(element, output);
        }
        // Single element tuple needs trailing comma
        if node.elements.len() == 1 {
            output.push(',');
        }
        output.push(')');
    }
    
    // Helper to determine if expression needs parentheses
    fn needs_parentheses(&self, expr: &ExprType) -> bool {
        matches!(expr, 
            ExprType::BinaryExprT { .. } | 
            ExprType::UnaryExprT { .. } |
            ExprType::AssignmentExprT { .. }
        )
    }
    
    // Transition statement
    fn visit_transition_statement_node(&mut self, node: &TransitionStatementNode) {
        use crate::frame_c::ast::IdentifierDeclScope;
        
        // Map the transition statement to the Frame source line
        self.builder.map_next(node.line);
        
        // Create compartment for target state
        let (target_state_name, target_state_ref, state_args_opt) = match &node.transition_expr_node.target_state_context_t {
            TargetStateContextType::StateRef { state_context_node } => {
                (self.format_state_name(&state_context_node.state_ref_node.name),
                 Some(&state_context_node.state_ref_node.name),
                 state_context_node.state_ref_args_opt.as_ref())
            }
            TargetStateContextType::StateStackPop {} => {
                // Handle state stack pop
                ("StateStackPop".to_string(), None, None)
            }
        };
        
        // Build state_vars dictionary for the target state
        let state_vars_dict = if let Some(state_name) = target_state_ref {
            // Find the state in the machine block
            if let Some(state_node_rcref) = self.get_state_node(state_name) {
                let state_node = state_node_rcref.borrow();
                let mut state_vars_entries = Vec::new();
                
                // Build a map of state param names to their transition argument values
                let mut state_param_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                if let (Some(params), Some(state_args)) = (&state_node.params_opt, state_args_opt) {
                    for (i, param) in params.iter().enumerate() {
                        if let Some(arg_expr) = state_args.exprs_t.get(i) {
                            let mut arg_value = String::new();
                            self.visit_expr_node_to_string(arg_expr, &mut arg_value);
                            state_param_map.insert(param.param_name.clone(), arg_value);
                        }
                    }
                }
                
                if let Some(vars) = &state_node.vars_opt {
                    for var_rcref in vars {
                        let var = var_rcref.borrow();
                        let var_name = &var.name;
                        
                        // Check if the initializer is a simple variable reference to a state parameter
                        // State parameters can be wrapped as either VariableExprT or CallChainExprT
                        let initializer_value = match var.value_rc.as_ref() {
                            ExprType::VariableExprT { var_node } => {
                                if let IdentifierDeclScope::StateParamScope = var_node.scope {
                                    // This is a state parameter reference - use the transition argument value
                                    if let Some(param_value) = state_param_map.get(&var_node.id_node.name.lexeme) {
                                        param_value.clone()
                                    } else {
                                        // Shouldn't happen, but fall back to default generation
                                        let mut value_str = String::new();
                                        self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                                        value_str
                                    }
                                } else {
                                    // Not a state parameter, generate normally
                                    let mut value_str = String::new();
                                    self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                                    value_str
                                }
                            }
                            ExprType::CallChainExprT { call_chain_expr_node } => {
                                // Check if it's a simple variable in a call chain
                                if call_chain_expr_node.call_chain.len() == 1 {
                                    if let Some(crate::frame_c::ast::CallChainNodeType::VariableNodeT { var_node }) = call_chain_expr_node.call_chain.front() {
                                        if let IdentifierDeclScope::StateParamScope = var_node.scope {
                                            // This is a state parameter reference wrapped in a call chain
                                            if let Some(param_value) = state_param_map.get(&var_node.id_node.name.lexeme) {
                                                param_value.clone()
                                            } else {
                                                let mut value_str = String::new();
                                                self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                                                value_str
                                            }
                                        } else {
                                            // Not a state parameter
                                            let mut value_str = String::new();
                                            self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                                            value_str
                                        }
                                    } else {
                                        // Not a simple variable node
                                        let mut value_str = String::new();
                                        self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                                        value_str
                                    }
                                } else {
                                    // Complex call chain
                                    let mut value_str = String::new();
                                    self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                                    value_str
                                }
                            }
                            _ => {
                                // Complex expression - generate normally
                                let mut value_str = String::new();
                                self.visit_expr_node_to_string(&var.value_rc, &mut value_str);
                                
                                // TEMPORARY WORKAROUND: If initializer references the variable itself,
                                // it's likely a parser bug. Use a default value instead.
                                if value_str.contains(var_name) {
                                    eprintln!("WARNING: State var '{}' initializer '{}' references itself - using 0", var_name, value_str);
                                    "0".to_string()  // Use 0 as default for numeric operations
                                } else {
                                    value_str
                                }
                            }
                        };
                        
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
            }
        } else {
            "{}".to_string()
        };
        
        // Build state_args dictionary from transition parameters
        let state_args_dict = if let (Some(state_args), Some(state_name)) = (state_args_opt, target_state_ref) {
            // Find the state node to get parameter names
            if let Some(state_node_rcref) = self.get_state_node(state_name) {
                let state_node = state_node_rcref.borrow();
                if let Some(params) = &state_node.params_opt {
                    let mut args_entries = Vec::new();
                    for (i, expr) in state_args.exprs_t.iter().enumerate() {
                        if let Some(param) = params.get(i) {
                            let mut value_str = String::new();
                            self.visit_expr_node_to_string(expr, &mut value_str);
                            args_entries.push(format!("'{}': {}", param.param_name, value_str));
                        }
                    }
                    if args_entries.is_empty() {
                        "{}".to_string()
                    } else {
                        format!("{{{}}}", args_entries.join(", "))
                    }
                } else {
                    "{}".to_string()
                }
            } else {
                "{}".to_string()
            }
        } else {
            "{}".to_string()
        };
        
        // Check if target state has a parent (for HSM support)
        let parent_compartment_creation = if let Some(target_state_name_str) = target_state_ref {
            if let Some(target_state_node_rcref) = self.get_state_node(target_state_name_str) {
                let target_state_node = target_state_node_rcref.borrow();
                if let Some(dispatch) = &target_state_node.dispatch_opt {
                    // Target state has a parent - create parent compartment first
                    let parent_state_name = self.format_state_name(&dispatch.target_state_ref.name);
                    Some((parent_state_name, true))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        if let Some((parent_state_name, _)) = parent_compartment_creation {
            // Create parent compartment and then child with parent reference
            self.builder.writeln(&format!(
                "parent_compartment = FrameCompartment('{}', None, None, None, None, {{}}, {{}})",
                parent_state_name
            ));
            self.builder.writeln_mapped(&format!(
                "next_compartment = FrameCompartment('{}', None, None, None, parent_compartment, {}, {})",
                target_state_name, state_vars_dict, state_args_dict
            ), node.line);
        } else {
            // No parent - create compartment normally
            self.builder.writeln_mapped(&format!(
                "next_compartment = FrameCompartment('{}', None, None, None, None, {}, {})",
                target_state_name, state_vars_dict, state_args_dict
            ), node.line);
        }
        self.builder.writeln_mapped("self.__transition(next_compartment)", node.line);
    }
    
    // Return statement
    fn visit_return_stmt_node(&mut self, node: &ReturnStmtNode) {
        // Map the statement line before writing
        self.builder.map_next(node.line);
        
        // Check if we're in a state handler (interface event handler)
        // If so, we need to set self.return_stack[-1] instead of returning directly
        let is_interface_handler = self.current_state_name_opt.is_some();
        
        if let Some(expr) = &node.expr_t_opt {
            let mut output = String::new();
            self.visit_expr_node_to_string(expr, &mut output);
            
            if is_interface_handler {
                // In interface handlers, set return_stack and then return
                self.builder.writeln(&format!("self.return_stack[-1] = {}", output));
                self.builder.writeln("return");
            } else {
                // Regular function return
                self.builder.writeln_mapped(&format!("return {}", output), node.line);
            }
        } else {
            self.builder.writeln_mapped("return", node.line);
        }
    }
    
    // Return assign statement (system.return = value)
    fn visit_return_assign_stmt_node(&mut self, node: &ReturnAssignStmtNode) {
        let mut output = String::new();
        self.visit_expr_node_to_string(&node.expr_t, &mut output);
        self.builder.writeln_mapped(
            &format!("self.return_stack[-1] = {}", output),
            node.line
        );
    }
    
    // Parent dispatch statement (=> $^)
    fn visit_parent_dispatch_stmt_node(&mut self, node: &ParentDispatchStmtNode) {
        // Dispatch to parent state
        if let Some(_parent_state) = &self.current_state_parent_opt {
            self.builder.writeln_mapped(
                "self.__router(__e, compartment.parent_compartment)",
                node.line
            );
        } else {
            self.builder.writeln_mapped(
                "# Warning: Parent dispatch with no parent state",
                node.line
            );
        }
    }
    
    // Variable node to string (needed for VariableExprT)
    fn visit_variable_node_to_string(&mut self, node: &VariableNode, output: &mut String) {
        use crate::frame_c::ast::IdentifierDeclScope;
        
        // Handle system.return special case
        if node.id_node.name.lexeme == "system.return" {
            output.push_str("self.return_stack[-1]");
        } else if matches!(node.scope, IdentifierDeclScope::StateParamScope) {
            // State parameters are accessed from compartment.state_args
            output.push_str(&format!("compartment.state_args[\"{}\"]", node.id_node.name.lexeme));
        } else if matches!(node.scope, IdentifierDeclScope::StateVarScope) {
            // State variables are accessed from compartment.state_vars
            output.push_str(&format!("compartment.state_vars[\"{}\"]", node.id_node.name.lexeme));
        } else if self.current_handler_locals.contains(&node.id_node.name.lexeme) ||
                  self.current_handler_params.contains(&node.id_node.name.lexeme) {
            // Local variables and parameters - use directly
            output.push_str(&node.id_node.name.lexeme);
        } else if self.domain_variables.contains(&node.id_node.name.lexeme) {
            // Domain variables need self. prefix
            output.push_str(&format!("self.{}", node.id_node.name.lexeme));
        } else if !self.module_context.is_empty() && self.current_module_variables.contains(&node.id_node.name.lexeme) {
            // This is a module variable - need to qualify with module path
            for (i, module) in self.module_context.iter().enumerate() {
                if i > 0 {
                    output.push('.');
                }
                output.push_str(module);
            }
            output.push('.');
            output.push_str(&node.id_node.name.lexeme);
        } else if self.current_module_variables.contains(&node.id_node.name.lexeme) {
            // We're in a module but module_context is empty - this can happen during function generation
            // Try to determine the module name from other context
            eprintln!("WARNING: Module variable '{}' accessed but module_context is empty", node.id_node.name.lexeme);
            eprintln!("  current_module_variables: {:?}", self.current_module_variables);
            eprintln!("  module_context: {:?}", self.module_context);
            // For now, just output the variable name without qualification
            // This will cause an error in the generated code
            output.push_str(&node.id_node.name.lexeme);
        } else {
            output.push_str(&node.id_node.name.lexeme);
        }
    }
    
    // Call chain statement
    fn visit_call_chain_statement_node(&mut self, node: &CallChainStmtNode) {
        // Map the statement line before writing
        self.builder.map_next(node.line);
        let mut output = String::new();
        self.visit_call_chain_node_to_string(&node.call_chain_literal_expr_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Call chain node to string
    fn visit_call_chain_node_to_string(&mut self, node: &CallChainExprNode, output: &mut String) {
        // Use the already implemented method
        self.visit_call_chain_expr_node_to_string(node, output);
    }
    
    // Call statement
    fn visit_call_statement_node(&mut self, node: &CallStmtNode) {
        // Map the statement line before writing
        self.builder.map_next(node.line);
        let mut output = String::new();
        self.visit_call_expression_node_to_string(&node.call_expr_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Binary statement
    fn visit_binary_stmt_node(&mut self, node: &BinaryStmtNode) {
        // Map the statement line before writing
        self.builder.map_next(node.line);
        let mut output = String::new();
        self.visit_binary_expr_node_to_string(&node.binary_expr_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Variable statement
    fn visit_variable_stmt_node(&mut self, node: &VariableStmtNode) {
        // Map the statement line before writing
        self.builder.map_next(node.var_node.line);
        let mut output = String::new();
        self.visit_variable_node_to_string(&node.var_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Expression list statement
    fn visit_expr_list_stmt_node(&mut self, node: &ExprListStmtNode) {
        // Map the statement line before writing
        self.builder.map_next(node.line);
        let mut output = String::new();
        for (i, expr) in node.expr_list_node.exprs_t.iter().enumerate() {
            if i > 0 {
                output.push_str(", ");
            }
            self.visit_expr_node_to_string(expr, &mut output);
        }
        self.builder.writeln(&output);
    }
    
    // List node visitor (for actual list rendering)
    fn visit_list_node(&mut self, node: &ListNode) {
        self.builder.write("[");
        for (i, element) in node.exprs_t.iter().enumerate() {
            if i > 0 {
                self.builder.write(", ");
            }
            self.visit_expr_node(element);
        }
        self.builder.write("]");
    }
    
    // Dict literal node visitor
    fn visit_dict_literal_node(&mut self, node: &DictLiteralNode) {
        self.builder.write("{");
        for (i, (key, value)) in node.pairs.iter().enumerate() {
            if i > 0 {
                self.builder.write(", ");
            }
            self.visit_expr_node(key);
            self.builder.write(": ");
            self.visit_expr_node(value);
        }
        self.builder.write("}");
    }
    
    // Set literal node visitor
    fn visit_set_literal_node(&mut self, node: &SetLiteralNode) {
        if node.elements.is_empty() {
            self.builder.write("set()");
        } else {
            self.builder.write("{");
            for (i, element) in node.elements.iter().enumerate() {
                if i > 0 {
                    self.builder.write(", ");
                }
                self.visit_expr_node(element);
            }
            self.builder.write("}");
        }
    }
    
    // Tuple literal node visitor
    fn visit_tuple_literal_node(&mut self, node: &TupleLiteralNode) {
        self.builder.write("(");
        for (i, element) in node.elements.iter().enumerate() {
            if i > 0 {
                self.builder.write(", ");
            }
            self.visit_expr_node(element);
        }
        if node.elements.len() == 1 {
            self.builder.write(",");
        }
        self.builder.write(")");
    }
    
    // Unary expression node
    fn visit_unary_expr_node(&mut self, node: &UnaryExprNode) {
        let mut output = String::new();
        self.visit_unary_expr_node_to_string(node, &mut output);
        self.builder.write(&output);
    }
    
    // Binary expression node
    fn visit_binary_expr_node(&mut self, node: &BinaryExprNode) {
        let mut output = String::new();
        self.visit_binary_expr_node_to_string(node, &mut output);
        self.builder.write(&output);
    }
    
    // Literal expression node
    fn visit_literal_expr_node(&mut self, node: &LiteralExprNode) {
        let mut output = String::new();
        self.visit_literal_expression_node_to_string(node, &mut output);
        self.builder.write(&output);
    }
    
    // Variable node
    fn visit_variable_node(&mut self, node: &VariableNode) {
        use crate::frame_c::ast::IdentifierDeclScope;
        
        if matches!(node.scope, IdentifierDeclScope::StateParamScope) {
            // State parameters are accessed from compartment.state_args
            self.builder.write(&format!("compartment.state_args[\"{}\"]", node.id_node.name.lexeme));
        } else if matches!(node.scope, IdentifierDeclScope::StateVarScope) {
            // State variables are accessed from compartment.state_vars
            self.builder.write(&format!("compartment.state_vars[\"{}\"]", node.id_node.name.lexeme));
        } else if self.current_handler_locals.contains(&node.id_node.name.lexeme) ||
           self.current_handler_params.contains(&node.id_node.name.lexeme) {
            // Local variables and parameters - use directly
            self.builder.write(&node.id_node.name.lexeme);
        } else if self.domain_variables.contains(&node.id_node.name.lexeme) {
            // Domain variables need self. prefix
            self.builder.write(&format!("self.{}", node.id_node.name.lexeme));
        } else {
            self.builder.write(&node.id_node.name.lexeme);
        }
    }
    
    // Expression node visitor
    fn visit_expr_node(&mut self, expr_t: &ExprType) {
        let mut output = String::new();
        self.visit_expr_node_to_string(expr_t, &mut output);
        self.builder.write(&output);
    }
    
    // Event handler node visitor
    fn visit_event_handler_node(&mut self, evt_handler: &EventHandlerNode) {
        // Get state name from current context
        let state_name = if let Some(state) = &self.current_state_name_opt {
            state.clone()
        } else {
            "unknown".to_string()
        };
        
        let handler_name = self.format_handler_name(&state_name, &evt_handler.msg_t);
        
        // Check if handler needs to be async
        let contains_async_ops = self.check_handler_has_async_operations(evt_handler);
        let is_async = evt_handler.is_async || self.system_has_async_runtime || contains_async_ops;
        
        // Clear and populate current handler parameters
        self.current_handler_params.clear();
        
        // Get event parameters from the event symbol
        let event_symbol = evt_handler.event_symbol_rcref.borrow();
        if let Some(params) = &event_symbol.event_symbol_params_opt {
            for param in params {
                self.current_handler_params.insert(param.name.clone());
            }
        }
        
        self.builder.newline();
        self.builder.write_function(
            &handler_name,
            "self, __e, compartment",
            is_async,
            evt_handler.line
        );
        
        // Collect and generate global declarations for module variables
        let global_vars = self.collect_modified_module_variables(&evt_handler.statements);
        
        // Generate global declarations
        if !global_vars.is_empty() {
            // eprintln!("DEBUG: Generating global declaration for: {:?}", global_vars);
            self.builder.writeln(&format!("global {}", global_vars.join(", ")));
        }
        
        // Extract parameters from event if present
        // First try the event_symbol_params_opt (which the parser should populate but doesn't)
        let event_symbol = evt_handler.event_symbol_rcref.borrow();
        let params_extracted = if let Some(params) = &event_symbol.event_symbol_params_opt {
            if !params.is_empty() {
                for param in params {
                    self.builder.writeln(&format!(
                        "{} = __e._parameters.get(\"{}\") if __e._parameters else None",
                        param.name, param.name
                    ));
                }
                true
            } else {
                false
            }
        } else {
            false
        };
        
        // If no parameters were extracted from event_symbol, try to get them from interface method
        if !params_extracted {
            if let MessageType::CustomMessage { message_node } = &evt_handler.msg_t {
                if let Some(param_names) = self.interface_methods.get(&message_node.name) {
                    for param_name in param_names {
                        self.builder.writeln(&format!(
                            "{} = __e._parameters.get(\"{}\") if __e._parameters else None",
                            param_name, param_name
                        ));
                    }
                }
            }
        }
        
        // Visit statements in the event handler
        for stmt in &evt_handler.statements {
            self.visit_decl_or_stmt(stmt);
        }
        
        // Check if the last statement was a return to avoid duplicates
        let last_is_return = evt_handler.statements.last().map_or(false, |stmt| {
            matches!(stmt, DeclOrStmtType::StmtT { stmt_t } if matches!(stmt_t, StatementType::ReturnStmt { .. }))
        });
        
        // Handle terminator (only if last statement wasn't already a return)
        if let Some(terminator) = &evt_handler.terminator_node {
            if !last_is_return {
                self.visit_event_handler_terminator_node(&terminator);
            }
        } else if !last_is_return {
            // Add implicit return if there was no explicit return
            self.builder.writeln("return");
        }
        
        self.builder.dedent();
        
        // Clear current handler parameters and locals
        self.current_handler_params.clear();
        self.current_handler_locals.clear();
    }
    
    // Machine block node visitor
    fn visit_machine_block_node(&mut self, machine: &MachineBlockNode) {
        // Visit all states to generate event handler implementations
        for state_rcref in &machine.states {
            let state = state_rcref.borrow();
            self.current_state_name_opt = Some(state.name.clone());
            
            for evt_handler_rcref in &state.evt_handlers_rcref {
                let evt_handler = evt_handler_rcref.borrow();
                self.visit_event_handler_node(&*evt_handler);
            }
            
            self.current_state_name_opt = None;
        }
    }
    
    // Call chain expression to string  
    fn visit_call_chain_expr_node_to_string(&mut self, node: &CallChainExprNode, output: &mut String) {
        // Special case: Check if this is super.method(...) pattern
        if node.call_chain.len() >= 2 {
            
            // Check if first element is super (could be VariableNodeT or UndeclaredIdentifierNodeT)
            let is_super_chain = if let Some(first) = node.call_chain.get(0) {
                match first {
                    CallChainNodeType::VariableNodeT { var_node } => 
                        var_node.id_node.name.lexeme == "super",
                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => 
                        id_node.name.lexeme == "super",
                    _ => false
                }
            } else {
                false
            };
                
            if is_super_chain {
                // Generate super() 
                output.push_str("super()");
                
                // Process the rest of the chain starting from index 1
                for call_part in node.call_chain.iter().skip(1) {
                    output.push('.');
                    
                    match call_part {
                        CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => {
                            // Special handling for init -> __init__
                            let method_name = if interface_method_call_expr_node.identifier.name.lexeme == "init" {
                                "__init__"
                            } else {
                                &interface_method_call_expr_node.identifier.name.lexeme
                            };
                            output.push_str(method_name);
                            output.push('(');
                            let mut first_arg = true;
                            for arg in &interface_method_call_expr_node.call_expr_list.exprs_t {
                                if !first_arg {
                                    output.push_str(", ");
                                }
                                self.visit_expr_node_to_string(arg, output);
                                first_arg = false;
                            }
                            output.push(')');
                        }
                        CallChainNodeType::UndeclaredCallT { call_node } => {
                            // Handle undeclared calls (like init on super)
                            let method_name = if call_node.identifier.name.lexeme == "init" {
                                "__init__"
                            } else {
                                &call_node.identifier.name.lexeme
                            };
                            output.push_str(method_name);
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
                        }
                        _ => {
                            // Handle other types of chain nodes if needed
                            // For now, just skip
                        }
                    }
                }
                return; // We've handled the super chain
            }
        }
        
        let mut first = true;
        
        // Track if we're in a self. context (first node is SelfT)
        let in_self_context = !node.call_chain.is_empty() && 
            matches!(node.call_chain[0], CallChainNodeType::SelfT { .. });
        
        for call_part in &node.call_chain {
            // Determine if we need a dot separator before this node
            let needs_dot = if first {
                false
            } else {
                // Check if this is a synthetic node (chained indexing)
                let is_synthetic = match call_part {
                    CallChainNodeType::ListElementNodeT { list_elem_node } => {
                        list_elem_node.identifier.name.lexeme == "@chain_index" || 
                        list_elem_node.identifier.name.lexeme == "@chain_slice"
                    }
                    CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                        list_elem_node.identifier.name.lexeme == "@chain_index" ||
                        list_elem_node.identifier.name.lexeme == "@chain_slice"
                    }
                    CallChainNodeType::SliceNodeT { slice_node } => {
                        slice_node.identifier.name.lexeme == "@chain_index" ||
                        slice_node.identifier.name.lexeme == "@chain_slice"
                    }
                    CallChainNodeType::UndeclaredSliceT { slice_node } => {
                        slice_node.identifier.name.lexeme == "@chain_index" ||
                        slice_node.identifier.name.lexeme == "@chain_slice"
                    }
                    CallChainNodeType::CallChainLiteralExprT { call_chain_literal_expr_node } => {
                        call_chain_literal_expr_node.value == "@chain_index" ||
                        call_chain_literal_expr_node.value == "@chain_slice"
                    }
                    CallChainNodeType::UndeclaredCallT { call_node } => {
                        // @indexed_call is also synthetic
                        call_node.identifier.name.lexeme == "@indexed_call"
                    }
                    _ => false
                };
                !is_synthetic
            };
            
            if needs_dot {
                output.push('.');
            }
            
            match call_part {
                CallChainNodeType::VariableNodeT { var_node } => {
                    if var_node.id_node.name.lexeme == "super" {
                        output.push_str("super()");
                    } else {
                        // Check if we're in a module context and this is a module variable
                        // In module static methods, we need to qualify module variables with the class name
                        
                        // Check if this is a module variable
                        let mut qualified = false;
                        if self.current_module_variables.contains(&var_node.id_node.name.lexeme) && first {
                            // We're in a module function - module variables need qualification
                            // Check if this looks like a module variable (not a parameter or local)
                            let is_local_or_param = self.current_handler_locals.contains(&var_node.id_node.name.lexeme) ||
                                                    self.current_handler_params.contains(&var_node.id_node.name.lexeme);
                            
                            if !is_local_or_param {
                                // This is a module variable, qualify it with the module path or saved name
                                if !self.module_context.is_empty() {
                                    // eprintln!("DEBUG: Qualifying {} with module_context {:?}", var_node.id_node.name.lexeme, self.module_context);
                                    for (i, module) in self.module_context.iter().enumerate() {
                                        if i > 0 {
                                            output.push('.');
                                        }
                                        output.push_str(module);
                                    }
                                    output.push('.');
                                    output.push_str(&var_node.id_node.name.lexeme);
                                    qualified = true;
                                } else if let Some(ref module_name) = self.current_module_name_opt {
                                    // eprintln!("DEBUG: Qualifying {} with current_module_name_opt {}", var_node.id_node.name.lexeme, module_name);
                                    // Use saved module name if module_context is empty
                                    output.push_str(module_name);
                                    output.push('.');
                                    output.push_str(&var_node.id_node.name.lexeme);
                                    qualified = true;
                                }
                            }
                        }
                        
                        // Check if this is a state variable by looking at scope
                        // For now, as a workaround, check if we're in an event handler context
                        // and if the variable matches a known state variable name
                        
                        // TODO: The parser/semantic analyzer should properly set IdentifierDeclScope::StateVarScope
                        // but currently it doesn't seem to be doing so for state variable references
                        
                        // Only continue with other checks if we haven't already qualified the variable
                        if !qualified {
                            // Check if this is truly a state variable (not a local or parameter)
                            let is_local_or_param = self.current_handler_locals.contains(&var_node.id_node.name.lexeme) ||
                                                    self.current_handler_params.contains(&var_node.id_node.name.lexeme);
                            
                            if var_node.scope == IdentifierDeclScope::StateParamScope {
                                // Access state parameters via compartment
                                output.push_str(&format!("compartment.state_args[\"{}\"]", 
                                    var_node.id_node.name.lexeme));
                            } else if !is_local_or_param && var_node.scope == IdentifierDeclScope::StateVarScope {
                            // Access state variables via compartment
                            output.push_str(&format!("compartment.state_vars[\"{}\"]", 
                                var_node.id_node.name.lexeme));
                        } else if !is_local_or_param && self.domain_variables.contains(&var_node.id_node.name.lexeme) &&
                                  !self.current_handler_params.contains(&var_node.id_node.name.lexeme) &&
                                  !in_self_context {
                            // Domain variable access (but not if it's a parameter or already in self. context)
                            output.push_str(&format!("self.{}", var_node.id_node.name.lexeme));
                        } else {
                            // Regular variable access
                            output.push_str(&var_node.id_node.name.lexeme);
                        }
                        }
                    }
                }
                CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => {
                    // Special handling for init method (should be __init__ in Python)
                    let method_name = if interface_method_call_expr_node.identifier.name.lexeme == "init" {
                        "__init__"
                    } else {
                        &interface_method_call_expr_node.identifier.name.lexeme
                    };
                    output.push_str(method_name);
                    output.push('(');
                    let mut first_arg = true;
                    for arg in &interface_method_call_expr_node.call_expr_list.exprs_t {
                        if !first_arg {
                            output.push_str(", ");
                        }
                        self.visit_expr_node_to_string(arg, output);
                        first_arg = false;
                    }
                    output.push(')');
                }
                CallChainNodeType::OperationCallT { operation_call_expr_node } => {
                    output.push_str(&operation_call_expr_node.identifier.name.lexeme);
                    output.push('(');
                    let mut first_arg = true;
                    for arg in &operation_call_expr_node.call_expr_list.exprs_t {
                        if !first_arg {
                            output.push_str(", ");
                        }
                        self.visit_expr_node_to_string(arg, output);
                        first_arg = false;
                    }
                    output.push(')');
                }
                CallChainNodeType::ActionCallT { action_call_expr_node } => {
                    // Actions use name mangling: __SystemName__action_name
                    output.push_str(&format!("__{}__{}", self.system_name, action_call_expr_node.identifier.name.lexeme));
                    output.push('(');
                    let mut first_arg = true;
                    for arg in &action_call_expr_node.call_expr_list.exprs_t {
                        if !first_arg {
                            output.push_str(", ");
                        }
                        self.visit_expr_node_to_string(arg, output);
                        first_arg = false;
                    }
                    output.push(')');
                }
                CallChainNodeType::SelfT { .. } => {
                    output.push_str("self");
                }
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    // Check for special @indexed_call node
                    if call_node.identifier.name.lexeme == "@indexed_call" {
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
                    } else {
                        // This is part of a call chain, so we need to handle it specially
                        // to avoid adding "self." prefix when it's a static method call
                        // like TestService.getDefaultConfig()
                        let func_name = &call_node.identifier.name.lexeme;
                        
                        // Only check for action/operation if this is the first node in the chain
                        // or if it's not preceded by a system/class name (to avoid incorrect
                        // self. prefix on cross-system static calls like UtilitySystem.calculate)
                        if first {
                            // Check if this is an action or operation call
                            if self.action_names.contains(func_name) {
                                // Generate action call: self.__SystemName__actionName
                                output.push_str(&format!("self.__{}__{}",
                                    self.system_name, func_name));
                            } else if self.operation_names.contains(func_name) {
                                // Generate operation call: self.operationName
                                output.push_str("self.");
                                output.push_str(func_name);
                            } else {
                                output.push_str(func_name);
                            }
                        } else {
                            // Not the first node - this is a qualified call (e.g., System.method)
                            // Don't add self. prefix
                            output.push_str(func_name);
                        }
                        
                        // Special handling for collection constructors with multiple arguments
                        // Python's set(), frozenset() constructors need a single iterable, not multiple args
                        if (func_name == "set" || func_name == "frozenset") && call_node.call_expr_list.exprs_t.len() > 1 {
                            // Convert set(1, 2, 3) to set([1, 2, 3])
                            output.push_str("([");
                            let mut first_arg = true;
                            for arg in &call_node.call_expr_list.exprs_t {
                                if !first_arg {
                                    output.push_str(", ");
                                }
                                self.visit_expr_node_to_string(arg, output);
                                first_arg = false;
                            }
                            output.push_str("])");
                        } else {
                            // Normal function call
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
                        }
                    }
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    let mut handled = false;
                    
                    // Check if this is a module variable first
                    if !handled && first && self.current_module_variables.contains(&id_node.name.lexeme) {
                        // This is a module variable that needs qualification
                        let is_local_or_param = self.current_handler_locals.contains(&id_node.name.lexeme) ||
                                                self.current_handler_params.contains(&id_node.name.lexeme);
                        
                        if !is_local_or_param {
                            // Qualify with module name or path
                            if !self.module_context.is_empty() {
                                // Use full module path if available
                                for (i, module) in self.module_context.iter().enumerate() {
                                    if i > 0 {
                                        output.push('.');
                                    }
                                    output.push_str(module);
                                }
                                output.push('.');
                                output.push_str(&id_node.name.lexeme);
                                handled = true;
                            } else if let Some(module_name) = &self.current_module_name_opt {
                                // Use saved module name if module_context is empty
                                output.push_str(module_name);
                                output.push('.');
                                output.push_str(&id_node.name.lexeme);
                                handled = true;
                            }
                        }
                    }
                    
                    // Check if this identifier is a module name that needs qualification
                    // When we're inside a module and reference a sibling module,
                    // we need to qualify it with the parent module path
                    if !handled && first && !self.module_context.is_empty() {
                        // Check if this might be a module reference
                        // For now, we'll check if it starts with uppercase (module convention)
                        if id_node.name.lexeme.chars().next().map_or(false, |c| c.is_uppercase()) {
                            // Qualify with parent module path
                            for (i, module) in self.module_context.iter().enumerate() {
                                if i > 0 {
                                    output.push('.');
                                }
                                output.push_str(module);
                            }
                            output.push('.');
                            // Note: we still need to output the identifier name below
                        }
                    }
                    
                    // Normal undeclared identifier handling (if not already handled above)
                    if !handled {
                        // Check if it's a state parameter
                        if self.current_state_params.contains(&id_node.name.lexeme) {
                            output.push_str(&format!("compartment.state_args[\"{}\"]", id_node.name.lexeme));
                        }
                        // Check if it's a domain variable that needs self. prefix
                        // But NOT if:
                        // 1. It's a parameter
                        // 2. We're already in a self. context (to avoid self.self.)
                        else if self.domain_variables.contains(&id_node.name.lexeme) &&
                           !self.current_handler_params.contains(&id_node.name.lexeme) &&
                           !in_self_context {
                            output.push_str(&format!("self.{}", id_node.name.lexeme));
                        } else {
                            output.push_str(&id_node.name.lexeme);
                        }
                    }
                }
                CallChainNodeType::ListElementNodeT { list_elem_node } => {
                    // Handle list/dict element access: identifier[expr]
                    // Check for synthetic identifier used for chained indexing
                    if list_elem_node.identifier.name.lexeme != "@chain_index" && 
                       list_elem_node.identifier.name.lexeme != "@chain_slice" {
                        output.push_str(&list_elem_node.identifier.name.lexeme);
                    }
                    output.push('[');
                    self.visit_expr_node_to_string(&list_elem_node.expr_t, output);
                    output.push(']');
                }
                CallChainNodeType::UndeclaredListElementT { list_elem_node } => {
                    // Same as ListElementNodeT for now
                    // Check for synthetic identifier used for chained indexing
                    if list_elem_node.identifier.name.lexeme != "@chain_index" && 
                       list_elem_node.identifier.name.lexeme != "@chain_slice" {
                        output.push_str(&list_elem_node.identifier.name.lexeme);
                    }
                    output.push('[');
                    self.visit_expr_node_to_string(&list_elem_node.expr_t, output);
                    output.push(']');
                }
                CallChainNodeType::SliceNodeT { slice_node } => {
                    // Handle slice operations: identifier[start:end:step]
                    // Check for synthetic identifier used for chained indexing
                    if slice_node.identifier.name.lexeme != "@chain_index" && 
                       slice_node.identifier.name.lexeme != "@chain_slice" {
                        output.push_str(&slice_node.identifier.name.lexeme);
                    }
                    output.push('[');
                    if let Some(start) = &slice_node.start_expr {
                        self.visit_expr_node_to_string(start, output);
                    }
                    output.push(':');
                    if let Some(end) = &slice_node.end_expr {
                        self.visit_expr_node_to_string(end, output);
                    }
                    if let Some(step) = &slice_node.step_expr {
                        output.push(':');
                        self.visit_expr_node_to_string(step, output);
                    }
                    output.push(']');
                }
                CallChainNodeType::UndeclaredSliceT { slice_node } => {
                    // Same as SliceNodeT for now
                    // Check for synthetic identifier used for chained indexing
                    if slice_node.identifier.name.lexeme != "@chain_index" && 
                       slice_node.identifier.name.lexeme != "@chain_slice" {
                        output.push_str(&slice_node.identifier.name.lexeme);
                    }
                    output.push('[');
                    if let Some(start) = &slice_node.start_expr {
                        self.visit_expr_node_to_string(start, output);
                    }
                    output.push(':');
                    if let Some(end) = &slice_node.end_expr {
                        self.visit_expr_node_to_string(end, output);
                    }
                    if let Some(step) = &slice_node.step_expr {
                        output.push(':');
                        self.visit_expr_node_to_string(step, output);
                    }
                    output.push(']');
                }
                CallChainNodeType::OperationRefT { operation_ref_expr_node } => {
                    output.push_str(&operation_ref_expr_node.name);
                }
                CallChainNodeType::CallChainLiteralExprT { call_chain_literal_expr_node } => {
                    // Call chain literal (simple value)
                    // Don't output synthetic markers
                    if call_chain_literal_expr_node.value != "@chain_index" && 
                       call_chain_literal_expr_node.value != "@chain_slice" {
                        output.push_str(&call_chain_literal_expr_node.value);
                    }
                }
            }
            first = false;
        }
    }

    fn visit_loop_stmt_node(&mut self, loop_stmt_node: &LoopStmtNode) {
        match &loop_stmt_node.loop_types {
            LoopStmtTypes::LoopForStmt { loop_for_stmt_node } => {
                self.visit_loop_for_stmt_node(loop_for_stmt_node);
            }
            LoopStmtTypes::LoopInStmt { loop_in_stmt_node } => {
                self.visit_loop_in_stmt_node(loop_in_stmt_node);
            }
            LoopStmtTypes::LoopInfiniteStmt { loop_infinite_stmt_node } => {
                self.visit_loop_infinite_stmt_node(loop_infinite_stmt_node);
            }
        }
    }

    fn visit_loop_for_stmt_node(&mut self, node: &LoopForStmtNode) {
        // Map the loop statement line
        self.builder.map_next(node.line);
        
        // Handle initialization
        if let Some(init_expr) = &node.loop_init_expr_rcref_opt {
            let init = init_expr.borrow();
            // Handle the various LoopFirstStmt types
            match &*init {
                LoopFirstStmt::VarAssign { assign_expr_node } => {
                    let mut left_str = String::new();
                    self.visit_expr_node_to_string(&assign_expr_node.l_value_box, &mut left_str);
                    let mut right_str = String::new();
                    self.visit_expr_node_to_string(&assign_expr_node.r_value_rc, &mut right_str);
                    self.builder.writeln(&format!("{} = {}", left_str, right_str));
                }
                LoopFirstStmt::Var { var_node } => {
                    self.builder.writeln(&var_node.id_node.name.lexeme);
                }
                LoopFirstStmt::CallChain { call_chain_expr_node } => {
                    let mut output = String::new();
                    self.visit_call_chain_expr_node_to_string(call_chain_expr_node, &mut output);
                    self.builder.writeln(&output);
                }
                LoopFirstStmt::VarDecl { var_decl_node_rcref } => {
                    let var_decl = var_decl_node_rcref.borrow();
                    self.visit_variable_decl_node(&*var_decl);
                }
                LoopFirstStmt::VarDeclAssign { var_decl_node_rcref } => {
                    let var_decl = var_decl_node_rcref.borrow();
                    self.visit_variable_decl_node(&*var_decl);
                }
                LoopFirstStmt::None => {
                    // No initialization
                }
            }
        }
        
        // Start while loop with condition
        let mut condition_str = String::new();
        if let Some(test_expr) = &node.test_expr_rcref_opt {
            let test = test_expr.borrow();
            self.visit_expr_node_to_string(&*test, &mut condition_str);
        } else {
            condition_str.push_str("True");
        }
        self.builder.writeln(&format!("while {}:", condition_str));
        self.builder.indent();
        
        // Visit loop body
        if node.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for decl_or_stmt in &node.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.visit_variable_decl_node(&*var_decl);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_statement(stmt_t);
                    }
                }
            }
        }
        
        // Handle post expression (increment/decrement)
        if let Some(post_expr) = &node.post_expr_rcref_opt {
            let post = post_expr.borrow();
            let mut post_str = String::new();
            self.visit_expr_node_to_string(&*post, &mut post_str);
            self.builder.writeln(&post_str);
        }
        
        self.builder.dedent();
    }

    fn visit_loop_in_stmt_node(&mut self, node: &LoopInStmtNode) {
        // Map the loop statement line
        self.builder.map_next(node.line);
        
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
        
        let mut expr_str = String::new();
        self.visit_expr_node_to_string(&node.iterable_expr, &mut expr_str);
        
        self.builder.writeln(&format!("for {} in {}:", var_name, expr_str));
        self.builder.indent();
        
        if node.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for decl_or_stmt in &node.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.visit_variable_decl_node(&*var_decl);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_statement(stmt_t);
                    }
                }
            }
        }
        
        self.builder.dedent();
    }

    fn visit_loop_infinite_stmt_node(&mut self, node: &LoopInfiniteStmtNode) {
        // Map the loop statement line
        self.builder.map_next(node.line);
        
        self.builder.writeln("while True:");
        self.builder.indent();
        
        if node.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for decl_or_stmt in &node.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.visit_variable_decl_node(&*var_decl);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_statement(stmt_t);
                    }
                }
            }
        }
        
        self.builder.dedent();
    }

    fn visit_while_stmt_node(&mut self, node: &WhileStmtNode) {
        // Map the while statement line
        self.builder.map_next(node.line);
        
        let mut condition_str = String::new();
        self.visit_expr_node_to_string(&node.condition, &mut condition_str);
        
        self.builder.writeln(&format!("while {}:", condition_str));
        self.builder.indent();
        
        if node.block.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for decl_or_stmt in &node.block.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.visit_variable_decl_node(&*var_decl);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_statement(stmt_t);
                    }
                }
            }
        }
        
        // Handle else clause if present
        if let Some(else_block) = &node.else_block {
            self.builder.dedent();
            // v0.78.10: Use BlockStmtNode's line field for accurate mapping
            self.builder.writeln_mapped("else:", else_block.line);
            self.builder.indent();
            
            if else_block.statements.is_empty() {
                self.builder.writeln_mapped("pass", else_block.line);
            } else {
                for decl_or_stmt in &else_block.statements {
                    match decl_or_stmt {
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            self.visit_variable_decl_node(&*var_decl);
                        }
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_statement(stmt_t);
                        }
                    }
                }
            }
        }
        
        self.builder.dedent();
    }

    fn visit_assert_stmt_node(&mut self, node: &AssertStmtNode) {
        // Map the assert statement line
        self.builder.map_next(node.line);
        
        let mut condition_str = String::new();
        self.visit_expr_node_to_string(&node.expr, &mut condition_str);
        self.builder.writeln(&format!("assert {}", condition_str));
    }

    fn visit_del_stmt_node(&mut self, node: &DelStmtNode) {
        // Map the del statement line
        self.builder.map_next(node.line);
        
        let mut target_str = String::new();
        self.visit_expr_node_to_string(&node.target, &mut target_str);
        self.builder.writeln(&format!("del {}", target_str));
    }

    fn visit_raise_stmt_node(&mut self, node: &RaiseStmtNode) {
        // Map the raise statement line
        self.builder.map_next(node.line);
        
        let mut raise_str = String::from("raise");
        
        // Add exception expression if present
        if let Some(exc_expr) = &node.exception_expr {
            raise_str.push(' ');
            let mut exc_str = String::new();
            self.visit_expr_node_to_string(exc_expr, &mut exc_str);
            raise_str.push_str(&exc_str);
        }
        
        // Add 'from' clause if present
        if let Some(from_expr) = &node.from_expr {
            raise_str.push_str(" from ");
            let mut from_str = String::new();
            self.visit_expr_node_to_string(from_expr, &mut from_str);
            raise_str.push_str(&from_str);
        }
        
        self.builder.writeln(&raise_str);
    }

    fn visit_with_stmt_node(&mut self, node: &WithStmtNode) {
        // Map the with statement line
        self.builder.map_next(node.line);
        
        // WORKAROUND: Parser doesn't properly set is_async for "async with" statements
        // Check if the context looks like an async context manager
        let should_be_async = node.is_async || self.looks_like_async_context(&node.context_expr);
        
        let mut with_line = if should_be_async {
            String::from("async with ")
        } else {
            String::from("with ")
        };
        
        // Add context expression
        let mut context_str = String::new();
        self.visit_expr_node_to_string(&node.context_expr, &mut context_str);
        with_line.push_str(&context_str);
        
        // Add optional target variable
        if let Some(target_var) = &node.target_var {
            with_line.push_str(" as ");
            with_line.push_str(target_var);
        }
        
        with_line.push(':');
        self.builder.writeln(&with_line);
        self.builder.indent();
        
        // Generate body
        if node.with_block.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for stmt in &node.with_block.statements {
                match stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.visit_variable_decl_node(&*var_decl);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_statement(&stmt_t);
                    }
                }
            }
        }
        
        self.builder.dedent();
    }

    fn visit_match_stmt_node(&mut self, node: &MatchStmtNode) {
        // Map the match statement line
        self.builder.map_next(node.line);
        
        // Generate match expression
        let mut match_expr = String::new();
        self.visit_expr_node_to_string(&node.match_expr, &mut match_expr);
        self.builder.writeln(&format!("match {}:", match_expr));
        self.builder.indent();
        
        // Generate case arms
        for case in &node.cases {
            self.visit_case_node(case);
        }
        
        self.builder.dedent();
    }

    fn visit_case_node(&mut self, case: &CaseNode) {
        let mut case_line = String::from("case ");
        
        // Generate pattern
        self.visit_pattern_node_to_string(&case.pattern, &mut case_line);
        
        // Add optional guard
        if let Some(guard) = &case.guard {
            case_line.push_str(" if ");
            let mut guard_str = String::new();
            self.visit_expr_node_to_string(guard, &mut guard_str);
            case_line.push_str(&guard_str);
        }
        
        case_line.push(':');
        
        // v0.78.7: Use CaseNode's line field directly for accurate mapping
        self.builder.writeln_mapped(&case_line, case.line);
        self.builder.indent();
        
        // Generate body
        if case.statements.is_empty() {
            // v0.78.7: Use case.line for pass statements too
            self.builder.writeln_mapped("pass", case.line);
        } else {
            for stmt in &case.statements {
                match stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.visit_variable_decl_node(&*var_decl);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_statement(&stmt_t);
                    }
                }
            }
        }
        
        self.builder.dedent();
    }

    fn visit_pattern_node_to_string(&mut self, pattern: &PatternNode, output: &mut String) {
        match pattern {
            PatternNode::Literal(literal) => {
                self.visit_literal_expression_node_to_string(literal, output);
            }
            PatternNode::Capture(name) => {
                output.push_str(name);
            }
            PatternNode::Wildcard => {
                output.push('_');
            }
            PatternNode::Sequence(elements) => {
                output.push('[');
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    self.visit_pattern_node_to_string(elem, output);
                }
                output.push(']');
            }
            PatternNode::Mapping(pairs) => {
                output.push('{');
                for (i, (key, value)) in pairs.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push('"');
                    output.push_str(key);
                    output.push_str("\": ");
                    self.visit_pattern_node_to_string(value, output);
                }
                output.push('}');
            }
            PatternNode::Or(patterns) => {
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        output.push_str(" | ");
                    }
                    self.visit_pattern_node_to_string(p, output);
                }
            }
            PatternNode::As(pattern, name) => {
                self.visit_pattern_node_to_string(pattern, output);
                output.push_str(" as ");
                output.push_str(name);
            }
            PatternNode::Star(name) => {
                output.push('*');
                output.push_str(name);
            }
            PatternNode::Class(class_name, args) => {
                output.push_str(class_name);
                output.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    self.visit_pattern_node_to_string(arg, output);
                }
                output.push(')');
            }
        }
    }

    fn visit_try_stmt_node(&mut self, node: &TryStmtNode) {
        // Map the try statement line
        self.builder.map_next(node.line);
        
        self.builder.writeln("try:");
        self.builder.indent();
        
        if node.try_block.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for decl_or_stmt in &node.try_block.statements {
                match decl_or_stmt {
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        self.visit_variable_decl_node(&*var_decl);
                    }
                    DeclOrStmtType::StmtT { stmt_t } => {
                        self.visit_statement(stmt_t);
                    }
                }
            }
        }
        self.builder.dedent();
        
        // Handle except clauses
        for except in &node.except_clauses {
            let mut except_line = String::from("except");
            
            if let Some(exception_types) = &except.exception_types {
                if !exception_types.is_empty() {
                    except_line.push(' ');
                    if exception_types.len() == 1 {
                        except_line.push_str(&exception_types[0]);
                    } else {
                        except_line.push('(');
                        except_line.push_str(&exception_types.join(", "));
                        except_line.push(')');
                    }
                }
                
                if let Some(var_name) = &except.var_name {
                    except_line.push_str(" as ");
                    except_line.push_str(var_name);
                }
            }
            
            // v0.78.7: Use ExceptClauseNode's line field directly for accurate mapping
            self.builder.writeln_mapped(&format!("{}:", except_line), except.line);
            self.builder.indent();
            
            if except.block.statements.is_empty() {
                self.builder.writeln_mapped("pass", except.line);
            } else {
                for decl_or_stmt in &except.block.statements {
                    match decl_or_stmt {
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            self.visit_variable_decl_node(&*var_decl);
                        }
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_statement(stmt_t);
                        }
                    }
                }
            }
            self.builder.dedent();
        }
        
        // Handle else clause
        if let Some(else_block) = &node.else_block {
            // v0.78.10: Use BlockStmtNode's line field for accurate mapping
            self.builder.writeln_mapped("else:", else_block.line);
            self.builder.indent();
            
            if else_block.statements.is_empty() {
                self.builder.writeln_mapped("pass", else_block.line);
            } else {
                for decl_or_stmt in &else_block.statements {
                    match decl_or_stmt {
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            self.visit_variable_decl_node(&*var_decl);
                        }
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_statement(stmt_t);
                        }
                    }
                }
            }
            self.builder.dedent();
        }
        
        // Handle finally clause
        if let Some(finally_block) = &node.finally_block {
            // v0.78.10: Use BlockStmtNode's line field for accurate mapping
            self.builder.writeln_mapped("finally:", finally_block.line);
            self.builder.indent();
            
            if finally_block.statements.is_empty() {
                self.builder.writeln_mapped("pass", finally_block.line);
            } else {
                for decl_or_stmt in &finally_block.statements {
                    match decl_or_stmt {
                        DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                            let var_decl = var_decl_t_rcref.borrow();
                            self.visit_variable_decl_node(&*var_decl);
                        }
                        DeclOrStmtType::StmtT { stmt_t } => {
                            self.visit_statement(stmt_t);
                        }
                    }
                }
            }
            self.builder.dedent();
        }
    }

    fn visit_for_stmt_node(&mut self, node: &ForStmtNode) {
        // Map the for statement line
        self.builder.map_next(node.line);
        
        // Determine the loop variable
        let var_name = if let Some(var) = &node.variable {
            var.id_node.name.lexeme.clone()
        } else if let Some(ident) = &node.identifier {
            ident.name.lexeme.clone()
        } else {
            "_".to_string()
        };
        
        // Track the loop variable as a local
        self.current_handler_locals.insert(var_name.clone());
        
        // Generate the iterable expression
        let mut iter_str = String::new();
        self.visit_expr_node_to_string(&node.iterable, &mut iter_str);
        
        // Handle enum iteration specially
        if node.is_enum_iteration {
            if let Some(enum_name) = &node.enum_type_name {
                // Check if this is a domain enum that needs prefixing
                let full_enum_name = if self.domain_enums.contains(enum_name) {
                    format!("{}_{}", self.system_name, enum_name)
                } else {
                    enum_name.clone()
                };
                self.builder.writeln(&format!("for {} in {}:", var_name, full_enum_name));
            } else {
                self.builder.writeln(&format!("for {} in {}:", var_name, iter_str));
            }
        } else {
            self.builder.writeln(&format!("for {} in {}:", var_name, iter_str));
        }
        
        self.builder.indent();
        
        // Generate the loop body
        if node.block.statements.is_empty() {
            self.builder.writeln("pass");
        } else {
            for stmt in &node.block.statements {
                self.visit_decl_or_stmt(stmt);
            }
        }
        
        self.builder.dedent();
        
        // Handle else clause if present
        if let Some(else_block) = &node.else_block {
            // v0.78.10: Use BlockStmtNode's line field for accurate mapping
            self.builder.writeln_mapped("else:", else_block.line);
            self.builder.indent();
            
            if else_block.statements.is_empty() {
                self.builder.writeln_mapped("pass", else_block.line);
            } else {
                for stmt in &else_block.statements {
                    self.visit_decl_or_stmt(stmt);
                }
            }
            
            self.builder.dedent();
        }
    }

    fn visit_state_stack_operation_statement_node(&mut self, node: &StateStackOperationStatementNode) {
        // State stack operations - v0.78.11: Add source mapping
        let line = node.state_stack_operation_node.line;
        match &node.state_stack_operation_node.operation_t {
            StateStackOperationType::Push => {
                // Push current state onto stack
                self.builder.map_next(line);  // Map the $$[+] operation
                self.builder.writeln("if not hasattr(self, '__state_stack'):");
                self.builder.indent();
                self.builder.writeln("self.__state_stack = []");
                self.builder.dedent();
                self.builder.writeln("self.__state_stack.append(self.__compartment)");
            }
            StateStackOperationType::Pop => {
                // Pop state from stack and transition
                self.builder.map_next(line);  // Map the $$[-] operation
                self.builder.writeln("if hasattr(self, '__state_stack') and self.__state_stack:");
                self.builder.indent();
                self.builder.writeln("target_compartment = self.__state_stack.pop()");
                self.builder.writeln("self.__transition(target_compartment)");
                self.builder.dedent();
                self.builder.writeln("else:");
                self.builder.indent();
                self.builder.writeln("pass  # State stack is empty");
                self.builder.dedent();
            }
        }
    }
    
}
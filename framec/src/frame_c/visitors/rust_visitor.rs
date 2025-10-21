// Rust Visitor for Frame Language Transpiler
// Generates type-safe Rust code from Frame AST using visitor generator patterns
// Generated implementation using established patterns v0.87.0

use crate::frame_c::ast::*;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::config::FrameConfig;
use crate::frame_c::scanner::{Token};
use crate::frame_c::symbol_table::{SymbolConfig, Arcanum};
use crate::frame_c::visitors::AstVisitor;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct RustConfig {
    pub features: RustFeatures,
    pub code: RustCodeConfig,
}

#[derive(Debug, Clone)]
pub struct RustFeatures {
    pub thread_safe: bool,
    pub use_arc_mutex: bool,
    pub generate_traits: bool,
    pub use_async: bool,
}

#[derive(Debug, Clone)]
pub struct RustCodeConfig {
    pub frame_event_type_name: String,
    pub state_enum_name: String,
    pub context_struct_name: String,
    pub machine_trait_name: String,
}

impl Default for RustConfig {
    fn default() -> Self {
        Self {
            features: RustFeatures {
                thread_safe: false,
                use_arc_mutex: false,
                generate_traits: true,
                use_async: false,
            },
            code: RustCodeConfig {
                frame_event_type_name: "FrameEvent".to_string(),
                state_enum_name: "State".to_string(),
                context_struct_name: "Context".to_string(),
                machine_trait_name: "StateMachine".to_string(),
            },
        }
    }
}

pub struct RustVisitor {
    // Core configuration
    config: FrameConfig,
    rust_config: RustConfig,
    
    // Code generation
    builder: CodeBuilder,
    
    // Symbol tracking
    symbol_config: SymbolConfig,
    arcanum: Vec<Arcanum>,
    
    // Current context
    current_state_name_opt: Option<String>,
    current_state_parent_opt: Option<String>,
    current_event_ret_type: String,
    current_class_name_opt: Option<String>,
    
    // System metadata
    system_name: String,
    system_has_async_runtime: bool,
    interface_methods: HashMap<String, InterfaceMethodSignature>,
    domain_variables: HashMap<String, String>, // name -> type
    current_handler_params: HashMap<String, String>, // param -> type
    current_state_params: HashMap<String, String>,
    action_signatures: HashMap<String, ActionSignature>,
    operation_signatures: HashMap<String, OperationSignature>,
    
    // State tracking
    states: Vec<String>,
    state_events: HashMap<String, Vec<String>>, // state -> event names
    
    // Type tracking
    declared_types: HashSet<String>,
    imported_types: HashSet<String>,
    
    // Generation flags
    is_generating_interface_method: bool,
    is_generating_action: bool,
    is_generating_operation: bool,
    
    // Comments (for future use)
    _comments: Vec<Token>,
}

#[derive(Debug, Clone)]
struct InterfaceMethodSignature {
    name: String,
    parameters: Vec<(String, String)>, // (name, type)
    return_type: Option<String>,
}

#[derive(Debug, Clone)]
struct ActionSignature {
    name: String,
    parameters: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct OperationSignature {
    name: String,
    parameters: Vec<(String, String)>,
    return_type: Option<String>,
}

impl RustVisitor {
    pub fn new(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
        config: FrameConfig,
        comments: Vec<Token>,
    ) -> Self {
        Self {
            config,
            rust_config: RustConfig::default(),
            builder: CodeBuilder::new("    "), // 4-space indent for Rust
            symbol_config,
            arcanum,
            current_state_name_opt: None,
            current_state_parent_opt: None,
            current_event_ret_type: String::new(),
            current_class_name_opt: None,
            system_name: String::new(),
            system_has_async_runtime: false,
            interface_methods: HashMap::new(),
            domain_variables: HashMap::new(),
            current_handler_params: HashMap::new(),
            current_state_params: HashMap::new(),
            action_signatures: HashMap::new(),
            operation_signatures: HashMap::new(),
            states: Vec::new(),
            state_events: HashMap::new(),
            declared_types: HashSet::new(),
            imported_types: HashSet::new(),
            is_generating_interface_method: false,
            is_generating_action: false,
            is_generating_operation: false,
            _comments: comments,
        }
    }
    
    /// Create a new Rust visitor with thread-safe configuration
    pub fn new_thread_safe(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
        config: FrameConfig,
        comments: Vec<Token>,
    ) -> Self {
        let mut visitor = Self::new(arcanum, symbol_config, config, comments);
        visitor.rust_config.features.thread_safe = true;
        visitor.rust_config.features.use_arc_mutex = true;
        visitor
    }
    
    pub fn run(mut self, frame_module: &FrameModule) -> String {
        // Visit the module and generate Rust code
        for system in &frame_module.systems {
            system.accept(&mut self);
        }
        
        self.builder.build().0  // CodeBuilder returns (String, Vec<SourceMapping>), we want just the String
    }
    
    // Generated Type System Methods (from manual implementation)
    fn callback_type(&self) -> String {
        let event_type = &self.rust_config.code.frame_event_type_name;
        if self.rust_config.features.thread_safe {
            format!("Box<dyn Fn({}) -> () + Send + Sync>", event_type)
        } else {
            format!("Box<dyn Fn({}) -> ()>", event_type)
        }
    }
    
    fn state_container_type(&self) -> String {
        let state_type = &self.rust_config.code.state_enum_name;
        if self.rust_config.features.thread_safe {
            format!("Arc<Mutex<{}>>", state_type)
        } else {
            format!("Rc<RefCell<{}>>", state_type)
        }
    }
    
    fn context_container_type(&self) -> String {
        let context_type = &self.rust_config.code.context_struct_name;
        if self.rust_config.features.thread_safe {
            format!("Arc<Mutex<{}>>", context_type)
        } else {
            format!("Rc<RefCell<{}>>", context_type)
        }
    }
    
    // Frame type to Rust type mapping (generated from patterns)
    fn frame_type_to_rust(&self, frame_type: &str) -> String {
        match frame_type {
            "int" => "i32".to_string(),
            "float" => "f64".to_string(),
            "string" => "String".to_string(),
            "bool" => "bool".to_string(),
            "void" => "()".to_string(),
            _ => {
                // Handle generic types like List<T>, Dict<K,V>
                if frame_type.starts_with("List<") && frame_type.ends_with(">") {
                    let inner = &frame_type[5..frame_type.len()-1];
                    format!("Vec<{}>", self.frame_type_to_rust(inner))
                } else if frame_type.starts_with("Dict<") && frame_type.ends_with(">") {
                    let inner = &frame_type[5..frame_type.len()-1];
                    if let Some(comma_pos) = inner.find(',') {
                        let key_type = inner[..comma_pos].trim();
                        let value_type = inner[comma_pos+1..].trim();
                        format!("HashMap<{}, {}>", 
                            self.frame_type_to_rust(key_type),
                            self.frame_type_to_rust(value_type))
                    } else {
                        "HashMap<String, String>".to_string() // fallback
                    }
                } else {
                    // Custom type or unrecognized - use as-is
                    frame_type.to_string()
                }
            }
        }
    }
    
    // Generated Code Generation Methods (extracted from manual implementation)
    fn generate_imports(&mut self) {
        self.builder.writeln("use std::collections::HashMap;");
        
        if self.rust_config.features.thread_safe {
            self.builder.writeln("use std::sync::{Arc, Mutex};");
        } else {
            self.builder.writeln("use std::rc::Rc;");
            self.builder.writeln("use std::cell::RefCell;");
        }
        
        if self.system_has_async_runtime {
            self.builder.writeln("use std::future::Future;");
            self.builder.writeln("use std::pin::Pin;");
        }
        
        self.builder.writeln("");
    }
    
    fn generate_frame_event_enum(&mut self) {
        let event_name = &self.rust_config.code.frame_event_type_name.clone();
        
        self.builder.writeln(&format!("#[derive(Debug, Clone)]"));
        self.builder.writeln(&format!("pub enum {} {{", event_name));
        self.builder.indent();
        
        // Generate variants for each interface method
        for (method_name, signature) in &self.interface_methods {
            if signature.parameters.is_empty() {
                self.builder.writeln(&format!("{},", self.to_pascal_case(method_name)));
            } else {
                self.builder.writeln(&format!("{} {{", self.to_pascal_case(method_name)));
                self.builder.indent();
                for (param_name, param_type) in &signature.parameters {
                    let rust_type = self.frame_type_to_rust(param_type);
                    self.builder.writeln(&format!("{}: {},", param_name, rust_type));
                }
                self.builder.dedent();
                self.builder.writeln("},");
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn generate_state_enum(&mut self) {
        let state_name = &self.rust_config.code.state_enum_name.clone();
        
        self.builder.writeln(&format!("#[derive(Debug, Clone, PartialEq)]"));
        self.builder.writeln(&format!("pub enum {} {{", state_name));
        self.builder.indent();
        
        if self.states.is_empty() {
            // Default state for testing
            self.builder.writeln("Initial,");
        } else {
            for state in &self.states {
                self.builder.writeln(&format!("{},", self.to_pascal_case(state)));
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn generate_context_struct(&mut self) {
        let context_name = &self.rust_config.code.context_struct_name.clone();
        
        self.builder.writeln(&format!("#[derive(Debug)]"));
        self.builder.writeln(&format!("pub struct {} {{", context_name));
        self.builder.indent();
        
        if self.domain_variables.is_empty() {
            self.builder.writeln("// TODO: Add domain variables");
            self.builder.writeln("_placeholder: (),");
        } else {
            for (var_name, var_type) in &self.domain_variables {
                let rust_type = self.frame_type_to_rust(var_type);
                self.builder.writeln(&format!("pub {}: {},", var_name, rust_type));
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
        
        // Generate implementation with new() method
        self.builder.writeln(&format!("impl {} {{", context_name));
        self.builder.indent();
        self.builder.writeln("pub fn new() -> Self {");
        self.builder.indent();
        self.builder.writeln("Self {");
        self.builder.indent();
        
        if self.domain_variables.is_empty() {
            self.builder.writeln("_placeholder: (),");
        } else {
            for (var_name, var_type) in &self.domain_variables {
                let default_value = self.get_default_value_for_type(var_type);
                self.builder.writeln(&format!("{}: {},", var_name, default_value));
            }
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn generate_system_struct(&mut self) {
        let system_struct_name = &self.system_name.clone();
        let state_type = self.state_container_type();
        let context_type = self.context_container_type();
        
        self.builder.writeln(&format!("pub struct {} {{", system_struct_name));
        self.builder.indent();
        self.builder.writeln(&format!("current_state: {},", state_type));
        self.builder.writeln(&format!("context: {},", context_type));
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn generate_constructor(&mut self) {
        let initial_state = if !self.states.is_empty() {
            format!("State::{}", self.to_pascal_case(&self.states[0]))
        } else {
            "State::Initial".to_string()
        };
        
        self.builder.writeln("pub fn new() -> Self {");
        self.builder.indent();
        
        if self.rust_config.features.thread_safe {
            self.builder.writeln(&format!("let current_state = Arc::new(Mutex::new({}));", initial_state));
            self.builder.writeln("let context = Arc::new(Mutex::new(Context::new()));");
        } else {
            self.builder.writeln(&format!("let current_state = Rc::new(RefCell::new({}));", initial_state));
            self.builder.writeln("let context = Rc::new(RefCell::new(Context::new()));");
        }
        
        self.builder.writeln("Self {");
        self.builder.indent();
        self.builder.writeln("current_state,");
        self.builder.writeln("context,");
        self.builder.dedent();
        self.builder.writeln("}");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    // Generated Utility Methods (extracted from manual implementation)
    fn to_pascal_case(&self, s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }
    
    fn to_snake_case(&self, input: &str) -> String {
        input.chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    vec!['_', c.to_lowercase().next().unwrap()]
                } else {
                    vec![c.to_lowercase().next().unwrap()]
                }
            })
            .collect()
    }
    
    fn get_default_value_for_type(&self, frame_type: &str) -> String {
        match frame_type {
            "int" | "i32" | "i64" => "0".to_string(),
            "float" | "f64" | "f32" => "0.0".to_string(),
            "string" | "String" => "String::new()".to_string(),
            "bool" => "false".to_string(),
            _ => {
                if frame_type.starts_with("List<") {
                    "Vec::new()".to_string()
                } else if frame_type.starts_with("Dict<") {
                    "HashMap::new()".to_string()
                } else {
                    "Default::default()".to_string()
                }
            }
        }
    }
    
    fn collect_domain_variables(&mut self, domain_block: &DomainBlockNode) {
        for var_decl_rcref in &domain_block.member_variables {
            let var_decl = var_decl_rcref.borrow();
            if let Some(type_node) = &var_decl.type_opt {
                let var_type = type_node.get_type_str();
                self.domain_variables.insert(var_decl.name.clone(), var_type);
            } else {
                // Default to string if no type specified
                self.domain_variables.insert(var_decl.name.clone(), "string".to_string());
            }
        }
    }
    
    fn collect_interface_methods(&mut self, interface_block: &InterfaceBlockNode) {
        for method_rcref in &interface_block.interface_methods {
            let method_node = method_rcref.borrow();
            
            let mut parameters = Vec::new();
            if let Some(param_nodes) = &method_node.params {
                for param in param_nodes {
                    let param_type = if let Some(type_node) = &param.param_type_opt {
                        type_node.get_type_str()
                    } else {
                        "()".to_string()
                    };
                    parameters.push((param.param_name.clone(), param_type));
                }
            }
            
            let return_type = if let Some(return_type_node) = &method_node.return_type_opt {
                Some(return_type_node.get_type_str())
            } else {
                None
            };
            
            let signature = InterfaceMethodSignature {
                name: method_node.name.clone(),
                parameters,
                return_type,
            };
            
            self.interface_methods.insert(method_node.name.clone(), signature);
        }
    }
    
    fn generate_event_dispatch_method(&mut self) {
        let event_type = &self.rust_config.code.frame_event_type_name.clone();
        
        self.builder.writeln(&format!("pub fn dispatch_event(&mut self, event: {}) {{", event_type));
        self.builder.indent();
        
        // Get current state and dispatch to appropriate handler
        if self.rust_config.features.thread_safe {
            self.builder.writeln("let current_state = self.current_state.lock().unwrap().clone();");
        } else {
            self.builder.writeln("let current_state = self.current_state.borrow().clone();");
        }
        
        self.builder.writeln("match current_state {");
        self.builder.indent();
        
        for state_name in &self.states {
            let pascal_state = self.to_pascal_case(state_name);
            self.builder.writeln(&format!("State::{} => self.dispatch_event_to_{}(event),", pascal_state, state_name.to_lowercase()));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn generate_state_dispatch_method(&mut self, state_name: &str) {
        let event_type = &self.rust_config.code.frame_event_type_name.clone();
        
        self.builder.writeln(&format!("fn dispatch_event_to_{}(&mut self, event: {}) {{", state_name.to_lowercase(), event_type));
        self.builder.indent();
        
        self.builder.writeln("match event {");
        self.builder.indent();
        
        // Generate handlers for each interface method
        for (method_name, _signature) in &self.interface_methods.clone() {
            let pascal_method = self.to_pascal_case(method_name);
            let handler_name = format!("handle_{}_{}", state_name.to_lowercase(), method_name.to_lowercase());
            
            self.builder.writeln(&format!("{}::{} {{ .. }} => self.{}(),", event_type, pascal_method, handler_name));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn generate_main_function(&mut self) {
        self.builder.writeln("");
        self.builder.writeln("fn main() {");
        self.builder.indent();
        
        // Create an instance of the system
        self.builder.writeln(&format!("let mut system = {}::new();", self.system_name));
        
        // Look for a 'test' interface method and call it if it exists
        if self.interface_methods.contains_key("test") {
            self.builder.writeln("system.test();");
        } else {
            // If no test method, just print that the system was created
            self.builder.writeln(&format!("println!(\"System {} created and initialized.\");", self.system_name));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
    
    // Generated method implementations (extracted patterns)
    fn generate_interface_method_impl(&mut self, method: &InterfaceMethodNode) {
        let method_name = &method.name;
        
        // Build parameter list
        let mut params = Vec::new();
        if let Some(param_nodes) = &method.params {
            for param in param_nodes {
                let param_type = if let Some(type_node) = &param.param_type_opt {
                    self.frame_type_to_rust(&type_node.get_type_str())
                } else {
                    "()".to_string()
                };
                params.push(format!("{}: {}", param.param_name, param_type));
            }
        }
        let params_str = params.join(", ");
        
        // Determine return type
        let return_type = if let Some(return_type_node) = &method.return_type_opt {
            self.frame_type_to_rust(&return_type_node.get_type_str())
        } else {
            "()".to_string()
        };
        
        // Generate method signature
        self.builder.writeln(&format!("pub fn {}(&mut self{}) -> {} {{", 
            method_name,
            if params_str.is_empty() { "".to_string() } else { format!(", {}", params_str) },
            return_type
        ));
        self.builder.indent();
        
        // Create event and dispatch to state machine
        let event_type = &self.rust_config.code.frame_event_type_name.clone();
        let pascal_method = self.to_pascal_case(method_name);
        
        if params.is_empty() {
            self.builder.writeln(&format!("let event = {}::{};", event_type, pascal_method));
        } else {
            self.builder.writeln(&format!("let event = {}::{} {{", event_type, pascal_method));
            self.builder.indent();
            
            // Add parameters to event
            if let Some(param_nodes) = &method.params {
                for param in param_nodes {
                    self.builder.writeln(&format!("{},", param.param_name));
                }
            }
            
            self.builder.dedent();
            self.builder.writeln("};");
        }
        
        self.builder.writeln("self.dispatch_event(event);");
        
        // For now, provide a basic default return for non-void methods
        if return_type != "()" {
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder.writeln(&format!("{}", default_value));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
}

// Generated AstVisitor Implementation (using established patterns)
impl AstVisitor for RustVisitor {
    fn visit_system_node(&mut self, node: &SystemNode) {
        self.system_name = node.name.clone();
        self.current_class_name_opt = Some(node.name.clone());
        
        // Generate file header and imports
        self.generate_imports();
        
        // First pass: collect metadata
        if let Some(domain) = &node.domain_block_node_opt {
            self.collect_domain_variables(domain);
        }
        
        if let Some(machine) = &node.machine_block_node_opt {
            // Collect states
            for state_rcref in &machine.states {
                let state_node = state_rcref.borrow();
                self.states.push(state_node.name.clone());
            }
        }
        
        // Collect operations and actions signatures for call resolution
        if let Some(operations) = &node.operations_block_node_opt {
            self.collect_operation_signatures(operations);
        }
        if let Some(actions) = &node.actions_block_node_opt {
            self.collect_action_signatures(actions);
        }
        
        // Collect interface methods for main function generation
        if let Some(interface) = &node.interface_block_node_opt {
            self.collect_interface_methods(interface);
        }
        
        // Generate type definitions
        self.generate_frame_event_enum();
        self.generate_state_enum();
        self.generate_context_struct();
        self.generate_system_struct();
        
        // Generate implementation
        self.builder.writeln(&format!("impl {} {{", self.system_name));
        self.builder.indent();
        
        // Constructor
        self.generate_constructor();
        
        // Interface methods
        if let Some(interface) = &node.interface_block_node_opt {
            self.visit_interface_block_node(interface);
        }
        
        // Machine block - state machine logic
        if let Some(machine) = &node.machine_block_node_opt {
            self.visit_machine_block_node(machine);
        }
        
        // Actions block
        if let Some(actions) = &node.actions_block_node_opt {
            self.visit_actions_block_node(actions);
        }
        
        // Operations block  
        if let Some(operations) = &node.operations_block_node_opt {
            self.visit_operations_block_node(operations);
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        
        // Generate main function for executable programs
        self.generate_main_function();
    }
    
    fn visit_interface_block_node(&mut self, interface_block: &InterfaceBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== Interface Methods ==================== //");
        self.builder.writeln("");
        
        for method_rcref in &interface_block.interface_methods {
            let method_node = method_rcref.borrow();
            self.generate_interface_method_impl(&method_node);
        }
    }
    
    fn visit_machine_block_node(&mut self, machine_block: &MachineBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== Event Handler Methods ==================== //");
        self.builder.writeln("");
        
        // Process states and generate event handlers
        for state_rcref in &machine_block.states {
            let state_node = state_rcref.borrow();
            self.current_state_name_opt = Some(state_node.name.clone());
            
            // Process each event handler in the state
            for handler_rcref in &state_node.evt_handlers_rcref {
                let handler = handler_rcref.borrow();
                self.visit_event_handler_node(&handler);
            }
        }
        
        self.current_state_name_opt = None;
        
        self.builder.writeln("// ==================== State Machine Logic ==================== //");
        self.builder.writeln("");
        
        // Generate event dispatch method
        self.generate_event_dispatch_method();
        
        // Generate state-specific dispatch methods
        for state_name in &self.states.clone() {
            self.generate_state_dispatch_method(state_name);
        }
    }
    
    fn visit_actions_block_node(&mut self, actions_block: &ActionsBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== Actions ==================== //");
        self.builder.writeln("");
        
        for action_rcref in &actions_block.actions {
            let action_node = action_rcref.borrow();
            self.visit_action_node(&action_node);
        }
    }
    
    fn visit_operations_block_node(&mut self, operations_block: &OperationsBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== Operations ==================== //");
        self.builder.writeln("");
        
        for operation_rcref in &operations_block.operations {
            let operation_node = operation_rcref.borrow();
            self.visit_operation_node(&operation_node);
        }
    }
    
    // Stub implementations for remaining required methods
    fn visit_interface_method_node(&mut self, _method: &InterfaceMethodNode) {
        // Implementation handled in visit_interface_block_node
    }
    
    fn visit_action_node(&mut self, action: &ActionNode) {
        // Build parameter list
        let mut params = Vec::new();
        if let Some(param_nodes) = &action.params {
            for param in param_nodes {
                let param_type = if let Some(type_node) = &param.param_type_opt {
                    self.frame_type_to_rust(&type_node.get_type_str())
                } else {
                    "()".to_string()
                };
                params.push(format!("{}: {}", param.param_name, param_type));
            }
        }
        let params_str = params.join(", ");
        
        // Generate action method
        self.builder.writeln(&format!("fn {}(&mut self{}) {{", 
            action.name,
            if params_str.is_empty() { "".to_string() } else { format!(", {}", params_str) }
        ));
        self.builder.indent();
        
        // Process statements in the action
        if !action.statements.is_empty() {
            let statements_code = self.process_statements(&action.statements);
            self.builder.write(&statements_code);
        } else {
            self.builder.writeln(&format!("// Empty action {}", action.name));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn visit_operation_node(&mut self, operation: &OperationNode) {
        // Build parameter list
        let mut params = Vec::new();
        if let Some(param_nodes) = &operation.params {
            for param in param_nodes {
                let param_type = if let Some(type_node) = &param.param_type_opt {
                    self.frame_type_to_rust(&type_node.get_type_str())
                } else {
                    "()".to_string()
                };
                params.push(format!("{}: {}", param.param_name, param_type));
            }
        }
        let params_str = params.join(", ");
        
        // Determine return type
        let return_type = if let Some(return_type_node) = &operation.type_opt {
            self.frame_type_to_rust(&return_type_node.get_type_str())
        } else {
            "()".to_string()
        };
        
        // Generate operation method
        self.builder.writeln(&format!("pub fn {}(&mut self{}) -> {} {{", 
            operation.name,
            if params_str.is_empty() { "".to_string() } else { format!(", {}", params_str) },
            return_type
        ));
        self.builder.indent();
        
        // Process statements in the operation
        if !operation.statements.is_empty() {
            let statements_code = self.process_statements(&operation.statements);
            self.builder.write(&statements_code);
        } else {
            self.builder.writeln("// Empty operation");
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
}

// Additional implementation methods for statement processing
impl RustVisitor {
    // Process statements from AST to generate Rust code
    fn process_statements(&self, statements: &Vec<DeclOrStmtType>) -> String {
        let mut code = String::new();
        
        for stmt in statements {
            match stmt {
                DeclOrStmtType::StmtT { stmt_t } => {
                    match stmt_t {
                        StatementType::ExpressionStmt { expr_stmt_t } => {
                            match expr_stmt_t {
                                ExprStmtType::CallStmtT { call_stmt_node } => {
                                    let call_code = self.process_call_statement(call_stmt_node);
                                    if !call_code.is_empty() {
                                        code.push_str(&format!("        {};\n", call_code));
                                    }
                                }
                                ExprStmtType::ActionCallStmtT { action_call_stmt_node } => {
                                    // Handle action calls - treat like function calls
                                    let action_name = &action_call_stmt_node.action_call_expr_node.identifier.name.lexeme;
                                    code.push_str(&format!("        self.{}();\n", action_name));
                                }
                                ExprStmtType::CallChainStmtT { call_chain_literal_stmt_node } => {
                                    // Handle call chains - often print statements
                                    let call_code = self.process_call_chain_statement(call_chain_literal_stmt_node);
                                    if !call_code.is_empty() {
                                        code.push_str(&format!("        {};\n", call_code));
                                    }
                                }
                                ExprStmtType::SystemInstanceStmtT { .. } => {
                                    code.push_str("        // System instance statement\n");
                                }
                                ExprStmtType::SystemTypeStmtT { .. } => {
                                    code.push_str("        // System type statement\n");
                                }
                                ExprStmtType::AssignmentStmtT { assignment_stmt_node } => {
                                    // Handle assignment statements
                                    let target = self.process_expression(&assignment_stmt_node.assignment_expr_node.l_value_box);
                                    let value = self.process_expression(&assignment_stmt_node.assignment_expr_node.r_value_rc);
                                    code.push_str(&format!("        {} = {};\n", target, value));
                                }
                                _ => {
                                    code.push_str("        // TODO: Implement expression statement type\n");
                                }
                            }
                        }
                        StatementType::ReturnStmt { return_stmt_node } => {
                            // Handle return statements with optional value
                            if let Some(return_expr) = &return_stmt_node.expr_t_opt {
                                let return_value = self.process_expression(return_expr);
                                code.push_str(&format!("        return {};\n", return_value));
                            } else {
                                code.push_str("        return;\n");
                            }
                        }
                        StatementType::TransitionStmt { transition_statement_node: _ } => {
                            // Handle state transitions - simplified for now
                            code.push_str("        // State transition - TODO: implement target state extraction\n");
                            code.push_str("        // *self.current_state.borrow_mut() = State::TargetState;\n");
                        }
                        StatementType::IfStmt { if_stmt_node } => {
                            // Handle conditional statements (if/else)
                            let condition = self.process_expression(&if_stmt_node.condition);
                            code.push_str(&format!("        if {} {{\n", condition));
                            
                            // Process if block
                            let if_code = self.process_statements(&if_stmt_node.if_block.statements);
                            code.push_str(&if_code);
                            
                            // Process elif clauses
                            for elif_clause in &if_stmt_node.elif_clauses {
                                let elif_condition = self.process_expression(&elif_clause.condition);
                                code.push_str(&format!("        }} else if {} {{\n", elif_condition));
                                let elif_code = self.process_statements(&elif_clause.block.statements);
                                code.push_str(&elif_code);
                            }
                            
                            // Process else block if it exists
                            if let Some(else_block) = &if_stmt_node.else_block {
                                code.push_str("        } else {\n");
                                let else_code = self.process_statements(&else_block.statements);
                                code.push_str(&else_code);
                            }
                            
                            code.push_str("        }\n");
                        }
                        StatementType::ForStmt { for_stmt_node } => {
                            // Handle for loops
                            let iterator = if let Some(var_node) = &for_stmt_node.variable {
                                &var_node.id_node.name.lexeme
                            } else if let Some(id_node) = &for_stmt_node.identifier {
                                &id_node.name.lexeme
                            } else {
                                "item"
                            };
                            let iterable = self.process_expression(&for_stmt_node.iterable);
                            code.push_str(&format!("        for {} in {} {{\n", iterator, iterable));
                            
                            // Process loop body
                            let loop_code = self.process_statements(&for_stmt_node.block.statements);
                            code.push_str(&loop_code);
                            
                            // Process else block if it exists
                            if let Some(else_block) = &for_stmt_node.else_block {
                                code.push_str("        } else {\n");
                                let else_code = self.process_statements(&else_block.statements);
                                code.push_str(&else_code);
                            }
                            
                            code.push_str("        }\n");
                        }
                        StatementType::WhileStmt { while_stmt_node } => {
                            // Handle while loops
                            let condition = self.process_expression(&while_stmt_node.condition);
                            code.push_str(&format!("        while {} {{\n", condition));
                            
                            // Process loop body
                            let loop_code = self.process_statements(&while_stmt_node.block.statements);
                            code.push_str(&loop_code);
                            
                            // Process else block if it exists
                            if let Some(else_block) = &while_stmt_node.else_block {
                                code.push_str("        } else {\n");
                                let else_code = self.process_statements(&else_block.statements);
                                code.push_str(&else_code);
                            }
                            
                            code.push_str("        }\n");
                        }
                        StatementType::LoopStmt { loop_stmt_node } => {
                            // Handle different types of loops
                            match &loop_stmt_node.loop_types {
                                LoopStmtTypes::LoopInfiniteStmt { loop_infinite_stmt_node } => {
                                    code.push_str("        loop {\n");
                                    if !loop_infinite_stmt_node.statements.is_empty() {
                                        let loop_code = self.process_statements(&loop_infinite_stmt_node.statements);
                                        code.push_str(&loop_code);
                                    }
                                    code.push_str("        }\n");
                                }
                                LoopStmtTypes::LoopForStmt { loop_for_stmt_node } => {
                                    // C-style for loop: for (init; test; post)
                                    code.push_str("        // C-style for loop converted to while loop\n");
                                    
                                    // Initialize variables if present
                                    if let Some(init_expr) = &loop_for_stmt_node.loop_init_expr_rcref_opt {
                                        // TODO: Process initialization expression
                                        code.push_str("        // TODO: Process init expression\n");
                                    }
                                    
                                    // Create while loop with test condition
                                    if let Some(test_expr) = &loop_for_stmt_node.test_expr_rcref_opt {
                                        let condition = self.process_expression(&*test_expr.borrow());
                                        code.push_str(&format!("        while {} {{\n", condition));
                                    } else {
                                        code.push_str("        loop {\n");
                                    }
                                    
                                    // Process loop body
                                    let loop_code = self.process_statements(&loop_for_stmt_node.statements);
                                    code.push_str(&loop_code);
                                    
                                    // Add post expression if present
                                    if let Some(post_expr) = &loop_for_stmt_node.post_expr_rcref_opt {
                                        let post_code = self.process_expression(&*post_expr.borrow());
                                        code.push_str(&format!("            {};\n", post_code));
                                    }
                                    
                                    code.push_str("        }\n");
                                }
                                _ => {
                                    code.push_str("        // TODO: Implement other loop types\n");
                                }
                            }
                        }
                        StatementType::ContinueStmt { .. } => {
                            code.push_str("        continue;\n");
                        }
                        StatementType::BreakStmt { .. } => {
                            code.push_str("        break;\n");
                        }
                        _ => {
                            code.push_str("        // TODO: Implement statement type\n");
                        }
                    }
                }
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    // Handle variable declarations
                    let var_decl = var_decl_t_rcref.borrow();
                    // For now, treat all variable declarations as local variables
                    {
                        let var_name = &var_decl.name;
                        // Use the initializer_value_rc for the variable initialization
                        let init_value = self.process_expression(&var_decl.initializer_value_rc);
                        
                        if let Some(type_node) = &var_decl.type_opt {
                            let var_type = self.frame_type_to_rust(&type_node.get_type_str());
                            code.push_str(&format!("        let mut {}: {} = {};\n", var_name, var_type, init_value));
                        } else {
                            // No type specified - infer from the initializer
                            code.push_str(&format!("        let mut {} = {};\n", var_name, init_value));
                        }
                    }
                }
            }
        }
        
        code
    }
    
    // Process a call statement (function call, operation call, etc.)
    fn process_call_statement(&self, call_stmt: &CallStmtNode) -> String {
        let func_name = &call_stmt.call_expr_node.identifier.name.lexeme;
        
        if func_name == "print" {
            // Process print arguments
            let args = self.process_call_arguments(&call_stmt.call_expr_node.call_expr_list);
            if args.is_empty() {
                "println!()".to_string()
            } else {
                format!("println!(\"{{}}\", {})", args)
            }
        } else {
            // Handle operation calls with arguments
            let args = self.process_call_arguments(&call_stmt.call_expr_node.call_expr_list);
            if args.is_empty() {
                format!("self.{}()", func_name)
            } else {
                format!("self.{}({})", func_name, args)
            }
        }
    }
    
    // Process a call chain statement (like print calls)
    fn process_call_chain_statement(&self, call_chain_stmt: &CallChainStmtNode) -> String {
        // Get the first call in the chain (usually the function name)
        if let Some(first_call) = call_chain_stmt.call_chain_literal_expr_node.call_chain.front() {
            match first_call {
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    let func_name = &call_node.identifier.name.lexeme;
                    if func_name == "print" {
                        // Process print arguments from the call
                        let args = self.process_call_arguments(&call_node.call_expr_list);
                        if args.is_empty() {
                            "println!()".to_string()
                        } else {
                            format!("println!(\"{{}}\", {})", args)
                        }
                    } else {
                        format!("self.{}()", func_name)
                    }
                }
                CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => {
                    let method_name = &interface_method_call_expr_node.identifier.name.lexeme;
                    let args = self.process_call_arguments(&interface_method_call_expr_node.call_expr_list);
                    if args.is_empty() {
                        format!("self.{}()", method_name)
                    } else {
                        format!("self.{}({})", method_name, args)
                    }
                }
                CallChainNodeType::OperationCallT { operation_call_expr_node } => {
                    let operation_name = &operation_call_expr_node.identifier.name.lexeme;
                    let args = self.process_call_arguments(&operation_call_expr_node.call_expr_list);
                    if args.is_empty() {
                        format!("self.{}()", operation_name)
                    } else {
                        format!("self.{}({})", operation_name, args)
                    }
                }
                CallChainNodeType::ActionCallT { action_call_expr_node } => {
                    let action_name = &action_call_expr_node.identifier.name.lexeme;
                    let args = self.process_call_arguments(&action_call_expr_node.call_expr_list);
                    if args.is_empty() {
                        format!("self.{}()", action_name)
                    } else {
                        format!("self.{}({})", action_name, args)
                    }
                }
                _ => {
                    "// TODO: Handle other call chain types".to_string()
                }
            }
        } else {
            "// Empty call chain".to_string()
        }
    }
    
    // Process call arguments for function calls
    fn process_call_arguments(&self, call_expr_list: &CallExprListNode) -> String {
        if !call_expr_list.exprs_t.is_empty() {
            let mut args = Vec::new();
            for expr in &call_expr_list.exprs_t {
                let arg = self.process_expression(expr);
                args.push(arg);
            }
            args.join(", ")  // Join with commas for function arguments
        } else {
            String::new()
        }
    }
    
    // Process an expression to generate Rust code
    fn process_expression(&self, expr: &ExprType) -> String {
        match expr {
            ExprType::LiteralExprT { literal_expr_node } => {
                // Handle literal values - use token type to determine formatting
                let value = &literal_expr_node.value;
                match literal_expr_node.token_t {
                    crate::frame_c::scanner::TokenType::String => {
                        // String literal - add quotes if not present
                        if value.starts_with('"') && value.ends_with('"') {
                            value.clone()
                        } else {
                            format!("\"{}\"", value)
                        }
                    }
                    _ => {
                        // Other literals (numbers, booleans, etc.)
                        value.clone()
                    }
                }
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                let left = self.process_expression(&binary_expr_node.left_rcref.borrow());
                let right = self.process_expression(&binary_expr_node.right_rcref.borrow());
                let operator = match &binary_expr_node.operator {
                    OperatorType::Plus => "+",
                    OperatorType::Minus => "-",
                    OperatorType::Multiply => "*",
                    OperatorType::Divide => "/",
                    OperatorType::EqualEqual => "==",
                    OperatorType::NotEqual => "!=",
                    OperatorType::Less => "<",
                    OperatorType::LessEqual => "<=",
                    OperatorType::Greater => ">",
                    OperatorType::GreaterEqual => ">=",
                    OperatorType::LogicalAnd => "&&",
                    OperatorType::LogicalOr => "||",
                    OperatorType::BitwiseAnd => "&",
                    OperatorType::BitwiseOr => "|",
                    OperatorType::BitwiseXor => "^",
                    OperatorType::LeftShift => "<<",
                    OperatorType::RightShift => ">>",
                    OperatorType::Percent => "%",
                    _ => "/* unknown_op */", // Unknown operator
                };
                
                // For simple operations like addition chains, we don't need excessive parentheses
                // Rust handles operator precedence correctly
                match &binary_expr_node.operator {
                    OperatorType::Plus | OperatorType::Minus => {
                        // For addition/subtraction chains, only add parentheses if needed for precedence
                        format!("{} {} {}", left, operator, right)
                    }
                    OperatorType::Multiply | OperatorType::Divide | OperatorType::Percent => {
                        // For multiplication/division, only add parentheses if needed
                        format!("{} {} {}", left, operator, right)
                    }
                    _ => {
                        // For comparison and logical operators, use parentheses for clarity
                        format!("({} {} {})", left, operator, right)
                    }
                }
            }
            ExprType::VariableExprT { var_node } => {
                // Variable reference - access through context if it's a domain variable
                let var_name = &var_node.id_node.name.lexeme;
                if self.domain_variables.contains_key(var_name) {
                    // Domain variable - access through context
                    if self.rust_config.features.thread_safe {
                        format!("self.context.lock().unwrap().{}", var_name)
                    } else {
                        format!("self.context.borrow().{}", var_name)
                    }
                } else {
                    // Local variable or parameter - use directly
                    var_name.clone()
                }
            }
            ExprType::CallExprT { call_expr_node } => {
                // Function call expression
                let func_name = &call_expr_node.identifier.name.lexeme;
                let args = self.process_call_arguments(&call_expr_node.call_expr_list);
                
                // Check if this is a call to an operation or action within the same system
                let is_operation = self.operation_signatures.contains_key(func_name);
                let is_action = self.action_signatures.contains_key(func_name);
                
                let call_target = if is_operation || is_action {
                    format!("self.{}", func_name)
                } else {
                    func_name.to_string()
                };
                
                if args.is_empty() {
                    format!("{}()", call_target)
                } else {
                    format!("{}({})", call_target, args)
                }
            }
            ExprType::UnaryExprT { unary_expr_node } => {
                let operand = self.process_expression(&unary_expr_node.right_rcref.borrow());
                match &unary_expr_node.operator {
                    OperatorType::Not => format!("!({})", operand),
                    OperatorType::Minus => format!("-({})", operand),
                    OperatorType::Plus => format!("+({})", operand),
                    OperatorType::Negated => format!("!({})", operand),
                    _ => format!("/* unary_unknown */ {}", operand),
                }
            }
            ExprType::CallChainExprT { call_chain_expr_node } => {
                // Handle call chain expressions - often variable references
                if let Some(first_node) = call_chain_expr_node.call_chain.front() {
                    match first_node {
                        CallChainNodeType::VariableNodeT { var_node } => {
                            // Variable node reference - access through context
                            let var_name = &var_node.id_node.name.lexeme;
                            if self.domain_variables.contains_key(var_name) {
                                // Domain variable - access through context
                                if self.rust_config.features.thread_safe {
                                    format!("self.context.lock().unwrap().{}", var_name)
                                } else {
                                    format!("self.context.borrow().{}", var_name)
                                }
                            } else {
                                // Local variable or parameter - use directly
                                var_name.clone()
                            }
                        }
                        CallChainNodeType::UndeclaredCallT { call_node } => {
                            // Function call within expression
                            let func_name = &call_node.identifier.name.lexeme;
                            let args = self.process_call_arguments(&call_node.call_expr_list);
                            if args.is_empty() {
                                format!("{}()", func_name)
                            } else {
                                format!("{}({})", func_name, args)
                            }
                        }
                        CallChainNodeType::SelfT { self_expr_node: _ } => {
                            // Self reference
                            "self".to_string()
                        }
                        CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                            // Simple identifier (variable, parameter, etc.)
                            let var_name = &id_node.name.lexeme;
                            if self.domain_variables.contains_key(var_name) {
                                // Domain variable - access through context
                                if self.rust_config.features.thread_safe {
                                    format!("self.context.lock().unwrap().{}", var_name)
                                } else {
                                    format!("self.context.borrow().{}", var_name)
                                }
                            } else {
                                // Local variable or parameter - use directly
                                var_name.clone()
                            }
                        }
                        _ => {
                            "/* TODO: other call chain node type */".to_string()
                        }
                    }
                } else {
                    "/* empty call chain */".to_string()
                }
            }
            _ => {
                "/* TODO: unhandled expression type */".to_string()
            }
        }
    }
    
    fn visit_event_handler_node(&mut self, handler: &EventHandlerNode) {
        let state_name = self.current_state_name_opt.as_ref().unwrap().clone();
        
        // Get event name from the message type
        let event_name = match &handler.msg_t {
            MessageType::CustomMessage { message_node } => {
                message_node.name.clone()
            }
            MessageType::None => {
                "unknown".to_string()
            }
        };
        
        // Generate handler method name
        let handler_method_name = if event_name == "$>" {
            format!("handle_{}_enter", state_name.to_lowercase())
        } else if event_name == "<$" {
            format!("handle_{}_exit", state_name.to_lowercase())
        } else {
            format!("handle_{}_{}", state_name.to_lowercase(), event_name.to_lowercase())
        };
        
        // Generate the handler method
        self.builder.writeln(&format!("fn {}(&mut self) {{", handler_method_name));
        self.builder.indent();
        
        // Process statements in the handler
        if !handler.statements.is_empty() {
            let statements_code = self.process_statements(&handler.statements);
            self.builder.write(&statements_code);
        } else {
            self.builder.writeln(&format!("// Empty {} handler for state {}", event_name, state_name));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn collect_operation_signatures(&mut self, operations_block: &OperationsBlockNode) {
        for operation_rcref in &operations_block.operations {
            let operation = operation_rcref.borrow();
            let operation_sig = OperationSignature {
                name: operation.name.clone(),
                parameters: if let Some(param_nodes) = &operation.params {
                    param_nodes.iter().map(|p| {
                        let param_type = if let Some(type_node) = &p.param_type_opt {
                            type_node.get_type_str()
                        } else {
                            "()".to_string()
                        };
                        (p.param_name.clone(), param_type)
                    }).collect()
                } else {
                    Vec::new()
                },
                return_type: if let Some(return_type_node) = &operation.type_opt {
                    Some(return_type_node.get_type_str())
                } else {
                    Some("()".to_string())
                },
            };
            self.operation_signatures.insert(operation.name.clone(), operation_sig);
        }
    }
    
    fn collect_action_signatures(&mut self, actions_block: &ActionsBlockNode) {
        for action_rcref in &actions_block.actions {
            let action = action_rcref.borrow();
            let action_sig = ActionSignature {
                name: action.name.clone(),
                parameters: if let Some(param_nodes) = &action.params {
                    param_nodes.iter().map(|p| {
                        let param_type = if let Some(type_node) = &p.param_type_opt {
                            type_node.get_type_str()
                        } else {
                            "()".to_string()
                        };
                        (p.param_name.clone(), param_type)
                    }).collect()
                } else {
                    Vec::new()
                },
            };
            self.action_signatures.insert(action.name.clone(), action_sig);
        }
    }
}
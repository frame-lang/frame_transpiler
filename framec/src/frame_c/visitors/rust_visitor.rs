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
        
        // Basic implementation - create event and send to state machine
        self.builder.writeln("// TODO: Implement proper event creation and state machine dispatch");
        
        // For now, provide a basic default return
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
    
    fn visit_machine_block_node(&mut self, _machine_block: &MachineBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== State Machine Logic ==================== //");
        self.builder.writeln("");
        self.builder.writeln("// TODO: Implement state machine dispatch logic");
        self.builder.writeln("");
    }
    
    fn visit_actions_block_node(&mut self, _actions_block: &ActionsBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== Actions ==================== //");
        self.builder.writeln("");
        self.builder.writeln("// TODO: Generate action methods");
        self.builder.writeln("");
    }
    
    fn visit_operations_block_node(&mut self, _operations_block: &OperationsBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== Operations ==================== //");
        self.builder.writeln("");
        self.builder.writeln("// TODO: Generate operation methods");
        self.builder.writeln("");
    }
    
    // Stub implementations for remaining required methods
    fn visit_interface_method_node(&mut self, _method: &InterfaceMethodNode) {
        // Implementation handled in visit_interface_block_node
    }
    
    fn visit_action_node(&mut self, _action: &ActionNode) {
        // Implementation handled in visit_actions_block_node
    }
    
    fn visit_operation_node(&mut self, _operation: &OperationNode) {
        // Implementation handled in visit_operations_block_node
    }
}
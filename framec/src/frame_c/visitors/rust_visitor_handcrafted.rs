// Hand-crafted Rust Visitor - Implements actual Frame state machine semantics
// Based on Python visitor patterns but adapted for Rust's ownership system

use crate::frame_c::ast::*;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::config::FrameConfig;
use crate::frame_c::scanner::Token;
use crate::frame_c::symbol_table::{SymbolConfig, Arcanum};
use crate::frame_c::visitors::AstVisitor;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RustConfig {
    pub generate_debug: bool,
    pub use_async: bool,
}

impl Default for RustConfig {
    fn default() -> Self {
        Self {
            generate_debug: false,
            use_async: false,
        }
    }
}

pub struct RustVisitorHandcrafted {
    // Core configuration
    config: FrameConfig,
    rust_config: RustConfig,
    
    // Code generation
    builder: CodeBuilder,
    
    // Symbol tracking
    symbol_config: SymbolConfig,
    arcanum: Vec<Arcanum>,
    
    // Current context tracking
    current_state_name_opt: Option<String>,
    current_event_ret_type: String,
    current_class_name_opt: Option<String>,
    
    // System metadata
    system_name: String,
    interface_methods: HashMap<String, (Vec<String>, Option<String>)>, // name -> (params, return_type)
    domain_variables: HashMap<String, String>, // name -> type
    states: Vec<String>,
    state_handlers: HashMap<String, Vec<String>>, // state -> handler names
    actions: HashMap<String, Vec<String>>, // action name -> param names
    operations: HashMap<String, (Vec<String>, Option<String>)>, // name -> (params, return_type)
    
    // Current handler context
    current_state_handlers: HashMap<String, String>, // event_name -> handler_method_name
    current_handler_params: Vec<String>,
    
    // Comments (for debugging)
    _comments: Vec<Token>,
}

impl RustVisitorHandcrafted {
    pub fn new(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
        config: FrameConfig,
        comments: Vec<Token>,
    ) -> Self {
        Self {
            config,
            rust_config: RustConfig::default(),
            builder: CodeBuilder::new("    "), // 4-space indent
            symbol_config,
            arcanum,
            current_state_name_opt: None,
            current_event_ret_type: String::new(),
            current_class_name_opt: None,
            system_name: String::new(),
            interface_methods: HashMap::new(),
            domain_variables: HashMap::new(),
            states: Vec::new(),
            state_handlers: HashMap::new(),
            actions: HashMap::new(),
            operations: HashMap::new(),
            current_state_handlers: HashMap::new(),
            current_handler_params: Vec::new(),
            _comments: comments,
        }
    }
    
    pub fn run(mut self, frame_module: &FrameModule) -> String {
        // Visit the module and generate Rust code
        for system in &frame_module.systems {
            system.accept(&mut self);
        }
        
        self.builder.build().0  // Return just the code string
    }
    
    // Frame type to Rust type mapping
    fn frame_type_to_rust(&self, frame_type: &str) -> String {
        match frame_type {
            "int" => "i32".to_string(),
            "float" => "f64".to_string(),
            "string" => "String".to_string(),
            "bool" => "bool".to_string(),
            "void" => "()".to_string(),
            _ => {
                if frame_type.starts_with("List<") && frame_type.ends_with(">") {
                    let inner = &frame_type[5..frame_type.len()-1];
                    format!("Vec<{}>", self.frame_type_to_rust(inner))
                } else if frame_type.starts_with("Dict<") && frame_type.ends_with(">") {
                    let inner = &frame_type[5..frame_type.len()-1];
                    if let Some(comma_pos) = inner.find(',') {
                        let key_type = inner[..comma_pos].trim();
                        let value_type = inner[comma_pos+1..].trim();
                        format!("std::collections::HashMap<{}, {}>", 
                            self.frame_type_to_rust(key_type),
                            self.frame_type_to_rust(value_type))
                    } else {
                        "std::collections::HashMap<String, String>".to_string()
                    }
                } else {
                    frame_type.to_string()
                }
            }
        }
    }
    
    fn get_default_value_for_type(&self, rust_type: &str) -> String {
        match rust_type {
            "i32" | "i64" => "0".to_string(),
            "f64" | "f32" => "0.0".to_string(),
            "String" => "String::new()".to_string(),
            "bool" => "false".to_string(),
            "()" => "()".to_string(),
            _ => {
                if rust_type.starts_with("Vec<") {
                    "Vec::new()".to_string()
                } else if rust_type.starts_with("std::collections::HashMap<") {
                    "std::collections::HashMap::new()".to_string()
                } else {
                    "Default::default()".to_string()
                }
            }
        }
    }
    
    // Generate the Rust Frame runtime structures
    fn generate_frame_runtime(&mut self) {
        self.builder.writeln("use std::collections::HashMap;");
        self.builder.writeln("");
        
        // Generate FrameEvent struct
        self.builder.writeln("#[derive(Debug, Clone)]");
        self.builder.writeln("pub struct FrameEvent {");
        self.builder.indent();
        self.builder.writeln("pub message: String,");
        self.builder.writeln("pub parameters: Option<HashMap<String, String>>,");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
        
        self.builder.writeln("impl FrameEvent {");
        self.builder.indent();
        self.builder.writeln("pub fn new(message: &str, parameters: Option<HashMap<String, String>>) -> Self {");
        self.builder.indent();
        self.builder.writeln("Self {");
        self.builder.indent();
        self.builder.writeln("message: message.to_string(),");
        self.builder.writeln("parameters,");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
        
        // Generate FrameCompartment for state management
        self.builder.writeln("#[derive(Debug, Clone)]");
        self.builder.writeln("pub struct FrameCompartment {");
        self.builder.indent();
        self.builder.writeln("pub state: String,");
        self.builder.writeln("pub state_vars: HashMap<String, String>,");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
        
        self.builder.writeln("impl FrameCompartment {");
        self.builder.indent();
        self.builder.writeln("pub fn new(state: &str) -> Self {");
        self.builder.indent();
        self.builder.writeln("Self {");
        self.builder.indent();
        self.builder.writeln("state: state.to_string(),");
        self.builder.writeln("state_vars: HashMap::new(),");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    // Generate the main system struct
    fn generate_system_struct(&mut self) {
        self.builder.writeln(&format!("pub struct {} {{", self.system_name));
        self.builder.indent();
        
        // Frame runtime state
        self.builder.writeln("compartment: FrameCompartment,");
        self.builder.writeln("return_stack: Vec<Option<String>>,");
        
        // Domain variables
        for (var_name, var_type) in &self.domain_variables {
            let rust_type = self.frame_type_to_rust(var_type);
            self.builder.writeln(&format!("pub {}: {},", var_name, rust_type));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    // Generate constructor
    fn generate_constructor(&mut self) {
        self.builder.writeln(&format!("impl {} {{", self.system_name));
        self.builder.indent();
        
        self.builder.writeln("pub fn new() -> Self {");
        self.builder.indent();
        
        // Get initial state
        let initial_state = if !self.states.is_empty() {
            format!("__{}_state_{}", self.system_name.to_lowercase(), self.states[0])
        } else {
            format!("__{}_state_initial", self.system_name.to_lowercase())
        };
        
        self.builder.writeln("let mut instance = Self {");
        self.builder.indent();
        self.builder.writeln(&format!("compartment: FrameCompartment::new(\"{}\"),", initial_state));
        self.builder.writeln("return_stack: vec![None],");
        
        // Initialize domain variables
        for (var_name, var_type) in &self.domain_variables {
            let rust_type = self.frame_type_to_rust(var_type);
            let default_value = self.get_default_value_for_type(&rust_type);
            self.builder.writeln(&format!("{}: {},", var_name, default_value));
        }
        
        self.builder.dedent();
        self.builder.writeln("};");
        self.builder.writeln("");
        
        // Send system start event
        self.builder.writeln("let start_event = FrameEvent::new(\"$>\", None);");
        self.builder.writeln("instance.frame_kernel(start_event);");
        self.builder.writeln("instance");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    // Generate interface methods that actually dispatch to the Frame kernel
    fn generate_interface_methods(&mut self) {
        for (method_name, (params, return_type_opt)) in &self.interface_methods.clone() {
            let return_type = if let Some(ret_type) = return_type_opt {
                self.frame_type_to_rust(ret_type)
            } else {
                "()".to_string()
            };
            
            // Build parameter list
            let mut param_list = Vec::new();
            let mut param_map = Vec::new();
            
            for param in params {
                param_list.push(format!("{}: {}", param, "String")); // Simplified: use String for all params
                param_map.push(format!("(\"{}\".to_string(), {}.to_string())", param, param));
            }
            
            let params_str = if param_list.is_empty() {
                String::new()
            } else {
                format!(", {}", param_list.join(", "))
            };
            
            self.builder.writeln(&format!("pub fn {}(&mut self{}) -> {} {{", method_name, params_str, return_type));
            self.builder.indent();
            
            // Create parameter map
            if !param_map.is_empty() {
                self.builder.writeln("let mut params = HashMap::new();");
                for param_assignment in param_map {
                    self.builder.writeln(&format!("params.insert({});", param_assignment));
                }
                self.builder.writeln(&format!("let event = FrameEvent::new(\"{}\", Some(params));", method_name));
            } else {
                self.builder.writeln(&format!("let event = FrameEvent::new(\"{}\", None);", method_name));
            }
            
            // Handle return values
            if return_type != "()" {
                self.builder.writeln("self.return_stack.push(None);");
                self.builder.writeln("self.frame_kernel(event);");
                self.builder.writeln("self.return_stack.pop().unwrap().unwrap_or_default().parse().unwrap_or_default()");
            } else {
                self.builder.writeln("self.frame_kernel(event);");
            }
            
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.writeln("");
        }
    }
    
    // Generate the Frame kernel - the heart of the state machine
    fn generate_frame_kernel(&mut self) {
        self.builder.writeln("fn frame_kernel(&mut self, event: FrameEvent) {");
        self.builder.indent();
        
        self.builder.writeln("match self.compartment.state.as_str() {");
        self.builder.indent();
        
        for state in &self.states.clone() {
            let state_method = format!("__{}_state_{}", self.system_name.to_lowercase(), state);
            self.builder.writeln(&format!("\"{}\" => self.{}(event),", state_method, state_method));
        }
        
        self.builder.writeln("_ => {} // Unknown state");
        
        self.builder.dedent();
        self.builder.writeln("}");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    // Generate state handler methods
    fn generate_state_methods(&mut self) {
        for state in &self.states.clone() {
            let state_method = format!("__{}_state_{}", self.system_name.to_lowercase(), state);
            
            self.builder.writeln(&format!("fn {}(&mut self, event: FrameEvent) {{", state_method));
            self.builder.indent();
            
            self.builder.writeln("match event.message.as_str() {");
            self.builder.indent();
            
            // Generate handlers for each event this state handles
            if let Some(handlers) = self.state_handlers.get(state) {
                for handler in handlers {
                    let handler_method = format!("_handle_{}_{}", state.to_lowercase(), handler);
                    self.builder.writeln(&format!("\"{}\" => self.{}(event),", handler, handler_method));
                }
            }
            
            self.builder.writeln("_ => {} // Unhandled event");
            
            self.builder.dedent();
            self.builder.writeln("}");
            
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.writeln("");
        }
    }
    
    // Generate main function
    fn generate_main_function(&mut self) {
        self.builder.writeln("fn main() {");
        self.builder.indent();
        
        self.builder.writeln(&format!("let mut system = {}::new();", self.system_name));
        
        // Look for a 'test' interface method and call it
        if self.interface_methods.contains_key("test") {
            self.builder.writeln("system.test();");
        } else {
            self.builder.writeln(&format!("println!(\"Frame system {} initialized and running\");", self.system_name));
        }
        
        self.builder.dedent();
        self.builder.writeln("}");
    }
}

// Implement the AstVisitor trait
impl AstVisitor for RustVisitorHandcrafted {
    fn visit_system_node(&mut self, node: &SystemNode) {
        self.system_name = node.name.clone();
        
        // First pass: collect all metadata
        if let Some(interface) = &node.interface_block_node_opt {
            self.collect_interface_methods(interface);
        }
        
        if let Some(domain) = &node.domain_block_node_opt {
            self.collect_domain_variables(domain);
        }
        
        if let Some(machine) = &node.machine_block_node_opt {
            self.collect_states_and_handlers(machine);
        }
        
        if let Some(actions) = &node.actions_block_node_opt {
            self.collect_actions(actions);
        }
        
        // Generate the code
        self.generate_frame_runtime();
        self.generate_system_struct();
        self.generate_constructor();
        self.generate_interface_methods();
        self.generate_frame_kernel();
        self.generate_state_methods();
        
        // Generate action implementations if any
        if let Some(actions) = &node.actions_block_node_opt {
            self.visit_actions_block_node(actions);
        }
        
        // Close the impl block
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
        
        // Generate main function
        self.generate_main_function();
    }
    
    fn visit_interface_block_node(&mut self, _interface_block: &InterfaceBlockNode) {
        // Already handled in collect phase
    }
    
    fn visit_machine_block_node(&mut self, _machine_block: &MachineBlockNode) {
        // Already handled in collect phase
    }
    
    fn visit_actions_block_node(&mut self, actions_block: &ActionsBlockNode) {
        self.builder.writeln("// ==================== Actions ==================== //");
        self.builder.writeln("");
        
        for action_rcref in &actions_block.actions {
            let action_node = action_rcref.borrow();
            self.visit_action_node(&action_node);
        }
    }
    
    fn visit_operations_block_node(&mut self, _operations_block: &OperationsBlockNode) {
        // TODO: Implement operations
    }
    
    fn visit_interface_method_node(&mut self, _method: &InterfaceMethodNode) {
        // Handled in collect phase
    }
    
    fn visit_action_node(&mut self, action: &ActionNode) {
        let action_name = &action.name;
        
        // Simple action implementation
        self.builder.writeln(&format!("fn _action_{}(&mut self) {{", action_name));
        self.builder.indent();
        self.builder.writeln(&format!("println!(\"Action {} executed\");", action_name));
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
    
    fn visit_operation_node(&mut self, _operation: &OperationNode) {
        // TODO: Implement operations
    }
}

// Helper methods for collecting metadata
impl RustVisitorHandcrafted {
    fn collect_interface_methods(&mut self, interface_block: &InterfaceBlockNode) {
        for method_rcref in &interface_block.interface_methods {
            let method_node = method_rcref.borrow();
            
            let mut params = Vec::new();
            if let Some(param_nodes) = &method_node.params {
                for param in param_nodes {
                    params.push(param.param_name.clone());
                }
            }
            
            let return_type = if let Some(return_type_node) = &method_node.return_type_opt {
                Some(return_type_node.get_type_str())
            } else {
                None
            };
            
            self.interface_methods.insert(method_node.name.clone(), (params, return_type));
        }
    }
    
    fn collect_domain_variables(&mut self, domain_block: &DomainBlockNode) {
        for var_decl_rcref in &domain_block.member_variables {
            let var_decl = var_decl_rcref.borrow();
            let var_type = if let Some(type_node) = &var_decl.type_opt {
                type_node.get_type_str()
            } else {
                "string".to_string() // Default type
            };
            self.domain_variables.insert(var_decl.name.clone(), var_type);
        }
    }
    
    fn collect_states_and_handlers(&mut self, machine_block: &MachineBlockNode) {
        for state_rcref in &machine_block.states {
            let state_node = state_rcref.borrow();
            let state_name = state_node.name.clone();
            self.states.push(state_name.clone());
            
            let mut handlers = Vec::new();
            for evt_handler_rcref in &state_node.evt_handlers_rcref {
                let evt_handler = evt_handler_rcref.borrow();
                let event_symbol = evt_handler.event_symbol_rcref.borrow();
                handlers.push(event_symbol.msg.clone());
            }
            
            self.state_handlers.insert(state_name, handlers);
        }
    }
    
    fn collect_actions(&mut self, actions_block: &ActionsBlockNode) {
        for action_rcref in &actions_block.actions {
            let action_node = action_rcref.borrow();
            // For now, just track action names
            self.actions.insert(action_node.name.clone(), Vec::new());
        }
    }
}
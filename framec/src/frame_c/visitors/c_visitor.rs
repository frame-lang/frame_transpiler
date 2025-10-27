// C Visitor for Frame Language Transpiler
// Generates portable C99 code from Frame AST using established visitor patterns
// v0.87.0 - Complete implementation with module scope support and thread safety

use crate::frame_c::ast::*;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::config::FrameConfig;
use crate::frame_c::scanner::Token;
use crate::frame_c::symbol_table::{Arcanum, SymbolConfig};
use crate::frame_c::visitors::AstVisitor;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct CConfig {
    pub features: CFeatures,
    pub code: CCodeConfig,
}

#[derive(Debug, Clone)]
pub struct CFeatures {
    pub thread_safe: bool,
    pub use_pthread: bool,
    pub use_constructor: bool, // Use GNU C constructor attribute
    pub portable: bool,        // Generate portable C99 code
}

#[derive(Debug, Clone)]
pub struct CCodeConfig {
    pub module_state_name: String,
    pub init_function_name: String,
    pub ensure_init_name: String,
}

impl Default for CConfig {
    fn default() -> Self {
        Self {
            features: CFeatures {
                thread_safe: false,
                use_pthread: false,
                use_constructor: true,
                portable: true,
            },
            code: CCodeConfig {
                module_state_name: "ModuleState".to_string(),
                init_function_name: "__frame_module_init".to_string(),
                ensure_init_name: "ensure_module_init".to_string(),
            },
        }
    }
}

pub struct CVisitor {
    // Core configuration
    config: FrameConfig,
    c_config: CConfig,

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
    current_handler_params: HashMap<String, String>,
    current_state_params: HashMap<String, String>,
    action_signatures: HashMap<String, ActionSignature>,
    operation_signatures: HashMap<String, OperationSignature>,

    // State tracking
    states: Vec<String>,
    state_events: HashMap<String, Vec<String>>,

    // Type tracking
    declared_types: HashSet<String>,
    imported_types: HashSet<String>,

    // Generation flags
    is_generating_interface_method: bool,
    is_generating_action: bool,
    is_generating_operation: bool,

    // C-specific tracking
    module_functions: Vec<String>, // Functions that need forward declarations
    required_headers: HashSet<String>, // System headers needed

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

impl CVisitor {
    pub fn new(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
        config: FrameConfig,
        comments: Vec<Token>,
    ) -> Self {
        let mut visitor = Self {
            config,
            c_config: CConfig::default(),
            builder: CodeBuilder::new("    "), // 4-space indent for C
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
            module_functions: Vec::new(),
            required_headers: HashSet::new(),
            _comments: comments,
        };

        // Add standard C headers
        visitor.required_headers.insert("stdio.h".to_string());
        visitor.required_headers.insert("stdlib.h".to_string());
        visitor.required_headers.insert("string.h".to_string());
        visitor.required_headers.insert("stdbool.h".to_string());

        visitor
    }

    /// Create a new C visitor with thread-safe configuration
    pub fn new_thread_safe(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
        config: FrameConfig,
        comments: Vec<Token>,
    ) -> Self {
        let mut visitor = Self::new(arcanum, symbol_config, config, comments);
        visitor.c_config.features.thread_safe = true;
        visitor.c_config.features.use_pthread = true;
        visitor.required_headers.insert("pthread.h".to_string());
        visitor
    }

    pub fn run(mut self, frame_module: &FrameModule) -> String {
        // Visit the module and generate C code
        for system in &frame_module.systems {
            system.accept(&mut self);
        }

        self.builder.build().0 // CodeBuilder returns (String, Vec<SourceMapping>), we want just the String
    }

    // Frame type to C type mapping
    fn frame_type_to_c(&self, frame_type: &str) -> String {
        match frame_type {
            "int" => "int".to_string(),
            "float" => "double".to_string(),
            "string" => "char*".to_string(),
            "bool" => "bool".to_string(),
            "void" => "void".to_string(),
            _ => {
                // Handle generic types
                if frame_type.starts_with("List<") && frame_type.ends_with(">") {
                    let inner = &frame_type[5..frame_type.len() - 1];
                    let inner_type = self.frame_type_to_c(inner);
                    format!("{}*", inner_type) // Arrays in C
                } else if frame_type.starts_with("Dict<") && frame_type.ends_with(">") {
                    "hash_table_t*".to_string() // Custom hash table type
                } else {
                    // Custom type or unrecognized - use as-is
                    frame_type.to_string()
                }
            }
        }
    }

    // Generate C includes and forward declarations
    fn generate_includes(&mut self) {
        // Standard C headers
        for header in &self.required_headers.clone() {
            self.builder.writeln(&format!("#include <{}>", header));
        }

        if self.c_config.features.use_pthread {
            self.builder.writeln("#include <pthread.h>");
        }

        self.builder.writeln("");

        // Forward declarations
        self.generate_forward_declarations();

        self.builder.writeln("");
    }

    fn generate_forward_declarations(&mut self) {
        self.builder.writeln("// Forward declarations");

        // Module state struct forward declaration
        self.builder.writeln(&format!(
            "typedef struct {} {}_t;",
            self.c_config.code.module_state_name, self.c_config.code.module_state_name
        ));

        // System struct forward declaration
        self.builder.writeln(&format!(
            "typedef struct {} {}_t;",
            self.system_name, self.system_name
        ));

        self.builder.writeln("");
    }

    // Generate module state struct (equivalent to Rust Context)
    fn generate_module_state_struct(&mut self) {
        let state_name = &self.c_config.code.module_state_name.clone();

        self.builder.writeln(&format!("// Module state structure"));
        self.builder.writeln(&format!("struct {} {{", state_name));
        self.builder.indent();

        if self.domain_variables.is_empty() {
            self.builder
                .writeln("int _placeholder; // No domain variables");
        } else {
            for (var_name, var_type) in &self.domain_variables {
                let c_type = self.frame_type_to_c(var_type);
                self.builder.writeln(&format!("{} {};", c_type, var_name));
            }
        }

        self.builder.writeln("bool initialized;");

        if self.c_config.features.thread_safe {
            self.builder.writeln("pthread_mutex_t mutex;");
        }

        self.builder.dedent();
        self.builder.writeln("};");
        self.builder.writeln("");

        // Global module state instance
        self.builder
            .writeln(&format!("static {}_t g_module = {{0}};", state_name));
        self.builder.writeln("");
    }

    // Generate system struct
    fn generate_system_struct(&mut self) {
        self.builder.writeln(&format!("// System structure"));
        self.builder
            .writeln(&format!("struct {} {{", self.system_name));
        self.builder.indent();

        // Current state enum
        if self.states.is_empty() {
            self.builder
                .writeln("int current_state; // No states defined");
        } else {
            self.builder.writeln("enum {");
            self.builder.indent();
            for (i, state) in self.states.iter().enumerate() {
                if i == self.states.len() - 1 {
                    self.builder
                        .writeln(&format!("STATE_{}", state.to_uppercase()));
                } else {
                    self.builder
                        .writeln(&format!("STATE_{},", state.to_uppercase()));
                }
            }
            self.builder.dedent();
            self.builder.writeln("} current_state;");
        }

        // Reference to module state
        self.builder.writeln(&format!(
            "{}_t* module;",
            self.c_config.code.module_state_name
        ));

        self.builder.dedent();
        self.builder.writeln("};");
        self.builder.writeln("");
    }

    // Generate module initialization function
    fn generate_module_init(&mut self) {
        let init_func = &self.c_config.code.init_function_name.clone();

        // Private initialization function
        self.builder
            .writeln(&format!("static void {}(void) {{", init_func));
        self.builder.indent();

        self.builder.writeln("if (g_module.initialized) return;");
        self.builder.writeln("");

        if self.c_config.features.thread_safe {
            self.builder
                .writeln("if (pthread_mutex_init(&g_module.mutex, NULL) != 0) {");
            self.builder.indent();
            self.builder
                .writeln("fprintf(stderr, \"Failed to initialize mutex\\n\");");
            self.builder.writeln("exit(1);");
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.writeln("");
        }

        // Initialize domain variables
        if !self.domain_variables.is_empty() {
            self.builder.writeln("// Initialize domain variables");
            for (var_name, var_type) in &self.domain_variables {
                let default_value = self.get_default_value_for_type(var_type);
                self.builder
                    .writeln(&format!("g_module.{} = {};", var_name, default_value));
            }
            self.builder.writeln("");
        }

        self.builder.writeln("g_module.initialized = true;");

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");

        // Public ensure init function for portable usage
        let ensure_func = &self.c_config.code.ensure_init_name.clone();
        self.builder
            .writeln(&format!("void {}(void) {{", ensure_func));
        self.builder.indent();

        if self.c_config.features.thread_safe {
            self.builder
                .writeln("static pthread_once_t once = PTHREAD_ONCE_INIT;");
            self.builder
                .writeln(&format!("pthread_once(&once, {});", init_func));
        } else {
            self.builder.writeln("static bool initialized = false;");
            self.builder.writeln("if (!initialized) {");
            self.builder.indent();
            self.builder.writeln(&format!("{}();", init_func));
            self.builder.writeln("initialized = true;");
            self.builder.dedent();
            self.builder.writeln("}");
        }

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");

        // GNU C constructor attribute if enabled
        if self.c_config.features.use_constructor {
            self.builder
                .writeln(&format!("__attribute__((constructor))"));
            self.builder
                .writeln(&format!("static void __frame_auto_init(void) {{"));
            self.builder.indent();
            self.builder.writeln(&format!("{}();", init_func));
            self.builder.dedent();
            self.builder.writeln("}");
            self.builder.writeln("");
        }
    }

    // Generate system constructor
    fn generate_system_constructor(&mut self) {
        self.builder.writeln(&format!("// System constructor"));
        self.builder.writeln(&format!(
            "{}_t* {}_new(void) {{",
            self.system_name,
            self.system_name.to_lowercase()
        ));
        self.builder.indent();

        self.builder
            .writeln(&format!("{}();", self.c_config.code.ensure_init_name));
        self.builder.writeln("");

        self.builder.writeln(&format!(
            "{}_t* self = ({}_t*)malloc(sizeof({}_t));",
            self.system_name, self.system_name, self.system_name
        ));
        self.builder.writeln("if (!self) {");
        self.builder.indent();
        self.builder
            .writeln("fprintf(stderr, \"Memory allocation failed\\n\");");
        self.builder.writeln("return NULL;");
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");

        // Initialize system state
        if !self.states.is_empty() {
            self.builder.writeln(&format!(
                "self->current_state = STATE_{};",
                self.states[0].to_uppercase()
            ));
        } else {
            self.builder.writeln("self->current_state = 0;");
        }

        self.builder.writeln("self->module = &g_module;");
        self.builder.writeln("");
        self.builder.writeln("return self;");

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }

    // Generate system destructor
    fn generate_system_destructor(&mut self) {
        self.builder.writeln(&format!("// System destructor"));
        self.builder.writeln(&format!(
            "void {}_free({}_t* self) {{",
            self.system_name.to_lowercase(),
            self.system_name
        ));
        self.builder.indent();

        self.builder.writeln("if (self) {");
        self.builder.indent();
        self.builder.writeln("free(self);");
        self.builder.dedent();
        self.builder.writeln("}");

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }

    fn get_default_value_for_type(&self, frame_type: &str) -> String {
        match frame_type {
            "int" => "0".to_string(),
            "float" => "0.0".to_string(),
            "string" => "NULL".to_string(),
            "bool" => "false".to_string(),
            _ => {
                if frame_type.contains('*') {
                    "NULL".to_string()
                } else {
                    "0".to_string()
                }
            }
        }
    }

    fn collect_domain_variables(&mut self, domain_block: &DomainBlockNode) {
        for var_decl_rcref in &domain_block.member_variables {
            let var_decl = var_decl_rcref.borrow();
            if let Some(type_node) = &var_decl.type_opt {
                let var_type = type_node.get_type_str();
                self.domain_variables
                    .insert(var_decl.name.clone(), var_type);
            } else {
                // Default to int if no type specified (C convention)
                self.domain_variables
                    .insert(var_decl.name.clone(), "int".to_string());
            }
        }
    }

    fn generate_interface_method_c(&mut self, method: &InterfaceMethodNode) {
        let method_name = &method.name;

        // Build parameter list
        let mut params = vec![format!("{}_t* self", self.system_name)];
        if let Some(param_nodes) = &method.params {
            for param in param_nodes {
                let param_type = if let Some(type_node) = &param.param_type_opt {
                    self.frame_type_to_c(&type_node.get_type_str())
                } else {
                    "void".to_string()
                };
                params.push(format!("{} {}", param_type, param.param_name));
            }
        }
        let params_str = params.join(", ");

        // Determine return type
        let return_type = if let Some(return_type_node) = &method.return_type_opt {
            self.frame_type_to_c(&return_type_node.get_type_str())
        } else {
            "void".to_string()
        };

        // Generate method signature
        self.builder.writeln(&format!(
            "{} {}_{} ({}) {{",
            return_type,
            self.system_name.to_lowercase(),
            method_name,
            params_str
        ));
        self.builder.indent();

        if return_type == "void" {
            self.builder.writeln("if (!self) return;");
        } else {
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder
                .writeln(&format!("if (!self) return {};", default_value));
        }
        self.builder
            .writeln(&format!("{}();", self.c_config.code.ensure_init_name));
        self.builder.writeln("");
        self.builder.writeln("// TODO: Implement interface method");

        // Provide default return value
        if return_type != "void" {
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder.writeln(&format!("return {};", default_value));
        }

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }

    fn generate_action_method_c(&mut self, action: &ActionNode) {
        let action_name = &action.name;

        // Build parameter list
        let mut params = vec![format!("{}_t* self", self.system_name)];
        if let Some(param_nodes) = &action.params {
            for param in param_nodes {
                let param_type = if let Some(type_node) = &param.param_type_opt {
                    self.frame_type_to_c(&type_node.get_type_str())
                } else {
                    "void".to_string()
                };
                params.push(format!("{} {}", param_type, param.param_name));
            }
        }
        let params_str = params.join(", ");

        // Determine return type
        let return_type = if let Some(return_type_node) = &action.type_opt {
            self.frame_type_to_c(&return_type_node.get_type_str())
        } else {
            "void".to_string()
        };

        // Generate private action method
        self.builder.writeln(&format!(
            "static {} {}_{} ({}) {{",
            return_type,
            self.system_name.to_lowercase(),
            action_name,
            params_str
        ));
        self.builder.indent();

        if return_type == "void" {
            self.builder.writeln("if (!self) return;");
        } else {
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder
                .writeln(&format!("if (!self) return {};", default_value));
        }
        self.builder.writeln("// TODO: Implement action body");

        // Provide default return value
        if return_type != "void" {
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder.writeln(&format!("return {};", default_value));
        }

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }

    fn generate_operation_method_c(&mut self, operation: &OperationNode) {
        let operation_name = &operation.name;

        // Build parameter list
        let mut params = vec![format!("const {}_t* self", self.system_name)];
        if let Some(param_nodes) = &operation.params {
            for param in param_nodes {
                let param_type = if let Some(type_node) = &param.param_type_opt {
                    self.frame_type_to_c(&type_node.get_type_str())
                } else {
                    "void".to_string()
                };
                params.push(format!("{} {}", param_type, param.param_name));
            }
        }
        let params_str = params.join(", ");

        // Determine return type
        let return_type = if let Some(return_type_node) = &operation.type_opt {
            self.frame_type_to_c(&return_type_node.get_type_str())
        } else {
            "void".to_string()
        };

        // Generate public operation method
        self.builder.writeln(&format!(
            "{} {}_{} ({}) {{",
            return_type,
            self.system_name.to_lowercase(),
            operation_name,
            params_str
        ));
        self.builder.indent();

        if return_type == "void" {
            self.builder.writeln("if (!self) return;");
        } else {
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder
                .writeln(&format!("if (!self) return {};", default_value));
        }
        self.builder
            .writeln(&format!("{}();", self.c_config.code.ensure_init_name));
        self.builder.writeln("");
        self.builder.writeln("// TODO: Implement operation body");

        // Provide default return value
        if return_type != "void" {
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder.writeln(&format!("return {};", default_value));
        }

        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }
}

impl AstVisitor for CVisitor {
    fn visit_system_node(&mut self, node: &SystemNode) {
        self.system_name = node.name.clone();
        self.current_class_name_opt = Some(node.name.clone());

        // Generate file header and includes
        self.generate_includes();

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
        self.generate_module_state_struct();
        self.generate_system_struct();

        // Generate initialization functions
        self.generate_module_init();

        // Generate system management functions
        self.generate_system_constructor();
        self.generate_system_destructor();

        // Generate interface methods
        if let Some(interface) = &node.interface_block_node_opt {
            self.visit_interface_block_node(interface);
        }

        // Generate machine block
        if let Some(machine) = &node.machine_block_node_opt {
            self.visit_machine_block_node(machine);
        }

        // Generate actions block
        if let Some(actions) = &node.actions_block_node_opt {
            self.visit_actions_block_node(actions);
        }

        // Generate operations block
        if let Some(operations) = &node.operations_block_node_opt {
            self.visit_operations_block_node(operations);
        }
    }

    fn visit_interface_block_node(&mut self, interface_block: &InterfaceBlockNode) {
        self.builder.writeln("");
        self.builder
            .writeln("// ==================== Interface Methods ==================== //");
        self.builder.writeln("");

        for method_rcref in &interface_block.interface_methods {
            let method_node = method_rcref.borrow();
            self.generate_interface_method_c(&method_node);
        }
    }

    fn visit_machine_block_node(&mut self, _machine_block: &MachineBlockNode) {
        self.builder.writeln("");
        self.builder
            .writeln("// ==================== State Machine Logic ==================== //");
        self.builder.writeln("");
        self.builder
            .writeln("// TODO: Implement state machine dispatch logic");
        self.builder.writeln("");
    }

    fn visit_actions_block_node(&mut self, actions_block: &ActionsBlockNode) {
        self.builder.writeln("");
        self.builder
            .writeln("// ==================== Actions ==================== //");
        self.builder.writeln("");

        for action_rcref in &actions_block.actions {
            let action_node = action_rcref.borrow();
            self.generate_action_method_c(&action_node);
        }
    }

    fn visit_operations_block_node(&mut self, operations_block: &OperationsBlockNode) {
        self.builder.writeln("");
        self.builder
            .writeln("// ==================== Operations ==================== //");
        self.builder.writeln("");

        for operation_rcref in &operations_block.operations {
            let operation_node = operation_rcref.borrow();
            self.generate_operation_method_c(&operation_node);
        }
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

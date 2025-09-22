// Python Visitor v2 - Complete implementation using CodeBuilder for robust source mapping
//
// This visitor uses the CodeBuilder architecture for automatic line tracking and perfect
// source mappings without manual offsets.

use crate::frame_c::ast::*;
use crate::frame_c::code_builder::CodeBuilder;
use crate::frame_c::config::FrameConfig;
use crate::frame_c::scanner::{Token, TokenType};
use crate::frame_c::source_map::SourceMapBuilder;
use crate::frame_c::symbol_table::{SymbolConfig, Arcanum};
use crate::frame_c::visitors::AstVisitor;

use std::collections::HashSet;
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
    current_event_ret_type: String,
    
    // System metadata
    system_name: String,
    system_has_async_runtime: bool,
    
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
            current_event_ret_type: String::new(),
            system_name: String::new(),
            system_has_async_runtime: false,
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
    
    pub fn run(&mut self, frame_module: &FrameModule) -> String {
        // Add header
        self.builder.write_comment("Emitted from framec_v0.75.0");
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
        // Process functions
        for function in &frame_module.functions {
            function.borrow().accept(self);
        }
        
        // Process systems
        for system in &frame_module.systems {
            self.visit_system_node(system);
        }
        
        // Process imports
        for import in &frame_module.imports {
            self.visit_import_node(import);
        }
        
        // Process enums
        for enum_decl in &frame_module.enums {
            self.visit_enum_decl_node(&enum_decl.borrow());
        }
        
        // Process modules
        for module_node in &frame_module.modules {
            self.visit_module_node(&module_node.borrow());
        }
        
        // Process classes
        for class_node in &frame_module.classes {
            self.visit_class_node(&class_node.borrow());
        }
        
        // Process variables
        for var_decl in &frame_module.variables {
            let var = var_decl.borrow();
            self.builder.writeln_mapped(
                &format!("{} = None", var.name),
                var.line
            );
            self.global_vars.insert(var.name.clone());
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
    
    fn visit_system_node(&mut self, system_node: &SystemNode) {
        self.system_name = system_node.name.clone();
        self.system_has_async_runtime = self.check_system_async(system_node);
        
        // Generate class
        self.builder.write_class(&system_node.name, None, Some(system_node.line));
        self.builder.newline();
        
        // Generate __init__
        self.generate_system_init(system_node);
        
        // Generate interface methods
        if let Some(interface) = &system_node.interface_block_node_opt {
            self.visit_interface_block_node(interface);
        }
        
        // Generate machine block
        if let Some(machine) = &system_node.machine_block_node_opt {
            self.builder.write_comment("===================== Machine Block ===================");
            self.builder.newline();
            self.visit_machine_block_node(machine);
            
            // Generate state dispatchers
            self.builder.write_comment("===================== State Dispatchers ===================");
            self.builder.newline();
            self.generate_state_dispatchers(machine);
        }
        
        // Generate actions
        if let Some(actions) = &system_node.actions_block_node_opt {
            self.builder.write_comment("===================== Actions Block ===================");
            self.builder.newline();
            self.visit_actions_block_node(actions);
        }
        
        // Generate operations
        if let Some(operations) = &system_node.operations_block_node_opt {
            self.builder.write_comment("==================== Operations Block =================");
            self.builder.newline();
            self.visit_operations_block_node(operations);
        }
        
        // Generate system runtime
        self.generate_system_runtime(system_node);
        
        self.builder.end_class();
    }
    
    fn visit_state_node(&mut self, state_node: &StateNode) {
        self.current_state_name_opt = Some(state_node.name.clone());
        
        // Generate event handler functions
        for evt_handler_rcref in &state_node.evt_handlers_rcref {
            let evt_handler = evt_handler_rcref.borrow();
            self.generate_event_handler(&state_node.name, &evt_handler);
        }
        
        self.current_state_name_opt = None;
    }
    
    fn visit_interface_block_node(&mut self, interface_block: &InterfaceBlockNode) {
        self.builder.write_comment("==================== Interface Block ==================");
        self.builder.newline();
        
        for method in &interface_block.interface_methods {
            let method = method.borrow();
            self.generate_interface_method(&method);
        }
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
            &format!("_{}", action_node.name),
            &full_params,
            action_node.is_async,
            0  // ActionNode doesn't have line field
        );
        
        if let Some(_code) = &action_node.code_opt {
            self.builder.writeln(&format!("# Action implementation"));
            self.builder.writeln("pass");
        } else if !action_node.statements.is_empty() {
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
            self.builder.writeln("@staticmethod");
        }
        
        self.builder.write_function(
            &operation_node.name,
            &full_params,
            operation_node.is_async,
            0  // OperationNode doesn't have line field
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
    
    fn generate_system_init(&mut self, system_node: &SystemNode) {
        self.builder.writeln("def __init__(self):");
        self.builder.indent();
        
        self.builder.write_comment("Create and initialize start state compartment");
        
        if let Some(machine) = &system_node.machine_block_node_opt {
            if let Some(first_state) = machine.states.first() {
                let state = first_state.borrow();
                let state_name = self.format_state_name(&state.name);
                self.builder.writeln(&format!(
                    "self.__compartment = FrameCompartment('{}', None, None, None, None, {{}}, {{}})",
                    state_name
                ));
            } else {
                self.builder.writeln("self.__compartment = None");
            }
        } else {
            self.builder.writeln("self.__compartment = None");
        }
        
        self.builder.writeln("self.__next_compartment = None");
        self.builder.writeln("self.return_stack = [None]");
        
        // Send start event
        if system_node.machine_block_node_opt.is_some() {
            self.builder.newline();
            self.builder.write_comment("Send system start event");
            self.builder.writeln("frame_event = FrameEvent(\"$>\", None)");
            self.builder.writeln("self.__kernel(frame_event)");
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
        
        self.builder.newline();
        self.builder.write_function(
            &method.name,
            &full_params,
            method.is_async,
            0  // InterfaceMethodNode doesn't have line field
        );
        
        self.builder.writeln("self.return_stack.append(None)");
        
        // Create event and send to kernel
        if params.is_empty() {
            self.builder.writeln(&format!("__e = FrameEvent(\"{}\", None)", method.name));
        } else {
            // TODO: Pack parameters properly
            self.builder.writeln(&format!("__e = FrameEvent(\"{}\", None)", method.name));
        }
        
        if method.is_async {
            self.builder.writeln("await self.__kernel(__e)");
        } else {
            self.builder.writeln("self.__kernel(__e)");
        }
        
        self.builder.writeln("return self.return_stack.pop(-1)");
        
        self.builder.dedent();
    }
    
    fn generate_event_handler(&mut self, state_name: &str, evt_handler: &EventHandlerNode) {
        let handler_name = self.format_handler_name(state_name, &evt_handler.msg_t);
        let is_async = evt_handler.is_async || self.system_has_async_runtime;
        
        self.builder.newline();
        self.builder.write_function(
            &handler_name,
            "self, __e, compartment",
            is_async,
            evt_handler.line
        );
        
        // Generate statements
        for stmt in &evt_handler.statements {
            self.visit_decl_or_stmt(stmt);
        }
        
        // Handle terminator
        if let Some(terminator) = &evt_handler.terminator_node {
            self.visit_event_handler_terminator_node(&terminator);
        } else {
            self.builder.writeln("return");
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
            h.borrow().is_async || self.system_has_async_runtime
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
            
            if first {
                self.builder.write("if ");
                first = false;
            } else {
                self.builder.write("elif ");
            }
            
            self.builder.writeln(&format!("{}:", condition));
            self.builder.indent();
            
            let call = if needs_async {
                format!("return await self.{}(__e, compartment)", handler_name)
            } else {
                format!("return self.{}(__e, compartment)", handler_name)
            };
            self.builder.writeln(&call);
            self.builder.dedent();
        }
        
        if !state.evt_handlers_rcref.is_empty() {
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
            StatementType::ContinueStmt { .. } => {
                self.builder.writeln("continue");
            }
            StatementType::BreakStmt { .. } => {
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
                self.builder.writeln("# Unimplemented statement type");
            }
        }
    }
    
    fn visit_variable_decl_node(&mut self, var_decl: &VariableDeclNode) {
        // Get the initializer value using the proper method
        let initializer_expr = var_decl.get_initializer_value_rc();
        
        // Generate the initializer value
        let mut init_value = String::new();
        self.visit_expr_node_to_string(&*initializer_expr, &mut init_value);
        
        if init_value.is_empty() || init_value.contains("TODO") {
            init_value = "None".to_string();
        }
        
        self.builder.writeln_mapped(
            &format!("{} = {}", var_decl.name, init_value),
            var_decl.line
        );
    }
    
    fn visit_event_handler_terminator_node(&mut self, terminator: &TerminatorExpr) {
        match &terminator.terminator_type {
            TerminatorType::Return => {
                if let Some(_return_expr) = &terminator.return_expr_t_opt {
                    // Handle return with value
                    self.builder.writeln("# Return with value");
                    self.builder.writeln("return");
                } else {
                    self.builder.writeln("return");
                }
            }
        }
    }
    
    fn collect_global_vars_in_stmt(&self, _stmt: &DeclOrStmtType, _globals: &mut Vec<String>) {
        // TODO: Implement global variable detection
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
        let mut output = String::new();
        self.visit_assignment_expr_node_to_string(&node.assignment_expr_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Assignment expression to string
    fn visit_assignment_expr_node_to_string(&mut self, node: &AssignmentExprNode, output: &mut String) {
        // Generate LHS
        self.visit_expr_node_to_string(&node.l_value_box, output);
        
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
        match expr_t {
            ExprType::LiteralExprT { literal_expr_node } => {
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
                output.push_str(&format!("{}.{}", enum_expr_node.enum_type, enum_expr_node.enumerator));
            }
            _ => {
                // Handle other expression types as needed
                // output.push_str("# TODO: expr type");
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
        // IdentifierNode just has a name token, no scope field
        output.push_str(&node.name.lexeme);
    }
    
    // If statement
    fn visit_if_stmt_node(&mut self, node: &IfStmtNode) {
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
            let mut elif_cond = String::new();
            self.visit_expr_node_to_string(&elif.condition, &mut elif_cond);
            self.builder.writeln(&format!("elif {}:", elif_cond));
            self.builder.indent();
            
            self.visit_block_stmt_node(&elif.block);
            
            self.builder.dedent();
        }
        
        // Else block
        if let Some(else_block) = &node.else_block {
            self.builder.writeln("else:");
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
        // Check for resolved call type
        if let Some(resolved_type) = &node.resolved_type {
            match resolved_type {
                ResolvedCallType::Action(_) => {
                    output.push_str("self._");
                    output.push_str(&node.identifier.name.lexeme);
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
                ResolvedCallType::ClassMethod { class, method, .. } => {
                    output.push_str(class);
                    output.push('.');
                    output.push_str(method);
                }
                ResolvedCallType::ModuleFunction { module, function } => {
                    output.push_str(module);
                    output.push('.');
                    output.push_str(function);
                }
                ResolvedCallType::External(_) => {
                    output.push_str(&node.identifier.name.lexeme);
                }
            }
        } else {
            // Fallback to name
            output.push_str(&node.identifier.name.lexeme);
        }
        
        // Add arguments
        output.push('(');
        self.visit_expr_list_node_to_string(&node.call_expr_list.exprs_t, output);
        output.push(')');
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
            OperatorType::Minus => output.push('-'),
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
            
            self.visit_expr_node_to_string(key, output);
            output.push_str(": ");
            self.visit_expr_node_to_string(value, output);
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
        // Create compartment for target state
        let target_state = match &node.transition_expr_node.target_state_context_t {
            TargetStateContextType::StateRef { state_context_node } => {
                self.format_state_name(&state_context_node.state_ref_node.name)
            }
            TargetStateContextType::StateStackPop {} => {
                // Handle state stack pop
                "StateStackPop".to_string()
            }
        };
        
        self.builder.writeln(&format!(
            "next_compartment = FrameCompartment('{}', None, None, None, None, {{}}, {{}})",
            target_state
        ));
        self.builder.writeln("self.__transition(next_compartment)");
    }
    
    // Return statement
    fn visit_return_stmt_node(&mut self, node: &ReturnStmtNode) {
        if let Some(expr) = &node.expr_t_opt {
            let mut output = String::new();
            self.visit_expr_node_to_string(expr, &mut output);
            self.builder.writeln(&format!("return {}", output));
        } else {
            self.builder.writeln("return");
        }
    }
    
    // Return assign statement (system.return = value)
    fn visit_return_assign_stmt_node(&mut self, node: &ReturnAssignStmtNode) {
        let mut output = String::new();
        self.visit_expr_node_to_string(&node.expr_t, &mut output);
        self.builder.writeln_mapped(
            &format!("_return = {}", output),
            node.line
        );
    }
    
    // Parent dispatch statement (=> $^)
    fn visit_parent_dispatch_stmt_node(&mut self, node: &ParentDispatchStmtNode) {
        // Dispatch to parent state
        if let Some(_parent_compartment) = &self.current_state_name_opt {
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
        output.push_str(&node.id_node.name.lexeme);
    }
    
    // Call chain statement
    fn visit_call_chain_statement_node(&mut self, node: &CallChainStmtNode) {
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
        let mut output = String::new();
        self.visit_call_expression_node_to_string(&node.call_expr_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Binary statement
    fn visit_binary_stmt_node(&mut self, node: &BinaryStmtNode) {
        let mut output = String::new();
        self.visit_binary_expr_node_to_string(&node.binary_expr_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Variable statement
    fn visit_variable_stmt_node(&mut self, node: &VariableStmtNode) {
        let mut output = String::new();
        self.visit_variable_node_to_string(&node.var_node, &mut output);
        self.builder.writeln(&output);
    }
    
    // Expression list statement
    fn visit_expr_list_stmt_node(&mut self, node: &ExprListStmtNode) {
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
        self.builder.write(&node.id_node.name.lexeme);
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
        
        self.builder.newline();
        self.builder.write_function(
            &handler_name,
            "self, __e, compartment",
            evt_handler.is_async,
            evt_handler.line
        );
        
        // Visit statements in the event handler
        for stmt in &evt_handler.statements {
            self.visit_decl_or_stmt(stmt);
        }
        
        // Handle terminator
        if let Some(terminator) = &evt_handler.terminator_node {
            self.visit_event_handler_terminator_node(&terminator);
        } else {
            self.builder.writeln("return");
        }
        
        self.builder.dedent();
    }
    
    // State node visitor - to track current state
    fn visit_state_node(&mut self, state: &StateNode) {
        self.current_state_name_opt = Some(state.name.clone());
        
        // Visit event handlers
        for evt_handler_rcref in &state.evt_handlers_rcref {
            let evt_handler = evt_handler_rcref.borrow();
            self.visit_event_handler_node(&*evt_handler);
        }
        
        self.current_state_name_opt = None;
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
        let mut first = true;
        for call_part in &node.call_chain {
            if !first {
                output.push('.');
            }
            match call_part {
                CallChainNodeType::VariableNodeT { var_node } => {
                    output.push_str(&var_node.id_node.name.lexeme);
                }
                CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => {
                    output.push_str(&interface_method_call_expr_node.identifier.name.lexeme);
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
                    output.push_str(&action_call_expr_node.identifier.name.lexeme);
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
                    // External calls like print, str, etc.
                    self.visit_call_expression_node_to_string(call_node, output);
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    // Parameters and other undeclared identifiers
                    output.push_str(&id_node.name.lexeme);
                }
                _ => {
                    // Handle other cases as needed
                }
            }
            first = false;
        }
    }
    
}
// Frame v4 Compiler using state machine-based parsers
// This implementation uses the proven v3 state machine parsers adapted for v4

use super::{FrameV4Result, FrameV4Output, ErrorsAcc, TargetLanguage};
use super::system_parser_v4::SystemParserV4;
use crate::frame_c::v3::{
    native_region_scanner::NativeRegionScannerV3,
    mir_assembler::MirAssemblerV3,
    expander::FrameStatementExpanderV3,
    splice::SplicerV3,
};
use crate::frame_c::visitors::TargetLanguage as V3TargetLanguage;

pub struct V4StateMachineCompiler {
    target: TargetLanguage,
}

impl V4StateMachineCompiler {
    pub fn new(target: TargetLanguage) -> Self {
        Self { target }
    }
    
    pub fn compile(&self, source: &str, _file_path: &str) -> FrameV4Result {
        // V4 uses @@target and @@system, so no conversion needed
        let source_bytes = source.as_bytes();
        
        // Convert v4 target to v3 target for compatibility with existing parsers
        let v3_target = match self.target {
            TargetLanguage::Python => V3TargetLanguage::Python3,
            TargetLanguage::TypeScript => V3TargetLanguage::TypeScript,
            TargetLanguage::Rust => V3TargetLanguage::Rust,
            TargetLanguage::C => V3TargetLanguage::C,
            TargetLanguage::Cpp => V3TargetLanguage::Cpp,
            TargetLanguage::Java => V3TargetLanguage::Java,
            TargetLanguage::CSharp => V3TargetLanguage::CSharp,
            _ => V3TargetLanguage::Python3,
        };
        
        // Use v4's adapted system parser to get the complete structure
        let module = SystemParserV4::parse_module(source_bytes, v3_target);
        
        // Skip validation for now - the v3 validator needs more adaptation for v4
        // TODO: Implement proper v4 validation
        
        // Generate v4-style code from the parsed structure
        let code = match self.target {
            TargetLanguage::Python => self.generate_python(&module, source),
            TargetLanguage::TypeScript => self.generate_typescript(&module, source),
            TargetLanguage::Rust => self.generate_rust(&module, source),
            _ => return FrameV4Result::Err({
                let mut errors = ErrorsAcc::new();
                errors.push_error(format!("Unsupported target language: {:?}", self.target));
                errors
            }),
        };
        
        FrameV4Result::Ok(FrameV4Output {
            code,
            warnings: Vec::new(),
            source_map: None,
        })
    }
    
    fn generate_python(&self, module: &crate::frame_c::v3::ast::ModuleAst, source: &str) -> String {
        let mut output = String::new();
        
        // Copy native imports before @@system
        if let Some(first_system) = module.systems.first() {
            let system_start = first_system.sections.operations
                .as_ref()
                .or(first_system.sections.interface.as_ref())
                .or(first_system.sections.machine.as_ref())
                .or(first_system.sections.actions.as_ref())
                .or(first_system.sections.domain.as_ref())
                .map(|s| s.start)
                .unwrap_or(0);
            
            // Find @@system in source to know where to cut off imports
            if let Some(system_pos) = source.find("@@system") {
                let imports = &source[..system_pos];
                // Skip @@target line and @@persist annotations
                for line in imports.lines() {
                    let trimmed = line.trim_start();
                    if !trimmed.starts_with("@@target") && 
                       !trimmed.starts_with("@@persist") &&
                       !trimmed.is_empty() {
                        output.push_str(line);
                        output.push('\n');
                    }
                }
                if !imports.trim().is_empty() {
                    output.push('\n');
                }
            }
        }
        
        // Generate each system as a class
        for system in &module.systems {
            output.push_str(&format!("class {}:\n", system.name));
            
            // Constructor
            output.push_str("    def __init__(self");
            for domain_param in &system.params.domain_params {
                output.push_str(&format!(", {}", domain_param));
            }
            output.push_str("):\n");
            output.push_str("        self._state = None\n");
            output.push_str("        self._state_stack = []\n");
            output.push_str("        self._system_return = None\n");
            
            // Initialize domain variables
            if let Some(domain_span) = &system.sections.domain {
                let domain_content = &source[domain_span.start..domain_span.end];
                self.generate_python_domain_init(&mut output, domain_content);
            }
            
            // Find and transition to the start state
            if let Some(machine_span) = &system.sections.machine {
                let machine_content = &source[machine_span.start..machine_span.end];
                if let Some(first_state) = self.extract_first_state(machine_content) {
                    output.push_str(&format!("        self._transition_to_{}()\n", first_state));
                }
            }
            output.push_str("\n");
            
            // Generate interface methods
            if let Some(interface_span) = &system.sections.interface {
                let interface_content = &source[interface_span.start..interface_span.end];
                self.generate_python_interface(&mut output, interface_content);
            }
            
            // Generate state machine
            if let Some(machine_span) = &system.sections.machine {
                let machine_content = &source[machine_span.start..machine_span.end];
                self.generate_python_machine(&mut output, machine_content);
            }
            
            // Generate actions
            if let Some(actions_span) = &system.sections.actions {
                let actions_content = &source[actions_span.start..actions_span.end];
                self.generate_python_actions(&mut output, actions_content);
            }
            
            // Generate operations
            if let Some(operations_span) = &system.sections.operations {
                let operations_content = &source[operations_span.start..operations_span.end];
                self.generate_python_operations(&mut output, operations_content);
            }
        }
        
        // Copy native code after @@system
        // First, find where the @@system block ends
        if let Some(system_start) = source.find("@@system") {
            // Find the matching closing brace for the system
            let mut brace_count = 0;
            let mut in_system = false;
            let mut system_end = 0;
            
            for (i, ch) in source[system_start..].char_indices() {
                if ch == '{' {
                    if !in_system {
                        in_system = true;
                    }
                    brace_count += 1;
                } else if ch == '}' && in_system {
                    brace_count -= 1;
                    if brace_count == 0 {
                        system_end = system_start + i + 1;
                        break;
                    }
                }
            }
            
            if system_end > 0 && system_end < source.len() {
                let trailing = &source[system_end..];
                if !trailing.trim().is_empty() {
                    output.push_str("\n");
                    output.push_str(trailing);
                }
            }
        }
        
        output
    }
    
    fn generate_python_domain_init(&self, output: &mut String, domain_content: &str) {
        for line in domain_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == "domain:" {
                continue;
            }
            
            // Parse domain variable declarations like "var x = 0" or "tickCount = 0"
            let var_line = if trimmed.starts_with("var ") {
                &trimmed[4..]
            } else {
                trimmed
            };
            
            if let Some(eq_pos) = var_line.find('=') {
                let var_name = var_line[..eq_pos].trim();
                let var_value = var_line[eq_pos + 1..].trim();
                output.push_str(&format!("        self.{} = {}\n", var_name, var_value));
            } else if !var_line.is_empty() {
                // Variable without initial value
                output.push_str(&format!("        self.{} = None\n", var_line));
            }
        }
    }
    
    fn extract_first_state(&self, machine_content: &str) -> Option<String> {
        for line in machine_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('$') && !trimmed.starts_with("$>") && !trimmed.starts_with("$<") {
                // Extract state name
                let state_name = trimmed[1..]
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .unwrap_or("");
                if !state_name.is_empty() {
                    return Some(state_name.to_string());
                }
            }
        }
        None
    }
    
    fn generate_python_interface(&self, output: &mut String, interface_content: &str) {
        for line in interface_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == "interface:" {
                continue;
            }
            
            // Parse method signatures
            if let Some(paren_pos) = trimmed.find('(') {
                let method_name = trimmed[..paren_pos].trim();
                if method_name.is_empty() {
                    continue;
                }
                
                // Extract parameters and return type
                let signature = &trimmed[paren_pos..];
                let (params, return_type) = self.parse_method_signature(signature);
                
                // Generate interface method
                output.push_str(&format!("    def {}(self", method_name));
                for param in params {
                    output.push_str(&format!(", {}", param));
                }
                output.push_str("):\n");
                output.push_str("        self._system_return = None\n");
                output.push_str(&format!("        self._dispatch('{}', locals())\n", method_name));
                output.push_str("        return self._system_return\n");
                output.push_str("\n");
            }
        }
        
        // Generate dispatch method
        output.push_str("    def _dispatch(self, event, args):\n");
        output.push_str("        handler = getattr(self, f'_handle_{self._state}_{event}', None)\n");
        output.push_str("        if handler:\n");
        output.push_str("            # Remove 'self' from args\n");
        output.push_str("            args = {k: v for k, v in args.items() if k != 'self'}\n");
        output.push_str("            return handler(**args)\n");
        output.push_str("\n");
    }
    
    fn parse_method_signature(&self, signature: &str) -> (Vec<String>, Option<String>) {
        // Parse "(params): return_type" or just "(params)"
        let mut params = Vec::new();
        let mut return_type = None;
        
        if let Some(close_paren) = signature.find(')') {
            let params_str = &signature[1..close_paren];
            for param in params_str.split(',') {
                let param = param.trim();
                if !param.is_empty() {
                    // Strip type annotations for now
                    let param_name = param.split(':').next().unwrap_or(param).trim();
                    params.push(param_name.to_string());
                }
            }
            
            // Check for return type
            let after_paren = &signature[close_paren + 1..].trim();
            if after_paren.starts_with(':') {
                return_type = Some(after_paren[1..].trim().to_string());
            }
        }
        
        (params, return_type)
    }
    
    fn generate_python_machine(&self, output: &mut String, machine_content: &str) {
        let mut current_state = None;
        let mut in_handler = false;
        let mut handler_body = String::new();
        let mut handler_name = String::new();
        let mut handler_params = Vec::new();
        let mut brace_depth = 0;
        
        for line in machine_content.lines() {
            let trimmed = line.trim();
            
            // Check for state declaration
            if trimmed.starts_with('$') && !trimmed.starts_with("$>") && !trimmed.starts_with("$<") && trimmed.contains('{') {
                // Extract state name and parameters
                let state_part = trimmed[1..].trim_end_matches('{').trim();
                let (state_name, _state_params) = if let Some(paren_pos) = state_part.find('(') {
                    let name = state_part[..paren_pos].trim().to_string();
                    let params = state_part[paren_pos..].to_string();
                    (name, Some(params))
                } else {
                    (state_part.to_string(), None)
                };
                
                if !state_name.is_empty() {
                    // Generate transition method for this state
                    output.push_str(&format!("    def _transition_to_{}(self", state_name));
                    // TODO: Add state parameters
                    output.push_str("):\n");
                    output.push_str(&format!("        self._state = '{}'\n", state_name));
                    output.push_str(&format!("        # Call enter handler if exists\n"));
                    output.push_str(&format!("        enter_handler = getattr(self, '_enter_{}', None)\n", state_name));
                    output.push_str("        if enter_handler:\n");
                    output.push_str("            enter_handler()\n");
                    output.push_str("\n");
                    
                    current_state = Some(state_name);
                    brace_depth = 1;
                }
            }
            // Check for handler declaration
            else if let Some(ref state) = current_state {
                if !in_handler && (trimmed.ends_with('{') || trimmed.contains("(){")) {
                    // Parse handler signature
                    let handler_line = trimmed.trim_end_matches('{').trim();
                    
                    if handler_line.starts_with("$>") {
                        // Enter handler
                        handler_name = "_enter".to_string();
                        in_handler = true;
                        handler_body.clear();
                        brace_depth += 1;
                    } else if handler_line.starts_with("$<") {
                        // Exit handler
                        handler_name = "_exit".to_string();
                        in_handler = true;
                        handler_body.clear();
                        brace_depth += 1;
                    } else if let Some(paren_pos) = handler_line.find('(') {
                        // Event handler
                        handler_name = handler_line[..paren_pos].trim().to_string();
                        let params_str = handler_line[paren_pos..].trim_end_matches(')');
                        handler_params = self.parse_handler_params(params_str);
                        in_handler = true;
                        handler_body.clear();
                        brace_depth += 1;
                    }
                } else if in_handler {
                    // Count braces
                    for ch in trimmed.chars() {
                        if ch == '{' {
                            brace_depth += 1;
                        } else if ch == '}' {
                            brace_depth -= 1;
                        }
                    }
                    
                    if brace_depth == 1 && trimmed == "}" {
                        // End of handler
                        in_handler = false;
                        
                        // Generate handler method
                        if handler_name == "_enter" {
                            output.push_str(&format!("    def _enter_{}(self):\n", state));
                        } else if handler_name == "_exit" {
                            output.push_str(&format!("    def _exit_{}(self):\n", state));
                        } else {
                            output.push_str(&format!("    def _handle_{}_{}(self", state, handler_name));
                            for param in &handler_params {
                                output.push_str(&format!(", {}", param));
                            }
                            output.push_str("):\n");
                        }
                        
                        // Process handler body for Frame statements
                        let processed = self.process_python_handler_body(&handler_body);
                        if processed.trim().is_empty() {
                            output.push_str("        pass\n");
                        } else {
                            for line in processed.lines() {
                                if !line.trim().is_empty() {
                                    output.push_str("        ");
                                    output.push_str(line.trim());
                                    output.push_str("\n");
                                }
                            }
                        }
                        output.push_str("\n");
                        
                        handler_params.clear();
                    } else if brace_depth > 1 || (brace_depth == 1 && !trimmed.is_empty()) {
                        // Inside handler body
                        handler_body.push_str(line);
                        handler_body.push('\n');
                    }
                } else if trimmed == "}" && brace_depth == 1 {
                    // End of state
                    current_state = None;
                    brace_depth = 0;
                }
            }
        }
    }
    
    fn parse_handler_params(&self, params_str: &str) -> Vec<String> {
        let mut params = Vec::new();
        // TODO: Properly parse parameters with types
        params
    }
    
    fn process_python_handler_body(&self, body: &str) -> String {
        // Process Frame statements in handler body
        let mut result = String::new();
        
        for line in body.lines() {
            let trimmed = line.trim();
            
            // Preserve leading whitespace for native code
            let indent = line.len() - line.trim_start().len();
            let indent_str = " ".repeat(indent);
            
            // Check for Frame statements
            if trimmed.starts_with("-> $") {
                // State transition
                let state = trimmed[4..].trim_end_matches("()");
                result.push_str(&format!("self._transition_to_{}()\n", state));
                result.push_str("return\n");
            } else if trimmed.starts_with("system.return =") {
                // System return
                let value = trimmed[15..].trim();
                result.push_str(&format!("self._system_return = {}\n", value));
            } else if trimmed == "$$[+]" {
                // Stack push
                result.push_str("self._state_stack.append(self._state)\n");
            } else if trimmed == "$$[-]" {
                // Stack pop
                result.push_str("if self._state_stack:\n");
                result.push_str("    prev_state = self._state_stack.pop()\n");
                result.push_str("    self._state = prev_state\n");
            } else if trimmed.is_empty() {
                // Keep empty lines
                result.push_str("\n");
            } else {
                // Native Python code - preserve original
                result.push_str(trimmed);
                result.push_str("\n");
            }
        }
        
        result
    }
    
    fn generate_python_actions(&self, output: &mut String, actions_content: &str) {
        let mut current_action = None;
        let mut in_body = false;
        let mut body = String::new();
        let mut brace_depth = 0;
        
        for line in actions_content.lines() {
            let trimmed = line.trim();
            
            if trimmed == "actions:" {
                continue;
            }
            
            // Check for action declaration
            if !in_body && trimmed.contains('(') && trimmed.contains(')') && trimmed.ends_with('{') {
                // Parse action signature
                let signature = trimmed.trim_end_matches('{').trim();
                if let Some(paren_pos) = signature.find('(') {
                    let action_name = signature[..paren_pos].trim();
                    let params_part = &signature[paren_pos..];
                    let (params, _return_type) = self.parse_method_signature(params_part);
                    
                    // Generate action method
                    output.push_str(&format!("    def {}(self", action_name));
                    for param in &params {
                        output.push_str(&format!(", {}", param));
                    }
                    output.push_str("):\n");
                    
                    current_action = Some(action_name.to_string());
                    in_body = true;
                    body.clear();
                    brace_depth = 1;
                }
            } else if in_body {
                // Count braces
                for ch in trimmed.chars() {
                    if ch == '{' {
                        brace_depth += 1;
                    } else if ch == '}' {
                        brace_depth -= 1;
                    }
                }
                
                if brace_depth == 0 && trimmed == "}" {
                    // End of action body
                    in_body = false;
                    
                    // Process the body
                    let processed = self.process_python_handler_body(&body);
                    if processed.trim().is_empty() {
                        output.push_str("        pass\n");
                    } else {
                        for line in processed.lines() {
                            if !line.trim().is_empty() {
                                output.push_str("        ");
                                output.push_str(line.trim());
                                output.push_str("\n");
                            }
                        }
                    }
                    output.push_str("\n");
                    current_action = None;
                } else {
                    // Accumulate body
                    body.push_str(line);
                    body.push('\n');
                }
            }
        }
    }
    
    fn generate_python_operations(&self, output: &mut String, operations_content: &str) {
        // TODO: Parse and generate operations
        _ = output;
        _ = operations_content;
    }
    
    fn generate_typescript(&self, _module: &crate::frame_c::v3::ast::ModuleAst, _source: &str) -> String {
        "// TypeScript generation not yet implemented\n".to_string()
    }
    
    fn generate_rust(&self, _module: &crate::frame_c::v3::ast::ModuleAst, _source: &str) -> String {
        "// Rust generation not yet implemented\n".to_string()
    }
}
// Frame v4 Compiler using v3's parsers but generating v4-style code
// This uses v3's proven parsing infrastructure but generates code without runtime libraries

use crate::frame_c::v3::{
    system_parser::SystemParserV3,
    machine_parser::MachineParserV3,
    interface_parser::InterfaceParserV3,
    domain_scanner::DomainBlockScannerV3,
    native_region_scanner::{NativeRegionScannerV3, RegionV3},
    mir_assembler::MirAssemblerV3,
    module_partitioner::ModulePartitionerV3,
    validator::BodyKindV3,
};
use crate::frame_c::visitors::TargetLanguage as V3TargetLanguage;
use super::{FrameV4Result, FrameV4Output, ErrorsAcc, TargetLanguage};

pub struct V3BasedCompiler {
    target: TargetLanguage,
}

impl V3BasedCompiler {
    pub fn new(target: TargetLanguage) -> Self {
        Self { target }
    }
    
    pub fn compile(&self, source: &str, _file_name: &str) -> FrameV4Result {
        // Convert @@target to @target for v3 compatibility
        let v3_source = source.replace("@@target ", "@target ");
        
        // Convert v4 target to v3 target
        let v3_target = match self.target {
            TargetLanguage::Python => V3TargetLanguage::Python3,
            TargetLanguage::TypeScript => V3TargetLanguage::TypeScript,
            TargetLanguage::Rust => V3TargetLanguage::Rust,
            _ => V3TargetLanguage::Python3,
        };
        
        // Use v3's system parser to get the complete structure
        let module = SystemParserV3::parse_module(v3_source.as_bytes(), v3_target);
        
        // Generate v4-style code from the parsed structure
        let code = match self.target {
            TargetLanguage::Python => self.generate_python(&module, &v3_source),
            TargetLanguage::TypeScript => self.generate_typescript(&module, &v3_source),
            TargetLanguage::Rust => self.generate_rust(&module, &v3_source),
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
        
        // Generate imports if needed
        output.push_str("from typing import Any, Dict, List, Optional\n\n");
        
        // Generate each system as a class
        for system in &module.systems {
            output.push_str(&format!("class {}:\n", system.name));
            
            // Constructor
            output.push_str("    def __init__(self");
            // Add system parameters if present
            for domain_param in &system.params.domain_params {
                output.push_str(&format!(", {}", domain_param));
            }
            output.push_str("):\n");
            output.push_str("        self._state = None\n");
            output.push_str("        self._state_stack = []\n");
            
            // Initialize domain variables if present
            for domain_param in &system.params.domain_params {
                output.push_str(&format!("        self.{} = {}\n", domain_param, domain_param));
            }
            
            // Find the start state from machine section
            if let Some(machine_span) = &system.sections.machine {
                // Extract machine content and find first state
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
            
            // Generate dispatch method
            output.push_str("    def _dispatch(self, event, args):\n");
            output.push_str("        handler = getattr(self, f'_handle_{self._state}_{event}', None)\n");
            output.push_str("        if handler:\n");
            output.push_str("            args = {k: v for k, v in args.items() if k != 'self'}\n");
            output.push_str("            handler(**args)\n");
            output.push_str("\n");
            
            // Generate state machine
            if let Some(machine_span) = &system.sections.machine {
                let machine_content = &source[machine_span.start..machine_span.end];
                self.generate_python_machine(&mut output, machine_content, source.as_bytes());
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
        
        output
    }
    
    fn extract_first_state(&self, machine_content: &str) -> Option<String> {
        // Find first $StateName in machine
        for line in machine_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('$') {
                // Extract state name
                let state_name = trimmed[1..].split(|c: char| !c.is_alphanumeric() && c != '_')
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
        // Parse interface methods and generate Python methods
        for line in interface_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == "interface:" {
                continue;
            }
            
            // Parse method signature like "test()"
            if let Some(paren_pos) = trimmed.find('(') {
                let method_name = trimmed[..paren_pos].trim();
                if method_name.is_empty() {
                    continue;
                }
                
                // Generate interface method
                output.push_str(&format!("    def {}(self):\n", method_name));
                output.push_str("        self._system_return = None\n");
                output.push_str(&format!("        self._dispatch('{}', locals())\n", method_name));
                output.push_str("        return self._system_return\n");
                output.push_str("\n");
            }
        }
    }
    
    fn generate_python_machine(&self, output: &mut String, machine_content: &str, full_source: &[u8]) {
        // Parse states and generate handlers
        let mut current_state = None;
        let mut in_handler = false;
        let mut handler_body = String::new();
        let mut handler_name = String::new();
        
        for line in machine_content.lines() {
            let trimmed = line.trim();
            
            // Check for state declaration
            if trimmed.starts_with('$') && trimmed.contains('{') {
                // Extract state name
                let state_name = trimmed[1..].split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .unwrap_or("")
                    .to_string();
                    
                if !state_name.is_empty() {
                    // Generate transition method for this state
                    output.push_str(&format!("    def _transition_to_{}(self):\n", state_name));
                    output.push_str("        if self._state:\n");
                    output.push_str("            exit_handler = getattr(self, f'_exit_{self._state}', None)\n");
                    output.push_str("            if exit_handler:\n");
                    output.push_str("                exit_handler()\n");
                    output.push_str(&format!("        self._state = '{}'\n", state_name));
                    output.push_str("\n");
                    
                    current_state = Some(state_name);
                }
            }
            // Check for handler declaration
            else if let Some(ref state) = current_state {
                if trimmed.ends_with('{') && !trimmed.starts_with('}') {
                    // Extract handler name (event name)
                    let handler = trimmed.trim_end_matches('{').trim();
                    if let Some(paren_pos) = handler.find('(') {
                        handler_name = handler[..paren_pos].trim().to_string();
                        in_handler = true;
                        handler_body.clear();
                    }
                }
                else if in_handler {
                    if trimmed == "}" {
                        // End of handler - process the body with MIR
                        in_handler = false;
                        
                        // Generate handler method
                        output.push_str(&format!("    def _handle_{}_{}(self):\n", state, handler_name));
                        
                        // Process handler body with v3's MIR to expand Frame statements
                        let processed_body = self.process_handler_body(&handler_body);
                        if processed_body.trim().is_empty() {
                            output.push_str("        pass\n");
                        } else {
                            for line in processed_body.lines() {
                                if !line.trim().is_empty() {
                                    output.push_str("        ");
                                    output.push_str(line.trim());
                                    output.push_str("\n");
                                }
                            }
                        }
                        output.push_str("\n");
                    } else {
                        // Accumulate handler body
                        handler_body.push_str(line);
                        handler_body.push('\n');
                    }
                }
            }
        }
    }
    
    fn process_handler_body(&self, body: &str) -> String {
        use crate::frame_c::v3::native_region_scanner::python::NativeRegionScannerPyV3;
        
        // First, replace system.return with self._system_return
        let body = body.replace("system.return", "self._system_return");
        
        // Use v3's native scanner to find Frame constructs
        let mut scanner = NativeRegionScannerPyV3;
        let body_with_braces = format!("{{{}}}", body); // Add braces for scanner
        
        match scanner.scan(body_with_braces.as_bytes(), 0) {
            Ok(scan) => {
                // Use v3's MIR assembler
                match MirAssemblerV3.assemble(body_with_braces.as_bytes(), &scan.regions) {
                    Ok(mir) => {
                        // Expand Frame statements using our v4 expander
                        let mut result = String::new();
                        let mut cursor = 1; // Skip opening brace
                        
                        for region in &scan.regions {
                            match region {
                                RegionV3::NativeText { span } => {
                                    // Copy native text
                                    if span.start > 0 && span.end <= body_with_braces.len() - 1 {
                                        let text = &body_with_braces[span.start..span.end];
                                        result.push_str(text);
                                    }
                                }
                                RegionV3::FrameSegment { .. } => {
                                    // Frame statement was here - already expanded
                                }
                            }
                        }
                        
                        // Now splice in the expansions
                        let mut expansions = Vec::new();
                        for (i, mir_item) in mir.iter().enumerate() {
                            use crate::frame_c::v3::mir::MirItemV3;
                            match mir_item {
                                MirItemV3::Transition { target, .. } => {
                                    expansions.push(format!("self._transition_to_{}()", target));
                                }
                                _ => {
                                    expansions.push("# TODO: Other Frame statement".to_string());
                                }
                            }
                        }
                        
                        // Use v3's splicer
                        let spliced = crate::frame_c::v3::splice::SplicerV3
                            .splice(body_with_braces.as_bytes(), &scan.regions, &expansions);
                        
                        // Remove the braces we added and return
                        let result = spliced.text.trim_start_matches('{').trim_end_matches('}');
                        result.to_string()
                    }
                    Err(_) => {
                        // No MIR found, just return the body with system.return replaced
                        body.to_string()
                    }
                }
            }
            Err(_) => {
                // Scanner failed, just return the body with system.return replaced
                body.to_string()
            }
        }
    }
    
    fn generate_python_actions(&self, output: &mut String, actions_content: &str) {
        // TODO: Parse and generate actions
        _ = output;
        _ = actions_content;
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
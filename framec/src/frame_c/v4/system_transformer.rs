use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::system_parser::SystemParser;
use crate::frame_c::v4::arcanum::Arcanum;

/// Transforms Frame system declarations into native language classes
pub struct SystemTransformer;

impl SystemTransformer {
    /// Transform a Frame system into a native language class
    /// This preserves the "oceans model" - native code with Frame islands
    pub fn transform_system(
        &self,
        bytes: &[u8],
        system_start: usize,
        system_end: usize,
        sys_name: &str,
        arc: &Arcanum,
        lang: TargetLanguage,
    ) -> String {
        match lang {
            TargetLanguage::Python3 => self.transform_to_python(bytes, system_start, system_end, sys_name, arc),
            TargetLanguage::TypeScript => self.transform_to_typescript(bytes, system_start, system_end, sys_name, arc),
            TargetLanguage::Rust => self.transform_to_rust(bytes, system_start, system_end, sys_name, arc),
            _ => {
                // For unsupported languages, return the original system unchanged
                String::from_utf8_lossy(&bytes[system_start..system_end]).to_string()
            }
        }
    }

    fn transform_to_python(&self, bytes: &[u8], start: usize, end: usize, sys_name: &str, arc: &Arcanum) -> String {
        let mut out = String::new();
        
        // Parse the system to understand its structure
        let module_ast = SystemParser::parse_module(bytes, TargetLanguage::Python3);
        let system = match module_ast.systems.iter().find(|s| s.name == sys_name) {
            Some(s) => s,
            None => return String::from_utf8_lossy(&bytes[start..end]).to_string(),
        };

        // Extract system parameters
        let param_groups = crate::frame_c::v4::parse_system_params(bytes, sys_name);
        
        // Get the start state
        let start_state = crate::frame_c::v4::find_start_state_name(arc, sys_name)
            .unwrap_or_else(|| "A".to_string());

        // Generate the class header
        out.push_str(&format!("class {}:\n", sys_name));
        
        // Generate __init__ method
        out.push_str("    def __init__(self");
        if !param_groups.declared.is_empty() {
            for param in &param_groups.declared {
                out.push_str(&format!(", {}", param));
            }
        }
        out.push_str("):\n");
        
        // Initialize Frame machinery
        out.push_str(&format!("        self._state = \"{}\"\n", start_state));
        out.push_str("        self._stack = []\n");
        
        // Initialize domain variables from parameters
        for param in &param_groups.domain {
            out.push_str(&format!("        self.{} = {}\n", param, param));
        }
        
        // Call enter handler if there are enter params
        if !param_groups.enter.is_empty() {
            out.push_str("        self._enter_state(self._state");
            for param in &param_groups.enter {
                out.push_str(&format!(", {}", param));
            }
            out.push_str(")\n");
        }
        out.push_str("\n");

        // Generate Frame machinery methods
        out.push_str("    def _frame_transition(self, next_state, *args):\n");
        out.push_str("        self._exit_state(self._state)\n");
        out.push_str("        self._state = next_state\n");
        out.push_str("        self._enter_state(next_state, *args)\n");
        out.push_str("\n");

        out.push_str("    def _frame_stack_push(self):\n");
        out.push_str("        self._stack.append(self._state)\n");
        out.push_str("\n");

        out.push_str("    def _frame_stack_pop(self):\n");
        out.push_str("        if self._stack:\n");
        out.push_str("            prev_state = self._stack.pop()\n");
        out.push_str("            self._frame_transition(prev_state)\n");
        out.push_str("\n");

        out.push_str("    def _enter_state(self, state, *args):\n");
        out.push_str("        # Override in derived class if needed\n");
        out.push_str("        pass\n");
        out.push_str("\n");

        out.push_str("    def _exit_state(self, state):\n");
        out.push_str("        # Override in derived class if needed\n");
        out.push_str("        pass\n");
        out.push_str("\n");

        // Extract and transform interface methods
        if let Some(iface_section) = system.sections.interface {
            let iface_bytes = &bytes[iface_section.start..iface_section.end];
            self.transform_interface_methods(&mut out, iface_bytes, sys_name);
        }

        // Extract and transform event handlers from machine section
        if let Some(machine_section) = system.sections.machine {
            let machine_bytes = &bytes[machine_section.start..machine_section.end];
            self.transform_machine_handlers(&mut out, machine_bytes, sys_name, arc);
        }

        // Extract and transform actions
        if let Some(actions_section) = system.sections.actions {
            let actions_bytes = &bytes[actions_section.start..actions_section.end];
            self.transform_actions(&mut out, actions_bytes);
        }

        // Extract and transform operations
        if let Some(ops_section) = system.sections.operations {
            let ops_bytes = &bytes[ops_section.start..ops_section.end];
            self.transform_operations(&mut out, ops_bytes);
        }

        // Extract domain variables
        if let Some(domain_section) = system.sections.domain {
            let domain_bytes = &bytes[domain_section.start..domain_section.end];
            self.transform_domain(&mut out, domain_bytes);
        }

        out
    }

    fn transform_interface_methods(&self, out: &mut String, iface_bytes: &[u8], sys_name: &str) {
        // Parse interface methods and generate Python method stubs
        let iface_str = String::from_utf8_lossy(iface_bytes);
        for line in iface_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }
            
            // Simple method detection - look for identifier followed by parentheses
            if let Some(paren_pos) = trimmed.find('(') {
                let method_name = trimmed[..paren_pos].trim();
                if !method_name.is_empty() && method_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    // Generate public method that routes to event handler
                    out.push_str(&format!("    def {}(self", method_name));
                    
                    // Extract parameters if any
                    if let Some(close_paren) = trimmed.find(')') {
                        let params_str = &trimmed[paren_pos+1..close_paren];
                        if !params_str.trim().is_empty() {
                            out.push_str(", ");
                            out.push_str(params_str);
                        }
                    }
                    
                    out.push_str("):\n");
                    out.push_str(&format!("        return self._handle_{}()\n", method_name));
                    out.push_str("\n");
                }
            }
        }
    }

    fn transform_machine_handlers(&self, out: &mut String, machine_bytes: &[u8], sys_name: &str, arc: &Arcanum) {
        // This would parse states and event handlers
        // For now, generate a simple dispatcher
        out.push_str("    def _route_event(self, event_name, *args):\n");
        out.push_str("        # Route events based on current state\n");
        out.push_str("        handler_name = f\"_handle_{self._state}_{event_name}\"\n");
        out.push_str("        handler = getattr(self, handler_name, None)\n");
        out.push_str("        if handler:\n");
        out.push_str("            return handler(*args)\n");
        out.push_str("\n");
    }

    fn transform_actions(&self, out: &mut String, actions_bytes: &[u8]) {
        // Transform action methods
        let actions_str = String::from_utf8_lossy(actions_bytes);
        out.push_str("    # Actions\n");
        
        for line in actions_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }
            
            // Detect action definitions
            if let Some(paren_pos) = trimmed.find('(') {
                let action_name = trimmed[..paren_pos].trim();
                if !action_name.is_empty() && action_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    out.push_str(&format!("    def _action_{}(self):\n", action_name));
                    out.push_str("        # Action implementation\n");
                    out.push_str("        pass\n");
                    out.push_str("\n");
                }
            }
        }
    }

    fn transform_operations(&self, out: &mut String, ops_bytes: &[u8]) {
        // Transform operation methods
        let ops_str = String::from_utf8_lossy(ops_bytes);
        out.push_str("    # Operations\n");
        
        for line in ops_str.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
                continue;
            }
            
            // Detect operation definitions
            if let Some(paren_pos) = trimmed.find('(') {
                let op_name = trimmed[..paren_pos].trim();
                if !op_name.is_empty() && op_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    out.push_str(&format!("    def {}(self):\n", op_name));
                    out.push_str("        # Operation implementation\n");
                    out.push_str("        pass\n");
                    out.push_str("\n");
                }
            }
        }
    }

    fn transform_domain(&self, out: &mut String, domain_bytes: &[u8]) {
        // Transform domain variables to class attributes
        out.push_str("    # Domain variables initialized in __init__\n");
    }

    fn transform_to_typescript(&self, bytes: &[u8], start: usize, end: usize, sys_name: &str, arc: &Arcanum) -> String {
        let mut out = String::new();
        
        // Generate TypeScript class
        out.push_str(&format!("class {} {{\n", sys_name));
        out.push_str("    private _state: string;\n");
        out.push_str("    private _stack: string[];\n");
        out.push_str("\n");
        
        out.push_str("    constructor() {\n");
        out.push_str(&format!("        this._state = \"{}\";\n", 
            crate::frame_c::v4::find_start_state_name(arc, sys_name).unwrap_or_else(|| "A".to_string())));
        out.push_str("        this._stack = [];\n");
        out.push_str("    }\n");
        out.push_str("\n");
        
        // Add Frame machinery methods
        out.push_str("    private _frame_transition(nextState: string): void {\n");
        out.push_str("        this._state = nextState;\n");
        out.push_str("    }\n");
        out.push_str("\n");
        
        out.push_str("    private _frame_stack_push(): void {\n");
        out.push_str("        this._stack.push(this._state);\n");
        out.push_str("    }\n");
        out.push_str("\n");
        
        out.push_str("    private _frame_stack_pop(): void {\n");
        out.push_str("        const prevState = this._stack.pop();\n");
        out.push_str("        if (prevState) {\n");
        out.push_str("            this._frame_transition(prevState);\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
        
        out.push_str("}\n");
        
        out
    }

    fn transform_to_rust(&self, bytes: &[u8], start: usize, end: usize, sys_name: &str, arc: &Arcanum) -> String {
        let mut out = String::new();
        
        // Generate Rust struct
        out.push_str(&format!("struct {} {{\n", sys_name));
        out.push_str("    state: String,\n");
        out.push_str("    stack: Vec<String>,\n");
        out.push_str("}\n");
        out.push_str("\n");
        
        out.push_str(&format!("impl {} {{\n", sys_name));
        out.push_str("    fn new() -> Self {\n");
        out.push_str(&format!("        Self {{\n            state: \"{}\".to_string(),\n", 
            crate::frame_c::v4::find_start_state_name(arc, sys_name).unwrap_or_else(|| "A".to_string())));
        out.push_str("            stack: Vec::new(),\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("\n");
        
        // Add Frame machinery methods
        out.push_str("    fn _frame_transition(&mut self, next_state: &str) {\n");
        out.push_str("        self.state = next_state.to_string();\n");
        out.push_str("    }\n");
        out.push_str("\n");
        
        out.push_str("    fn _frame_stack_push(&mut self) {\n");
        out.push_str("        self.stack.push(self.state.clone());\n");
        out.push_str("    }\n");
        out.push_str("\n");
        
        out.push_str("    fn _frame_stack_pop(&mut self) {\n");
        out.push_str("        if let Some(prev_state) = self.stack.pop() {\n");
        out.push_str("            self._frame_transition(&prev_state);\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
        
        out.push_str("}\n");
        
        out
    }
}
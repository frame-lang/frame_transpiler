// Frame v4 Code Generator - Native code generation
//
// Generates pure native code with no Frame runtime dependencies

use super::ast::*;
use super::TargetLanguage;
use super::error::ErrorsAcc;
use super::mir::{MirItem, Region};

pub fn generate(ast: &SystemAst, target: TargetLanguage) -> Result<String, ErrorsAcc> {
    match target {
        TargetLanguage::Python => {
            let gen = PythonGenerator::new();
            gen.generate(ast)
        }
        TargetLanguage::TypeScript => {
            let gen = TypeScriptGenerator::new();
            gen.generate(ast)
        }
        TargetLanguage::Rust => {
            let gen = RustGenerator::new();
            gen.generate(ast)
        }
        _ => Err(ErrorsAcc::from_error(format!("Target language {:?} not yet implemented", target))),
    }
}

pub fn generate_source_map(ast: &SystemAst, code: &str, file_path: &str) -> String {
    // TODO: Implement source map generation
    format!("// Source map for {} -> generated code", file_path)
}

// Python code generator
struct PythonGenerator;

impl PythonGenerator {
    fn new() -> Self {
        Self
    }

    fn generate(&self, ast: &SystemAst) -> Result<String, ErrorsAcc> {
        let mut output = String::new();
        
        // Add imports
        for import in &ast.native_imports {
            output.push_str(import);
            output.push('\n');
        }
        if !ast.native_imports.is_empty() {
            output.push('\n');
        }
        
        // Add annotations
        for annotation in &ast.annotations {
            match annotation {
                Annotation::Native { content } => {
                    output.push_str(content);
                    output.push('\n');
                }
                Annotation::Frame { name, .. } => {
                    if name != "target" {
                        output.push_str(&format!("# @@{}\n", name));
                    }
                }
            }
        }
        
        // Generate class
        output.push_str(&format!("class {}:\n", ast.name));
        
        // Generate __init__
        output.push_str("    def __init__(self");
        
        // Add system parameters to __init__
        for param in &ast.params.domain_params {
            output.push_str(&format!(", {}", param.name));
        }
        for param in &ast.params.start_state_params {
            output.push_str(&format!(", {}", param.name));
        }
        for param in &ast.params.enter_params {
            output.push_str(&format!(", {}", param.name));
        }
        
        output.push_str("):\n");
        
        // Initialize state
        output.push_str("        self._state = None\n");
        output.push_str("        self._state_stack = []\n");
        
        // Initialize domain variables
        if let Some(domain) = &ast.domain {
            for var in &domain.variables {
                if let Some(init) = &var.initializer {
                    output.push_str(&format!("        self.{} = {}\n", var.name, init));
                } else {
                    output.push_str(&format!("        self.{} = None\n", var.name));
                }
            }
        }
        
        // Initialize domain params
        for param in &ast.params.domain_params {
            output.push_str(&format!("        self.{} = {}\n", param.name, param.name));
        }
        
        // Transition to initial state
        if let Some(initial_state) = ast.initial_state() {
            output.push_str(&format!("        self._transition_to_{}(", initial_state.name));
            
            // Pass start state params
            for (i, param) in ast.params.start_state_params.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                output.push_str(&param.name);
            }
            output.push_str(")\n");
            
            // Call enter handler if enter params exist
            if !ast.params.enter_params.is_empty() {
                output.push_str("        self._enter_state(");
                for (i, param) in ast.params.enter_params.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&param.name);
                }
                output.push_str(")\n");
            }
        }
        
        output.push('\n');
        
        // Generate interface methods
        if let Some(interface) = &ast.interface {
            for method in &interface.methods {
                output.push_str(&format!("    def {}(self", method.name));
                
                for param in &method.params {
                    output.push_str(&format!(", {}", param.name));
                    if let Some(type_hint) = &param.type_hint {
                        output.push_str(&format!(": {}", type_hint));
                    }
                }
                
                output.push(')');
                if let Some(return_type) = &method.return_type {
                    output.push_str(&format!(" -> {}", return_type));
                }
                output.push_str(":\n");
                
                // Generate dispatch logic
                output.push_str("        self._system_return = None\n");
                output.push_str(&format!("        self._dispatch('{}', locals())\n", method.name));
                output.push_str("        return self._system_return\n");
                output.push('\n');
            }
        }
        
        // Generate operations methods
        if let Some(operations) = &ast.operations {
            for method in &operations.methods {
                if method.is_static {
                    output.push_str("    @staticmethod\n");
                    output.push_str(&format!("    def {}(", method.name));
                    let mut first = true;
                    for param in &method.params {
                        if !first {
                            output.push_str(", ");
                        }
                        output.push_str(&param.name);
                        if let Some(type_hint) = &param.type_hint {
                            output.push_str(&format!(": {}", type_hint));
                        }
                        first = false;
                    }
                } else {
                    output.push_str(&format!("    def {}(self", method.name));
                    for param in &method.params {
                        output.push_str(&format!(", {}", param.name));
                        if let Some(type_hint) = &param.type_hint {
                            output.push_str(&format!(": {}", type_hint));
                        }
                    }
                }
                
                output.push(')');
                if let Some(return_type) = &method.return_type {
                    output.push_str(&format!(" -> {}", return_type));
                }
                output.push_str(":\n");
                
                // Generate method body using MIR
                let body = method.mir_block.generate_code(|item| self.expand_frame_item(item));
                if !body.trim().is_empty() {
                    for line in body.lines() {
                        if !line.is_empty() {
                            output.push_str("        ");
                            output.push_str(line);
                        }
                        output.push('\n');
                    }
                } else {
                    output.push_str("        pass\n");
                }
                output.push('\n');
            }
        }
        
        // Generate dispatch method
        output.push_str("    def _dispatch(self, event, args):\n");
        output.push_str("        handler = getattr(self, f'_handle_{self._state}_{event}', None)\n");
        output.push_str("        if handler:\n");
        output.push_str("            # Remove 'self' from args\n");
        output.push_str("            args = {k: v for k, v in args.items() if k != 'self'}\n");
        output.push_str("            handler(**args)\n");
        output.push('\n');
        
        // Generate state handlers
        if let Some(machine) = &ast.machine {
            for state in &machine.states {
                // Generate transition method
                output.push_str(&format!("    def _transition_to_{}(self", state.name));
                for param in &state.params {
                    output.push_str(&format!(", {}", param.name));
                }
                output.push_str("):\n");
                
                // Exit current state
                output.push_str("        if self._state:\n");
                output.push_str("            exit_handler = getattr(self, f'_exit_{self._state}', None)\n");
                output.push_str("            if exit_handler:\n");
                output.push_str("                exit_handler()\n");
                
                // Update state
                output.push_str(&format!("        self._state = '{}'\n", state.name));
                
                // Store state params if needed
                for param in &state.params {
                    output.push_str(&format!("        self._state_param_{} = {}\n", param.name, param.name));
                }
                
                output.push('\n');
                
                // Generate event handlers
                for handler in &state.handlers {
                    match handler.handler_type {
                        HandlerType::Event => {
                            if let Some(event_name) = &handler.name {
                                output.push_str(&format!("    def _handle_{}_{}(self", state.name, event_name));
                                
                                for param in &handler.params {
                                    output.push_str(&format!(", {}", param.name));
                                }
                                output.push_str("):\n");
                                
                                // Generate handler body using MIR
                                let body = handler.mir_block.generate_code(|item| self.expand_frame_item(item));
                                if !body.trim().is_empty() {
                                    for line in body.lines() {
                                        if !line.is_empty() {
                                            output.push_str("        ");
                                            output.push_str(line);
                                        }
                                        output.push('\n');
                                    }
                                } else {
                                    output.push_str("        pass\n");
                                }
                                
                                output.push('\n');
                            }
                        }
                        HandlerType::Enter => {
                            output.push_str(&format!("    def _enter_{}(self", state.name));
                            for param in &handler.params {
                                output.push_str(&format!(", {}", param.name));
                            }
                            output.push_str("):\n");
                            
                            // Generate enter handler body using MIR
                            let body = handler.mir_block.generate_code(|item| self.expand_frame_item(item));
                            if !body.trim().is_empty() {
                                for line in body.lines() {
                                    if !line.is_empty() {
                                        output.push_str("        ");
                                        output.push_str(line);
                                    }
                                    output.push('\n');
                                }
                            } else {
                                output.push_str("        pass\n");
                            }
                            
                            output.push('\n');
                        }
                        HandlerType::Exit => {
                            output.push_str(&format!("    def _exit_{}(self):\n", state.name));
                            
                            // Generate exit handler body using MIR
                            let body = handler.mir_block.generate_code(|item| self.expand_frame_item(item));
                            if !body.trim().is_empty() {
                                for line in body.lines() {
                                    if !line.is_empty() {
                                        output.push_str("        ");
                                        output.push_str(line);
                                    }
                                    output.push('\n');
                                }
                            } else {
                                output.push_str("        pass\n");
                            }
                            
                            output.push('\n');
                        }
                    }
                }
            }
        }
        
        // Generate actions
        if let Some(actions) = &ast.actions {
            for method in &actions.methods {
                output.push_str(&format!("    def _{}(self", method.name));
                
                for param in &method.params {
                    output.push_str(&format!(", {}", param.name));
                    if let Some(type_hint) = &param.type_hint {
                        output.push_str(&format!(": {}", type_hint));
                    }
                }
                
                output.push(')');
                if let Some(return_type) = &method.return_type {
                    output.push_str(&format!(" -> {}", return_type));
                }
                output.push_str(":\n");
                
                // Generate actions method body using MIR
                let body = method.mir_block.generate_code(|item| self.expand_frame_item(item));
                if !body.trim().is_empty() {
                    for line in body.lines() {
                        if !line.is_empty() {
                            output.push_str("        ");
                            output.push_str(line);
                        }
                        output.push('\n');
                    }
                } else {
                    output.push_str("        pass\n");
                }
                
                output.push('\n');
            }
        }
        
        Ok(output)
    }

    fn expand_frame_item(&self, item: &MirItem) -> String {
        match item {
            MirItem::Transition { target, state_args, .. } => {
                let mut result = format!("self._transition_to_{}(", target);
                for (i, arg) in state_args.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(arg);
                }
                result.push_str(")\n");
                result
            }
            MirItem::Forward { .. } => {
                // Forward to interface - not yet implemented
                "# => $^ (forward not yet implemented)\n".to_string()
            }
            MirItem::StackPush { .. } => {
                "self._state_stack.append(self._state)\n".to_string()
            }
            MirItem::StackPop { .. } => {
                "if self._state_stack:\n    prev_state = self._state_stack.pop()\n    self._state = prev_state\n".to_string()
            }
            MirItem::SystemReturn { expression, .. } => {
                format!("self._system_return = {}\n", expression)
            }
        }
    }

    fn generate_frame_statement(&self, output: &mut String, stmt: &FrameStatement) {
        match stmt {
            FrameStatement::Transition { target, args, .. } => {
                output.push_str(&format!("        self._transition_to_{}(", target));
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(arg);
                }
                output.push_str(")\n");
            }
            FrameStatement::ChangeState { target, args, .. } => {
                // Change state is same as transition in v4 (no :> operator)
                output.push_str(&format!("        self._transition_to_{}(", target));
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(arg);
                }
                output.push_str(")\n");
            }
            FrameStatement::StackPush { .. } => {
                output.push_str("        self._state_stack.append(self._state)\n");
            }
            FrameStatement::StackPop { .. } => {
                output.push_str("        if self._state_stack:\n");
                output.push_str("            prev_state = self._state_stack.pop()\n");
                output.push_str("            self._state = prev_state\n");
            }
            FrameStatement::SystemReturn { expression, .. } => {
                if let Some(expr) = expression {
                    output.push_str(&format!("        self._system_return = {}\n", expr));
                }
            }
            _ => {}
        }
    }
}

// TypeScript code generator
struct TypeScriptGenerator;

impl TypeScriptGenerator {
    fn new() -> Self {
        Self
    }

    fn generate(&self, _ast: &SystemAst) -> Result<String, ErrorsAcc> {
        // TODO: Implement TypeScript generation
        Ok("// TypeScript generation not yet implemented\n".to_string())
    }
}

// Rust code generator
struct RustGenerator;

impl RustGenerator {
    fn new() -> Self {
        Self
    }

    fn generate(&self, _ast: &SystemAst) -> Result<String, ErrorsAcc> {
        // TODO: Implement Rust generation
        Ok("// Rust generation not yet implemented\n".to_string())
    }
}
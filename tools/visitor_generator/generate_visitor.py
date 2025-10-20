#!/usr/bin/env python3
"""
Frame Transpiler Visitor Generator

This tool generates language visitors for the Frame transpiler using templates
and language-specific configurations. It extracts common patterns from existing
visitors and generates new ones for target languages.

Usage:
    python3 generate_visitor.py <language> [options]
    
Examples:
    python3 generate_visitor.py c
    python3 generate_visitor.py cpp --thread-safe
    python3 generate_visitor.py java --output-dir ./generated
"""

import os
import sys
import argparse
import toml
from pathlib import Path
from typing import Dict, Any, List
import json

class VisitorGenerator:
    def __init__(self, config_path: str, template_dir: str):
        self.config_path = Path(config_path)
        self.template_dir = Path(template_dir)
        self.config = self._load_config()
        
    def _load_config(self) -> Dict[str, Any]:
        """Load language configurations from TOML file."""
        if not self.config_path.exists():
            raise FileNotFoundError(f"Config file not found: {self.config_path}")
            
        with open(self.config_path, 'r') as f:
            return toml.load(f)
    
    def _load_template(self, template_name: str) -> str:
        """Load a template file."""
        template_path = self.template_dir / template_name
        if not template_path.exists():
            raise FileNotFoundError(f"Template not found: {template_path}")
            
        with open(template_path, 'r') as f:
            return f.read()
    
    def _get_language_config(self, language: str) -> Dict[str, Any]:
        """Get configuration for a specific language."""
        if language not in self.config['languages']:
            available = list(self.config['languages'].keys())
            raise ValueError(f"Language '{language}' not supported. Available: {available}")
            
        return self.config['languages'][language]
    
    def _snake_case(self, name: str) -> str:
        """Convert PascalCase to snake_case."""
        result = ""
        for i, char in enumerate(name):
            if char.isupper() and i > 0:
                result += "_"
            result += char.lower()
        return result
    
    def _pascal_case(self, name: str) -> str:
        """Convert snake_case to PascalCase."""
        return ''.join(word.capitalize() for word in name.split('_'))
    
    def _generate_type_mappings(self, lang_config: Dict[str, Any]) -> List[Dict[str, str]]:
        """Generate Frame to target language type mappings."""
        type_mappings = []
        if 'types' in lang_config:
            for frame_type, target_type in lang_config['types'].items():
                type_mappings.append({
                    'frame_type': frame_type,
                    'target_type': target_type
                })
        return type_mappings
    
    def _generate_imports(self, lang_config: Dict[str, Any]) -> List[str]:
        """Generate import statements for the language."""
        imports = []
        
        # Add base imports/includes
        for import_key in ['imports', 'includes', 'usings']:
            if import_key in lang_config and 'base' in lang_config[import_key]:
                imports.extend(lang_config[import_key]['base'])
                
        return imports
    
    def _generate_helper_methods(self, language: str, lang_config: Dict[str, Any]) -> List[str]:
        """Generate language-specific helper methods."""
        helpers = []
        
        # Generate utility methods based on language characteristics
        if language == 'c':
            helpers.extend([
                self._generate_c_memory_helpers(),
                self._generate_c_string_helpers(),
            ])
        elif language == 'cpp':
            helpers.extend([
                self._generate_cpp_smart_pointer_helpers(),
                self._generate_cpp_container_helpers(),
            ])
        elif language == 'java':
            helpers.extend([
                self._generate_java_collection_helpers(),
            ])
        elif language == 'go':
            helpers.extend([
                self._generate_go_interface_helpers(),
            ])
        elif language == 'csharp':
            helpers.extend([
                self._generate_csharp_linq_helpers(),
            ])
            
        return helpers
    
    def _generate_c_memory_helpers(self) -> str:
        """Generate C-specific memory management helpers."""
        return """
    // C Memory Management Helpers
    void* safe_malloc(size_t size) {
        void* ptr = malloc(size);
        if (!ptr) {
            fprintf(stderr, "Memory allocation failed\\n");
            exit(1);
        }
        return ptr;
    }
    
    void safe_free(void** ptr) {
        if (ptr && *ptr) {
            free(*ptr);
            *ptr = NULL;
        }
    }"""
    
    def _generate_cpp_smart_pointer_helpers(self) -> str:
        """Generate C++ smart pointer helpers."""
        return """
    // C++ Smart Pointer Helpers
    template<typename T>
    std::shared_ptr<T> make_shared_safe(T&& value) {
        return std::make_shared<T>(std::forward<T>(value));
    }"""
    
    def _generate_java_collection_helpers(self) -> str:
        """Generate Java collection helpers."""
        return """
    // Java Collection Helpers  
    private <T> List<T> createList() {
        return new ArrayList<>();
    }
    
    private <K, V> Map<K, V> createMap() {
        return new HashMap<>();
    }"""
    
    def _generate_go_interface_helpers(self) -> str:
        """Generate Go interface helpers."""
        return """
// Go Interface Helpers
func (v *GoVisitor) createEvent(name string, params map[string]interface{}) Event {
    return Event{
        Name: name,
        Params: params,
    }
}"""
    
    def _generate_csharp_linq_helpers(self) -> str:
        """Generate C# LINQ helpers."""
        return """
    // C# LINQ Helpers
    private List<T> CreateList<T>() {
        return new List<T>();
    }
    
    private Dictionary<TKey, TValue> CreateDictionary<TKey, TValue>() {
        return new Dictionary<TKey, TValue>();
    }"""
    
    def _generate_visitor_methods(self, language: str, lang_config: Dict[str, Any]) -> List[str]:
        """Generate language-specific visitor method implementations."""
        methods = []
        
        # Generate standard visitor methods with language-specific implementations
        methods.append(self._generate_interface_method_visitor(language, lang_config))
        methods.append(self._generate_machine_block_visitor(language, lang_config))
        methods.append(self._generate_state_visitor(language, lang_config))
        methods.append(self._generate_action_visitor(language, lang_config))
        methods.append(self._generate_operation_visitor(language, lang_config))
        
        return methods
    
    def _generate_interface_method_visitor(self, language: str, lang_config: Dict[str, Any]) -> str:
        """Generate interface method visitor implementation."""
        return f"""
    fn visit_interface_method_node(&mut self, method: &InterfaceMethodNode) {{
        let method_name = &method.name;
        
        // Build parameter list
        let mut params = Vec::new();
        if let Some(param_nodes) = &method.params {{
            for param in param_nodes {{
                let param_type = if let Some(type_node) = &param.param_type_opt {{
                    self.frame_type_to_{self._snake_case(language)}(&type_node.get_type_str())
                }} else {{
                    "{lang_config.get('types', {}).get('void', 'void')}".to_string()
                }};
                params.push(format!("{{}}: {{}}", param.param_name, param_type));
            }}
        }}
        let params_str = params.join(", ");
        
        // Determine return type
        let return_type = if let Some(return_type_node) = &method.return_type_opt {{
            self.frame_type_to_{self._snake_case(language)}(&return_type_node.get_type_str())
        }} else {{
            "{lang_config.get('types', {}).get('void', 'void')}".to_string()
        }};
        
        // Generate method signature
        self.builder.writeln(&format!("{lang_config.get('methods', {}).get('public_prefix', 'pub fn')} {{}}({{}}){lang_config.get('methods', {}).get('return_arrow', ' -> ')}{{}} {{{{", 
            method_name,
            if params_str.is_empty() {{ "{lang_config.get('methods', {}).get('self_param', '&mut self')}".to_string() }} else {{ format!("{lang_config.get('methods', {}).get('self_param', '&mut self')}, {{}}", params_str) }},
            return_type
        ));
        self.builder.indent();
        
        // Basic implementation
        self.builder.writeln("// TODO: Implement interface method");
        
        // Provide default return value
        if return_type != "{lang_config.get('types', {}).get('void', 'void')}" {{
            let default_value = self.get_default_value_for_type(&return_type);
            self.builder.writeln(&format!("{{}}", default_value));
        }}
        
        self.builder.dedent();
        self.builder.writeln("}}");
        self.builder.writeln("");
    }}"""
    
    def _generate_machine_block_visitor(self, language: str, lang_config: Dict[str, Any]) -> str:
        """Generate machine block visitor implementation."""
        return """
    fn visit_machine_block_node(&mut self, machine_block: &MachineBlockNode) {
        self.builder.writeln("");
        self.builder.writeln("// ==================== State Machine Logic ==================== //");
        self.builder.writeln("");
        
        // Generate event dispatcher method
        self.generate_event_dispatcher(machine_block);
        
        // Process each state and its handlers
        for state_rcref in &machine_block.states {
            let state_node = state_rcref.borrow();
            self.visit_state_node(&state_node);
        }
    }"""
    
    def _generate_state_visitor(self, language: str, lang_config: Dict[str, Any]) -> str:
        """Generate state visitor implementation."""
        return """
    fn visit_state_node(&mut self, state_node: &StateNode) {
        let state_name = &state_node.name;
        self.builder.writeln(&format!("// State: {}", state_name));
        
        // Generate state handler method
        self.builder.writeln(&format!("fn handle_{}(&mut self, event: &str) -> Option<String> {{", 
            self.to_snake_case(state_name)));
        self.builder.indent();
        
        self.builder.writeln("match event {");
        self.builder.indent();
        
        // Process event handlers
        for handler_rcref in &state_node.evt_handlers_rcref {
            let handler = handler_rcref.borrow();
            self.visit_event_handler_node(&handler);
        }
        
        self.builder.writeln("_ => None,");
        self.builder.dedent();
        self.builder.writeln("}");
        
        self.builder.dedent();
        self.builder.writeln("}");
        self.builder.writeln("");
    }"""
    
    def _generate_action_visitor(self, language: str, lang_config: Dict[str, Any]) -> str:
        """Generate action visitor implementation."""
        return """
    fn visit_action_node(&mut self, action_node: &ActionNode) {
        self.generate_action_method(action_node);
    }"""
    
    def _generate_operation_visitor(self, language: str, lang_config: Dict[str, Any]) -> str:
        """Generate operation visitor implementation."""
        return """
    fn visit_operation_node(&mut self, operation_node: &OperationNode) {
        self.generate_operation_method(operation_node);
    }"""
    
    def generate_visitor(self, language: str, options: Dict[str, Any] = None) -> str:
        """Generate a complete visitor for the specified language."""
        if options is None:
            options = {}
            
        lang_config = self._get_language_config(language)
        
        # Prepare template variables
        template_vars = {
            'language_name': lang_config['name'],
            'language_snake': self._snake_case(language),
            'visitor_name': lang_config['visitor_name'],
            'indent': lang_config['indent'],
            'version': '1.0.0',  # Could be made configurable
            
            # Type mappings
            'type_mappings': self._generate_type_mappings(lang_config),
            
            # Imports/includes
            'imports': self._generate_imports(lang_config),
            'base_imports': self._generate_imports(lang_config),
            
            # Helper methods
            'helper_methods': self._generate_helper_methods(language, lang_config),
            
            # Visitor methods
            'visitor_methods': self._generate_visitor_methods(language, lang_config),
            
            # Configuration
            'has_config': language in ['rust', 'cpp'],  # Languages that need config structs
            'config_name': f"{self._pascal_case(language)}Config",
            
            # Language-specific features
            'has_thread_safe': language in ['rust', 'cpp', 'java', 'csharp'],
            'has_conditional_imports': language in ['rust', 'cpp'],
            
            # Generation methods
            'type_generation_calls': [
                {'method_name': 'generate_frame_event_enum'},
                {'method_name': 'generate_state_enum'},
                {'method_name': 'generate_context_struct'},
                {'method_name': 'generate_system_struct'},
            ],
            
            'system_generation': [
                'self.builder.writeln(&format!("impl {} {{", self.system_name));',
                'self.builder.indent();',
                'self.generate_constructor();',
            ],
            
            'system_generation_end': [
                'self.builder.dedent();',
                'self.builder.writeln("}");',
            ],
        }
        
        # Load and render template
        template = self._load_template('visitor_template.rs')
        
        # Simple template rendering (could use a proper template engine)
        result = template
        for key, value in template_vars.items():
            if isinstance(value, list):
                # Handle list rendering
                placeholder = f"{{{{{key}}}}}"
                if placeholder in result:
                    rendered_list = '\n'.join(str(item) for item in value)
                    result = result.replace(placeholder, rendered_list)
            else:
                # Handle simple substitutions
                result = result.replace(f"{{{{{key}}}}}", str(value))
        
        return result
    
    def save_visitor(self, language: str, output_dir: str, options: Dict[str, Any] = None):
        """Generate and save a visitor to file."""
        visitor_code = self.generate_visitor(language, options)
        
        lang_config = self._get_language_config(language)
        filename = f"{self._snake_case(language)}_visitor.rs"
        output_path = Path(output_dir) / filename
        
        # Ensure output directory exists
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output_path, 'w') as f:
            f.write(visitor_code)
            
        print(f"Generated {lang_config['name']} visitor: {output_path}")
        return output_path

def main():
    parser = argparse.ArgumentParser(description='Generate Frame transpiler language visitors')
    parser.add_argument('language', help='Target language (c, cpp, java, go, csharp)')
    parser.add_argument('--config', default='language_configs.toml', help='Language configuration file')
    parser.add_argument('--templates', default='templates', help='Template directory')
    parser.add_argument('--output-dir', default='../../framec/src/frame_c/visitors', help='Output directory')
    parser.add_argument('--thread-safe', action='store_true', help='Generate thread-safe version')
    parser.add_argument('--dry-run', action='store_true', help='Print generated code without saving')
    
    args = parser.parse_args()
    
    try:
        # Get the directory of this script
        script_dir = Path(__file__).parent
        config_path = script_dir / args.config
        template_dir = script_dir / args.templates
        
        generator = VisitorGenerator(config_path, template_dir)
        
        options = {
            'thread_safe': args.thread_safe
        }
        
        if args.dry_run:
            # Just print the generated code
            visitor_code = generator.generate_visitor(args.language, options)
            print(visitor_code)
        else:
            # Save to file
            output_path = generator.save_visitor(args.language, args.output_dir, options)
            print(f"Successfully generated visitor: {output_path}")
            
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()
# Visitor Common Patterns Analysis

## Core Structural Patterns

All Frame language visitors follow these common patterns:

### 1. **Visitor Struct Pattern**
```rust
pub struct {LanguageVisitor} {
    // Core configuration
    config: FrameConfig,
    
    // Code generation
    builder: CodeBuilder,
    
    // Symbol tracking  
    symbol_config: SymbolConfig,
    arcanum: Vec<Arcanum>,
    
    // Current context
    current_state_name_opt: Option<String>,
    current_event_ret_type: String,
    current_class_name_opt: Option<String>,
    
    // System metadata
    system_name: String,
    interface_methods: {LanguageSpecificType},
    domain_variables: {LanguageSpecificType},
    current_handler_params: {LanguageSpecificType},
    action_names: {LanguageSpecificType},
    operation_names: {LanguageSpecificType},
    
    // Generation flags
    is_generating_interface_method: bool,
    is_generating_action: bool,
    is_generating_operation: bool,
}
```

### 2. **Constructor Pattern**
```rust
impl {LanguageVisitor} {
    pub fn new(
        arcanum: Vec<Arcanum>,
        symbol_config: SymbolConfig,
        config: FrameConfig,
        comments: Vec<Token>,
    ) -> Self {
        Self {
            // Initialize all fields with language-specific defaults
            builder: CodeBuilder::new("{indent_string}"),
            // ... other initializations
        }
    }
}
```

### 3. **Core Visitor Methods Pattern**
All visitors implement the same set of core methods:

```rust
impl AstVisitor for {LanguageVisitor} {
    fn visit_system_node(&mut self, node: &SystemNode) {
        // 1. Set system context
        self.system_name = node.name.clone();
        
        // 2. Generate language-specific imports/headers
        self.generate_imports();
        
        // 3. First pass: collect metadata
        self.collect_domain_variables(node.domain_block_node_opt);
        self.collect_states(node.machine_block_node_opt);
        
        // 4. Generate language-specific type definitions
        self.generate_types();
        
        // 5. Generate system struct/class
        self.generate_system_struct();
        
        // 6. Visit blocks in order
        self.visit_interface_block(node.interface_block_node_opt);
        self.visit_machine_block(node.machine_block_node_opt);
        self.visit_actions_block(node.actions_block_node_opt);
        self.visit_operations_block(node.operations_block_node_opt);
    }
    
    fn visit_interface_method_node(&mut self, method: &InterfaceMethodNode) {
        // 1. Build parameter list with language-specific types
        // 2. Determine return type
        // 3. Generate method signature
        // 4. Generate method body (skeleton)
    }
    
    fn visit_machine_block_node(&mut self, machine: &MachineBlockNode) {
        // 1. Generate event dispatcher
        // 2. Visit each state
    }
    
    fn visit_state_node(&mut self, state: &StateNode) {
        // 1. Generate state handler method
        // 2. Process event handlers
    }
    
    fn visit_action_node(&mut self, action: &ActionNode) {
        // Generate private action method
    }
    
    fn visit_operation_node(&mut self, operation: &OperationNode) {
        // Generate public operation method
    }
}
```

## Language-Specific Variations

### Type System Patterns
- **Python**: Dynamic typing, minimal type annotations
- **TypeScript**: Static typing with interfaces, union types
- **Rust**: Strong static typing with ownership, `Rc<RefCell<T>>`

### Memory Management Patterns
- **Python**: Garbage collected, simple references
- **TypeScript**: Garbage collected, object references  
- **Rust**: Manual memory management with RAII, `Rc<RefCell<T>>` for shared state

### State Management Patterns
- **Python**: Class-based with instance variables
- **TypeScript**: Class-based with private fields
- **Rust**: Struct-based with container types

### Method Generation Patterns
- **Python**: `def method_name(self, params):`
- **TypeScript**: `public methodName(params): ReturnType {`
- **Rust**: `pub fn method_name(&mut self, params) -> ReturnType {`

## Templatable Components

1. **Imports/Headers** - Language-specific module imports
2. **Type Definitions** - Enums, structs, classes for states and events
3. **Constructor Logic** - System initialization patterns
4. **Method Signatures** - Parameter handling and return types
5. **State Machine Logic** - Event dispatching and state transitions
6. **Memory Management** - Language-specific container types
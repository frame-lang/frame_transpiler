//! System Code Generation from Frame AST
//!
//! This module transforms Frame AST (SystemAst) into CodegenNode for emission
//! by language-specific backends.
//!
//! Uses the "oceans model" - native code is preserved exactly, Frame segments
//! are replaced with generated code using the splicer.

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::frame_ast::{
    SystemAst, MachineAst,
    ActionAst, OperationAst, Type,
    Expression, Literal, BinaryOp, UnaryOp, StateVarAst,
    InterfaceMethod, MethodParam, Span,
};
use crate::frame_c::v4::arcanum::{Arcanum, HandlerEntry};
use crate::frame_c::v4::splice::Splicer;
use crate::frame_c::v4::native_region_scanner::{
    NativeRegionScanner, Region, FrameSegmentKind,
    python::NativeRegionScannerPy,
    typescript::NativeRegionScannerTs,
    rust::NativeRegionScannerRust,
    csharp::NativeRegionScannerCs,
    c::NativeRegionScannerC,
    cpp::NativeRegionScannerCpp,
    java::NativeRegionScannerJava,
};
use super::ast::*;
use super::backend::get_backend;

/// Context for handler expansion - tracks parent state and event for HSM forwarding
#[derive(Clone, Default)]
struct HandlerContext {
    pub system_name: String,
    pub state_name: String,
    pub event_name: String,
    pub parent_state: Option<String>,
    /// Set of defined system names in the module (for @@System() validation)
    pub defined_systems: std::collections::HashSet<String>,
    /// True if we're in a state handler that has __sv_comp available for HSM state var access
    pub use_sv_comp: bool,
}

/// Generate a complete CodegenNode for a Frame system
///
/// # Arguments
/// * `system` - The parsed Frame system AST
/// * `arcanum` - Symbol table for the system (used for handler info and validation)
/// * `lang` - Target language for code generation
/// * `source` - Original source bytes (used to extract native code via spans)
pub fn generate_system(system: &SystemAst, arcanum: &Arcanum, lang: TargetLanguage, source: &[u8]) -> CodegenNode {
    let backend = get_backend(lang);
    let syntax = backend.class_syntax();

    // Generate fields
    let fields = generate_fields(system, &syntax);

    // Generate methods
    let mut methods = Vec::new();

    // Constructor
    methods.push(generate_constructor(system, &syntax));

    // Frame machinery (transition, state management)
    methods.extend(generate_frame_machinery(system, &syntax, lang));

    // Interface wrappers
    methods.extend(generate_interface_wrappers(system, &syntax));

    // Check if system has states with state variables (for Rust compartment-based push/pop)
    let has_state_vars = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| !s.state_vars.is_empty()))
        .unwrap_or(false);

    // State handlers - use enhanced Arcanum for clean iteration
    if let Some(ref machine) = system.machine {
        methods.extend(generate_state_handlers_via_arcanum(&system.name, machine, arcanum, source, lang, has_state_vars));
    }

    // Actions - extract native code from source using spans
    for action in &system.actions {
        methods.push(generate_action(action, &syntax, source));
    }

    // Operations - extract native code from source using spans
    for operation in &system.operations {
        methods.push(generate_operation(operation, &syntax, source));
    }

    // Persistence methods (when @@persist is present)
    if system.persist_attr.is_some() {
        methods.extend(generate_persistence_methods(system, &syntax));
    }

    CodegenNode::Class {
        name: system.name.clone(),
        fields,
        methods,
        base_classes: vec![],
        is_abstract: false,
        derives: vec![],  // Derives not used - we manually build JSON
    }
}

/// Generate class fields for the system
fn generate_fields(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> Vec<Field> {
    let mut fields = Vec::new();
    let compartment_type = format!("{}Compartment", system.name);

    // State stack - for push/pop state operations
    // All languages: stack of compartments (state vars live on compartment.state_context)
    let stack_type = if matches!(syntax.language, TargetLanguage::Rust) {
        format!("Vec<{}Compartment>", system.name)
    } else {
        "List".to_string()
    };
    fields.push(Field::new("_state_stack")
        .with_visibility(Visibility::Private)
        .with_type(&stack_type));

    // Compartment field - canonical compartment architecture for ALL languages
    // Holds the current compartment with state, state_vars, forward_event
    let nullable_compartment_type = if matches!(syntax.language, TargetLanguage::Rust) {
        format!("Option<{}>", compartment_type)
    } else {
        format!("{} | null", compartment_type)
    };
    fields.push(Field::new("__compartment")
        .with_visibility(Visibility::Private)
        .with_type(&compartment_type));

    // Next compartment field - for deferred transition caching in __kernel
    // __transition() sets this, __kernel() processes it after handler returns
    // This field is nullable (None/null when no transition is pending)
    fields.push(Field::new("__next_compartment")
        .with_visibility(Visibility::Private)
        .with_type(&nullable_compartment_type));

    // Context stack for reentrancy - holds FrameContext objects (event ref + return + data)
    // For Rust: Vec<{System}FrameContext>
    // For Python/TypeScript: list/array of FrameContext objects
    let context_stack_type = if matches!(syntax.language, TargetLanguage::Rust) {
        format!("Vec<{}FrameContext>", system.name)
    } else {
        "List".to_string()
    };
    fields.push(Field::new("_context_stack")
        .with_visibility(Visibility::Private)
        .with_type(&context_stack_type));

    // Domain variables (V4: native code pass-through for most languages, C needs parsed types)
    for domain_var in &system.domain {
        // For V4, var_type is Unknown - extract actual type from raw_code for C compatibility
        let type_str = if domain_var.var_type == Type::Unknown && domain_var.raw_code.is_some() {
            // Extract type from native declaration (e.g., "char* last" -> "char*")
            extract_type_from_raw_domain(&domain_var.raw_code, &domain_var.name)
        } else {
            type_to_string(&domain_var.var_type)
        };

        if let Some(ref raw_code) = domain_var.raw_code {
            // V4: Pass through native code verbatim for Python/TypeScript/Rust
            // But also include type annotation for C backend which needs it
            let field = Field::new(&domain_var.name)
                .with_visibility(Visibility::Public)
                .with_type(&type_str)
                .with_raw_code(raw_code);
            fields.push(field);
        } else {
            // Domain vars are public so generated FSMs can be driven externally
            let mut field = Field::new(&domain_var.name)
                .with_visibility(Visibility::Public)
                .with_type(&type_str);

            if let Some(ref init) = &domain_var.initializer {
                field = field.with_initializer(convert_expression(init));
            }

            fields.push(field);
        }
    }

    // Rust state vars now live on compartment.state_context (StateContext enum)
    // No _sv_* struct fields needed

    fields
}

/// Generate Rust runtime types for a system
///
/// This is the public entry point for generating the Frame runtime infrastructure
/// for Rust that matches the Python/TypeScript kernel/router/transition pattern.
///
/// Returns the Rust code for:
/// - FrameEvent struct with message field
/// - Compartment struct with state, state_vars, forward_event fields
/// - Context structs for states with state variables (for typed push/pop)
/// - StateContext enum for typed state variable storage
pub fn generate_rust_compartment_types(system: &SystemAst) -> String {
    generate_rust_runtime_types(system)
}

/// Generate FrameEvent class for Python/TypeScript
///
/// The FrameEvent class is a lean routing object:
/// - _message: string - Event name (e.g., "$>", "<$", "start")
/// - _parameters: dict - Event parameters (positional args as indexed dict)
///
/// Note: _return is NOT on FrameEvent - it's on FrameContext for proper reentrancy
///
/// Returns None for Rust (which uses a different pattern)
pub fn generate_frame_event_class(system: &SystemAst, lang: TargetLanguage) -> Option<CodegenNode> {
    // Rust uses a different pattern - return None
    if matches!(lang, TargetLanguage::Rust) {
        return None;
    }

    let class_name = format!("{}FrameEvent", system.name);

    // Constructor parameters: message and parameters
    let constructor_params = match lang {
        TargetLanguage::Python3 => vec![
            Param::new("message").with_type("str"),
            Param::new("parameters"),
        ],
        TargetLanguage::TypeScript => vec![
            Param::new("message").with_type("string"),
            Param::new("parameters").with_type("Record<string, any> | null"),
        ],
        _ => vec![],
    };

    // Constructor body: initialize fields (no _return - that's on FrameContext)
    let constructor_body = match lang {
        TargetLanguage::Python3 => vec![
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_message"),
                CodegenNode::ident("message"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_parameters"),
                CodegenNode::ident("parameters"),
            ),
        ],
        TargetLanguage::TypeScript => vec![
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_message"),
                CodegenNode::ident("message"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_parameters"),
                CodegenNode::ident("parameters"),
            ),
        ],
        _ => vec![],
    };

    // Fields for TypeScript (Python doesn't need field declarations)
    // Note: no _return field - that's on FrameContext for proper reentrancy
    let fields = if matches!(lang, TargetLanguage::TypeScript) {
        vec![
            Field::new("_message").with_type("string").with_visibility(Visibility::Public),
            Field::new("_parameters").with_type("Record<string, any> | null").with_visibility(Visibility::Public),
        ]
    } else {
        vec![]
    };

    Some(CodegenNode::Class {
        name: class_name,
        fields,
        methods: vec![
            CodegenNode::Constructor {
                params: constructor_params,
                body: constructor_body,
                super_call: None,
            },
        ],
        base_classes: vec![],
        is_abstract: false,
        derives: vec![],
    })
}

/// Generate FrameContext class for Python/TypeScript
///
/// The FrameContext class holds the call context for reentrancy support:
/// - event: FrameEvent - Reference to the interface event (message + parameters)
/// - _return: any - Return value slot (default or None)
/// - _data: dict - Call-scoped data (empty by default)
///
/// Context is pushed when interface is called, popped when it returns.
/// Lifecycle events ($>, <$) use the existing context without push/pop.
///
/// Returns None for Rust (which uses a different pattern)
pub fn generate_frame_context_class(system: &SystemAst, lang: TargetLanguage) -> Option<CodegenNode> {
    // Rust uses a different pattern - return None
    if matches!(lang, TargetLanguage::Rust) {
        return None;
    }

    let class_name = format!("{}FrameContext", system.name);
    let event_class = format!("{}FrameEvent", system.name);

    // Constructor parameters: event and optional default_return
    let constructor_params = match lang {
        TargetLanguage::Python3 => vec![
            Param::new("event").with_type(&event_class),
            Param::new("default_return"),
        ],
        TargetLanguage::TypeScript => vec![
            Param::new("event").with_type(&event_class),
            Param::new("default_return").with_type("any"),
        ],
        _ => vec![],
    };

    // Constructor body: initialize fields
    let constructor_body = match lang {
        TargetLanguage::Python3 => vec![
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "event"),
                CodegenNode::ident("event"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_return"),
                CodegenNode::ident("default_return"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_data"),
                CodegenNode::Dict(vec![]),
            ),
        ],
        TargetLanguage::TypeScript => vec![
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "event"),
                CodegenNode::ident("event"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_return"),
                CodegenNode::ident("default_return"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_data"),
                CodegenNode::Dict(vec![]),
            ),
        ],
        _ => vec![],
    };

    // Fields for TypeScript (Python doesn't need field declarations)
    let fields = if matches!(lang, TargetLanguage::TypeScript) {
        vec![
            Field::new("event").with_type(&event_class).with_visibility(Visibility::Public),
            Field::new("_return").with_type("any").with_visibility(Visibility::Public),
            Field::new("_data").with_type("Record<string, any>").with_visibility(Visibility::Public),
        ]
    } else {
        vec![]
    };

    Some(CodegenNode::Class {
        name: class_name,
        fields,
        methods: vec![
            CodegenNode::Constructor {
                params: constructor_params,
                body: constructor_body,
                super_call: None,
            },
        ],
        base_classes: vec![],
        is_abstract: false,
        derives: vec![],
    })
}

/// Generate Compartment class for Python/TypeScript
///
/// The Compartment class encapsulates all state-related data following the canonical 7-field model:
/// - state: string - Current state identifier
/// - state_args: dict - State parameters ($State(args))
/// - state_vars: dict - State variables ($.varName)
/// - enter_args: dict - Enter transition args (-> (args) $State)
/// - exit_args: dict - Exit transition args ((args) -> $State)
/// - forward_event: Event? - For event forwarding (-> =>)
/// - parent_compartment: Compartment? - For HSM parent state reference
///
/// Returns None for Rust (which uses the specialized enum-of-structs pattern)
pub fn generate_compartment_class(system: &SystemAst, lang: TargetLanguage) -> Option<CodegenNode> {
    // Rust uses a different pattern - return None
    if matches!(lang, TargetLanguage::Rust) {
        return None;
    }

    let class_name = format!("{}Compartment", system.name);

    // Constructor parameters: state and optional parent_compartment
    let constructor_params = match lang {
        TargetLanguage::Python3 => vec![
            Param::new("state").with_type("str"),
            Param::new("parent_compartment").with_default(CodegenNode::null()),
        ],
        TargetLanguage::TypeScript => vec![
            Param::new("state").with_type("string"),
            Param::new("parent_compartment").with_type(&format!("{} | null", class_name)).with_default(CodegenNode::null()),
        ],
        _ => vec![
            Param::new("state").with_type("str"),
        ],
    };

    // Constructor body: initialize all 7 fields
    let constructor_body = match lang {
        TargetLanguage::Python3 => vec![
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "state"),
                CodegenNode::ident("state"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "state_args"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "state_vars"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "enter_args"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "exit_args"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "forward_event"),
                CodegenNode::null(),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "parent_compartment"),
                CodegenNode::ident("parent_compartment"),
            ),
        ],
        TargetLanguage::TypeScript => vec![
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "state"),
                CodegenNode::ident("state"),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "state_args"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "state_vars"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "enter_args"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "exit_args"),
                CodegenNode::Dict(vec![]),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "forward_event"),
                CodegenNode::null(),
            ),
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "parent_compartment"),
                CodegenNode::ident("parent_compartment"),
            ),
        ],
        _ => vec![],
    };

    // Generate copy() method
    let copy_method = generate_compartment_copy_method(&class_name, lang);

    // Build the class
    let methods = vec![
        CodegenNode::Constructor {
            params: constructor_params,
            body: constructor_body,
            super_call: None,
        },
        copy_method,
    ];

    // Fields for TypeScript (Python doesn't need field declarations)
    let fields = if matches!(lang, TargetLanguage::TypeScript) {
        vec![
            Field::new("state").with_type("string").with_visibility(Visibility::Public),
            Field::new("state_args").with_type("Record<string, any>").with_visibility(Visibility::Public),
            Field::new("state_vars").with_type("Record<string, any>").with_visibility(Visibility::Public),
            Field::new("enter_args").with_type("Record<string, any>").with_visibility(Visibility::Public),
            Field::new("exit_args").with_type("Record<string, any>").with_visibility(Visibility::Public),
            Field::new("forward_event").with_type("any").with_visibility(Visibility::Public),
            Field::new("parent_compartment").with_type(&format!("{} | null", class_name)).with_visibility(Visibility::Public),
        ]
    } else {
        vec![]
    };

    Some(CodegenNode::Class {
        name: class_name,
        fields,
        methods,
        base_classes: vec![],
        is_abstract: false,
        derives: vec![],
    })
}

/// Generate the copy() method for Compartment class
fn generate_compartment_copy_method(class_name: &str, lang: TargetLanguage) -> CodegenNode {
    let copy_body = match lang {
        TargetLanguage::Python3 => {
            // Python: c = {Class}Compartment(self.state, self.parent_compartment); c.state_args = self.state_args.copy(); ...
            vec![CodegenNode::NativeBlock {
                code: format!(
                    r#"c = {}(self.state, self.parent_compartment)
c.state_args = self.state_args.copy()
c.state_vars = self.state_vars.copy()
c.enter_args = self.enter_args.copy()
c.exit_args = self.exit_args.copy()
c.forward_event = self.forward_event
return c"#,
                    class_name
                ),
                span: None,
            }]
        }
        TargetLanguage::TypeScript => {
            // TypeScript: const c = new {Class}(this.state, this.parent_compartment); c.state_args = {...this.state_args}; ...
            vec![CodegenNode::NativeBlock {
                code: format!(
                    r#"const c = new {}(this.state, this.parent_compartment);
c.state_args = {{...this.state_args}};
c.state_vars = {{...this.state_vars}};
c.enter_args = {{...this.enter_args}};
c.exit_args = {{...this.exit_args}};
c.forward_event = this.forward_event;
return c;"#,
                    class_name
                ),
                span: None,
            }]
        }
        _ => vec![CodegenNode::comment("copy() not implemented")],
    };

    // Use string annotation for Python to avoid forward reference issues
    let return_type = match lang {
        TargetLanguage::Python3 => format!("'{}'", class_name),  // 'ClassName' forward reference
        _ => class_name.to_string(),
    };

    CodegenNode::Method {
        name: "copy".to_string(),
        params: vec![],
        return_type: Some(return_type),
        body: copy_body,
        is_async: false,
        is_static: false,
        visibility: Visibility::Public,
        decorators: vec![],
    }
}

/// Generate Rust runtime types (FrameEvent and Compartment structs)
///
/// Generates the standard Frame runtime infrastructure for Rust:
/// - FooFrameEvent struct with message field
/// - FooCompartment struct with state and state_vars fields
/// - Context structs for states with state variables (for typed push/pop)
fn generate_rust_runtime_types(system: &SystemAst) -> String {
    let system_name = &system.name;
    let mut code = String::new();

    // Generate FrameEvent struct (lean routing object - message + parameters only)
    // Parameters use Box<dyn Any> for typed storage with downcasting
    code.push_str("#[allow(dead_code)]\n");
    code.push_str(&format!("struct {}FrameEvent {{\n", system_name));
    code.push_str("    message: String,\n");
    code.push_str("    parameters: std::collections::HashMap<String, Box<dyn std::any::Any>>,\n");
    code.push_str("}\n\n");

    // Generate Clone impl manually since Box<dyn Any> doesn't implement Clone
    // For forward_event we only need message, parameters are empty for lifecycle events
    code.push_str(&format!("impl Clone for {}FrameEvent {{\n", system_name));
    code.push_str("    fn clone(&self) -> Self {\n");
    code.push_str("        Self {\n");
    code.push_str("            message: self.message.clone(),\n");
    code.push_str("            parameters: std::collections::HashMap::new(),\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate FrameEvent impl with new() and new_with_params()
    code.push_str(&format!("impl {}FrameEvent {{\n", system_name));
    code.push_str("    fn new(message: &str) -> Self {\n");
    code.push_str("        Self {\n");
    code.push_str("            message: message.to_string(),\n");
    code.push_str("            parameters: std::collections::HashMap::new(),\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("    fn new_with_params(message: &str, params: &std::collections::HashMap<String, String>) -> Self {\n");
    code.push_str("        Self {\n");
    code.push_str("            message: message.to_string(),\n");
    code.push_str("            parameters: params.iter().map(|(k, v)| (k.clone(), Box::new(v.clone()) as Box<dyn std::any::Any>)).collect(),\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate FrameContext struct (call context for reentrancy)
    code.push_str("#[allow(dead_code)]\n");
    code.push_str(&format!("struct {}FrameContext {{\n", system_name));
    code.push_str(&format!("    event: {}FrameEvent,\n", system_name));
    code.push_str("    _return: Option<Box<dyn std::any::Any>>,\n");
    code.push_str("    _data: std::collections::HashMap<String, Box<dyn std::any::Any>>,\n");
    code.push_str("}\n\n");

    // Generate FrameContext impl with new()
    code.push_str(&format!("impl {}FrameContext {{\n", system_name));
    code.push_str(&format!("    fn new(event: {}FrameEvent, default_return: Option<Box<dyn std::any::Any>>) -> Self {{\n", system_name));
    code.push_str("        Self {\n");
    code.push_str("            event,\n");
    code.push_str("            _return: default_return,\n");
    code.push_str("            _data: std::collections::HashMap::new(),\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate state context types (must come before Compartment which references them)
    // Context structs for states with state variables
    if let Some(ref machine) = system.machine {
        let states_with_vars: Vec<_> = machine.states.iter()
            .filter(|s| !s.state_vars.is_empty())
            .collect();

        for state in &states_with_vars {
            code.push_str(&format!("#[derive(Clone)]\nstruct {}Context {{\n", state.name));
            for var in &state.state_vars {
                let type_str = type_to_string(&var.var_type);
                code.push_str(&format!("    {}: {},\n", var.name, type_str));
            }
            code.push_str("}\n\n");

            // Manual Default impl with declared initializers
            code.push_str(&format!("impl Default for {}Context {{\n", state.name));
            code.push_str("    fn default() -> Self {\n");
            code.push_str("        Self {\n");
            for var in &state.state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::Rust)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::Rust)
                };
                code.push_str(&format!("            {}: {},\n", var.name, init_val));
            }
            code.push_str("        }\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");
        }
    }

    // StateContext enum — typed state variable storage on the compartment
    code.push_str(&format!("#[derive(Clone)]\nenum {}StateContext {{\n", system_name));
    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            if state.state_vars.is_empty() {
                code.push_str(&format!("    {},\n", state.name));
            } else {
                code.push_str(&format!("    {}({}Context),\n", state.name, state.name));
            }
        }
    }
    code.push_str("    Empty,\n");
    code.push_str("}\n\n");

    // Default impl for StateContext
    if let Some(ref machine) = system.machine {
        if let Some(first_state) = machine.states.first() {
            code.push_str(&format!("impl Default for {}StateContext {{\n", system_name));
            code.push_str("    fn default() -> Self {\n");
            if first_state.state_vars.is_empty() {
                code.push_str(&format!("        {}StateContext::{}\n", system_name, first_state.name));
            } else {
                code.push_str(&format!("        {}StateContext::{}({}Context::default())\n",
                    system_name, first_state.name, first_state.name));
            }
            code.push_str("    }\n");
            code.push_str("}\n\n");
        }
    }

    // Generate Compartment struct
    code.push_str(&format!("#[allow(dead_code)]\n#[derive(Clone)]\nstruct {}Compartment {{\n", system_name));
    code.push_str("    state: String,\n");
    code.push_str(&format!("    state_context: {}StateContext,\n", system_name));
    code.push_str("    enter_args: std::collections::HashMap<String, String>,\n");
    code.push_str("    exit_args: std::collections::HashMap<String, String>,\n");
    code.push_str(&format!("    forward_event: Option<{}FrameEvent>,\n", system_name));
    code.push_str(&format!("    parent_compartment: Option<Box<{}Compartment>>,\n", system_name));
    code.push_str("}\n\n");

    // Generate Compartment impl with new()
    // new() automatically sets state_context to the correct variant with defaults
    code.push_str(&format!("impl {}Compartment {{\n", system_name));
    code.push_str("    fn new(state: &str) -> Self {\n");
    code.push_str(&format!("        let state_context = match state {{\n"));
    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            if state.state_vars.is_empty() {
                code.push_str(&format!("            \"{}\" => {}StateContext::{},\n",
                    state.name, system_name, state.name));
            } else {
                code.push_str(&format!("            \"{}\" => {}StateContext::{}({}Context::default()),\n",
                    state.name, system_name, state.name, state.name));
            }
        }
    }
    code.push_str(&format!("            _ => {}StateContext::Empty,\n", system_name));
    code.push_str("        };\n");
    code.push_str("        Self {\n");
    code.push_str("            state: state.to_string(),\n");
    code.push_str("            state_context,\n");
    code.push_str("            enter_args: std::collections::HashMap::new(),\n");
    code.push_str("            exit_args: std::collections::HashMap::new(),\n");
    code.push_str("            forward_event: None,\n");
    code.push_str("            parent_compartment: None,\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code
}

/// Generate C runtime types (public wrapper)
///
/// Generates the standard Frame runtime infrastructure for C:
/// - FrameDict hash map implementation
/// - FrameVec dynamic array implementation
/// - FrameEvent struct
/// - FrameContext struct
/// - Compartment struct
/// All prefixed with the system name (e.g., Minimal_FrameDict)
pub fn generate_c_compartment_types(system: &SystemAst) -> String {
    generate_c_runtime_types(system)
}

/// Generate C runtime types (internal implementation)
fn generate_c_runtime_types(system: &SystemAst) -> String {
    let sys = &system.name;
    let mut code = String::new();

    // Standard includes
    code.push_str("#include <stdlib.h>\n");
    code.push_str("#include <string.h>\n");
    code.push_str("#include <stdio.h>\n");
    code.push_str("#include <stdbool.h>\n");
    code.push_str("#include <stdint.h>\n\n");

    // ============================================================================
    // FrameDict - String-keyed hash map
    // ============================================================================
    code.push_str(&format!("// ============================================================================\n"));
    code.push_str(&format!("// {}_FrameDict - String-keyed dictionary\n", sys));
    code.push_str(&format!("// ============================================================================\n\n"));

    code.push_str(&format!("typedef struct {}_FrameDictEntry {{\n", sys));
    code.push_str("    char* key;\n");
    code.push_str("    void* value;\n");
    code.push_str(&format!("    struct {}_FrameDictEntry* next;\n", sys));
    code.push_str(&format!("}} {}_FrameDictEntry;\n\n", sys));

    code.push_str(&format!("typedef struct {{\n"));
    code.push_str(&format!("    {}_FrameDictEntry** buckets;\n", sys));
    code.push_str("    int bucket_count;\n");
    code.push_str("    int size;\n");
    code.push_str(&format!("}} {}_FrameDict;\n\n", sys));

    // Hash function
    code.push_str(&format!("static unsigned int {}_hash_string(const char* str) {{\n", sys));
    code.push_str("    unsigned int hash = 5381;\n");
    code.push_str("    int c;\n");
    code.push_str("    while ((c = *str++)) {\n");
    code.push_str("        hash = ((hash << 5) + hash) + c;\n");
    code.push_str("    }\n");
    code.push_str("    return hash;\n");
    code.push_str("}\n\n");

    // FrameDict_new
    code.push_str(&format!("static {}_FrameDict* {}_FrameDict_new(void) {{\n", sys, sys));
    code.push_str(&format!("    {}_FrameDict* d = malloc(sizeof({}_FrameDict));\n", sys, sys));
    code.push_str("    d->bucket_count = 16;\n");
    code.push_str(&format!("    d->buckets = calloc(d->bucket_count, sizeof({}_FrameDictEntry*));\n", sys));
    code.push_str("    d->size = 0;\n");
    code.push_str("    return d;\n");
    code.push_str("}\n\n");

    // FrameDict_set
    code.push_str(&format!("static void {}_FrameDict_set({}_FrameDict* d, const char* key, void* value) {{\n", sys, sys));
    code.push_str(&format!("    unsigned int idx = {}_hash_string(key) % d->bucket_count;\n", sys));
    code.push_str(&format!("    {}_FrameDictEntry* entry = d->buckets[idx];\n", sys));
    code.push_str("    while (entry) {\n");
    code.push_str("        if (strcmp(entry->key, key) == 0) {\n");
    code.push_str("            entry->value = value;\n");
    code.push_str("            return;\n");
    code.push_str("        }\n");
    code.push_str("        entry = entry->next;\n");
    code.push_str("    }\n");
    code.push_str(&format!("    {}_FrameDictEntry* new_entry = malloc(sizeof({}_FrameDictEntry));\n", sys, sys));
    code.push_str("    new_entry->key = strdup(key);\n");
    code.push_str("    new_entry->value = value;\n");
    code.push_str("    new_entry->next = d->buckets[idx];\n");
    code.push_str("    d->buckets[idx] = new_entry;\n");
    code.push_str("    d->size++;\n");
    code.push_str("}\n\n");

    // FrameDict_get
    code.push_str(&format!("static void* {}_FrameDict_get({}_FrameDict* d, const char* key) {{\n", sys, sys));
    code.push_str(&format!("    unsigned int idx = {}_hash_string(key) % d->bucket_count;\n", sys));
    code.push_str(&format!("    {}_FrameDictEntry* entry = d->buckets[idx];\n", sys));
    code.push_str("    while (entry) {\n");
    code.push_str("        if (strcmp(entry->key, key) == 0) {\n");
    code.push_str("            return entry->value;\n");
    code.push_str("        }\n");
    code.push_str("        entry = entry->next;\n");
    code.push_str("    }\n");
    code.push_str("    return NULL;\n");
    code.push_str("}\n\n");

    // FrameDict_has - check if key exists
    code.push_str(&format!("static int {}_FrameDict_has({}_FrameDict* d, const char* key) {{\n", sys, sys));
    code.push_str(&format!("    unsigned int idx = {}_hash_string(key) % d->bucket_count;\n", sys));
    code.push_str(&format!("    {}_FrameDictEntry* entry = d->buckets[idx];\n", sys));
    code.push_str("    while (entry) {\n");
    code.push_str("        if (strcmp(entry->key, key) == 0) {\n");
    code.push_str("            return 1;\n");
    code.push_str("        }\n");
    code.push_str("        entry = entry->next;\n");
    code.push_str("    }\n");
    code.push_str("    return 0;\n");
    code.push_str("}\n\n");

    // FrameDict_copy
    code.push_str(&format!("static {}_FrameDict* {}_FrameDict_copy({}_FrameDict* src) {{\n", sys, sys, sys));
    code.push_str(&format!("    {}_FrameDict* dst = {}_FrameDict_new();\n", sys, sys));
    code.push_str("    for (int i = 0; i < src->bucket_count; i++) {\n");
    code.push_str(&format!("        {}_FrameDictEntry* entry = src->buckets[i];\n", sys));
    code.push_str("        while (entry) {\n");
    code.push_str(&format!("            {}_FrameDict_set(dst, entry->key, entry->value);\n", sys));
    code.push_str("            entry = entry->next;\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("    return dst;\n");
    code.push_str("}\n\n");

    // FrameDict_destroy
    code.push_str(&format!("static void {}_FrameDict_destroy({}_FrameDict* d) {{\n", sys, sys));
    code.push_str("    for (int i = 0; i < d->bucket_count; i++) {\n");
    code.push_str(&format!("        {}_FrameDictEntry* entry = d->buckets[i];\n", sys));
    code.push_str("        while (entry) {\n");
    code.push_str(&format!("            {}_FrameDictEntry* next = entry->next;\n", sys));
    code.push_str("            free(entry->key);\n");
    code.push_str("            free(entry);\n");
    code.push_str("            entry = next;\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("    free(d->buckets);\n");
    code.push_str("    free(d);\n");
    code.push_str("}\n\n");

    // ============================================================================
    // FrameVec - Dynamic array
    // ============================================================================
    code.push_str(&format!("// ============================================================================\n"));
    code.push_str(&format!("// {}_FrameVec - Dynamic array\n", sys));
    code.push_str(&format!("// ============================================================================\n\n"));

    code.push_str(&format!("typedef struct {{\n"));
    code.push_str("    void** items;\n");
    code.push_str("    int size;\n");
    code.push_str("    int capacity;\n");
    code.push_str(&format!("}} {}_FrameVec;\n\n", sys));

    // FrameVec_new
    code.push_str(&format!("static {}_FrameVec* {}_FrameVec_new(void) {{\n", sys, sys));
    code.push_str(&format!("    {}_FrameVec* v = malloc(sizeof({}_FrameVec));\n", sys, sys));
    code.push_str("    v->capacity = 8;\n");
    code.push_str("    v->size = 0;\n");
    code.push_str("    v->items = malloc(sizeof(void*) * v->capacity);\n");
    code.push_str("    return v;\n");
    code.push_str("}\n\n");

    // FrameVec_push
    code.push_str(&format!("static void {}_FrameVec_push({}_FrameVec* v, void* item) {{\n", sys, sys));
    code.push_str("    if (v->size >= v->capacity) {\n");
    code.push_str("        v->capacity *= 2;\n");
    code.push_str("        v->items = realloc(v->items, sizeof(void*) * v->capacity);\n");
    code.push_str("    }\n");
    code.push_str("    v->items[v->size++] = item;\n");
    code.push_str("}\n\n");

    // FrameVec_pop
    code.push_str(&format!("static void* {}_FrameVec_pop({}_FrameVec* v) {{\n", sys, sys));
    code.push_str("    if (v->size == 0) return NULL;\n");
    code.push_str("    return v->items[--v->size];\n");
    code.push_str("}\n\n");

    // FrameVec_last
    code.push_str(&format!("static void* {}_FrameVec_last({}_FrameVec* v) {{\n", sys, sys));
    code.push_str("    if (v->size == 0) return NULL;\n");
    code.push_str("    return v->items[v->size - 1];\n");
    code.push_str("}\n\n");

    // FrameVec_get (indexed access)
    code.push_str(&format!("static void* {}_FrameVec_get({}_FrameVec* v, int index) {{\n", sys, sys));
    code.push_str("    if (index < 0 || index >= v->size) return NULL;\n");
    code.push_str("    return v->items[index];\n");
    code.push_str("}\n\n");

    // FrameVec_size
    code.push_str(&format!("static int {}_FrameVec_size({}_FrameVec* v) {{\n", sys, sys));
    code.push_str("    return v->size;\n");
    code.push_str("}\n\n");

    // FrameVec_destroy
    code.push_str(&format!("static void {}_FrameVec_destroy({}_FrameVec* v) {{\n", sys, sys));
    code.push_str("    free(v->items);\n");
    code.push_str("    free(v);\n");
    code.push_str("}\n\n");

    // ============================================================================
    // FrameEvent - Event routing object
    // ============================================================================
    code.push_str(&format!("// ============================================================================\n"));
    code.push_str(&format!("// {}_FrameEvent - Event routing object\n", sys));
    code.push_str(&format!("// ============================================================================\n\n"));

    code.push_str(&format!("typedef struct {{\n"));
    code.push_str("    const char* _message;\n");
    code.push_str(&format!("    {}_FrameDict* _parameters;\n", sys));
    code.push_str(&format!("}} {}_FrameEvent;\n\n", sys));

    // FrameEvent_new
    code.push_str(&format!("static {}_FrameEvent* {}_FrameEvent_new(const char* message, {}_FrameDict* parameters) {{\n", sys, sys, sys));
    code.push_str(&format!("    {}_FrameEvent* e = malloc(sizeof({}_FrameEvent));\n", sys, sys));
    code.push_str("    e->_message = message;\n");
    code.push_str("    e->_parameters = parameters;\n");
    code.push_str("    return e;\n");
    code.push_str("}\n\n");

    // FrameEvent_destroy
    code.push_str(&format!("static void {}_FrameEvent_destroy({}_FrameEvent* e) {{\n", sys, sys));
    code.push_str("    // Note: _parameters ownership depends on context\n");
    code.push_str("    free(e);\n");
    code.push_str("}\n\n");

    // ============================================================================
    // FrameContext - Interface call context
    // ============================================================================
    code.push_str(&format!("// ============================================================================\n"));
    code.push_str(&format!("// {}_FrameContext - Interface call context\n", sys));
    code.push_str(&format!("// ============================================================================\n\n"));

    code.push_str(&format!("typedef struct {{\n"));
    code.push_str(&format!("    {}_FrameEvent* event;\n", sys));
    code.push_str("    void* _return;\n");
    code.push_str(&format!("    {}_FrameDict* _data;\n", sys));
    code.push_str(&format!("}} {}_FrameContext;\n\n", sys));

    // FrameContext_new
    code.push_str(&format!("static {}_FrameContext* {}_FrameContext_new({}_FrameEvent* event, void* default_return) {{\n", sys, sys, sys));
    code.push_str(&format!("    {}_FrameContext* ctx = malloc(sizeof({}_FrameContext));\n", sys, sys));
    code.push_str("    ctx->event = event;\n");
    code.push_str("    ctx->_return = default_return;\n");
    code.push_str(&format!("    ctx->_data = {}_FrameDict_new();\n", sys));
    code.push_str("    return ctx;\n");
    code.push_str("}\n\n");

    // FrameContext_destroy
    code.push_str(&format!("static void {}_FrameContext_destroy({}_FrameContext* ctx) {{\n", sys, sys));
    code.push_str(&format!("    {}_FrameDict_destroy(ctx->_data);\n", sys));
    code.push_str("    free(ctx);\n");
    code.push_str("}\n\n");

    // ============================================================================
    // Compartment - State closure
    // ============================================================================
    code.push_str(&format!("// ============================================================================\n"));
    code.push_str(&format!("// {}_Compartment - State closure\n", sys));
    code.push_str(&format!("// ============================================================================\n\n"));

    code.push_str(&format!("typedef struct {}_Compartment {{\n", sys));
    code.push_str("    const char* state;\n");
    code.push_str(&format!("    {}_FrameDict* state_args;\n", sys));
    code.push_str(&format!("    {}_FrameDict* state_vars;\n", sys));
    code.push_str(&format!("    {}_FrameDict* enter_args;\n", sys));
    code.push_str(&format!("    {}_FrameDict* exit_args;\n", sys));
    code.push_str(&format!("    {}_FrameEvent* forward_event;\n", sys));
    code.push_str(&format!("    struct {}_Compartment* parent_compartment;\n", sys));
    code.push_str(&format!("}} {}_Compartment;\n\n", sys));

    // Compartment_new
    code.push_str(&format!("static {}_Compartment* {}_Compartment_new(const char* state) {{\n", sys, sys));
    code.push_str(&format!("    {}_Compartment* c = malloc(sizeof({}_Compartment));\n", sys, sys));
    code.push_str("    c->state = state;\n");
    code.push_str(&format!("    c->state_args = {}_FrameDict_new();\n", sys));
    code.push_str(&format!("    c->state_vars = {}_FrameDict_new();\n", sys));
    code.push_str(&format!("    c->enter_args = {}_FrameDict_new();\n", sys));
    code.push_str(&format!("    c->exit_args = {}_FrameDict_new();\n", sys));
    code.push_str("    c->forward_event = NULL;\n");
    code.push_str("    c->parent_compartment = NULL;\n");
    code.push_str("    return c;\n");
    code.push_str("}\n\n");

    // Compartment_copy
    code.push_str(&format!("static {}_Compartment* {}_Compartment_copy({}_Compartment* src) {{\n", sys, sys, sys));
    code.push_str(&format!("    {}_Compartment* c = malloc(sizeof({}_Compartment));\n", sys, sys));
    code.push_str("    c->state = src->state;\n");
    code.push_str(&format!("    c->state_args = {}_FrameDict_copy(src->state_args);\n", sys));
    code.push_str(&format!("    c->state_vars = {}_FrameDict_copy(src->state_vars);\n", sys));
    code.push_str(&format!("    c->enter_args = {}_FrameDict_copy(src->enter_args);\n", sys));
    code.push_str(&format!("    c->exit_args = {}_FrameDict_copy(src->exit_args);\n", sys));
    code.push_str("    c->forward_event = src->forward_event;  // Shallow copy OK\n");
    code.push_str("    c->parent_compartment = src->parent_compartment;\n");
    code.push_str("    return c;\n");
    code.push_str("}\n\n");

    // Compartment_destroy
    code.push_str(&format!("static void {}_Compartment_destroy({}_Compartment* c) {{\n", sys, sys));
    code.push_str(&format!("    {}_FrameDict_destroy(c->state_args);\n", sys));
    code.push_str(&format!("    {}_FrameDict_destroy(c->state_vars);\n", sys));
    code.push_str(&format!("    {}_FrameDict_destroy(c->enter_args);\n", sys));
    code.push_str(&format!("    {}_FrameDict_destroy(c->exit_args);\n", sys));
    code.push_str("    free(c);\n");
    code.push_str("}\n\n");

    // Helper macros for context access
    code.push_str(&format!("// Helper macros for context access\n"));
    code.push_str(&format!("#define {}_CTX(self) (({}_FrameContext*){}_FrameVec_last((self)->_context_stack))\n", sys, sys, sys));
    code.push_str(&format!("#define {}_PARAM(self, key) {}_FrameDict_get({}_CTX(self)->event->_parameters, key)\n", sys, sys, sys));
    code.push_str(&format!("#define {}_RETURN(self) {}_CTX(self)->_return\n", sys, sys));
    code.push_str(&format!("#define {}_DATA(self, key) {}_FrameDict_get({}_CTX(self)->_data, key)\n", sys, sys, sys));
    code.push_str(&format!("#define {}_DATA_SET(self, key, val) {}_FrameDict_set({}_CTX(self)->_data, key, val)\n\n", sys, sys, sys));

    // System destroy function (declared as part of forward declarations, defined later)
    // This will be declared as a forward declaration in the class emission

    code
}

/// Generate the constructor
fn generate_constructor(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> CodegenNode {
    let mut body = Vec::new();

    // Initialize state stack - language specific
    match syntax.language {
        TargetLanguage::C => {
            // C: Use FrameVec_new()
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_state_stack"),
                CodegenNode::Ident(format!("{}_FrameVec_new()", system.name)),
            ));
        }
        _ => {
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_state_stack"),
                CodegenNode::Array(vec![]),
            ));
        }
    }

    // Initialize context stack (for reentrancy support) - language specific
    match syntax.language {
        TargetLanguage::C => {
            // C: Use FrameVec_new()
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_context_stack"),
                CodegenNode::Ident(format!("{}_FrameVec_new()", system.name)),
            ));
        }
        _ => {
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_context_stack"),
                CodegenNode::Array(vec![]),
            ));
        }
    }

    // Initialize domain variables
    for domain_var in &system.domain {
        if let Some(ref raw_code) = domain_var.raw_code {
            // V4: Native code pass-through
            // Python: emit as self.<raw_code> in __init__
            // TypeScript: already in class fields, skip constructor init
            // C: struct is zeroed by calloc
            // Rust: need explicit init in struct literal
            if matches!(syntax.language, TargetLanguage::Python3) {
                body.push(CodegenNode::NativeBlock {
                    code: format!("self.{}", raw_code),
                    span: None,
                });
            } else if matches!(syntax.language, TargetLanguage::Rust) {
                // For Rust, initialize with Default::default()
                body.push(CodegenNode::assign(
                    CodegenNode::field(CodegenNode::self_ref(), &domain_var.name),
                    CodegenNode::Ident("Default::default()".to_string()),
                ));
            }
        } else if let Some(ref init) = &domain_var.initializer {
            // Legacy: Construct from parsed components
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), &domain_var.name),
                convert_expression(init),
            ));
        } else if matches!(syntax.language, TargetLanguage::Rust) {
            // Rust requires all struct fields to be initialized.
            // Domain vars with no initializer get Default::default().
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), &domain_var.name),
                CodegenNode::Ident("Default::default()".to_string()),
            ));
        }
    }

    // Rust state vars now live on compartment.state_context — no _sv_ field init needed.
    // State vars are initialized when compartments are created (in transition codegen).

    // Set initial state (first state in machine)
    // All languages now use the kernel/router/compartment pattern
    if let Some(ref machine) = system.machine {
        if let Some(first_state) = machine.states.first() {
            let compartment_class = format!("{}Compartment", system.name);
            let event_class = format!("{}FrameEvent", system.name);

            // HSM: Build ancestor chain if start state has a parent
            // We need to create compartments for all ancestors and link them via parent_compartment
            let has_hsm_parent = first_state.parent.is_some();

            // Build ancestor chain from root to leaf (reversed order for creation)
            let mut ancestor_chain: Vec<&crate::frame_c::v4::frame_ast::StateAst> = Vec::new();
            if has_hsm_parent {
                let mut current_parent = first_state.parent.as_ref();
                while let Some(parent_name) = current_parent {
                    if let Some(parent_state) = machine.states.iter().find(|s| &s.name == parent_name) {
                        ancestor_chain.push(parent_state);
                        current_parent = parent_state.parent.as_ref();
                    } else {
                        break;
                    }
                }
                // Reverse so we start from root (topmost parent)
                ancestor_chain.reverse();
            }

            // Initialize __compartment with initial state
            match syntax.language {
                TargetLanguage::Rust => {
                    // Rust: Create compartment chain for HSM if start state has parent
                    if !ancestor_chain.is_empty() {
                        // For Rust, use a block expression inside struct literal
                        // This creates parent chain and returns the child compartment
                        let mut block_expr = String::new();
                        block_expr.push_str("{\n");

                        // Create compartments from root to leaf
                        let mut prev_comp_var = "None".to_string();
                        for (i, ancestor) in ancestor_chain.iter().enumerate() {
                            let comp_var = format!("__parent_comp_{}", i);
                            block_expr.push_str(&format!(
                                "let mut {} = {}Compartment::new(\"{}\");\n",
                                comp_var, system.name, ancestor.name
                            ));
                            block_expr.push_str(&format!(
                                "{}.parent_compartment = {};\n",
                                comp_var, prev_comp_var
                            ));
                            // state_context is auto-set by Compartment::new()
                            prev_comp_var = format!("Some(Box::new({}))", comp_var);
                        }
                        // Create the start state compartment with parent link
                        // state_context is auto-set by Compartment::new()
                        block_expr.push_str(&format!(
                            "let mut __child = {}Compartment::new(\"{}\");\n",
                            system.name, first_state.name
                        ));
                        block_expr.push_str(&format!(
                            "__child.parent_compartment = {};\n",
                            prev_comp_var
                        ));
                        block_expr.push_str("__child\n}");

                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__compartment"),
                            CodegenNode::Ident(block_expr),
                        ));
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__next_compartment"),
                            CodegenNode::Ident("None".to_string()),
                        ));
                    } else {
                        // No HSM parent - simple compartment creation
                        // state_context is auto-set by Compartment::new()
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__compartment"),
                            CodegenNode::Ident(format!("{}Compartment::new(\"{}\")", system.name, first_state.name)),
                        ));
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__next_compartment"),
                            CodegenNode::Ident("None".to_string()),
                        ));
                    }
                }
                TargetLanguage::C => {
                    // C: Create compartment chain for HSM if start state has parent
                    if !ancestor_chain.is_empty() {
                        let mut hsm_init_code = String::new();
                        hsm_init_code.push_str("// HSM: Create parent compartment chain\n");

                        // Create compartments from root to leaf
                        let mut prev_comp_var = "NULL".to_string();
                        for (i, ancestor) in ancestor_chain.iter().enumerate() {
                            let comp_var = format!("__parent_comp_{}", i);
                            hsm_init_code.push_str(&format!(
                                "{}_Compartment* {} = {}_Compartment_new(\"{}\");\n",
                                system.name, comp_var, system.name, ancestor.name
                            ));
                            hsm_init_code.push_str(&format!(
                                "{}->parent_compartment = {};\n",
                                comp_var, prev_comp_var
                            ));
                            // Initialize state vars for this ancestor
                            for var in &ancestor.state_vars {
                                let init_val = if let Some(ref init) = var.init {
                                    expression_to_string(init, TargetLanguage::C)
                                } else {
                                    state_var_init_value(&var.var_type, TargetLanguage::C)
                                };
                                hsm_init_code.push_str(&format!(
                                    "{}_FrameDict_set({}->state_vars, \"{}\", (void*)(intptr_t){});\n",
                                    system.name, comp_var, var.name, init_val
                                ));
                            }
                            prev_comp_var = comp_var;
                        }
                        // Create the start state compartment with parent link
                        hsm_init_code.push_str(&format!(
                            "self->__compartment = {}_Compartment_new(\"{}\");\n",
                            system.name, first_state.name
                        ));
                        hsm_init_code.push_str(&format!(
                            "self->__compartment->parent_compartment = {};\n",
                            prev_comp_var
                        ));
                        hsm_init_code.push_str("self->__next_compartment = NULL;");

                        body.push(CodegenNode::NativeBlock {
                            code: hsm_init_code,
                            span: None,
                        });
                    } else {
                        // No HSM parent - simple compartment creation
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__compartment"),
                            CodegenNode::Ident(format!("{}_Compartment_new(\"{}\")", system.name, first_state.name)),
                        ));
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__next_compartment"),
                            CodegenNode::null(),
                        ));
                    }
                }
                TargetLanguage::Python3 => {
                    // Python: Create compartment chain for HSM if start state has parent
                    if !ancestor_chain.is_empty() {
                        let mut hsm_init_code = String::new();
                        hsm_init_code.push_str("# HSM: Create parent compartment chain\n");

                        // Create compartments from root to leaf
                        let mut prev_comp_var = "None".to_string();
                        for (i, ancestor) in ancestor_chain.iter().enumerate() {
                            let comp_var = format!("__parent_comp_{}", i);
                            hsm_init_code.push_str(&format!(
                                "{} = {}(\"{}\", parent_compartment={})\n",
                                comp_var, compartment_class, ancestor.name, prev_comp_var
                            ));
                            // Initialize state vars for this ancestor
                            for var in &ancestor.state_vars {
                                let init_val = if let Some(ref init) = var.init {
                                    expression_to_string(init, TargetLanguage::Python3)
                                } else {
                                    state_var_init_value(&var.var_type, TargetLanguage::Python3)
                                };
                                hsm_init_code.push_str(&format!(
                                    "{}.state_vars[\"{}\"] = {}\n",
                                    comp_var, var.name, init_val
                                ));
                            }
                            prev_comp_var = comp_var;
                        }
                        // Create the start state compartment with parent link
                        hsm_init_code.push_str(&format!(
                            "self.__compartment = {}(\"{}\", parent_compartment={})\n",
                            compartment_class, first_state.name, prev_comp_var
                        ));
                        hsm_init_code.push_str("self.__next_compartment = None");

                        body.push(CodegenNode::NativeBlock {
                            code: hsm_init_code,
                            span: None,
                        });
                    } else {
                        // No HSM parent - simple compartment creation
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__compartment"),
                            CodegenNode::New {
                                class: compartment_class.clone(),
                                args: vec![CodegenNode::string(&first_state.name)],
                            },
                        ));
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__next_compartment"),
                            CodegenNode::null(),
                        ));
                    }
                }
                TargetLanguage::TypeScript => {
                    // TypeScript: Create compartment chain for HSM if start state has parent
                    if !ancestor_chain.is_empty() {
                        let mut hsm_init_code = String::new();
                        hsm_init_code.push_str("// HSM: Create parent compartment chain\n");

                        // Create compartments from root to leaf
                        let mut prev_comp_var = "null".to_string();
                        for (i, ancestor) in ancestor_chain.iter().enumerate() {
                            let comp_var = format!("__parent_comp_{}", i);
                            hsm_init_code.push_str(&format!(
                                "const {} = new {}(\"{}\", {});\n",
                                comp_var, compartment_class, ancestor.name, prev_comp_var
                            ));
                            // Initialize state vars for this ancestor
                            for var in &ancestor.state_vars {
                                let init_val = if let Some(ref init) = var.init {
                                    expression_to_string(init, TargetLanguage::TypeScript)
                                } else {
                                    state_var_init_value(&var.var_type, TargetLanguage::TypeScript)
                                };
                                hsm_init_code.push_str(&format!(
                                    "{}.state_vars[\"{}\"] = {};\n",
                                    comp_var, var.name, init_val
                                ));
                            }
                            prev_comp_var = comp_var;
                        }
                        // Create the start state compartment with parent link
                        hsm_init_code.push_str(&format!(
                            "this.__compartment = new {}(\"{}\", {});\n",
                            compartment_class, first_state.name, prev_comp_var
                        ));
                        hsm_init_code.push_str("this.__next_compartment = null;");

                        body.push(CodegenNode::NativeBlock {
                            code: hsm_init_code,
                            span: None,
                        });
                    } else {
                        // No HSM parent - simple compartment creation
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__compartment"),
                            CodegenNode::New {
                                class: compartment_class.clone(),
                                args: vec![CodegenNode::string(&first_state.name)],
                            },
                        ));
                        body.push(CodegenNode::assign(
                            CodegenNode::field(CodegenNode::self_ref(), "__next_compartment"),
                            CodegenNode::null(),
                        ));
                    }
                }
                _ => {
                    // Other languages: New expression (HSM support to be added)
                    body.push(CodegenNode::assign(
                        CodegenNode::field(CodegenNode::self_ref(), "__compartment"),
                        CodegenNode::New {
                            class: compartment_class.clone(),
                            args: vec![CodegenNode::string(&first_state.name)],
                        },
                    ));
                    body.push(CodegenNode::assign(
                        CodegenNode::field(CodegenNode::self_ref(), "__next_compartment"),
                        CodegenNode::null(),
                    ));
                }
            }

            // Send $> (enter) event via __kernel - language-specific
            let init_event_code = match syntax.language {
                TargetLanguage::Python3 => format!(
                    r#"__frame_event = {}("$>", None)
self.__kernel(__frame_event)"#,
                    event_class
                ),
                TargetLanguage::TypeScript => format!(
                    r#"const __frame_event = new {}("$>", null);
this.__kernel(__frame_event);"#,
                    event_class
                ),
                TargetLanguage::Rust => format!(
                    r#"let __frame_event = {}::new("$>");
self.__kernel(__frame_event)"#,
                    event_class
                ),
                TargetLanguage::C => format!(
                    r#"{}_FrameEvent* __frame_event = {}_FrameEvent_new("$>", NULL);
{}_kernel(self, __frame_event);
{}_FrameEvent_destroy(__frame_event);"#,
                    system.name, system.name, system.name, system.name
                ),
                _ => format!(
                    r#"const __frame_event = new {}("$>", null);
this.__kernel(__frame_event);"#,
                    event_class
                ),
            };
            body.push(CodegenNode::NativeBlock {
                code: init_event_code,
                span: None,
            });
        }
    }

    // Params from system params
    let params: Vec<Param> = system.params.iter().map(|p| {
        let type_str = type_to_string(&p.param_type);
        Param::new(&p.name).with_type(&type_str)
    }).collect();

    CodegenNode::Constructor {
        params,
        body,
        super_call: None,
    }
}

/// Generate Frame machinery methods
///
/// For Python/TypeScript: Proper Frame runtime with __kernel, __router, __transition
/// For Rust: Simplified implementation (proper runtime in future task)
fn generate_frame_machinery(system: &SystemAst, _syntax: &super::backend::ClassSyntax, lang: TargetLanguage) -> Vec<CodegenNode> {
    let mut methods = Vec::new();
    let compartment_class = format!("{}Compartment", system.name);
    let event_class = format!("{}FrameEvent", system.name);

    match lang {
        TargetLanguage::Python3 => {
            // __kernel method - the main event processing loop
            // Routes event to current state, then processes any pending transition
            methods.push(CodegenNode::Method {
                name: "__kernel".to_string(),
                params: vec![Param::new("__e")],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: format!(
                        r#"# Route event to current state
self.__router(__e)
# Process any pending transition
while self.__next_compartment is not None:
    next_compartment = self.__next_compartment
    self.__next_compartment = None
    # Exit current state
    exit_event = {}("<$", self.__compartment.exit_args)
    self.__router(exit_event)
    # Switch to new compartment
    self.__compartment = next_compartment
    # Enter new state (or forward event)
    if next_compartment.forward_event is None:
        enter_event = {}("$>", self.__compartment.enter_args)
        self.__router(enter_event)
    else:
        # Forward event to new state
        forward_event = next_compartment.forward_event
        next_compartment.forward_event = None
        if forward_event._message == "$>":
            # Forwarding enter event - just send it
            self.__router(forward_event)
        else:
            # Forwarding other event - send $> first, then forward
            enter_event = {}("$>", self.__compartment.enter_args)
            self.__router(enter_event)
            self.__router(forward_event)"#,
                        event_class, event_class, event_class
                    ),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __router method - dispatches events to state methods
            methods.push(CodegenNode::Method {
                name: "__router".to_string(),
                params: vec![Param::new("__e")],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: r#"state_name = self.__compartment.state
handler_name = f"_state_{state_name}"
handler = getattr(self, handler_name, None)
if handler:
    handler(__e)"#.to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __transition method - caches next compartment (deferred transition)
            // Does NOT execute transition - __kernel does that after handler returns
            methods.push(CodegenNode::Method {
                name: "__transition".to_string(),
                params: vec![Param::new("next_compartment")],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: "self.__next_compartment = next_compartment".to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });
        }
        TargetLanguage::TypeScript => {
            // __kernel method - the main event processing loop
            methods.push(CodegenNode::Method {
                name: "__kernel".to_string(),
                params: vec![Param::new("__e").with_type(&event_class)],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: format!(
                        r#"// Route event to current state
this.__router(__e);
// Process any pending transition
while (this.__next_compartment !== null) {{
    const next_compartment = this.__next_compartment;
    this.__next_compartment = null;
    // Exit current state
    const exit_event = new {}("<$", this.__compartment.exit_args);
    this.__router(exit_event);
    // Switch to new compartment
    this.__compartment = next_compartment;
    // Enter new state (or forward event)
    if (next_compartment.forward_event === null) {{
        const enter_event = new {}("$>", this.__compartment.enter_args);
        this.__router(enter_event);
    }} else {{
        // Forward event to new state
        const forward_event = next_compartment.forward_event;
        next_compartment.forward_event = null;
        if (forward_event._message === "$>") {{
            // Forwarding enter event - just send it
            this.__router(forward_event);
        }} else {{
            // Forwarding other event - send $> first, then forward
            const enter_event = new {}("$>", this.__compartment.enter_args);
            this.__router(enter_event);
            this.__router(forward_event);
        }}
    }}
}}"#,
                        event_class, event_class, event_class
                    ),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __router method - dispatches events to state methods
            methods.push(CodegenNode::Method {
                name: "__router".to_string(),
                params: vec![Param::new("__e").with_type(&event_class)],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: r#"const state_name = this.__compartment.state;
const handler_name = `_state_${state_name}`;
const handler = (this as any)[handler_name];
if (handler) {
    handler.call(this, __e);
}"#.to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __transition method - caches next compartment (deferred transition)
            methods.push(CodegenNode::Method {
                name: "__transition".to_string(),
                params: vec![Param::new("next_compartment").with_type(&compartment_class)],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: "this.__next_compartment = next_compartment;".to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });
        }
        TargetLanguage::Rust => {
            // Rust: Full kernel/router/transition pattern matching Python/TypeScript

            // __kernel method - the main event processing loop with deferred transitions
            methods.push(CodegenNode::Method {
                name: "__kernel".to_string(),
                params: vec![Param::new("__e").with_type(&event_class)],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: format!(
                        r#"// Route event to current state
self.__router(&__e);
// Process any pending transition
while self.__next_compartment.is_some() {{
    let next_compartment = self.__next_compartment.take().unwrap();
    // Exit current state (with exit_args from current compartment)
    let exit_event = {}::new_with_params("<$", &self.__compartment.exit_args);
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {{
        let enter_event = {}::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
    }} else {{
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {{
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        }} else {{
            // Forwarding other event - send $> first, then forward
            let enter_event = {}::new_with_params("$>", &self.__compartment.enter_args);
            self.__router(&enter_event);
            self.__router(&forward_event);
        }}
    }}
}}"#,
                        event_class, event_class, event_class
                    ),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __router method - dispatches events to state dispatch methods
            let router_code = generate_rust_router_dispatch(system);
            methods.push(CodegenNode::Method {
                name: "__router".to_string(),
                params: vec![Param::new("__e").with_type(&format!("&{}", event_class))],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: router_code,
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __transition method - caches next compartment (deferred transition)
            methods.push(CodegenNode::Method {
                name: "__transition".to_string(),
                params: vec![Param::new("next_compartment").with_type(&compartment_class)],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: "self.__next_compartment = Some(next_compartment);".to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });
        }
        TargetLanguage::C => {
            // C: Full kernel/router/transition pattern with string comparison dispatch
            let sys = &system.name;

            // __kernel method - the main event processing loop
            methods.push(CodegenNode::Method {
                name: "__kernel".to_string(),
                params: vec![Param::new("__e").with_type(&format!("{}_FrameEvent*", sys))],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: format!(
                        r#"// Route event to current state
{sys}_router(self, __e);
// Process any pending transition
while (self->__next_compartment != NULL) {{
    {sys}_Compartment* next_compartment = self->__next_compartment;
    self->__next_compartment = NULL;
    // Exit current state (with exit_args from current compartment)
    {sys}_FrameEvent* exit_event = {sys}_FrameEvent_new("<$", self->__compartment->exit_args);
    {sys}_router(self, exit_event);
    {sys}_FrameEvent_destroy(exit_event);
    // Switch to new compartment
    {sys}_Compartment_destroy(self->__compartment);
    self->__compartment = next_compartment;
    // Enter new state (or forward event)
    if (next_compartment->forward_event == NULL) {{
        {sys}_FrameEvent* enter_event = {sys}_FrameEvent_new("$>", self->__compartment->enter_args);
        {sys}_router(self, enter_event);
        {sys}_FrameEvent_destroy(enter_event);
    }} else {{
        // Forward event to new state
        // Note: forward_event is a borrowed pointer to the caller's __e, do NOT destroy it
        {sys}_FrameEvent* forward_event = next_compartment->forward_event;
        next_compartment->forward_event = NULL;
        if (strcmp(forward_event->_message, "$>") == 0) {{
            // Forwarding enter event - just send it
            {sys}_router(self, forward_event);
        }} else {{
            // Forwarding other event - send $> first, then forward
            {sys}_FrameEvent* enter_event = {sys}_FrameEvent_new("$>", self->__compartment->enter_args);
            {sys}_router(self, enter_event);
            {sys}_FrameEvent_destroy(enter_event);
            {sys}_router(self, forward_event);
        }}
        // Do NOT destroy forward_event - it's owned by the interface method caller
    }}
}}"#,
                        sys = sys
                    ),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __router method - dispatches events to state handler functions
            let router_code = generate_c_router_dispatch(system);
            methods.push(CodegenNode::Method {
                name: "__router".to_string(),
                params: vec![Param::new("__e").with_type(&format!("{}_FrameEvent*", sys))],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: router_code,
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // __transition method - caches next compartment (deferred transition)
            methods.push(CodegenNode::Method {
                name: "__transition".to_string(),
                params: vec![Param::new("next_compartment").with_type(&format!("{}_Compartment*", sys))],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: "self->__next_compartment = next_compartment;".to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });

            // destroy method - cleanup system resources
            methods.push(CodegenNode::Method {
                name: "destroy".to_string(),
                params: vec![],
                return_type: None,
                body: vec![CodegenNode::NativeBlock {
                    code: format!(
                        r#"if (self->__compartment) {sys}_Compartment_destroy(self->__compartment);
if (self->_state_stack) {sys}_FrameVec_destroy(self->_state_stack);
if (self->_context_stack) {sys}_FrameVec_destroy(self->_context_stack);
free(self);"#,
                        sys = sys
                    ),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Public,
                decorators: vec![],
            });
        }
        _ => {
            // Default fallback - use TypeScript-style
            methods.push(CodegenNode::Method {
                name: "__kernel".to_string(),
                params: vec![Param::new("__e")],
                return_type: None,
                body: vec![CodegenNode::comment("Kernel implementation needed for this language")],
                is_async: false,
                is_static: false,
                visibility: Visibility::Private,
                decorators: vec![],
            });
        }
    }

    // NOTE: _change_state method removed - it was deprecated legacy (->> operator)
    // The ->> syntax should not compile in V4

    // NOTE: Rust now uses kernel pattern like Python/TypeScript
    // $>/$< events are handled via _state_X dispatch methods, not _enter/_exit dispatchers

    // Generate __push_transition method for Rust when there's a machine
    // Uses mem::replace to move the current compartment to the stack (no clone)
    if matches!(lang, TargetLanguage::Rust) && system.machine.is_some() {
        methods.push(generate_rust_push_transition(system));
    }

    methods
}

/// Generate Rust __push_transition method
/// Uses mem::replace to move the current compartment to the stack (no clone),
/// then processes exit/enter events matching the kernel's transition logic.
fn generate_rust_push_transition(system: &SystemAst) -> CodegenNode {
    let system_name = &system.name;
    let event_class = format!("{}FrameEvent", system_name);
    let compartment_class = format!("{}Compartment", system_name);

    let code = format!(
        r#"// Exit current state (old compartment still in place for routing)
let exit_event = {event_class}::new_with_params("<$", &self.__compartment.exit_args);
self.__router(&exit_event);
// Swap: old compartment moves to stack, new takes its place
let old = std::mem::replace(&mut self.__compartment, new_compartment);
self._state_stack.push(old);
// Enter new state (or forward event) — matches kernel logic
if self.__compartment.forward_event.is_none() {{
    let enter_event = {event_class}::new_with_params("$>", &self.__compartment.enter_args);
    self.__router(&enter_event);
}} else {{
    let forward_event = self.__compartment.forward_event.take().unwrap();
    if forward_event.message == "$>" {{
        self.__router(&forward_event);
    }} else {{
        let enter_event = {event_class}::new_with_params("$>", &self.__compartment.enter_args);
        self.__router(&enter_event);
        self.__router(&forward_event);
    }}
}}"#,
        event_class = event_class
    );

    CodegenNode::Method {
        name: "__push_transition".to_string(),
        params: vec![Param::new("new_compartment").with_type(&compartment_class)],
        return_type: None,
        body: vec![CodegenNode::NativeBlock { code, span: None }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}


/// Get default initialization value for a type
fn state_var_init_value(var_type: &Type, lang: TargetLanguage) -> String {
    match var_type {
        Type::Custom(name) => {
            match name.to_lowercase().as_str() {
                "int" | "i32" | "i64" | "u32" | "u64" | "number" => "0".to_string(),
                "float" | "f32" | "f64" => "0.0".to_string(),
                "bool" | "boolean" => match lang {
                    TargetLanguage::Python3 => "False".to_string(),
                    _ => "false".to_string(),
                },
                "str" | "string" => "\"\"".to_string(),
                _ => match lang {
                    TargetLanguage::Python3 => "None".to_string(),
                    _ => "null".to_string(),
                },
            }
        }
        Type::Unknown => match lang {
            TargetLanguage::Python3 => "None".to_string(),
            _ => "null".to_string(),
        },
    }
}

/// Convert an Expression to a string representation for inline code
fn expression_to_string(expr: &Expression, lang: TargetLanguage) -> String {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => n.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Bool(b) => match lang {
                TargetLanguage::Python3 => if *b { "True".to_string() } else { "False".to_string() },
                _ => if *b { "true".to_string() } else { "false".to_string() },
            },
            Literal::Null => match lang {
                TargetLanguage::Python3 => "None".to_string(),
                _ => "null".to_string(),
            },
        },
        Expression::Var(name) => name.clone(),
        Expression::Binary { left, op, right } => {
            let op_str = match op {
                BinaryOp::Add => "+", BinaryOp::Sub => "-", BinaryOp::Mul => "*",
                BinaryOp::Div => "/", BinaryOp::Mod => "%",
                BinaryOp::Eq => "==", BinaryOp::Ne => "!=",
                BinaryOp::Lt => "<", BinaryOp::Le => "<=",
                BinaryOp::Gt => ">", BinaryOp::Ge => ">=",
                BinaryOp::And => "&&", BinaryOp::Or => "||",
                BinaryOp::BitAnd => "&", BinaryOp::BitOr => "|", BinaryOp::BitXor => "^",
            };
            format!("{} {} {}",
                expression_to_string(left, lang),
                op_str,
                expression_to_string(right, lang))
        }
        Expression::Unary { op, expr } => {
            let op_str = match op {
                UnaryOp::Not => "!", UnaryOp::Neg => "-", UnaryOp::BitNot => "~",
            };
            format!("{}{}", op_str, expression_to_string(expr, lang))
        }
        _ => "0".to_string(), // Fallback for complex expressions
    }
}

/// Generate Rust router dispatch match statement
///
/// Routes events to _state_X methods based on current compartment state
fn generate_rust_router_dispatch(system: &SystemAst) -> String {
    let mut code = String::new();
    code.push_str("match self.__compartment.state.as_str() {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            code.push_str(&format!(
                "    \"{}\" => self._state_{}(__e),\n",
                state.name, state.name
            ));
        }
    }

    code.push_str("    _ => {}\n");
    code.push_str("}");
    code
}

/// Generate C router dispatch using if-else chain with strcmp
fn generate_c_router_dispatch(system: &SystemAst) -> String {
    let sys = &system.name;
    let mut code = String::new();
    code.push_str("const char* state_name = self->__compartment->state;\n");

    if let Some(ref machine) = system.machine {
        for (i, state) in machine.states.iter().enumerate() {
            let cond = if i == 0 { "if" } else { "} else if" };
            code.push_str(&format!(
                "{} (strcmp(state_name, \"{}\") == 0) {{\n    {}_state_{}(self, __e);\n",
                cond, state.name, sys, state.name
            ));
        }
        if !machine.states.is_empty() {
            code.push_str("}");
        }
    }

    code
}


/// Generate interface wrapper methods
///
/// For Python/TypeScript: Create FrameEvent and call __kernel
/// For Rust: Use match-based dispatch directly
///
/// If no explicit interface is defined, auto-generate interface methods from
/// unique event handlers found in the machine states (excluding lifecycle events).
fn generate_interface_wrappers(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> Vec<CodegenNode> {
    // Get the target language from the syntax
    let lang = syntax.language;
    let event_class = format!("{}FrameEvent", system.name);

    // If explicit interface is defined, use it
    // Otherwise, collect unique events from state handlers
    let interface_methods: Vec<InterfaceMethod> = if !system.interface.is_empty() {
        system.interface.clone()
    } else {
        // Auto-generate interface from event handlers
        let mut events: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut method_info: std::collections::HashMap<String, (Vec<MethodParam>, Option<Type>)> = std::collections::HashMap::new();

        if let Some(ref machine) = system.machine {
            for state in &machine.states {
                for handler in &state.handlers {
                    // Skip lifecycle events
                    if handler.event == "$>" || handler.event == "<$" || handler.event == "$>|" || handler.event == "<$|" {
                        continue;
                    }
                    if events.insert(handler.event.clone()) {
                        // First time seeing this event - capture its params and return type
                        let params: Vec<MethodParam> = handler.params.iter().map(|p| {
                            MethodParam {
                                name: p.name.clone(),
                                param_type: p.param_type.clone(),
                                default: None,
                                span: Span::new(0, 0),
                            }
                        }).collect();
                        method_info.insert(handler.event.clone(), (params, handler.return_type.clone()));
                    }
                }
            }
        }

        events.into_iter().map(|event| {
            let (params, return_type) = method_info.get(&event).cloned().unwrap_or_default();
            InterfaceMethod {
                name: event,
                params,
                return_type,
                return_init: None,
                span: Span::new(0, 0),
            }
        }).collect()
    };

    interface_methods.iter().map(|method| {
        let params: Vec<Param> = method.params.iter().map(|p| {
            let type_str = type_to_string(&p.param_type);
            Param::new(&p.name).with_type(&type_str)
        }).collect();

        let _args: Vec<CodegenNode> = method.params.iter()
            .map(|p| CodegenNode::ident(&p.name))
            .collect();

        // Language-specific dispatch - all languages now use kernel pattern
        let body_stmt = match lang {
            TargetLanguage::Rust => {
                // Rust: Use context stack pattern per V4 spec
                // 1. Create FrameEvent with parameters
                // 2. Create FrameContext with event and default return
                // 3. Push context to _context_stack
                // 4. Call __kernel
                // 5. Pop context and return _return (with downcast)
                let context_class = format!("{}FrameContext", system.name);

                // Build parameters HashMap insertion code
                let params_code = if method.params.is_empty() {
                    String::new()
                } else {
                    // Clone parameters before boxing since we also pass them directly to handlers
                    method.params.iter()
                        .map(|p| format!("__e.parameters.insert(\"{}\".to_string(), Box::new({}.clone()) as Box<dyn std::any::Any>);", p.name, p.name))
                        .collect::<Vec<_>>()
                        .join("\n")
                };

                let mut match_code = format!("let mut __e = {}::new(\"{}\");\n", event_class, method.name);
                if !params_code.is_empty() {
                    match_code.push_str(&params_code);
                    match_code.push('\n');
                }

                // Create context and push to stack
                match_code.push_str(&format!("let __ctx = {}::new(__e.clone(), None);\n", context_class));
                match_code.push_str("self._context_stack.push(__ctx);\n");

                // Call kernel to route event and process transitions
                match_code.push_str("self.__kernel(__e);\n");

                // Pop context and return
                if method.return_type.is_some() {
                    let return_type = type_to_string(method.return_type.as_ref().unwrap());
                    match_code.push_str(&format!(
                        r#"let __ctx = self._context_stack.pop().unwrap();
if let Some(ret) = __ctx._return {{
    *ret.downcast::<{}>().unwrap()
}} else {{
    Default::default()
}}"#, return_type));
                } else {
                    match_code.push_str("self._context_stack.pop();");
                }

                CodegenNode::NativeBlock {
                    code: match_code,
                    span: None,
                }
            }
            TargetLanguage::Python3 => {
                // Python: Create FrameEvent + FrameContext, push context, call __kernel, pop and return
                // Parameters are passed as a dict with parameter names as keys for @@ syntax access
                let context_class = format!("{}FrameContext", system.name);
                let params_code = if method.params.is_empty() {
                    "None".to_string()
                } else {
                    let param_items: Vec<String> = method.params.iter()
                        .map(|p| format!("\"{}\": {}", p.name, p.name))
                        .collect();
                    format!("{{{}}}", param_items.join(", "))
                };

                if method.return_type.is_some() {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"__e = {}("{}", {})
__ctx = {}(__e, None)
self._context_stack.append(__ctx)
self.__kernel(__e)
return self._context_stack.pop()._return"#,
                            event_class, method.name, params_code, context_class
                        ),
                        span: None,
                    }
                } else {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"__e = {}("{}", {})
__ctx = {}(__e, None)
self._context_stack.append(__ctx)
self.__kernel(__e)
self._context_stack.pop()"#,
                            event_class, method.name, params_code, context_class
                        ),
                        span: None,
                    }
                }
            }
            TargetLanguage::TypeScript => {
                // TypeScript: Create FrameEvent + FrameContext, push context, call __kernel, pop and return
                let context_class = format!("{}FrameContext", system.name);
                // Parameters are passed as a dict with parameter names as keys for @@ syntax access
                let params_code = if method.params.is_empty() {
                    "null".to_string()
                } else {
                    let param_items: Vec<String> = method.params.iter()
                        .map(|p| format!("\"{}\": {}", p.name, p.name))
                        .collect();
                    format!("{{{}}}", param_items.join(", "))
                };

                if method.return_type.is_some() {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"const __e = new {}("{}", {});
const __ctx = new {}(__e, null);
this._context_stack.push(__ctx);
this.__kernel(__e);
return this._context_stack.pop()!._return;"#,
                            event_class, method.name, params_code, context_class
                        ),
                        span: None,
                    }
                } else {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"const __e = new {}("{}", {});
const __ctx = new {}(__e, null);
this._context_stack.push(__ctx);
this.__kernel(__e);
this._context_stack.pop();"#,
                            event_class, method.name, params_code, context_class
                        ),
                        span: None,
                    }
                }
            }
            TargetLanguage::C => {
                // C: Create FrameEvent + FrameContext, push context, call kernel, pop and return
                let sys = &system.name;

                // Build parameters dict creation (with semicolon)
                let params_code = if method.params.is_empty() {
                    format!("{}_FrameEvent* __e = {}_FrameEvent_new(\"{}\", NULL);", sys, sys, method.name)
                } else {
                    let mut code = format!("{}_FrameDict* __params = {}_FrameDict_new();\n", sys, sys);
                    for p in &method.params {
                        code.push_str(&format!("{}_FrameDict_set(__params, \"{}\", (void*)(intptr_t){});\n", sys, p.name, p.name));
                    }
                    code.push_str(&format!("{}_FrameEvent* __e = {}_FrameEvent_new(\"{}\", __params);", sys, sys, method.name));
                    code
                };

                // Check if method has a non-void return type
                let return_type_str = method.return_type.as_ref().map(|t| type_to_string(t));
                // Convert Frame types to C types
                let return_type_str = return_type_str.map(|s| {
                    match s.as_str() {
                        "str" | "string" | "String" => "char*".to_string(),
                        "bool" | "boolean" => "bool".to_string(),
                        "int" | "number" | "Any" => "int".to_string(),
                        "float" | "double" => "double".to_string(),
                        "void" | "None" => "void".to_string(),
                        _ => s
                    }
                });
                let has_return_value = return_type_str.as_ref()
                    .map(|s| s != "void" && s != "None")
                    .unwrap_or(false);

                // Generate default return value for FrameContext_new
                let default_return = if let Some(ref init_expr) = method.return_init {
                    // Cast the expression to void* for storage
                    format!("(void*)(intptr_t)({})", init_expr)
                } else {
                    "NULL".to_string()
                };

                if has_return_value {
                    let return_type_str = return_type_str.unwrap();
                    let cast = match return_type_str.as_str() {
                        "bool" | "int" => "(intptr_t)",
                        _ => "",
                    };
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"{}
{}_FrameContext* __ctx = {}_FrameContext_new(__e, {});
{}_FrameVec_push(self->_context_stack, __ctx);
{}_kernel(self, __e);
{}_FrameContext* __result_ctx = ({}_FrameContext*){}_FrameVec_pop(self->_context_stack);
{} __result = ({}){}__result_ctx->_return;
{}_FrameContext_destroy(__result_ctx);
{}_FrameEvent_destroy(__e);
return __result;"#,
                            params_code, sys, sys, default_return, sys, sys, sys, sys, sys,
                            return_type_str, return_type_str, cast, sys, sys
                        ),
                        span: None,
                    }
                } else {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"{}
{}_FrameContext* __ctx = {}_FrameContext_new(__e, {});
{}_FrameVec_push(self->_context_stack, __ctx);
{}_kernel(self, __e);
{}_FrameContext* __result_ctx = ({}_FrameContext*){}_FrameVec_pop(self->_context_stack);
{}_FrameContext_destroy(__result_ctx);
{}_FrameEvent_destroy(__e);"#,
                            params_code, sys, sys, default_return, sys, sys, sys, sys, sys, sys, sys
                        ),
                        span: None,
                    }
                }
            }
            _ => {
                // Default: Same as TypeScript with context stack
                let context_class = format!("{}FrameContext", system.name);
                // Parameters are passed as a dict with parameter names as keys for @@ syntax access
                let params_code = if method.params.is_empty() {
                    "null".to_string()
                } else {
                    let param_items: Vec<String> = method.params.iter()
                        .map(|p| format!("\"{}\": {}", p.name, p.name))
                        .collect();
                    format!("{{{}}}", param_items.join(", "))
                };
                CodegenNode::NativeBlock {
                    code: format!(
                        r#"const __e = new {}("{}", {});
const __ctx = new {}(__e, null);
this._context_stack.push(__ctx);
this.__kernel(__e);
this._context_stack.pop();"#,
                        event_class, method.name, params_code, context_class
                    ),
                    span: None,
                }
            }
        };

        CodegenNode::Method {
            name: method.name.clone(),
            params,
            return_type: method.return_type.as_ref().map(|t| type_to_string(t)),
            body: vec![body_stmt],
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            decorators: vec![],
        }
    }).collect()
}

/// Generate state handler methods using the enhanced Arcanum
///
/// For all languages: Generates `_state_{StateName}(__e)` methods that dispatch internally
/// based on the event message, plus individual handler methods
fn generate_state_handlers_via_arcanum(system_name: &str, machine: &MachineAst, arcanum: &Arcanum, source: &[u8], lang: TargetLanguage, has_state_vars: bool) -> Vec<CodegenNode> {
    let mut methods = Vec::new();

    // Collect all defined system names for @@System() validation
    let defined_systems: std::collections::HashSet<String> = arcanum.systems.keys().cloned().collect();

    // Generate one _state_{StateName} dispatch method per state for ALL languages
    for state_entry in arcanum.get_enhanced_states(system_name) {
        // Find state variables and default_forward for this state from the machine AST
        let state_ast = machine.states.iter().find(|s| s.name == state_entry.name);
        let state_vars = state_ast.map(|s| &s.state_vars[..]).unwrap_or(&[]);
        // V4: Enable default_forward ONLY if explicitly set with `=> $^` in state body
        // Having a parent (HSM) does NOT imply auto-forwarding
        let has_explicit_forward = state_ast.map(|s| s.default_forward).unwrap_or(false);
        let default_forward = has_explicit_forward;

        let method = generate_state_method(
            system_name,
            &state_entry.name,
            state_entry.parent.as_deref(),
            &state_entry.handlers,
            state_vars,
            source,
            lang,
            has_state_vars,
            default_forward,
            &defined_systems,
        );
        methods.push(method);
    }

    // For Rust: Also generate individual handler methods that the dispatch calls
    // (Python/TypeScript inline the handler code in the dispatch method)
    if matches!(lang, TargetLanguage::Rust) {
        for state_entry in arcanum.get_enhanced_states(system_name) {
            for (_event, handler_entry) in &state_entry.handlers {
                let method = generate_handler_from_arcanum(
                    system_name,
                    &state_entry.name,
                    state_entry.parent.as_deref(),
                    handler_entry,
                    source,
                    lang,
                    has_state_vars,
                    &defined_systems,
                );
                methods.push(method);
            }
        }
    }

    methods
}

/// Generate a `__state_{StateName}(__e)` method for Python/TypeScript
///
/// The method receives a FrameEvent and dispatches based on __e._message
fn generate_state_method(
    _system_name: &str,
    state_name: &str,
    parent_state: Option<&str>,
    handlers: &std::collections::HashMap<String, HandlerEntry>,
    state_vars: &[StateVarAst],
    source: &[u8],
    lang: TargetLanguage,
    _has_state_vars: bool,
    default_forward: bool,
    defined_systems: &std::collections::HashSet<String>,
) -> CodegenNode {
    // Use single underscore prefix to avoid Python name mangling
    // Python mangles __name to _ClassName__name, which breaks dynamic lookup
    let method_name = format!("_state_{}", state_name);

    // Build context for HSM forwarding
    // use_sv_comp is true when this state has state vars - we'll navigate to correct compartment
    let ctx = HandlerContext {
        system_name: _system_name.to_string(),
        state_name: state_name.to_string(),
        event_name: String::new(), // Will be set per-handler
        parent_state: parent_state.map(|s| s.to_string()),
        defined_systems: defined_systems.clone(),
        use_sv_comp: !state_vars.is_empty(),
    };

    // Generate the dispatch body based on __e._message / __e.message
    let body_code = match lang {
        TargetLanguage::Python3 => generate_python_state_dispatch(_system_name, state_name, handlers, state_vars, source, &ctx, default_forward),
        TargetLanguage::TypeScript => generate_typescript_state_dispatch(_system_name, state_name, handlers, state_vars, source, &ctx, default_forward),
        TargetLanguage::Rust => generate_rust_state_dispatch(_system_name, state_name, handlers, state_vars, parent_state, default_forward),
        TargetLanguage::C => generate_c_state_dispatch(_system_name, state_name, handlers, state_vars, source, &ctx, default_forward),
        _ => String::new(),
    };

    let params = match lang {
        TargetLanguage::TypeScript => {
            let event_type = format!("{}FrameEvent", _system_name);
            vec![Param::new("__e").with_type(&event_type)]
        }
        TargetLanguage::Rust => {
            let event_type = format!("&{}FrameEvent", _system_name);
            vec![Param::new("__e").with_type(&event_type)]
        }
        TargetLanguage::C => {
            let event_type = format!("{}_FrameEvent*", _system_name);
            vec![Param::new("__e").with_type(&event_type)]
        }
        _ => vec![Param::new("__e")],
    };

    CodegenNode::Method {
        name: method_name,
        params,
        return_type: None,
        body: vec![CodegenNode::NativeBlock {
            code: body_code,
            span: None,
        }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate Python state dispatch code (if/elif chain on __e._message)
fn generate_python_state_dispatch(
    _system_name: &str,
    state_name: &str,
    handlers: &std::collections::HashMap<String, HandlerEntry>,
    state_vars: &[StateVarAst],
    source: &[u8],
    ctx: &HandlerContext,
    default_forward: bool,
) -> String {
    let mut code = String::new();
    let mut first = true;
    let has_enter_handler = handlers.contains_key("$>") || handlers.contains_key("enter");

    // HSM Compartment Navigation: When this handler accesses state vars, we need to ensure
    // we're accessing the correct compartment. If this handler was invoked via forwarding
    // from a child state, __compartment points to the child's compartment, not this state's.
    // Navigate the parent_compartment chain to find this state's compartment.
    // The while loop is a no-op if we're already in this state's compartment directly.
    if !state_vars.is_empty() {
        code.push_str(&format!(
            r#"# HSM: Navigate to this state's compartment for state var access
__sv_comp = self.__compartment
while __sv_comp is not None and __sv_comp.state != "{}":
    __sv_comp = __sv_comp.parent_compartment
"#, state_name));
    }

    // If state has state variables but no explicit $> handler, generate one
    // Use conditional initialization to preserve values on pop-restore
    // Uses __sv_comp which was set up in preamble for HSM compartment navigation
    if !state_vars.is_empty() && !has_enter_handler {
        code.push_str("if __e._message == \"$>\":\n");
        for var in state_vars {
            let init_val = if let Some(ref init) = var.init {
                expression_to_string(init, TargetLanguage::Python3)
            } else {
                state_var_init_value(&var.var_type, TargetLanguage::Python3)
            };
            // Only initialize if not already set (preserves pop-restored values)
            code.push_str(&format!("    if \"{}\" not in __sv_comp.state_vars:\n", var.name));
            code.push_str(&format!("        __sv_comp.state_vars[\"{}\"] = {}\n", var.name, init_val));
        }
        first = false;
    }

    // Sort handlers for deterministic output
    let mut sorted_handlers: Vec<_> = handlers.iter().collect();
    sorted_handlers.sort_by_key(|(event, _)| *event);

    for (event, handler) in sorted_handlers {
        // Map Frame events to their message names
        let message = match event.as_str() {
            "$>" | "enter" => "$>",
            "$<" | "exit" => "<$",
            _ => event.as_str(),
        };

        let condition = if first {
            format!("if __e._message == \"{}\":", message)
        } else {
            format!("elif __e._message == \"{}\":", message)
        };
        first = false;

        code.push_str(&condition);
        code.push('\n');

        // For enter handlers with state vars, initialize state vars first
        // Use conditional initialization to preserve values on pop-restore
        // Uses __sv_comp which was set up in preamble for HSM compartment navigation
        if (event == "$>" || event == "enter") && !state_vars.is_empty() {
            for var in state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::Python3)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::Python3)
                };
                // Only initialize if not already set (preserves pop-restored values)
                code.push_str(&format!("    if \"{}\" not in __sv_comp.state_vars:\n", var.name));
                code.push_str(&format!("        __sv_comp.state_vars[\"{}\"] = {}\n", var.name, init_val));
            }
        }

        // Generate parameter unpacking if handler has params
        // For enter/exit handlers, use positional indices (transition args are positional)
        // For other handlers, use parameter names as keys (matching interface method generation)
        let is_lifecycle_handler = event == "$>" || event == "enter" || event == "$<" || event == "exit" || event == "<$";
        for (i, param) in handler.params.iter().enumerate() {
            if is_lifecycle_handler {
                // Lifecycle handlers receive positional args from transition
                code.push_str(&format!("    {} = __e._parameters[\"{}\"]\n", param.name, i));
            } else {
                // Interface handlers receive named args
                code.push_str(&format!("    {} = __e._parameters[\"{}\"]\n", param.name, param.name));
            }
        }

        // Generate the handler body
        let mut handler_ctx = ctx.clone();
        handler_ctx.event_name = event.clone();
        let body = splice_handler_body_from_span(&handler.body_span, source, TargetLanguage::Python3, &handler_ctx);

        // Indent the body
        let mut body_has_content = false;
        for line in body.lines() {
            if !line.trim().is_empty() {
                code.push_str("    ");
                code.push_str(line);
                body_has_content = true;
            }
            code.push('\n');
        }

        // If body was empty (no statements), add pass to avoid IndentationError
        if !body_has_content {
            code.push_str("    pass\n");
        }
    }

    // Add default forward clause if state has => $^ at state level
    if default_forward {
        if let Some(ref parent) = ctx.parent_state {
            // Only add else clause if we have at least one if/elif above
            if !first {
                code.push_str("else:\n");
                code.push_str(&format!("    self._state_{}(__e)\n", parent));
            } else {
                // No handlers at all - just forward everything
                code.push_str(&format!("self._state_{}(__e)\n", parent));
            }
        }
    }

    // Trim trailing newlines
    code.trim_end().to_string()
}

/// Generate TypeScript state dispatch code (if/else chain on __e._message)
fn generate_typescript_state_dispatch(
    _system_name: &str,
    state_name: &str,
    handlers: &std::collections::HashMap<String, HandlerEntry>,
    state_vars: &[StateVarAst],
    source: &[u8],
    ctx: &HandlerContext,
    default_forward: bool,
) -> String {
    let mut code = String::new();
    let mut first = true;
    let has_enter_handler = handlers.contains_key("$>") || handlers.contains_key("enter");

    // HSM Compartment Navigation: When this handler accesses state vars, we need to ensure
    // we're accessing the correct compartment. If this handler was invoked via forwarding
    // from a child state, __compartment points to the child's compartment, not this state's.
    // Navigate the parent_compartment chain to find this state's compartment.
    if !state_vars.is_empty() {
        code.push_str(&format!(
            r#"// HSM: Navigate to this state's compartment for state var access
let __sv_comp: any = this.__compartment;
while (__sv_comp !== null && __sv_comp.state !== "{}") {{
    __sv_comp = __sv_comp.parent_compartment;
}}
"#, state_name));
    }

    // If state has state variables but no explicit $> handler, generate one
    // Use conditional initialization to preserve values on pop-restore
    if !state_vars.is_empty() && !has_enter_handler {
        code.push_str("if (__e._message === \"$>\") {\n");
        for var in state_vars {
            let init_val = if let Some(ref init) = var.init {
                expression_to_string(init, TargetLanguage::TypeScript)
            } else {
                state_var_init_value(&var.var_type, TargetLanguage::TypeScript)
            };
            // Only initialize if not already set (preserves pop-restored values)
            code.push_str(&format!("    if (!(\"{0}\" in __sv_comp.state_vars)) {{\n", var.name));
            code.push_str(&format!("        __sv_comp.state_vars[\"{}\"] = {};\n", var.name, init_val));
            code.push_str("    }\n");
        }
        first = false;
    }

    // Sort handlers for deterministic output
    let mut sorted_handlers: Vec<_> = handlers.iter().collect();
    sorted_handlers.sort_by_key(|(event, _)| *event);

    for (event, handler) in sorted_handlers {
        // Map Frame events to their message names
        let message = match event.as_str() {
            "$>" | "enter" => "$>",
            "$<" | "exit" => "<$",
            _ => event.as_str(),
        };

        let condition = if first {
            format!("if (__e._message === \"{}\") {{", message)
        } else {
            format!("}} else if (__e._message === \"{}\") {{", message)
        };
        first = false;

        code.push_str(&condition);
        code.push('\n');

        // For enter handlers with state vars, initialize state vars first
        // Use conditional initialization to preserve values on pop-restore
        // Uses __sv_comp which was set up in preamble for HSM compartment navigation
        if (event == "$>" || event == "enter") && !state_vars.is_empty() {
            for var in state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::TypeScript)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::TypeScript)
                };
                // Only initialize if not already set (preserves pop-restored values)
                code.push_str(&format!("    if (!(\"{0}\" in __sv_comp.state_vars)) {{\n", var.name));
                code.push_str(&format!("        __sv_comp.state_vars[\"{}\"] = {};\n", var.name, init_val));
                code.push_str("    }\n");
            }
        }

        // Generate parameter unpacking if handler has params
        // For enter/exit handlers, use positional indices (transition args are positional)
        // For other handlers, use parameter names as keys (matching interface method generation)
        let is_lifecycle_handler = event == "$>" || event == "enter" || event == "$<" || event == "exit" || event == "<$";
        for (i, param) in handler.params.iter().enumerate() {
            if is_lifecycle_handler {
                // Lifecycle handlers receive positional args from transition
                code.push_str(&format!("    const {} = __e._parameters?.[\"{}\"];\n", param.name, i));
            } else {
                // Interface handlers receive named args
                code.push_str(&format!("    const {} = __e._parameters?.[\"{}\"];\n", param.name, param.name));
            }
        }

        // Generate the handler body
        let mut handler_ctx = ctx.clone();
        handler_ctx.event_name = event.clone();
        let body = splice_handler_body_from_span(&handler.body_span, source, TargetLanguage::TypeScript, &handler_ctx);

        // Indent the body
        for line in body.lines() {
            if !line.trim().is_empty() {
                code.push_str("    ");
                code.push_str(line);
            }
            code.push('\n');
        }
    }

    // Add default forward clause or close the last if block
    if default_forward {
        if let Some(ref parent) = ctx.parent_state {
            if !first {
                // Close previous block and add else clause
                code.push_str("} else {\n");
                code.push_str(&format!("    this._state_{}(__e);\n", parent));
                code.push_str("}");
            } else {
                // No handlers at all - just forward everything
                code.push_str(&format!("this._state_{}(__e);", parent));
            }
        } else if !first {
            code.push_str("}");
        }
    } else if !first {
        code.push_str("}");
    }

    code
}

/// Generate C state dispatch code (if-else chain with strcmp)
fn generate_c_state_dispatch(
    system_name: &str,
    state_name: &str,
    handlers: &std::collections::HashMap<String, HandlerEntry>,
    state_vars: &[StateVarAst],
    source: &[u8],
    ctx: &HandlerContext,
    default_forward: bool,
) -> String {
    let mut code = String::new();
    let mut first = true;
    let has_enter_handler = handlers.contains_key("$>") || handlers.contains_key("enter");

    // HSM Compartment Navigation: When this handler accesses state vars, we need to ensure
    // we're accessing the correct compartment. If this handler was invoked via forwarding
    // from a child state, __compartment points to the child's compartment, not this state's.
    // Navigate the parent_compartment chain to find this state's compartment.
    if !state_vars.is_empty() {
        code.push_str(&format!(
            r#"// HSM: Navigate to this state's compartment for state var access
{}_Compartment* __sv_comp = self->__compartment;
while (__sv_comp != NULL && strcmp(__sv_comp->state, "{}") != 0) {{
    __sv_comp = __sv_comp->parent_compartment;
}}
"#, system_name, state_name));
    }

    // If state has state variables but no explicit $> handler, generate one
    // Use conditional initialization to preserve values on pop-restore
    if !state_vars.is_empty() && !has_enter_handler {
        code.push_str("if (strcmp(__e->_message, \"$>\") == 0) {\n");
        for var in state_vars {
            let init_val = if let Some(ref init) = var.init {
                expression_to_string(init, TargetLanguage::C)
            } else {
                state_var_init_value(&var.var_type, TargetLanguage::C)
            };
            // Only initialize if not already set (preserves pop-restored values)
            // Use __sv_comp which was set up in preamble for HSM compartment navigation
            code.push_str(&format!("    if (!{}_FrameDict_has(__sv_comp->state_vars, \"{}\")) {{\n",
                system_name, var.name));
            code.push_str(&format!("        {}_FrameDict_set(__sv_comp->state_vars, \"{}\", (void*)(intptr_t){});\n",
                system_name, var.name, init_val));
            code.push_str("    }\n");
        }
        first = false;
    }

    // Sort handlers for deterministic output
    let mut sorted_handlers: Vec<_> = handlers.iter().collect();
    sorted_handlers.sort_by_key(|(event, _)| *event);

    for (event, handler) in sorted_handlers {
        // Map Frame events to their message names
        let message = match event.as_str() {
            "$>" | "enter" => "$>",
            "$<" | "exit" => "<$",
            _ => event.as_str(),
        };

        let condition = if first {
            format!("if (strcmp(__e->_message, \"{}\") == 0) {{", message)
        } else {
            format!("}} else if (strcmp(__e->_message, \"{}\") == 0) {{", message)
        };
        first = false;

        code.push_str(&condition);
        code.push('\n');

        // For enter handlers with state vars, initialize state vars first
        // Use conditional initialization to preserve values on pop-restore
        // Uses __sv_comp which was set up in preamble for HSM compartment navigation
        if (event == "$>" || event == "enter") && !state_vars.is_empty() {
            for var in state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::C)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::C)
                };
                // Only initialize if not already set (preserves pop-restored values)
                code.push_str(&format!("    if (!{}_FrameDict_has(__sv_comp->state_vars, \"{}\")) {{\n",
                    system_name, var.name));
                code.push_str(&format!("        {}_FrameDict_set(__sv_comp->state_vars, \"{}\", (void*)(intptr_t){});\n",
                    system_name, var.name, init_val));
                code.push_str("    }\n");
            }
        }

        // Generate parameter unpacking if handler has params
        // For enter/exit handlers, use positional indices (transition args are positional)
        // For other handlers, use parameter names as keys (matching interface method generation)
        let is_lifecycle_handler = event == "$>" || event == "enter" || event == "$<" || event == "exit" || event == "<$";
        for (i, param) in handler.params.iter().enumerate() {
            let param_type = param.symbol_type.as_ref().map(|s| s.as_str()).unwrap_or("int");
            let c_type = match param_type {
                "int" | "i32" | "i64" => "int",
                "bool" | "boolean" => "bool",
                "float" | "double" | "f32" | "f64" => "double",
                "str" | "string" | "String" => "char*",
                _ => "void*",
            };
            let cast = if c_type == "int" || c_type == "bool" { "(intptr_t)" } else { "" };
            if is_lifecycle_handler {
                // Lifecycle handlers receive positional args from transition
                code.push_str(&format!("    {} {} = ({}){}{}_FrameDict_get(__e->_parameters, \"{}\");\n",
                    c_type, param.name, c_type, cast, system_name, i));
            } else {
                // Interface handlers receive named args
                code.push_str(&format!("    {} {} = ({}){}{}_FrameDict_get(__e->_parameters, \"{}\");\n",
                    c_type, param.name, c_type, cast, system_name, param.name));
            }
        }

        // Generate the handler body
        let mut handler_ctx = ctx.clone();
        handler_ctx.event_name = event.clone();
        let body = splice_handler_body_from_span(&handler.body_span, source, TargetLanguage::C, &handler_ctx);

        // Indent the body
        for line in body.lines() {
            if !line.trim().is_empty() {
                code.push_str("    ");
                code.push_str(line);
            }
            code.push('\n');
        }
    }

    // Add default forward clause or close the last if block
    if default_forward {
        if let Some(ref parent) = ctx.parent_state {
            if !first {
                // Close previous block and add else clause
                code.push_str("} else {\n");
                code.push_str(&format!("    {}_state_{}(self, __e);\n", system_name, parent));
                code.push_str("}");
            } else {
                // No handlers at all - just forward everything
                code.push_str(&format!("{}_state_{}(self, __e);", system_name, parent));
            }
        } else if !first {
            code.push_str("}");
        }
    } else if !first {
        code.push_str("}");
    }

    code
}

/// Generate Rust state dispatch code (match on __e.message)
///
/// Unlike Python/TypeScript which inline handler code, Rust dispatches to separate methods
fn generate_rust_state_dispatch(
    _system_name: &str,
    state_name: &str,
    handlers: &std::collections::HashMap<String, HandlerEntry>,
    state_vars: &[StateVarAst],
    parent_state: Option<&str>,
    default_forward: bool,
) -> String {
    let mut code = String::new();
    code.push_str("match __e.message.as_str() {\n");

    // Sort handlers for deterministic output
    let mut sorted_handlers: Vec<_> = handlers.iter().collect();
    sorted_handlers.sort_by_key(|(event, _)| *event);

    // Track if we need to initialize state vars in $>
    // (State vars now live on compartment.state_context — no enter-handler init needed)
    let _has_enter_handler = handlers.contains_key("$>") || handlers.contains_key("enter");
    let _needs_state_var_init = !state_vars.is_empty();

    for (event, handler) in sorted_handlers {
        // Map Frame events to their message names
        let message = match event.as_str() {
            "$>" | "enter" => "$>",
            "$<" | "exit" => "<$",
            _ => event.as_str(),
        };

        // Determine handler method name
        let handler_method = match event.as_str() {
            "$>" | "enter" => format!("_s_{}_enter", state_name),
            "$<" | "exit" => format!("_s_{}_exit", state_name),
            _ => format!("_s_{}_{}", state_name, event),
        };

        // Handle enter/exit handlers with parameters specially - extract params from event
        let is_lifecycle = event == "$>" || event == "enter" || event == "$<" || event == "exit" || event == "<$";
        if !handler.params.is_empty() && is_lifecycle {
            // Extract parameters from event and call handler
            code.push_str(&format!("    \"{}\" => {{\n", message));

            // State vars live on compartment.state_context — no init needed in enter handler

            // Extract parameters and call handler
            for (i, param) in handler.params.iter().enumerate() {
                code.push_str(&format!("        let {} = __e.parameters.get(\"{}\").and_then(|v| v.downcast_ref::<String>()).cloned().unwrap_or_default();\n", param.name, i));
            }
            let param_names: Vec<_> = handler.params.iter().map(|p| p.name.clone()).collect();
            code.push_str(&format!("        self.{}(__e, {});\n", handler_method, param_names.join(", ")));
            code.push_str("    }\n");
            continue;
        }

        // Handle non-lifecycle handlers with parameters - extract from event
        if !handler.params.is_empty() {
            code.push_str(&format!("    \"{}\" => {{\n", message));
            for param in &handler.params {
                // Extract parameter from event, downcast to the appropriate type
                // For now, use String and let the handler deal with typing
                let param_type = param.symbol_type.as_deref().unwrap_or("String");
                code.push_str(&format!(
                    "        let {}: {} = __e.parameters.get(\"{}\").and_then(|v| v.downcast_ref::<{}>()).cloned().unwrap_or_default();\n",
                    param.name, param_type, param.name, param_type
                ));
            }
            let param_names: Vec<_> = handler.params.iter().map(|p| p.name.clone()).collect();
            code.push_str(&format!("        self.{}(__e, {});\n", handler_method, param_names.join(", ")));
            code.push_str("    }\n");
            continue;
        }

        // State vars live on compartment.state_context — no init needed in enter handler
        // Use block syntax to ignore handler return value (dispatch doesn't return)
        code.push_str(&format!("    \"{}\" => {{ self.{}(__e); }}\n", message, handler_method));
    }

    // State vars live on compartment.state_context — no auto-generated $> init needed

    // Default case - forward to parent if default_forward, else do nothing
    if default_forward {
        if let Some(parent) = parent_state {
            code.push_str(&format!("    _ => self._state_{}(__e),\n", parent));
        } else {
            code.push_str("    _ => {}\n");
        }
    } else {
        code.push_str("    _ => {}\n");
    }

    code.push_str("}");
    code
}

/// Generate a handler method from Arcanum's HandlerEntry
///
/// Uses the handler's body_span to extract and splice native code with Frame expansions.
fn generate_handler_from_arcanum(
    system_name: &str,
    state_name: &str,
    parent_state: Option<&str>,
    handler: &HandlerEntry,
    source: &[u8],
    lang: TargetLanguage,
    _has_state_vars: bool,
    defined_systems: &std::collections::HashSet<String>,
) -> CodegenNode {
    // Build params from handler's parameter symbols
    // V4 uses native types, so we just pass them through as-is
    // For Rust: Add __e: &FrameEvent as first param
    let mut params: Vec<Param> = Vec::new();

    if matches!(lang, TargetLanguage::Rust) {
        // Rust handlers receive the FrameEvent reference
        let event_type = format!("&{}FrameEvent", system_name);
        params.push(Param::new("__e").with_type(&event_type));
    }

    // Add handler parameters
    for p in &handler.params {
        let type_str = p.symbol_type.as_deref().unwrap_or("Any");
        // Clean up the type string (remove "Some(" prefix if present from debug format)
        let clean_type = if type_str.starts_with("Some(") {
            type_str.trim_start_matches("Some(").trim_end_matches(")")
        } else {
            type_str
        };
        params.push(Param::new(&p.name).with_type(clean_type));
    }

    // Determine method name based on handler type
    let method_name = if handler.is_enter {
        format!("_s_{}_enter", state_name)
    } else if handler.is_exit {
        format!("_s_{}_exit", state_name)
    } else {
        format!("_s_{}_{}", state_name, handler.event)
    };

    // Build context for HSM forwarding
    let ctx = HandlerContext {
        system_name: system_name.to_string(),
        state_name: state_name.to_string(),
        event_name: handler.event.clone(),
        parent_state: parent_state.map(|s| s.to_string()),
        defined_systems: defined_systems.clone(),
        use_sv_comp: false, // Handler-specific methods don't have __sv_comp preamble
    };

    // Splice the handler body: preserve native code, expand Frame segments
    let body_code = splice_handler_body_from_span(&handler.body_span, source, lang, &ctx);

    // Note: Rust handlers are now void (context stack pattern), no need to strip trailing semicolons

    // For TypeScript and Rust, don't put return types on handler methods
    // The interface wrappers handle returns via context stack pattern
    // Handlers set @@:return (context._return) and the interface method returns it
    let method_return_type = match lang {
        TargetLanguage::TypeScript | TargetLanguage::Rust => None,
        _ => handler.return_type.clone(),
    };

    CodegenNode::Method {
        name: method_name,
        params,
        return_type: method_return_type,
        body: vec![CodegenNode::NativeBlock {
            code: body_code,
            span: Some(crate::frame_c::v4::frame_ast::Span {
                start: handler.body_span.start,
                end: handler.body_span.end,
            }),
        }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Splice handler body from a span (used by Arcanum-based generation)
fn splice_handler_body_from_span(span: &crate::frame_c::v4::ast::Span, source: &[u8], lang: TargetLanguage, ctx: &HandlerContext) -> String {
    // Ensure span is within bounds
    if span.start >= source.len() || span.end > source.len() || span.start >= span.end {
        return String::new();
    }

    let body_bytes = &source[span.start..span.end];

    // Find the opening brace
    let open_brace = body_bytes.iter().position(|&b| b == b'{');
    if open_brace.is_none() {
        // No brace found - return the content as-is (might be a simple body)
        return String::from_utf8_lossy(body_bytes).trim().to_string();
    }

    // Scan for Frame segments within the body
    let mut scanner = get_native_scanner(lang);
    let scan_result = match scanner.scan(body_bytes, open_brace.unwrap()) {
        Ok(r) => r,
        Err(_) => return String::new(),
    };

    // Generate expansions for each Frame segment
    let mut expansions = Vec::new();
    for region in &scan_result.regions {
        if let Region::FrameSegment { span, kind, indent } = region {
            let expansion = generate_frame_expansion(body_bytes, span, *kind, *indent, lang, ctx);
            expansions.push(expansion);
        }
    }

    // Use splicer to combine native + generated Frame code
    let splicer = Splicer;
    let spliced = splicer.splice(body_bytes, &scan_result.regions, &expansions);

    if std::env::var("FRAME_DEBUG_SPLICER").is_ok() {
        eprintln!("[splice_handler_body_from_span] Spliced result: {:?}", spliced.text);
    }

    // The splicer produces content WITHOUT the outer braces
    // Normalize indentation: remove common leading whitespace from all lines
    let text = spliced.text.trim_start_matches('\n').trim_end();
    normalize_indentation(text)
}

/// Normalize indentation by removing common leading whitespace from all lines
fn normalize_indentation(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    // Find minimum indentation (ignoring empty lines)
    let min_indent = lines.iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    // Strip the common indentation from all lines
    lines.iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                line.trim()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}


/// Generate code expansion for a Frame segment
///
/// NOTE: The scanner leaves a gap between NativeText and FrameSegment where leading
/// whitespace lives. Since the splicer doesn't copy this gap, we MUST include the
/// indentation in the expansion to preserve proper code structure.
fn generate_frame_expansion(body_bytes: &[u8], span: &crate::frame_c::v4::native_region_scanner::RegionSpan, kind: FrameSegmentKind, indent: usize, lang: TargetLanguage, ctx: &HandlerContext) -> String {
    let segment_text = String::from_utf8_lossy(&body_bytes[span.start..span.end]);
    // Use scanner's indent value to match native code indentation
    // This ensures Frame expansions align with surrounding native code
    let indent_str = " ".repeat(indent);

    match kind {
        FrameSegmentKind::Transition => {
            // Parse transition: (exit_args)? -> (enter_args)? $State(state_args)?
            // For Python/TypeScript: Create compartment and call __transition()
            // For Rust: Use simpler _transition() approach

            // Check for pop-transition: -> pop$
            if segment_text.contains("pop$") {
                // Pop-transition: pop state from stack and transition to it
                match lang {
                    TargetLanguage::Python3 => format!(
                        "{}__saved = self._state_stack.pop()\n{}self.__transition(__saved)",
                        indent_str, indent_str
                    ),
                    TargetLanguage::TypeScript => format!(
                        "{}const __saved = this._state_stack.pop()!;\n{}this.__transition(__saved);",
                        indent_str, indent_str
                    ),
                    TargetLanguage::Rust => format!(
                        "{}let __popped = self._state_stack.pop().unwrap();\n{}self.__transition(__popped);\n{}return;",
                        indent_str, indent_str, indent_str
                    ),
                    TargetLanguage::C => format!(
                        "{}{}_Compartment* __saved = ({}_Compartment*){}_FrameVec_pop(self->_state_stack);\n{}{}_transition(self, __saved);",
                        indent_str, ctx.system_name, ctx.system_name, ctx.system_name, indent_str, ctx.system_name
                    ),
                    _ => format!(
                        "{}const __saved = this._state_stack.pop()!;\n{}this.__transition(__saved);",
                        indent_str, indent_str
                    ),
                }
            } else {
                let target = extract_transition_target(&segment_text);
                let (exit_args, enter_args) = extract_transition_args(&segment_text);
                let state_args = extract_state_args(&segment_text);

                // Expand state variable references in arguments
                let exit_str = exit_args.map(|a| expand_state_vars_in_expr(&a, lang, ctx));
                let enter_str = enter_args.map(|a| expand_state_vars_in_expr(&a, lang, ctx));
                let state_str = state_args.map(|a| expand_state_vars_in_expr(&a, lang, ctx));

                // Get compartment class name from system name
                let _compartment_class = format!("{}Compartment", ctx.system_name);

                match lang {
                    TargetLanguage::Python3 => {
                        // Create compartment, set fields, call __transition
                        // Store exit_args in CURRENT compartment before creating new one
                        let mut code = String::new();

                        // Store exit_args in current compartment if present
                        // Use string keys for consistency with parameter unpacking
                        if let Some(ref exit) = exit_str {
                            code.push_str(&format!("{}self.__compartment.exit_args = {{str(i): v for i, v in enumerate(({},))}}\n", indent_str, exit));
                        }

                        // Create new compartment with parent_compartment for HSM support
                        code.push_str(&format!("{}__compartment = {}Compartment(\"{}\", parent_compartment=self.__compartment.copy())\n", indent_str, ctx.system_name, target));

                        // Set state_args if present
                        if let Some(ref state) = state_str {
                            let args: Vec<&str> = state.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
                            if !args.is_empty() {
                                let entries: Vec<String> = args.iter().enumerate()
                                    .map(|(i, a)| {
                                        // Check for named argument (e.g., "k=3")
                                        if let Some(eq_pos) = a.find('=') {
                                            let name = a[..eq_pos].trim();
                                            let value = a[eq_pos + 1..].trim();
                                            format!("\"{}\": {}", name, value)
                                        } else {
                                            // Positional argument - use index as key
                                            format!("\"{}\": {}", i, a)
                                        }
                                    })
                                    .collect();
                                code.push_str(&format!("{}__compartment.state_args = {{{}}}\n", indent_str, entries.join(", ")));
                            }
                        }

                        // Set enter_args if present
                        // Use string keys for consistency with parameter unpacking
                        if let Some(ref enter) = enter_str {
                            code.push_str(&format!("{}__compartment.enter_args = {{str(i): v for i, v in enumerate(({},))}}\n", indent_str, enter));
                        }

                        // Call __transition and return to exit the handler
                        code.push_str(&format!("{}self.__transition(__compartment)\n{}return", indent_str, indent_str));
                        code
                    }
                    TargetLanguage::TypeScript => {
                        // Create compartment, set fields, call __transition
                        let mut code = String::new();

                        // Store exit_args in current compartment if present
                        if let Some(ref exit) = exit_str {
                            code.push_str(&format!("{}this.__compartment.exit_args = Object.fromEntries([{}].map((v, i) => [String(i), v]));\n", indent_str, exit));
                        }

                        // Create new compartment with parent_compartment for HSM support
                        code.push_str(&format!("{}const __compartment = new {}Compartment(\"{}\", this.__compartment.copy());\n", indent_str, ctx.system_name, target));

                        // Set state_args if present
                        if let Some(ref state) = state_str {
                            let args: Vec<&str> = state.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
                            if !args.is_empty() {
                                let entries: Vec<String> = args.iter().enumerate()
                                    .map(|(i, a)| {
                                        // Check for named argument (e.g., "k=3")
                                        if let Some(eq_pos) = a.find('=') {
                                            let name = a[..eq_pos].trim();
                                            let value = a[eq_pos + 1..].trim();
                                            format!("\"{}\": {}", name, value)
                                        } else {
                                            // Positional argument - use index as key
                                            format!("\"{}\": {}", i, a)
                                        }
                                    })
                                    .collect();
                                code.push_str(&format!("{}__compartment.state_args = {{{}}};\n", indent_str, entries.join(", ")));
                            }
                        }

                        // Set enter_args if present
                        if let Some(ref enter) = enter_str {
                            code.push_str(&format!("{}__compartment.enter_args = Object.fromEntries([{}].map((v, i) => [String(i), v]));\n", indent_str, enter));
                        }

                        // Call __transition and return to exit the handler
                        code.push_str(&format!("{}this.__transition(__compartment);\n{}return;", indent_str, indent_str));
                        code
                    }
                    TargetLanguage::Rust => {
                        // Rust uses compartment-based transition with enter/exit args
                        let mut code = String::new();

                        // Store exit_args in current compartment if present
                        if let Some(ref exit) = exit_str {
                            let args: Vec<&str> = exit.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
                            for (i, arg) in args.iter().enumerate() {
                                code.push_str(&format!("{}self.__compartment.exit_args.insert(\"{}\".to_string(), {}.to_string());\n", indent_str, i, arg));
                            }
                        }

                        // Create new compartment with parent_compartment for HSM support
                        code.push_str(&format!("{}let mut __compartment = {}Compartment::new(\"{}\");\n", indent_str, ctx.system_name, target));
                        code.push_str(&format!("{}__compartment.parent_compartment = Some(Box::new(self.__compartment.clone()));\n", indent_str));

                        // Set enter_args if present
                        if let Some(ref enter) = enter_str {
                            let args: Vec<&str> = enter.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
                            for (i, arg) in args.iter().enumerate() {
                                code.push_str(&format!("{}__compartment.enter_args.insert(\"{}\".to_string(), {}.to_string());\n", indent_str, i, arg));
                            }
                        }

                        // Call __transition and return to exit the handler
                        code.push_str(&format!("{}self.__transition(__compartment);\n{}return;", indent_str, indent_str));
                        code
                    }
                    TargetLanguage::C => {
                        // C: Create compartment and call transition
                        let mut code = String::new();

                        // Store exit_args in current compartment if present (split by comma for positional args)
                        if let Some(ref exit) = exit_str {
                            let args: Vec<&str> = exit.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
                            for (i, arg) in args.iter().enumerate() {
                                code.push_str(&format!("{}{}_FrameDict_set(self->__compartment->exit_args, \"{}\", (void*)(intptr_t)({}));\n", indent_str, ctx.system_name, i, arg));
                            }
                        }

                        // Create new compartment
                        code.push_str(&format!("{}{}_Compartment* __compartment = {}_Compartment_new(\"{}\");\n", indent_str, ctx.system_name, ctx.system_name, target));
                        code.push_str(&format!("{}__compartment->parent_compartment = {}_Compartment_copy(self->__compartment);\n", indent_str, ctx.system_name));

                        // Set state_args if present (split by comma for positional args)
                        if let Some(ref state) = state_str {
                            let args: Vec<&str> = state.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
                            for (i, arg) in args.iter().enumerate() {
                                code.push_str(&format!("{}{}_FrameDict_set(__compartment->state_args, \"{}\", (void*)(intptr_t)({}));\n", indent_str, ctx.system_name, i, arg));
                            }
                        }

                        // Set enter_args if present (split by comma for positional args)
                        if let Some(ref enter) = enter_str {
                            let args: Vec<&str> = enter.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
                            for (i, arg) in args.iter().enumerate() {
                                code.push_str(&format!("{}{}_FrameDict_set(__compartment->enter_args, \"{}\", (void*)(intptr_t)({}));\n", indent_str, ctx.system_name, i, arg));
                            }
                        }

                        // Call transition and return to exit the handler
                        code.push_str(&format!("{}{}_transition(self, __compartment);\n{}return;", indent_str, ctx.system_name, indent_str));
                        code
                    }
                    _ => {
                        // Default: same as TypeScript
                        let mut code = String::new();
                        code.push_str(&format!("{}const __compartment = new {}Compartment(\"{}\", this.__compartment.copy());\n", indent_str, ctx.system_name, target));
                        code.push_str(&format!("{}this.__transition(__compartment);\n{}return;", indent_str, indent_str));
                        code
                    }
                }
            }
        }
        FrameSegmentKind::TransitionForward => {
            // Transition-Forward: -> => $State
            // 1. Transition to the target state (exit current)
            // 2. Forward current event to new state (instead of sending $>)
            // 3. Return (event was handled by new state)
            let target = extract_transition_target(&segment_text);
            match lang {
                TargetLanguage::Python3 => {
                    // Create compartment with forward_event set to current event
                    let mut code = String::new();
                    code.push_str(&format!("{}__compartment = {}Compartment(\"{}\", parent_compartment=self.__compartment.copy())\n", indent_str, ctx.system_name, target));
                    code.push_str(&format!("{}__compartment.forward_event = __e\n", indent_str));
                    code.push_str(&format!("{}self.__transition(__compartment)\n", indent_str));
                    code.push_str(&format!("{}return", indent_str));
                    code
                }
                TargetLanguage::TypeScript => {
                    // Create compartment with forward_event set to current event
                    let mut code = String::new();
                    code.push_str(&format!("{}const __compartment = new {}Compartment(\"{}\", this.__compartment.copy());\n", indent_str, ctx.system_name, target));
                    code.push_str(&format!("{}__compartment.forward_event = __e;\n", indent_str));
                    code.push_str(&format!("{}this.__transition(__compartment);\n", indent_str));
                    code.push_str(&format!("{}return;", indent_str));
                    code
                }
                TargetLanguage::Rust => {
                    // Rust uses compartment-based transition with forward event
                    let mut code = String::new();
                    code.push_str(&format!("{}let mut __compartment = {}Compartment::new(\"{}\");\n", indent_str, ctx.system_name, target));
                    code.push_str(&format!("{}__compartment.forward_event = Some(__e.clone());\n", indent_str));
                    code.push_str(&format!("{}self.__transition(__compartment);\n", indent_str));
                    code.push_str(&format!("{}return;", indent_str));
                    code
                }
                TargetLanguage::C => {
                    // C: Create compartment with forward event and call transition
                    let mut code = String::new();
                    code.push_str(&format!("{}{}_Compartment* __compartment = {}_Compartment_new(\"{}\");\n", indent_str, ctx.system_name, ctx.system_name, target));
                    code.push_str(&format!("{}__compartment->parent_compartment = {}_Compartment_copy(self->__compartment);\n", indent_str, ctx.system_name));
                    code.push_str(&format!("{}__compartment->forward_event = __e;\n", indent_str));
                    code.push_str(&format!("{}{}_transition(self, __compartment);\n", indent_str, ctx.system_name));
                    code.push_str(&format!("{}return;", indent_str));
                    code
                }
                _ => {
                    // Default: same as TypeScript
                    let mut code = String::new();
                    code.push_str(&format!("{}const __compartment = new {}Compartment(\"{}\", this.__compartment.copy());\n", indent_str, ctx.system_name, target));
                    code.push_str(&format!("{}__compartment.forward_event = __e;\n", indent_str));
                    code.push_str(&format!("{}this.__transition(__compartment);\n", indent_str));
                    code.push_str(&format!("{}return;", indent_str));
                    code
                }
            }
        }
        FrameSegmentKind::Forward => {
            // HSM forward: call parent state's handler for the same event
            if let Some(ref parent) = ctx.parent_state {
                match lang {
                    // Python/TypeScript: call _state_Parent(__e) to dispatch via unified state method
                    TargetLanguage::Python3 => format!("{}self._state_{}(__e)", indent_str, parent),
                    TargetLanguage::TypeScript => format!("{}this._state_{}(__e);", indent_str, parent),
                    // Rust: call parent state router (not specific handler) to dispatch via match
                    TargetLanguage::Rust => format!("{}self._state_{}(__e);", indent_str, parent),
                    // C: call System_state_Parent(self, __e) since C has no methods
                    TargetLanguage::C => format!("{}{}_state_{}(self, __e);", indent_str, ctx.system_name, parent),
                    _ => format!("{}this._state_{}(__e);", indent_str, parent),
                }
            } else {
                // No parent state - just return (shouldn't happen in valid HSM)
                match lang {
                    TargetLanguage::Python3 => format!("{}return  # Forward to parent (no parent)", indent_str),
                    _ => format!("{}return; // Forward to parent (no parent)", indent_str),
                }
            }
        }
        FrameSegmentKind::StackPush => {
            // Check if this is a push-then-transition: push$ -> $State
            let has_transition = segment_text.contains("->");
            let target = if has_transition {
                extract_transition_target(&segment_text)
            } else {
                String::new()
            };

            match lang {
                // Python/TypeScript/C: push copy, then separate transition
                TargetLanguage::Python3 => {
                    let push_code = format!("{}self._state_stack.append(self.__compartment.copy())", indent_str);
                    if !target.is_empty() {
                        format!("{}\n{}self._transition(\"{}\", None, None)", push_code, indent_str, target)
                    } else {
                        push_code
                    }
                }
                TargetLanguage::TypeScript => {
                    let push_code = format!("{}this._state_stack.push(this.__compartment.copy());", indent_str);
                    if !target.is_empty() {
                        format!("{}\n{}this._transition(\"{}\", null, null);", push_code, indent_str, target)
                    } else {
                        push_code
                    }
                }
                // Rust: __push_transition atomically moves compartment to stack and transitions
                TargetLanguage::Rust => {
                    if !target.is_empty() {
                        format!("{}self.__push_transition({}Compartment::new(\"{}\"));\n{}return;",
                            indent_str, ctx.system_name, target, indent_str)
                    } else {
                        // Push without transition (bare push$) — clone state name first to avoid borrow conflict
                        format!("{}{{\n{0}    let __state = self.__compartment.state.clone();\n{0}    self._state_stack.push(std::mem::replace(&mut self.__compartment, {}Compartment::new(&__state)));\n{0}}}",
                            indent_str, ctx.system_name)
                    }
                }
                TargetLanguage::C => {
                    let push_code = format!("{}{}_FrameVec_push(self->_state_stack, {}_Compartment_copy(self->__compartment));",
                        indent_str, ctx.system_name, ctx.system_name);
                    if !target.is_empty() {
                        format!("{}\n{}{}_transition(self, {}_Compartment_new(\"{}\"));",
                            push_code, indent_str, ctx.system_name, ctx.system_name, target)
                    } else {
                        push_code
                    }
                }
                _ => {
                    let push_code = format!("{}this._state_stack.push(this.__compartment.copy());", indent_str);
                    if !target.is_empty() {
                        format!("{}\n{}this._transition(\"{}\", null, null);", push_code, indent_str, target)
                    } else {
                        push_code
                    }
                }
            }
        }
        FrameSegmentKind::StackPop => {
            // Restore compartment from stack - use __transition to go through kernel
            // (exit current, swap to popped compartment, enter restored state)
            match lang {
                TargetLanguage::Python3 => format!(
                    "{}self.__transition(self._state_stack.pop())\n{}return",
                    indent_str, indent_str
                ),
                TargetLanguage::TypeScript => format!(
                    "{}this.__transition(this._state_stack.pop()!);\n{}return;",
                    indent_str, indent_str
                ),
                // Rust: pop compartment to local first, then transition (avoids borrow conflict)
                TargetLanguage::Rust => {
                    format!("{}let __popped = self._state_stack.pop().unwrap();\n{}self.__transition(__popped);\n{}return;",
                        indent_str, indent_str, indent_str)
                }
                TargetLanguage::C => {
                    format!("{}{}_transition(self, ({}_Compartment*){}_FrameVec_pop(self->_state_stack));\n{}return;",
                        indent_str, ctx.system_name, ctx.system_name, ctx.system_name, indent_str)
                }
                _ => format!(
                    "{}this.__compartment = this._state_stack.pop()!;\n{}return;",
                    indent_str, indent_str
                ),
            }
        }
        FrameSegmentKind::StateVar => {
            // Extract variable name from "$.varName"
            let var_name = extract_state_var_name(&segment_text);
            // State variables are stored in compartment.state_vars
            // For HSM: use __sv_comp if available (navigates to correct compartment for parent states)
            match lang {
                TargetLanguage::Python3 => {
                    if ctx.use_sv_comp {
                        format!("__sv_comp.state_vars[\"{}\"]", var_name)
                    } else {
                        format!("self.__compartment.state_vars[\"{}\"]", var_name)
                    }
                }
                TargetLanguage::TypeScript => {
                    if ctx.use_sv_comp {
                        format!("__sv_comp.state_vars[\"{}\"]", var_name)
                    } else {
                        format!("this.__compartment.state_vars[\"{}\"]", var_name)
                    }
                }
                TargetLanguage::Rust => {
                    // Access state var via compartment chain navigation + state_context matching
                    // Navigation handles HSM: walks parent_compartment chain to find correct state
                    format!("{{ let mut __sv_comp = &self.__compartment; while __sv_comp.state != \"{}\" {{ __sv_comp = __sv_comp.parent_compartment.as_ref().unwrap(); }} match &__sv_comp.state_context {{ {}StateContext::{}(ctx) => ctx.{}, _ => unreachable!() }} }}",
                        ctx.state_name, ctx.system_name, ctx.state_name, var_name)
                },
                TargetLanguage::C => {
                    // For C, access via FrameDict_get with cast
                    // Note: This is for reads; writes are handled by detecting assignment context
                    format!("(int)(intptr_t){}_FrameDict_get(self->__compartment->state_vars, \"{}\")", ctx.system_name, var_name)
                },
                _ => format!("this.__compartment.state_vars[\"{}\"]", var_name),
            }
        }
        FrameSegmentKind::StateVarAssign => {
            // State variable assignment: $.varName = expr
            // For C, this needs to become FrameDict_set(...)
            // Parse: $.varName = expr;
            let text = segment_text.trim();
            // Extract variable name: skip "$." and collect identifier
            let var_name = if text.starts_with("$.") {
                let rest = &text[2..];
                let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(rest.len());
                &rest[..end]
            } else {
                ""
            };
            // Extract expression: everything after '='
            let expr = if let Some(eq_pos) = text.find('=') {
                let after_eq = &text[eq_pos + 1..];
                // Trim trailing semicolon if present
                after_eq.trim().trim_end_matches(';').trim()
            } else {
                ""
            };
            // Expand state vars in the expression
            let expanded_expr = expand_state_vars_in_expr(expr, lang, ctx);

            match lang {
                TargetLanguage::Python3 => {
                    if ctx.use_sv_comp {
                        format!("{}__sv_comp.state_vars[\"{}\"] = {}", indent_str, var_name, expanded_expr)
                    } else {
                        format!("{}self.__compartment.state_vars[\"{}\"] = {}", indent_str, var_name, expanded_expr)
                    }
                }
                TargetLanguage::TypeScript => {
                    if ctx.use_sv_comp {
                        format!("{}__sv_comp.state_vars[\"{}\"] = {};", indent_str, var_name, expanded_expr)
                    } else {
                        format!("{}this.__compartment.state_vars[\"{}\"] = {};", indent_str, var_name, expanded_expr)
                    }
                }
                TargetLanguage::Rust => {
                    // Evaluate RHS first (immutable borrow) to avoid borrow conflict with mutable write.
                    // Navigation handles HSM: walks parent_compartment chain to find correct state.
                    format!(concat!(
                        "{}{{\n",
                        "{0}    let __rhs = {};\n",
                        "{0}    let mut __sv_comp: *mut {}Compartment = &mut self.__compartment;\n",
                        "{0}    unsafe {{ while (*__sv_comp).state != \"{}\" {{ __sv_comp = (*__sv_comp).parent_compartment.as_mut().unwrap().as_mut(); }} }}\n",
                        "{0}    unsafe {{ if let {}StateContext::{}(ref mut ctx) = (*__sv_comp).state_context {{ ctx.{} = __rhs; }} }}\n",
                        "{0}}}"
                    ),
                        indent_str, expanded_expr,
                        ctx.system_name, ctx.state_name,
                        ctx.system_name, ctx.state_name, var_name)
                },
                TargetLanguage::C => {
                    if ctx.use_sv_comp {
                        format!("{}{}_FrameDict_set(__sv_comp->state_vars, \"{}\", (void*)(intptr_t)({}));",
                            indent_str, ctx.system_name, var_name, expanded_expr)
                    } else {
                        format!("{}{}_FrameDict_set(self->__compartment->state_vars, \"{}\", (void*)(intptr_t)({}));",
                            indent_str, ctx.system_name, var_name, expanded_expr)
                    }
                }
                _ => format!("{}this.__compartment.state_vars[\"{}\"] = {};", indent_str, var_name, expanded_expr),
            }
        }
        FrameSegmentKind::ReturnSugar => {
            // return <expr> — sugar for: set @@:return = expr, then return (early exit)
            // return (bare) — just a native return from the handler
            //
            // Return value is stored on context stack: _context_stack[-1]._return
            // This enables reentrancy - nested calls have their own context
            let expr = extract_return_sugar_expr(&segment_text);
            // Expand any state variable references in the expression
            let expanded_expr = expand_state_vars_in_expr(&expr, lang, ctx);

            match lang {
                TargetLanguage::Python3 => {
                    if expanded_expr.is_empty() {
                        format!("{}return", indent_str)
                    } else {
                        format!("{}self._context_stack[-1]._return = {}\n{}return", indent_str, expanded_expr, indent_str)
                    }
                }
                TargetLanguage::TypeScript => {
                    if expanded_expr.is_empty() {
                        format!("{}return;", indent_str)
                    } else {
                        format!("{}this._context_stack[this._context_stack.length - 1]._return = {};\n{}return;", indent_str, expanded_expr, indent_str)
                    }
                }
                TargetLanguage::C => {
                    if expanded_expr.is_empty() {
                        format!("{}return;", indent_str)
                    } else {
                        format!("{}{}_CTX(self)->_return = (void*)(intptr_t)({});\n{}return;", indent_str, ctx.system_name, expanded_expr, indent_str)
                    }
                }
                TargetLanguage::Rust => {
                    if expanded_expr.is_empty() {
                        format!("{}return;", indent_str)
                    } else {
                        format!("{}let __return_val = Box::new({}) as Box<dyn std::any::Any>;\n{}if let Some(ctx) = self._context_stack.last_mut() {{ ctx._return = Some(__return_val); }}\n{}return;", indent_str, expanded_expr, indent_str, indent_str)
                    }
                }
                _ => {
                    if expanded_expr.is_empty() {
                        format!("{}return;", indent_str)
                    } else {
                        format!("{}this._context_stack[this._context_stack.length - 1]._return = {};\n{}return;", indent_str, expanded_expr, indent_str)
                    }
                }
            }
        }
        FrameSegmentKind::ContextParamShorthand => {
            // @@.param - shorthand for parameter access from context stack
            // Extract param name from "@@.paramName"
            let param_name = segment_text.strip_prefix("@@.").unwrap_or(&segment_text);
            match lang {
                TargetLanguage::Python3 => format!("self._context_stack[-1].event._parameters[\"{}\"]", param_name),
                TargetLanguage::TypeScript => format!("this._context_stack[this._context_stack.length - 1].event._parameters[\"{}\"]", param_name),
                TargetLanguage::C => format!("(int)(intptr_t){}_PARAM(self, \"{}\")", ctx.system_name, param_name),
                // Rust: handlers receive parameters directly, so just use the param name
                // This avoids Box<dyn Any> downcast complexity
                TargetLanguage::Rust => param_name.to_string(),
                _ => format!("this._context_stack[this._context_stack.length - 1].event._parameters[\"{}\"]", param_name),
            }
        }
        FrameSegmentKind::ContextReturn => {
            // @@:return - return value slot (assignment or read)
            // Check if this is assignment (@@:return = expr) or read (@@:return)
            let trimmed = segment_text.trim();
            if let Some(eq_pos) = trimmed.find('=') {
                // Check it's not ==
                if eq_pos + 1 < trimmed.len() && trimmed.as_bytes().get(eq_pos + 1) != Some(&b'=') {
                    // Assignment: @@:return = expr
                    let expr = trimmed[eq_pos + 1..].trim().trim_end_matches(';').trim();
                    let expanded_expr = expand_state_vars_in_expr(expr, lang, ctx);
                    match lang {
                        TargetLanguage::Python3 => format!("{}self._context_stack[-1]._return = {}", indent_str, expanded_expr),
                        TargetLanguage::TypeScript => format!("{}this._context_stack[this._context_stack.length - 1]._return = {};", indent_str, expanded_expr),
                        TargetLanguage::C => format!("{}{}_CTX(self)->_return = (void*)(intptr_t)({});", indent_str, ctx.system_name, expanded_expr),
                        TargetLanguage::Rust => {
                            // For Rust, evaluate expression first to avoid borrow conflicts
                            // (expression may read from context_stack while we need mutable access to set _return)
                            format!("{}let __return_val = Box::new({}) as Box<dyn std::any::Any>;\n{}if let Some(ctx) = self._context_stack.last_mut() {{ ctx._return = Some(__return_val); }}", indent_str, expanded_expr, indent_str)
                        }
                        _ => format!("{}this._context_stack[this._context_stack.length - 1]._return = {};", indent_str, expanded_expr),
                    }
                } else {
                    // Read: @@:return (== check)
                    match lang {
                        TargetLanguage::Python3 => "self._context_stack[-1]._return".to_string(),
                        TargetLanguage::TypeScript => "this._context_stack[this._context_stack.length - 1]._return".to_string(),
                        TargetLanguage::C => format!("{}_RETURN(self)", ctx.system_name),
                        TargetLanguage::Rust => "self._context_stack.last().and_then(|ctx| ctx._return.as_ref()).cloned()".to_string(),
                        _ => "this._context_stack[this._context_stack.length - 1]._return".to_string(),
                    }
                }
            } else {
                // Read: @@:return
                match lang {
                    TargetLanguage::Python3 => "self._context_stack[-1]._return".to_string(),
                    TargetLanguage::TypeScript => "this._context_stack[this._context_stack.length - 1]._return".to_string(),
                    TargetLanguage::Rust => "self._return_value".to_string(),
                    _ => "this._context_stack[this._context_stack.length - 1]._return".to_string(),
                }
            }
        }
        FrameSegmentKind::ContextEvent => {
            // @@:event - interface event name
            match lang {
                TargetLanguage::Python3 => "self._context_stack[-1].event._message".to_string(),
                TargetLanguage::TypeScript => "this._context_stack[this._context_stack.length - 1].event._message".to_string(),
                TargetLanguage::C => format!("{}_CTX(self)->event->_message", ctx.system_name),
                // Rust: handlers receive __e as parameter, use it directly to avoid borrow conflicts
                TargetLanguage::Rust => "__e.message.clone()".to_string(),
                _ => "this._context_stack[this._context_stack.length - 1].event._message".to_string(),
            }
        }
        FrameSegmentKind::ContextData => {
            // @@:data[key] - call-scoped data (read)
            // Extract key from "@@:data[key]" — key includes user quotes (e.g. 'key' or "key")
            let key = extract_bracket_key(&segment_text, "@@:data");
            let bare_key = key.trim_matches('"').trim_matches('\'');
            match lang {
                TargetLanguage::Python3 => format!("self._context_stack[-1]._data[{}]", key),
                TargetLanguage::TypeScript => format!("this._context_stack[this._context_stack.length - 1]._data[{}]", key),
                TargetLanguage::C => format!("{}_DATA(self, \"{}\")", ctx.system_name, bare_key),
                TargetLanguage::Rust => {
                    // For Rust read, we need to handle the dynamic type
                    // The _data HashMap stores Box<dyn Any>, so we need downcast
                    format!("self._context_stack.last().and_then(|ctx| ctx._data.get(\"{}\")).and_then(|v| v.downcast_ref::<String>()).cloned().unwrap_or_default()", bare_key)
                }
                _ => format!("this._context_stack[this._context_stack.length - 1]._data[{}]", key),
            }
        }
        FrameSegmentKind::ContextDataAssign => {
            // @@:data[key] = expr - call-scoped data (assignment)
            // Extract key and value from "@@:data[key] = expr;"
            let key = extract_bracket_key(&segment_text, "@@:data");
            let bare_key = key.trim_matches('"').trim_matches('\'');
            // Find the = and extract the expression
            let trimmed = segment_text.trim();
            let eq_pos = trimmed.find('=').unwrap_or(trimmed.len());
            let expr = trimmed[eq_pos + 1..].trim().trim_end_matches(';').trim();
            let expanded_expr = expand_state_vars_in_expr(expr, lang, ctx);
            match lang {
                TargetLanguage::Python3 => format!("{}self._context_stack[-1]._data[{}] = {}", indent_str, key, expanded_expr),
                TargetLanguage::TypeScript => format!("{}this._context_stack[this._context_stack.length - 1]._data[{}] = {};", indent_str, key, expanded_expr),
                TargetLanguage::C => format!("{}{}_DATA_SET(self, \"{}\", {});", indent_str, ctx.system_name, bare_key, expanded_expr),
                TargetLanguage::Rust => {
                    // For Rust, insert into the HashMap with Box<dyn Any>
                    format!("{}if let Some(ctx) = self._context_stack.last_mut() {{ ctx._data.insert(\"{}\".to_string(), Box::new({}) as Box<dyn std::any::Any>); }}", indent_str, bare_key, expanded_expr)
                }
                _ => format!("{}this._context_stack[this._context_stack.length - 1]._data[{}] = {};", indent_str, key, expanded_expr),
            }
        }
        FrameSegmentKind::ContextParams => {
            // @@:params[key] - explicit parameter access
            // Extract key from "@@:params[key]" — key includes user quotes
            let key = extract_bracket_key(&segment_text, "@@:params");
            let bare_key = key.trim_matches('"').trim_matches('\'');
            match lang {
                TargetLanguage::Python3 => format!("self._context_stack[-1].event._parameters[{}]", key),
                TargetLanguage::TypeScript => format!("this._context_stack[this._context_stack.length - 1].event._parameters[{}]", key),
                TargetLanguage::C => format!("{}_PARAM(self, \"{}\")", ctx.system_name, bare_key),
                TargetLanguage::Rust => format!("self._context_stack.last().and_then(|ctx| ctx.event.parameters.get(\"{}\")).cloned()", bare_key),
                _ => format!("this._context_stack[this._context_stack.length - 1].event._parameters[{}]", key),
            }
        }
        FrameSegmentKind::TaggedInstantiation => {
            // @@SystemName(args) - validated system instantiation
            // Strip @@ prefix and validate system name exists
            let native_call = segment_text.strip_prefix("@@").unwrap_or(&segment_text);

            // Extract system name (before the parenthesis)
            let tagged_system_name = if let Some(paren_pos) = native_call.find('(') {
                &native_call[..paren_pos]
            } else {
                native_call
            };

            // Validate that the system name exists in defined_systems
            if !ctx.defined_systems.contains(tagged_system_name) {
                // System not found - generate an error that will fail compilation
                match lang {
                    TargetLanguage::Python3 => {
                        format!("raise NameError(\"Frame Error E421: Undefined system '{}' in tagged instantiation @@{}. Did you mean one of: {:?}?\")",
                            tagged_system_name, tagged_system_name, ctx.defined_systems)
                    }
                    TargetLanguage::TypeScript => {
                        format!("throw new Error(\"Frame Error E421: Undefined system '{}' in tagged instantiation @@{}. Did you mean one of: {:?}?\");",
                            tagged_system_name, tagged_system_name, ctx.defined_systems)
                    }
                    TargetLanguage::Rust => {
                        format!("compile_error!(\"Frame Error E421: Undefined system '{}' in tagged instantiation @@{}\");",
                            tagged_system_name, tagged_system_name)
                    }
                    TargetLanguage::C => {
                        format!("#error \"Frame Error E421: Undefined system '{}' in tagged instantiation @@{}\"",
                            tagged_system_name, tagged_system_name)
                    }
                    _ => {
                        format!("/* Frame Error E421: Undefined system '{}' in tagged instantiation @@{} */",
                            tagged_system_name, tagged_system_name)
                    }
                }
            } else {
                // System found - generate valid constructor call
                match lang {
                    TargetLanguage::C => {
                        // C: @@System() becomes System_new()
                        if let Some(paren_pos) = native_call.find('(') {
                            let args = &native_call[paren_pos..];
                            format!("{}_new{}", tagged_system_name, args)
                        } else {
                            native_call.to_string()
                        }
                    }
                    _ => {
                        // Python/TypeScript/Rust: @@System() becomes System()
                        native_call.to_string()
                    }
                }
            }
        }
    }
}

/// Extract bracketed key from syntax like "@@:data[key]" or "@@:params[key]"
/// Returns the raw content between [ and ] — including any user-supplied quotes.
/// For languages that need a bare key (C, Rust), call .trim_matches on the result.
fn extract_bracket_key(text: &str, prefix: &str) -> String {
    if let Some(rest) = text.strip_prefix(prefix) {
        if let Some(start) = rest.find('[') {
            if let Some(end) = rest.find(']') {
                return rest[start + 1..end].trim().to_string();
            }
        }
    }
    "".to_string()
}

/// Extract transition target from transition text
fn extract_transition_target(text: &str) -> String {
    // Find $StateName after -> in the transition text
    // This handles both "-> $State" and "$$[+] -> $State"
    if let Some(arrow_pos) = text.find("->") {
        let after_arrow = &text[arrow_pos + 2..];
        if let Some(dollar_pos) = after_arrow.find('$') {
            let after_dollar = &after_arrow[dollar_pos + 1..];
            let end = after_dollar.find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(after_dollar.len());
            return after_dollar[..end].to_string();
        }
    }
    // Fallback: find last $ (for simple "-> $State" without prefix)
    if let Some(dollar_pos) = text.rfind('$') {
        let after_dollar = &text[dollar_pos + 1..];
        let end = after_dollar.find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(after_dollar.len());
        after_dollar[..end].to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Extract transition arguments (exit_args, enter_args) from transition text
/// Syntax: (exit_args)? -> (enter_args)? $State(state_args)?
fn extract_transition_args(text: &str) -> (Option<String>, Option<String>) {
    let text = text.trim();
    let bytes = text.as_bytes();
    let n = bytes.len();
    let mut i = 0;

    // Skip leading whitespace
    while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }

    // Check for (exit_args) before ->
    let mut exit_args: Option<String> = None;
    if i < n && bytes[i] == b'(' {
        if let Some(close_idx) = find_balanced_paren(bytes, i, n) {
            exit_args = Some(String::from_utf8_lossy(&bytes[i+1..close_idx-1]).to_string());
            i = close_idx;
            while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
        }
    }

    // Skip ->
    if i + 1 < n && bytes[i] == b'-' && bytes[i + 1] == b'>' {
        i += 2;
        while i < n && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
    }

    // Check for (enter_args) after ->
    let mut enter_args: Option<String> = None;
    if i < n && bytes[i] == b'(' {
        if let Some(close_idx) = find_balanced_paren(bytes, i, n) {
            enter_args = Some(String::from_utf8_lossy(&bytes[i+1..close_idx-1]).to_string());
        }
    }

    (exit_args, enter_args)
}

/// Extract state_args from transition text: -> $State(state_args)?
/// Returns the comma-separated args string inside the parens after $State
fn extract_state_args(text: &str) -> Option<String> {
    // Find $StateName( pattern
    let bytes = text.as_bytes();
    let n = bytes.len();

    // Find $ after ->
    let arrow_pos = text.find("->")?;
    let after_arrow = &text[arrow_pos + 2..];

    // Find $ for state name
    let dollar_pos = after_arrow.find('$')?;
    let state_start = dollar_pos + 1;

    // Skip the state name (alphanumeric + underscore)
    let mut i = state_start;
    while i < after_arrow.len() {
        let c = after_arrow.as_bytes()[i];
        if c.is_ascii_alphanumeric() || c == b'_' {
            i += 1;
        } else {
            break;
        }
    }

    // Check if there's a ( immediately after state name
    if i < after_arrow.len() && after_arrow.as_bytes()[i] == b'(' {
        let paren_start = arrow_pos + 2 + i;
        if let Some(close_idx) = find_balanced_paren(bytes, paren_start, n) {
            // Return content between ( and )
            return Some(String::from_utf8_lossy(&bytes[paren_start+1..close_idx-1]).to_string());
        }
    }

    None
}

/// Find the closing paren for a balanced paren block, returns index after ')'
fn find_balanced_paren(bytes: &[u8], mut i: usize, end: usize) -> Option<usize> {
    if i >= end || bytes[i] != b'(' { return None; }
    let mut depth = 0i32;
    let mut in_str: Option<u8> = None;
    while i < end {
        let b = bytes[i];
        if let Some(q) = in_str {
            if b == b'\\' { i += 2; continue; }
            if b == q { in_str = None; }
            i += 1; continue;
        }
        match b {
            b'\'' | b'"' => { in_str = Some(b); i += 1; }
            b'(' => { depth += 1; i += 1; }
            b')' => { depth -= 1; i += 1; if depth == 0 { return Some(i); } }
            _ => { i += 1; }
        }
    }
    None
}

/// Extract state variable name from "$.varName"
fn extract_state_var_name(text: &str) -> String {
    // Skip "$." prefix and get identifier
    if text.starts_with("$.") {
        let after_prefix = &text[2..];
        let end = after_prefix.find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(after_prefix.len());
        after_prefix[..end].to_string()
    } else {
        "unknown".to_string()
    }
}

/// Extract expression from return <expr> sugar
fn extract_return_sugar_expr(text: &str) -> String {
    let text = text.trim();
    if text.starts_with("return ") {
        let after_return = text[7..].trim();
        return after_return.to_string();
    }
    String::new()
}

/// Expand state variable references ($.varName) and context syntax (@@) in an expression string
/// Uses compartment.state_vars for Python/TypeScript
/// For HSM: uses __sv_comp when ctx.use_sv_comp is true (navigates to correct parent compartment)
fn expand_state_vars_in_expr(expr: &str, lang: TargetLanguage, ctx: &HandlerContext) -> String {
    let mut result = String::new();
    let bytes = expr.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'$' && bytes[i + 1] == b'.' {
            // Found $.varName
            i += 2; // Skip "$."
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let var_name = String::from_utf8_lossy(&bytes[start..i]).to_string();
            match lang {
                TargetLanguage::Python3 => {
                    if ctx.use_sv_comp {
                        result.push_str(&format!("__sv_comp.state_vars[\"{}\"]", var_name))
                    } else {
                        result.push_str(&format!("self.__compartment.state_vars[\"{}\"]", var_name))
                    }
                }
                TargetLanguage::TypeScript => {
                    if ctx.use_sv_comp {
                        result.push_str(&format!("__sv_comp.state_vars[\"{}\"]", var_name))
                    } else {
                        result.push_str(&format!("this.__compartment.state_vars[\"{}\"]", var_name))
                    }
                }
                TargetLanguage::Rust => result.push_str(&format!(
                    "{{ let mut __sv_comp = &self.__compartment; while __sv_comp.state != \"{}\" {{ __sv_comp = __sv_comp.parent_compartment.as_ref().unwrap(); }} match &__sv_comp.state_context {{ {}StateContext::{}(ctx) => ctx.{}, _ => unreachable!() }} }}",
                    ctx.state_name, ctx.system_name, ctx.state_name, var_name)),
                TargetLanguage::C => {
                    if ctx.use_sv_comp {
                        result.push_str(&format!("(int)(intptr_t){}_FrameDict_get(__sv_comp->state_vars, \"{}\")", ctx.system_name, var_name))
                    } else {
                        result.push_str(&format!("(int)(intptr_t){}_FrameDict_get(self->__compartment->state_vars, \"{}\")", ctx.system_name, var_name))
                    }
                }
                _ => result.push_str(&format!("this.__compartment.state_vars[\"{}\"]", var_name)),
            }
        } else if i + 1 < bytes.len() && bytes[i] == b'@' && bytes[i + 1] == b'@' {
            // Found @@ - context syntax
            i += 2; // Skip "@@"
            if i < bytes.len() && bytes[i] == b'.' {
                // @@.param - shorthand parameter access from context stack
                i += 1; // Skip "."
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                let param_name = String::from_utf8_lossy(&bytes[start..i]).to_string();
                match lang {
                    TargetLanguage::Python3 => result.push_str(&format!("self._context_stack[-1].event._parameters[\"{}\"]", param_name)),
                    TargetLanguage::TypeScript => result.push_str(&format!("this._context_stack[this._context_stack.length - 1].event._parameters[\"{}\"]", param_name)),
                    TargetLanguage::C => result.push_str(&format!("(int)(intptr_t){}_PARAM(self, \"{}\")", ctx.system_name, param_name)),
                    // Rust: handlers receive parameters directly, so just use the param name
                    TargetLanguage::Rust => result.push_str(&param_name),
                    _ => result.push_str(&format!("this._context_stack[this._context_stack.length - 1].event._parameters[\"{}\"]", param_name)),
                }
            } else if i < bytes.len() && bytes[i] == b':' {
                i += 1; // Skip ":"
                // Check which context field
                if i + 5 < bytes.len() && &bytes[i..i + 6] == b"return" {
                    // @@:return
                    i += 6;
                    match lang {
                        TargetLanguage::Python3 => result.push_str("self._context_stack[-1]._return"),
                        TargetLanguage::TypeScript => result.push_str("this._context_stack[this._context_stack.length - 1]._return"),
                        TargetLanguage::C => result.push_str(&format!("{}_RETURN(self)", ctx.system_name)),
                        TargetLanguage::Rust => result.push_str("self._context_stack.last().and_then(|ctx| ctx._return.as_ref()).cloned()"),
                        _ => result.push_str("this._context_stack[this._context_stack.length - 1]._return"),
                    }
                } else if i + 4 < bytes.len() && &bytes[i..i + 5] == b"event" {
                    // @@:event
                    i += 5;
                    match lang {
                        TargetLanguage::Python3 => result.push_str("self._context_stack[-1].event._message"),
                        TargetLanguage::TypeScript => result.push_str("this._context_stack[this._context_stack.length - 1].event._message"),
                        TargetLanguage::C => result.push_str(&format!("{}_CTX(self)->event->_message", ctx.system_name)),
                        // Rust: handlers receive __e as parameter, use it directly to avoid borrow conflicts
                        TargetLanguage::Rust => result.push_str("__e.message.clone()"),
                        _ => result.push_str("this._context_stack[this._context_stack.length - 1].event._message"),
                    }
                } else if i + 3 < bytes.len() && &bytes[i..i + 4] == b"data" {
                    // @@:data[key]
                    i += 4;
                    if i < bytes.len() && bytes[i] == b'[' {
                        i += 1; // Skip '['
                        let start = i;
                        while i < bytes.len() && bytes[i] != b']' {
                            i += 1;
                        }
                        let key = String::from_utf8_lossy(&bytes[start..i]).trim().trim_matches('"').trim_matches('\'').to_string();
                        if i < bytes.len() {
                            i += 1; // Skip ']'
                        }
                        match lang {
                            TargetLanguage::Python3 => result.push_str(&format!("self._context_stack[-1]._data[\"{}\"]", key)),
                            TargetLanguage::TypeScript => result.push_str(&format!("this._context_stack[this._context_stack.length - 1]._data[\"{}\"]", key)),
                            TargetLanguage::C => result.push_str(&format!("{}_DATA(self, \"{}\")", ctx.system_name, key)),
                            TargetLanguage::Rust => result.push_str(&format!("self._context_stack.last().and_then(|ctx| ctx._data.get(\"{}\")).and_then(|v| v.downcast_ref::<String>()).cloned().unwrap_or_default()", key)),
                            _ => result.push_str(&format!("this._context_stack[this._context_stack.length - 1]._data[\"{}\"]", key)),
                        }
                    }
                } else if i + 5 < bytes.len() && &bytes[i..i + 6] == b"params" {
                    // @@:params[key]
                    i += 6;
                    if i < bytes.len() && bytes[i] == b'[' {
                        i += 1; // Skip '['
                        let start = i;
                        while i < bytes.len() && bytes[i] != b']' {
                            i += 1;
                        }
                        let key = String::from_utf8_lossy(&bytes[start..i]).trim().trim_matches('"').trim_matches('\'').to_string();
                        if i < bytes.len() {
                            i += 1; // Skip ']'
                        }
                        match lang {
                            TargetLanguage::Python3 => result.push_str(&format!("self._context_stack[-1].event._parameters[\"{}\"]", key)),
                            TargetLanguage::TypeScript => result.push_str(&format!("this._context_stack[this._context_stack.length - 1].event._parameters[\"{}\"]", key)),
                            TargetLanguage::C => result.push_str(&format!("{}_PARAM(self, \"{}\")", ctx.system_name, key)),
                            // Rust: for params access, just use the handler's direct parameter
                            TargetLanguage::Rust => result.push_str(&key),
                            _ => result.push_str(&format!("this._context_stack[this._context_stack.length - 1].event._parameters[\"{}\"]", key)),
                        }
                    }
                } else {
                    // Unknown, pass through
                    result.push_str("@@:");
                }
            } else {
                // Just @@ without . or :, pass through
                result.push_str("@@");
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

/// Get the native region scanner for the target language
fn get_native_scanner(lang: TargetLanguage) -> Box<dyn NativeRegionScanner> {
    match lang {
        TargetLanguage::Python3 => Box::new(NativeRegionScannerPy),
        TargetLanguage::TypeScript => Box::new(NativeRegionScannerTs),
        TargetLanguage::Rust => Box::new(NativeRegionScannerRust),
        TargetLanguage::CSharp => Box::new(NativeRegionScannerCs),
        TargetLanguage::C => Box::new(NativeRegionScannerC),
        TargetLanguage::Cpp => Box::new(NativeRegionScannerCpp),
        TargetLanguage::Java => Box::new(NativeRegionScannerJava),
        _ => Box::new(NativeRegionScannerPy), // Default to Python
    }
}

/// Generate action method
///
/// Extracts native code from source using the body span
fn generate_action(action: &ActionAst, _syntax: &super::backend::ClassSyntax, source: &[u8]) -> CodegenNode {
    let params: Vec<Param> = action.params.iter().map(|p| {
        let type_str = type_to_string(&p.param_type);
        Param::new(&p.name).with_type(&type_str)
    }).collect();

    // Extract native code from source using span (oceans model)
    let code = extract_body_content(source, &action.body.span);

    CodegenNode::Method {
        name: action.name.clone(),
        params,
        return_type: None,  // Actions don't have explicit return types
        body: vec![CodegenNode::NativeBlock {
            code,
            span: Some(action.body.span.clone()),
        }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate operation method
///
/// Extracts native code from source using the body span
fn generate_operation(operation: &OperationAst, _syntax: &super::backend::ClassSyntax, source: &[u8]) -> CodegenNode {
    let params: Vec<Param> = operation.params.iter().map(|p| {
        let type_str = type_to_string(&p.param_type);
        Param::new(&p.name).with_type(&type_str)
    }).collect();

    // Extract native code from source using span (oceans model)
    let code = extract_body_content(source, &operation.body.span);

    // Backend handles is_static flag for @staticmethod decorator
    CodegenNode::Method {
        name: operation.name.clone(),
        params,
        return_type: Some(type_to_string(&operation.return_type)),
        body: vec![CodegenNode::NativeBlock {
            code,
            span: Some(operation.body.span.clone()),
        }],
        is_async: false,
        is_static: operation.is_static,
        visibility: Visibility::Public,
        decorators: vec![],
    }
}

/// Extract body content from source using span
///
/// Strips the outer braces and extracts the inner content while preserving
/// consistent line-by-line indentation for proper re-indentation by backends.
fn extract_body_content(source: &[u8], span: &crate::frame_c::v4::frame_ast::Span) -> String {
    let bytes = &source[span.start..span.end];
    let content = String::from_utf8_lossy(bytes).to_string();

    // Strip outer braces if present
    let trimmed = content.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        // Extract content between braces
        let inner = &trimmed[1..trimmed.len()-1];

        // Split into lines, preserving structure
        let lines: Vec<&str> = inner.lines().collect();

        // Skip leading and trailing empty lines, but preserve internal structure
        let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
        let end = lines.iter().rposition(|l| !l.trim().is_empty()).map(|i| i + 1).unwrap_or(lines.len());

        if start >= end {
            return String::new();
        }

        // Return lines with preserved indentation - let NativeBlock emitter normalize
        lines[start..end].join("\n")
    } else {
        trimmed.to_string()
    }
}

/// Generate persistence methods (save_state, restore_state) for @@persist
fn generate_persistence_methods(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> Vec<CodegenNode> {
    let mut methods = Vec::new();

    match syntax.language {
        TargetLanguage::Python3 => {
            // Python uses pickle by default (stdlib, complete serialization)
            // Generate save_state method - returns bytes
            methods.push(CodegenNode::Method {
                name: "save_state".to_string(),
                params: vec![],
                return_type: Some("bytes".to_string()),
                body: vec![CodegenNode::NativeBlock {
                    code: "import pickle\nreturn pickle.dumps(self)".to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Public,
                decorators: vec![],
            });

            // Generate restore_state - takes bytes, returns instance
            methods.push(CodegenNode::Method {
                name: "restore_state".to_string(),
                params: vec![Param::new("data").with_type("bytes")],
                return_type: Some(format!("'{}'", system.name)),
                body: vec![CodegenNode::NativeBlock {
                    code: "import pickle\nreturn pickle.loads(data)".to_string(),
                    span: None,
                }],
                is_async: false,
                is_static: true,
                visibility: Visibility::Public,
                decorators: vec![],
            });
        }
        TargetLanguage::TypeScript => {
            // Generate saveState method
            // Phase 14.6: Serialize compartment structure including HSM parent_compartment chain
            let mut save_body = String::new();
            // Helper to serialize compartment chain recursively
            save_body.push_str("const serializeComp = (c: any): any => {\n");
            save_body.push_str("    if (!c) return null;\n");
            save_body.push_str("    return {\n");
            save_body.push_str("        state: c.state,\n");
            save_body.push_str("        state_args: {...c.state_args},\n");
            save_body.push_str("        state_vars: {...c.state_vars},\n");
            save_body.push_str("        enter_args: {...c.enter_args},\n");
            save_body.push_str("        exit_args: {...c.exit_args},\n");
            save_body.push_str("        forward_event: c.forward_event,\n");
            save_body.push_str("        parent_compartment: serializeComp(c.parent_compartment),\n");
            save_body.push_str("    };\n");
            save_body.push_str("};\n");
            save_body.push_str("return JSON.stringify({\n");
            save_body.push_str("    _compartment: serializeComp(this.__compartment),\n");
            // Stack stores compartment objects - serialize each with its parent chain
            save_body.push_str("    _state_stack: this._state_stack.map((c: any) => serializeComp(c)),\n");

            // Add domain variables
            for var in &system.domain {
                save_body.push_str(&format!("    {}: this.{},\n", var.name, var.name));
            }

            save_body.push_str("});\n");

            methods.push(CodegenNode::Method {
                name: "saveState".to_string(),
                params: vec![],
                return_type: Some("string".to_string()),  // Returns JSON string
                body: vec![CodegenNode::NativeBlock {
                    code: save_body,
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Public,
                decorators: vec![],
            });

            // Generate restoreState static method
            // Phase 14.6: Restore compartment structure including HSM parent_compartment chain
            let mut restore_body = String::new();
            // Helper to deserialize compartment chain recursively
            restore_body.push_str(&format!("const deserializeComp = (data: any): {}Compartment | null => {{\n", system.name));
            restore_body.push_str("    if (!data) return null;\n");
            restore_body.push_str(&format!("    const comp = new {}Compartment(data.state);\n", system.name));
            restore_body.push_str("    comp.state_args = {...(data.state_args || {})};\n");
            restore_body.push_str("    comp.state_vars = {...(data.state_vars || {})};\n");
            restore_body.push_str("    comp.enter_args = {...(data.enter_args || {})};\n");
            restore_body.push_str("    comp.exit_args = {...(data.exit_args || {})};\n");
            restore_body.push_str("    comp.forward_event = data.forward_event;\n");
            restore_body.push_str("    comp.parent_compartment = deserializeComp(data.parent_compartment);\n");
            restore_body.push_str("    return comp;\n");
            restore_body.push_str("};\n");
            restore_body.push_str("const data = JSON.parse(json);\n");
            restore_body.push_str(&format!("const instance = Object.create({}.prototype);\n", system.name));
            // Restore compartment with full parent chain
            restore_body.push_str("instance.__compartment = deserializeComp(data._compartment);\n");
            restore_body.push_str("instance.__next_compartment = null;\n");
            // Restore stack - each element is a serialized compartment with its parent chain
            restore_body.push_str("instance._state_stack = (data._state_stack || []).map((c: any) => deserializeComp(c));\n");
            restore_body.push_str("instance._context_stack = [];\n");

            // Restore domain variables
            for var in &system.domain {
                restore_body.push_str(&format!("instance.{} = data.{};\n", var.name, var.name));
            }

            restore_body.push_str("return instance;");

            methods.push(CodegenNode::Method {
                name: "restoreState".to_string(),
                params: vec![Param::new("json").with_type("string")],
                return_type: Some(system.name.clone()),
                body: vec![CodegenNode::NativeBlock {
                    code: restore_body,
                    span: None,
                }],
                is_async: false,
                is_static: true,
                visibility: Visibility::Public,
                decorators: vec![],
            });
        }
        TargetLanguage::Rust => {
            // Rust uses serde_json (requires serde_json in Cargo.toml)
            // HSM persistence: serialize entire compartment chain including parent_compartment
            // State vars live on compartment.state_context (StateContext enum)

            {
                // Generate save_state that recursively serializes compartment chain
                let mut save_body = String::new();

                // Helper: serialize state_context enum to JSON
                save_body.push_str(&format!("fn serialize_state_context(ctx: &{}StateContext) -> serde_json::Value {{\n", system.name));
                save_body.push_str("    match ctx {\n");
                if let Some(ref machine) = system.machine {
                    for state in &machine.states {
                        if state.state_vars.is_empty() {
                            save_body.push_str(&format!(
                                "        {}StateContext::{} => serde_json::json!({{}}),\n",
                                system.name, state.name
                            ));
                        } else {
                            save_body.push_str(&format!(
                                "        {}StateContext::{}(ctx) => serde_json::json!({{\n",
                                system.name, state.name
                            ));
                            for var in &state.state_vars {
                                save_body.push_str(&format!(
                                    "            \"{}\": ctx.{},\n",
                                    var.name, var.name
                                ));
                            }
                            save_body.push_str("        }),\n");
                        }
                    }
                }
                save_body.push_str(&format!("        {}StateContext::Empty => serde_json::json!({{}}),\n", system.name));
                save_body.push_str("    }\n");
                save_body.push_str("}\n");

                // Helper function to serialize a compartment and its parent chain
                save_body.push_str(&format!("fn serialize_comp(comp: &{}Compartment) -> serde_json::Value {{\n", system.name));
                save_body.push_str("    let parent = match &comp.parent_compartment {\n");
                save_body.push_str("        Some(p) => serialize_comp(p),\n");
                save_body.push_str("        None => serde_json::Value::Null,\n");
                save_body.push_str("    };\n");
                save_body.push_str("    serde_json::json!({\n");
                save_body.push_str("        \"state\": comp.state,\n");
                save_body.push_str("        \"state_context\": serialize_state_context(&comp.state_context),\n");
                save_body.push_str("        \"parent_compartment\": parent,\n");
                save_body.push_str("    })\n");
                save_body.push_str("}\n");

                save_body.push_str("let compartment_data = serialize_comp(&self.__compartment);\n");
                save_body.push_str("let stack_data: Vec<serde_json::Value> = self._state_stack.iter()\n");
                save_body.push_str("    .map(|comp| serialize_comp(comp))\n");
                save_body.push_str("    .collect();\n");
                save_body.push_str("serde_json::json!({\n");
                save_body.push_str("    \"_compartment\": compartment_data,\n");
                save_body.push_str("    \"_state_stack\": stack_data,\n");

                // Add domain variables
                for var in &system.domain {
                    save_body.push_str(&format!("    \"{}\": self.{},\n", var.name, var.name));
                }

                save_body.push_str("}).to_string()");

                methods.push(CodegenNode::Method {
                    name: "save_state".to_string(),
                    params: vec![],
                    return_type: Some("String".to_string()),
                    body: vec![CodegenNode::NativeBlock {
                        code: save_body,
                        span: None,
                    }],
                    is_async: false,
                    is_static: false,
                    visibility: Visibility::Public,
                    decorators: vec![],
                });

                // Generate restore_state that recursively deserializes compartment chain
                let mut restore_body = String::new();
                restore_body.push_str("let data: serde_json::Value = serde_json::from_str(json).unwrap();\n");

                // Helper: deserialize state_context from JSON based on state name
                restore_body.push_str(&format!("fn deserialize_state_context(state: &str, data: &serde_json::Value) -> {}StateContext {{\n", system.name));
                restore_body.push_str("    match state {\n");
                if let Some(ref machine) = system.machine {
                    for state in &machine.states {
                        if state.state_vars.is_empty() {
                            restore_body.push_str(&format!(
                                "        \"{}\" => {}StateContext::{},\n",
                                state.name, system.name, state.name
                            ));
                        } else {
                            restore_body.push_str(&format!(
                                "        \"{}\" => {}StateContext::{}({}Context {{\n",
                                state.name, system.name, state.name, state.name
                            ));
                            for var in &state.state_vars {
                                let json_extract = match &var.var_type {
                                    Type::Custom(name) => {
                                        match name.to_lowercase().as_str() {
                                            "int" | "i32" => format!("data[\"{}\"].as_i64().unwrap_or(0) as i32", var.name),
                                            "i64" => format!("data[\"{}\"].as_i64().unwrap_or(0)", var.name),
                                            "float" | "f32" | "f64" => format!("data[\"{}\"].as_f64().unwrap_or(0.0)", var.name),
                                            "bool" => format!("data[\"{}\"].as_bool().unwrap_or(false)", var.name),
                                            "str" | "string" => format!("data[\"{}\"].as_str().unwrap_or(\"\").to_string()", var.name),
                                            _ => format!("serde_json::from_value(data[\"{}\"].clone()).unwrap_or_default()", var.name),
                                        }
                                    }
                                    _ => format!("serde_json::from_value(data[\"{}\"].clone()).unwrap_or_default()", var.name),
                                };
                                restore_body.push_str(&format!("            {}: {},\n", var.name, json_extract));
                            }
                            restore_body.push_str("        }),\n");
                        }
                    }
                }
                restore_body.push_str(&format!("        _ => {}StateContext::Empty,\n", system.name));
                restore_body.push_str("    }\n");
                restore_body.push_str("}\n");

                // Helper function to deserialize a compartment and its parent chain
                restore_body.push_str(&format!("fn deserialize_comp(data: &serde_json::Value) -> {}Compartment {{\n", system.name));
                restore_body.push_str(&format!("    let state = data[\"state\"].as_str().unwrap();\n"));
                restore_body.push_str(&format!("    let mut comp = {}Compartment::new(state);\n", system.name));
                restore_body.push_str("    let ctx_data = &data[\"state_context\"];\n");
                restore_body.push_str("    if !ctx_data.is_null() {\n");
                restore_body.push_str(&format!("        comp.state_context = deserialize_state_context(state, ctx_data);\n"));
                restore_body.push_str("    }\n");
                restore_body.push_str("    if !data[\"parent_compartment\"].is_null() {\n");
                restore_body.push_str("        comp.parent_compartment = Some(Box::new(deserialize_comp(&data[\"parent_compartment\"])));\n");
                restore_body.push_str("    }\n");
                restore_body.push_str("    comp\n");
                restore_body.push_str("}\n");

                // Restore stack as Vec<Compartment>
                restore_body.push_str(&format!("let stack: Vec<{}Compartment> = data[\"_state_stack\"].as_array()\n", system.name));
                restore_body.push_str("    .map(|arr| arr.iter()\n");
                restore_body.push_str("        .map(|v| deserialize_comp(v))\n");
                restore_body.push_str("        .collect())\n");
                restore_body.push_str("    .unwrap_or_default();\n");

                // Deserialize compartment
                restore_body.push_str("let compartment = deserialize_comp(&data[\"_compartment\"]);\n");

                restore_body.push_str(&format!("let instance = {} {{\n", system.name));
                restore_body.push_str("    _state_stack: stack,\n");
                restore_body.push_str("    _context_stack: vec![],\n");
                restore_body.push_str("    __compartment: compartment,\n");
                restore_body.push_str("    __next_compartment: None,\n");

                // Restore domain variables
                for var in &system.domain {
                    let _type_str = type_to_string(&var.var_type);
                    let json_extract = match &var.var_type {
                        Type::Custom(name) => {
                            match name.to_lowercase().as_str() {
                                "int" | "i32" => format!("data[\"{}\"].as_i64().unwrap() as i32", var.name),
                                "i64" => format!("data[\"{}\"].as_i64().unwrap()", var.name),
                                "float" | "f32" | "f64" => format!("data[\"{}\"].as_f64().unwrap()", var.name),
                                "bool" => format!("data[\"{}\"].as_bool().unwrap()", var.name),
                                "str" | "string" => format!("data[\"{}\"].as_str().unwrap().to_string()", var.name),
                                _ => format!("serde_json::from_value(data[\"{}\"].clone()).unwrap()", var.name),
                            }
                        }
                        _ => format!("serde_json::from_value(data[\"{}\"].clone()).unwrap()", var.name),
                    };
                    restore_body.push_str(&format!("    {}: {},\n", var.name, json_extract));
                }

                restore_body.push_str("};\n");
                restore_body.push_str("instance");

                methods.push(CodegenNode::Method {
                    name: "restore_state".to_string(),
                    params: vec![Param::new("json").with_type("&str")],
                    return_type: Some(system.name.clone()),
                    body: vec![CodegenNode::NativeBlock {
                        code: restore_body,
                        span: None,
                    }],
                    is_async: false,
                    is_static: true,
                    visibility: Visibility::Public,
                    decorators: vec![],
                });
            }
        }
        TargetLanguage::C => {
            // C uses cJSON library (requires cJSON.h/cJSON.c or -lcjson)
            // HSM persistence: serialize entire compartment chain including parent_compartment

            // First, generate helper functions for compartment serialization/deserialization
            // These will be generated as static functions before save_state/restore_state

            // Generate serialize_compartment helper
            let mut serialize_helper = String::new();
            serialize_helper.push_str(&format!("static cJSON* {}_serialize_compartment({}_Compartment* comp) {{\n", system.name, system.name));
            serialize_helper.push_str("    if (!comp) return cJSON_CreateNull();\n");
            serialize_helper.push_str("    cJSON* obj = cJSON_CreateObject();\n");
            serialize_helper.push_str("    cJSON_AddStringToObject(obj, \"state\", comp->state);\n");
            // Serialize state_vars (iterate over bucket-based linked list)
            serialize_helper.push_str("    cJSON* vars = cJSON_CreateObject();\n");
            serialize_helper.push_str(&format!("    {}_FrameDict* sv = comp->state_vars;\n", system.name));
            serialize_helper.push_str("    if (sv) {\n");
            serialize_helper.push_str("        for (int i = 0; i < sv->bucket_count; i++) {\n");
            serialize_helper.push_str(&format!("            {}_FrameDictEntry* entry = sv->buckets[i];\n", system.name));
            serialize_helper.push_str("            while (entry) {\n");
            serialize_helper.push_str("                cJSON_AddNumberToObject(vars, entry->key, (double)(intptr_t)entry->value);\n");
            serialize_helper.push_str("                entry = entry->next;\n");
            serialize_helper.push_str("            }\n");
            serialize_helper.push_str("        }\n");
            serialize_helper.push_str("    }\n");
            serialize_helper.push_str("    cJSON_AddItemToObject(obj, \"state_vars\", vars);\n");
            // Recursively serialize parent
            serialize_helper.push_str(&format!("    cJSON_AddItemToObject(obj, \"parent_compartment\", {}_serialize_compartment(comp->parent_compartment));\n", system.name));
            serialize_helper.push_str("    return obj;\n");
            serialize_helper.push_str("}\n\n");

            // Generate deserialize_compartment helper
            let mut deserialize_helper = String::new();
            deserialize_helper.push_str(&format!("static {}_Compartment* {}_deserialize_compartment(cJSON* data) {{\n", system.name, system.name));
            deserialize_helper.push_str("    if (!data || cJSON_IsNull(data)) return NULL;\n");
            deserialize_helper.push_str("    cJSON* state_item = cJSON_GetObjectItem(data, \"state\");\n");
            // strdup the state string since cJSON memory will be freed
            deserialize_helper.push_str(&format!("    {}_Compartment* comp = {}_Compartment_new(strdup(state_item->valuestring));\n", system.name, system.name));
            // Deserialize state_vars
            deserialize_helper.push_str("    cJSON* vars = cJSON_GetObjectItem(data, \"state_vars\");\n");
            deserialize_helper.push_str("    if (vars) {\n");
            deserialize_helper.push_str("        cJSON* var_item;\n");
            deserialize_helper.push_str("        cJSON_ArrayForEach(var_item, vars) {\n");
            deserialize_helper.push_str(&format!("            {}_FrameDict_set(comp->state_vars, var_item->string, (void*)(intptr_t)(int)var_item->valuedouble);\n", system.name));
            deserialize_helper.push_str("        }\n");
            deserialize_helper.push_str("    }\n");
            // Recursively deserialize parent
            deserialize_helper.push_str("    cJSON* parent = cJSON_GetObjectItem(data, \"parent_compartment\");\n");
            deserialize_helper.push_str(&format!("    comp->parent_compartment = {}_deserialize_compartment(parent);\n", system.name));
            deserialize_helper.push_str("    return comp;\n");
            deserialize_helper.push_str("}\n\n");

            // Add helper functions as a NativeBlock method (will be output before other methods)
            methods.push(CodegenNode::NativeBlock {
                code: serialize_helper + &deserialize_helper,
                span: None,
            });

            // Generate save_state function - returns char* (JSON string, caller must free)
            let mut save_body = String::new();
            save_body.push_str("cJSON* root = cJSON_CreateObject();\n");
            // Serialize entire compartment chain
            save_body.push_str(&format!("cJSON_AddItemToObject(root, \"_compartment\", {}_serialize_compartment(self->__compartment));\n", system.name));

            // Serialize state stack (simplified - just states for now)
            save_body.push_str("cJSON* stack_arr = cJSON_CreateArray();\n");
            save_body.push_str(&format!("for (int i = 0; i < {}_FrameVec_size(self->_state_stack); i++) {{\n", system.name));
            save_body.push_str(&format!("    {}_Compartment* comp = ({}_Compartment*){}_FrameVec_get(self->_state_stack, i);\n",
                system.name, system.name, system.name));
            save_body.push_str("    cJSON* stack_obj = cJSON_CreateObject();\n");
            save_body.push_str("    cJSON_AddStringToObject(stack_obj, \"state\", comp->state);\n");
            save_body.push_str("    cJSON_AddItemToArray(stack_arr, stack_obj);\n");
            save_body.push_str("}\n");
            save_body.push_str("cJSON_AddItemToObject(root, \"_state_stack\", stack_arr);\n");

            // Serialize domain variables
            for var in &system.domain {
                let type_str = extract_type_from_raw_domain(&var.raw_code, &var.name);

                let json_add = if is_int_type(&type_str) {
                    format!("cJSON_AddNumberToObject(root, \"{}\", (double)self->{});\n", var.name, var.name)
                } else if is_float_type(&type_str) {
                    format!("cJSON_AddNumberToObject(root, \"{}\", self->{});\n", var.name, var.name)
                } else if is_bool_type(&type_str) {
                    format!("cJSON_AddBoolToObject(root, \"{}\", self->{});\n", var.name, var.name)
                } else if is_string_type(&type_str) {
                    format!("cJSON_AddStringToObject(root, \"{}\", self->{});\n", var.name, var.name)
                } else {
                    format!("cJSON_AddNumberToObject(root, \"{}\", (double)(intptr_t)self->{});\n", var.name, var.name)
                };
                save_body.push_str(&json_add);
            }

            save_body.push_str("char* json = cJSON_PrintUnformatted(root);\n");
            save_body.push_str("cJSON_Delete(root);\n");
            save_body.push_str("return json;");

            methods.push(CodegenNode::Method {
                name: "save_state".to_string(),
                params: vec![],
                return_type: Some("char*".to_string()),
                body: vec![CodegenNode::NativeBlock {
                    code: save_body,
                    span: None,
                }],
                is_async: false,
                is_static: false,
                visibility: Visibility::Public,
                decorators: vec![],
            });

            // Generate restore_state function - takes const char*, returns instance pointer
            let mut restore_body = String::new();
            restore_body.push_str("cJSON* root = cJSON_Parse(json);\n");
            restore_body.push_str("if (!root) return NULL;\n\n");

            restore_body.push_str(&format!("{}* instance = malloc(sizeof({}));\n", system.name, system.name));
            restore_body.push_str(&format!("instance->_state_stack = {}_FrameVec_new();\n", system.name));
            restore_body.push_str(&format!("instance->_context_stack = {}_FrameVec_new();\n", system.name));
            restore_body.push_str("instance->__next_compartment = NULL;\n\n");

            // Restore entire compartment chain
            restore_body.push_str("cJSON* comp_data = cJSON_GetObjectItem(root, \"_compartment\");\n");
            restore_body.push_str(&format!("instance->__compartment = {}_deserialize_compartment(comp_data);\n\n", system.name));

            // Restore state stack
            restore_body.push_str("cJSON* stack_arr = cJSON_GetObjectItem(root, \"_state_stack\");\n");
            restore_body.push_str("if (stack_arr) {\n");
            restore_body.push_str("    cJSON* stack_item;\n");
            restore_body.push_str("    cJSON_ArrayForEach(stack_item, stack_arr) {\n");
            restore_body.push_str("        cJSON* state_obj = cJSON_GetObjectItem(stack_item, \"state\");\n");
            restore_body.push_str(&format!("        {}_Compartment* comp = {}_Compartment_new(strdup(state_obj->valuestring));\n",
                system.name, system.name));
            restore_body.push_str(&format!("        {}_FrameVec_push(instance->_state_stack, comp);\n", system.name));
            restore_body.push_str("    }\n");
            restore_body.push_str("}\n\n");

            // Restore domain variables
            for var in &system.domain {
                let type_str = extract_type_from_raw_domain(&var.raw_code, &var.name);

                let json_get = if is_int_type(&type_str) {
                    format!("instance->{} = (int)cJSON_GetObjectItem(root, \"{}\")->valuedouble;\n", var.name, var.name)
                } else if is_float_type(&type_str) {
                    format!("instance->{} = cJSON_GetObjectItem(root, \"{}\")->valuedouble;\n", var.name, var.name)
                } else if is_bool_type(&type_str) {
                    format!("instance->{} = cJSON_IsTrue(cJSON_GetObjectItem(root, \"{}\"));\n", var.name, var.name)
                } else if is_string_type(&type_str) {
                    format!("instance->{} = strdup(cJSON_GetObjectItem(root, \"{}\")->valuestring);\n", var.name, var.name)
                } else {
                    format!("instance->{} = (int)cJSON_GetObjectItem(root, \"{}\")->valuedouble;\n", var.name, var.name)
                };
                restore_body.push_str(&json_get);
            }

            restore_body.push_str("\ncJSON_Delete(root);\n");
            restore_body.push_str("return instance;");

            methods.push(CodegenNode::Method {
                name: "restore_state".to_string(),
                params: vec![Param::new("json").with_type("const char*")],
                return_type: Some(format!("{}*", system.name)),
                body: vec![CodegenNode::NativeBlock {
                    code: restore_body,
                    span: None,
                }],
                is_async: false,
                is_static: true,
                visibility: Visibility::Public,
                decorators: vec![],
            });
        }
        _ => {
            // Other languages not yet supported
        }
    }

    methods
}


/// Convert Type enum to string representation
fn type_to_string(t: &Type) -> String {
    match t {
        Type::Custom(name) => name.clone(),
        Type::Unknown => "Any".to_string(),
    }
}

/// Convert Expression AST to CodegenNode
fn convert_expression(expr: &Expression) -> CodegenNode {
    match expr {
        Expression::Var(name) => CodegenNode::ident(name),
        Expression::Literal(lit) => convert_literal(lit),
        Expression::Binary { left, op, right } => {
            let codegen_op = match op {
                BinaryOp::Add => crate::frame_c::v4::codegen::ast::BinaryOp::Add,
                BinaryOp::Sub => crate::frame_c::v4::codegen::ast::BinaryOp::Sub,
                BinaryOp::Mul => crate::frame_c::v4::codegen::ast::BinaryOp::Mul,
                BinaryOp::Div => crate::frame_c::v4::codegen::ast::BinaryOp::Div,
                BinaryOp::Mod => crate::frame_c::v4::codegen::ast::BinaryOp::Mod,
                BinaryOp::Eq => crate::frame_c::v4::codegen::ast::BinaryOp::Eq,
                BinaryOp::Ne => crate::frame_c::v4::codegen::ast::BinaryOp::Ne,
                BinaryOp::Lt => crate::frame_c::v4::codegen::ast::BinaryOp::Lt,
                BinaryOp::Le => crate::frame_c::v4::codegen::ast::BinaryOp::Le,
                BinaryOp::Gt => crate::frame_c::v4::codegen::ast::BinaryOp::Gt,
                BinaryOp::Ge => crate::frame_c::v4::codegen::ast::BinaryOp::Ge,
                BinaryOp::And => crate::frame_c::v4::codegen::ast::BinaryOp::And,
                BinaryOp::Or => crate::frame_c::v4::codegen::ast::BinaryOp::Or,
                BinaryOp::BitAnd => crate::frame_c::v4::codegen::ast::BinaryOp::BitAnd,
                BinaryOp::BitOr => crate::frame_c::v4::codegen::ast::BinaryOp::BitOr,
                BinaryOp::BitXor => crate::frame_c::v4::codegen::ast::BinaryOp::BitXor,
            };
            CodegenNode::BinaryOp {
                op: codegen_op,
                left: Box::new(convert_expression(left)),
                right: Box::new(convert_expression(right)),
            }
        }
        Expression::Unary { op, expr } => {
            let codegen_op = match op {
                UnaryOp::Neg => crate::frame_c::v4::codegen::ast::UnaryOp::Neg,
                UnaryOp::Not => crate::frame_c::v4::codegen::ast::UnaryOp::Not,
                UnaryOp::BitNot => crate::frame_c::v4::codegen::ast::UnaryOp::BitNot,
            };
            CodegenNode::UnaryOp {
                op: codegen_op,
                operand: Box::new(convert_expression(expr)),
            }
        }
        Expression::Call { func, args } => {
            CodegenNode::Call {
                target: Box::new(CodegenNode::ident(func)),
                args: args.iter().map(convert_expression).collect(),
            }
        }
        Expression::Index { object, index } => {
            CodegenNode::IndexAccess {
                object: Box::new(convert_expression(object)),
                index: Box::new(convert_expression(index)),
            }
        }
        Expression::Member { object, field } => {
            CodegenNode::FieldAccess {
                object: Box::new(convert_expression(object)),
                field: field.clone(),
            }
        }
        Expression::Assign { target, value } => {
            CodegenNode::assign(
                convert_expression(target),
                convert_expression(value),
            )
        }
        Expression::NativeExpr(code) => {
            // Pass through native expression verbatim
            CodegenNode::native(code)
        }
    }
}

/// Convert Literal to CodegenNode
fn convert_literal(lit: &Literal) -> CodegenNode {
    match lit {
        Literal::Int(n) => CodegenNode::int(*n),
        Literal::Float(f) => CodegenNode::float(*f),
        Literal::String(s) => CodegenNode::string(s),
        Literal::Bool(b) => CodegenNode::bool(*b),
        Literal::Null => CodegenNode::null(),
    }
}

/// Extract type from raw domain declaration
/// Handles formats: "name: type = init" (Frame) or "type name = init" (C-style)
fn extract_type_from_raw_domain(raw_code: &Option<String>, name: &str) -> String {
    match raw_code {
        Some(code) => {
            let code = code.trim();

            // Try Frame-style: "name: type = init" or "name: type"
            if let Some(colon_pos) = code.find(':') {
                let before_colon = &code[..colon_pos].trim();
                // Verify it's the variable name before the colon
                if before_colon.ends_with(name) || *before_colon == name {
                    let after_colon = &code[colon_pos + 1..];
                    // Extract type until '=' or end of line
                    let type_end = after_colon.find('=').unwrap_or(after_colon.len());
                    return after_colon[..type_end].trim().to_string();
                }
            }

            // Try C-style: "type name = init" - first word is type
            let first_word = code.split_whitespace().next().unwrap_or("");
            first_word.to_string()
        }
        None => String::new(),
    }
}

/// Check if type string represents an integer type
fn is_int_type(type_str: &str) -> bool {
    matches!(type_str, "int" | "i32" | "i64" | "i8" | "i16" | "u8" | "u16" | "u32" | "u64"
             | "int8_t" | "int16_t" | "int32_t" | "int64_t"
             | "uint8_t" | "uint16_t" | "uint32_t" | "uint64_t")
}

/// Check if type string represents a float type
fn is_float_type(type_str: &str) -> bool {
    matches!(type_str, "float" | "double" | "f32" | "f64")
}

/// Check if type string represents a boolean type
fn is_bool_type(type_str: &str) -> bool {
    matches!(type_str, "bool" | "boolean" | "_Bool")
}

/// Check if type string represents a string type
fn is_string_type(type_str: &str) -> bool {
    matches!(type_str, "str" | "string" | "String" | "char*" | "&str")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::frame_ast::{SystemAst, DomainVar, Type, Expression, Literal, Span};
    use crate::frame_c::visitors::TargetLanguage;

    fn create_test_system() -> SystemAst {
        SystemAst::new("TestSystem".to_string(), Span::new(0, 0))
    }

    #[test]
    fn test_generate_simple_system() {
        let system = create_test_system();
        let arcanum = Arcanum::new();
        // Empty source since test system has no actions/operations with native code
        let source = b"";
        let node = generate_system(&system, &arcanum, TargetLanguage::Python3, source);

        match node {
            CodegenNode::Class { name, .. } => {
                assert_eq!(name, "TestSystem");
            }
            _ => panic!("Expected Class node"),
        }
    }

    #[test]
    fn test_generate_fields() {
        let mut system = create_test_system();
        system.domain.push(DomainVar {
            name: "counter".to_string(),
            var_type: Type::Custom("int".into()),
            initializer: Some(Expression::Literal(Literal::Int(0))),
            is_frame: false,
            raw_code: None,
            span: Span::new(0, 0),
        });

        let syntax = super::super::backend::ClassSyntax::python();
        let fields = generate_fields(&system, &syntax);

        // Should have _state, _state_stack, _state_context, and counter
        assert!(fields.len() >= 4);
        assert!(fields.iter().any(|f| f.name == "counter"));
    }

    #[test]
    fn test_generate_constructor() {
        let system = create_test_system();
        let syntax = super::super::backend::ClassSyntax::python();
        let constructor = generate_constructor(&system, &syntax);

        match constructor {
            CodegenNode::Constructor { body, .. } => {
                assert!(!body.is_empty());
            }
            _ => panic!("Expected Constructor node"),
        }
    }
}

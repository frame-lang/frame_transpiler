//! System Code Generation from Frame AST
//!
//! This module transforms Frame AST (SystemAst) into CodegenNode for emission
//! by language-specific backends.
//!
//! Uses the "oceans model" - native code is preserved exactly, Frame segments
//! are replaced with generated code using the splicer.

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::frame_ast::{
    SystemAst, StateAst, HandlerAst, HandlerBody, Statement, MachineAst,
    ActionAst, OperationAst, Type, LoopKind, EventParam,
    Expression, Literal, BinaryOp, UnaryOp, StateVarAst,
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
    /// True if the system has states with state variables (for Rust compartment-based push/pop)
    pub has_state_vars: bool,
    /// Map of state names to their parent state (for HSM child state transitions)
    pub state_parents: std::collections::HashMap<String, String>,
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
    // For Rust: Vec<(String, {System}StateContext)> - typed context enum for stack
    // For others: list/array of compartments
    let stack_type = if matches!(syntax.language, TargetLanguage::Rust) {
        format!("Vec<(String, {}StateContext)>", system.name)
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

    // Return value field for system.return (not needed for Rust since we use native return)
    if !matches!(syntax.language, TargetLanguage::Rust) {
        fields.push(Field::new("_return_value")
            .with_visibility(Visibility::Private)
            .with_type("Any"));
    }

    // Domain variables
    for domain_var in &system.domain {
        let mut field = Field::new(&domain_var.name)
            .with_visibility(Visibility::Private);

        // Convert Type enum to string representation
        let type_str = type_to_string(&domain_var.var_type);
        field = field.with_type(&type_str);

        if let Some(ref init) = &domain_var.initializer {
            // Convert initializer expression to CodegenNode
            field = field.with_initializer(convert_expression(init));
        }

        fields.push(field);
    }

    // For Rust: Generate _sv_ fields for state variables (for direct access in handlers)
    // These are initialized in _enter() and saved/restored via compartment on push/pop
    if matches!(syntax.language, TargetLanguage::Rust) {
        if let Some(ref machine) = system.machine {
            for state in &machine.states {
                for var in &state.state_vars {
                    let type_str = type_to_string(&var.var_type);
                    fields.push(Field::new(&format!("_sv_{}", var.name))
                        .with_visibility(Visibility::Private)
                        .with_type(&type_str));
                }
            }
        }
    }

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
/// The FrameEvent class encapsulates event information:
/// - _message: string - Event name (e.g., "$>", "<$", "start")
/// - _parameters: dict - Event parameters (positional args as indexed dict)
/// - _return: any - Return value (set by handlers using ^(value))
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

    // Constructor body: initialize fields
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
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_return"),
                CodegenNode::null(),
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
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_return"),
                CodegenNode::null(),
            ),
        ],
        _ => vec![],
    };

    // Fields for TypeScript (Python doesn't need field declarations)
    let fields = if matches!(lang, TargetLanguage::TypeScript) {
        vec![
            Field::new("_message").with_type("string").with_visibility(Visibility::Public),
            Field::new("_parameters").with_type("Record<string, any> | null").with_visibility(Visibility::Public),
            Field::new("_return").with_type("any").with_visibility(Visibility::Public),
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

    // Generate FrameEvent struct
    code.push_str(&format!("#[derive(Clone, Debug)]\nstruct {}FrameEvent {{\n", system_name));
    code.push_str("    message: String,\n");
    code.push_str("}\n\n");

    // Generate FrameEvent impl with new()
    code.push_str(&format!("impl {}FrameEvent {{\n", system_name));
    code.push_str("    fn new(message: &str) -> Self {\n");
    code.push_str("        Self { message: message.to_string() }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate Compartment struct
    code.push_str(&format!("#[derive(Clone)]\nstruct {}Compartment {{\n", system_name));
    code.push_str("    state: String,\n");
    code.push_str("    state_vars: std::collections::HashMap<String, i32>,\n");
    code.push_str(&format!("    forward_event: Option<{}FrameEvent>,\n", system_name));
    code.push_str("}\n\n");

    // Generate Compartment impl with new()
    code.push_str(&format!("impl {}Compartment {{\n", system_name));
    code.push_str("    fn new(state: &str) -> Self {\n");
    code.push_str("        Self {\n");
    code.push_str("            state: state.to_string(),\n");
    code.push_str("            state_vars: std::collections::HashMap::new(),\n");
    code.push_str("            forward_event: None,\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Generate state context types for stack push/pop operations
    // Always generate for Rust systems with a machine (needed for stack operations)
    if system.machine.is_some() {
        // Collect states with state variables
        let states_with_vars: Vec<_> = system.machine.as_ref()
            .map(|m| m.states.iter()
                .filter(|s| !s.state_vars.is_empty())
                .collect())
            .unwrap_or_default();

        // Generate context structs for each state with vars
        for state in &states_with_vars {
            code.push_str(&format!("#[derive(Clone, Default)]\nstruct {}Context {{\n", state.name));
            for var in &state.state_vars {
                let type_str = type_to_string(&var.var_type);
                code.push_str(&format!("    {}: {},\n", var.name, type_str));
            }
            code.push_str("}\n\n");
        }

        // Generate compartment enum for typed state variable storage
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

        // Generate Default impl for the enum
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
    }

    code
}

/// Generate the constructor
fn generate_constructor(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> CodegenNode {
    let mut body = Vec::new();

    // Initialize state stack
    body.push(CodegenNode::assign(
        CodegenNode::field(CodegenNode::self_ref(), "_state_stack"),
        CodegenNode::Array(vec![]),
    ));

    // Initialize return value (for system.return chain semantics)
    // Not needed for Rust which uses native return
    if !matches!(syntax.language, TargetLanguage::Rust) {
        body.push(CodegenNode::assign(
            CodegenNode::field(CodegenNode::self_ref(), "_return_value"),
            CodegenNode::null(),
        ));
    }

    // Initialize domain variables
    for domain_var in &system.domain {
        if let Some(ref init) = &domain_var.initializer {
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), &domain_var.name),
                convert_expression(init),
            ));
        }
    }

    // For Rust: Initialize _sv_ fields with default values
    // They'll be properly initialized in _enter() when entering the start state
    if matches!(syntax.language, TargetLanguage::Rust) {
        if let Some(ref machine) = system.machine {
            for state in &machine.states {
                for var in &state.state_vars {
                    let init_value = match &var.var_type {
                        Type::Int => CodegenNode::int(0),
                        Type::Float => CodegenNode::float(0.0),
                        Type::Bool => CodegenNode::bool(false),
                        Type::String => CodegenNode::string(""),
                        Type::Custom(name) => {
                            match name.to_lowercase().as_str() {
                                "i32" | "i64" | "u32" | "u64" | "isize" | "usize" => CodegenNode::int(0),
                                "f32" | "f64" => CodegenNode::float(0.0),
                                "bool" => CodegenNode::bool(false),
                                _ => CodegenNode::int(0), // Default for unknown types
                            }
                        }
                        Type::Unknown => CodegenNode::int(0),
                    };
                    body.push(CodegenNode::assign(
                        CodegenNode::field(CodegenNode::self_ref(), &format!("_sv_{}", var.name)),
                        init_value,
                    ));
                }
            }
        }
    }

    // Set initial state (first state in machine)
    // All languages now use the kernel/router/compartment pattern
    if let Some(ref machine) = system.machine {
        if let Some(first_state) = machine.states.first() {
            let compartment_class = format!("{}Compartment", system.name);
            let event_class = format!("{}FrameEvent", system.name);

            // Initialize __compartment with initial state
            match syntax.language {
                TargetLanguage::Rust => {
                    // For Rust, we need to use Ident with the full constructor expression
                    // This is a workaround since CodegenNode::New doesn't work for Rust ::new()
                    body.push(CodegenNode::assign(
                        CodegenNode::field(CodegenNode::self_ref(), "__compartment"),
                        CodegenNode::Ident(format!("{}::new(\"{}\")", compartment_class, first_state.name)),
                    ));
                    body.push(CodegenNode::assign(
                        CodegenNode::field(CodegenNode::self_ref(), "__next_compartment"),
                        CodegenNode::Ident("None".to_string()),
                    ));
                }
                _ => {
                    // Python/TypeScript: New expression
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
fn generate_frame_machinery(system: &SystemAst, syntax: &super::backend::ClassSyntax, lang: TargetLanguage) -> Vec<CodegenNode> {
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
    // Exit current state
    let exit_event = {}::new("$<");
    self.__router(&exit_event);
    // Switch to new compartment
    self.__compartment = next_compartment;
    // Enter new state (or forward event)
    if self.__compartment.forward_event.is_none() {{
        let enter_event = {}::new("$>");
        self.__router(&enter_event);
    }} else {{
        // Forward event to new state
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {{
            // Forwarding enter event - just send it
            self.__router(&forward_event);
        }} else {{
            // Forwarding other event - send $> first, then forward
            let enter_event = {}::new("$>");
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

    // For Rust: Generate _state_stack_push() and _state_stack_pop() methods
    // These handle typed compartment save/restore for state variable preservation
    // Only generate when there are states with state variables
    let has_state_vars = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| !s.state_vars.is_empty()))
        .unwrap_or(false);
    // Generate stack push/pop methods for Rust when there's a machine
    // These are needed for any system that uses stack operations (push$/pop$)
    if matches!(lang, TargetLanguage::Rust) && system.machine.is_some() {
        methods.push(generate_rust_state_stack_push(system));
        methods.push(generate_rust_state_stack_pop(system));
    }

    methods
}

/// Generate Rust _state_stack_push method
/// Builds a state context from current _sv_ fields and pushes to stack
fn generate_rust_state_stack_push(system: &SystemAst) -> CodegenNode {
    let system_name = &system.name;
    let mut code = String::new();

    // Build state context from current _sv_ fields based on current state
    code.push_str("let state_context = match self.__compartment.state.as_str() {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            if state.state_vars.is_empty() {
                // State without vars - unit variant
                code.push_str(&format!(
                    "    \"{}\" => {}StateContext::{},\n",
                    state.name, system_name, state.name
                ));
            } else {
                // State with vars - build context struct from _sv_ fields
                let field_inits: Vec<String> = state.state_vars.iter()
                    .map(|v| format!("{}: self._sv_{}", v.name, v.name))
                    .collect();
                code.push_str(&format!(
                    "    \"{}\" => {}StateContext::{}({}Context {{ {} }}),\n",
                    state.name, system_name, state.name, state.name, field_inits.join(", ")
                ));
            }
        }
    }

    code.push_str(&format!("    _ => {}StateContext::Empty,\n", system_name));
    code.push_str("};\n");
    code.push_str("self._state_stack.push((self.__compartment.state.clone(), state_context));");

    CodegenNode::Method {
        name: "_state_stack_push".to_string(),
        params: vec![],
        return_type: None,
        body: vec![CodegenNode::NativeBlock { code, span: None }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate Rust _state_stack_pop method
/// Pops from stack, sends exit event, restores _sv_ fields from context
fn generate_rust_state_stack_pop(system: &SystemAst) -> CodegenNode {
    let system_name = &system.name;
    let event_class = format!("{}FrameEvent", system_name);
    let mut code = String::new();

    // Pop the saved state and context
    code.push_str("let (saved_state, state_context) = self._state_stack.pop().unwrap();\n");
    // Send exit event to current state via kernel pattern
    code.push_str(&format!("let exit_event = {}::new(\"$<\");\n", event_class));
    code.push_str("self.__router(&exit_event);\n");
    // Update compartment state
    code.push_str("self.__compartment.state = saved_state;\n");

    // Restore _sv_ fields from state context
    code.push_str("match state_context {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            if state.state_vars.is_empty() {
                // State without vars - nothing to restore
                code.push_str(&format!(
                    "    {}StateContext::{} => {{}}\n",
                    system_name, state.name
                ));
            } else {
                // State with vars - restore from context
                code.push_str(&format!(
                    "    {}StateContext::{}(ctx) => {{\n",
                    system_name, state.name
                ));
                for var in &state.state_vars {
                    code.push_str(&format!(
                        "        self._sv_{} = ctx.{};\n",
                        var.name, var.name
                    ));
                }
                code.push_str("    }\n");
            }
        }
    }

    code.push_str(&format!("    {}StateContext::Empty => {{}}\n", system_name));
    code.push_str("}");

    CodegenNode::Method {
        name: "_state_stack_pop".to_string(),
        params: vec![],
        return_type: None,
        body: vec![CodegenNode::NativeBlock { code, span: None }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate enter event dispatcher
/// Uses language-specific dynamic dispatch pattern
/// Also initializes state variables for the target state
fn generate_enter_dispatcher(system: &SystemAst, lang: TargetLanguage) -> CodegenNode {
    // Check if any states have enter handlers
    let has_enter_handlers = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| s.enter.is_some()))
        .unwrap_or(false);

    // Check if any states have state variables
    let has_state_vars = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| !s.state_vars.is_empty()))
        .unwrap_or(false);

    // Generate state variable initialization code
    let state_var_init = if has_state_vars {
        generate_state_var_init(system, lang)
    } else {
        String::new()
    };

    // Language-specific params for varargs
    let params = match lang {
        TargetLanguage::Python3 => vec![Param::new("*args")],
        TargetLanguage::TypeScript => vec![Param::new("...args").with_type("any[]")],
        _ => vec![],
    };

    let body = if !has_enter_handlers && !has_state_vars {
        vec![CodegenNode::comment("No enter handlers")]
    } else {
        match lang {
            TargetLanguage::Python3 => {
                let init_code = if !state_var_init.is_empty() {
                    format!("{}\n", state_var_init)
                } else {
                    String::new()
                };
                let handler_code = if has_enter_handlers {
                    "handler_name = f\"_s_{self._compartment.state}_enter\"\nhandler = getattr(self, handler_name, None)\nif handler:\n    handler(*args)"
                } else {
                    ""
                };
                vec![CodegenNode::NativeBlock {
                    code: format!("{}{}", init_code, handler_code),
                    span: None,
                }]
            }
            TargetLanguage::TypeScript => {
                let init_code = if !state_var_init.is_empty() {
                    format!("{}\n", state_var_init)
                } else {
                    String::new()
                };
                let handler_code = if has_enter_handlers {
                    "const handler_name = `_s_${this._compartment.state}_enter`;\nconst handler = (this as any)[handler_name];\nif (handler) {\n    handler.call(this, ...args);\n}"
                } else {
                    ""
                };
                vec![CodegenNode::NativeBlock {
                    code: format!("{}{}", init_code, handler_code),
                    span: None,
                }]
            }
            TargetLanguage::Rust => {
                // Generate match-based dispatch for Rust with state var init
                vec![CodegenNode::NativeBlock {
                    code: generate_rust_enter_dispatch_with_vars(system),
                    span: None,
                }]
            }
            _ => {
                vec![CodegenNode::comment("Enter dispatch needed for this language")]
            }
        }
    };

    CodegenNode::Method {
        name: "_enter".to_string(),
        params,
        return_type: None,
        body,
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate exit event dispatcher
/// Uses language-specific dynamic dispatch pattern
fn generate_exit_dispatcher(system: &SystemAst, lang: TargetLanguage) -> CodegenNode {
    // Check if any states have exit handlers
    let has_exit_handlers = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| s.exit.is_some()))
        .unwrap_or(false);

    // Language-specific params for varargs
    let params = match lang {
        TargetLanguage::Python3 => vec![Param::new("*args")],
        TargetLanguage::TypeScript => vec![Param::new("...args").with_type("any[]")],
        _ => vec![],
    };

    let body = if !has_exit_handlers {
        vec![CodegenNode::comment("No exit handlers")]
    } else {
        match lang {
            TargetLanguage::Python3 => {
                vec![CodegenNode::NativeBlock {
                    code: "handler_name = f\"_s_{self._compartment.state}_exit\"\nhandler = getattr(self, handler_name, None)\nif handler:\n    handler(*args)".to_string(),
                    span: None,
                }]
            }
            TargetLanguage::TypeScript => {
                vec![CodegenNode::NativeBlock {
                    code: "const handler_name = `_s_${this._compartment.state}_exit`;\nconst handler = (this as any)[handler_name];\nif (handler) {\n    handler.call(this, ...args);\n}".to_string(),
                    span: None,
                }]
            }
            TargetLanguage::Rust => {
                // Generate match-based dispatch for Rust
                vec![CodegenNode::NativeBlock {
                    code: generate_rust_enter_exit_dispatch(system, "exit"),
                    span: None,
                }]
            }
            _ => {
                vec![CodegenNode::comment("Exit dispatch needed for this language")]
            }
        }
    };

    CodegenNode::Method {
        name: "_exit".to_string(),
        params,
        return_type: None,
        body,
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate state variable initialization code for Python/TypeScript
/// Initializes state variables in compartment.state_vars
fn generate_state_var_init(system: &SystemAst, lang: TargetLanguage) -> String {
    let mut code = String::new();
    let self_ref = match lang {
        TargetLanguage::Python3 => "self",
        _ => "this",
    };

    if let Some(ref machine) = system.machine {
        // Build a switch/if-else chain based on current state
        let mut first = true;
        for state in &machine.states {
            if state.state_vars.is_empty() {
                continue;
            }

            // Python/TypeScript: use _compartment.state (no _state field)
            let condition = if first {
                format!("if {}._compartment.state == \"{}\":", self_ref, state.name)
            } else {
                format!("elif {}._compartment.state == \"{}\":", self_ref, state.name)
            };
            first = false;

            match lang {
                TargetLanguage::Python3 => {
                    code.push_str(&format!("{}\n", condition));
                    for var in &state.state_vars {
                        // Use explicit initializer if provided, otherwise fall back to type default
                        let init_val = if let Some(ref init) = var.init {
                            expression_to_string(init, lang)
                        } else {
                            state_var_init_value(&var.var_type, lang)
                        };
                        // Initialize in compartment.state_vars (Phase 14.6 - no longer using _state_context)
                        code.push_str(&format!("    {}._compartment.state_vars[\"{}\"] = {}\n", self_ref, var.name, init_val));
                    }
                }
                TargetLanguage::TypeScript => {
                    let ts_condition = if code.is_empty() {
                        format!("if ({}._compartment.state === \"{}\") {{\n", self_ref, state.name)
                    } else {
                        format!("}} else if ({}._compartment.state === \"{}\") {{\n", self_ref, state.name)
                    };
                    code.push_str(&ts_condition);
                    for var in &state.state_vars {
                        let init_val = if let Some(ref init) = var.init {
                            expression_to_string(init, lang)
                        } else {
                            state_var_init_value(&var.var_type, lang)
                        };
                        // Initialize in compartment.state_vars (Phase 14.6 - no longer using _state_context)
                        code.push_str(&format!("    {}._compartment.state_vars[\"{}\"] = {};\n", self_ref, var.name, init_val));
                    }
                }
                _ => {}
            }
        }
        // Close TypeScript braces
        if matches!(lang, TargetLanguage::TypeScript) && !code.is_empty() {
            code.push_str("}");
        }
    }

    code
}

/// Get default initialization value for a type
fn state_var_init_value(var_type: &Type, lang: TargetLanguage) -> String {
    match var_type {
        Type::Int => "0".to_string(),
        Type::Float => "0.0".to_string(),
        Type::Bool => match lang {
            TargetLanguage::Python3 => "False".to_string(),
            _ => "false".to_string(),
        },
        Type::String => "\"\"".to_string(),
        Type::Custom(name) => {
            // Try to infer from common type names
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

/// Generate Rust enter dispatch with state variable initialization
/// Initializes _sv_ fields for the target state
fn generate_rust_enter_dispatch_with_vars(system: &SystemAst) -> String {
    let mut match_code = String::new();
    match_code.push_str("match self._state.as_str() {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            let has_enter = state.enter.is_some();
            let has_vars = !state.state_vars.is_empty();

            if !has_enter && !has_vars {
                continue;
            }

            match_code.push_str(&format!("    \"{}\" => {{\n", state.name));

            // Initialize _sv_ fields for this state
            for var in &state.state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::Rust)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::Rust)
                };
                match_code.push_str(&format!(
                    "        self._sv_{} = {};\n",
                    var.name, init_val
                ));
            }

            // Call enter handler if exists
            if has_enter {
                match_code.push_str(&format!("        self._s_{}_enter();\n", state.name));
            }

            match_code.push_str("    }\n");
        }
    }

    match_code.push_str("    _ => {}\n");
    match_code.push_str("}");
    match_code
}

/// Generate Rust match-based dispatch for enter/exit handlers
fn generate_rust_enter_exit_dispatch(system: &SystemAst, handler_type: &str) -> String {
    let mut match_code = String::new();
    match_code.push_str("match self._state.as_str() {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            // Check if this state has the handler type
            let has_handler = if handler_type == "enter" {
                state.enter.is_some()
            } else {
                state.exit.is_some()
            };

            if has_handler {
                let handler_name = format!("_s_{}_{}", state.name, handler_type);
                match_code.push_str(&format!("            \"{}\" => {{ self.{}(); }}\n", state.name, handler_name));
            }
        }
    }

    match_code.push_str("            _ => {}\n");
    match_code.push_str("        }");
    match_code
}

/// Generate interface wrapper methods
///
/// For Python/TypeScript: Create FrameEvent and call __kernel
/// For Rust: Use match-based dispatch directly
fn generate_interface_wrappers(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> Vec<CodegenNode> {
    // Get the target language from the syntax
    let lang = syntax.language;
    let event_class = format!("{}FrameEvent", system.name);

    system.interface.iter().map(|method| {
        let params: Vec<Param> = method.params.iter().map(|p| {
            let type_str = type_to_string(&p.param_type);
            Param::new(&p.name).with_type(&type_str)
        }).collect();

        let args: Vec<CodegenNode> = method.params.iter()
            .map(|p| CodegenNode::ident(&p.name))
            .collect();

        // Language-specific dispatch - all languages now use kernel pattern
        let body_stmt = match lang {
            TargetLanguage::Rust => {
                // Rust: Create FrameEvent, call __kernel
                // For return values or methods with parameters, use direct dispatch
                // (kernel pattern doesn't support passing parameters)
                if method.return_type.is_some() || !method.params.is_empty() {
                    // Handlers with return types or parameters need direct dispatch
                    let has_return = method.return_type.is_some();
                    generate_rust_interface_dispatch(system, &method.name, &args, has_return)
                } else {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"let __e = {}::new("{}");
self.__kernel(__e)"#,
                            event_class, method.name
                        ),
                        span: None,
                    }
                }
            }
            TargetLanguage::Python3 => {
                // Python: Create FrameEvent, call __kernel, return __e._return
                // Parameters are passed as a dict with string keys "0", "1", etc.
                let params_code = if method.params.is_empty() {
                    "None".to_string()
                } else {
                    let param_items: Vec<String> = method.params.iter().enumerate()
                        .map(|(i, p)| format!("\"{}\": {}", i, p.name))
                        .collect();
                    format!("{{{}}}", param_items.join(", "))
                };

                if method.return_type.is_some() {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"self._return_value = None
__e = {}("{}", {})
self.__kernel(__e)
return self._return_value"#,
                            event_class, method.name, params_code
                        ),
                        span: None,
                    }
                } else {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"__e = {}("{}", {})
self.__kernel(__e)"#,
                            event_class, method.name, params_code
                        ),
                        span: None,
                    }
                }
            }
            TargetLanguage::TypeScript => {
                // TypeScript: Create FrameEvent, call __kernel, return _return_value
                let params_code = if method.params.is_empty() {
                    "null".to_string()
                } else {
                    let param_items: Vec<String> = method.params.iter().enumerate()
                        .map(|(i, p)| format!("\"{}\": {}", i, p.name))
                        .collect();
                    format!("{{{}}}", param_items.join(", "))
                };

                if method.return_type.is_some() {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"this._return_value = null;
const __e = new {}("{}", {});
this.__kernel(__e);
return this._return_value;"#,
                            event_class, method.name, params_code
                        ),
                        span: None,
                    }
                } else {
                    CodegenNode::NativeBlock {
                        code: format!(
                            r#"const __e = new {}("{}", {});
this.__kernel(__e);"#,
                            event_class, method.name, params_code
                        ),
                        span: None,
                    }
                }
            }
            _ => {
                // Default: Same as TypeScript
                let params_code = if method.params.is_empty() {
                    "null".to_string()
                } else {
                    let param_items: Vec<String> = method.params.iter().enumerate()
                        .map(|(i, p)| format!("\"{}\": {}", i, p.name))
                        .collect();
                    format!("{{{}}}", param_items.join(", "))
                };
                CodegenNode::NativeBlock {
                    code: format!(
                        r#"const __e = new {}("{}", {});
this.__kernel(__e);"#,
                        event_class, method.name, params_code
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

/// Generate Rust match-based dispatch for interface methods with return values
/// Used when kernel pattern can't easily return values
/// Creates a FrameEvent and passes it to handlers (which now require __e)
fn generate_rust_interface_dispatch(system: &SystemAst, event: &str, args: &[CodegenNode], has_return: bool) -> CodegenNode {
    let event_class = format!("{}FrameEvent", system.name);

    // Build match arms for each state that handles this event
    let mut match_code = String::new();

    // Convert args to comma-separated parameter names
    let args_str = if args.is_empty() {
        String::new()
    } else {
        let arg_names: Vec<String> = args.iter().map(|arg| {
            match arg {
                CodegenNode::Ident(name) => name.clone(),
                _ => "".to_string(),
            }
        }).collect();
        format!(", {}", arg_names.join(", "))
    };

    // Create FrameEvent first
    match_code.push_str(&format!("let __e = {}::new(\"{}\");\n", event_class, event));
    match_code.push_str("match self.__compartment.state.as_str() {\n");

    if let Some(ref machine) = system.machine {
        // First pass: collect states that handle the event directly
        let mut states_with_handler: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for state in &machine.states {
            if state.handlers.iter().any(|h| h.event == event) {
                states_with_handler.insert(&state.name);
            }
        }

        for state in &machine.states {
            // Check if this state handles the event directly
            let handles_event = states_with_handler.contains(state.name.as_str());

            if handles_event {
                let handler_name = format!("_s_{}_{}", state.name, event);
                if has_return {
                    match_code.push_str(&format!("            \"{}\" => self.{}(&__e{}),\n", state.name, handler_name, args_str));
                } else {
                    match_code.push_str(&format!("            \"{}\" => {{ self.{}(&__e{}); }}\n", state.name, handler_name, args_str));
                }
            } else if state.default_forward {
                // State has default_forward but no handler for this event - forward to parent
                if let Some(ref parent) = state.parent {
                    // Check if parent handles this event
                    if states_with_handler.contains(parent.as_str()) {
                        let parent_handler_name = format!("_s_{}_{}", parent, event);
                        if has_return {
                            match_code.push_str(&format!("            \"{}\" => self.{}(&__e{}),\n", state.name, parent_handler_name, args_str));
                        } else {
                            match_code.push_str(&format!("            \"{}\" => {{ self.{}(&__e{}); }}\n", state.name, parent_handler_name, args_str));
                        }
                    }
                }
            }
        }
    }

    // Default arm
    if has_return {
        match_code.push_str("            _ => Default::default(),\n");
    } else {
        match_code.push_str("            _ => {}\n");
    }
    match_code.push_str("        }");

    // For methods without return values, process any pending transitions
    // (since we bypassed the kernel which normally handles this)
    if !has_return {
        match_code.push_str(&format!(r#"
// Process any pending transitions (bypassed kernel)
while self.__next_compartment.is_some() {{
    let next_compartment = self.__next_compartment.take().unwrap();
    let exit_event = {}::new("$<");
    self.__router(&exit_event);
    self.__compartment = next_compartment;
    if self.__compartment.forward_event.is_none() {{
        let enter_event = {}::new("$>");
        self.__router(&enter_event);
    }} else {{
        let forward_event = self.__compartment.forward_event.take().unwrap();
        if forward_event.message == "$>" {{
            self.__router(&forward_event);
        }} else {{
            let enter_event = {}::new("$>");
            self.__router(&enter_event);
            self.__router(&forward_event);
        }}
    }}
}}"#, event_class, event_class, event_class));
    }

    CodegenNode::NativeBlock {
        code: match_code,
        span: None,
    }
}

/// Generate state handler methods using the enhanced Arcanum
///
/// For all languages: Generates `_state_{StateName}(__e)` methods that dispatch internally
/// based on the event message, plus individual handler methods
fn generate_state_handlers_via_arcanum(system_name: &str, machine: &MachineAst, arcanum: &Arcanum, source: &[u8], lang: TargetLanguage, has_state_vars: bool) -> Vec<CodegenNode> {
    let mut methods = Vec::new();

    // Generate one _state_{StateName} dispatch method per state for ALL languages
    for state_entry in arcanum.get_enhanced_states(system_name) {
        // Find state variables and default_forward for this state from the machine AST
        let state_ast = machine.states.iter().find(|s| s.name == state_entry.name);
        let state_vars = state_ast.map(|s| &s.state_vars[..]).unwrap_or(&[]);
        let default_forward = state_ast.map(|s| s.default_forward).unwrap_or(false);

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
    has_state_vars: bool,
    default_forward: bool,
) -> CodegenNode {
    // Use single underscore prefix to avoid Python name mangling
    // Python mangles __name to _ClassName__name, which breaks dynamic lookup
    let method_name = format!("_state_{}", state_name);

    // Build context for HSM forwarding
    let ctx = HandlerContext {
        system_name: _system_name.to_string(),
        state_name: state_name.to_string(),
        event_name: String::new(), // Will be set per-handler
        parent_state: parent_state.map(|s| s.to_string()),
        has_state_vars,
        state_parents: std::collections::HashMap::new(), // TODO: populate for child state transitions
    };

    // Generate the dispatch body based on __e._message / __e.message
    let body_code = match lang {
        TargetLanguage::Python3 => generate_python_state_dispatch(_system_name, state_name, handlers, state_vars, source, &ctx, default_forward),
        TargetLanguage::TypeScript => generate_typescript_state_dispatch(_system_name, state_name, handlers, state_vars, source, &ctx, default_forward),
        TargetLanguage::Rust => generate_rust_state_dispatch(_system_name, state_name, handlers, state_vars, parent_state, default_forward),
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
    let mut has_enter_handler = handlers.contains_key("$>") || handlers.contains_key("enter");

    // If state has state variables but no explicit $> handler, generate one
    if !state_vars.is_empty() && !has_enter_handler {
        code.push_str("if __e._message == \"$>\":\n");
        for var in state_vars {
            let init_val = if let Some(ref init) = var.init {
                expression_to_string(init, TargetLanguage::Python3)
            } else {
                state_var_init_value(&var.var_type, TargetLanguage::Python3)
            };
            code.push_str(&format!("    self.__compartment.state_vars[\"{}\"] = {}\n", var.name, init_val));
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

        // For enter handlers with state vars, also initialize state vars first
        if (event == "$>" || event == "enter") && !state_vars.is_empty() {
            for var in state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::Python3)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::Python3)
                };
                code.push_str(&format!("    self.__compartment.state_vars[\"{}\"] = {}\n", var.name, init_val));
            }
        }

        // Generate parameter unpacking if handler has params
        for (i, param) in handler.params.iter().enumerate() {
            code.push_str(&format!("    {} = __e._parameters[\"{}\"]\n", param.name, i));
        }

        // Generate the handler body
        let mut handler_ctx = ctx.clone();
        handler_ctx.event_name = event.clone();
        let body = splice_handler_body_from_span(&handler.body_span, source, TargetLanguage::Python3, &handler_ctx);

        // Indent the body
        for line in body.lines() {
            if !line.trim().is_empty() {
                code.push_str("    ");
                code.push_str(line);
            }
            code.push('\n');
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

    // If state has state variables but no explicit $> handler, generate one
    if !state_vars.is_empty() && !has_enter_handler {
        code.push_str("if (__e._message === \"$>\") {\n");
        for var in state_vars {
            let init_val = if let Some(ref init) = var.init {
                expression_to_string(init, TargetLanguage::TypeScript)
            } else {
                state_var_init_value(&var.var_type, TargetLanguage::TypeScript)
            };
            code.push_str(&format!("    this.__compartment.state_vars[\"{}\"] = {};\n", var.name, init_val));
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

        // For enter handlers with state vars, also initialize state vars first
        if (event == "$>" || event == "enter") && !state_vars.is_empty() {
            for var in state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::TypeScript)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::TypeScript)
                };
                code.push_str(&format!("    this.__compartment.state_vars[\"{}\"] = {};\n", var.name, init_val));
            }
        }

        // Generate parameter unpacking if handler has params
        for (i, param) in handler.params.iter().enumerate() {
            code.push_str(&format!("    const {} = __e._parameters?.[\"{}\"];\n", param.name, i));
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
    let has_enter_handler = handlers.contains_key("$>") || handlers.contains_key("enter");
    let needs_state_var_init = !state_vars.is_empty();

    for (event, handler) in sorted_handlers {
        // Skip handlers with parameters - they can only be called from interface methods directly
        // (state dispatch doesn't have access to the parameters)
        if !handler.params.is_empty() {
            continue;
        }

        // Map Frame events to their message names
        let message = match event.as_str() {
            "$>" | "enter" => "$>",
            "$<" | "exit" => "$<",
            _ => event.as_str(),
        };

        // Determine handler method name
        let handler_method = match event.as_str() {
            "$>" | "enter" => format!("_s_{}_enter", state_name),
            "$<" | "exit" => format!("_s_{}_exit", state_name),
            _ => format!("_s_{}_{}", state_name, event),
        };

        // For $> handler with state vars, initialize them first
        if (event == "$>" || event == "enter") && needs_state_var_init {
            code.push_str(&format!("    \"{}\" => {{\n", message));
            for var in state_vars {
                let init_val = if let Some(ref init) = var.init {
                    expression_to_string(init, TargetLanguage::Rust)
                } else {
                    state_var_init_value(&var.var_type, TargetLanguage::Rust)
                };
                code.push_str(&format!("        self._sv_{} = {};\n", var.name, init_val));
            }
            code.push_str(&format!("        self.{}(__e);\n", handler_method));
            code.push_str("    }\n");
        } else {
            // Use block syntax to ignore handler return value (dispatch doesn't return)
            code.push_str(&format!("    \"{}\" => {{ self.{}(__e); }}\n", message, handler_method));
        }
    }

    // If no $> handler but state has state vars, generate a match arm for state var init
    if needs_state_var_init && !has_enter_handler {
        code.push_str("    \"$>\" => {\n");
        for var in state_vars {
            let init_val = if let Some(ref init) = var.init {
                expression_to_string(init, TargetLanguage::Rust)
            } else {
                state_var_init_value(&var.var_type, TargetLanguage::Rust)
            };
            code.push_str(&format!("        self._sv_{} = {};\n", var.name, init_val));
        }
        code.push_str("    }\n");
    }

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
    has_state_vars: bool,
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
        has_state_vars,
        state_parents: std::collections::HashMap::new(), // TODO: populate for child state transitions
    };

    // Splice the handler body: preserve native code, expand Frame segments
    let body_code = splice_handler_body_from_span(&handler.body_span, source, lang, &ctx);

    // For handlers with return types, remove trailing semicolon from the last expression
    // This is needed for Rust where the last expression is the return value
    let body_code = if handler.return_type.is_some() && matches!(lang, TargetLanguage::Rust) {
        strip_trailing_semicolon(&body_code)
    } else {
        body_code
    };

    // For TypeScript/typed languages, don't put return types on handler methods
    // The interface wrappers handle returns via _return_value pattern
    // This avoids TypeScript errors for handlers that don't explicitly return
    let method_return_type = match lang {
        TargetLanguage::TypeScript => None,
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

/// Strip trailing semicolon from the last line of code
/// Used for Rust handlers with return types where the last expression is the return value
fn strip_trailing_semicolon(code: &str) -> String {
    let mut lines: Vec<&str> = code.lines().collect();
    if lines.is_empty() {
        return code.to_string();
    }

    // Find the last non-empty line
    let last_idx = lines.iter().rposition(|l| !l.trim().is_empty());
    if let Some(idx) = last_idx {
        // Strip trailing semicolon from the last line
        let last_line = lines[idx].trim_end();
        if last_line.ends_with(';') {
            lines[idx] = &last_line[..last_line.len()-1];
        }
    }

    lines.join("\n")
}

/// Generate state handler methods (legacy - kept for reference)
///
/// Uses the splicer to preserve native code and splice in generated Frame code
#[allow(dead_code)]
fn generate_state_handlers(machine: &MachineAst, syntax: &super::backend::ClassSyntax, source: &[u8], lang: TargetLanguage) -> Vec<CodegenNode> {
    let mut methods = Vec::new();

    for state in &machine.states {
        // Main state handler
        for handler in &state.handlers {
            methods.push(generate_handler(state, handler, syntax, source, lang));
        }

        // Enter handler
        if let Some(ref enter) = state.enter {
            methods.push(generate_enter_exit_handler(
                &format!("_s_{}_enter", state.name),
                &enter.params,
                &enter.body,
                source,
                lang,
            ));
        }

        // Exit handler
        if let Some(ref exit) = state.exit {
            methods.push(generate_enter_exit_handler(
                &format!("_s_{}_exit", state.name),
                &exit.params,
                &exit.body,
                source,
                lang,
            ));
        }
    }

    methods
}

/// Generate a single handler method using the splicer
///
/// Scans the handler body to find Frame segments, generates code for them,
/// then splices the generated code back into the original native code.
fn generate_handler(state: &StateAst, handler: &HandlerAst, syntax: &super::backend::ClassSyntax, source: &[u8], lang: TargetLanguage) -> CodegenNode {
    let params: Vec<Param> = handler.params.iter().map(|p| {
        let type_str = type_to_string(&p.param_type);
        Param::new(&p.name).with_type(&type_str)
    }).collect();

    // Use splicer to combine native code + generated Frame code
    let body_code = splice_handler_body(&handler.body, source, lang);

    CodegenNode::Method {
        name: format!("_s_{}_{}", state.name, handler.event),
        params,
        return_type: None,
        body: vec![CodegenNode::NativeBlock {
            code: body_code,
            span: Some(handler.body.span.clone()),
        }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate enter/exit handler method using the splicer
fn generate_enter_exit_handler(name: &str, params: &[EventParam], body: &HandlerBody, source: &[u8], lang: TargetLanguage) -> CodegenNode {
    let body_code = splice_handler_body(body, source, lang);

    // Convert EventParams to Params
    let method_params: Vec<Param> = params.iter().map(|p| {
        let type_str = type_to_string(&p.param_type);
        Param::new(&p.name).with_type(&type_str)
    }).collect();

    CodegenNode::Method {
        name: name.to_string(),
        params: method_params,
        return_type: None,
        body: vec![CodegenNode::NativeBlock {
            code: body_code,
            span: Some(body.span.clone()),
        }],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Splice handler body: preserve native code, replace Frame segments with generated code
#[allow(dead_code)]
fn splice_handler_body(body: &HandlerBody, source: &[u8], lang: TargetLanguage) -> String {
    // Get the body bytes from source
    let body_bytes = &source[body.span.start..body.span.end];

    // Find the opening brace
    let open_brace = body_bytes.iter().position(|&b| b == b'{');
    if open_brace.is_none() {
        return String::new();
    }
    let inner_start = open_brace.unwrap() + 1;

    // Scan for Frame segments within the body
    let mut scanner = get_native_scanner(lang);
    let scan_result = match scanner.scan(body_bytes, open_brace.unwrap()) {
        Ok(r) => r,
        Err(_) => return String::new(),
    };

    // Legacy path: use empty context (no HSM forward support)
    let ctx = HandlerContext::default();

    // Generate expansions for each Frame segment
    let mut expansions = Vec::new();
    for region in &scan_result.regions {
        if let Region::FrameSegment { span, kind, indent } = region {
            let expansion = generate_frame_expansion(body_bytes, span, *kind, *indent, lang, &ctx);
            expansions.push(expansion);
        }
    }

    // Use splicer to combine native + generated Frame code
    let splicer = Splicer;
    let spliced = splicer.splice(body_bytes, &scan_result.regions, &expansions);

    // Strip only the outer braces, preserve internal whitespace structure
    let text = &spliced.text;

    // Find opening brace
    let open = text.find('{');
    // Find closing brace (last one)
    let close = text.rfind('}');

    match (open, close) {
        (Some(o), Some(c)) if o < c => {
            // Get content between braces
            let inner = &text[o + 1..c];
            // Trim only the first newline after { and trailing whitespace before }
            inner.trim_start_matches('\n').trim_end().to_string()
        }
        _ => text.to_string()
    }
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
                        "{}self._state_stack_pop()",
                        indent_str
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
                let exit_str = exit_args.map(|a| expand_state_vars_in_expr(&a, lang));
                let enter_str = enter_args.map(|a| expand_state_vars_in_expr(&a, lang));
                let state_str = state_args.map(|a| expand_state_vars_in_expr(&a, lang));

                // Get compartment class name from ctx.state_name (extract system name)
                // For now, use a generic name - it will be replaced when we have system context
                let compartment_class = format!("{}Compartment", ctx.system_name);

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
                                    .map(|(i, a)| format!("\"{}\": {}", i, a))
                                    .collect();
                                code.push_str(&format!("{}__compartment.state_args = {{{}}}\n", indent_str, entries.join(", ")));
                            }
                        }

                        // Set enter_args if present
                        // Use string keys for consistency with parameter unpacking
                        if let Some(ref enter) = enter_str {
                            code.push_str(&format!("{}__compartment.enter_args = {{str(i): v for i, v in enumerate(({},))}}\n", indent_str, enter));
                        }

                        // Call __transition
                        code.push_str(&format!("{}self.__transition(__compartment)", indent_str));
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
                                    .map(|(i, a)| format!("\"{}\": {}", i, a))
                                    .collect();
                                code.push_str(&format!("{}__compartment.state_args = {{{}}};\n", indent_str, entries.join(", ")));
                            }
                        }

                        // Set enter_args if present
                        if let Some(ref enter) = enter_str {
                            code.push_str(&format!("{}__compartment.enter_args = Object.fromEntries([{}].map((v, i) => [String(i), v]));\n", indent_str, enter));
                        }

                        // Call __transition
                        code.push_str(&format!("{}this.__transition(__compartment);", indent_str));
                        code
                    }
                    TargetLanguage::Rust => {
                        // Rust uses compartment-based transition
                        format!("{}self.__transition({}Compartment::new(\"{}\"))", indent_str, ctx.system_name, target)
                    }
                    _ => {
                        // Default: same as TypeScript
                        let mut code = String::new();
                        code.push_str(&format!("{}const __compartment = new {}Compartment(\"{}\", this.__compartment.copy());\n", indent_str, ctx.system_name, target));
                        code.push_str(&format!("{}this.__transition(__compartment);", indent_str));
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
                    // Rust: call parent handler with __e parameter
                    TargetLanguage::Rust => {
                        let parent_handler = format!("_s_{}_{}", parent, ctx.event_name);
                        format!("{}self.{}(__e)", indent_str, parent_handler)
                    }
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
            let transition_code = if has_transition {
                // Extract target state and generate transition
                let target = extract_transition_target(&segment_text);
                if !target.is_empty() {
                    match lang {
                        TargetLanguage::Python3 => format!("\n{}self._transition(\"{}\", None, None)", indent_str, target),
                        TargetLanguage::TypeScript => format!("\n{}this._transition(\"{}\", null, null);", indent_str, target),
                        TargetLanguage::Rust => format!("\n{}self.__transition({}Compartment::new(\"{}\"))", indent_str, ctx.system_name, target),
                        _ => format!("\n{}this._transition(\"{}\", null, null);", indent_str, target),
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            // Save compartment (with state vars) to stack using compartment.copy()
            let push_code = match lang {
                TargetLanguage::Python3 => format!("{}self._state_stack.append(self.__compartment.copy())", indent_str),
                TargetLanguage::TypeScript => format!("{}this._state_stack.push(this.__compartment.copy());", indent_str),
                // Rust: Use compartment-based method for state stack push
                TargetLanguage::Rust => {
                    format!("{}self._state_stack_push();", indent_str)
                }
                _ => format!("{}this._state_stack.push(this.__compartment.copy());", indent_str),
            };

            format!("{}{}", push_code, transition_code)
        }
        FrameSegmentKind::StackPop => {
            // Restore compartment from stack - preserves state vars
            // Call _exit() on current state, then restore compartment (no _enter since we're restoring)
            // Phase 14.6: No _state field for Python/TypeScript - state lives in compartment.state
            match lang {
                TargetLanguage::Python3 => format!(
                    "{}self.__compartment = self._state_stack.pop()\n{}return",
                    indent_str, indent_str
                ),
                TargetLanguage::TypeScript => format!(
                    "{}this.__compartment = this._state_stack.pop()!;\n{}return;",
                    indent_str, indent_str
                ),
                // Rust: Use compartment-based state stack pop
                TargetLanguage::Rust => {
                    format!("{}self._state_stack_pop();\n{}return;", indent_str, indent_str)
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
            match lang {
                TargetLanguage::Python3 => format!("self.__compartment.state_vars[\"{}\"]", var_name),
                TargetLanguage::TypeScript => format!("this.__compartment.state_vars[\"{}\"]", var_name),
                TargetLanguage::Rust => {
                    // For Rust, access via _sv_ fields on struct
                    format!("self._sv_{}", var_name)
                },
                _ => format!("this.__compartment.state_vars[\"{}\"]", var_name),
            }
        }
        FrameSegmentKind::SystemReturn => {
            // system.return = <expr> or return <expr>
            // return <expr> is sugar for system.return = <expr>; return (early exit)
            // system.return = just sets value without returning (for chain semantics)
            let trimmed = segment_text.trim_start();
            let is_return_sugar = trimmed.starts_with("return ");
            let expr = extract_system_return_expr(&segment_text);
            // Expand any state variable references in the expression
            let expanded_expr = expand_state_vars_in_expr(&expr, lang);

            // For Python/TypeScript with proper runtime:
            // - Set both __e._return AND self._return_value
            // - __e._return is for immediate returns within the same handler
            // - _return_value persists across enter/exit handlers during transitions
            match lang {
                TargetLanguage::Python3 => {
                    if expanded_expr.is_empty() {
                        if is_return_sugar {
                            format!("{}return", indent_str)
                        } else {
                            // system.return with no expr - just a read, shouldn't happen here
                            "".to_string()
                        }
                    } else if is_return_sugar {
                        // ^ expr: set _return_value AND __e._return, then return (early exit)
                        format!("{}self._return_value = {}\n{}__e._return = self._return_value\n{}return", indent_str, expanded_expr, indent_str, indent_str)
                    } else {
                        // system.return = expr: just set _return_value (chain semantics)
                        format!("{}self._return_value = {}", indent_str, expanded_expr)
                    }
                }
                TargetLanguage::TypeScript => {
                    if expanded_expr.is_empty() {
                        if is_return_sugar {
                            format!("{}return;", indent_str)
                        } else {
                            "".to_string()
                        }
                    } else if is_return_sugar {
                        // Set _return_value AND __e._return, then return
                        format!("{}this._return_value = {};\n{}__e._return = this._return_value;\n{}return;", indent_str, expanded_expr, indent_str, indent_str)
                    } else {
                        // system.return = expr: just set _return_value (chain semantics)
                        format!("{}this._return_value = {};", indent_str, expanded_expr)
                    }
                }
                TargetLanguage::Rust => {
                    // Rust still uses native return for now (no _return_value pattern)
                    if expanded_expr.is_empty() {
                        format!("{}return;", indent_str)
                    } else {
                        format!("{}return {};", indent_str, expanded_expr)
                    }
                }
                _ => {
                    if expanded_expr.is_empty() {
                        if is_return_sugar {
                            format!("{}return;", indent_str)
                        } else {
                            "".to_string()
                        }
                    } else if is_return_sugar {
                        format!("{}__e._return = {};\n{}return;", indent_str, expanded_expr, indent_str)
                    } else {
                        format!("{}__e._return = {};", indent_str, expanded_expr)
                    }
                }
            }
        }
        FrameSegmentKind::SystemReturnExpr => {
            // bare system.return - read current return value
            // For Python/TypeScript with proper runtime: Use __e._return
            match lang {
                TargetLanguage::Python3 => "__e._return".to_string(),
                TargetLanguage::TypeScript => "__e._return".to_string(),
                TargetLanguage::Rust => "self._return_value".to_string(),
                _ => "__e._return".to_string(),
            }
        }
    }
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

/// Extract expression from system.return = <expr> or return <expr>
fn extract_system_return_expr(text: &str) -> String {
    let text = text.trim();
    // Handle return sugar: return <expr>
    if text.starts_with("return ") {
        let after_return = text[7..].trim();
        return after_return.to_string();
    }
    // Handle system.return = <expr>
    if text.starts_with("system.return") {
        let after = &text[13..];
        let trimmed = after.trim();
        if trimmed.starts_with('=') {
            return trimmed[1..].trim().to_string();
        }
    }
    String::new()
}

/// Expand state variable references ($.varName) in an expression string
/// Uses compartment.state_vars for Python/TypeScript
fn expand_state_vars_in_expr(expr: &str, lang: TargetLanguage) -> String {
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
                TargetLanguage::Python3 => result.push_str(&format!("self.__compartment.state_vars[\"{}\"]", var_name)),
                TargetLanguage::TypeScript => result.push_str(&format!("this.__compartment.state_vars[\"{}\"]", var_name)),
                TargetLanguage::Rust => result.push_str(&format!("self._sv_{}", var_name)),
                _ => result.push_str(&format!("this.__compartment.state_vars[\"{}\"]", var_name)),
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
fn generate_action(action: &ActionAst, syntax: &super::backend::ClassSyntax, source: &[u8]) -> CodegenNode {
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
fn generate_operation(operation: &OperationAst, syntax: &super::backend::ClassSyntax, source: &[u8]) -> CodegenNode {
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
            // Phase 14.6: Serialize compartment structure (no _state field, state lives in compartment)
            let mut save_body = String::new();
            save_body.push_str("return {\n");
            save_body.push_str("    _compartment: this.__compartment.copy(),\n");
            // Stack stores compartment objects
            save_body.push_str("    _state_stack: this._state_stack.map(c => c.copy()),\n");

            // Add domain variables
            for var in &system.domain {
                save_body.push_str(&format!("    {}: this.{},\n", var.name, var.name));
            }

            save_body.push_str("};");

            methods.push(CodegenNode::Method {
                name: "saveState".to_string(),
                params: vec![],
                return_type: Some("any".to_string()),  // 'any' allows property access
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
            // Phase 14.6: Restore compartment structure (no _state field, state lives in compartment)
            let mut restore_body = String::new();
            restore_body.push_str(&format!("const instance = Object.create({}.prototype);\n", system.name));
            // Restore compartment - create fresh instance and copy properties (state is in compartment.state)
            restore_body.push_str(&format!("instance.__compartment = new {}Compartment(data._compartment.state);\n", system.name));
            restore_body.push_str("instance.__compartment.state_args = {...(data._compartment.state_args || {})};\n");
            restore_body.push_str("instance.__compartment.state_vars = {...(data._compartment.state_vars || {})};\n");
            restore_body.push_str("instance.__compartment.enter_args = {...(data._compartment.enter_args || {})};\n");
            restore_body.push_str("instance.__compartment.exit_args = {...(data._compartment.exit_args || {})};\n");
            restore_body.push_str("instance.__compartment.forward_event = data._compartment.forward_event;\n");
            restore_body.push_str("instance.__next_compartment = null;\n");
            // Restore stack - each element is a serialized compartment
            restore_body.push_str(&format!("instance._state_stack = (data._state_stack || []).map((c: any) => {{\n"));
            restore_body.push_str(&format!("    const comp = new {}Compartment(c.state);\n", system.name));
            restore_body.push_str("    comp.state_args = {...(c.state_args || {})};\n");
            restore_body.push_str("    comp.state_vars = {...(c.state_vars || {})};\n");
            restore_body.push_str("    comp.enter_args = {...(c.enter_args || {})};\n");
            restore_body.push_str("    comp.exit_args = {...(c.exit_args || {})};\n");
            restore_body.push_str("    comp.forward_event = c.forward_event;\n");
            restore_body.push_str("    return comp;\n");
            restore_body.push_str("});\n");
            restore_body.push_str("instance._return_value = null;\n");

            // Restore domain variables
            for var in &system.domain {
                restore_body.push_str(&format!("instance.{} = data.{};\n", var.name, var.name));
            }

            restore_body.push_str("return instance;");

            methods.push(CodegenNode::Method {
                name: "restoreState".to_string(),
                params: vec![Param::new("data").with_type("any")],
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
            // Rust uses serde by default (requires serde, serde_json in Cargo.toml)
            // Project owner is responsible for adding dependencies

            {
                // Generate save_state that manually builds JSON
                // Use compartment-based architecture
                let mut save_body = String::new();
                save_body.push_str("let stack_states: Vec<&str> = self._state_stack.iter().map(|(s, _)| s.as_str()).collect();\n");
                save_body.push_str("serde_json::json!({\n");
                save_body.push_str("    \"_state\": self.__compartment.state,\n");
                save_body.push_str("    \"_state_stack\": stack_states,\n");

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

                // Generate restore_state that parses JSON and creates new instance
                let mut restore_body = String::new();
                restore_body.push_str("let data: serde_json::Value = serde_json::from_str(json).unwrap();\n");

                // Restore stack as Vec<(String, StateContext)>
                restore_body.push_str(&format!("let stack: Vec<(String, {}StateContext)> = data[\"_state_stack\"].as_array()\n", system.name));
                restore_body.push_str("    .map(|arr| arr.iter()\n");
                restore_body.push_str(&format!("        .filter_map(|v| v.as_str().map(|s| (s.to_string(), {}StateContext::Empty)))\n", system.name));
                restore_body.push_str("        .collect())\n");
                restore_body.push_str("    .unwrap_or_default();\n");

                restore_body.push_str(&format!("let mut instance = {} {{\n", system.name));
                restore_body.push_str("    _state_stack: stack,\n");
                restore_body.push_str(&format!("    __compartment: {}Compartment::new(data[\"_state\"].as_str().unwrap()),\n", system.name));
                restore_body.push_str("    __next_compartment: None,\n");

                // Restore domain variables
                for var in &system.domain {
                    let _type_str = type_to_string(&var.var_type);
                    let json_extract = match &var.var_type {
                        Type::Int => format!("data[\"{}\"].as_i64().unwrap() as i32", var.name),
                        Type::Float => format!("data[\"{}\"].as_f64().unwrap()", var.name),
                        Type::Bool => format!("data[\"{}\"].as_bool().unwrap()", var.name),
                        Type::String => format!("data[\"{}\"].as_str().unwrap().to_string()", var.name),
                        Type::Custom(name) if name == "i64" => format!("data[\"{}\"].as_i64().unwrap()", var.name),
                        Type::Custom(name) if name == "f64" => format!("data[\"{}\"].as_f64().unwrap()", var.name),
                        Type::Custom(name) if name == "String" => format!("data[\"{}\"].as_str().unwrap().to_string()", var.name),
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
        _ => {
            // Other languages not yet supported
        }
    }

    methods
}

/// Convert Frame AST statements to CodegenNode
fn convert_statements(stmts: &[Statement]) -> Vec<CodegenNode> {
    stmts.iter().map(convert_statement).collect()
}

/// Convert a single Frame AST statement to CodegenNode
fn convert_statement(stmt: &Statement) -> CodegenNode {
    match stmt {
        Statement::Transition(trans) => {
            CodegenNode::Transition {
                target_state: trans.target.clone(),
                exit_args: vec![],  // TODO: parse from args
                enter_args: trans.args.iter().map(convert_expression).collect(),
                state_args: vec![],
                indent: trans.indent,
            }
        }
        Statement::Forward(forward) => {
            // Check if forwarding to parent (event == "^")
            let to_parent = forward.event == "^";
            CodegenNode::Forward { to_parent, indent: forward.indent }
        }
        Statement::TransitionForward(tf) => {
            // Transition-forward: transition to state then dispatch event
            // The actual expansion happens in generate_frame_expansion
            // This converts to a transition followed by forward
            CodegenNode::Transition {
                target_state: tf.target.clone(),
                exit_args: vec![],
                enter_args: vec![],
                state_args: vec![],
                indent: tf.indent,
            }
        }
        Statement::StackPush(push) => {
            CodegenNode::StackPush { indent: push.indent }
        }
        Statement::StackPop(pop) => {
            CodegenNode::StackPop { indent: pop.indent }
        }
        Statement::Return(ret) => {
            CodegenNode::ret(ret.value.as_ref().map(convert_expression))
        }
        Statement::Continue(_) => {
            CodegenNode::Continue
        }
        // Note: Statement::Native no longer exists - native code is handled by splicer
        Statement::If(if_ast) => {
            let then_block = vec![convert_statement(&if_ast.then_branch)];
            let else_block = if_ast.else_branch.as_ref()
                .map(|e| vec![convert_statement(e)]);
            CodegenNode::If {
                condition: Box::new(convert_expression(&if_ast.condition)),
                then_block,
                else_block,
            }
        }
        Statement::Loop(loop_ast) => {
            match &loop_ast.kind {
                LoopKind::While(cond) => {
                    CodegenNode::While {
                        condition: Box::new(convert_expression(cond)),
                        body: vec![convert_statement(&loop_ast.body)],
                    }
                }
                LoopKind::For(var, iterable) => {
                    CodegenNode::For {
                        var: var.clone(),
                        iterable: Box::new(convert_expression(iterable)),
                        body: vec![convert_statement(&loop_ast.body)],
                    }
                }
                LoopKind::Loop => {
                    // Loop forever with true condition
                    CodegenNode::While {
                        condition: Box::new(CodegenNode::bool(true)),
                        body: vec![convert_statement(&loop_ast.body)],
                    }
                }
            }
        }
        Statement::Expression(expr_ast) => {
            CodegenNode::ExprStmt(Box::new(convert_expression(&expr_ast.expr)))
        }
    }
}

/// Convert Type enum to string representation
fn type_to_string(t: &Type) -> String {
    match t {
        Type::Int => "int".to_string(),
        Type::Float => "float".to_string(),
        Type::String => "str".to_string(),
        Type::Bool => "bool".to_string(),
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
            var_type: Type::Int,
            initializer: Some(Expression::Literal(Literal::Int(0))),
            is_frame: false,
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

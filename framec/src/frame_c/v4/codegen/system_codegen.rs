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
    Expression, Literal, BinaryOp, UnaryOp,
};
use crate::frame_c::v4::arcanum::{Arcanum, HandlerEntry};
use crate::frame_c::v4::splice::SplicerV3;
use crate::frame_c::v4::native_region_scanner::{
    NativeRegionScannerV3, RegionV3, FrameSegmentKindV3,
    python::NativeRegionScannerPyV3,
    typescript::NativeRegionScannerTsV3,
    rust::NativeRegionScannerRustV3,
    csharp::NativeRegionScannerCsV3,
    c::NativeRegionScannerCV3,
    cpp::NativeRegionScannerCppV3,
    java::NativeRegionScannerJavaV3,
};
use super::ast::*;
use super::backend::get_backend;

/// Context for handler expansion - tracks parent state and event for HSM forwarding
#[derive(Clone, Default)]
struct HandlerContext {
    pub state_name: String,
    pub event_name: String,
    pub parent_state: Option<String>,
    /// True if the system has states with state variables (for Rust compartment-based push/pop)
    pub has_state_vars: bool,
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
    if system.machine.is_some() {
        methods.extend(generate_state_handlers_via_arcanum(&system.name, arcanum, source, lang, has_state_vars));
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

    // State field - stores current state name as string
    fields.push(Field::new("_state")
        .with_visibility(Visibility::Private)
        .with_type("str"));

    // State stack - for push/pop state operations
    // For Rust with state vars: Vec<(String, {System}Compartment)> - fully typed
    // For others: list/array
    let has_state_vars = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| !s.state_vars.is_empty()))
        .unwrap_or(false);

    let stack_type = if matches!(syntax.language, TargetLanguage::Rust) && has_state_vars {
        format!("Vec<(String, {}Compartment)>", system.name)
    } else {
        "List".to_string()
    };
    fields.push(Field::new("_state_stack")
        .with_visibility(Visibility::Private)
        .with_type(&stack_type));

    // State context (for state parameters) - ONLY for Rust which doesn't use compartment model
    // Python/TypeScript use _compartment.state_vars instead (Phase 14.6)
    if matches!(syntax.language, TargetLanguage::Rust) {
        fields.push(Field::new("_state_context")
            .with_visibility(Visibility::Private)
            .with_type("Dict"));
    }

    // Compartment field - for Python/TypeScript canonical compartment architecture
    // Holds the current compartment with all 6 canonical fields
    // Not used by Rust (which uses the enum-of-structs pattern)
    if !matches!(syntax.language, TargetLanguage::Rust) {
        let compartment_type = format!("{}Compartment", system.name);
        fields.push(Field::new("_compartment")
            .with_visibility(Visibility::Private)
            .with_type(&compartment_type));
    }

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

/// Generate Rust compartment types for a system
///
/// This is the public entry point for generating the compartment enum and context structs
/// that are needed for type-safe state variable storage in Rust.
///
/// Returns the Rust code for:
/// - Context structs for each state that has state variables
/// - A compartment enum with variants for each state
/// - Default impl for the enum
pub fn generate_rust_compartment_types(system: &SystemAst) -> String {
    generate_rust_compartment_enum(system)
}

/// Generate Compartment class for Python/TypeScript
///
/// The Compartment class encapsulates all state-related data following the canonical 6-field model:
/// - state: string - Current state identifier
/// - state_args: dict - State parameters ($State(args))
/// - state_vars: dict - State variables ($.varName)
/// - enter_args: dict - Enter transition args (-> (args) $State)
/// - exit_args: dict - Exit transition args ((args) -> $State)
/// - forward_event: Event? - For event forwarding (-> =>)
///
/// Returns None for Rust (which uses the specialized enum-of-structs pattern)
pub fn generate_compartment_class(system: &SystemAst, lang: TargetLanguage) -> Option<CodegenNode> {
    // Rust uses a different pattern - return None
    if matches!(lang, TargetLanguage::Rust) {
        return None;
    }

    let class_name = format!("{}Compartment", system.name);

    // Constructor parameters: just state
    let constructor_params = vec![
        Param::new("state").with_type("str"),
    ];

    // Constructor body: initialize all 6 fields
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
            // Python: c = {Class}Compartment(self.state); c.state_args = self.state_args.copy(); ...
            vec![CodegenNode::NativeBlock {
                code: format!(
                    r#"c = {}(self.state)
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
            // TypeScript: const c = new {Class}(this.state); c.state_args = {...this.state_args}; ...
            vec![CodegenNode::NativeBlock {
                code: format!(
                    r#"const c = new {}(this.state);
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

/// Generate Rust compartment enum and context structs
///
/// Generates an enum-of-structs pattern for type-safe state variable storage:
/// ```rust
/// #[derive(Clone)]
/// enum FooCompartment {
///     Counter(CounterContext),
///     Other(OtherContext),
///     Empty,
/// }
/// struct CounterContext { count: i32 }
/// struct OtherContext { other_count: i32 }
/// ```
fn generate_rust_compartment_enum(system: &SystemAst) -> String {
    // Only generate compartment types if there are states with variables
    let has_state_vars = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| !s.state_vars.is_empty()))
        .unwrap_or(false);

    if !has_state_vars {
        return String::new();
    }

    let mut code = String::new();
    let system_name = &system.name;

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

    // Generate compartment enum
    code.push_str(&format!("#[derive(Clone)]\nenum {}Compartment {{\n", system_name));

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            if state.state_vars.is_empty() {
                // State without vars - unit variant
                code.push_str(&format!("    {},\n", state.name));
            } else {
                // State with vars - tuple variant holding context
                code.push_str(&format!("    {}({}Context),\n", state.name, state.name));
            }
        }
    }

    // Always add Empty variant for edge cases
    code.push_str("    Empty,\n");
    code.push_str("}\n\n");

    // Generate Default impl for the enum (returns first state's variant)
    if let Some(ref machine) = system.machine {
        if let Some(first_state) = machine.states.first() {
            code.push_str(&format!("impl Default for {}Compartment {{\n", system_name));
            code.push_str("    fn default() -> Self {\n");
            if first_state.state_vars.is_empty() {
                code.push_str(&format!("        {}Compartment::{}\n", system_name, first_state.name));
            } else {
                code.push_str(&format!("        {}Compartment::{}({}Context::default())\n",
                    system_name, first_state.name, first_state.name));
            }
            code.push_str("    }\n");
            code.push_str("}\n\n");
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

    // Initialize state context - ONLY for Rust which doesn't use compartment model
    // Python/TypeScript use _compartment.state_vars instead (Phase 14.6)
    if matches!(syntax.language, TargetLanguage::Rust) {
        body.push(CodegenNode::assign(
            CodegenNode::field(CodegenNode::self_ref(), "_state_context"),
            CodegenNode::Dict(vec![]),
        ));
    }

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

    // Set initial state as string (first state in machine)
    if let Some(ref machine) = system.machine {
        if let Some(first_state) = machine.states.first() {
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_state"),
                CodegenNode::string(&first_state.name),
            ));

            // Initialize compartment with initial state (for Python/TypeScript)
            // The compartment starts with the initial state name
            if !matches!(syntax.language, TargetLanguage::Rust) {
                let compartment_class = format!("{}Compartment", system.name);
                body.push(CodegenNode::assign(
                    CodegenNode::field(CodegenNode::self_ref(), "_compartment"),
                    CodegenNode::New {
                        class: compartment_class,
                        args: vec![CodegenNode::string(&first_state.name)],
                    },
                ));
            }

            // Call enter handler on initial state
            body.push(CodegenNode::ExprStmt(Box::new(
                CodegenNode::method_call(
                    CodegenNode::self_ref(),
                    "_enter",
                    vec![],
                ),
            )));
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

/// Generate Frame machinery methods (_transition, _change_state, _dispatch_event, etc.)
fn generate_frame_machinery(system: &SystemAst, syntax: &super::backend::ClassSyntax, lang: TargetLanguage) -> Vec<CodegenNode> {
    let mut methods = Vec::new();

    // _transition method - takes state name as string
    // Language-specific parameter types
    let transition_params = match lang {
        TargetLanguage::Rust => vec![
            Param::new("target_state").with_type("&str"),
        ],
        TargetLanguage::TypeScript => vec![
            Param::new("target_state").with_type("string"),
            Param::new("exit_args").with_type("any").with_default(CodegenNode::null()),
            Param::new("enter_args").with_type("any").with_default(CodegenNode::null()),
        ],
        _ => vec![
            Param::new("target_state"),
            Param::new("exit_args").with_default(CodegenNode::null()),
            Param::new("enter_args").with_default(CodegenNode::null()),
        ],
    };
    // Language-specific state assignment (Rust needs .to_string())
    let state_value = match lang {
        TargetLanguage::Rust => CodegenNode::method_call(
            CodegenNode::ident("target_state"),
            "to_string",
            vec![],
        ),
        _ => CodegenNode::ident("target_state"),
    };
    // Language-specific _transition body that passes args to _exit and _enter
    // For Python/TypeScript: create new compartment and sync legacy _state field
    let compartment_class = format!("{}Compartment", system.name);
    let transition_body = match lang {
        TargetLanguage::Python3 => {
            vec![CodegenNode::NativeBlock {
                code: format!(
                    r#"if exit_args:
    self._exit(*exit_args)
else:
    self._exit()
self._compartment = {}(target_state)
self._state = target_state
if enter_args:
    self._enter(*enter_args)
else:
    self._enter()"#,
                    compartment_class
                ),
                span: None,
            }]
        }
        TargetLanguage::TypeScript => {
            vec![CodegenNode::NativeBlock {
                code: format!(
                    r#"if (exit_args) {{
    this._exit(...exit_args);
}} else {{
    this._exit();
}}
this._compartment = new {}(target_state);
this._state = target_state;
if (enter_args) {{
    this._enter(...enter_args);
}} else {{
    this._enter();
}}"#,
                    compartment_class
                ),
                span: None,
            }]
        }
        TargetLanguage::Rust => {
            // Rust doesn't support variable args yet, use simple dispatch
            vec![
                CodegenNode::ExprStmt(Box::new(
                    CodegenNode::method_call(CodegenNode::self_ref(), "_exit", vec![]),
                )),
                CodegenNode::assign(
                    CodegenNode::field(CodegenNode::self_ref(), "_state"),
                    state_value.clone(),
                ),
                CodegenNode::ExprStmt(Box::new(
                    CodegenNode::method_call(CodegenNode::self_ref(), "_enter", vec![]),
                )),
            ]
        }
        _ => {
            vec![CodegenNode::NativeBlock {
                code: format!(
                    r#"if (exit_args) {{
    this._exit(...exit_args);
}} else {{
    this._exit();
}}
this._compartment = new {}(target_state);
this._state = target_state;
if (enter_args) {{
    this._enter(...enter_args);
}} else {{
    this._enter();
}}"#,
                    compartment_class
                ),
                span: None,
            }]
        }
    };

    methods.push(CodegenNode::Method {
        name: "_transition".to_string(),
        params: transition_params,
        return_type: None,
        body: transition_body,
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    });

    // NOTE: _change_state method removed - it was deprecated V3 legacy (->> operator)
    // The ->> syntax should not compile in V4

    // _dispatch_event method - routes events to current state's handler
    // Language-specific dynamic dispatch implementation
    let (dispatch_params, dispatch_body) = match lang {
        TargetLanguage::Python3 => {
            (
                vec![Param::new("event"), Param::new("*args")],
                vec![CodegenNode::NativeBlock {
                    code: "handler_name = f\"_s_{self._state}_{event}\"\nhandler = getattr(self, handler_name, None)\nif handler:\n    return handler(*args)".to_string(),
                    span: None,
                }],
            )
        }
        TargetLanguage::TypeScript => {
            (
                vec![
                    Param::new("event").with_type("string"),
                    Param::new("...args").with_type("any[]"),
                ],
                vec![CodegenNode::NativeBlock {
                    code: "const handler_name = `_s_${this._state}_${event}`;\nconst handler = (this as any)[handler_name];\nif (handler) {\n    return handler.apply(this, args);\n}".to_string(),
                    span: None,
                }],
            )
        }
        TargetLanguage::Rust => {
            (
                vec![Param::new("event").with_type("&str")],
                vec![CodegenNode::NativeBlock {
                    code: "let handler_name = format!(\"_s_{}_{}\", self._state, event);\n// Rust requires match-based dispatch or a handler registry\n// For now, use explicit match in caller".to_string(),
                    span: None,
                }],
            )
        }
        _ => {
            // Default fallback for other languages
            (
                vec![Param::new("event"), Param::new("args")],
                vec![CodegenNode::comment("Dispatch implementation needed for this language")],
            )
        }
    };

    methods.push(CodegenNode::Method {
        name: "_dispatch_event".to_string(),
        params: dispatch_params,
        return_type: None,
        body: dispatch_body,
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    });

    // _enter and _exit dispatchers
    methods.push(generate_enter_dispatcher(system, lang));
    methods.push(generate_exit_dispatcher(system, lang));

    // For Rust: Generate _state_stack_push() and _state_stack_pop() methods
    // These handle typed compartment save/restore for state variable preservation
    // Only generate when there are states with state variables
    let has_state_vars = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| !s.state_vars.is_empty()))
        .unwrap_or(false);
    if matches!(lang, TargetLanguage::Rust) && has_state_vars {
        methods.push(generate_rust_state_stack_push(system));
        methods.push(generate_rust_state_stack_pop(system));
    }

    methods
}

/// Generate Rust _state_stack_push method
/// Builds a compartment from current _sv_ fields and pushes to stack
fn generate_rust_state_stack_push(system: &SystemAst) -> CodegenNode {
    let system_name = &system.name;
    let mut code = String::new();

    // Build compartment from current _sv_ fields based on current state
    code.push_str("let compartment = match self._state.as_str() {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            if state.state_vars.is_empty() {
                // State without vars - unit variant
                code.push_str(&format!(
                    "    \"{}\" => {}Compartment::{},\n",
                    state.name, system_name, state.name
                ));
            } else {
                // State with vars - build context struct from _sv_ fields
                let field_inits: Vec<String> = state.state_vars.iter()
                    .map(|v| format!("{}: self._sv_{}", v.name, v.name))
                    .collect();
                code.push_str(&format!(
                    "    \"{}\" => {}Compartment::{}({}Context {{ {} }}),\n",
                    state.name, system_name, state.name, state.name, field_inits.join(", ")
                ));
            }
        }
    }

    code.push_str(&format!("    _ => {}Compartment::Empty,\n", system_name));
    code.push_str("};\n");
    code.push_str("self._state_stack.push((self._state.clone(), compartment));");

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
/// Pops from stack, calls exit, restores _sv_ fields from compartment
fn generate_rust_state_stack_pop(system: &SystemAst) -> CodegenNode {
    let system_name = &system.name;
    let mut code = String::new();

    // Pop the saved state and compartment
    code.push_str("let (saved_state, compartment) = self._state_stack.pop().unwrap();\n");
    code.push_str("self._exit();\n");
    code.push_str("self._state = saved_state;\n");

    // Restore _sv_ fields from compartment
    code.push_str("match compartment {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            if state.state_vars.is_empty() {
                // State without vars - nothing to restore
                code.push_str(&format!(
                    "    {}Compartment::{} => {{}}\n",
                    system_name, state.name
                ));
            } else {
                // State with vars - restore from context
                let field_names: Vec<&str> = state.state_vars.iter()
                    .map(|v| v.name.as_str())
                    .collect();
                code.push_str(&format!(
                    "    {}Compartment::{}(ctx) => {{\n",
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

    code.push_str(&format!("    {}Compartment::Empty => {{}}\n", system_name));
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
                    "handler_name = f\"_s_{self._state}_enter\"\nhandler = getattr(self, handler_name, None)\nif handler:\n    handler(*args)"
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
                    "const handler_name = `_s_${this._state}_enter`;\nconst handler = (this as any)[handler_name];\nif (handler) {\n    handler.call(this, ...args);\n}"
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
                    code: "handler_name = f\"_s_{self._state}_exit\"\nhandler = getattr(self, handler_name, None)\nif handler:\n    handler(*args)".to_string(),
                    span: None,
                }]
            }
            TargetLanguage::TypeScript => {
                vec![CodegenNode::NativeBlock {
                    code: "const handler_name = `_s_${this._state}_exit`;\nconst handler = (this as any)[handler_name];\nif (handler) {\n    handler.call(this, ...args);\n}".to_string(),
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

            let condition = if first {
                format!("if {}._state == \"{}\":", self_ref, state.name)
            } else {
                format!("elif {}._state == \"{}\":", self_ref, state.name)
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
                        format!("if ({}._state === \"{}\") {{\n", self_ref, state.name)
                    } else {
                        format!("}} else if ({}._state === \"{}\") {{\n", self_ref, state.name)
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
fn generate_interface_wrappers(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> Vec<CodegenNode> {
    // Get the target language from the syntax
    let lang = syntax.language;

    system.interface.iter().map(|method| {
        let params: Vec<Param> = method.params.iter().map(|p| {
            let type_str = type_to_string(&p.param_type);
            Param::new(&p.name).with_type(&type_str)
        }).collect();

        let args: Vec<CodegenNode> = method.params.iter()
            .map(|p| CodegenNode::ident(&p.name))
            .collect();

        // Language-specific dispatch
        let body_stmt = match lang {
            TargetLanguage::Rust => {
                // For Rust, generate a match-based dispatch directly in the interface method
                generate_rust_interface_dispatch(system, &method.name, &args, method.return_type.is_some())
            }
            _ => {
                // For dynamic languages, use _dispatch_event with _return_value pattern
                let mut dispatch_args = vec![CodegenNode::string(&method.name)];
                dispatch_args.extend(args);

                let dispatch_call = CodegenNode::method_call(
                    CodegenNode::self_ref(),
                    "_dispatch_event",
                    dispatch_args,
                );

                // If method has return type, use _return_value pattern for chain semantics
                if method.return_type.is_some() {
                    // Generate: self._return_value = None; self._dispatch_event(...); return self._return_value
                    let init_return = match lang {
                        TargetLanguage::Python3 => "self._return_value = None",
                        TargetLanguage::TypeScript => "this._return_value = null",
                        _ => "this._return_value = null",
                    };
                    let return_expr = match lang {
                        TargetLanguage::Python3 => "self._return_value",
                        TargetLanguage::TypeScript => "this._return_value",
                        _ => "this._return_value",
                    };
                    CodegenNode::NativeBlock {
                        code: format!("{}\n{}\nreturn {}",
                            init_return,
                            match lang {
                                TargetLanguage::Python3 => format!("self._dispatch_event(\"{}\"{})",
                                    method.name,
                                    if method.params.is_empty() { "".to_string() }
                                    else { format!(", {}", method.params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ")) }
                                ),
                                _ => format!("this._dispatch_event(\"{}\"{})",
                                    method.name,
                                    if method.params.is_empty() { "".to_string() }
                                    else { format!(", {}", method.params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ")) }
                                ),
                            },
                            return_expr
                        ),
                        span: None,
                    }
                } else {
                    CodegenNode::ExprStmt(Box::new(dispatch_call))
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

/// Generate Rust match-based dispatch for interface methods
fn generate_rust_interface_dispatch(system: &SystemAst, event: &str, args: &[CodegenNode], has_return: bool) -> CodegenNode {
    // Build match arms for each state that handles this event
    let mut match_code = String::new();
    match_code.push_str("match self._state.as_str() {\n");

    if let Some(ref machine) = system.machine {
        for state in &machine.states {
            // Check if this state handles the event
            let handles_event = state.handlers.iter().any(|h| h.event == event);
            if handles_event {
                let handler_name = format!("_s_{}_{}", state.name, event);
                let args_str = if args.is_empty() {
                    String::new()
                } else {
                    args.iter().map(|a| {
                        // Extract identifier name
                        if let CodegenNode::Ident(name) = a {
                            name.clone()
                        } else {
                            "arg".to_string()
                        }
                    }).collect::<Vec<_>>().join(", ")
                };

                if has_return {
                    match_code.push_str(&format!("            \"{}\" => self.{}({}),\n", state.name, handler_name, args_str));
                } else {
                    match_code.push_str(&format!("            \"{}\" => {{ self.{}({}); }}\n", state.name, handler_name, args_str));
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

    CodegenNode::NativeBlock {
        code: match_code,
        span: None,
    }
}

/// Generate state handler methods using the enhanced Arcanum
///
/// This is the preferred method - uses the Arcanum's handler tracking for clean iteration.
/// The Arcanum was populated from the AST, so this is functionally equivalent but cleaner.
fn generate_state_handlers_via_arcanum(system_name: &str, arcanum: &Arcanum, source: &[u8], lang: TargetLanguage, has_state_vars: bool) -> Vec<CodegenNode> {
    let mut methods = Vec::new();

    // Iterate over all enhanced states in the system
    for state_entry in arcanum.get_enhanced_states(system_name) {
        // Generate handler methods for each handler in the state
        for (event, handler_entry) in &state_entry.handlers {
            let method = generate_handler_from_arcanum(
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

    methods
}

/// Generate a handler method from Arcanum's HandlerEntry
///
/// Uses the handler's body_span to extract and splice native code with Frame expansions.
fn generate_handler_from_arcanum(
    state_name: &str,
    parent_state: Option<&str>,
    handler: &HandlerEntry,
    source: &[u8],
    lang: TargetLanguage,
    has_state_vars: bool,
) -> CodegenNode {
    // Build params from handler's parameter symbols
    // V4 uses native types, so we just pass them through as-is
    let params: Vec<Param> = handler.params.iter().map(|p| {
        let type_str = p.symbol_type.as_deref().unwrap_or("Any");
        // Clean up the type string (remove "Some(" prefix if present from debug format)
        let clean_type = if type_str.starts_with("Some(") {
            type_str.trim_start_matches("Some(").trim_end_matches(")")
        } else {
            type_str
        };
        Param::new(&p.name).with_type(clean_type)
    }).collect();

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
        state_name: state_name.to_string(),
        event_name: handler.event.clone(),
        parent_state: parent_state.map(|s| s.to_string()),
        has_state_vars,
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
        if let RegionV3::FrameSegment { span, kind, indent } = region {
            let expansion = generate_frame_expansion(body_bytes, span, *kind, *indent, lang, ctx);
            expansions.push(expansion);
        }
    }

    // Use splicer to combine native + generated Frame code
    let splicer = SplicerV3;
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
        if let RegionV3::FrameSegment { span, kind, indent } = region {
            let expansion = generate_frame_expansion(body_bytes, span, *kind, *indent, lang, &ctx);
            expansions.push(expansion);
        }
    }

    // Use splicer to combine native + generated Frame code
    let splicer = SplicerV3;
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
fn generate_frame_expansion(body_bytes: &[u8], span: &crate::frame_c::v4::native_region_scanner::RegionSpan, kind: FrameSegmentKindV3, indent: usize, lang: TargetLanguage, ctx: &HandlerContext) -> String {
    let segment_text = String::from_utf8_lossy(&body_bytes[span.start..span.end]);
    // Use scanner's indent value to match native code indentation
    // This ensures Frame expansions align with surrounding native code
    let indent_str = " ".repeat(indent);

    match kind {
        FrameSegmentKindV3::Transition => {
            // Parse transition: (exit_args)? -> (enter_args)? $State(state_args)?
            // Check for pop-transition: -> pop$
            if segment_text.contains("pop$") {
                // Pop-transition: pop state from stack and transition to it
                match lang {
                    TargetLanguage::Python3 => format!(
                        "{}__saved = self._state_stack.pop()\n{}self._transition(__saved[0], None, None)",
                        indent_str, indent_str
                    ),
                    TargetLanguage::TypeScript => format!(
                        "{}const __saved = this._state_stack.pop()!;\n{}this._transition(__saved.state, null, null);",
                        indent_str, indent_str
                    ),
                    TargetLanguage::Rust => format!(
                        "{}let __saved = self._state_stack.pop().unwrap();\n{}let __target = __saved.downcast_ref::<String>().unwrap();\n{}self._transition(__target)",
                        indent_str, indent_str, indent_str
                    ),
                    _ => format!(
                        "{}const __saved = this._state_stack.pop();\n{}this._transition(__saved.state, null, null);",
                        indent_str, indent_str
                    ),
                }
            } else {
                let target = extract_transition_target(&segment_text);
                let (exit_args, enter_args) = extract_transition_args(&segment_text);

                // Expand state variable references in arguments
                let exit_str = exit_args.map(|a| expand_state_vars_in_expr(&a, lang));
                let enter_str = enter_args.map(|a| expand_state_vars_in_expr(&a, lang));

                match lang {
                    TargetLanguage::Python3 => {
                        // Use trailing comma to ensure tuple even with single arg: (arg,)
                        let exit_param = exit_str.as_ref().map_or("None".to_string(), |s| format!("({},)", s));
                        let enter_param = enter_str.as_ref().map_or("None".to_string(), |s| format!("({},)", s));
                        format!("{}self._transition(\"{}\", {}, {})", indent_str, target, exit_param, enter_param)
                    }
                    TargetLanguage::TypeScript => {
                        let exit_param = exit_str.as_ref().map_or("null".to_string(), |s| format!("[{}]", s));
                        let enter_param = enter_str.as_ref().map_or("null".to_string(), |s| format!("[{}]", s));
                        format!("{}this._transition(\"{}\", {}, {});", indent_str, target, exit_param, enter_param)
                    }
                    TargetLanguage::Rust => {
                        // Rust currently doesn't support enter/exit args in _transition
                        format!("{}self._transition(\"{}\")", indent_str, target)
                    }
                    _ => {
                        let exit_param = exit_str.as_ref().map_or("null".to_string(), |s| format!("[{}]", s));
                        let enter_param = enter_str.as_ref().map_or("null".to_string(), |s| format!("[{}]", s));
                        format!("{}this._transition(\"{}\", {}, {});", indent_str, target, exit_param, enter_param)
                    }
                }
            }
        }
        FrameSegmentKindV3::TransitionForward => {
            // Transition-Forward: -> => $State
            // 1. Transition to the target state (exit current, enter new)
            // 2. Dispatch the current event to the new state's handler
            // 3. Return (event was handled by new state)
            let target = extract_transition_target(&segment_text);
            match lang {
                TargetLanguage::Python3 => format!(
                    "{}self._transition(\"{}\", None, None)\n{}return self._dispatch_event(\"{}\")",
                    indent_str, target, indent_str, ctx.event_name
                ),
                TargetLanguage::TypeScript => format!(
                    "{}this._transition(\"{}\", null, null);\n{}return this._dispatch_event(\"{}\");",
                    indent_str, target, indent_str, ctx.event_name
                ),
                TargetLanguage::Rust => {
                    // For Rust, call the target state's handler directly since _dispatch_event is a stub
                    let handler_name = format!("_s_{}_{}", target, ctx.event_name);
                    format!(
                        "{}self._transition(\"{}\");\n{}return self.{}();",
                        indent_str, target, indent_str, handler_name
                    )
                }
                _ => format!(
                    "{}this._transition(\"{}\", null, null);\n{}return this._dispatch_event(\"{}\");",
                    indent_str, target, indent_str, ctx.event_name
                ),
            }
        }
        FrameSegmentKindV3::Forward => {
            // HSM forward: call parent state's handler for the same event
            if let Some(ref parent) = ctx.parent_state {
                let parent_handler = format!("_s_{}_{}", parent, ctx.event_name);
                match lang {
                    TargetLanguage::Python3 => format!("{}self.{}()", indent_str, parent_handler),
                    TargetLanguage::TypeScript => format!("{}this.{}();", indent_str, parent_handler),
                    TargetLanguage::Rust => format!("{}self.{}()", indent_str, parent_handler),
                    _ => format!("{}this.{}();", indent_str, parent_handler),
                }
            } else {
                // No parent state - just return (shouldn't happen in valid HSM)
                match lang {
                    TargetLanguage::Python3 => format!("{}return  # Forward to parent (no parent)", indent_str),
                    _ => format!("{}return; // Forward to parent (no parent)", indent_str),
                }
            }
        }
        FrameSegmentKindV3::StackPush => {
            // Check if this is a push-then-transition: push$ -> $State
            let has_transition = segment_text.contains("->");
            let transition_code = if has_transition {
                // Extract target state and generate transition
                let target = extract_transition_target(&segment_text);
                if !target.is_empty() {
                    match lang {
                        TargetLanguage::Python3 => format!("\n{}self._transition(\"{}\", None, None)", indent_str, target),
                        TargetLanguage::TypeScript => format!("\n{}this._transition(\"{}\", null, null);", indent_str, target),
                        TargetLanguage::Rust => format!("\n{}self._transition(\"{}\")", indent_str, target),
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
                TargetLanguage::Python3 => format!("{}self._state_stack.append(self._compartment.copy())", indent_str),
                TargetLanguage::TypeScript => format!("{}this._state_stack.push(this._compartment.copy());", indent_str),
                // Rust: Use compartment-based method if there are state vars, else simple push
                TargetLanguage::Rust => {
                    if ctx.has_state_vars {
                        format!("{}self._state_stack_push();", indent_str)
                    } else {
                        format!("{}self._state_stack.push(Box::new(self._state.clone()));", indent_str)
                    }
                }
                _ => format!("{}this._state_stack.push(this._compartment.copy());", indent_str),
            };

            format!("{}{}", push_code, transition_code)
        }
        FrameSegmentKindV3::StackPop => {
            // Restore compartment from stack - preserves state vars
            // Call _exit() on current state, then restore compartment (no _enter since we're restoring)
            // Phase 14.6: No _state_context sync needed - state vars live in compartment.state_vars
            match lang {
                TargetLanguage::Python3 => format!(
                    "{}self._exit()\n{}self._compartment = self._state_stack.pop()\n{}self._state = self._compartment.state\n{}return",
                    indent_str, indent_str, indent_str, indent_str
                ),
                TargetLanguage::TypeScript => format!(
                    "{}this._exit();\n{}this._compartment = this._state_stack.pop()!;\n{}this._state = this._compartment.state;\n{}return;",
                    indent_str, indent_str, indent_str, indent_str
                ),
                // Rust: Use compartment-based method if there are state vars, else simple pop
                TargetLanguage::Rust => {
                    if ctx.has_state_vars {
                        format!("{}self._state_stack_pop();\n{}return;", indent_str, indent_str)
                    } else {
                        format!(
                            "{}let __popped_state = *self._state_stack.pop().unwrap().downcast::<String>().unwrap();\n{}self._transition(&__popped_state);\n{}return;",
                            indent_str, indent_str, indent_str
                        )
                    }
                }
                _ => format!(
                    "{}this._exit();\n{}this._compartment = this._state_stack.pop()!;\n{}this._state = this._compartment.state;\n{}return;",
                    indent_str, indent_str, indent_str, indent_str
                ),
            }
        }
        FrameSegmentKindV3::StateVar => {
            // Extract variable name from "$.varName"
            let var_name = extract_state_var_name(&segment_text);
            // State variables are stored in compartment.state_vars
            match lang {
                TargetLanguage::Python3 => format!("self._compartment.state_vars[\"{}\"]", var_name),
                TargetLanguage::TypeScript => format!("this._compartment.state_vars[\"{}\"]", var_name),
                TargetLanguage::Rust => {
                    // For Rust, access via _sv_ fields on struct
                    format!("self._sv_{}", var_name)
                },
                _ => format!("this._compartment.state_vars[\"{}\"]", var_name),
            }
        }
        FrameSegmentKindV3::SystemReturn => {
            // system.return = <expr> or return <expr>
            // return <expr> is sugar for system.return = <expr>; return (early exit)
            // system.return = just sets value without returning (for chain semantics)
            let trimmed = segment_text.trim_start();
            let is_return_sugar = trimmed.starts_with("return ");
            let expr = extract_system_return_expr(&segment_text);
            // Expand any state variable references in the expression
            let expanded_expr = expand_state_vars_in_expr(&expr, lang);

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
                        // ^ expr: set _return_value AND return (early exit)
                        format!("{}self._return_value = {}\n{}return", indent_str, expanded_expr, indent_str)
                    } else {
                        // system.return = expr: just set _return_value (chain semantics)
                        format!("{}self._return_value = {}", indent_str, expanded_expr)
                    }
                }
                TargetLanguage::TypeScript => {
                    if expanded_expr.is_empty() {
                        if is_return_sugar {
                            format!("{}return this._return_value;", indent_str)
                        } else {
                            "".to_string()
                        }
                    } else if is_return_sugar {
                        // Set _return_value AND return it (for TypeScript type compatibility)
                        format!("{}this._return_value = {};\n{}return this._return_value;", indent_str, expanded_expr, indent_str)
                    } else {
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
                            format!("{}return this._return_value;", indent_str)
                        } else {
                            "".to_string()
                        }
                    } else if is_return_sugar {
                        format!("{}this._return_value = {};\n{}return this._return_value;", indent_str, expanded_expr, indent_str)
                    } else {
                        format!("{}this._return_value = {};", indent_str, expanded_expr)
                    }
                }
            }
        }
        FrameSegmentKindV3::SystemReturnExpr => {
            // bare system.return - read current return value
            // This uses the _return_value field for expression context
            match lang {
                TargetLanguage::Python3 => "self._return_value".to_string(),
                TargetLanguage::TypeScript => "this._return_value".to_string(),
                TargetLanguage::Rust => "self._return_value".to_string(),
                _ => "this._return_value".to_string(),
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
                TargetLanguage::Python3 => result.push_str(&format!("self._compartment.state_vars[\"{}\"]", var_name)),
                TargetLanguage::TypeScript => result.push_str(&format!("this._compartment.state_vars[\"{}\"]", var_name)),
                TargetLanguage::Rust => result.push_str(&format!("self._sv_{}", var_name)),
                _ => result.push_str(&format!("this._compartment.state_vars[\"{}\"]", var_name)),
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

/// Get the native region scanner for the target language
fn get_native_scanner(lang: TargetLanguage) -> Box<dyn NativeRegionScannerV3> {
    match lang {
        TargetLanguage::Python3 => Box::new(NativeRegionScannerPyV3),
        TargetLanguage::TypeScript => Box::new(NativeRegionScannerTsV3),
        TargetLanguage::Rust => Box::new(NativeRegionScannerRustV3),
        TargetLanguage::CSharp => Box::new(NativeRegionScannerCsV3),
        TargetLanguage::C => Box::new(NativeRegionScannerCV3),
        TargetLanguage::Cpp => Box::new(NativeRegionScannerCppV3),
        TargetLanguage::Java => Box::new(NativeRegionScannerJavaV3),
        _ => Box::new(NativeRegionScannerPyV3), // Default to Python
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
            // Phase 14.6: Serialize compartment structure (no _state_context)
            let mut save_body = String::new();
            save_body.push_str("return {\n");
            save_body.push_str("    _state: this._state,\n");
            save_body.push_str("    _compartment: this._compartment.copy(),\n");
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
            // Phase 14.6: Restore compartment structure (no _state_context)
            let mut restore_body = String::new();
            restore_body.push_str(&format!("const instance = Object.create({}.prototype);\n", system.name));
            restore_body.push_str("instance._state = data._state;\n");
            // Restore compartment - create fresh instance and copy properties
            restore_body.push_str(&format!("instance._compartment = new {}Compartment(data._compartment.state);\n", system.name));
            restore_body.push_str("instance._compartment.state_args = {...(data._compartment.state_args || {})};\n");
            restore_body.push_str("instance._compartment.state_vars = {...(data._compartment.state_vars || {})};\n");
            restore_body.push_str("instance._compartment.enter_args = {...(data._compartment.enter_args || {})};\n");
            restore_body.push_str("instance._compartment.exit_args = {...(data._compartment.exit_args || {})};\n");
            restore_body.push_str("instance._compartment.forward_event = data._compartment.forward_event;\n");
            // Restore stack - each element is a serialized compartment
            restore_body.push_str(&format!("instance._state_stack = (data._state_stack || []).map((c: any) => {{\n"));
            restore_body.push_str(&format!("    const comp = new {}Compartment(c.state);\n", system.name));
            restore_body.push_str("    comp.state_args = {...(c.state_args || {})};\n");
            restore_body.push_str("    comp.state_vars = {...(c.state_vars || {})};\n");
            restore_body.push_str("    comp.enter_args = {...(c.enter_args || {})};\n");
            restore_body.push_str("    comp.exit_args = {...(c.exit_args || {})};\n");
            restore_body.push_str("    comp.forward_event = c.forward_event;\n");
            restore_body.push_str("    return comp;\n");
            restore_body.push_str("}});\n");
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

            // Check if any state has state variables (affects stack serialization)
            let has_state_vars = system.machine.as_ref()
                .map(|m| m.states.iter().any(|s| !s.state_vars.is_empty()))
                .unwrap_or(false);

            {
                // Generate save_state that manually builds JSON
                let mut save_body = String::new();

                if has_state_vars {
                    // With state vars, stack contains (String, Compartment) - can't fully serialize
                    // Extract just state names for best-effort persistence
                    save_body.push_str("let stack_states: Vec<&str> = self._state_stack.iter().map(|(s, _)| s.as_str()).collect();\n");
                    save_body.push_str("serde_json::json!({\n");
                    save_body.push_str("    \"_state\": self._state,\n");
                    save_body.push_str("    \"_state_stack\": stack_states,\n");
                } else {
                    // Without state vars, stack is Vec<Box<dyn Any>> containing Strings
                    // Extract state names by downcasting
                    save_body.push_str("let stack_states: Vec<String> = self._state_stack.iter()\n");
                    save_body.push_str("    .filter_map(|b| b.downcast_ref::<String>().cloned())\n");
                    save_body.push_str("    .collect();\n");
                    save_body.push_str("serde_json::json!({\n");
                    save_body.push_str("    \"_state\": self._state,\n");
                    save_body.push_str("    \"_state_stack\": stack_states,\n");
                }

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

                if has_state_vars {
                    // With state vars, we can only restore state names (context lost)
                    // User would need to handle re-entering states with proper args
                    restore_body.push_str("// Note: State stack restored with state names only; state vars lost\n");
                    restore_body.push_str("let stack: Vec<(String, _)> = data[\"_state_stack\"].as_array()\n");
                    restore_body.push_str("    .map(|arr| arr.iter()\n");
                    restore_body.push_str("        .filter_map(|v| v.as_str().map(|s| (s.to_string(), Default::default())))\n");
                    restore_body.push_str("        .collect())\n");
                    restore_body.push_str("    .unwrap_or_default();\n");
                } else {
                    // Without state vars, restore as Vec<Box<dyn Any>> containing Strings
                    restore_body.push_str("let stack: Vec<Box<dyn std::any::Any>> = data[\"_state_stack\"].as_array()\n");
                    restore_body.push_str("    .map(|arr| arr.iter()\n");
                    restore_body.push_str("        .filter_map(|v| v.as_str().map(|s| Box::new(s.to_string()) as Box<dyn std::any::Any>))\n");
                    restore_body.push_str("        .collect())\n");
                    restore_body.push_str("    .unwrap_or_default();\n");
                }

                restore_body.push_str(&format!("let mut instance = {} {{\n", system.name));
                restore_body.push_str("    _state: data[\"_state\"].as_str().unwrap().to_string(),\n");
                restore_body.push_str("    _state_stack: stack,\n");
                restore_body.push_str("    _state_context: std::collections::HashMap::new(),\n");

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

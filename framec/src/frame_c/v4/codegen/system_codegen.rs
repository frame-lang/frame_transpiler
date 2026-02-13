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
    ActionAst, OperationAst, Type, LoopKind,
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

    // State handlers - use enhanced Arcanum for clean iteration
    if system.machine.is_some() {
        methods.extend(generate_state_handlers_via_arcanum(&system.name, arcanum, source, lang));
    }

    // Actions - extract native code from source using spans
    for action in &system.actions {
        methods.push(generate_action(action, &syntax, source));
    }

    // Operations - extract native code from source using spans
    for operation in &system.operations {
        methods.push(generate_operation(operation, &syntax, source));
    }

    CodegenNode::Class {
        name: system.name.clone(),
        fields,
        methods,
        base_classes: vec![],
        is_abstract: false,
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
    fields.push(Field::new("_state_stack")
        .with_visibility(Visibility::Private)
        .with_type("List"));

    // State context (for state parameters)
    fields.push(Field::new("_state_context")
        .with_visibility(Visibility::Private)
        .with_type("Dict"));

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

    fields
}

/// Generate the constructor
fn generate_constructor(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> CodegenNode {
    let mut body = Vec::new();

    // Initialize state stack
    body.push(CodegenNode::assign(
        CodegenNode::field(CodegenNode::self_ref(), "_state_stack"),
        CodegenNode::Array(vec![]),
    ));

    // Initialize state context
    body.push(CodegenNode::assign(
        CodegenNode::field(CodegenNode::self_ref(), "_state_context"),
        CodegenNode::Dict(vec![]),
    ));

    // Initialize domain variables
    for domain_var in &system.domain {
        if let Some(ref init) = &domain_var.initializer {
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), &domain_var.name),
                convert_expression(init),
            ));
        }
    }

    // Set initial state as string (first state in machine)
    if let Some(ref machine) = system.machine {
        if let Some(first_state) = machine.states.first() {
            body.push(CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_state"),
                CodegenNode::string(&first_state.name),
            ));

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
    methods.push(CodegenNode::Method {
        name: "_transition".to_string(),
        params: transition_params,
        return_type: None,
        body: vec![
            // Call exit handler
            CodegenNode::ExprStmt(Box::new(
                CodegenNode::method_call(CodegenNode::self_ref(), "_exit", vec![]),
            )),
            // Change state
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_state"),
                state_value.clone(),
            ),
            // Call enter handler
            CodegenNode::ExprStmt(Box::new(
                CodegenNode::method_call(CodegenNode::self_ref(), "_enter", vec![]),
            )),
        ],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    });

    // _change_state method (no enter/exit)
    let change_state_params = match lang {
        TargetLanguage::Rust => vec![Param::new("target_state").with_type("&str")],
        TargetLanguage::TypeScript => vec![Param::new("target_state").with_type("string")],
        _ => vec![Param::new("target_state")],
    };
    // Reuse state_value for _change_state (Rust needs .to_string())
    let change_state_value = match lang {
        TargetLanguage::Rust => CodegenNode::method_call(
            CodegenNode::ident("target_state"),
            "to_string",
            vec![],
        ),
        _ => CodegenNode::ident("target_state"),
    };
    methods.push(CodegenNode::Method {
        name: "_change_state".to_string(),
        params: change_state_params,
        return_type: None,
        body: vec![
            CodegenNode::assign(
                CodegenNode::field(CodegenNode::self_ref(), "_state"),
                change_state_value,
            ),
        ],
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    });

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

    methods
}

/// Generate enter event dispatcher
/// Uses language-specific dynamic dispatch pattern
fn generate_enter_dispatcher(system: &SystemAst, lang: TargetLanguage) -> CodegenNode {
    // Check if any states have enter handlers
    let has_enter_handlers = system.machine.as_ref()
        .map(|m| m.states.iter().any(|s| s.enter.is_some()))
        .unwrap_or(false);

    let body = if !has_enter_handlers {
        vec![CodegenNode::comment("No enter handlers")]
    } else {
        match lang {
            TargetLanguage::Python3 => {
                vec![CodegenNode::NativeBlock {
                    code: "handler_name = f\"_s_{self._state}_enter\"\nhandler = getattr(self, handler_name, None)\nif handler:\n    handler()".to_string(),
                    span: None,
                }]
            }
            TargetLanguage::TypeScript => {
                vec![CodegenNode::NativeBlock {
                    code: "const handler_name = `_s_${this._state}_enter`;\nconst handler = (this as any)[handler_name];\nif (handler) {\n    handler.call(this);\n}".to_string(),
                    span: None,
                }]
            }
            TargetLanguage::Rust => {
                vec![CodegenNode::NativeBlock {
                    code: "let handler_name = format!(\"_s_{}_enter\", self._state);\n// Rust: dispatch via match or registry".to_string(),
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
        params: vec![],
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

    let body = if !has_exit_handlers {
        vec![CodegenNode::comment("No exit handlers")]
    } else {
        match lang {
            TargetLanguage::Python3 => {
                vec![CodegenNode::NativeBlock {
                    code: "handler_name = f\"_s_{self._state}_exit\"\nhandler = getattr(self, handler_name, None)\nif handler:\n    handler()".to_string(),
                    span: None,
                }]
            }
            TargetLanguage::TypeScript => {
                vec![CodegenNode::NativeBlock {
                    code: "const handler_name = `_s_${this._state}_exit`;\nconst handler = (this as any)[handler_name];\nif (handler) {\n    handler.call(this);\n}".to_string(),
                    span: None,
                }]
            }
            TargetLanguage::Rust => {
                vec![CodegenNode::NativeBlock {
                    code: "let handler_name = format!(\"_s_{}_exit\", self._state);\n// Rust: dispatch via match or registry".to_string(),
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
        params: vec![],
        return_type: None,
        body,
        is_async: false,
        is_static: false,
        visibility: Visibility::Private,
        decorators: vec![],
    }
}

/// Generate interface wrapper methods
fn generate_interface_wrappers(system: &SystemAst, syntax: &super::backend::ClassSyntax) -> Vec<CodegenNode> {
    system.interface.iter().map(|method| {
        let params: Vec<Param> = method.params.iter().map(|p| {
            let type_str = type_to_string(&p.param_type);
            Param::new(&p.name).with_type(&type_str)
        }).collect();

        let args: Vec<CodegenNode> = method.params.iter()
            .map(|p| CodegenNode::ident(&p.name))
            .collect();

        // Generate dispatch: call _dispatch_event(event_name, *args)
        let mut dispatch_args = vec![CodegenNode::string(&method.name)];
        dispatch_args.extend(args);

        CodegenNode::Method {
            name: method.name.clone(),
            params,
            return_type: method.return_type.as_ref().map(|t| type_to_string(t)),
            body: vec![
                CodegenNode::ExprStmt(Box::new(
                    CodegenNode::method_call(
                        CodegenNode::self_ref(),
                        "_dispatch_event",
                        dispatch_args,
                    ),
                )),
            ],
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            decorators: vec![],
        }
    }).collect()
}

/// Generate state handler methods using the enhanced Arcanum
///
/// This is the preferred method - uses the Arcanum's handler tracking for clean iteration.
/// The Arcanum was populated from the AST, so this is functionally equivalent but cleaner.
fn generate_state_handlers_via_arcanum(system_name: &str, arcanum: &Arcanum, source: &[u8], lang: TargetLanguage) -> Vec<CodegenNode> {
    let mut methods = Vec::new();

    // Iterate over all enhanced states in the system
    for state_entry in arcanum.get_enhanced_states(system_name) {
        // Generate handler methods for each handler in the state
        for (event, handler_entry) in &state_entry.handlers {
            let method = generate_handler_from_arcanum(
                &state_entry.name,
                handler_entry,
                source,
                lang,
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
    handler: &HandlerEntry,
    source: &[u8],
    lang: TargetLanguage,
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

    // Splice the handler body: preserve native code, expand Frame segments
    let body_code = splice_handler_body_from_span(&handler.body_span, source, lang);

    CodegenNode::Method {
        name: method_name,
        params,
        return_type: None,
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
fn splice_handler_body_from_span(span: &crate::frame_c::v4::ast::Span, source: &[u8], lang: TargetLanguage) -> String {
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
            let expansion = generate_frame_expansion(body_bytes, span, *kind, *indent, lang);
            expansions.push(expansion);
        }
    }

    // Use splicer to combine native + generated Frame code
    let splicer = SplicerV3;
    let spliced = splicer.splice(body_bytes, &scan_result.regions, &expansions);

    if std::env::var("FRAME_DEBUG_SPLICER").is_ok() {
        eprintln!("[splice_handler_body_from_span] Spliced result: {:?}", spliced.text);
    }

    // The splicer already produces content WITHOUT the outer braces
    // (since regions start after { and end before })
    // Just trim leading newline and trailing whitespace
    // V4 uses native syntax, so no transpilation needed - just return as-is
    spliced.text.trim_start_matches('\n').trim_end().to_string()
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
                &enter.body,
                source,
                lang,
            ));
        }

        // Exit handler
        if let Some(ref exit) = state.exit {
            methods.push(generate_enter_exit_handler(
                &format!("_s_{}_exit", state.name),
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
fn generate_enter_exit_handler(name: &str, body: &HandlerBody, source: &[u8], lang: TargetLanguage) -> CodegenNode {
    let body_code = splice_handler_body(body, source, lang);

    CodegenNode::Method {
        name: name.to_string(),
        params: vec![],
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

    // Generate expansions for each Frame segment
    let mut expansions = Vec::new();
    for region in &scan_result.regions {
        if let RegionV3::FrameSegment { span, kind, indent } = region {
            let expansion = generate_frame_expansion(body_bytes, span, *kind, *indent, lang);
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
fn generate_frame_expansion(body_bytes: &[u8], span: &crate::frame_c::v4::native_region_scanner::RegionSpan, kind: FrameSegmentKindV3, indent: usize, lang: TargetLanguage) -> String {
    let segment_text = String::from_utf8_lossy(&body_bytes[span.start..span.end]);
    // Indentation prefix to replace the gap not covered by scanner
    let indent_str = " ".repeat(indent);

    match kind {
        FrameSegmentKindV3::Transition => {
            // Parse transition: -> $State or -> $State(args)
            let target = extract_transition_target(&segment_text);
            match lang {
                TargetLanguage::Python3 => format!("{}self._transition(\"{}\", None, None)", indent_str, target),
                TargetLanguage::TypeScript => format!("{}this._transition(\"{}\");", indent_str, target),
                _ => format!("{}self._transition(\"{}\")", indent_str, target),
            }
        }
        FrameSegmentKindV3::Forward => {
            match lang {
                TargetLanguage::Python3 => format!("{}return  # Forward to parent", indent_str),
                _ => format!("{}return; // Forward to parent", indent_str),
            }
        }
        FrameSegmentKindV3::StackPush => {
            match lang {
                TargetLanguage::Python3 => format!("{}self._state_stack.append(self._state)", indent_str),
                _ => format!("{}this._state_stack.push(this._state);", indent_str),
            }
        }
        FrameSegmentKindV3::StackPop => {
            match lang {
                TargetLanguage::Python3 => format!("{}self._transition(self._state_stack.pop())", indent_str),
                _ => format!("{}this._transition(this._state_stack.pop());", indent_str),
            }
        }
    }
}

/// Extract transition target from transition text
fn extract_transition_target(text: &str) -> String {
    // Find $StateName in the transition text
    if let Some(dollar_pos) = text.find('$') {
        let after_dollar = &text[dollar_pos + 1..];
        let end = after_dollar.find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(after_dollar.len());
        after_dollar[..end].to_string()
    } else {
        "Unknown".to_string()
    }
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

    CodegenNode::Method {
        name: operation.name.clone(),
        params,
        return_type: Some(type_to_string(&operation.return_type)),
        body: vec![CodegenNode::NativeBlock {
            code,
            span: Some(operation.body.span.clone()),
        }],
        is_async: false,
        is_static: false,
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

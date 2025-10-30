use crate::frame_c::ast::*;
use crate::frame_c::scanner::TokenType;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write as _;

use super::context::{
    ActionEntry, ActionParam, MainLocal, MainScope, MethodNames, StateEntry, StateParam,
    SystemEmitContext, SystemSummary,
};
use super::utils::{
    call_chain_node_kind, encode_c_string, expr_kind, extract_string_literal, format_f64,
    sanitize_identifier, sanitize_numeric_literal,
};
use super::value::{
    infer_value_kind_from_type, DomainField, DomainFieldInit, DomainFieldType, LocalBinding,
    ValueKind, ValueRef,
};

pub(super) struct LLVMModuleBuilder {
    header: String,
    body: String,
    string_literals: Vec<StringLiteral>,
    string_map: HashMap<String, usize>,
    defined_structs: HashSet<String>,
    needs_puts: bool,
    needs_print_int: bool,
    needs_print_double: bool,
    needs_print_bool: bool,
    needs_runtime_api: bool,
    needs_runtime_event: bool,
    generated_enter_handlers: HashSet<String>,
    generated_exit_handlers: HashSet<String>,
    indent: usize,
    temp_counter: usize,
}

struct StringLiteral {
    name: String,
    len: usize,
    encoded: String,
}

struct StringRef {
    name: String,
    len: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HandlerKind {
    Event,
    Enter,
    Exit,
}

#[derive(Clone)]
struct HandlerParam {
    name: String,
    kind: ValueKind,
}

#[derive(Clone, Copy)]
struct HandlerScope<'a> {
    state: &'a StateEntry,
    kind: HandlerKind,
    event_params: &'a [HandlerParam],
}

impl<'a> HandlerScope<'a> {
    fn event_param(&self, name: &str) -> Option<&'a HandlerParam> {
        self.event_params.iter().find(|param| param.name == name)
    }
}

impl LLVMModuleBuilder {
    pub(super) fn new() -> Self {
        let mut header = String::new();
        header.push_str("; ModuleID = 'framec-llvm'\n");
        header.push_str("source_filename = \"framec\"\n\n");

        LLVMModuleBuilder {
            header,
            body: String::new(),
            string_literals: Vec::new(),
            string_map: HashMap::new(),
            defined_structs: HashSet::new(),
            needs_puts: false,
            needs_runtime_api: false,
            needs_runtime_event: false,
            needs_print_int: false,
            needs_print_double: false,
            needs_print_bool: false,
            generated_enter_handlers: HashSet::new(),
            generated_exit_handlers: HashSet::new(),
            indent: 0,
            temp_counter: 0,
        }
    }

    pub(super) fn ensure_struct(&mut self, struct_name: &str, fields: &[String]) {
        if self.defined_structs.insert(struct_name.to_string()) {
            let field_list = if fields.is_empty() {
                "i32".to_string()
            } else {
                fields.join(", ")
            };
            writeln!(
                &mut self.header,
                "{} = type {{ {} }}",
                struct_name, field_list
            )
            .unwrap();
        }
    }

    pub(super) fn emit_init_function(&mut self, ctx: &SystemEmitContext) {
        self.begin_function();
        let fn_name = format!("@{}__init", ctx.sanitized_name);

        writeln!(
            &mut self.body,
            "define void {}({}* %self) {{",
            fn_name, ctx.struct_name
        )
        .unwrap();
        self.indent += 1;

        let state_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 0",
            state_ptr, ctx.struct_name, ctx.struct_name
        ));
        self.push_line(&format!(
            "store i32 {}, i32* {}",
            ctx.start_state_index, state_ptr
        ));

        for field in &ctx.domain_fields {
            let field_ptr = self.next_temp();
            self.push_line(&format!(
                "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
                field_ptr, ctx.struct_name, ctx.struct_name, field.struct_index
            ));
            match field.field_type {
                DomainFieldType::I32 => {
                    let value = match field.initializer {
                        DomainFieldInit::Int(v) => v,
                        DomainFieldInit::Bool(true) => 1,
                        DomainFieldInit::Bool(false) => 0,
                        DomainFieldInit::Float(f) => f as i64,
                        DomainFieldInit::CString(_) => 0,
                    };
                    self.push_line(&format!("store i32 {}, i32* {}", value, field_ptr));
                }
                DomainFieldType::F64 => {
                    let value = match field.initializer {
                        DomainFieldInit::Float(v) => format_f64(v),
                        DomainFieldInit::Int(v) => format_f64(v as f64),
                        DomainFieldInit::Bool(true) => format_f64(1.0),
                        DomainFieldInit::Bool(false) => format_f64(0.0),
                        DomainFieldInit::CString(_) => format_f64(0.0),
                    };
                    self.push_line(&format!("store double {}, double* {}", value, field_ptr));
                }
                DomainFieldType::Bool => {
                    let value = match field.initializer {
                        DomainFieldInit::Bool(v) => {
                            if v {
                                1
                            } else {
                                0
                            }
                        }
                        DomainFieldInit::Int(v) => {
                            if v != 0 {
                                1
                            } else {
                                0
                            }
                        }
                        DomainFieldInit::Float(v) => {
                            if v != 0.0 {
                                1
                            } else {
                                0
                            }
                        }
                        DomainFieldInit::CString(_) => 0,
                    };
                    self.push_line(&format!("store i1 {}, i1* {}", value, field_ptr));
                }
                DomainFieldType::CString => {
                    let text = match &field.initializer {
                        DomainFieldInit::CString(ref s) => s.as_str(),
                        _ => "",
                    };
                    let literal = self.intern_string(text, false);
                    let data_ptr = self.next_temp();
                    self.push_line(&format!(
                        "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                        data_ptr, literal.len, literal.len, literal.name
                    ));
                    self.push_line(&format!("store i8* {}, i8** {}", data_ptr, field_ptr));
                }
            }
        }

        self.require_runtime_api();
        let start_state_literal = self.intern_string(ctx.start_state_name(), false);
        let start_state_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            start_state_ptr,
            start_state_literal.len,
            start_state_literal.len,
            start_state_literal.name
        ));
        let runtime_compartment = self.next_temp();
        self.push_line(&format!(
            "{} = call ptr @frame_runtime_compartment_new(ptr {})",
            runtime_compartment, start_state_ptr
        ));
        self.push_line(&format!(
            "call void @frame_runtime_compartment_set_enter_event(ptr {}, ptr null)",
            runtime_compartment
        ));
        self.push_line(&format!(
            "call void @frame_runtime_compartment_set_exit_event(ptr {}, ptr null)",
            runtime_compartment
        ));
        let runtime_kernel = self.next_temp();
        self.push_line(&format!(
            "{} = call ptr @frame_runtime_kernel_new(ptr {})",
            runtime_kernel, runtime_compartment
        ));
        let kernel_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            kernel_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.runtime_field_index()
        ));
        self.push_line(&format!(
            "store ptr {}, ptr {}",
            runtime_kernel, kernel_field_ptr
        ));

        let compartment_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            compartment_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.compartment_field_index()
        ));

        self.push_line(&format!(
            "store ptr {}, ptr {}",
            runtime_compartment, compartment_field_ptr
        ));

        self.push_line("ret void");

        self.indent -= 1;
        self.push_line("}");
        self.push_blank_line();
    }

    pub(super) fn emit_deinit_function(&mut self, ctx: &SystemEmitContext) {
        self.begin_function();
        let fn_name = format!("@{}__deinit", ctx.sanitized_name);
        writeln!(
            &mut self.body,
            "define void {}({}* %self) {{",
            fn_name, ctx.struct_name
        )
        .unwrap();
        self.indent += 1;

        self.require_runtime_api();
        let kernel_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            kernel_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.runtime_field_index()
        ));
        let kernel_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = load ptr, ptr {}",
            kernel_ptr, kernel_field_ptr
        ));

        let compartment_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            compartment_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.compartment_field_index()
        ));
        let has_kernel = self.next_temp();
        self.push_line(&format!(
            "{} = icmp ne ptr {}, null",
            has_kernel, kernel_ptr
        ));

        let free_label = format!("{}_deinit_free", ctx.sanitized_name);
        let end_label = format!("{}_deinit_end", ctx.sanitized_name);
        self.push_line(&format!(
            "br i1 {}, label %{}, label %{}",
            has_kernel, free_label, end_label
        ));

        self.push_line(&format!("{}:", free_label));
        self.indent += 1;
        self.push_line(&format!(
            "call void @frame_runtime_kernel_free(ptr {})",
            kernel_ptr
        ));
        self.push_line(&format!("store ptr null, ptr {}", kernel_field_ptr));
        self.push_line(&format!("br label %{}", end_label));
        self.indent -= 1;

        self.push_line(&format!("{}:", end_label));
        self.indent += 1;
        self.push_line("ret void");
        self.indent -= 1;
        self.push_line("}");
        self.push_blank_line();
    }

    pub(super) fn emit_interface_method(
        &mut self,
        ctx: &SystemEmitContext,
        names: &MethodNames,
        method: &InterfaceMethodNode,
    ) {
        for state in &ctx.states {
            self.emit_state_enter_function(ctx, state);
            self.emit_state_exit_function(ctx, state);
        }
        self.begin_function();
        let default_label = format!(
            "{}_{}_dispatch_default",
            ctx.sanitized_name, names.method_ident
        );
        let end_label = format!("{}_{}_dispatch_end", ctx.sanitized_name, names.method_ident);

        let mut signature_parts = Vec::new();
        signature_parts.push(format!("{}* %self", ctx.struct_name));
        let mut param_bindings: Vec<(String, ValueKind, String)> = Vec::new();
        if let Some(params) = &method.params {
            for (idx, param) in params.iter().enumerate() {
                let kind = infer_value_kind_from_type(param.param_type_opt.as_ref());
                let llvm_ty = Self::llvm_type_for_kind(kind);
                let ir_name = format!("%{}_arg{}", sanitize_identifier(&param.param_name), idx);
                signature_parts.push(format!("{} {}", llvm_ty, ir_name));
                param_bindings.push((param.param_name.clone(), kind, ir_name));
            }
        }
        let param_specs: Vec<(String, ValueKind)> = param_bindings
            .iter()
            .map(|(name, kind, _)| (name.clone(), *kind))
            .collect();

        writeln!(
            &mut self.body,
            "define void {}({}) {{",
            names.fn_name,
            signature_parts.join(", ")
        )
        .unwrap();
        self.indent += 1;

        self.require_runtime_api();
        self.require_runtime_event();
        let kernel_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            kernel_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.runtime_field_index()
        ));
        let kernel_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = load ptr, ptr {}",
            kernel_ptr, kernel_field_ptr
        ));

        let mut locals: HashMap<String, LocalBinding> = HashMap::new();
        for (name, kind, ir_name) in &param_bindings {
            let ptr = self.alloca_for_kind(*kind);
            self.push_line(&format!(
                "store {} {}, {}* {}",
                Self::llvm_type_for_kind(*kind),
                ir_name,
                Self::llvm_type_for_kind(*kind),
                ptr
            ));
            locals.insert(
                name.clone(),
                LocalBinding {
                    ptr: ptr.clone(),
                    kind: *kind,
                },
            );
        }

        let state_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 0",
            state_ptr, ctx.struct_name, ctx.struct_name
        ));

        let state_val = self.next_temp();
        self.push_line(&format!("{} = load i32, i32* {}", state_val, state_ptr));

        self.push_line(&format!(
            "switch i32 {}, label %{} [",
            state_val, default_label
        ));
        self.indent += 1;
        for (idx, state) in ctx.states.iter().enumerate() {
            let state_label = ctx.state_label(&names.method_ident, &state.name);
            self.push_line(&format!("i32 {}, label %{}", idx, state_label));
        }
        self.indent -= 1;
        self.push_line("]");
        self.push_blank_line();

        // Generate per-state handlers
        for (idx, state) in ctx.states.iter().enumerate() {
            let label = ctx.state_label(&names.method_ident, &state.name);
            self.push_line(&format!("{}:", label));
            self.indent += 1;

            let event_name = method
                .alias
                .as_ref()
                .map(|msg| msg.name.clone())
                .unwrap_or_else(|| method.name.clone());

            if let Some(handler_rc) = state.handlers.get(&event_name) {
                let handler = handler_rc.borrow();
                let message_literal = self.intern_string(&event_name, false);
                let message_ptr = self.next_temp();
                self.push_line(&format!(
                    "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    message_ptr, message_literal.len, message_literal.len, message_literal.name
                ));
                let event_handle = self.next_temp();
                self.push_line(&format!(
                    "{} = call ptr @frame_runtime_event_new(ptr {})",
                    event_handle, message_ptr
                ));
                if !param_specs.is_empty() {
                    self.push_event_params_from_locals(&event_handle, &param_specs, &locals);
                }
                self.push_line(&format!(
                    "call i32 @frame_runtime_kernel_dispatch(ptr {}, ptr {})",
                    kernel_ptr, event_handle
                ));
                self.push_line(&format!(
                    "call void @frame_runtime_event_free(ptr {})",
                    event_handle
                ));
                self.emit_event_handler_body(
                    ctx,
                    &kernel_ptr,
                    &state_ptr,
                    &end_label,
                    &handler,
                    idx as i32,
                    &names.method_ident,
                    &event_name,
                    state.parent_state_name.as_deref(),
                    Some(&locals),
                );
            } else {
                self.push_comment("no handler for event in this state");
                self.push_line(&format!("br label %{}", end_label));
            }

            self.indent -= 1;
            self.push_blank_line();
        }

        // Default branch
        self.push_line(&format!("{}:", default_label));
        self.indent += 1;
        self.push_comment("unhandled state dispatch");
        self.push_line(&format!("br label %{}", end_label));
        self.indent -= 1;
        self.push_blank_line();

        // Function epilogue
        self.push_line(&format!("{}:", end_label));
        self.indent += 1;
        self.push_line("ret void");
        self.indent -= 1;
        self.push_line("}");
        self.indent -= 1;
        self.push_blank_line();
    }

    pub(super) fn emit_action_function(&mut self, ctx: &SystemEmitContext, action: &ActionEntry) {
        self.begin_function();
        let mut param_decls = Vec::new();
        let mut param_bindings: Vec<(ActionParam, String)> = Vec::new();
        for (idx, param) in action.params.iter().enumerate() {
            let llvm_ty = Self::llvm_type_for_kind(param.kind);
            let param_name = format!("%{}_arg{}", sanitize_identifier(&param.name), idx);
            param_decls.push(format!("{} {}", llvm_ty, param_name));
            param_bindings.push((param.clone(), param_name));
        }

        let param_suffix = if param_decls.is_empty() {
            String::new()
        } else {
            format!(", {}", param_decls.join(", "))
        };

        writeln!(
            &mut self.body,
            "define void {}({}* %self{}) {{",
            action.fn_name, ctx.struct_name, param_suffix
        )
        .unwrap();
        self.indent += 1;

        let mut locals: HashMap<String, LocalBinding> = HashMap::new();
        let kernel_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            kernel_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.runtime_field_index()
        ));
        let kernel_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = load ptr, ptr {}",
            kernel_ptr, kernel_field_ptr
        ));
        for (param, ir_name) in &param_bindings {
            let ptr = self.alloca_for_kind(param.kind);
            self.push_line(&format!(
                "store {} {}, {}* {}",
                Self::llvm_type_for_kind(param.kind),
                ir_name,
                Self::llvm_type_for_kind(param.kind),
                ptr
            ));
            let binding = LocalBinding {
                ptr,
                kind: param.kind,
            };
            locals.insert(param.name.clone(), binding);
        }

        {
            let action_node = action.node.borrow();
            for stmt in &action_node.statements {
                match stmt {
                    DeclOrStmtType::StmtT { stmt_t } => match stmt_t {
                        StatementType::ExpressionStmt { expr_stmt_t } => {
                            self.emit_expression_statement(
                                ctx,
                                "%self",
                                expr_stmt_t,
                                Some(&locals),
                                None,
                            );
                        }
                        StatementType::ReturnStmt { .. } => {
                            self.push_comment("return statements in actions not yet supported");
                        }
                        _ => {
                            self.push_comment("unsupported statement in action body");
                        }
                    },
                    DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                        let var_decl = var_decl_t_rcref.borrow();
                        let init_expr = var_decl.get_initializer_value_rc();
                        let value = match self.emit_expression_value(
                            ctx,
                            "%self",
                            Some(&locals),
                            &*init_expr,
                            None,
                        ) {
                            Some(value) => value,
                            None => {
                                self.push_comment(
                                    "unsupported initializer for action local variable",
                                );
                                continue;
                            }
                        };

                        let ptr = self.alloca_for_kind(value.kind);
                        let binding = LocalBinding {
                            ptr: ptr.clone(),
                            kind: value.kind,
                        };
                        let coerced = match self.coerce_value_for_kind(value, binding.kind) {
                            Some(value) => value,
                            None => {
                                self.push_comment("type mismatch in local variable initializer");
                                continue;
                            }
                        };
                        self.store_local_value(&binding, coerced);
                        locals.insert(var_decl.name.clone(), binding);
                    }
                }
            }

            if matches!(
                action_node.terminator_expr.terminator_type,
                TerminatorType::Return
            ) && action_node.terminator_expr.return_expr_t_opt.is_some()
            {
                self.push_comment("action terminator return expressions not yet supported");
            }
        }

        self.push_line("ret void");
        self.indent -= 1;
        self.push_line("}");
        self.push_blank_line();
    }

    pub(super) fn emit_main_function(
        &mut self,
        function: &FunctionNode,
        systems: &HashMap<String, SystemSummary>,
    ) {
        self.begin_function();
        self.push_line("define i32 @main() {");
        self.indent += 1;

        let mut locals = MainScope::new();

        for stmt in &function.statements {
            match stmt {
                DeclOrStmtType::VarDeclT { var_decl_t_rcref } => {
                    let var_decl = var_decl_t_rcref.borrow();
                    let init_expr = var_decl.get_initializer_value_rc();
                    if !self.emit_main_system_var(&var_decl.name, &*init_expr, systems, &mut locals)
                    {
                        self.push_comment(&format!(
                            "unsupported variable initializer in main: {}",
                            expr_kind(&init_expr)
                        ));
                    }
                }
                DeclOrStmtType::StmtT { stmt_t } => {
                    match stmt_t {
                        StatementType::ExpressionStmt { expr_stmt_t } => {
                            self.emit_main_expression(expr_stmt_t, &locals);
                        }
                        StatementType::ReturnStmt { .. } | StatementType::NoStmt => {
                            // Ignore explicit returns; we'll emit a single ret at the end.
                        }
                        _ => {
                            self.push_comment("unsupported statement in main function");
                        }
                    }
                }
            }
        }

        for var_name in locals.drop_order().rev() {
            if let Some(local) = locals.get(var_name.as_str()) {
                self.push_line(&format!(
                    "call void {}({}* {})",
                    local.system.deinit_fn(),
                    local.system.struct_name,
                    local.ptr
                ));
            }
        }

        self.push_line("ret i32 0");
        self.indent -= 1;
        self.push_line("}");
        self.push_blank_line();
    }

    fn emit_main_expression(&mut self, expr_stmt: &ExprStmtType, locals: &MainScope) {
        match expr_stmt {
            ExprStmtType::CallStmtT { call_stmt_node } => {
                if self.handle_call_expr(None, None, None, &call_stmt_node.call_expr_node, None) {
                    return;
                }
                self.push_comment("unsupported call in main");
            }
            ExprStmtType::CallChainStmtT {
                call_chain_literal_stmt_node,
            } => {
                let chain = &call_chain_literal_stmt_node
                    .call_chain_literal_expr_node
                    .call_chain;
                let mut iter = chain.iter();
                match (iter.next(), iter.next()) {
                    (
                        Some(CallChainNodeType::VariableNodeT { var_node }),
                        Some(CallChainNodeType::UndeclaredCallT { call_node }),
                    ) => {
                        let var_name = var_node.get_name();
                        if let Some(local) = locals.get(var_name) {
                            let method_name = call_node.identifier.name.lexeme.as_str();
                            let fn_name = local.system.method_fn(method_name);
                            let mut call_parts = Vec::new();
                            call_parts.push(format!("{}* {}", local.system.struct_name, local.ptr));
                            let mut arg_values = Vec::new();
                            for expr in &call_node.call_expr_list.exprs_t {
                                match self.emit_main_value_with_system(
                                    &local.system,
                                    &local.ptr,
                                    expr,
                                ) {
                                    Some(value) => arg_values.push(value),
                                    None => {
                                        self.push_comment(
                                            "unsupported argument expression in main call",
                                        );
                                        return;
                                    }
                                }
                            }
                            if let Some(expected) = local.system.interface_params(method_name) {
                                if expected.len() != arg_values.len() {
                                    self.push_comment("interface argument count mismatch in main");
                                    return;
                                }
                                let mut adjusted = Vec::with_capacity(arg_values.len());
                                for (value, expected_kind) in
                                    arg_values.into_iter().zip(expected.iter())
                                {
                                    if value.kind == *expected_kind {
                                        adjusted.push(value);
                                    } else if let Some(coerced) =
                                        self.coerce_value_for_kind(value, *expected_kind)
                                    {
                                        adjusted.push(coerced);
                                    } else {
                                        self.push_comment(
                                            "type mismatch in main interface call argument",
                                        );
                                        return;
                                    }
                                }
                                arg_values = adjusted;
                            }
                            for value in arg_values {
                                call_parts.push(format!(
                                    "{} {}",
                                    Self::llvm_type_for_kind(value.kind),
                                    value.value
                                ));
                            }
                            self.push_line(&format!(
                                "call void {}({})",
                                fn_name,
                                call_parts.join(", ")
                            ));
                            return;
                        }
                        self.push_comment("unknown variable used in main call");
                    }
                    (
                        Some(CallChainNodeType::VariableNodeT { var_node }),
                        Some(CallChainNodeType::InterfaceMethodCallT {
                            interface_method_call_expr_node,
                        }),
                    ) => {
                        let var_name = var_node.get_name();
                        if let Some(local) = locals.get(var_name) {
                            let method_name = interface_method_call_expr_node
                                .identifier
                                .name
                                .lexeme
                                .as_str();
                            let fn_name = local.system.method_fn(method_name);
                            let mut call_parts = Vec::new();
                            call_parts.push(format!("{}* {}", local.system.struct_name, local.ptr));
                            let mut arg_values = Vec::new();
                            for expr in &interface_method_call_expr_node.call_expr_list.exprs_t {
                                match self.emit_main_value_with_system(
                                    &local.system,
                                    &local.ptr,
                                    expr,
                                ) {
                                    Some(value) => arg_values.push(value),
                                    None => {
                                        self.push_comment(
                                            "unsupported argument expression in main call",
                                        );
                                        return;
                                    }
                                }
                            }
                            if let Some(expected) = local.system.interface_params(method_name) {
                                if expected.len() != arg_values.len() {
                                    self.push_comment("interface argument count mismatch in main");
                                    return;
                                }
                                let mut adjusted = Vec::with_capacity(arg_values.len());
                                for (value, expected_kind) in
                                    arg_values.into_iter().zip(expected.iter())
                                {
                                    if value.kind == *expected_kind {
                                        adjusted.push(value);
                                    } else if let Some(coerced) =
                                        self.coerce_value_for_kind(value, *expected_kind)
                                    {
                                        adjusted.push(coerced);
                                    } else {
                                        self.push_comment(
                                            "type mismatch in main interface call argument",
                                        );
                                        return;
                                    }
                                }
                                arg_values = adjusted;
                            }
                            for value in arg_values {
                                call_parts.push(format!(
                                    "{} {}",
                                    Self::llvm_type_for_kind(value.kind),
                                    value.value
                                ));
                            }
                            self.push_line(&format!(
                                "call void {}({})",
                                fn_name,
                                call_parts.join(", ")
                            ));
                            return;
                        } else {
                            self.push_comment("unknown variable used in main call");
                        }
                        self.push_comment("unsupported interface call in main");
                    }
                    _ => {
                        let kinds: Vec<&'static str> =
                            chain.iter().map(call_chain_node_kind).collect();
                        self.push_comment(&format!(
                            "unsupported call chain expression in main: {:?}",
                            kinds
                        ));
                    }
                }
            }
            _ => {
                self.push_comment("unsupported expression statement in main");
            }
        }
    }

    fn emit_main_system_var(
        &mut self,
        var_name: &str,
        init_expr: &ExprType,
        systems: &HashMap<String, SystemSummary>,
        locals: &mut MainScope,
    ) -> bool {
        let (system_name, check_args) = match init_expr {
            ExprType::SystemInstanceExprT {
                system_instance_expr_node,
            } => (
                system_instance_expr_node.identifier.name.lexeme.as_str(),
                Some(system_instance_expr_node),
            ),
            ExprType::CallExprT { call_expr_node } => {
                (call_expr_node.identifier.name.lexeme.as_str(), None)
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                if let Some(CallChainNodeType::UndeclaredCallT { call_node }) =
                    call_chain_expr_node.call_chain.front()
                {
                    (call_node.identifier.name.lexeme.as_str(), None)
                } else {
                    return false;
                }
            }
            _ => return false,
        };

        let summary = match systems.get(system_name) {
            Some(summary) => summary.clone(),
            None => {
                self.push_comment("unknown system in main initializer");
                return true;
            }
        };

        if let Some(node) = check_args {
            if node.start_state_state_args_opt.is_some()
                || node.start_state_enter_args_opt.is_some()
                || node.domain_args_opt.is_some()
            {
                self.push_comment("state/domain arguments not yet supported in main");
                return true;
            }
        }
        if let ExprType::CallExprT { call_expr_node } = init_expr {
            if !call_expr_node.call_expr_list.exprs_t.is_empty() {
                self.push_comment("system constructor arguments not yet supported in main");
                return true;
            }
        }
        if let ExprType::CallChainExprT {
            call_chain_expr_node,
        } = init_expr
        {
            if let Some(CallChainNodeType::UndeclaredCallT { call_node }) =
                call_chain_expr_node.call_chain.front()
            {
                if !call_node.call_expr_list.exprs_t.is_empty() {
                    self.push_comment("system constructor arguments not yet supported in main");
                    return true;
                }
            }
        }

        let ptr_name = self.next_temp();
        self.push_line(&format!(
            "{} = alloca {}, align {}",
            ptr_name, summary.struct_name, summary.align
        ));
        self.push_line(&format!(
            "call void {}({}* {})",
            summary.init_fn(),
            summary.struct_name,
            ptr_name
        ));

        locals.insert(
            var_name.to_string(),
            MainLocal {
                ptr: ptr_name,
                system: summary,
            },
        );

        true
    }

    fn emit_event_handler_body(
        &mut self,
        ctx: &SystemEmitContext,
        kernel_ptr: &str,
        state_ptr: &str,
        end_label: &str,
        handler: &EventHandlerNode,
        current_state_index: i32,
        method_ident: &str,
        event_name: &str,
        parent_state_name: Option<&str>,
        locals: Option<&HashMap<String, LocalBinding>>,
    ) {
        let compartment_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            compartment_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.compartment_field_index()
        ));

        self.require_runtime_api();
        let current_state_entry = ctx.state(current_state_index as usize);
        let event_params: Vec<HandlerParam> = {
            let symbol = handler.event_symbol_rcref.borrow();
            symbol
                .event_symbol_params_opt
                .as_ref()
                .map(|params| {
                    params
                        .iter()
                        .map(|param| HandlerParam {
                            name: param.name.clone(),
                            kind: infer_value_kind_from_type(param.param_type_opt.as_ref()),
                        })
                        .collect()
                })
                .unwrap_or_default()
        };
        let handler_scope = HandlerScope {
            state: current_state_entry,
            kind: HandlerKind::Event,
            event_params: &event_params,
        };

        let queue_loop_label = format!("{}_queue_loop_{}", ctx.sanitized_name, current_state_index);
        let queue_exit_label = format!("{}_queue_exit_{}", ctx.sanitized_name, current_state_index);
        let queue_default_label = format!(
            "{}_queue_default_{}",
            ctx.sanitized_name, current_state_index
        );
        let queue_check_label = if ctx.interface_methods().is_empty() {
            queue_default_label.clone()
        } else {
            format!("{}_queue_check_{}", ctx.sanitized_name, current_state_index)
        };

        let child_state_backup = self.next_temp();
        self.push_line(&format!(
            "{} = load i32, i32* {}",
            child_state_backup, state_ptr
        ));
        let child_comp_backup = self.next_temp();
        self.push_line(&format!(
            "{} = load ptr, ptr {}",
            child_comp_backup, compartment_field_ptr
        ));

        'stmt_loop: for stmt in &handler.statements {
            if let DeclOrStmtType::StmtT { stmt_t } = stmt {
                match stmt_t {
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        self.emit_expression_statement(
                            ctx,
                            "%self",
                            expr_stmt_t,
                            locals,
                            Some(handler_scope),
                        );
                    }
                    StatementType::TransitionStmt {
                        transition_statement_node,
                    } => {
                        if let TargetStateContextType::StateStackPop {} = transition_statement_node
                            .transition_expr_node
                            .target_state_context_t
                        {
                            self.emit_state_stack_pop(
                                ctx,
                                kernel_ptr,
                                state_ptr,
                                compartment_field_ptr.as_str(),
                                queue_loop_label.as_str(),
                                current_state_index,
                                method_ident,
                            );
                            continue 'stmt_loop;
                        }
                        if let Some(target_index) =
                            ctx.transition_target_index(transition_statement_node)
                        {
                            let current_state = handler_scope.state;
                            if current_state.exit_handler.is_some() {
                                let exit_fn = ctx.state_exit_fn(&current_state.name);
                                self.push_line(&format!(
                                    "call void {}({}* %self)",
                                    exit_fn, ctx.struct_name
                                ));
                            }
                            self.push_line(&format!(
                                "store i32 {}, i32* {}",
                                target_index, state_ptr
                            ));

                            if let TargetStateContextType::StateRef { state_context_node } =
                                &transition_statement_node
                                    .transition_expr_node
                                    .target_state_context_t
                            {
                                let target_state = ctx.state(target_index as usize);
                                let pending_state_args = state_context_node
                                    .state_ref_args_opt
                                    .as_ref()
                                    .map(|exprs| {
                                        self.collect_pending_args(
                                            ctx,
                                            "%self",
                                            exprs,
                                            &target_state.state_params,
                                            locals,
                                            Some(handler_scope),
                                            "state",
                                        )
                                    })
                                    .unwrap_or_default();
                                let pending_enter_args = state_context_node
                                    .enter_args_opt
                                    .as_ref()
                                    .map(|exprs| {
                                        self.collect_pending_args(
                                            ctx,
                                            "%self",
                                            exprs,
                                            &target_state.enter_params,
                                            locals,
                                            Some(handler_scope),
                                            "enter",
                                        )
                                    })
                                    .unwrap_or_default();

                                let state_literal = self
                                    .intern_string(&state_context_node.state_ref_node.name, false);
                                let literal_ptr = self.next_temp();
                                self.push_line(&format!(
                                    "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                                    literal_ptr,
                                    state_literal.len,
                                    state_literal.len,
                                    state_literal.name
                                ));
                                self.push_line(&format!(
                                    "call void @frame_runtime_kernel_set_state(ptr {}, ptr {})",
                                    kernel_ptr, literal_ptr
                                ));

                                let state_literal_again = self
                                    .intern_string(&state_context_node.state_ref_node.name, false);
                                let new_state_cstr = self.next_temp();
                                self.push_line(&format!(
                                    "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                                    new_state_cstr,
                                    state_literal_again.len,
                                    state_literal_again.len,
                                    state_literal_again.name
                                ));
                                let next_compartment = self.next_temp();
                                self.push_line(&format!(
                                    "{} = call ptr @frame_runtime_compartment_new(ptr {})",
                                    next_compartment, new_state_cstr
                                ));
                                self.push_line(&format!(
                                    "call void @frame_runtime_compartment_set_enter_event(ptr {}, ptr null)",
                                    next_compartment
                                ));
                                self.push_line(&format!(
                                    "call void @frame_runtime_compartment_set_exit_event(ptr {}, ptr null)",
                                    next_compartment
                                ));

                                for (literal, value) in pending_state_args {
                                    let StringRef {
                                        name: key_name,
                                        len: key_len,
                                    } = literal;
                                    let ValueRef {
                                        kind,
                                        value: value_reg,
                                    } = value;
                                    let key_ptr = self.next_temp();
                                    self.push_line(&format!(
                                        "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                                        key_ptr, key_len, key_len, key_name
                                    ));
                                    match kind {
                                        ValueKind::I32 => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_state_arg_set_i32(ptr {}, ptr {}, i32 {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                        ValueKind::Double => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_state_arg_set_double(ptr {}, ptr {}, double {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                        ValueKind::Bool => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_state_arg_set_bool(ptr {}, ptr {}, i1 {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                        ValueKind::CString => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_state_arg_set_cstring(ptr {}, ptr {}, ptr {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                    }
                                }

                                for (literal, value) in pending_enter_args {
                                    let StringRef {
                                        name: key_name,
                                        len: key_len,
                                    } = literal;
                                    let ValueRef {
                                        kind,
                                        value: value_reg,
                                    } = value;
                                    let key_ptr = self.next_temp();
                                    self.push_line(&format!(
                                        "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                                        key_ptr, key_len, key_len, key_name
                                    ));
                                    match kind {
                                        ValueKind::I32 => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_enter_arg_set_i32(ptr {}, ptr {}, i32 {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                        ValueKind::Double => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_enter_arg_set_double(ptr {}, ptr {}, double {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                        ValueKind::Bool => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_enter_arg_set_bool(ptr {}, ptr {}, i1 {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                        ValueKind::CString => {
                                            self.push_line(&format!(
                                                "call void @frame_runtime_compartment_enter_arg_set_cstring(ptr {}, ptr {}, ptr {})",
                                                next_compartment, key_ptr, value_reg
                                            ));
                                        }
                                    }
                                }

                                let active_compartment = self.next_temp();
                                self.push_line(&format!(
                                    "{} = call ptr @frame_runtime_kernel_push_compartment(ptr {}, ptr {})",
                                    active_compartment, kernel_ptr, next_compartment
                                ));
                                self.push_line(&format!(
                                    "store ptr {}, ptr {}",
                                    active_compartment, compartment_field_ptr
                                ));

                                if target_state.enter_handler.is_some() {
                                    let enter_fn = ctx.state_enter_fn(&target_state.name);
                                    self.push_line(&format!(
                                        "call void {}({}* %self)",
                                        enter_fn, ctx.struct_name
                                    ));
                                }
                            } else {
                                self.push_comment("unsupported transition target");
                            }
                        } else {
                            self.push_comment("unsupported transition");
                        }
                    }
                    StatementType::StateStackStmt {
                        state_stack_operation_statement_node,
                    } => {
                        match state_stack_operation_statement_node
                            .state_stack_operation_node
                            .operation_t
                        {
                            StateStackOperationType::Push => {
                                self.require_runtime_api();
                                let current_state_val = self.next_temp();
                                self.push_line(&format!(
                                    "{} = load i32, i32* {}",
                                    current_state_val, state_ptr
                                ));
                                self.push_line(&format!(
                                    "call void @frame_runtime_kernel_state_stack_push(ptr {}, i32 {})",
                                    kernel_ptr, current_state_val
                                ));
                            }
                            StateStackOperationType::Pop => {
                                self.emit_state_stack_pop(
                                    ctx,
                                    kernel_ptr,
                                    state_ptr,
                                    compartment_field_ptr.as_str(),
                                    queue_loop_label.as_str(),
                                    current_state_index,
                                    method_ident,
                                );
                                continue 'stmt_loop;
                            }
                        }
                    }
                    StatementType::ParentDispatchStmt { .. } => {
                        if let Some(parent_name) = parent_state_name {
                            let current_compartment = self.next_temp();
                            self.push_line(&format!(
                                "{} = load ptr, ptr {}",
                                current_compartment, compartment_field_ptr
                            ));
                            let parent_compartment = self.next_temp();
                            self.push_line(&format!(
                                "{} = call ptr @frame_runtime_compartment_get_parent(ptr {})",
                                parent_compartment, current_compartment
                            ));
                            let has_parent = self.next_temp();
                            self.push_line(&format!(
                                "{} = icmp ne ptr {}, null",
                                has_parent, parent_compartment
                            ));
                            let enqueue_label = format!(
                                "{}_{}_parent_dispatch_enqueue_{}",
                                ctx.sanitized_name, method_ident, current_state_index
                            );
                            let missing_label = format!(
                                "{}_{}_parent_dispatch_missing_{}",
                                ctx.sanitized_name, method_ident, current_state_index
                            );
                            self.push_line(&format!(
                                "br i1 {}, label %{}, label %{}",
                                has_parent, enqueue_label, missing_label
                            ));

                            self.push_line(&format!("{}:", enqueue_label));
                            self.indent += 1;
                            let parent_literal = self.intern_string(event_name, false);
                            let parent_literal_ptr = self.next_temp();
                            self.push_line(&format!(
                                "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                                parent_literal_ptr,
                                parent_literal.len,
                                parent_literal.len,
                                parent_literal.name
                            ));
                            let forwarded_event = self.next_temp();
                            self.push_line(&format!(
                                "{} = call ptr @frame_runtime_event_new(ptr {})",
                                forwarded_event, parent_literal_ptr
                            ));
                            if let Some(locals_map) = locals {
                                if !handler_scope.event_params.is_empty() {
                                    let param_specs: Vec<(String, ValueKind)> = handler_scope
                                        .event_params
                                        .iter()
                                        .map(|param| (param.name.clone(), param.kind))
                                        .collect();
                                    self.push_event_params_from_locals(
                                        &forwarded_event,
                                        &param_specs,
                                        locals_map,
                                    );
                                }
                            } else if !handler_scope.event_params.is_empty() {
                                self.push_comment(
                                    "event parameters unavailable for parent dispatch forward",
                                );
                            }
                            self.push_line(&format!(
                                "call void @frame_runtime_compartment_set_forward_event(ptr {}, ptr {})",
                                parent_compartment, forwarded_event
                            ));
                            if let Some(parent_index) = ctx.state_index(parent_name) {
                                self.push_line(&format!(
                                    "store i32 {}, i32* {}",
                                    parent_index, state_ptr
                                ));
                            }
                            self.push_line(&format!(
                                "store ptr {}, ptr {}",
                                parent_compartment, compartment_field_ptr
                            ));
                            self.push_line(&format!("br label %{}", queue_loop_label));
                            self.indent -= 1;

                            self.push_line(&format!("{}:", missing_label));
                            self.indent += 1;
                            self.push_comment("parent dispatch ignored: state has no parent");
                            self.push_line(&format!("br label %{}", queue_loop_label));
                            self.indent -= 1;
                            continue 'stmt_loop;
                        } else {
                            self.push_comment("parent dispatch ignored: state has no parent");
                        }
                    }
                    _ => {
                        self.push_comment("unsupported statement in handler");
                    }
                }
            } else {
                self.push_comment("variable declarations not supported in LLVM backend yet");
            }
        }

        // If no transitions occurred, state remains — ensure explicit branch to end
        let _ = current_state_index; // reserved for future use

        self.push_line(&format!("br label %{}", queue_loop_label));
        self.push_line(&format!("{}:", queue_loop_label));
        self.indent += 1;
        let queue_event_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = call ptr @frame_runtime_kernel_next_event(ptr {})",
            queue_event_ptr, kernel_ptr
        ));
        let queue_has_event = self.next_temp();
        self.push_line(&format!(
            "{} = icmp ne ptr {}, null",
            queue_has_event, queue_event_ptr
        ));
        self.push_line(&format!(
            "br i1 {}, label %{}, label %{}",
            queue_has_event, queue_check_label, queue_exit_label
        ));
        self.indent -= 1;

        if !ctx.interface_methods().is_empty() {
            let mut check_labels = Vec::new();
            for (idx, _) in ctx.interface_methods().iter().enumerate() {
                if idx == 0 {
                    check_labels.push(queue_check_label.clone());
                } else {
                    check_labels.push(format!(
                        "{}_queue_check_next_{}_{}",
                        ctx.sanitized_name, current_state_index, idx
                    ));
                }
            }

            for (idx, method) in ctx.interface_methods().iter().enumerate() {
                let check_label = check_labels[idx].clone();
                if idx == 0 {
                    self.push_line(&format!("{}:", check_label));
                } else {
                    self.push_line(&format!("{}:", check_label));
                }
                self.indent += 1;

                let literal = self.intern_string(&method.message, false);
                let literal_ptr = self.next_temp();
                self.push_line(&format!(
                    "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    literal_ptr, literal.len, literal.len, literal.name
                ));
                let matches_ptr = self.next_temp();
                self.push_line(&format!(
                    "{} = call i1 @frame_runtime_event_is_message(ptr {}, ptr {})",
                    matches_ptr, queue_event_ptr, literal_ptr
                ));

                let match_label = format!(
                    "{}_queue_match_{}_{}",
                    ctx.sanitized_name, current_state_index, idx
                );
                let next_label = if idx + 1 < check_labels.len() {
                    check_labels[idx + 1].clone()
                } else {
                    queue_default_label.clone()
                };

                self.push_line(&format!(
                    "br i1 {}, label %{}, label %{}",
                    matches_ptr, match_label, next_label
                ));
                self.indent -= 1;

                self.push_line(&format!("{}:", match_label));
                self.indent += 1;
                let mut call_args = Vec::new();
                call_args.push(format!("{}* %self", ctx.struct_name));
                if let Some(method_info) = ctx.interface_method_by_message(&method.message) {
                    for (param_index, param) in method_info.params.iter().enumerate() {
                        let value = self.load_event_param_value(
                            queue_event_ptr.as_str(),
                            param_index,
                            param.kind,
                        );
                        call_args.push(format!(
                            "{} {}",
                            Self::llvm_type_for_kind(param.kind),
                            value.value
                        ));
                    }
                } else {
                    self.push_comment("interface method metadata missing for queued event");
                }
                self.push_line(&format!(
                    "call void {}({})",
                    method.fn_name,
                    call_args.join(", ")
                ));
                let restored_ptr = self.next_temp();
                self.push_line(&format!(
                    "{} = call ptr @frame_runtime_kernel_pop_compartment(ptr {})",
                    restored_ptr, kernel_ptr
                ));
                let has_restored = self.next_temp();
                self.push_line(&format!(
                    "{} = icmp ne ptr {}, null",
                    has_restored, restored_ptr
                ));
                let restored_or_backup = self.next_temp();
                self.push_line(&format!(
                    "{} = select i1 {}, ptr {}, ptr {}",
                    restored_or_backup, has_restored, restored_ptr, child_comp_backup
                ));
                self.push_line(&format!(
                    "store ptr {}, ptr {}",
                    restored_or_backup, compartment_field_ptr
                ));
                self.push_line(&format!(
                    "store i32 {}, i32* {}",
                    child_state_backup, state_ptr
                ));
                self.push_line(&format!(
                    "call void @frame_runtime_event_free(ptr {})",
                    queue_event_ptr
                ));
                self.push_line(&format!("br label %{}", queue_loop_label));
                self.indent -= 1;
            }
        }

        self.push_line(&format!("{}:", queue_default_label));
        self.indent += 1;
        self.push_comment("no matching interface method for queued event");
        self.push_line(&format!(
            "call void @frame_runtime_event_free(ptr {})",
            queue_event_ptr
        ));
        self.push_line(&format!("br label %{}", queue_loop_label));
        self.indent -= 1;

        self.push_line(&format!("{}:", queue_exit_label));
        self.indent += 1;
        self.push_line(&format!("br label %{}", end_label));
        self.indent -= 1;
    }

    fn emit_expression_statement(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        expr_stmt: &ExprStmtType,
        locals: Option<&HashMap<String, LocalBinding>>,
        scope: Option<HandlerScope>,
    ) {
        match expr_stmt {
            ExprStmtType::CallStmtT { call_stmt_node } => {
                if self.handle_call_expr(
                    Some(ctx),
                    Some(self_ptr),
                    locals,
                    &call_stmt_node.call_expr_node,
                    scope,
                ) {
                    return;
                }
                self.push_comment("unsupported call expression");
            }
            ExprStmtType::CallChainStmtT {
                call_chain_literal_stmt_node,
            } => {
                let call_chain = &call_chain_literal_stmt_node.call_chain_literal_expr_node;
                if let Some(node) = call_chain.call_chain.front() {
                    if let CallChainNodeType::UndeclaredCallT { call_node } = node {
                        if self.handle_call_expr(
                            Some(ctx),
                            Some(self_ptr),
                            locals,
                            call_node,
                            scope,
                        ) {
                            return;
                        }
                    }
                }
                self.push_comment("unsupported call chain expression");
            }
            ExprStmtType::AssignmentStmtT {
                assignment_stmt_node,
            } => self.emit_assignment_statement(ctx, self_ptr, locals, assignment_stmt_node, scope),
            _ => {
                self.push_comment("unsupported expression statement");
            }
        }
    }

    fn emit_assignment_statement(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        locals: Option<&HashMap<String, LocalBinding>>,
        stmt: &AssignmentStmtNode,
        scope: Option<HandlerScope>,
    ) {
        let expr = &stmt.assignment_expr_node;

        if expr.is_multiple_assignment {
            self.push_comment("multiple assignment not yet supported in LLVM backend");
            return;
        }

        if !matches!(expr.assignment_op, AssignmentOperator::Equals) {
            self.push_comment("compound assignments not yet supported in LLVM backend");
            return;
        }

        let mut target_local: Option<&LocalBinding> = None;
        let mut target_field: Option<&DomainField> = None;

        match &*expr.l_value_box {
            ExprType::VariableExprT { var_node } => {
                let name = var_node.get_name();
                if let Some(locals_map) = locals {
                    if let Some(binding) = locals_map.get(name) {
                        target_local = Some(binding);
                    } else if let Some(field) = ctx.domain_field(name) {
                        target_field = Some(field);
                    } else {
                        self.push_comment(
                            "assignment target not found in locals or domain variables",
                        );
                        return;
                    }
                } else if let Some(field) = ctx.domain_field(name) {
                    target_field = Some(field);
                } else {
                    self.push_comment(
                        "assignment to non-domain variables not yet supported in LLVM backend",
                    );
                    return;
                }
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                if let Some(field) =
                    self.domain_field_from_call_chain(ctx, &call_chain_expr_node.call_chain)
                {
                    target_field = Some(field);
                } else {
                    self.push_comment(&format!(
                        "unsupported call chain assignment target in LLVM backend: {}",
                        self.describe_call_chain(&call_chain_expr_node.call_chain)
                    ));
                    return;
                }
            }
            _ => {
                self.push_comment("unsupported assignment target in LLVM backend");
                return;
            }
        }

        let value =
            match self.emit_expression_value(ctx, self_ptr, locals, &*expr.r_value_rc, scope) {
                Some(value) => value,
                None => {
                    self.push_comment("unsupported assignment expression in LLVM backend");
                    return;
                }
            };

        if let Some(binding) = target_local {
            let coerced = match self.coerce_value_for_kind(value, binding.kind) {
                Some(value) => value,
                None => {
                    self.push_comment("assignment type mismatch for local variable");
                    return;
                }
            };
            self.store_local_value(binding, coerced);
            return;
        }

        let field = match target_field {
            Some(field) => field,
            None => {
                self.push_comment("assignment target could not be resolved");
                return;
            }
        };

        let coerced = match self.coerce_value_for_kind(value, field.field_type.value_kind()) {
            Some(value) => value,
            None => {
                self.push_comment("assignment type mismatch in LLVM backend");
                return;
            }
        };

        self.store_domain_field(ctx, self_ptr, field, coerced);
    }

    fn emit_basic_handler(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        handler: &EventHandlerNode,
        scope: Option<HandlerScope>,
    ) {
        for stmt in &handler.statements {
            match stmt {
                DeclOrStmtType::StmtT { stmt_t } => match stmt_t {
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        self.emit_expression_statement(ctx, self_ptr, expr_stmt_t, None, scope);
                    }
                    StatementType::ReturnStmt { .. } | StatementType::NoStmt => {
                        // Exit/enter handlers commonly end with return; nothing to emit.
                    }
                    _ => {
                        self.push_comment("unsupported statement in enter/exit handler");
                    }
                },
                DeclOrStmtType::VarDeclT { .. } => {
                    self.push_comment("local variables unsupported in enter/exit handler");
                }
            }
        }
    }

    fn emit_state_stack_pop(
        &mut self,
        ctx: &SystemEmitContext,
        kernel_ptr: &str,
        state_ptr: &str,
        compartment_field_ptr: &str,
        queue_loop_label: &str,
        current_state_index: i32,
        method_ident: &str,
    ) {
        self.require_runtime_api();
        let popped_compartment = self.next_temp();
        self.push_line(&format!(
            "{} = call ptr @frame_runtime_kernel_state_stack_pop(ptr {}, i32* {})",
            popped_compartment, kernel_ptr, state_ptr
        ));
        let has_popped = self.next_temp();
        self.push_line(&format!(
            "{} = icmp ne ptr {}, null",
            has_popped, popped_compartment
        ));
        let pop_success_label = format!(
            "{}_{}_stack_pop_success_{}",
            ctx.sanitized_name, method_ident, current_state_index
        );
        let pop_empty_label = format!(
            "{}_{}_stack_pop_empty_{}",
            ctx.sanitized_name, method_ident, current_state_index
        );
        let pop_resume_label = format!(
            "{}_{}_stack_pop_resume_{}",
            ctx.sanitized_name, method_ident, current_state_index
        );
        self.push_line(&format!(
            "br i1 {}, label %{}, label %{}",
            has_popped, pop_success_label, pop_empty_label
        ));
        self.push_blank_line();

        self.push_line(&format!("{}:", pop_success_label));
        self.indent += 1;
        let current_state = ctx.state(current_state_index as usize);
        if current_state.exit_handler.is_some() {
            let exit_fn = ctx.state_exit_fn(&current_state.name);
            self.push_line(&format!(
                "call void {}({}* %self)",
                exit_fn, ctx.struct_name
            ));
        }
        self.push_line(&format!(
            "store ptr {}, ptr {}",
            popped_compartment, compartment_field_ptr
        ));
        let popped_state_val = self.next_temp();
        self.push_line(&format!(
            "{} = load i32, i32* {}",
            popped_state_val, state_ptr
        ));
        self.push_line(&format!(
            "switch i32 {}, label %{} [",
            popped_state_val, pop_resume_label
        ));
        self.indent += 1;
        for (idx, state_entry) in ctx.states.iter().enumerate() {
            let state_case_label = format!(
                "{}_{}_stack_pop_state_{}_{}_{}",
                ctx.sanitized_name,
                method_ident,
                current_state_index,
                idx,
                sanitize_identifier(&state_entry.name)
            );
            self.push_line(&format!("i32 {}, label %{}", idx, state_case_label));
        }
        self.indent -= 1;
        self.push_line("]");
        self.push_blank_line();

        for (idx, state_entry) in ctx.states.iter().enumerate() {
            let state_case_label = format!(
                "{}_{}_stack_pop_state_{}_{}_{}",
                ctx.sanitized_name,
                method_ident,
                current_state_index,
                idx,
                sanitize_identifier(&state_entry.name)
            );
            self.push_line(&format!("{}:", state_case_label));
            self.indent += 1;
            let state_literal = self.intern_string(&state_entry.name, false);
            let literal_ptr = self.next_temp();
            self.push_line(&format!(
                "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                literal_ptr, state_literal.len, state_literal.len, state_literal.name
            ));
            self.push_line(&format!(
                "call void @frame_runtime_kernel_set_state(ptr {}, ptr {})",
                kernel_ptr, literal_ptr
            ));
            if state_entry.enter_handler.is_some() {
                let enter_fn = ctx.state_enter_fn(&state_entry.name);
                self.push_line(&format!(
                    "call void {}({}* %self)",
                    enter_fn, ctx.struct_name
                ));
            }
            self.push_line(&format!("br label %{}", pop_resume_label));
            self.indent -= 1;
        }

        self.push_line(&format!("{}:", pop_resume_label));
        self.indent += 1;
        self.push_line(&format!("br label %{}", queue_loop_label));
        self.indent -= 1;
        self.push_blank_line();

        self.push_line(&format!("{}:", pop_empty_label));
        self.indent += 1;
        self.push_comment("state stack pop ignored: stack empty");
        self.push_line(&format!("br label %{}", queue_loop_label));
        self.indent -= 1;
        self.push_blank_line();
    }

    fn handle_call_expr(
        &mut self,
        ctx: Option<&SystemEmitContext>,
        self_ptr: Option<&str>,
        locals: Option<&HashMap<String, LocalBinding>>,
        call: &CallExprNode,
        scope: Option<HandlerScope>,
    ) -> bool {
        let func_name = call.get_name();
        if func_name == "print" {
            if call.call_expr_list.exprs_t.len() != 1 {
                self.push_comment("print expects a single argument");
                return true;
            }
            let arg = &call.call_expr_list.exprs_t[0];

            if let (Some(ctx_eval), Some(self_ptr_eval)) = (ctx, self_ptr) {
                if let Some(value) =
                    self.emit_expression_value(ctx_eval, self_ptr_eval, locals, arg, scope)
                {
                    match value.kind {
                        ValueKind::I32 => {
                            self.require_print_int();
                            self.push_line(&format!(
                                "call void @frame_runtime_print_int(i32 {})",
                                value.value
                            ));
                            return true;
                        }
                        ValueKind::Double => {
                            self.require_print_double();
                            self.push_line(&format!(
                                "call void @frame_runtime_print_double(double {})",
                                value.value
                            ));
                            return true;
                        }
                        ValueKind::Bool => {
                            self.require_print_bool();
                            self.push_line(&format!(
                                "call void @frame_runtime_print_bool(i1 {})",
                                value.value
                            ));
                            return true;
                        }
                        ValueKind::CString => {
                            self.require_puts();
                            self.push_line(&format!("call i32 @puts(i8* {})", value.value));
                            return true;
                        }
                    }
                }
            }

            if let Some(binding) = match arg {
                ExprType::VariableExprT { var_node } => {
                    locals.and_then(|map| map.get(var_node.get_name()))
                }
                ExprType::CallChainExprT {
                    call_chain_expr_node,
                } => self.local_binding_from_call_chain(locals, &call_chain_expr_node.call_chain),
                _ => None,
            } {
                if binding.kind == ValueKind::CString {
                    let loaded = self.load_local_value(binding);
                    self.require_puts();
                    self.push_line(&format!("call i32 @puts(i8* {})", loaded.value));
                    return true;
                }
            }

            if let Some(text) = extract_string_literal(arg) {
                let literal = self.intern_string(&text, false);
                let tmp = self.next_temp();
                self.push_line(&format!(
                    "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    tmp, literal.len, literal.len, literal.name
                ));
                self.require_puts();
                self.push_line(&format!("call i32 @puts(i8* {})", tmp));
                return true;
            }

            if let (Some(ctx_domain), Some(self_ptr_domain)) = (ctx, self_ptr) {
                if let Some(field) = match arg {
                    ExprType::VariableExprT { var_node } => {
                        ctx_domain.domain_field(var_node.get_name())
                    }
                    ExprType::CallChainExprT {
                        call_chain_expr_node,
                    } => self
                        .domain_field_from_call_chain(ctx_domain, &call_chain_expr_node.call_chain),
                    _ => None,
                } {
                    let field_ptr = self.next_temp();
                    self.push_line(&format!(
                        "{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                        field_ptr,
                        ctx_domain.struct_name,
                        ctx_domain.struct_name,
                        self_ptr_domain,
                        field.struct_index
                    ));
                    match field.field_type {
                        DomainFieldType::I32 => {
                            let loaded = self.next_temp();
                            self.push_line(&format!("{} = load i32, i32* {}", loaded, field_ptr));
                            self.require_print_int();
                            self.push_line(&format!(
                                "call void @frame_runtime_print_int(i32 {})",
                                loaded
                            ));
                            return true;
                        }
                        DomainFieldType::F64 => {
                            let loaded = self.next_temp();
                            self.push_line(&format!(
                                "{} = load double, double* {}",
                                loaded, field_ptr
                            ));
                            self.require_print_double();
                            self.push_line(&format!(
                                "call void @frame_runtime_print_double(double {})",
                                loaded
                            ));
                            return true;
                        }
                        DomainFieldType::Bool => {
                            let loaded = self.next_temp();
                            self.push_line(&format!("{} = load i1, i1* {}", loaded, field_ptr));
                            self.require_print_bool();
                            self.push_line(&format!(
                                "call void @frame_runtime_print_bool(i1 {})",
                                loaded
                            ));
                            return true;
                        }
                        DomainFieldType::CString => {
                            let loaded = self.next_temp();
                            self.push_line(&format!("{} = load i8*, i8** {}", loaded, field_ptr));
                            self.require_puts();
                            self.push_line(&format!("call i32 @puts(i8* {})", loaded));
                            return true;
                        }
                    }
                }
            }

            self.push_comment("unsupported print arguments");
            return true;
        }

        if let (Some(ctx), Some(self_ptr)) = (ctx, self_ptr) {
            if let Some(action) = ctx.action(func_name) {
                if call.call_expr_list.exprs_t.len() != action.params.len() {
                    self.push_comment("action argument count mismatch");
                    return true;
                }

                let locals_map = locals;
                let mut arg_strings = Vec::new();

                for (param, expr) in action.params.iter().zip(call.call_expr_list.exprs_t.iter()) {
                    let value =
                        match self.emit_expression_value(ctx, self_ptr, locals_map, expr, scope) {
                            Some(value) => value,
                            None => {
                                self.push_comment("unsupported action argument expression");
                                return true;
                            }
                        };

                    let coerced = match self.coerce_value_for_kind(value, param.kind) {
                        Some(value) => value,
                        None => {
                            self.push_comment("action argument type mismatch");
                            return true;
                        }
                    };

                    arg_strings.push(format!(
                        "{} {}",
                        Self::llvm_type_for_kind(param.kind),
                        coerced.value
                    ));
                }

                let mut call_args = Vec::with_capacity(arg_strings.len() + 1);
                call_args.push(format!("{}* {}", ctx.struct_name, self_ptr));
                call_args.extend(arg_strings);

                self.push_line(&format!(
                    "call void {}({})",
                    action.fn_name,
                    call_args.join(", ")
                ));
                return true;
            }
        }
        false
    }

    fn emit_expression_value(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        locals: Option<&HashMap<String, LocalBinding>>,
        expr: &ExprType,
        scope: Option<HandlerScope>,
    ) -> Option<ValueRef> {
        match expr {
            ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t.clone()
            {
                TokenType::Number => {
                    let sanitized = sanitize_numeric_literal(&literal_expr_node.value);
                    if literal_expr_node.value.contains('.')
                        || literal_expr_node.value.contains('e')
                        || literal_expr_node.value.contains('E')
                    {
                        Some(ValueRef::new(ValueKind::Double, sanitized))
                    } else {
                        Some(ValueRef::new(ValueKind::I32, sanitized))
                    }
                }
                TokenType::True => Some(ValueRef::new(ValueKind::Bool, "1")),
                TokenType::False => Some(ValueRef::new(ValueKind::Bool, "0")),
                TokenType::String => {
                    let literal = self.intern_string(&literal_expr_node.value, false);
                    let ptr = self.next_temp();
                    self.push_line(&format!(
                        "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                        ptr, literal.len, literal.len, literal.name
                    ));
                    Some(ValueRef::new(ValueKind::CString, ptr))
                }
                _ => None,
            },
            ExprType::VariableExprT { var_node } => {
                if let Some(locals_map) = locals {
                    if let Some(binding) = locals_map.get(var_node.get_name()) {
                        return Some(self.load_local_value(binding));
                    }
                }
                if let Some(scope_ctx) = scope {
                    if let Some(value) =
                        self.load_scoped_variable(ctx, self_ptr, scope_ctx, var_node.get_name())
                    {
                        return Some(value);
                    }
                }
                if let Some(field) = ctx.domain_field(var_node.get_name()) {
                    let field_ptr = self.domain_field_ptr(ctx, self_ptr, field);
                    let loaded = self.next_temp();
                    match field.field_type {
                        DomainFieldType::I32 => {
                            self.push_line(&format!("{} = load i32, i32* {}", loaded, field_ptr));
                            Some(ValueRef::new(ValueKind::I32, loaded))
                        }
                        DomainFieldType::F64 => {
                            self.push_line(&format!(
                                "{} = load double, double* {}",
                                loaded, field_ptr
                            ));
                            Some(ValueRef::new(ValueKind::Double, loaded))
                        }
                        DomainFieldType::Bool => {
                            self.push_line(&format!("{} = load i1, i1* {}", loaded, field_ptr));
                            Some(ValueRef::new(ValueKind::Bool, loaded))
                        }
                        DomainFieldType::CString => {
                            self.push_line(&format!("{} = load i8*, i8** {}", loaded, field_ptr));
                            Some(ValueRef::new(ValueKind::CString, loaded))
                        }
                    }
                } else {
                    None
                }
            }
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                if let Some(binding) =
                    self.local_binding_from_call_chain(locals, &call_chain_expr_node.call_chain)
                {
                    return Some(self.load_local_value(binding));
                }
                self.domain_field_from_call_chain(ctx, &call_chain_expr_node.call_chain)
                    .map(|field| {
                        let field_ptr = self.domain_field_ptr(ctx, self_ptr, field);
                        let loaded = self.next_temp();
                        match field.field_type {
                            DomainFieldType::I32 => {
                                self.push_line(&format!(
                                    "{} = load i32, i32* {}",
                                    loaded, field_ptr
                                ));
                                ValueRef::new(ValueKind::I32, loaded)
                            }
                            DomainFieldType::F64 => {
                                self.push_line(&format!(
                                    "{} = load double, double* {}",
                                    loaded, field_ptr
                                ));
                                ValueRef::new(ValueKind::Double, loaded)
                            }
                            DomainFieldType::Bool => {
                                self.push_line(&format!("{} = load i1, i1* {}", loaded, field_ptr));
                                ValueRef::new(ValueKind::Bool, loaded)
                            }
                            DomainFieldType::CString => {
                                self.push_line(&format!(
                                    "{} = load i8*, i8** {}",
                                    loaded, field_ptr
                                ));
                                ValueRef::new(ValueKind::CString, loaded)
                            }
                        }
                    })
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                let left_ref = binary_expr_node.left_rcref.borrow();
                let left = self.emit_expression_value(ctx, self_ptr, locals, &*left_ref, scope)?;
                drop(left_ref);
                let right_ref = binary_expr_node.right_rcref.borrow();
                let right =
                    self.emit_expression_value(ctx, self_ptr, locals, &*right_ref, scope)?;
                drop(right_ref);
                match (binary_expr_node.operator.clone(), left.kind, right.kind) {
                    (OperatorType::Plus, ValueKind::I32, ValueKind::I32) => {
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = add i32 {}, {}",
                            result, left.value, right.value
                        ));
                        Some(ValueRef::new(ValueKind::I32, result))
                    }
                    (OperatorType::Plus, ValueKind::Double, ValueKind::Double) => {
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = fadd double {}, {}",
                            result, left.value, right.value
                        ));
                        Some(ValueRef::new(ValueKind::Double, result))
                    }
                    (OperatorType::Plus, ValueKind::I32, ValueKind::Double) => {
                        let left_conv = self.convert_i32_to_double(left);
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = fadd double {}, {}",
                            result, left_conv.value, right.value
                        ));
                        Some(ValueRef::new(ValueKind::Double, result))
                    }
                    (OperatorType::Plus, ValueKind::Double, ValueKind::I32) => {
                        let right_conv = self.convert_i32_to_double(right);
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = fadd double {}, {}",
                            result, left.value, right_conv.value
                        ));
                        Some(ValueRef::new(ValueKind::Double, result))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn load_local_value(&mut self, binding: &LocalBinding) -> ValueRef {
        let loaded = self.next_temp();
        match binding.kind {
            ValueKind::I32 => {
                self.push_line(&format!("{} = load i32, i32* {}", loaded, binding.ptr));
                ValueRef::new(ValueKind::I32, loaded)
            }
            ValueKind::Double => {
                self.push_line(&format!(
                    "{} = load double, double* {}",
                    loaded, binding.ptr
                ));
                ValueRef::new(ValueKind::Double, loaded)
            }
            ValueKind::Bool => {
                self.push_line(&format!("{} = load i1, i1* {}", loaded, binding.ptr));
                ValueRef::new(ValueKind::Bool, loaded)
            }
            ValueKind::CString => {
                self.push_line(&format!("{} = load i8*, i8** {}", loaded, binding.ptr));
                ValueRef::new(ValueKind::CString, loaded)
            }
        }
    }

    fn load_compartment_ptr(&mut self, ctx: &SystemEmitContext, self_ptr: &str) -> String {
        let field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
            field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            self_ptr,
            ctx.compartment_field_index()
        ));
        let compartment_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = load ptr, ptr {}",
            compartment_ptr, field_ptr
        ));
        compartment_ptr
    }

    fn load_state_arg(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        name: &str,
        kind: ValueKind,
    ) -> ValueRef {
        self.require_runtime_api();
        let literal = self.intern_string(name, false);
        let StringRef {
            name: literal_name,
            len: literal_len,
        } = literal;
        let key_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            key_ptr, literal_len, literal_len, literal_name
        ));
        let compartment_ptr = self.load_compartment_ptr(ctx, self_ptr);
        let value_reg = self.next_temp();
        match kind {
            ValueKind::I32 => {
                self.push_line(&format!(
                    "{} = call i32 @frame_runtime_compartment_state_arg_get_i32(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::I32, value_reg)
            }
            ValueKind::Double => {
                self.push_line(&format!(
                    "{} = call double @frame_runtime_compartment_state_arg_get_double(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::Double, value_reg)
            }
            ValueKind::Bool => {
                self.push_line(&format!(
                    "{} = call i1 @frame_runtime_compartment_state_arg_get_bool(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::Bool, value_reg)
            }
            ValueKind::CString => {
                self.push_line(&format!(
                    "{} = call ptr @frame_runtime_compartment_state_arg_get_cstring(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::CString, value_reg)
            }
        }
    }

    fn load_enter_arg(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        name: &str,
        kind: ValueKind,
    ) -> ValueRef {
        self.require_runtime_api();
        let literal = self.intern_string(name, false);
        let StringRef {
            name: literal_name,
            len: literal_len,
        } = literal;
        let key_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
            key_ptr, literal_len, literal_len, literal_name
        ));
        let compartment_ptr = self.load_compartment_ptr(ctx, self_ptr);
        let value_reg = self.next_temp();
        match kind {
            ValueKind::I32 => {
                self.push_line(&format!(
                    "{} = call i32 @frame_runtime_compartment_enter_arg_get_i32(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::I32, value_reg)
            }
            ValueKind::Double => {
                self.push_line(&format!(
                    "{} = call double @frame_runtime_compartment_enter_arg_get_double(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::Double, value_reg)
            }
            ValueKind::Bool => {
                self.push_line(&format!(
                    "{} = call i1 @frame_runtime_compartment_enter_arg_get_bool(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::Bool, value_reg)
            }
            ValueKind::CString => {
                self.push_line(&format!(
                    "{} = call ptr @frame_runtime_compartment_enter_arg_get_cstring(ptr {}, ptr {})",
                    value_reg, compartment_ptr, key_ptr
                ));
                ValueRef::new(ValueKind::CString, value_reg)
            }
        }
    }

    fn load_scoped_variable(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        scope: HandlerScope,
        name: &str,
    ) -> Option<ValueRef> {
        if scope.kind == HandlerKind::Enter {
            if let Some(param) = scope.state.enter_param(name) {
                return Some(self.load_enter_arg(ctx, self_ptr, &param.name, param.kind));
            }
        }
        if let Some(param) = scope.state.state_param(name) {
            return Some(self.load_state_arg(ctx, self_ptr, &param.name, param.kind));
        }
        if scope.kind == HandlerKind::Event {
            let _ = scope.event_param(name);
        }
        None
    }

    fn collect_pending_args(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        exprs: &ExprListNode,
        params: &[StateParam],
        locals: Option<&HashMap<String, LocalBinding>>,
        scope: Option<HandlerScope>,
        description: &str,
    ) -> Vec<(StringRef, ValueRef)> {
        if exprs.exprs_t.len() != params.len() {
            self.push_comment(&format!("{} argument count mismatch", description));
            return Vec::new();
        }

        let mut pending = Vec::new();
        for (param, expr) in params.iter().zip(exprs.exprs_t.iter()) {
            let value = match self.emit_expression_value(ctx, self_ptr, locals, expr, scope) {
                Some(value) => value,
                None => {
                    self.push_comment(&format!(
                        "unsupported {} expression for parameter '{}'",
                        description, param.name
                    ));
                    continue;
                }
            };

            let coerced = match self.coerce_value_for_kind(value, param.kind) {
                Some(value) => value,
                None => {
                    self.push_comment(&format!(
                        "type mismatch for {} parameter '{}'",
                        description, param.name
                    ));
                    continue;
                }
            };

            let literal = self.intern_string(&param.name, false);
            pending.push((literal, coerced));
        }

        pending
    }

    fn push_event_params_from_locals(
        &mut self,
        event_ptr: &str,
        params: &[(String, ValueKind)],
        locals: &HashMap<String, LocalBinding>,
    ) {
        if params.is_empty() {
            return;
        }
        self.require_runtime_event();
        for (name, kind) in params {
            match locals.get(name) {
                Some(binding) => {
                    self.push_event_param_from_binding(event_ptr, binding, *kind);
                }
                None => {
                    self.push_comment(&format!(
                        "event parameter '{}' not found in local bindings",
                        name
                    ));
                }
            }
        }
    }

    fn push_event_param_from_binding(
        &mut self,
        event_ptr: &str,
        binding: &LocalBinding,
        expected: ValueKind,
    ) {
        let mut value = self.load_local_value(binding);
        if binding.kind != expected {
            if let Some(coerced) = self.coerce_value_for_kind(value, expected) {
                value = coerced;
            } else {
                self.push_comment("event parameter type mismatch in binding");
                return;
            }
        }
        self.push_event_param_value(event_ptr, expected, value);
    }

    fn push_event_param_value(&mut self, event_ptr: &str, kind: ValueKind, value: ValueRef) {
        self.require_runtime_event();
        match kind {
            ValueKind::I32 => {
                self.push_line(&format!(
                    "call void @frame_runtime_event_push_param_i32(ptr {}, i32 {})",
                    event_ptr, value.value
                ));
            }
            ValueKind::Double => {
                self.push_line(&format!(
                    "call void @frame_runtime_event_push_param_double(ptr {}, double {})",
                    event_ptr, value.value
                ));
            }
            ValueKind::Bool => {
                self.push_line(&format!(
                    "call void @frame_runtime_event_push_param_bool(ptr {}, i1 {})",
                    event_ptr, value.value
                ));
            }
            ValueKind::CString => {
                self.push_line(&format!(
                    "call void @frame_runtime_event_push_param_cstring(ptr {}, ptr {})",
                    event_ptr, value.value
                ));
            }
        }
    }

    fn load_event_param_value(
        &mut self,
        event_ptr: &str,
        index: usize,
        kind: ValueKind,
    ) -> ValueRef {
        self.require_runtime_event();
        let reg = self.next_temp();
        match kind {
            ValueKind::I32 => {
                self.push_line(&format!(
                    "{} = call i32 @frame_runtime_event_get_param_i32(ptr {}, i32 {})",
                    reg, event_ptr, index
                ));
                ValueRef::new(ValueKind::I32, reg)
            }
            ValueKind::Double => {
                self.push_line(&format!(
                    "{} = call double @frame_runtime_event_get_param_double(ptr {}, i32 {})",
                    reg, event_ptr, index
                ));
                ValueRef::new(ValueKind::Double, reg)
            }
            ValueKind::Bool => {
                self.push_line(&format!(
                    "{} = call i1 @frame_runtime_event_get_param_bool(ptr {}, i32 {})",
                    reg, event_ptr, index
                ));
                ValueRef::new(ValueKind::Bool, reg)
            }
            ValueKind::CString => {
                self.push_line(&format!(
                    "{} = call ptr @frame_runtime_event_get_param_cstring(ptr {}, i32 {})",
                    reg, event_ptr, index
                ));
                ValueRef::new(ValueKind::CString, reg)
            }
        }
    }

    fn local_binding_from_call_chain<'a>(
        &self,
        locals: Option<&'a HashMap<String, LocalBinding>>,
        call_chain: &VecDeque<CallChainNodeType>,
    ) -> Option<&'a LocalBinding> {
        let locals_map = locals?;
        let mut iter = call_chain.iter();
        match (iter.next(), iter.next(), iter.next()) {
            (Some(CallChainNodeType::VariableNodeT { var_node }), None, None) => {
                locals_map.get(var_node.get_name())
            }
            (Some(CallChainNodeType::UndeclaredIdentifierNodeT { id_node }), None, None) => {
                locals_map.get(id_node.name.lexeme.as_str())
            }
            _ => None,
        }
    }

    fn coerce_value_for_kind(&mut self, value: ValueRef, target: ValueKind) -> Option<ValueRef> {
        if value.kind == target {
            return Some(value);
        }

        match (target, value.kind) {
            (ValueKind::I32, ValueKind::Bool) => Some(self.convert_bool_to_i32(value)),
            (ValueKind::Double, ValueKind::I32) => Some(self.convert_i32_to_double(value)),
            (ValueKind::Double, ValueKind::Bool) => Some(self.convert_bool_to_double(value)),
            (ValueKind::Bool, ValueKind::I32) => Some(self.convert_i32_to_bool(value)),
            _ => None,
        }
    }

    fn domain_field_ptr(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        field: &DomainField,
    ) -> String {
        let ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
            ptr, ctx.struct_name, ctx.struct_name, self_ptr, field.struct_index
        ));
        ptr
    }

    fn store_domain_field(
        &mut self,
        ctx: &SystemEmitContext,
        self_ptr: &str,
        field: &DomainField,
        value: ValueRef,
    ) {
        let field_ptr = self.domain_field_ptr(ctx, self_ptr, field);
        match field.field_type {
            DomainFieldType::I32 => {
                debug_assert_eq!(value.kind, ValueKind::I32);
                self.push_line(&format!("store i32 {}, i32* {}", value.value, field_ptr));
            }
            DomainFieldType::F64 => {
                debug_assert_eq!(value.kind, ValueKind::Double);
                self.push_line(&format!(
                    "store double {}, double* {}",
                    value.value, field_ptr
                ));
            }
            DomainFieldType::Bool => {
                debug_assert_eq!(value.kind, ValueKind::Bool);
                self.push_line(&format!("store i1 {}, i1* {}", value.value, field_ptr));
            }
            DomainFieldType::CString => {
                debug_assert_eq!(value.kind, ValueKind::CString);
                self.push_line(&format!("store i8* {}, i8** {}", value.value, field_ptr));
            }
        }
    }

    fn store_local_value(&mut self, binding: &LocalBinding, value: ValueRef) {
        debug_assert_eq!(binding.kind, value.kind);
        match binding.kind {
            ValueKind::I32 => {
                self.push_line(&format!("store i32 {}, i32* {}", value.value, binding.ptr));
            }
            ValueKind::Double => {
                self.push_line(&format!(
                    "store double {}, double* {}",
                    value.value, binding.ptr
                ));
            }
            ValueKind::Bool => {
                self.push_line(&format!("store i1 {}, i1* {}", value.value, binding.ptr));
            }
            ValueKind::CString => {
                self.push_line(&format!("store i8* {}, i8** {}", value.value, binding.ptr));
            }
        }
    }

    fn alloca_for_kind(&mut self, kind: ValueKind) -> String {
        let ptr = self.next_temp();
        self.push_line(&format!(
            "{} = alloca {}, align {}",
            ptr,
            Self::llvm_type_for_kind(kind),
            Self::align_for_kind(kind)
        ));
        ptr
    }

    fn llvm_type_for_kind(kind: ValueKind) -> &'static str {
        match kind {
            ValueKind::I32 => "i32",
            ValueKind::Double => "double",
            ValueKind::Bool => "i1",
            ValueKind::CString => "i8*",
        }
    }

    fn align_for_kind(kind: ValueKind) -> usize {
        match kind {
            ValueKind::I32 => 4,
            ValueKind::Double => 8,
            ValueKind::Bool => 1,
            ValueKind::CString => 8,
        }
    }

    fn domain_field_from_call_chain<'a>(
        &self,
        ctx: &'a SystemEmitContext,
        call_chain: &VecDeque<CallChainNodeType>,
    ) -> Option<&'a DomainField> {
        let mut iter = call_chain.iter();
        match (iter.next(), iter.next(), iter.next()) {
            (Some(node), None, None) => match node {
                CallChainNodeType::VariableNodeT { var_node } => {
                    ctx.domain_field(var_node.get_name())
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    ctx.domain_field(id_node.name.lexeme.as_str())
                }
                _ => None,
            },
            (Some(first), Some(second), None) => {
                if !self.call_chain_node_is_self(first) {
                    return None;
                }
                let field_name = match second {
                    CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                        id_node.name.lexeme.as_str()
                    }
                    CallChainNodeType::VariableNodeT { var_node } => var_node.get_name(),
                    _ => return None,
                };
                ctx.domain_field(field_name)
            }
            _ => None,
        }
    }

    fn call_chain_node_is_self(&self, node: &CallChainNodeType) -> bool {
        match node {
            CallChainNodeType::SelfT { .. } => true,
            CallChainNodeType::VariableNodeT { var_node } => var_node.get_name() == "self",
            CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                id_node.name.lexeme.as_str() == "self"
            }
            _ => false,
        }
    }

    fn describe_call_chain(&self, call_chain: &VecDeque<CallChainNodeType>) -> String {
        let mut parts = Vec::new();
        for node in call_chain {
            let label = match node {
                CallChainNodeType::SelfT { .. } => "Self".to_string(),
                CallChainNodeType::VariableNodeT { var_node } => {
                    format!("Var({})", var_node.get_name())
                }
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    format!("Ident({})", id_node.name.lexeme.as_str())
                }
                CallChainNodeType::CallChainLiteralExprT { .. } => "Literal".to_string(),
                CallChainNodeType::InterfaceMethodCallT { .. } => "InterfaceCall".to_string(),
                CallChainNodeType::ActionCallT { .. } => "ActionCall".to_string(),
                CallChainNodeType::OperationCallT { .. } => "OperationCall".to_string(),
                CallChainNodeType::OperationRefT { .. } => "OperationRef".to_string(),
                CallChainNodeType::ListElementNodeT { .. } => "ListElement".to_string(),
                CallChainNodeType::SliceNodeT { .. } => "Slice".to_string(),
                CallChainNodeType::UndeclaredCallT { .. } => "UndeclaredCall".to_string(),
                CallChainNodeType::UndeclaredListElementT { .. } => {
                    "UndeclaredListElem".to_string()
                }
                CallChainNodeType::UndeclaredSliceT { .. } => "UndeclaredSlice".to_string(),
            };
            parts.push(label);
        }
        parts.join(" -> ")
    }

    fn emit_state_enter_function(&mut self, ctx: &SystemEmitContext, state: &StateEntry) {
        if state.enter_handler.is_none() {
            return;
        }
        let fn_name = ctx.state_enter_fn(&state.name);
        if !self.generated_enter_handlers.insert(fn_name.clone()) {
            return;
        }
        self.begin_function();
        writeln!(
            &mut self.body,
            "define void {}({}* %self) {{",
            fn_name, ctx.struct_name
        )
        .unwrap();
        self.indent += 1;
        if let Some(handler_rc) = &state.enter_handler {
            let handler = handler_rc.borrow();
            let empty_params: &[HandlerParam] = &[];
            let handler_scope = HandlerScope {
                state,
                kind: HandlerKind::Enter,
                event_params: empty_params,
            };
            self.emit_basic_handler(ctx, "%self", &handler, Some(handler_scope));
        }
        self.require_runtime_api();
        let compartment_ptr = self.load_compartment_ptr(ctx, "%self");
        self.push_line(&format!(
            "call void @frame_runtime_compartment_enter_args_clear(ptr {})",
            compartment_ptr
        ));
        self.push_line("ret void");
        self.indent -= 1;
        self.push_line("}");
        self.push_blank_line();
    }

    fn emit_state_exit_function(&mut self, ctx: &SystemEmitContext, state: &StateEntry) {
        if state.exit_handler.is_none() {
            return;
        }
        let fn_name = ctx.state_exit_fn(&state.name);
        if !self.generated_exit_handlers.insert(fn_name.clone()) {
            return;
        }
        self.begin_function();
        writeln!(
            &mut self.body,
            "define void {}({}* %self) {{",
            fn_name, ctx.struct_name
        )
        .unwrap();
        self.indent += 1;
        if let Some(handler_rc) = &state.exit_handler {
            let handler = handler_rc.borrow();
            let empty_params: &[HandlerParam] = &[];
            let handler_scope = HandlerScope {
                state,
                kind: HandlerKind::Exit,
                event_params: empty_params,
            };
            self.emit_basic_handler(ctx, "%self", &handler, Some(handler_scope));
        }
        self.push_line("ret void");
        self.indent -= 1;
        self.push_line("}");
        self.push_blank_line();
    }

    fn convert_bool_to_i32(&mut self, value: ValueRef) -> ValueRef {
        debug_assert_eq!(value.kind, ValueKind::Bool);
        let tmp = self.next_temp();
        self.push_line(&format!("{} = zext i1 {} to i32", tmp, value.value));
        ValueRef::new(ValueKind::I32, tmp)
    }

    fn convert_i32_to_double(&mut self, value: ValueRef) -> ValueRef {
        debug_assert_eq!(value.kind, ValueKind::I32);
        let tmp = self.next_temp();
        self.push_line(&format!("{} = sitofp i32 {} to double", tmp, value.value));
        ValueRef::new(ValueKind::Double, tmp)
    }

    fn convert_bool_to_double(&mut self, value: ValueRef) -> ValueRef {
        debug_assert_eq!(value.kind, ValueKind::Bool);
        let tmp = self.next_temp();
        self.push_line(&format!("{} = uitofp i1 {} to double", tmp, value.value));
        ValueRef::new(ValueKind::Double, tmp)
    }

    fn convert_i32_to_bool(&mut self, value: ValueRef) -> ValueRef {
        debug_assert_eq!(value.kind, ValueKind::I32);
        let tmp = self.next_temp();
        self.push_line(&format!("{} = icmp ne i32 {}, 0", tmp, value.value));
        ValueRef::new(ValueKind::Bool, tmp)
    }

    fn emit_main_value(&mut self, expr: &ExprType) -> Option<ValueRef> {
        match expr {
            ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t.clone()
            {
                TokenType::Number => {
                    let sanitized = sanitize_numeric_literal(&literal_expr_node.value);
                    if literal_expr_node.value.contains('.')
                        || literal_expr_node.value.contains('e')
                        || literal_expr_node.value.contains('E')
                    {
                        Some(ValueRef::new(ValueKind::Double, sanitized))
                    } else {
                        Some(ValueRef::new(ValueKind::I32, sanitized))
                    }
                }
                TokenType::True => Some(ValueRef::new(ValueKind::Bool, "1")),
                TokenType::False => Some(ValueRef::new(ValueKind::Bool, "0")),
                TokenType::String => {
                    let literal = self.intern_string(&literal_expr_node.value, false);
                    let ptr = self.next_temp();
                    self.push_line(&format!(
                        "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                        ptr, literal.len, literal.len, literal.name
                    ));
                    Some(ValueRef::new(ValueKind::CString, ptr))
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn emit_main_value_with_system(
        &mut self,
        summary: &SystemSummary,
        self_ptr: &str,
        expr: &ExprType,
    ) -> Option<ValueRef> {
        match expr {
            ExprType::LiteralExprT { .. } => self.emit_main_value(expr),
            ExprType::CallChainExprT {
                call_chain_expr_node,
            } => {
                if let Some(field) = self
                    .main_domain_field_from_call_chain(summary, &call_chain_expr_node.call_chain)
                {
                    let field_ptr = self.main_domain_field_ptr(summary, self_ptr, field);
                    let loaded = self.next_temp();
                    match field.field_type {
                        DomainFieldType::I32 => {
                            self.push_line(&format!("{} = load i32, i32* {}", loaded, field_ptr));
                            Some(ValueRef::new(ValueKind::I32, loaded))
                        }
                        DomainFieldType::F64 => {
                            self.push_line(&format!(
                                "{} = load double, double* {}",
                                loaded, field_ptr
                            ));
                            Some(ValueRef::new(ValueKind::Double, loaded))
                        }
                        DomainFieldType::Bool => {
                            self.push_line(&format!("{} = load i1, i1* {}", loaded, field_ptr));
                            Some(ValueRef::new(ValueKind::Bool, loaded))
                        }
                        DomainFieldType::CString => {
                            self.push_line(&format!("{} = load i8*, i8** {}", loaded, field_ptr));
                            Some(ValueRef::new(ValueKind::CString, loaded))
                        }
                    }
                } else {
                    None
                }
            }
            ExprType::BinaryExprT { binary_expr_node } => {
                let left_ref = binary_expr_node.left_rcref.borrow();
                let left = self.emit_main_value_with_system(summary, self_ptr, &*left_ref)?;
                drop(left_ref);
                let right_ref = binary_expr_node.right_rcref.borrow();
                let right = self.emit_main_value_with_system(summary, self_ptr, &*right_ref)?;
                drop(right_ref);
                match (binary_expr_node.operator.clone(), left.kind, right.kind) {
                    (OperatorType::Plus, ValueKind::I32, ValueKind::I32) => {
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = add i32 {}, {}",
                            result, left.value, right.value
                        ));
                        Some(ValueRef::new(ValueKind::I32, result))
                    }
                    (OperatorType::Plus, ValueKind::Double, ValueKind::Double) => {
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = fadd double {}, {}",
                            result, left.value, right.value
                        ));
                        Some(ValueRef::new(ValueKind::Double, result))
                    }
                    (OperatorType::Plus, ValueKind::I32, ValueKind::Double) => {
                        let left_conv = self.convert_i32_to_double(left);
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = fadd double {}, {}",
                            result, left_conv.value, right.value
                        ));
                        Some(ValueRef::new(ValueKind::Double, result))
                    }
                    (OperatorType::Plus, ValueKind::Double, ValueKind::I32) => {
                        let right_conv = self.convert_i32_to_double(right);
                        let result = self.next_temp();
                        self.push_line(&format!(
                            "{} = fadd double {}, {}",
                            result, left.value, right_conv.value
                        ));
                        Some(ValueRef::new(ValueKind::Double, result))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn main_domain_field_from_call_chain<'a>(
        &self,
        summary: &'a SystemSummary,
        call_chain: &VecDeque<CallChainNodeType>,
    ) -> Option<&'a DomainField> {
        let mut iter = call_chain.iter();
        match (iter.next(), iter.next(), iter.next()) {
            (Some(CallChainNodeType::VariableNodeT { .. }), Some(second), None)
            | (Some(CallChainNodeType::SelfT { .. }), Some(second), None) => match second {
                CallChainNodeType::UndeclaredIdentifierNodeT { id_node } => {
                    summary.domain_field(id_node.name.lexeme.as_str())
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    summary.domain_field(var_node.get_name())
                }
                _ => None,
            },
            (Some(CallChainNodeType::UndeclaredIdentifierNodeT { id_node }), None, None) => {
                summary.domain_field(id_node.name.lexeme.as_str())
            }
            _ => None,
        }
    }

    fn main_domain_field_ptr(
        &mut self,
        summary: &SystemSummary,
        self_ptr: &str,
        field: &DomainField,
    ) -> String {
        let ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
            ptr, summary.struct_name, summary.struct_name, self_ptr, field.struct_index
        ));
        ptr
    }

    fn begin_function(&mut self) {
        self.temp_counter = 0;
    }

    fn next_temp(&mut self) -> String {
        let name = format!("%tmp{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn push_line(&mut self, line: &str) {
        for _ in 0..self.indent {
            self.body.push_str("  ");
        }
        self.body.push_str(line);
        self.body.push('\n');
    }

    fn push_blank_line(&mut self) {
        self.body.push('\n');
    }

    fn push_comment(&mut self, text: &str) {
        self.push_line(&format!("; {}", text));
    }

    fn intern_string(&mut self, raw: &str, append_newline: bool) -> StringRef {
        let mut key = raw.to_string();
        if append_newline && !key.ends_with('\n') {
            key.push('\n');
        }

        if let Some(&index) = self.string_map.get(&key) {
            let lit = &self.string_literals[index];
            return StringRef {
                name: lit.name.clone(),
                len: lit.len,
            };
        }

        let (encoded, len) = encode_c_string(&key);
        let name = format!("@.str{}", self.string_literals.len());

        self.string_literals.push(StringLiteral {
            name: name.clone(),
            len,
            encoded,
        });
        self.string_map.insert(key, self.string_literals.len() - 1);

        StringRef { name, len }
    }

    fn require_puts(&mut self) {
        self.needs_puts = true;
    }

    fn require_print_int(&mut self) {
        self.needs_print_int = true;
    }

    fn require_print_double(&mut self) {
        self.needs_print_double = true;
    }

    fn require_print_bool(&mut self) {
        self.needs_print_bool = true;
    }

    fn require_runtime_api(&mut self) {
        self.needs_runtime_api = true;
    }

    fn require_runtime_event(&mut self) {
        self.needs_runtime_event = true;
    }

    pub(super) fn finish(self) -> String {
        let mut output = self.header;

        if !self.string_literals.is_empty() {
            output.push('\n');
            for literal in &self.string_literals {
                writeln!(
                    output,
                    "{} = private unnamed_addr constant [{} x i8] c\"{}\"",
                    literal.name, literal.len, literal.encoded
                )
                .unwrap();
            }
        }

        if self.needs_puts {
            output.push('\n');
            output.push_str("declare i32 @puts(i8*)\n");
        }

        if self.needs_print_int {
            output.push('\n');
            output.push_str("declare void @frame_runtime_print_int(i32)\n");
        }

        if self.needs_print_double {
            output.push('\n');
            output.push_str("declare void @frame_runtime_print_double(double)\n");
        }

        if self.needs_print_bool {
            output.push('\n');
            output.push_str("declare void @frame_runtime_print_bool(i1)\n");
        }

        if self.needs_runtime_api {
            output.push('\n');
            output.push_str("declare ptr @frame_runtime_compartment_new(ptr)\n");
            output.push_str("declare ptr @frame_runtime_kernel_new(ptr)\n");
            output.push_str("declare void @frame_runtime_kernel_free(ptr)\n");
            output.push_str("declare i32 @frame_runtime_kernel_dispatch(ptr, ptr)\n");
            output.push_str("declare void @frame_runtime_kernel_set_state(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_kernel_push_compartment(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_kernel_pop_compartment(ptr)\n");
            output.push_str("declare void @frame_runtime_kernel_state_stack_push(ptr, i32)\n");
            output.push_str("declare ptr @frame_runtime_kernel_state_stack_pop(ptr, i32*)\n");
            output.push_str("declare ptr @frame_runtime_compartment_get_parent(ptr)\n");
            output.push_str("declare void @frame_runtime_compartment_set_enter_event(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_compartment_take_enter_event(ptr)\n");
            output.push_str("declare void @frame_runtime_compartment_set_exit_event(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_compartment_take_exit_event(ptr)\n");
            output
                .push_str("declare void @frame_runtime_compartment_set_forward_event(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_kernel_next_event(ptr)\n");
            output.push_str(
                "declare void @frame_runtime_compartment_state_arg_set_i32(ptr, ptr, i32)\n",
            );
            output.push_str(
                "declare void @frame_runtime_compartment_state_arg_set_double(ptr, ptr, double)\n",
            );
            output.push_str(
                "declare void @frame_runtime_compartment_state_arg_set_bool(ptr, ptr, i1)\n",
            );
            output.push_str(
                "declare void @frame_runtime_compartment_state_arg_set_cstring(ptr, ptr, ptr)\n",
            );
            output.push_str("declare i32 @frame_runtime_compartment_state_arg_get_i32(ptr, ptr)\n");
            output.push_str(
                "declare double @frame_runtime_compartment_state_arg_get_double(ptr, ptr)\n",
            );
            output.push_str("declare i1 @frame_runtime_compartment_state_arg_get_bool(ptr, ptr)\n");
            output.push_str(
                "declare ptr @frame_runtime_compartment_state_arg_get_cstring(ptr, ptr)\n",
            );
            output.push_str(
                "declare void @frame_runtime_compartment_enter_arg_set_i32(ptr, ptr, i32)\n",
            );
            output.push_str(
                "declare void @frame_runtime_compartment_enter_arg_set_double(ptr, ptr, double)\n",
            );
            output.push_str(
                "declare void @frame_runtime_compartment_enter_arg_set_bool(ptr, ptr, i1)\n",
            );
            output.push_str(
                "declare void @frame_runtime_compartment_enter_arg_set_cstring(ptr, ptr, ptr)\n",
            );
            output.push_str("declare i32 @frame_runtime_compartment_enter_arg_get_i32(ptr, ptr)\n");
            output.push_str(
                "declare double @frame_runtime_compartment_enter_arg_get_double(ptr, ptr)\n",
            );
            output.push_str("declare i1 @frame_runtime_compartment_enter_arg_get_bool(ptr, ptr)\n");
            output.push_str(
                "declare ptr @frame_runtime_compartment_enter_arg_get_cstring(ptr, ptr)\n",
            );
            output.push_str("declare void @frame_runtime_compartment_enter_args_clear(ptr)\n");
        }

        if self.needs_runtime_event {
            output.push('\n');
            output.push_str("declare ptr @frame_runtime_event_new(ptr)\n");
            output.push_str("declare void @frame_runtime_event_free(ptr)\n");
            output.push_str("declare i1 @frame_runtime_event_is_message(ptr, ptr)\n");
            output.push_str("declare void @frame_runtime_event_push_param_i32(ptr, i32)\n");
            output.push_str("declare void @frame_runtime_event_push_param_double(ptr, double)\n");
            output.push_str("declare void @frame_runtime_event_push_param_bool(ptr, i1)\n");
            output.push_str("declare void @frame_runtime_event_push_param_cstring(ptr, ptr)\n");
            output.push_str("declare i32 @frame_runtime_event_get_param_i32(ptr, i32)\n");
            output.push_str("declare double @frame_runtime_event_get_param_double(ptr, i32)\n");
            output.push_str("declare i1 @frame_runtime_event_get_param_bool(ptr, i32)\n");
            output.push_str("declare ptr @frame_runtime_event_get_param_cstring(ptr, i32)\n");
        }

        if !self.body.trim().is_empty() {
            output.push('\n');
            output.push_str(&self.body);
        }

        output
    }
}

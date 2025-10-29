use crate::frame_c::ast::*;
use crate::frame_c::scanner::TokenType;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write as _;

use super::context::{
    ActionEntry, ActionParam, MainLocal, MainScope, MethodNames, SystemEmitContext, SystemSummary,
};
use super::utils::{
    call_chain_node_kind, encode_c_string, expr_kind, extract_string_literal, format_f64,
    sanitize_identifier, sanitize_numeric_literal,
};
use super::value::{
    DomainField, DomainFieldInit, DomainFieldType, LocalBinding, ValueKind, ValueRef,
};

pub(super) struct LLVMModuleBuilder {
    header: String,
    body: String,
    string_literals: Vec<StringLiteral>,
    string_map: HashMap<String, usize>,
    defined_structs: HashSet<String>,
    needs_puts: bool,
    needs_runtime_api: bool,
    needs_runtime_event: bool,
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
        self.begin_function();
        let default_label = format!(
            "{}_{}_dispatch_default",
            ctx.sanitized_name, names.method_ident
        );
        let end_label = format!("{}_{}_dispatch_end", ctx.sanitized_name, names.method_ident);

        writeln!(
            &mut self.body,
            "define void {}({}* %self) {{",
            names.fn_name, ctx.struct_name
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
                    state.parent_state_name.as_deref(),
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
                if self.handle_call_expr(None, None, None, &call_stmt_node.call_expr_node) {
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
                            if call_node.call_expr_list.exprs_t.is_empty() {
                                let method_name = call_node.identifier.name.lexeme.as_str();
                                let fn_name = local.system.method_fn(method_name);
                                self.push_line(&format!(
                                    "call void {}({}* {})",
                                    fn_name, local.system.struct_name, local.ptr
                                ));
                                return;
                            }
                            self.push_comment("method arguments not yet supported in main");
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
                            if interface_method_call_expr_node
                                .call_expr_list
                                .exprs_t
                                .is_empty()
                            {
                                let method_name = interface_method_call_expr_node
                                    .identifier
                                    .name
                                    .lexeme
                                    .as_str();
                                let fn_name = local.system.method_fn(method_name);
                                self.push_line(&format!(
                                    "call void {}({}* {})",
                                    fn_name, local.system.struct_name, local.ptr
                                ));
                                return;
                            }
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
        parent_state_name: Option<&str>,
    ) {
        let compartment_field_ptr = self.next_temp();
        self.push_line(&format!(
            "{} = getelementptr inbounds {}, {}* %self, i32 0, i32 {}",
            compartment_field_ptr,
            ctx.struct_name,
            ctx.struct_name,
            ctx.compartment_field_index()
        ));

        for stmt in &handler.statements {
            if let DeclOrStmtType::StmtT { stmt_t } = stmt {
                match stmt_t {
                    StatementType::ExpressionStmt { expr_stmt_t } => {
                        self.emit_expression_statement(ctx, "%self", expr_stmt_t, None);
                    }
                    StatementType::TransitionStmt {
                        transition_statement_node,
                    } => {
                        if let Some(target_index) =
                            ctx.transition_target_index(transition_statement_node)
                        {
                            self.push_line(&format!(
                                "store i32 {}, i32* {}",
                                target_index, state_ptr
                            ));
                            if let Some(target_name) =
                                ctx.transition_target_name(transition_statement_node)
                            {
                                let state_literal = self.intern_string(&target_name, false);
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

                                let new_state_cstr = self.next_temp();
                                self.push_line(&format!(
                                    "{} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                                    new_state_cstr,
                                    state_literal.len,
                                    state_literal.len,
                                    state_literal.name
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
                                let active_compartment = self.next_temp();
                                self.push_line(&format!(
                                    "{} = call ptr @frame_runtime_kernel_push_compartment(ptr {}, ptr {})",
                                    active_compartment, kernel_ptr, next_compartment
                                ));
                                self.push_line(&format!(
                                    "store ptr {}, ptr {}",
                                    active_compartment, compartment_field_ptr
                                ));
                            }
                        } else {
                            self.push_comment("unsupported transition");
                        }
                    }
                    StatementType::ParentDispatchStmt { .. } => {
                        if let Some(parent_name) = parent_state_name {
                            let parent_label = ctx.state_label(method_ident, parent_name);
                            self.push_line(&format!("br label %{}", parent_label));
                            return;
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
                self.push_line(&format!(
                    "call void {}({}* %self)",
                    method.fn_name, ctx.struct_name
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
    ) {
        match expr_stmt {
            ExprStmtType::CallStmtT { call_stmt_node } => {
                if self.handle_call_expr(
                    Some(ctx),
                    Some(self_ptr),
                    locals,
                    &call_stmt_node.call_expr_node,
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
                        if self.handle_call_expr(Some(ctx), Some(self_ptr), locals, call_node) {
                            return;
                        }
                    }
                }
                self.push_comment("unsupported call chain expression");
            }
            ExprStmtType::AssignmentStmtT {
                assignment_stmt_node,
            } => self.emit_assignment_statement(ctx, self_ptr, locals, assignment_stmt_node),
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
            } => match self.domain_field_from_call_chain(ctx, &call_chain_expr_node.call_chain) {
                Some(field) => target_field = Some(field),
                None => {
                    self.push_comment(&format!(
                        "unsupported call chain assignment target in LLVM backend: {}",
                        self.describe_call_chain(&call_chain_expr_node.call_chain)
                    ));
                    return;
                }
            },
            _ => {
                self.push_comment("unsupported assignment target in LLVM backend");
                return;
            }
        }

        let value = match self.emit_expression_value(ctx, self_ptr, locals, &*expr.r_value_rc) {
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

    fn handle_call_expr(
        &mut self,
        ctx: Option<&SystemEmitContext>,
        self_ptr: Option<&str>,
        locals: Option<&HashMap<String, LocalBinding>>,
        call: &CallExprNode,
    ) -> bool {
        let func_name = call.get_name();
        if func_name == "print" {
            if let Some(arg) = call.call_expr_list.exprs_t.first() {
                if let Some(binding) = match arg {
                    ExprType::VariableExprT { var_node } => {
                        locals.and_then(|map| map.get(var_node.get_name()))
                    }
                    ExprType::CallChainExprT {
                        call_chain_expr_node,
                    } => {
                        self.local_binding_from_call_chain(locals, &call_chain_expr_node.call_chain)
                    }
                    _ => None,
                } {
                    if binding.kind == ValueKind::CString {
                        let loaded = self.load_local_value(binding);
                        self.require_puts();
                        self.push_line(&format!("call i32 @puts(i8* {})", loaded.value));
                        return true;
                    } else {
                        self.push_comment("print currently supports only string locals");
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
                if let (Some(ctx), Some(self_ptr)) = (ctx, self_ptr) {
                    if let Some(field) = match arg {
                        ExprType::VariableExprT { var_node } => {
                            ctx.domain_field(var_node.get_name())
                        }
                        ExprType::CallChainExprT {
                            call_chain_expr_node,
                        } => {
                            self.domain_field_from_call_chain(ctx, &call_chain_expr_node.call_chain)
                        }
                        _ => None,
                    } {
                        match field.field_type {
                            DomainFieldType::CString => {
                                let field_ptr = self.next_temp();
                                self.push_line(&format!(
                                    "{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                                    field_ptr,
                                    ctx.struct_name,
                                    ctx.struct_name,
                                    self_ptr,
                                    field.struct_index
                                ));
                                let loaded = self.next_temp();
                                self.push_line(&format!(
                                    "{} = load i8*, i8** {}",
                                    loaded, field_ptr
                                ));
                                self.require_puts();
                                self.push_line(&format!("call i32 @puts(i8* {})", loaded));
                                return true;
                            }
                            _ => {
                                self.push_comment(
                                    "print currently supports only string domain variables",
                                );
                                return true;
                            }
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
                    let value = match self.emit_expression_value(ctx, self_ptr, locals_map, expr) {
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

        if self.needs_runtime_api {
            output.push('\n');
            output.push_str("declare ptr @frame_runtime_compartment_new(ptr)\n");
            output.push_str("declare ptr @frame_runtime_kernel_new(ptr)\n");
            output.push_str("declare void @frame_runtime_kernel_free(ptr)\n");
            output.push_str("declare i32 @frame_runtime_kernel_dispatch(ptr, ptr)\n");
            output.push_str("declare void @frame_runtime_kernel_set_state(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_kernel_push_compartment(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_compartment_get_parent(ptr)\n");
            output.push_str("declare void @frame_runtime_compartment_set_enter_event(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_compartment_take_enter_event(ptr)\n");
            output.push_str("declare void @frame_runtime_compartment_set_exit_event(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_compartment_take_exit_event(ptr)\n");
            output
                .push_str("declare void @frame_runtime_compartment_set_forward_event(ptr, ptr)\n");
            output.push_str("declare ptr @frame_runtime_kernel_next_event(ptr)\n");
        }

        if self.needs_runtime_event {
            output.push('\n');
            output.push_str("declare ptr @frame_runtime_event_new(ptr)\n");
            output.push_str("declare void @frame_runtime_event_free(ptr)\n");
            output.push_str("declare i1 @frame_runtime_event_is_message(ptr, ptr)\n");
        }

        if !self.body.trim().is_empty() {
            output.push('\n');
            output.push_str(&self.body);
        }

        output
    }
}

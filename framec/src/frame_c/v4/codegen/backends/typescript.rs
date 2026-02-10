//! TypeScript code generation backend

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::codegen::ast::*;
use crate::frame_c::v4::codegen::backend::*;

/// TypeScript backend for code generation
pub struct TypeScriptBackend;

impl LanguageBackend for TypeScriptBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String {
        match node {
            // ===== Structural =====

            CodegenNode::Module { imports, items } => {
                let mut result = String::new();

                for import in imports {
                    result.push_str(&self.emit(import, ctx));
                    result.push('\n');
                }

                if !imports.is_empty() && !items.is_empty() {
                    result.push('\n');
                }

                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str("\n\n");
                    }
                    result.push_str(&self.emit(item, ctx));
                }

                result
            }

            CodegenNode::Import { module, items, alias } => {
                if items.is_empty() {
                    if let Some(alias) = alias {
                        format!("import * as {} from \"{}\";", alias, module)
                    } else {
                        format!("import \"{}\"", module)
                    }
                } else {
                    format!("import {{ {} }} from \"{}\";", items.join(", "), module)
                }
            }

            CodegenNode::Class { name, fields, methods, base_classes, is_abstract } => {
                let mut result = String::new();

                let abstract_kw = if *is_abstract { "abstract " } else { "" };
                let extends = if base_classes.is_empty() {
                    String::new()
                } else {
                    format!(" extends {}", base_classes[0])
                };

                result.push_str(&format!("{}{}class {}{} {{\n", ctx.get_indent(), abstract_kw, name, extends));

                ctx.push_indent();

                // Fields
                for field in fields {
                    let vis = match field.visibility {
                        Visibility::Public => "public ",
                        Visibility::Private => "private ",
                        Visibility::Protected => "protected ",
                    };
                    let static_kw = if field.is_static { "static " } else { "" };
                    let type_ann = field.type_annotation.as_ref()
                        .map(|t| format!(": {}", self.convert_type(t)))
                        .unwrap_or_default();
                    let init = field.initializer.as_ref()
                        .map(|i| format!(" = {}", self.emit(i, ctx)))
                        .unwrap_or_default();

                    result.push_str(&format!("{}{}{}{}{}{};\n",
                        ctx.get_indent(), vis, static_kw, field.name, type_ann, init));
                }

                if !fields.is_empty() && !methods.is_empty() {
                    result.push('\n');
                }

                // Methods
                for (i, method) in methods.iter().enumerate() {
                    if i > 0 {
                        result.push('\n');
                    }
                    result.push_str(&self.emit(method, ctx));
                }

                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));

                result
            }

            CodegenNode::Enum { name, variants } => {
                let mut result = format!("{}enum {} {{\n", ctx.get_indent(), name);
                ctx.push_indent();

                for (i, variant) in variants.iter().enumerate() {
                    let comma = if i < variants.len() - 1 { "," } else { "" };
                    if let Some(value) = &variant.value {
                        result.push_str(&format!(
                            "{}{} = {}{}\n",
                            ctx.get_indent(),
                            variant.name,
                            self.emit(value, ctx),
                            comma
                        ));
                    } else {
                        result.push_str(&format!("{}{}{}\n", ctx.get_indent(), variant.name, comma));
                    }
                }

                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            // ===== Methods =====

            CodegenNode::Method { name, params, return_type, body, is_async, is_static, visibility, decorators } => {
                let mut result = String::new();

                let vis = match visibility {
                    Visibility::Public => "public ",
                    Visibility::Private => "private ",
                    Visibility::Protected => "protected ",
                };
                let static_kw = if *is_static { "static " } else { "" };
                let async_kw = if *is_async { "async " } else { "" };
                let params_str = self.emit_params(params);
                let return_str = return_type.as_ref()
                    .map(|rt| format!(": {}", rt))
                    .unwrap_or_default();

                result.push_str(&format!(
                    "{}{}{}{}{}({}){} {{\n",
                    ctx.get_indent(), vis, static_kw, async_kw, name, params_str, return_str
                ));

                ctx.push_indent();
                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    if self.needs_semicolon(stmt) {
                        result.push_str(";\n");
                    } else {
                        result.push('\n');
                    }
                }
                ctx.pop_indent();

                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::Constructor { params, body, super_call } => {
                let mut result = String::new();

                let params_str = self.emit_params(params);
                result.push_str(&format!("{}constructor({}) {{\n", ctx.get_indent(), params_str));

                ctx.push_indent();

                if let Some(super_call) = super_call {
                    result.push_str(&self.emit(super_call, ctx));
                    result.push_str(";\n");
                }

                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    if self.needs_semicolon(stmt) {
                        result.push_str(";\n");
                    } else {
                        result.push('\n');
                    }
                }
                ctx.pop_indent();

                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            // ===== Statements =====

            CodegenNode::VarDecl { name, type_annotation, init, is_const } => {
                let keyword = if *is_const { "const" } else { "let" };
                let type_ann = type_annotation.as_ref()
                    .map(|t| format!(": {}", self.convert_type(t)))
                    .unwrap_or_default();

                if let Some(init_expr) = init {
                    let init_str = self.emit(init_expr, ctx);
                    format!("{}{} {}{} = {}", ctx.get_indent(), keyword, name, type_ann, init_str)
                } else {
                    format!("{}{} {}{}", ctx.get_indent(), keyword, name, type_ann)
                }
            }

            CodegenNode::Assignment { target, value } => {
                let target_str = self.emit(target, ctx);
                let value_str = self.emit(value, ctx);
                format!("{}{} = {}", ctx.get_indent(), target_str, value_str)
            }

            CodegenNode::Return { value } => {
                if let Some(val) = value {
                    format!("{}return {}", ctx.get_indent(), self.emit(val, ctx))
                } else {
                    format!("{}return", ctx.get_indent())
                }
            }

            CodegenNode::If { condition, then_block, else_block } => {
                let mut result = String::new();
                let cond_str = self.emit(condition, ctx);
                result.push_str(&format!("{}if ({}) {{\n", ctx.get_indent(), cond_str));

                ctx.push_indent();
                for stmt in then_block {
                    result.push_str(&self.emit(stmt, ctx));
                    if self.needs_semicolon(stmt) {
                        result.push_str(";\n");
                    } else {
                        result.push('\n');
                    }
                }
                ctx.pop_indent();

                if let Some(else_stmts) = else_block {
                    result.push_str(&format!("{}}} else {{\n", ctx.get_indent()));
                    ctx.push_indent();
                    for stmt in else_stmts {
                        result.push_str(&self.emit(stmt, ctx));
                        if self.needs_semicolon(stmt) {
                            result.push_str(";\n");
                        } else {
                            result.push('\n');
                        }
                    }
                    ctx.pop_indent();
                }

                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::Match { scrutinee, arms } => {
                let mut result = String::new();
                let scrutinee_str = self.emit(scrutinee, ctx);
                result.push_str(&format!("{}switch ({}) {{\n", ctx.get_indent(), scrutinee_str));

                ctx.push_indent();
                for arm in arms {
                    let pattern_str = self.emit(&arm.pattern, ctx);
                    result.push_str(&format!("{}case {}:\n", ctx.get_indent(), pattern_str));

                    ctx.push_indent();
                    for stmt in &arm.body {
                        result.push_str(&self.emit(stmt, ctx));
                        if self.needs_semicolon(stmt) {
                            result.push_str(";\n");
                        } else {
                            result.push('\n');
                        }
                    }
                    result.push_str(&format!("{}break;\n", ctx.get_indent()));
                    ctx.pop_indent();
                }
                ctx.pop_indent();

                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::While { condition, body } => {
                let mut result = String::new();
                let cond_str = self.emit(condition, ctx);
                result.push_str(&format!("{}while ({}) {{\n", ctx.get_indent(), cond_str));

                ctx.push_indent();
                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    if self.needs_semicolon(stmt) {
                        result.push_str(";\n");
                    } else {
                        result.push('\n');
                    }
                }
                ctx.pop_indent();

                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::For { var, iterable, body } => {
                let mut result = String::new();
                let iter_str = self.emit(iterable, ctx);
                result.push_str(&format!("{}for (const {} of {}) {{\n", ctx.get_indent(), var, iter_str));

                ctx.push_indent();
                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    if self.needs_semicolon(stmt) {
                        result.push_str(";\n");
                    } else {
                        result.push('\n');
                    }
                }
                ctx.pop_indent();

                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::Break => format!("{}break", ctx.get_indent()),
            CodegenNode::Continue => format!("{}continue", ctx.get_indent()),

            CodegenNode::ExprStmt(expr) => {
                format!("{}{}", ctx.get_indent(), self.emit(expr, ctx))
            }

            CodegenNode::Comment { text, is_doc } => {
                if *is_doc {
                    format!("{}/** {} */", ctx.get_indent(), text)
                } else {
                    format!("{}// {}", ctx.get_indent(), text)
                }
            }

            CodegenNode::Empty => String::new(),

            // ===== Expressions =====

            CodegenNode::Ident(name) => name.clone(),

            CodegenNode::Literal(lit) => self.emit_literal(lit, ctx),

            CodegenNode::BinaryOp { op, left, right } => {
                self.emit_binary_op(op, left, right, ctx)
            }

            CodegenNode::UnaryOp { op, operand } => {
                self.emit_unary_op(op, operand, ctx)
            }

            CodegenNode::Call { target, args } => {
                let target_str = self.emit(target, ctx);
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("{}({})", target_str, args_str.join(", "))
            }

            CodegenNode::MethodCall { object, method, args } => {
                let obj_str = self.emit(object, ctx);
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("{}.{}({})", obj_str, method, args_str.join(", "))
            }

            CodegenNode::FieldAccess { object, field } => {
                let obj_str = self.emit(object, ctx);
                format!("{}.{}", obj_str, field)
            }

            CodegenNode::IndexAccess { object, index } => {
                let obj_str = self.emit(object, ctx);
                let idx_str = self.emit(index, ctx);
                format!("{}[{}]", obj_str, idx_str)
            }

            CodegenNode::SelfRef => "this".to_string(),

            CodegenNode::Array(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| self.emit(e, ctx)).collect();
                format!("[{}]", elems.join(", "))
            }

            CodegenNode::Dict(pairs) => {
                let pairs_str: Vec<String> = pairs.iter().map(|(k, v)| {
                    format!("{}: {}", self.emit(k, ctx), self.emit(v, ctx))
                }).collect();
                format!("{{ {} }}", pairs_str.join(", "))
            }

            CodegenNode::Ternary { condition, then_expr, else_expr } => {
                let cond = self.emit(condition, ctx);
                let then_val = self.emit(then_expr, ctx);
                let else_val = self.emit(else_expr, ctx);
                format!("{} ? {} : {}", cond, then_val, else_val)
            }

            CodegenNode::Lambda { params, body } => {
                let params_str = params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ");
                let body_str = self.emit(body, ctx);
                format!("({}) => {}", params_str, body_str)
            }

            CodegenNode::Cast { expr, target_type } => {
                let expr_str = self.emit(expr, ctx);
                format!("{} as {}", expr_str, target_type)
            }

            CodegenNode::New { class, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("new {}({})", class, args_str.join(", "))
            }

            // ===== Frame-Specific =====

            CodegenNode::Transition { target_state, exit_args, enter_args, state_args, indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                let mut args = vec![format!("this.{}", target_state)];

                if !exit_args.is_empty() || !enter_args.is_empty() || !state_args.is_empty() {
                    let exit_str: Vec<String> = exit_args.iter().map(|a| self.emit(a, ctx)).collect();
                    args.push(format!("[{}]", exit_str.join(", ")));
                }

                if !enter_args.is_empty() || !state_args.is_empty() {
                    let enter_str: Vec<String> = enter_args.iter().map(|a| self.emit(a, ctx)).collect();
                    args.push(format!("[{}]", enter_str.join(", ")));
                }

                if !state_args.is_empty() {
                    let state_str: Vec<String> = state_args.iter().map(|a| self.emit(a, ctx)).collect();
                    args.push(format!("[{}]", state_str.join(", ")));
                }

                format!("{}this._transition({})", ind, args.join(", "))
            }

            CodegenNode::ChangeState { target_state, state_args, indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                if state_args.is_empty() {
                    format!("{}this._changeState(this.{})", ind, target_state)
                } else {
                    let args_str: Vec<String> = state_args.iter().map(|a| self.emit(a, ctx)).collect();
                    format!("{}this._changeState(this.{}, [{}])", ind, target_state, args_str.join(", "))
                }
            }

            CodegenNode::Forward { to_parent: _, indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}return", ind)
            }

            CodegenNode::StackPush { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}this._stateStack.push(this._state)", ind)
            }

            CodegenNode::StackPop { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}this._transition(this._stateStack.pop()!)", ind)
            }

            CodegenNode::StateContext { state_name } => {
                format!("this._stateContext[\"{}\"]", state_name)
            }

            CodegenNode::SendEvent { event, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                if args_str.is_empty() {
                    format!("{}this.{}()", ctx.get_indent(), event)
                } else {
                    format!("{}this.{}({})", ctx.get_indent(), event, args_str.join(", "))
                }
            }

            // ===== Native Code Preservation =====

            CodegenNode::NativeBlock { code, span: _ } => {
                code.clone()
            }

            CodegenNode::SplicePoint { id } => {
                format!("// SPLICE_POINT: {}", id)
            }
        }
    }

    fn runtime_imports(&self) -> Vec<String> {
        vec![]
    }

    fn class_syntax(&self) -> ClassSyntax {
        ClassSyntax::typescript()
    }

    fn target_language(&self) -> TargetLanguage {
        TargetLanguage::TypeScript
    }
}

impl TypeScriptBackend {
    /// Convert type annotation from generic/Python types to TypeScript types
    fn convert_type(&self, type_str: &str) -> String {
        match type_str {
            "Any" => "any".to_string(),
            "int" => "number".to_string(),
            "float" => "number".to_string(),
            "str" => "string".to_string(),
            "bool" => "boolean".to_string(),
            "None" => "void".to_string(),
            "List" => "Array".to_string(),
            "Dict" => "Record<string, any>".to_string(),
            other => other.to_string(),
        }
    }

    fn emit_params(&self, params: &[Param]) -> String {
        params.iter().map(|p| {
            let mut s = p.name.clone();
            if let Some(ref t) = p.type_annotation {
                s.push_str(&format!(": {}", self.convert_type(t)));
            }
            if let Some(ref d) = p.default_value {
                let mut ctx = EmitContext::new();
                s.push_str(&format!(" = {}", self.emit(d, &mut ctx)));
            }
            s
        }).collect::<Vec<_>>().join(", ")
    }

    fn needs_semicolon(&self, node: &CodegenNode) -> bool {
        !matches!(node,
            CodegenNode::If { .. } |
            CodegenNode::While { .. } |
            CodegenNode::For { .. } |
            CodegenNode::Match { .. } |
            CodegenNode::Comment { .. } |
            CodegenNode::Empty
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_literal() {
        let backend = TypeScriptBackend;
        let mut ctx = EmitContext::new();

        assert_eq!(backend.emit(&CodegenNode::int(42), &mut ctx), "42");
        assert_eq!(backend.emit(&CodegenNode::bool(true), &mut ctx), "true");
        assert_eq!(backend.emit(&CodegenNode::null(), &mut ctx), "null");
    }

    #[test]
    fn test_emit_field_access() {
        let backend = TypeScriptBackend;
        let mut ctx = EmitContext::new();

        let node = CodegenNode::field(CodegenNode::self_ref(), "_state");
        assert_eq!(backend.emit(&node, &mut ctx), "this._state");
    }

    #[test]
    fn test_emit_class() {
        let backend = TypeScriptBackend;
        let mut ctx = EmitContext::new();

        let node = CodegenNode::Class {
            name: "TestClass".to_string(),
            fields: vec![Field::new("_state").with_visibility(Visibility::Private)],
            methods: vec![],
            base_classes: vec![],
            is_abstract: false,
        };

        let result = backend.emit(&node, &mut ctx);
        assert!(result.contains("class TestClass {"));
        assert!(result.contains("private _state"));
    }
}

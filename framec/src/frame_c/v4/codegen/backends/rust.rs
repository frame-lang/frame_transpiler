//! Rust code generation backend

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::codegen::ast::*;
use crate::frame_c::v4::codegen::backend::*;

/// Rust backend for code generation
pub struct RustBackend;

impl LanguageBackend for RustBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String {
        match node {
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
                    format!("use {};", module)
                } else {
                    format!("use {}::{{{}}};", module, items.join(", "))
                }
            }

            CodegenNode::Class { name, fields, methods, base_classes, is_abstract } => {
                let mut result = String::new();

                // Struct definition
                result.push_str(&format!("{}pub struct {} {{\n", ctx.get_indent(), name));
                ctx.push_indent();
                for field in fields {
                    let vis = if matches!(field.visibility, Visibility::Public) { "pub " } else { "" };
                    let type_ann = field.type_annotation.as_ref().unwrap_or(&"()".to_string()).clone();
                    result.push_str(&format!("{}{}{}: {},\n", ctx.get_indent(), vis, field.name, type_ann));
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}\n\n", ctx.get_indent()));

                // Impl block
                result.push_str(&format!("{}impl {} {{\n", ctx.get_indent(), name));
                ctx.push_indent();
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
                let mut result = format!("{}pub enum {} {{\n", ctx.get_indent(), name);
                ctx.push_indent();
                for variant in variants {
                    result.push_str(&format!("{}{},\n", ctx.get_indent(), variant.name));
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::Method { name, params, return_type, body, is_async, is_static, visibility, decorators } => {
                let mut result = String::new();
                let vis = if matches!(visibility, Visibility::Public) { "pub " } else { "" };
                let async_kw = if *is_async { "async " } else { "" };

                let params_str = self.emit_params(params, !*is_static);
                let return_str = return_type.as_ref()
                    .map(|rt| format!(" -> {}", rt))
                    .unwrap_or_default();

                result.push_str(&format!("{}{}{}fn {}({}){} {{\n",
                    ctx.get_indent(), vis, async_kw, name, params_str, return_str));

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
                let params_str = self.emit_params(params, false);
                let mut result = format!("{}pub fn new({}) -> Self {{\n", ctx.get_indent(), params_str);

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

            CodegenNode::VarDecl { name, type_annotation, init, is_const } => {
                let kw = if *is_const { "let" } else { "let mut" };
                let type_ann = type_annotation.as_ref().map(|t| format!(": {}", t)).unwrap_or_default();
                if let Some(init_expr) = init {
                    format!("{}{} {}{} = {}", ctx.get_indent(), kw, name, type_ann, self.emit(init_expr, ctx))
                } else {
                    format!("{}{} {}{}", ctx.get_indent(), kw, name, type_ann)
                }
            }

            CodegenNode::Assignment { target, value } => {
                format!("{}{} = {}", ctx.get_indent(), self.emit(target, ctx), self.emit(value, ctx))
            }

            CodegenNode::Return { value } => {
                if let Some(val) = value {
                    format!("{}{}", ctx.get_indent(), self.emit(val, ctx))
                } else {
                    format!("{}return", ctx.get_indent())
                }
            }

            CodegenNode::If { condition, then_block, else_block } => {
                let mut result = format!("{}if {} {{\n", ctx.get_indent(), self.emit(condition, ctx));
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
                let mut result = format!("{}match {} {{\n", ctx.get_indent(), self.emit(scrutinee, ctx));
                ctx.push_indent();
                for arm in arms {
                    result.push_str(&format!("{}{} => {{\n", ctx.get_indent(), self.emit(&arm.pattern, ctx)));
                    ctx.push_indent();
                    for stmt in &arm.body {
                        result.push_str(&self.emit(stmt, ctx));
                        if self.needs_semicolon(stmt) {
                            result.push_str(";\n");
                        } else {
                            result.push('\n');
                        }
                    }
                    ctx.pop_indent();
                    result.push_str(&format!("{}}}\n", ctx.get_indent()));
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::While { condition, body } => {
                let mut result = format!("{}while {} {{\n", ctx.get_indent(), self.emit(condition, ctx));
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
                let mut result = format!("{}for {} in {} {{\n", ctx.get_indent(), var, self.emit(iterable, ctx));
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

            CodegenNode::ExprStmt(expr) => format!("{}{}", ctx.get_indent(), self.emit(expr, ctx)),

            CodegenNode::Comment { text, is_doc } => {
                if *is_doc { format!("{}/// {}", ctx.get_indent(), text) }
                else { format!("{}// {}", ctx.get_indent(), text) }
            }

            CodegenNode::Empty => String::new(),

            CodegenNode::Ident(name) => name.clone(),
            CodegenNode::Literal(lit) => self.emit_literal(lit, ctx),
            CodegenNode::BinaryOp { op, left, right } => self.emit_binary_op(op, left, right, ctx),
            CodegenNode::UnaryOp { op, operand } => self.emit_unary_op(op, operand, ctx),

            CodegenNode::Call { target, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("{}({})", self.emit(target, ctx), args_str.join(", "))
            }

            CodegenNode::MethodCall { object, method, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("{}.{}({})", self.emit(object, ctx), method, args_str.join(", "))
            }

            CodegenNode::FieldAccess { object, field } => {
                format!("{}.{}", self.emit(object, ctx), field)
            }

            CodegenNode::IndexAccess { object, index } => {
                format!("{}[{}]", self.emit(object, ctx), self.emit(index, ctx))
            }

            CodegenNode::SelfRef => "self".to_string(),

            CodegenNode::Array(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| self.emit(e, ctx)).collect();
                format!("vec![{}]", elems.join(", "))
            }

            CodegenNode::Dict(pairs) => {
                let pairs_str: Vec<String> = pairs.iter().map(|(k, v)| {
                    format!("({}, {})", self.emit(k, ctx), self.emit(v, ctx))
                }).collect();
                format!("HashMap::from([{}])", pairs_str.join(", "))
            }

            CodegenNode::Ternary { condition, then_expr, else_expr } => {
                format!("if {} {{ {} }} else {{ {} }}",
                    self.emit(condition, ctx), self.emit(then_expr, ctx), self.emit(else_expr, ctx))
            }

            CodegenNode::Lambda { params, body } => {
                let params_str = params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ");
                format!("|{}| {}", params_str, self.emit(body, ctx))
            }

            CodegenNode::Cast { expr, target_type } => {
                format!("{} as {}", self.emit(expr, ctx), target_type)
            }

            CodegenNode::New { class, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("{}::new({})", class, args_str.join(", "))
            }

            // Frame-specific
            CodegenNode::Transition { target_state, exit_args: _, enter_args: _, state_args: _, indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}self._transition(Self::{})", ind, target_state)
            }

            CodegenNode::ChangeState { target_state, state_args: _, indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}self._change_state(Self::{})", ind, target_state)
            }

            CodegenNode::Forward { to_parent: _, indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}return", ind)
            }
            CodegenNode::StackPush { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}self._state_stack.push(self._state)", ind)
            }
            CodegenNode::StackPop { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}self._transition(self._state_stack.pop().unwrap())", ind)
            }

            CodegenNode::StateContext { state_name } => {
                format!("self._state_context.get(\"{}\").unwrap()", state_name)
            }

            CodegenNode::SendEvent { event, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                if args_str.is_empty() {
                    format!("{}self.{}()", ctx.get_indent(), event)
                } else {
                    format!("{}self.{}({})", ctx.get_indent(), event, args_str.join(", "))
                }
            }

            CodegenNode::NativeBlock { code, span: _ } => code.clone(),
            CodegenNode::SplicePoint { id } => format!("// SPLICE_POINT: {}", id),
        }
    }

    fn runtime_imports(&self) -> Vec<String> {
        vec!["use std::collections::HashMap;".to_string()]
    }

    fn class_syntax(&self) -> ClassSyntax {
        ClassSyntax::rust()
    }

    fn target_language(&self) -> TargetLanguage {
        TargetLanguage::Rust
    }
}

impl RustBackend {
    fn emit_params(&self, params: &[Param], include_self: bool) -> String {
        let mut all_params = Vec::new();
        if include_self {
            all_params.push("&mut self".to_string());
        }
        for param in params {
            let type_ann = param.type_annotation.as_ref().unwrap_or(&"()".to_string()).clone();
            all_params.push(format!("{}: {}", param.name, type_ann));
        }
        all_params.join(", ")
    }

    fn needs_semicolon(&self, node: &CodegenNode) -> bool {
        !matches!(node,
            CodegenNode::If { .. } | CodegenNode::While { .. } |
            CodegenNode::For { .. } | CodegenNode::Match { .. } |
            CodegenNode::Comment { .. } | CodegenNode::Empty
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_self() {
        let backend = RustBackend;
        let mut ctx = EmitContext::new();
        assert_eq!(backend.emit(&CodegenNode::SelfRef, &mut ctx), "self");
    }

    #[test]
    fn test_emit_array() {
        let backend = RustBackend;
        let mut ctx = EmitContext::new();
        let node = CodegenNode::Array(vec![CodegenNode::int(1), CodegenNode::int(2)]);
        assert_eq!(backend.emit(&node, &mut ctx), "vec![1, 2]");
    }
}

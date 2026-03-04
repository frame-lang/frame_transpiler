//! Java code generation backend

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::codegen::ast::*;
use crate::frame_c::v4::codegen::backend::*;

/// Java backend for code generation
pub struct JavaBackend;

impl LanguageBackend for JavaBackend {
    fn emit(&self, node: &CodegenNode, ctx: &mut EmitContext) -> String {
        match node {
            CodegenNode::Module { imports, items } => {
                let mut result = String::new();
                for import in imports {
                    result.push_str(&self.emit(import, ctx));
                    result.push('\n');
                }
                if !imports.is_empty() && !items.is_empty() { result.push('\n'); }
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { result.push_str("\n\n"); }
                    result.push_str(&self.emit(item, ctx));
                }
                result
            }

            CodegenNode::Import { module, items, .. } => {
                if items.is_empty() {
                    format!("import {}.*;", module)
                } else {
                    items.iter().map(|i| format!("import {}.{};", module, i)).collect::<Vec<_>>().join("\n")
                }
            }

            CodegenNode::Class { name, fields, methods, base_classes, is_abstract, .. } => {
                let mut result = String::new();
                let abstract_kw = if *is_abstract { "abstract " } else { "" };
                let extends = if base_classes.is_empty() {
                    String::new()
                } else {
                    format!(" extends {}", base_classes[0])
                };

                result.push_str(&format!("{}{}public class {}{} {{\n", ctx.get_indent(), abstract_kw, name, extends));
                ctx.push_indent();

                for field in fields {
                    let vis = self.emit_visibility(field.visibility);
                    let type_ann = field.type_annotation.as_ref().unwrap_or(&"Object".to_string()).clone();
                    result.push_str(&format!("{}{} {} {};\n", ctx.get_indent(), vis, type_ann, field.name));
                }
                if !fields.is_empty() && !methods.is_empty() { result.push('\n'); }

                for (i, method) in methods.iter().enumerate() {
                    if i > 0 { result.push('\n'); }
                    result.push_str(&self.emit(method, ctx));
                }

                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::Enum { name, variants } => {
                let mut result = format!("{}public enum {} {{\n", ctx.get_indent(), name);
                ctx.push_indent();
                for (i, variant) in variants.iter().enumerate() {
                    let sep = if i < variants.len() - 1 { "," } else { "" };
                    result.push_str(&format!("{}{}{}\n", ctx.get_indent(), variant.name, sep));
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::Method { name, params, return_type, body, is_async: _, is_static, visibility, .. } => {
                let vis = self.emit_visibility(*visibility);
                let static_kw = if *is_static { "static " } else { "" };
                let return_str = return_type.as_ref().unwrap_or(&"void".to_string()).clone();
                let params_str = self.emit_params(params);

                let mut result = format!("{}{} {}{} {}({}) {{\n",
                    ctx.get_indent(), vis, static_kw, return_str, name, params_str);

                ctx.push_indent();
                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    result.push_str(";\n");
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::Constructor { params, body, super_call } => {
                let class_name = ctx.system_name.clone().unwrap_or("Class".to_string());
                let params_str = self.emit_params(params);

                let mut result = format!("{}public {}({}) {{\n", ctx.get_indent(), class_name, params_str);
                ctx.push_indent();

                if super_call.is_some() {
                    result.push_str(&format!("{}super();\n", ctx.get_indent()));
                }

                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    result.push_str(";\n");
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::VarDecl { name, type_annotation, init, is_const } => {
                let final_kw = if *is_const { "final " } else { "" };
                let type_str = type_annotation.as_ref().unwrap_or(&"var".to_string()).clone();
                if let Some(init_expr) = init {
                    format!("{}{}{} {} = {}", ctx.get_indent(), final_kw, type_str, name, self.emit(init_expr, ctx))
                } else {
                    format!("{}{}{} {}", ctx.get_indent(), final_kw, type_str, name)
                }
            }

            CodegenNode::Assignment { target, value } => {
                format!("{}{} = {}", ctx.get_indent(), self.emit(target, ctx), self.emit(value, ctx))
            }

            CodegenNode::Return { value } => {
                if let Some(val) = value {
                    format!("{}return {}", ctx.get_indent(), self.emit(val, ctx))
                } else {
                    format!("{}return", ctx.get_indent())
                }
            }

            CodegenNode::If { condition, then_block, else_block } => {
                let mut result = format!("{}if ({}) {{\n", ctx.get_indent(), self.emit(condition, ctx));
                ctx.push_indent();
                for stmt in then_block { result.push_str(&self.emit(stmt, ctx)); result.push_str(";\n"); }
                ctx.pop_indent();

                if let Some(else_stmts) = else_block {
                    result.push_str(&format!("{}}} else {{\n", ctx.get_indent()));
                    ctx.push_indent();
                    for stmt in else_stmts { result.push_str(&self.emit(stmt, ctx)); result.push_str(";\n"); }
                    ctx.pop_indent();
                }
                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::Match { scrutinee, arms } => {
                let mut result = format!("{}switch ({}) {{\n", ctx.get_indent(), self.emit(scrutinee, ctx));
                ctx.push_indent();
                for arm in arms {
                    result.push_str(&format!("{}case {}:\n", ctx.get_indent(), self.emit(&arm.pattern, ctx)));
                    ctx.push_indent();
                    for stmt in &arm.body { result.push_str(&self.emit(stmt, ctx)); result.push_str(";\n"); }
                    result.push_str(&format!("{}break;\n", ctx.get_indent()));
                    ctx.pop_indent();
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::While { condition, body } => {
                let mut result = format!("{}while ({}) {{\n", ctx.get_indent(), self.emit(condition, ctx));
                ctx.push_indent();
                for stmt in body { result.push_str(&self.emit(stmt, ctx)); result.push_str(";\n"); }
                ctx.pop_indent();
                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::For { var, iterable, body } => {
                let mut result = format!("{}for (var {} : {}) {{\n", ctx.get_indent(), var, self.emit(iterable, ctx));
                ctx.push_indent();
                for stmt in body { result.push_str(&self.emit(stmt, ctx)); result.push_str(";\n"); }
                ctx.pop_indent();
                result.push_str(&format!("{}}}", ctx.get_indent()));
                result
            }

            CodegenNode::Break => format!("{}break", ctx.get_indent()),
            CodegenNode::Continue => format!("{}continue", ctx.get_indent()),
            CodegenNode::ExprStmt(expr) => format!("{}{}", ctx.get_indent(), self.emit(expr, ctx)),

            CodegenNode::Comment { text, is_doc } => {
                if *is_doc { format!("{}/** {} */", ctx.get_indent(), text) }
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

            CodegenNode::FieldAccess { object, field } => format!("{}.{}", self.emit(object, ctx), field),
            CodegenNode::IndexAccess { object, index } => format!("{}.get({})", self.emit(object, ctx), self.emit(index, ctx)),
            CodegenNode::SelfRef => "this".to_string(),

            CodegenNode::Array(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| self.emit(e, ctx)).collect();
                format!("Arrays.asList({})", elems.join(", "))
            }

            CodegenNode::Dict(pairs) => {
                let pairs_str: Vec<String> = pairs.iter().map(|(k, v)| {
                    format!("Map.entry({}, {})", self.emit(k, ctx), self.emit(v, ctx))
                }).collect();
                format!("Map.ofEntries({})", pairs_str.join(", "))
            }

            CodegenNode::Ternary { condition, then_expr, else_expr } => {
                format!("{} ? {} : {}", self.emit(condition, ctx), self.emit(then_expr, ctx), self.emit(else_expr, ctx))
            }

            CodegenNode::Lambda { params, body } => {
                let params_str = params.iter().map(|p| p.name.clone()).collect::<Vec<_>>().join(", ");
                format!("({}) -> {}", params_str, self.emit(body, ctx))
            }

            CodegenNode::Cast { expr, target_type } => format!("({}){}", target_type, self.emit(expr, ctx)),

            CodegenNode::New { class, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("new {}({})", class, args_str.join(", "))
            }

            // Frame-specific
            CodegenNode::Transition { target_state, indent, .. } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}this._transition(this.{})", ind, target_state)
            }
            CodegenNode::ChangeState { target_state, indent, .. } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}this._changeState(this.{})", ind, target_state)
            }
            CodegenNode::Forward { indent, .. } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}return", ind)
            }
            CodegenNode::StackPush { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}this._stateStack.push(this._state)", ind)
            }
            CodegenNode::StackPop { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}this._transition(this._stateStack.pop())", ind)
            }
            CodegenNode::StateContext { state_name } => format!("this._stateContext.get(\"{}\")", state_name),

            CodegenNode::SendEvent { event, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                if args_str.is_empty() {
                    format!("{}this.{}()", ctx.get_indent(), event)
                } else {
                    format!("{}this.{}({})", ctx.get_indent(), event, args_str.join(", "))
                }
            }

            CodegenNode::NativeBlock { code, .. } => code.clone(),
            CodegenNode::SplicePoint { id } => format!("// SPLICE_POINT: {}", id),
        }
    }

    fn runtime_imports(&self) -> Vec<String> {
        vec!["import java.util.*;".to_string()]
    }

    fn class_syntax(&self) -> ClassSyntax { ClassSyntax::java() }
    fn target_language(&self) -> TargetLanguage { TargetLanguage::Java }
}

impl JavaBackend {
    fn emit_params(&self, params: &[Param]) -> String {
        params.iter().map(|p| {
            let type_ann = p.type_annotation.as_ref().unwrap_or(&"Object".to_string()).clone();
            format!("{} {}", type_ann, p.name)
        }).collect::<Vec<_>>().join(", ")
    }
}

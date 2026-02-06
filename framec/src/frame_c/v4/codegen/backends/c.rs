//! C code generation backend

use crate::frame_c::visitors::TargetLanguage;
use crate::frame_c::v4::codegen::ast::*;
use crate::frame_c::v4::codegen::backend::*;

/// C backend for code generation
pub struct CBackend;

impl LanguageBackend for CBackend {
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

            CodegenNode::Import { module, .. } => format!("#include <{}.h>", module),

            CodegenNode::Class { name, fields, methods, .. } => {
                let mut result = String::new();

                // Struct definition
                result.push_str(&format!("{}typedef struct {} {{\n", ctx.get_indent(), name));
                ctx.push_indent();
                for field in fields {
                    let type_ann = field.type_annotation.as_ref().unwrap_or(&"void*".to_string()).clone();
                    result.push_str(&format!("{}{} {};\n", ctx.get_indent(), type_ann, field.name));
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}} {};\n\n", ctx.get_indent(), name));

                // Function declarations and definitions
                for method in methods {
                    result.push_str(&self.emit(method, ctx));
                    result.push('\n');
                }
                result
            }

            CodegenNode::Enum { name, variants } => {
                let mut result = format!("{}typedef enum {{\n", ctx.get_indent());
                ctx.push_indent();
                for (i, variant) in variants.iter().enumerate() {
                    let comma = if i < variants.len() - 1 { "," } else { "" };
                    result.push_str(&format!("{}{}_{}{}\n", ctx.get_indent(), name, variant.name, comma));
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}} {};\n", ctx.get_indent(), name));
                result
            }

            CodegenNode::Method { name, params, return_type, body, .. } => {
                let return_str = return_type.as_ref().unwrap_or(&"void".to_string()).clone();
                let params_str = self.emit_params(params, ctx);

                let mut result = format!("{}{} {}({}) {{\n", ctx.get_indent(), return_str, name, params_str);
                ctx.push_indent();
                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    result.push_str(";\n");
                }
                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::Constructor { params, body, .. } => {
                let class_name = ctx.system_name.clone().unwrap_or("Struct".to_string());
                let params_str = self.emit_params(params, ctx);

                let mut result = format!("{}{}* {}_new({}) {{\n", ctx.get_indent(), class_name, class_name, params_str);
                ctx.push_indent();
                result.push_str(&format!("{}{}* self = malloc(sizeof({}));\n", ctx.get_indent(), class_name, class_name));
                for stmt in body {
                    result.push_str(&self.emit(stmt, ctx));
                    result.push_str(";\n");
                }
                result.push_str(&format!("{}return self;\n", ctx.get_indent()));
                ctx.pop_indent();
                result.push_str(&format!("{}}}\n", ctx.get_indent()));
                result
            }

            CodegenNode::VarDecl { name, type_annotation, init, .. } => {
                let type_str = type_annotation.as_ref().unwrap_or(&"int".to_string()).clone();
                if let Some(init_expr) = init {
                    format!("{}{} {} = {}", ctx.get_indent(), type_str, name, self.emit(init_expr, ctx))
                } else {
                    format!("{}{} {}", ctx.get_indent(), type_str, name)
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
                // C doesn't have for-each, generate a comment
                let mut result = format!("{}/* for {} in {} */\n", ctx.get_indent(), var, self.emit(iterable, ctx));
                result.push_str(&format!("{}/* TODO: implement iteration */", ctx.get_indent()));
                result
            }

            CodegenNode::Break => format!("{}break", ctx.get_indent()),
            CodegenNode::Continue => format!("{}continue", ctx.get_indent()),
            CodegenNode::ExprStmt(expr) => format!("{}{}", ctx.get_indent(), self.emit(expr, ctx)),
            CodegenNode::Comment { text, .. } => format!("{}/* {} */", ctx.get_indent(), text),
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
                // C doesn't have methods, generate function call with self
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                let all_args = if args_str.is_empty() {
                    self.emit(object, ctx)
                } else {
                    format!("{}, {}", self.emit(object, ctx), args_str.join(", "))
                };
                format!("{}({})", method, all_args)
            }

            CodegenNode::FieldAccess { object, field } => format!("{}->{}", self.emit(object, ctx), field),
            CodegenNode::IndexAccess { object, index } => format!("{}[{}]", self.emit(object, ctx), self.emit(index, ctx)),
            CodegenNode::SelfRef => "self".to_string(),

            CodegenNode::Array(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| self.emit(e, ctx)).collect();
                format!("{{ {} }}", elems.join(", "))
            }

            CodegenNode::Dict(_) => "/* Dict not supported in C */".to_string(),

            CodegenNode::Ternary { condition, then_expr, else_expr } => {
                format!("({}) ? ({}) : ({})", self.emit(condition, ctx), self.emit(then_expr, ctx), self.emit(else_expr, ctx))
            }

            CodegenNode::Lambda { .. } => "/* Lambda not supported in C */".to_string(),
            CodegenNode::Cast { expr, target_type } => format!("({})({})", target_type, self.emit(expr, ctx)),

            CodegenNode::New { class, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                format!("{}_new({})", class, args_str.join(", "))
            }

            // Frame-specific
            CodegenNode::Transition { target_state, indent, .. } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}_transition(self, {})", ind, target_state)
            }
            CodegenNode::ChangeState { target_state, indent, .. } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}_change_state(self, {})", ind, target_state)
            }
            CodegenNode::Forward { indent, .. } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}return", ind)
            }
            CodegenNode::StackPush { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}_state_stack_push(self)", ind)
            }
            CodegenNode::StackPop { indent } => {
                let ind = format!("{}{}", ctx.get_indent(), " ".repeat(*indent));
                format!("{}_state_stack_pop(self)", ind)
            }
            CodegenNode::StateContext { state_name } => format!("self->_state_context[\"{}\"]", state_name),

            CodegenNode::SendEvent { event, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit(a, ctx)).collect();
                if args_str.is_empty() {
                    format!("{}{}(self)", ctx.get_indent(), event)
                } else {
                    format!("{}{}(self, {})", ctx.get_indent(), event, args_str.join(", "))
                }
            }

            CodegenNode::NativeBlock { code, .. } => code.clone(),
            CodegenNode::SplicePoint { id } => format!("/* SPLICE_POINT: {} */", id),
        }
    }

    fn runtime_imports(&self) -> Vec<String> {
        vec!["#include <stdlib.h>".to_string(), "#include <stdio.h>".to_string()]
    }

    fn class_syntax(&self) -> ClassSyntax { ClassSyntax::c() }
    fn target_language(&self) -> TargetLanguage { TargetLanguage::C }

    fn null_keyword(&self) -> &'static str { "NULL" }
}

impl CBackend {
    fn emit_params(&self, params: &[Param], ctx: &EmitContext) -> String {
        if params.is_empty() {
            "void".to_string()
        } else {
            params.iter().map(|p| {
                let type_ann = p.type_annotation.as_ref().unwrap_or(&"int".to_string()).clone();
                format!("{} {}", type_ann, p.name)
            }).collect::<Vec<_>>().join(", ")
        }
    }
}

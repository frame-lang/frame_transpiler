use crate::frame_c::ast::{ExprType, OperatorType, TypeNode, VariableDeclNode};
use crate::frame_c::scanner::TokenType;

use super::utils::sanitize_numeric_literal;

#[derive(Clone)]
pub(super) enum DomainFieldType {
    I32,
    F64,
    Bool,
    CString,
}

impl DomainFieldType {
    pub(super) fn llvm_type(&self) -> &'static str {
        match self {
            DomainFieldType::I32 => "i32",
            DomainFieldType::F64 => "double",
            DomainFieldType::Bool => "i1",
            DomainFieldType::CString => "i8*",
        }
    }

    pub(super) fn needs_eight_byte_align(&self) -> bool {
        matches!(self, DomainFieldType::F64 | DomainFieldType::CString)
    }

    pub(super) fn value_kind(&self) -> ValueKind {
        match self {
            DomainFieldType::I32 => ValueKind::I32,
            DomainFieldType::F64 => ValueKind::Double,
            DomainFieldType::Bool => ValueKind::Bool,
            DomainFieldType::CString => ValueKind::CString,
        }
    }
}

#[derive(Clone)]
pub(super) enum DomainFieldInit {
    Int(i64),
    Float(f64),
    Bool(bool),
    CString(String),
}

#[derive(Clone)]
pub(super) struct DomainField {
    pub(super) name: String,
    pub(super) field_type: DomainFieldType,
    pub(super) initializer: DomainFieldInit,
    pub(super) struct_index: usize,
}

impl DomainField {
    pub(super) fn from_var_decl(var_decl: &VariableDeclNode) -> Option<Self> {
        let init_expr_rc = var_decl.get_initializer_value_rc();
        let field_type = infer_domain_type(var_decl, &init_expr_rc)?;
        let initializer = build_domain_initializer(&field_type, &init_expr_rc);
        Some(DomainField {
            name: var_decl.name.clone(),
            field_type,
            initializer,
            struct_index: 0,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum ValueKind {
    I32,
    Double,
    Bool,
    CString,
}

#[derive(Clone)]
pub(super) struct LocalBinding {
    pub(super) ptr: String,
    pub(super) kind: ValueKind,
}

pub(super) struct ValueRef {
    pub(super) kind: ValueKind,
    pub(super) value: String,
}

impl ValueRef {
    pub(super) fn new(kind: ValueKind, value: impl Into<String>) -> Self {
        ValueRef {
            kind,
            value: value.into(),
        }
    }
}

pub(super) fn infer_value_kind_from_type(type_opt: Option<&TypeNode>) -> ValueKind {
    type_opt
        .and_then(|type_node| match type_node.type_str.as_str() {
            "int" | "i32" | "i64" => Some(ValueKind::I32),
            "float" | "double" => Some(ValueKind::Double),
            "bool" => Some(ValueKind::Bool),
            "string" => Some(ValueKind::CString),
            _ => None,
        })
        .unwrap_or(ValueKind::CString)
}

pub(super) fn infer_domain_type(
    var_decl: &VariableDeclNode,
    init_expr: &ExprType,
) -> Option<DomainFieldType> {
    if let Some(type_node) = &var_decl.type_opt {
        match type_node.type_str.as_str() {
            "int" | "i32" | "i64" => Some(DomainFieldType::I32),
            "float" | "double" => Some(DomainFieldType::F64),
            "bool" => Some(DomainFieldType::Bool),
            "string" => Some(DomainFieldType::CString),
            _ => None,
        }
    } else {
        infer_domain_type_from_expr(init_expr)
    }
}

fn infer_domain_type_from_expr(expr: &ExprType) -> Option<DomainFieldType> {
    match expr {
        ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t.clone() {
            TokenType::Number => {
                let value = &literal_expr_node.value;
                if value.contains('.') || value.contains('e') || value.contains('E') {
                    Some(DomainFieldType::F64)
                } else {
                    Some(DomainFieldType::I32)
                }
            }
            TokenType::String => Some(DomainFieldType::CString),
            TokenType::True | TokenType::False => Some(DomainFieldType::Bool),
            _ => None,
        },
        ExprType::UnaryExprT { unary_expr_node } => {
            if matches!(
                unary_expr_node.operator,
                OperatorType::Minus | OperatorType::Plus
            ) {
                let inner = unary_expr_node.right_rcref.borrow();
                infer_domain_type_from_expr(&*inner)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn build_domain_initializer(field_type: &DomainFieldType, expr: &ExprType) -> DomainFieldInit {
    match field_type {
        DomainFieldType::I32 => extract_int_literal(expr)
            .map(DomainFieldInit::Int)
            .unwrap_or(DomainFieldInit::Int(0)),
        DomainFieldType::F64 => extract_float_literal(expr)
            .map(DomainFieldInit::Float)
            .unwrap_or(DomainFieldInit::Float(0.0)),
        DomainFieldType::Bool => extract_bool_literal(expr)
            .map(DomainFieldInit::Bool)
            .unwrap_or(DomainFieldInit::Bool(false)),
        DomainFieldType::CString => extract_string_literal_value(expr)
            .map(DomainFieldInit::CString)
            .unwrap_or_else(|| DomainFieldInit::CString(String::new())),
    }
}

fn extract_int_literal(expr: &ExprType) -> Option<i64> {
    match expr {
        ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t.clone() {
            TokenType::Number => {
                let sanitized = sanitize_numeric_literal(&literal_expr_node.value);
                sanitized.parse::<i64>().ok()
            }
            _ => None,
        },
        ExprType::UnaryExprT { unary_expr_node } => {
            let inner = unary_expr_node.right_rcref.borrow();
            match unary_expr_node.operator {
                OperatorType::Minus => extract_int_literal(&*inner).map(|v| -v),
                OperatorType::Plus => extract_int_literal(&*inner),
                _ => None,
            }
        }
        _ => None,
    }
}

fn extract_float_literal(expr: &ExprType) -> Option<f64> {
    match expr {
        ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t.clone() {
            TokenType::Number => {
                let sanitized = sanitize_numeric_literal(&literal_expr_node.value);
                sanitized.parse::<f64>().ok()
            }
            _ => None,
        },
        ExprType::UnaryExprT { unary_expr_node } => {
            let inner = unary_expr_node.right_rcref.borrow();
            match unary_expr_node.operator {
                OperatorType::Minus => extract_float_literal(&*inner).map(|v| -v),
                OperatorType::Plus => extract_float_literal(&*inner),
                _ => None,
            }
        }
        _ => None,
    }
}

fn extract_bool_literal(expr: &ExprType) -> Option<bool> {
    match expr {
        ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t.clone() {
            TokenType::True => Some(true),
            TokenType::False => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn extract_string_literal_value(expr: &ExprType) -> Option<String> {
    match expr {
        ExprType::LiteralExprT { literal_expr_node } => match literal_expr_node.token_t.clone() {
            TokenType::String => Some(literal_expr_node.value.clone()),
            _ => None,
        },
        _ => None,
    }
}

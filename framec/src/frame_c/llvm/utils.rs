use crate::frame_c::ast::{CallChainNodeType, ExprType};
use std::fmt::Write as _;

pub(super) fn sanitize_identifier(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("unnamed");
    }
    if !out
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic() || c == '_')
        .unwrap_or(false)
    {
        out.insert(0, '_');
    }
    out
}

pub(super) fn to_pascal_case(raw: &str) -> String {
    let mut out = String::new();
    let mut upper = true;
    for ch in raw.chars() {
        if ch == '_' || !ch.is_ascii_alphanumeric() {
            upper = true;
            continue;
        }
        if upper {
            out.push(ch.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(ch.to_ascii_lowercase());
        }
    }
    if out.is_empty() {
        "System".to_string()
    } else {
        out
    }
}

pub(super) fn encode_c_string(text: &str) -> (String, usize) {
    let mut encoded = String::new();
    for byte in text.as_bytes() {
        match byte {
            b'\n' => encoded.push_str("\\0A"),
            b'\r' => encoded.push_str("\\0D"),
            b'\t' => encoded.push_str("\\09"),
            b'\\' => encoded.push_str("\\5C"),
            b'"' => encoded.push_str("\\22"),
            32..=126 => encoded.push(*byte as char),
            other => write!(&mut encoded, "\\{:02X}", other).unwrap(),
        }
    }
    encoded.push_str("\\00");
    (encoded, text.len() + 1)
}

pub(super) fn format_f64(value: f64) -> String {
    if value.is_finite() {
        format!("{:.6e}", value)
    } else {
        "0.000000e+00".to_string()
    }
}

pub(super) fn extract_string_literal(expr: &ExprType) -> Option<String> {
    match expr {
        ExprType::LiteralExprT { literal_expr_node } => Some(literal_expr_node.value.clone()),
        _ => None,
    }
}

pub(super) fn expr_kind(expr: &ExprType) -> &'static str {
    match expr {
        ExprType::SystemInstanceExprT { .. } => "SystemInstance",
        ExprType::CallExprT { .. } => "CallExpr",
        ExprType::CallChainExprT { .. } => "CallChain",
        ExprType::AssignmentExprT { .. } => "Assignment",
        ExprType::VariableExprT { .. } => "Variable",
        ExprType::LiteralExprT { .. } => "Literal",
        ExprType::LambdaExprT { .. } => "Lambda",
        _ => "Other",
    }
}

pub(super) fn call_chain_node_kind(node: &CallChainNodeType) -> &'static str {
    match node {
        CallChainNodeType::VariableNodeT { .. } => "Variable",
        CallChainNodeType::InterfaceMethodCallT { .. } => "InterfaceMethod",
        CallChainNodeType::UndeclaredCallT { .. } => "UndeclaredCall",
        CallChainNodeType::CallChainLiteralExprT { .. } => "LiteralCall",
        CallChainNodeType::UndeclaredIdentifierNodeT { .. } => "UndeclaredId",
        CallChainNodeType::ActionCallT { .. } => "ActionCall",
        CallChainNodeType::OperationCallT { .. } => "OperationCall",
        CallChainNodeType::SelfT { .. } => "Self",
        _ => "Other",
    }
}

pub(super) fn sanitize_numeric_literal(value: &str) -> String {
    value.chars().filter(|c| *c != '_').collect()
}

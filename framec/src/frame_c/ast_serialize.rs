// AST Serialization for debugging and testing
// This module provides JSON serialization of the Frame AST

use serde::{Deserialize, Serialize};
use super::ast::*;
use std::rc::Rc;
use std::cell::RefCell;

/// Simplified, serializable representation of the AST
/// This is designed for debugging and testing, not for round-tripping
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableAst {
    pub module_name: String,
    pub systems: Vec<SerializableSystem>,
    pub functions: Vec<SerializableFunction>,
    pub enums: Vec<SerializableEnum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableSystem {
    pub name: String,
    pub line: usize,
    pub machine: Option<SerializableMachine>,
    pub interface: Vec<SerializableInterfaceMethod>,
    pub operations: Vec<SerializableOperation>,
    pub actions: Vec<SerializableAction>,
    pub domain: Vec<SerializableDomainVar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableMachine {
    pub states: Vec<SerializableState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableState {
    pub name: String,
    pub line: usize,
    pub event_handlers: Vec<SerializableEventHandler>,
    pub parent_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableEventHandler {
    pub event_name: String,
    pub line: usize,
    pub parameters: Vec<String>,
    pub statements: Vec<SerializableStatement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableInterfaceMethod {
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableOperation {
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
    pub is_static: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableAction {
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableDomainVar {
    pub name: String,
    pub line: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableFunction {
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
    pub is_async: bool,
    pub statements: Vec<SerializableStatement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableEnum {
    pub name: String,
    pub line: usize,
    pub members: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SerializableStatement {
    Assignment {
        line: usize,
        left: String,
        right: SerializableExpression,
    },
    VariableDecl {
        line: usize,
        name: String,
        value: Option<SerializableExpression>,
    },
    Return {
        line: usize,
        value: Option<SerializableExpression>,
    },
    Expression {
        line: usize,
        expr: SerializableExpression,
    },
    If {
        line: usize,
        condition: SerializableExpression,
        then_branch: Vec<SerializableStatement>,
        else_branch: Option<Vec<SerializableStatement>>,
    },
    While {
        line: usize,
        condition: SerializableExpression,
        body: Vec<SerializableStatement>,
    },
    For {
        line: usize,
        variable: String,
        iterable: SerializableExpression,
        body: Vec<SerializableStatement>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "expr_type")]
pub enum SerializableExpression {
    Literal {
        value: String,
    },
    Variable {
        name: String,
    },
    Binary {
        left: Box<SerializableExpression>,
        operator: String,
        right: Box<SerializableExpression>,
    },
    Unary {
        operator: String,
        operand: Box<SerializableExpression>,
    },
    Call {
        function: String,
        arguments: Vec<SerializableExpression>,
        resolved_type: Option<String>,  // v0.62: Semantic resolution result
    },
    ActionCall {
        action: String,
        arguments: Vec<SerializableExpression>,
    },
    OperationCall {
        operation: String,
        arguments: Vec<SerializableExpression>,
    },
    InterfaceMethodCall {
        method: String,
        arguments: Vec<SerializableExpression>,
    },
    CallChain {
        nodes: Vec<SerializableCallChainNode>,
    },
    List {
        elements: Vec<SerializableExpression>,
    },
    Dict {
        entries: Vec<(SerializableExpression, SerializableExpression)>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "node_type")]
pub enum SerializableCallChainNode {
    Self_,
    Variable {
        name: String,
    },
    ActionCall {
        name: String,
        parameters: Vec<SerializableExpression>,
    },
    OperationCall {
        name: String,
        parameters: Vec<SerializableExpression>,
    },
    InterfaceMethodCall {
        name: String,
        parameters: Vec<SerializableExpression>,
    },
    UndeclaredCall {
        name: String,
        parameters: Vec<SerializableExpression>,
    },
    ListElement {
        index: Box<SerializableExpression>,
    },
}

/// Trait for converting AST nodes to their serializable representation
pub trait ToSerializable {
    type Output;
    fn to_serializable(&self) -> Self::Output;
}

// We'll implement conversions for the most important nodes first
impl ToSerializable for CallChainExprNode {
    type Output = SerializableExpression;
    
    fn to_serializable(&self) -> Self::Output {
        let mut nodes = Vec::new();
        
        for node in &self.call_chain {
            let serializable_node = match node {
                CallChainNodeType::SelfT { .. } => {
                    SerializableCallChainNode::Self_
                }
                CallChainNodeType::VariableNodeT { var_node } => {
                    SerializableCallChainNode::Variable {
                        name: var_node.id_node.name.lexeme.clone(),
                    }
                }
                CallChainNodeType::ActionCallT { action_call_expr_node } => {
                    SerializableCallChainNode::ActionCall {
                        name: action_call_expr_node.identifier.name.lexeme.clone(),
                        parameters: action_call_expr_node.call_expr_list.exprs_t
                            .iter()
                            .map(|e| expr_to_serializable(e))
                            .collect(),
                    }
                }
                CallChainNodeType::OperationCallT { operation_call_expr_node } => {
                    SerializableCallChainNode::OperationCall {
                        name: operation_call_expr_node.identifier.name.lexeme.clone(),
                        parameters: operation_call_expr_node.call_expr_list.exprs_t
                            .iter()
                            .map(|e| expr_to_serializable(e))
                            .collect(),
                    }
                }
                CallChainNodeType::InterfaceMethodCallT { interface_method_call_expr_node } => {
                    SerializableCallChainNode::InterfaceMethodCall {
                        name: interface_method_call_expr_node.identifier.name.lexeme.clone(),
                        parameters: interface_method_call_expr_node.call_expr_list.exprs_t
                            .iter()
                            .map(|e| expr_to_serializable(e))
                            .collect(),
                    }
                }
                CallChainNodeType::UndeclaredCallT { call_node } => {
                    SerializableCallChainNode::UndeclaredCall {
                        name: call_node.identifier.name.lexeme.clone(),
                        parameters: call_node.call_expr_list.exprs_t
                            .iter()
                            .map(|e| expr_to_serializable(e))
                            .collect(),
                    }
                }
                _ => {
                    // For other types, create a placeholder
                    SerializableCallChainNode::Variable {
                        name: "UNHANDLED".to_string(),
                    }
                }
            };
            nodes.push(serializable_node);
        }
        
        SerializableExpression::CallChain { nodes }
    }
}

/// Helper function to convert OperatorType to string
fn operator_to_string(op: &OperatorType) -> String {
    match op {
        OperatorType::Plus => "+".to_string(),
        OperatorType::Minus => "-".to_string(),
        OperatorType::Multiply => "*".to_string(),
        OperatorType::Divide => "/".to_string(),
        OperatorType::FloorDivide => "//".to_string(),
        OperatorType::Percent => "%".to_string(),
        OperatorType::EqualEqual => "==".to_string(),
        OperatorType::NotEqual => "!=".to_string(),
        OperatorType::Greater => ">".to_string(),
        OperatorType::GreaterEqual => ">=".to_string(),
        OperatorType::Less => "<".to_string(),
        OperatorType::LessEqual => "<=".to_string(),
        OperatorType::LogicalAnd => "and".to_string(),
        OperatorType::LogicalOr => "or".to_string(),
        OperatorType::Not => "not".to_string(),
        OperatorType::BitwiseAnd => "&".to_string(),
        OperatorType::BitwiseOr => "|".to_string(),
        OperatorType::BitwiseXor => "^".to_string(),
        OperatorType::BitwiseNot => "~".to_string(),
        OperatorType::LeftShift => "<<".to_string(),
        OperatorType::RightShift => ">>".to_string(),
        OperatorType::Power => "**".to_string(),
        OperatorType::MatMul => "@".to_string(),
        OperatorType::Is => "is".to_string(),
        OperatorType::IsNot => "is not".to_string(),
        OperatorType::In => "in".to_string(),
        OperatorType::NotIn => "not in".to_string(),
        OperatorType::LogicalXor => "xor".to_string(),
        OperatorType::Negated => "-".to_string(),
        OperatorType::Unknown => "???".to_string(),
    }
}

/// Helper function to convert any ExprType to SerializableExpression
pub fn expr_to_serializable(expr: &ExprType) -> SerializableExpression {
    match expr {
        ExprType::CallChainExprT { call_chain_expr_node } => {
            call_chain_expr_node.to_serializable()
        }
        ExprType::VariableExprT { var_node } => {
            SerializableExpression::Variable {
                name: var_node.id_node.name.lexeme.clone(),
            }
        }
        ExprType::LiteralExprT { literal_expr_node } => {
            SerializableExpression::Literal {
                value: format!("{:?}", literal_expr_node.value),
            }
        }
        ExprType::ActionCallExprT { action_call_expr_node } => {
            SerializableExpression::ActionCall {
                action: action_call_expr_node.identifier.name.lexeme.clone(),
                arguments: action_call_expr_node.call_expr_list.exprs_t
                    .iter()
                    .map(|e| expr_to_serializable(e))
                    .collect(),
            }
        }
        ExprType::CallExprT { call_expr_node } => {
            SerializableExpression::Call {
                function: call_expr_node.identifier.name.lexeme.clone(),
                arguments: call_expr_node.call_expr_list.exprs_t
                    .iter()
                    .map(|e| expr_to_serializable(e))
                    .collect(),
                // v0.62: Include semantic resolution result
                resolved_type: call_expr_node.resolved_type.as_ref().map(|rt| format!("{:?}", rt)),
            }
        }
        ExprType::BinaryExprT { binary_expr_node } => {
            SerializableExpression::Binary {
                left: Box::new(expr_to_serializable(&*binary_expr_node.left_rcref.borrow())),
                operator: operator_to_string(&binary_expr_node.operator),
                right: Box::new(expr_to_serializable(&*binary_expr_node.right_rcref.borrow())),
            }
        }
        ExprType::UnaryExprT { unary_expr_node } => {
            SerializableExpression::Unary {
                operator: operator_to_string(&unary_expr_node.operator),
                operand: Box::new(expr_to_serializable(&*unary_expr_node.right_rcref.borrow())),
            }
        }
        ExprType::NilExprT => {
            SerializableExpression::Literal {
                value: "None".to_string(),
            }
        }
        ExprType::SystemInstanceExprT { .. } => {
            SerializableExpression::Literal {
                value: "system_instance".to_string(),
            }
        }
        ExprType::SystemTypeExprT { .. } => {
            SerializableExpression::Literal {
                value: "system_type".to_string(),
            }
        }
        ExprType::FrameEventExprT { .. } => {
            SerializableExpression::Literal {
                value: "frame_event".to_string(),
            }
        }
        _ => {
            // Placeholder for any remaining unhandled expression types
            SerializableExpression::Literal {
                value: format!("<UNHANDLED_EXPR: {}>", expr.expr_type_name()),
            }
        }
    }
}

/// Serialize a system node
fn serialize_system(system: &SystemNode) -> SerializableSystem {
    SerializableSystem {
        name: system.name.clone(),
        line: system.line,
        machine: system.machine_block_node_opt.as_ref().map(|m| serialize_machine(m)),
        interface: system.interface_block_node_opt.as_ref()
            .map(|i| i.interface_methods.iter().map(|m| serialize_interface_method(m)).collect())
            .unwrap_or_default(),
        operations: system.operations_block_node_opt.as_ref()
            .map(|o| o.operations.iter().map(|op| serialize_operation(op)).collect())
            .unwrap_or_default(),
        actions: system.actions_block_node_opt.as_ref()
            .map(|a| a.actions.iter().map(|action| serialize_action(action)).collect())
            .unwrap_or_default(),
        domain: system.domain_block_node_opt.as_ref()
            .map(|d| d.member_variables.iter().map(|var| serialize_domain_var(var)).collect())
            .unwrap_or_default(),
    }
}

/// Serialize a machine block
fn serialize_machine(machine: &MachineBlockNode) -> SerializableMachine {
    SerializableMachine {
        states: machine.states.iter().map(|s| serialize_state(s)).collect(),
    }
}

/// Serialize a state node
fn serialize_state(state: &Rc<RefCell<StateNode>>) -> SerializableState {
    let state_borrowed = state.borrow();
    SerializableState {
        name: state_borrowed.name.clone(),
        line: state_borrowed.line,
        event_handlers: state_borrowed.evt_handlers_rcref.iter().map(|h| serialize_event_handler(h)).collect(),
        parent_state: None, // StateNode doesn't have parent_state_opt field
    }
}

/// Serialize an event handler
fn serialize_event_handler(handler: &Rc<RefCell<EventHandlerNode>>) -> SerializableEventHandler {
    let handler_borrowed = handler.borrow();
    let event_name = match &handler_borrowed.msg_t {
        MessageType::CustomMessage { message_node } => message_node.name.clone(),
        MessageType::None => "<none>".to_string(),
    };
    SerializableEventHandler {
        event_name,
        line: handler_borrowed.line,
        parameters: vec![], // Simplified for now
        statements: vec![], // Simplified for now
    }
}

/// Serialize an interface method
fn serialize_interface_method(method: &Rc<RefCell<InterfaceMethodNode>>) -> SerializableInterfaceMethod {
    let method_borrowed = method.borrow();
    SerializableInterfaceMethod {
        name: method_borrowed.name.clone(),
        line: 0, // InterfaceMethodNode doesn't have line field
        parameters: method_borrowed.params.as_ref()
            .map(|p| p.iter().map(|param| param.param_name.clone()).collect())
            .unwrap_or_default(),
    }
}

/// Serialize an operation
fn serialize_operation(operation: &Rc<RefCell<OperationNode>>) -> SerializableOperation {
    let op_borrowed = operation.borrow();
    SerializableOperation {
        name: op_borrowed.name.clone(),
        line: op_borrowed.terminator_expr.line,
        parameters: op_borrowed.params.as_ref()
            .map(|p| p.iter().map(|param| param.param_name.clone()).collect())
            .unwrap_or_default(),
        is_static: op_borrowed.is_static(),
    }
}

/// Serialize an action
fn serialize_action(action: &Rc<RefCell<ActionNode>>) -> SerializableAction {
    let action_borrowed = action.borrow();
    SerializableAction {
        name: action_borrowed.name.clone(),
        line: action_borrowed.terminator_expr.line,
        parameters: action_borrowed.params.as_ref()
            .map(|p| p.iter().map(|param| param.param_name.clone()).collect())
            .unwrap_or_default(),
    }
}

/// Serialize a domain variable
fn serialize_domain_var(var: &Rc<RefCell<VariableDeclNode>>) -> SerializableDomainVar {
    let var_borrowed = var.borrow();
    SerializableDomainVar {
        name: var_borrowed.name.clone(),
        line: var_borrowed.line,
    }
}

/// Serialize a function
fn serialize_function(function: &Rc<RefCell<FunctionNode>>) -> SerializableFunction {
    let func_borrowed = function.borrow();
    SerializableFunction {
        name: func_borrowed.name.clone(),
        line: func_borrowed.terminator_expr.line,
        parameters: func_borrowed.params.as_ref()
            .map(|p| p.iter().map(|param| param.param_name.clone()).collect())
            .unwrap_or_default(),
        is_async: func_borrowed.is_async,
        statements: vec![], // Simplified for now
    }
}

/// Serialize an enum
fn serialize_enum(enum_node: &Rc<RefCell<EnumDeclNode>>) -> SerializableEnum {
    let enum_borrowed = enum_node.borrow();
    SerializableEnum {
        name: enum_borrowed.name.clone(),
        line: 0, // EnumDeclNode doesn't have line field
        members: enum_borrowed.enums.iter().map(|item| item.name.clone()).collect(),
    }
}

/// Serialize a statement
fn serialize_statement(stmt: &StatementType) -> SerializableStatement {
    // For now, create simplified placeholder for all statement types
    // TODO: Implement proper serialization for each statement type
    SerializableStatement::Expression {
        line: 0,
        expr: SerializableExpression::Literal {
            value: "<PLACEHOLDER_STATEMENT>".to_string(),
        },
    }
}

/// Main function to serialize an AST to JSON
pub fn serialize_ast_to_json(module: &FrameModule) -> Result<String, serde_json::Error> {
    let ast = SerializableAst {
        module_name: "FrameModule".to_string(),
        systems: module.systems.iter().map(|s| serialize_system(s)).collect(),
        functions: module.functions.iter().map(|f| serialize_function(f)).collect(),
        enums: module.enums.iter().map(|e| serialize_enum(e)).collect(),
    };
    
    serde_json::to_string_pretty(&ast)
}

/// Function to dump a specific expression for debugging
pub fn debug_expression(expr: &ExprType) -> String {
    let serializable = expr_to_serializable(expr);
    serde_json::to_string_pretty(&serializable).unwrap_or_else(|e| format!("Serialization error: {}", e))
}

/// Function to dump a specific statement for debugging
pub fn debug_statement(stmt: &StatementType) -> String {
    let serializable = serialize_statement(stmt);
    serde_json::to_string_pretty(&serializable).unwrap_or_else(|e| format!("Serialization error: {}", e))
}

/// Save AST to file with error handling
pub fn save_ast_to_file(module: &FrameModule, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;
    
    let json = serialize_ast_to_json(module)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Generate AST summary for quick inspection
pub fn ast_summary(module: &FrameModule) -> String {
    let systems_summary: Vec<String> = module.systems.iter().map(|s| {
        let states_count = s.machine_block_node_opt.as_ref()
            .map(|m| m.states.len())
            .unwrap_or(0);
        format!("{} ({} states)", s.name, states_count)
    }).collect();
    
    let functions_summary: Vec<String> = module.functions.iter().map(|f| {
        let f_borrowed = f.borrow();
        let async_marker = if f_borrowed.is_async { "async " } else { "" };
        format!("{}{}", async_marker, f_borrowed.name)
    }).collect();
    
    format!(
        "AST Summary:\n  Systems ({}): {}\n  Functions ({}): {}\n  Enums ({}): {}",
        module.systems.len(),
        if systems_summary.is_empty() { "None".to_string() } else { systems_summary.join(", ") },
        module.functions.len(),
        if functions_summary.is_empty() { "None".to_string() } else { functions_summary.join(", ") },
        module.enums.len(),
        module.enums.iter().map(|e| e.borrow().name.clone()).collect::<Vec<_>>().join(", ")
    )
}

/// Generate detailed line number mapping for debugging
pub fn generate_line_map(module: &FrameModule) -> String {
    let mut lines = Vec::new();
    
    for system in &module.systems {
        lines.push(format!("System '{}' at line {}", system.name, system.line));
        
        if let Some(machine) = &system.machine_block_node_opt {
            for state in &machine.states {
                let state_borrowed = state.borrow();
                lines.push(format!("  State '{}' at line {}", state_borrowed.name, state_borrowed.line));
                for handler in &state_borrowed.evt_handlers_rcref {
                    let handler_borrowed = handler.borrow();
                    let event_name = match &handler_borrowed.msg_t {
                        MessageType::CustomMessage { message_node } => message_node.name.clone(),
                        MessageType::None => "<none>".to_string(),
                    };
                    lines.push(format!("    Handler '{}' at line {}", 
                        event_name, handler_borrowed.line));
                }
            }
        }
        
        if let Some(interface) = &system.interface_block_node_opt {
            for method in &interface.interface_methods {
                let method_borrowed = method.borrow();
                lines.push(format!("  Interface '{}' at line {}", method_borrowed.name, 0));
            }
        }
        
        if let Some(operations) = &system.operations_block_node_opt {
            for op in &operations.operations {
                let op_borrowed = op.borrow();
                lines.push(format!("  Operation '{}' at line {}", op_borrowed.name, op_borrowed.terminator_expr.line));
            }
        }
        
        if let Some(actions) = &system.actions_block_node_opt {
            for action in &actions.actions {
                let action_borrowed = action.borrow();
                lines.push(format!("  Action '{}' at line {}", action_borrowed.name, action_borrowed.terminator_expr.line));
            }
        }
    }
    
    for function in &module.functions {
        let f_borrowed = function.borrow();
        let async_marker = if f_borrowed.is_async { " (async)" } else { "" };
        lines.push(format!("Function '{}' at line {}{}", f_borrowed.name, f_borrowed.line, async_marker));
    }
    
    for enum_node in &module.enums {
        let enum_borrowed = enum_node.borrow();
        lines.push(format!("Enum '{}' at line {}", enum_borrowed.name, 0)); // EnumDeclNode doesn't have line field
    }
    
    lines.join("\n")
}
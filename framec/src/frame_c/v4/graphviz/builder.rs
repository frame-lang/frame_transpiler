/// Builds a SystemGraph IR from SystemAst + Arcanum.

use crate::frame_c::v4::arcanum::Arcanum;
use crate::frame_c::v4::frame_ast::{
    SystemAst, Statement, Expression, Literal, BinaryOp, UnaryOp, Type,
};
use super::ir::{
    SystemGraph, StateNode, HandlerInfo, StateVar,
    StateParam as IrStateParam, TransitionEdge, TransitionTarget, TransitionKind,
};

/// Build a GraphViz IR from a parsed Frame system and its symbol table.
pub fn build_system_graph(system: &SystemAst, _arcanum: &Arcanum) -> SystemGraph {
    let machine = match &system.machine {
        Some(m) => m,
        None => {
            return SystemGraph {
                name: system.name.clone(),
                states: vec![],
                transitions: vec![],
                entry_state: None,
                has_state_stack: false,
            };
        }
    };

    let mut states = Vec::new();
    let mut transitions = Vec::new();
    let mut has_state_stack = false;

    // First pass: collect all states with their metadata
    for state_ast in &machine.states {
        let mut handlers = Vec::new();
        for handler in &state_ast.handlers {
            let params: Vec<(String, String)> = handler
                .params
                .iter()
                .map(|p| (p.name.clone(), format_type(&p.param_type)))
                .collect();
            handlers.push(HandlerInfo {
                event: handler.event.clone(),
                params,
            });
        }

        let state_vars: Vec<StateVar> = state_ast
            .state_vars
            .iter()
            .map(|sv| StateVar {
                name: sv.name.clone(),
                var_type: Some(format_type(&sv.var_type)),
            })
            .collect();

        let state_params: Vec<IrStateParam> = state_ast
            .params
            .iter()
            .map(|sp| IrStateParam {
                name: sp.name.clone(),
                param_type: Some(format_type(&sp.param_type)),
            })
            .collect();

        // Children are derived: any state whose parent == this state's name
        let children: Vec<String> = machine
            .states
            .iter()
            .filter(|s| s.parent.as_deref() == Some(&state_ast.name))
            .map(|s| s.name.clone())
            .collect();

        states.push(StateNode {
            name: state_ast.name.clone(),
            parent: state_ast.parent.clone(),
            children,
            has_enter: state_ast.enter.is_some(),
            has_exit: state_ast.exit.is_some(),
            handlers,
            state_vars,
            state_params,
        });

        // Extract transitions from handlers
        for handler in &state_ast.handlers {
            extract_transitions_from_statements(
                &handler.body.statements,
                &state_ast.name,
                &handler.event,
                None,
                &mut transitions,
                &mut has_state_stack,
            );
        }

        // Extract transitions from enter handler
        if let Some(enter) = &state_ast.enter {
            extract_transitions_from_statements(
                &enter.body.statements,
                &state_ast.name,
                "$>",
                None,
                &mut transitions,
                &mut has_state_stack,
            );
        }

        // Extract transitions from exit handler
        if let Some(exit) = &state_ast.exit {
            extract_transitions_from_statements(
                &exit.body.statements,
                &state_ast.name,
                "<$",
                None,
                &mut transitions,
                &mut has_state_stack,
            );
        }

        // State-level default forward
        if state_ast.default_forward {
            transitions.push(TransitionEdge {
                source: state_ast.name.clone(),
                target: TransitionTarget::ParentForward,
                event: "*".to_string(),
                label: None,
                kind: TransitionKind::Forward,
                guard: None,
            });
        }
    }

    let entry_state = machine.states.first().map(|s| s.name.clone());

    SystemGraph {
        name: system.name.clone(),
        states,
        transitions,
        entry_state,
        has_state_stack,
    }
}

/// Recursively extract transitions from handler body statements.
fn extract_transitions_from_statements(
    statements: &[Statement],
    source_state: &str,
    event: &str,
    guard: Option<&str>,
    transitions: &mut Vec<TransitionEdge>,
    has_state_stack: &mut bool,
) {
    for stmt in statements {
        match stmt {
            Statement::Transition(t) => {
                transitions.push(TransitionEdge {
                    source: source_state.to_string(),
                    target: TransitionTarget::State(t.target.clone()),
                    event: event.to_string(),
                    label: t.label.clone(),
                    kind: TransitionKind::Transition,
                    guard: guard.map(|s| s.to_string()),
                });
            }
            Statement::TransitionForward(tf) => {
                transitions.push(TransitionEdge {
                    source: source_state.to_string(),
                    target: TransitionTarget::State(tf.target.clone()),
                    event: event.to_string(),
                    label: None,
                    kind: TransitionKind::Transition,
                    guard: guard.map(|s| s.to_string()),
                });
            }
            Statement::Forward(_f) => {
                transitions.push(TransitionEdge {
                    source: source_state.to_string(),
                    target: TransitionTarget::ParentForward,
                    event: event.to_string(),
                    label: None,
                    kind: TransitionKind::Forward,
                    guard: guard.map(|s| s.to_string()),
                });
            }
            Statement::StackPush(_) => {
                *has_state_stack = true;
            }
            Statement::StackPop(_) => {
                *has_state_stack = true;
                transitions.push(TransitionEdge {
                    source: source_state.to_string(),
                    target: TransitionTarget::StackPop,
                    event: event.to_string(),
                    label: None,
                    kind: TransitionKind::Transition,
                    guard: guard.map(|s| s.to_string()),
                });
            }
            Statement::If(if_ast) => {
                let condition_text = format_expression(&if_ast.condition);
                let guard_text = match guard {
                    Some(g) => format!("{} && {}", g, condition_text),
                    None => condition_text,
                };
                // Recurse into then branch
                extract_transitions_from_statement(
                    &if_ast.then_branch,
                    source_state,
                    event,
                    Some(&guard_text),
                    transitions,
                    has_state_stack,
                );
                // Recurse into else branch
                if let Some(else_branch) = &if_ast.else_branch {
                    let else_guard = match guard {
                        Some(g) => format!("{} && else", g),
                        None => "else".to_string(),
                    };
                    extract_transitions_from_statement(
                        else_branch,
                        source_state,
                        event,
                        Some(&else_guard),
                        transitions,
                        has_state_stack,
                    );
                }
            }
            // Native code, expressions, returns, continues, loops — skip
            _ => {}
        }
    }
}

/// Extract transitions from a single statement (used for if/else branches).
fn extract_transitions_from_statement(
    stmt: &Statement,
    source_state: &str,
    event: &str,
    guard: Option<&str>,
    transitions: &mut Vec<TransitionEdge>,
    has_state_stack: &mut bool,
) {
    // A branch may be a single statement or wrapped — handle both
    match stmt {
        Statement::Transition(_)
        | Statement::TransitionForward(_)
        | Statement::Forward(_)
        | Statement::StackPush(_)
        | Statement::StackPop(_)
        | Statement::If(_) => {
            extract_transitions_from_statements(
                &[stmt.clone()],
                source_state,
                event,
                guard,
                transitions,
                has_state_stack,
            );
        }
        _ => {}
    }
}

/// Format a Type to its string representation.
fn format_type(t: &Type) -> String {
    match t {
        Type::Custom(s) => s.clone(),
        Type::Unknown => String::new(),
    }
}

/// Format an Expression to a human-readable string for guard labels.
fn format_expression(expr: &Expression) -> String {
    match expr {
        Expression::Var(name) => name.clone(),
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => n.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Bool(b) => b.to_string(),
            Literal::Null => "null".to_string(),
        },
        Expression::Binary { left, op, right } => {
            let l = format_expression(left);
            let r = format_expression(right);
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Mod => "%",
                BinaryOp::Eq => "==",
                BinaryOp::Ne => "!=",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
                BinaryOp::BitAnd => "&",
                BinaryOp::BitOr => "|",
                BinaryOp::BitXor => "^",
            };
            format!("{} {} {}", l, op_str, r)
        }
        Expression::Unary { op, expr } => {
            let e = format_expression(expr);
            match op {
                UnaryOp::Not => format!("!{}", e),
                UnaryOp::Neg => format!("-{}", e),
                UnaryOp::BitNot => format!("~{}", e),
            }
        }
        Expression::NativeExpr(s) => s.clone(),
        Expression::Call { func, args } => {
            let args_str: Vec<String> = args.iter().map(|a| format_expression(a)).collect();
            format!("{}({})", func, args_str.join(", "))
        }
        Expression::Member { object, field } => {
            format!("{}.{}", format_expression(object), field)
        }
        _ => "...".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::frame_ast::{
        Span, MachineAst, StateAst, HandlerAst, HandlerBody,
        TransitionAst, IfAst,
    };

    fn span() -> Span {
        Span { start: 0, end: 0 }
    }

    #[test]
    fn test_empty_system() {
        let system = SystemAst::new("Empty".to_string(), span());
        let arcanum = Arcanum::default();
        let graph = build_system_graph(&system, &arcanum);
        assert_eq!(graph.name, "Empty");
        assert!(graph.states.is_empty());
        assert!(graph.transitions.is_empty());
        assert!(graph.entry_state.is_none());
    }

    #[test]
    fn test_simple_two_state_machine() {
        let mut system = SystemAst::new("Simple".to_string(), span());
        system.machine = Some(MachineAst {
            states: vec![
                StateAst {
                    name: "A".to_string(),
                    params: vec![],
                    parent: None,
                    state_vars: vec![],
                    handlers: vec![HandlerAst {
                        event: "go".to_string(),
                        params: vec![],
                        return_type: None,
                        body: HandlerBody {
                            statements: vec![Statement::Transition(TransitionAst {
                                target: "B".to_string(),
                                args: vec![],
                                label: None,
                                span: span(),
                                indent: 0,
                            })],
                            span: span(),
                        },
                        span: span(),
                    }],
                    enter: None,
                    exit: None,
                    default_forward: false,
                    span: span(),
                    body_span: span(),
                },
                StateAst {
                    name: "B".to_string(),
                    params: vec![],
                    parent: None,
                    state_vars: vec![],
                    handlers: vec![],
                    enter: None,
                    exit: None,
                    default_forward: false,
                    span: span(),
                    body_span: span(),
                },
            ],
            span: span(),
        });

        let arcanum = Arcanum::default();
        let graph = build_system_graph(&system, &arcanum);

        assert_eq!(graph.states.len(), 2);
        assert_eq!(graph.transitions.len(), 1);
        assert_eq!(graph.entry_state, Some("A".to_string()));

        let edge = &graph.transitions[0];
        assert_eq!(edge.source, "A");
        assert_eq!(edge.event, "go");
        match &edge.target {
            TransitionTarget::State(name) => assert_eq!(name, "B"),
            _ => panic!("Expected State target"),
        }
    }

    #[test]
    fn test_hsm_parent_children() {
        let mut system = SystemAst::new("Hsm".to_string(), span());
        system.machine = Some(MachineAst {
            states: vec![
                StateAst {
                    name: "Parent".to_string(),
                    params: vec![],
                    parent: None,
                    state_vars: vec![],
                    handlers: vec![],
                    enter: None,
                    exit: None,
                    default_forward: false,
                    span: span(),
                    body_span: span(),
                },
                StateAst {
                    name: "Child".to_string(),
                    params: vec![],
                    parent: Some("Parent".to_string()),
                    state_vars: vec![],
                    handlers: vec![],
                    enter: None,
                    exit: None,
                    default_forward: false,
                    span: span(),
                    body_span: span(),
                },
            ],
            span: span(),
        });

        let arcanum = Arcanum::default();
        let graph = build_system_graph(&system, &arcanum);

        let parent = graph.states.iter().find(|s| s.name == "Parent").unwrap();
        assert_eq!(parent.children, vec!["Child".to_string()]);
        assert!(parent.parent.is_none());

        let child = graph.states.iter().find(|s| s.name == "Child").unwrap();
        assert_eq!(child.parent, Some("Parent".to_string()));
        assert!(child.children.is_empty());
    }

    #[test]
    fn test_guarded_transitions() {
        let mut system = SystemAst::new("Guarded".to_string(), span());
        system.machine = Some(MachineAst {
            states: vec![
                StateAst {
                    name: "Check".to_string(),
                    params: vec![],
                    parent: None,
                    state_vars: vec![],
                    handlers: vec![HandlerAst {
                        event: "eval".to_string(),
                        params: vec![],
                        return_type: None,
                        body: HandlerBody {
                            statements: vec![Statement::If(IfAst {
                                condition: Expression::Binary {
                                    left: Box::new(Expression::Var("x".to_string())),
                                    op: BinaryOp::Gt,
                                    right: Box::new(Expression::Literal(Literal::Int(0))),
                                },
                                then_branch: Box::new(Statement::Transition(TransitionAst {
                                    target: "Good".to_string(),
                                    args: vec![],
                                    label: None,
                                    span: span(),
                                    indent: 0,
                                })),
                                else_branch: Some(Box::new(Statement::Transition(
                                    TransitionAst {
                                        target: "Bad".to_string(),
                                        args: vec![],
                                        label: None,
                                        span: span(),
                                        indent: 0,
                                    },
                                ))),
                                span: span(),
                            })],
                            span: span(),
                        },
                        span: span(),
                    }],
                    enter: None,
                    exit: None,
                    default_forward: false,
                    span: span(),
                    body_span: span(),
                },
                StateAst {
                    name: "Good".to_string(),
                    params: vec![],
                    parent: None,
                    state_vars: vec![],
                    handlers: vec![],
                    enter: None,
                    exit: None,
                    default_forward: false,
                    span: span(),
                    body_span: span(),
                },
                StateAst {
                    name: "Bad".to_string(),
                    params: vec![],
                    parent: None,
                    state_vars: vec![],
                    handlers: vec![],
                    enter: None,
                    exit: None,
                    default_forward: false,
                    span: span(),
                    body_span: span(),
                },
            ],
            span: span(),
        });

        let arcanum = Arcanum::default();
        let graph = build_system_graph(&system, &arcanum);

        assert_eq!(graph.transitions.len(), 2);
        let to_good = graph
            .transitions
            .iter()
            .find(|t| matches!(&t.target, TransitionTarget::State(n) if n == "Good"))
            .unwrap();
        assert_eq!(to_good.guard, Some("x > 0".to_string()));

        let to_bad = graph
            .transitions
            .iter()
            .find(|t| matches!(&t.target, TransitionTarget::State(n) if n == "Bad"))
            .unwrap();
        assert_eq!(to_bad.guard, Some("else".to_string()));
    }
}

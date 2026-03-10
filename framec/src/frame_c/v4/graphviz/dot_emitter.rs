/// Emits DOT (GraphViz) text from a SystemGraph IR.

use super::ir::*;
use std::collections::HashSet;
use std::fmt::Write;

/// Emit a complete DOT digraph from a SystemGraph.
pub fn emit_dot(graph: &SystemGraph) -> String {
    let mut out = String::new();

    // Graph header + globals (V3 compatible)
    writeln!(out, "digraph {} {{", graph.name).unwrap();
    writeln!(out, "    compound=true").unwrap();
    writeln!(
        out,
        "    node [color=\"deepskyblue4\" style=\"rounded, filled\" fillcolor=\"azure\"]"
    )
    .unwrap();
    writeln!(out, "    edge[color=\"red\"]").unwrap();
    writeln!(out).unwrap();

    // Entry point
    if let Some(entry) = &graph.entry_state {
        writeln!(
            out,
            "    Entry[width=0.2 shape=\"circle\" style=\"filled\" fillcolor=\"black\" label=\"\"]"
        )
        .unwrap();
        writeln!(out, "    Entry -> {}", entry).unwrap();
        writeln!(out).unwrap();
    }

    // Collect parent state names for cluster rendering
    let parent_names: HashSet<&str> = graph
        .states
        .iter()
        .filter(|s| !s.children.is_empty())
        .map(|s| s.name.as_str())
        .collect();

    // Emit states — parents as clusters, leaves as nodes
    // We need to emit in hierarchy order: top-level states first, children inside clusters
    let top_level: Vec<&StateNode> = graph
        .states
        .iter()
        .filter(|s| s.parent.is_none())
        .collect();

    for state in &top_level {
        emit_state_recursive(state, &graph.states, &parent_names, &mut out, 1);
    }

    // State stack pop node (if needed)
    if graph.has_state_stack {
        writeln!(out).unwrap();
        writeln!(
            out,
            "    Stack[shape=\"circle\" label=\"H*\" width=\"0\" margin=\"0\"]"
        )
        .unwrap();
    }

    // Edges
    writeln!(out).unwrap();
    for edge in &graph.transitions {
        emit_edge(edge, &parent_names, &mut out);
    }

    writeln!(out, "}}").unwrap();
    out
}

/// Emit a multi-system DOT output with comment separators.
/// Format required by VSCode extension's parseGraphVizOutput().
pub fn emit_multi_system(systems: &[(String, String)]) -> String {
    if systems.len() == 1 {
        return systems[0].1.clone();
    }

    let mut out = String::new();
    writeln!(out, "// Frame Module: {} systems", systems.len()).unwrap();

    for (name, dot) in systems {
        writeln!(out).unwrap();
        writeln!(out, "// System: {}", name).unwrap();
        out.push_str(dot);
    }
    out
}

/// Recursively emit a state and its children.
fn emit_state_recursive(
    state: &StateNode,
    all_states: &[StateNode],
    parent_names: &HashSet<&str>,
    out: &mut String,
    indent: usize,
) {
    let pad = "    ".repeat(indent);

    if !state.children.is_empty() {
        // Parent state → subgraph cluster
        writeln!(out, "{}subgraph cluster_{} {{", pad, state.name).unwrap();
        writeln!(out, "{}    label = <", pad).unwrap();
        writeln!(
            out,
            "{}        <table cellborder=\"0\" border=\"0\">",
            pad
        )
        .unwrap();
        writeln!(out, "{}            <tr><td>{}</td></tr>", pad, state.name).unwrap();
        writeln!(out, "{}            <hr/>", pad).unwrap();
        writeln!(out, "{}            <tr><td></td></tr>", pad).unwrap();
        writeln!(out, "{}        </table>", pad).unwrap();
        writeln!(out, "{}    >", pad).unwrap();
        writeln!(out, "{}    style = rounded", pad).unwrap();
        // Invisible anchor node for compound edges
        writeln!(
            out,
            "{}    {} [shape=\"point\" width=\"0\"]",
            pad, state.name
        )
        .unwrap();
        writeln!(out).unwrap();

        // Render children inside the cluster
        for child_name in &state.children {
            if let Some(child) = all_states.iter().find(|s| s.name == *child_name) {
                emit_state_recursive(child, all_states, parent_names, out, indent + 1);
            }
        }

        writeln!(out, "{}}}", pad).unwrap();
    } else {
        // Leaf state → HTML-label node
        emit_leaf_node(state, out, &pad);
    }
}

/// Emit a leaf state node with HTML label.
fn emit_leaf_node(state: &StateNode, out: &mut String, pad: &str) {
    writeln!(out, "{}{} [label = <", pad, state.name).unwrap();
    writeln!(
        out,
        "{}    <table CELLBORDER=\"0\" CELLPADDING=\"5\" style=\"rounded\">",
        pad
    )
    .unwrap();

    // Header row: state name (with params if any)
    let header = if state.state_params.is_empty() {
        format!("<b>{}</b>", state.name)
    } else {
        let params: Vec<String> = state
            .state_params
            .iter()
            .map(|p| match &p.param_type {
                Some(t) => format!("{}: {}", p.name, t),
                None => p.name.clone(),
            })
            .collect();
        format!("<b>{}({})</b>", state.name, params.join(", "))
    };
    writeln!(out, "{}        <tr><td>{}</td></tr>", pad, header).unwrap();
    writeln!(out, "{}        <hr/>", pad).unwrap();

    // Body row: handler list (enter/exit + event handlers)
    let has_handlers = state.has_enter || state.has_exit || !state.handlers.is_empty();
    if has_handlers {
        writeln!(
            out,
            "{}        <tr><td align=\"left\"><font point-size=\"10\">",
            pad
        )
        .unwrap();

        let mut handler_lines: Vec<String> = Vec::new();
        if state.has_enter {
            handler_lines.push("$&gt;() [enter]".to_string());
        }
        if state.has_exit {
            handler_lines.push("&lt;$() [exit]".to_string());
        }
        for h in &state.handlers {
            if h.params.is_empty() {
                handler_lines.push(format!("{}()", h.event));
            } else {
                let params: Vec<String> = h
                    .params
                    .iter()
                    .map(|(name, typ)| format!("{}: {}", name, escape_html(typ)))
                    .collect();
                handler_lines.push(format!("{}({})", h.event, params.join(", ")));
            }
        }

        let body_text = handler_lines.join("<br/>\n");
        // Indent each line
        for (i, line) in body_text.split('\n').enumerate() {
            if i > 0 {
                writeln!(out).unwrap();
            }
            write!(out, "{}            {}", pad, line).unwrap();
        }
        writeln!(out).unwrap();

        writeln!(out, "{}        </font></td></tr>", pad).unwrap();
    } else {
        // Empty body row (V3 compatible)
        writeln!(out, "{}        <tr><td></td></tr>", pad).unwrap();
    }

    // State variables section (V4 improvement)
    if !state.state_vars.is_empty() {
        writeln!(out, "{}        <hr/>", pad).unwrap();
        writeln!(
            out,
            "{}        <tr><td align=\"left\"><font point-size=\"9\" color=\"gray40\">",
            pad
        )
        .unwrap();
        let var_lines: Vec<String> = state
            .state_vars
            .iter()
            .map(|sv| match &sv.var_type {
                Some(t) => format!("{}: {}", sv.name, escape_html(t)),
                None => sv.name.clone(),
            })
            .collect();
        write!(
            out,
            "{}            {}",
            pad,
            var_lines.join(&format!("<br/>\n{}            ", pad))
        )
        .unwrap();
        writeln!(out).unwrap();
        writeln!(out, "{}        </font></td></tr>", pad).unwrap();
    }

    writeln!(
        out,
        "{}    </table>",
        pad
    )
    .unwrap();
    writeln!(out, "{}> margin=0 shape=none]", pad).unwrap();
}

/// Emit a transition edge.
fn emit_edge(edge: &TransitionEdge, parent_names: &HashSet<&str>, out: &mut String) {
    let target_node = match &edge.target {
        TransitionTarget::State(name) => name.clone(),
        TransitionTarget::StackPop => "Stack".to_string(),
        TransitionTarget::ParentForward => {
            // Forward to parent — edge goes to the parent state's anchor node
            // (the source state must have a parent; if not, skip)
            return; // Parent forward edges are handled below
        }
    };

    // Build edge label
    let label_text = match &edge.label {
        Some(label) => escape_html(label),
        None => edge.event.clone(),
    };
    let label = match &edge.guard {
        Some(guard) => format!(" {} [{}] ", label_text, escape_html(guard)),
        None => format!(" {} ", label_text),
    };

    // Build edge attributes
    let mut attrs = vec![format!("label=\"{}\"", label)];

    // Edge style based on transition kind
    match edge.kind {
        TransitionKind::ChangeState => {
            attrs.push("style=\"dashed\"".to_string());
        }
        TransitionKind::Forward => {
            attrs.push("style=\"dotted\"".to_string());
            attrs.push("color=\"blue\"".to_string());
        }
        TransitionKind::Transition => {}
    }

    // Compound edge: ltail if source is a parent state
    if parent_names.contains(edge.source.as_str()) {
        attrs.push(format!("ltail=\"cluster_{}\"", edge.source));
    }

    // Compound edge: lhead if target is a parent state
    if let TransitionTarget::State(ref name) = edge.target {
        if parent_names.contains(name.as_str()) {
            attrs.push(format!("lhead=\"cluster_{}\"", name));
        }
    }

    writeln!(
        out,
        "    {} -> {} [{}]",
        edge.source,
        target_node,
        attrs.join(" ")
    )
    .unwrap();
}

/// Escape special HTML characters for DOT HTML labels.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('|', "&#124;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dot_output() {
        let graph = SystemGraph {
            name: "Simple".to_string(),
            states: vec![
                StateNode {
                    name: "A".to_string(),
                    parent: None,
                    children: vec![],
                    has_enter: false,
                    has_exit: false,
                    handlers: vec![HandlerInfo {
                        event: "go".to_string(),
                        params: vec![],
                    }],
                    state_vars: vec![],
                    state_params: vec![],
                },
                StateNode {
                    name: "B".to_string(),
                    parent: None,
                    children: vec![],
                    has_enter: false,
                    has_exit: false,
                    handlers: vec![],
                    state_vars: vec![],
                    state_params: vec![],
                },
            ],
            transitions: vec![TransitionEdge {
                source: "A".to_string(),
                target: TransitionTarget::State("B".to_string()),
                event: "go".to_string(),
                label: None,
                kind: TransitionKind::Transition,
                guard: None,
            }],
            entry_state: Some("A".to_string()),
            has_state_stack: false,
        };

        let dot = emit_dot(&graph);
        assert!(dot.contains("digraph Simple {"));
        assert!(dot.contains("compound=true"));
        assert!(dot.contains("Entry -> A"));
        assert!(dot.contains("A -> B"));
        assert!(dot.contains("label=\" go \""));
        assert!(dot.contains("A [label = <"));
        assert!(dot.contains("go()"));
    }

    #[test]
    fn test_hsm_cluster_output() {
        let graph = SystemGraph {
            name: "Hsm".to_string(),
            states: vec![
                StateNode {
                    name: "Parent".to_string(),
                    parent: None,
                    children: vec!["Child".to_string()],
                    has_enter: false,
                    has_exit: false,
                    handlers: vec![],
                    state_vars: vec![],
                    state_params: vec![],
                },
                StateNode {
                    name: "Child".to_string(),
                    parent: Some("Parent".to_string()),
                    children: vec![],
                    has_enter: false,
                    has_exit: false,
                    handlers: vec![],
                    state_vars: vec![],
                    state_params: vec![],
                },
            ],
            transitions: vec![],
            entry_state: Some("Parent".to_string()),
            has_state_stack: false,
        };

        let dot = emit_dot(&graph);
        assert!(dot.contains("subgraph cluster_Parent {"));
        assert!(dot.contains("Parent [shape=\"point\" width=\"0\"]"));
        assert!(dot.contains("Child [label = <"));
    }

    #[test]
    fn test_stack_pop_node() {
        let graph = SystemGraph {
            name: "Stack".to_string(),
            states: vec![StateNode {
                name: "A".to_string(),
                parent: None,
                children: vec![],
                has_enter: false,
                has_exit: false,
                handlers: vec![],
                state_vars: vec![],
                state_params: vec![],
            }],
            transitions: vec![TransitionEdge {
                source: "A".to_string(),
                target: TransitionTarget::StackPop,
                event: "pop".to_string(),
                label: None,
                kind: TransitionKind::Transition,
                guard: None,
            }],
            entry_state: Some("A".to_string()),
            has_state_stack: true,
        };

        let dot = emit_dot(&graph);
        assert!(dot.contains("Stack[shape=\"circle\" label=\"H*\""));
        assert!(dot.contains("A -> Stack"));
    }

    #[test]
    fn test_multi_system_output() {
        let systems = vec![
            ("Sys1".to_string(), "digraph Sys1 {\n}\n".to_string()),
            ("Sys2".to_string(), "digraph Sys2 {\n}\n".to_string()),
        ];
        let output = emit_multi_system(&systems);
        assert!(output.contains("// Frame Module: 2 systems"));
        assert!(output.contains("// System: Sys1"));
        assert!(output.contains("// System: Sys2"));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("a < b"), "a &lt; b");
        assert_eq!(escape_html("a > b"), "a &gt; b");
        assert_eq!(escape_html("a | b"), "a &#124; b");
    }
}

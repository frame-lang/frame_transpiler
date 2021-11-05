//! This module defines an interface for rendering a Frame state machine in smcat. For the style
//! options, see the [smcat README](https://github.com/sverweij/state-machine-cat/blob/develop/README.md).
//!
//! Some of the style options here (e.g. state and transition types) should more properly be
//! enumerations rather than strings. However, strings are used for simplicity and forward
//! compatibility with new types that may be added.

use crate::info::*;
use crate::live::*;
use std::fmt;

/// Style options for smcat states.
///
/// See:
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#marking-states-active>
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#classes>
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#colors-and-line-width>
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#state-display-names>
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#overriding-the-type-of-a-state>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeStyle {
    pub active: bool,
    pub class: Option<String>,
    pub color: Option<String>,
    pub label: Option<String>,
    pub ntype: Option<String>,
}

/// Style options for smcat transitions.
///
/// See:
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#classes>
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#colors-and-line-width>
///  * <https://github.com/sverweij/state-machine-cat/blob/develop/README.md#internal-and-external-transitions>
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeStyle {
    pub class: Option<String>,
    pub color: Option<String>,
    pub etype: Option<String>,
    pub width: Option<f32>,
}

impl Default for NodeStyle {
    fn default() -> Self {
        NodeStyle {
            active: false,
            class: None,
            color: None,
            label: None,
            ntype: None,
        }
    }
}

impl Default for EdgeStyle {
    fn default() -> Self {
        EdgeStyle {
            class: None,
            color: None,
            etype: None,
            width: None,
        }
    }
}

impl fmt::Display for NodeStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.active {
            parts.push("active".to_string());
        }
        if let Some(s) = &self.class {
            parts.push(format!("class=\"{}\"", s));
        }
        if let Some(s) = &self.color {
            parts.push(format!("color=\"{}\"", s));
        }
        if let Some(s) = &self.label {
            parts.push(format!("label=\"{}\"", s));
        }
        if let Some(s) = &self.ntype {
            parts.push(format!("type={}", s));
        }
        if parts.is_empty() {
            write!(formatter, "")
        } else {
            write!(formatter, " [{}]", parts.join(" "))
        }
    }
}

impl fmt::Display for EdgeStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if let Some(s) = &self.class {
            parts.push(format!("class=\"{}\"", s));
        }
        if let Some(s) = &self.color {
            parts.push(format!("color=\"{}\"", s));
        }
        if let Some(s) = &self.etype {
            parts.push(format!("type={}", s));
        }
        if let Some(f) = &self.width {
            parts.push(format!("width={}", f));
        }
        if parts.is_empty() {
            write!(formatter, "")
        } else {
            write!(formatter, " [{}]", parts.join(" "))
        }
    }
}

/// A trait that enables looking up the style of elements in a state machine.
pub trait Style {
    /// Get the node style for a state.
    fn node(&self, info: &dyn StateInfo, active: bool) -> NodeStyle;
    /// Get the edge style for a transition.
    fn edge(&self, info: &TransitionInfo, active: bool) -> EdgeStyle;
}

/// A style implementation that relegates all formatting to CSS via the "class" style options.
pub struct CssStyle {}

/// A simple style implementation that doesn't require CSS.
pub struct SimpleStyle {}

impl Style for CssStyle {
    fn node(&self, info: &dyn StateInfo, active: bool) -> NodeStyle {
        let mut classes = Vec::new();
        if active {
            classes.push("active");
        }
        if !info.children().is_empty() {
            classes.push("parent");
        }
        if info.is_stack_pop() {
            classes.push("stack-pop");
        }
        NodeStyle {
            class: if classes.is_empty() {
                None
            } else {
                Some(classes.join(" "))
            },
            ..NodeStyle::default()
        }
    }
    fn edge(&self, info: &TransitionInfo, active: bool) -> EdgeStyle {
        let mut classes = Vec::new();
        if active {
            classes.push("active");
        }
        if info.is_change_state() {
            classes.push("change-state");
        }
        EdgeStyle {
            class: if classes.is_empty() {
                None
            } else {
                Some(classes.join(" "))
            },
            ..EdgeStyle::default()
        }
    }
}

impl Style for SimpleStyle {
    fn node(&self, _info: &dyn StateInfo, active: bool) -> NodeStyle {
        let mut style = NodeStyle::default();
        if active {
            style.active = true;
            style.color = Some("red".to_string());
        }
        style
    }
    fn edge(&self, info: &TransitionInfo, active: bool) -> EdgeStyle {
        let mut style = EdgeStyle::default();
        if active {
            style.width = Some(2.0);
            if info.is_change_state() {
                style.color = Some("pink".to_string());
            } else {
                style.color = Some("red".to_string());
            }
        } else if info.is_change_state() {
            style.color = Some("grey".to_string());
        }
        style
    }
}

/// Generates smcat diagrams from Frame state machines.
pub struct Renderer {
    style: Box<dyn Style>,
}

impl Renderer {
    /// Create a new renderer with the given style configuration.
    pub fn new(style: Box<dyn Style>) -> Self {
        Renderer { style }
    }

    /// Generate an smcat diagram illustrating the structure of a state machine, independent of any
    /// particular execution.
    pub fn render_static(&self, machine_info: &dyn MachineInfo) -> String {
        self.render_common(machine_info, None, None)
    }

    /// Generate an smcat diagram from a snapshot of a running state machine. Depending on the
    /// style configuration, this can be expected to highlight the running state, most recent
    /// transition, etc. Eventually, it may show the current values of variables.
    pub fn render_live<'a>(
        &self,
        machine: &'a dyn MachineInstance<'a>,
        last_transition: Option<usize>,
    ) -> String {
        let machine_info = machine.info();
        let active_state = machine.state().info().name();
        self.render_common(machine_info, Some(active_state), last_transition)
    }

    fn render_common(
        &self,
        machine_info: &dyn MachineInfo,
        active_state: Option<&'static str>,
        last_transition: Option<usize>,
    ) -> String {
        let mut output = String::new();

        // render states
        output.push_str("initial,\n");
        self.render_states(
            active_state,
            0,
            &machine_info.top_level_states(),
            &mut output,
        );
        output.push_str(";\n\n");

        // render transitions
        if let Some(init) = machine_info.initial_state() {
            output.push_str(&format!("initial => {};\n", init.name()));
        }
        for transition in machine_info.transitions() {
            self.render_transition(last_transition, &transition, &mut output);
        }
        output
    }

    fn render_states(
        &self,
        active: Option<&'static str>,
        indent: usize,
        states: &[&dyn StateInfo],
        output: &mut String,
    ) {
        let mut state_iter = states.iter().peekable();
        while let Some(state) = state_iter.next() {
            let style = self.style.node(*state, active == Some(state.name()));
            let children = state.children();
            output.push_str(&"  ".repeat(indent));
            output.push_str(&format!("{}{}", state.name(), style));
            if !children.is_empty() {
                output.push_str(" {\n");
                self.render_states(active, indent + 1, &children, output);
                output.push_str(&"  ".repeat(indent));
                output.push('}');
            }
            if state_iter.peek().is_some() {
                output.push_str(",\n");
            }
        }
    }

    fn render_transition(
        &self,
        active: Option<usize>,
        transition: &TransitionInfo,
        output: &mut String,
    ) {
        let style = self.style.edge(transition, active == Some(transition.id));
        let mut label = transition.label.to_string();
        if !label.is_empty() {
            label = format!("/ {}", label);
        }
        output.push_str(&format!(
            "{} -> {}{} : \"  {}{}  \";\n",
            transition.source.name(),
            transition.target.name(),
            style,
            transition.event.name,
            label
        ));
    }
}

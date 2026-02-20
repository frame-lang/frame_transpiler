//! Structural validation pass for Frame V4
//!
//! Validates structural correctness of Frame systems:
//! - E111: Duplicate system parameter
//! - E113: System blocks out of order
//! - E114: Duplicate section block
//! - E115: Multiple fn main functions
//! - E116: Duplicate state name in machine
//! - E117: Duplicate handler in state
//! - E410: Duplicate state variable in state

use crate::frame_c::v4::arcanum::Arcanum;
use crate::frame_c::v4::frame_ast::{FrameAst, SystemAst, SystemSectionKind};
use crate::frame_c::v4::validation::pass::{ValidationContext, ValidationPass};
use crate::frame_c::v4::validation::types::ValidationIssue;

/// Structural validation pass
///
/// Validates that Frame systems have correct structure:
/// - Sections in canonical order
/// - No duplicate sections
/// - No duplicate parameters
pub struct StructuralPass;

impl ValidationPass for StructuralPass {
    fn name(&self) -> &'static str {
        "structural"
    }

    fn run(
        &self,
        ast: &FrameAst,
        _arcanum: &Arcanum,
        _ctx: &mut ValidationContext,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        match ast {
            FrameAst::System(system) => {
                self.validate_system(system, &mut issues);
            }
            FrameAst::Module(module) => {
                // E115: Multiple fn main would be detected by native compiler
                // ModuleAst doesn't track native functions (preserved as bytes)

                // Validate each system
                for system in &module.systems {
                    self.validate_system(system, &mut issues);
                }
            }
        }

        issues
    }
}

impl StructuralPass {
    /// Validate a single system
    fn validate_system(&self, system: &SystemAst, issues: &mut Vec<ValidationIssue>) {
        self.validate_section_order(system, issues);
        self.validate_duplicate_sections(system, issues);
        self.validate_duplicate_params(system, issues);
        self.validate_duplicate_states(system, issues);
        self.validate_state_internals(system, issues);
    }

    /// E113: Validate system section order
    ///
    /// Canonical order: operations:, interface:, machine:, actions:, domain:
    fn validate_section_order(&self, system: &SystemAst, issues: &mut Vec<ValidationIssue>) {
        if system.section_order.is_empty() {
            return;
        }

        // Canonical order indexes
        // Operations=0, Interface=1, Machine=2, Actions=3, Domain=4
        let mut last_idx: i32 = -1;
        for kind in &system.section_order {
            let idx = match kind {
                SystemSectionKind::Operations => 0,
                SystemSectionKind::Interface => 1,
                SystemSectionKind::Machine => 2,
                SystemSectionKind::Actions => 3,
                SystemSectionKind::Domain => 4,
            };
            if (idx as i32) < last_idx {
                issues.push(
                    ValidationIssue::error(
                        "E113",
                        format!(
                            "System '{}' blocks out of order",
                            system.name
                        )
                    )
                    .with_span(system.span.clone())
                    .with_note("Expected order: operations:, interface:, machine:, actions:, domain:")
                    .with_fix("Reorder sections to match canonical order")
                );
                break; // Only report once per system
            }
            last_idx = idx as i32;
        }
    }

    /// E114: Validate no duplicate sections
    fn validate_duplicate_sections(&self, system: &SystemAst, issues: &mut Vec<ValidationIssue>) {
        if let Some(dup_kind) = system.has_duplicate_sections() {
            let section_name = match dup_kind {
                SystemSectionKind::Operations => "operations:",
                SystemSectionKind::Interface => "interface:",
                SystemSectionKind::Machine => "machine:",
                SystemSectionKind::Actions => "actions:",
                SystemSectionKind::Domain => "domain:",
            };
            issues.push(
                ValidationIssue::error(
                    "E114",
                    format!(
                        "Duplicate '{}' section in system '{}'",
                        section_name, system.name
                    )
                )
                .with_span(system.span.clone())
                .with_fix(format!("Remove one of the '{}' sections", section_name))
            );
        }
    }

    /// E111: Validate no duplicate system parameters
    fn validate_duplicate_params(&self, system: &SystemAst, issues: &mut Vec<ValidationIssue>) {
        let mut seen = std::collections::HashSet::new();
        for param in &system.params {
            if !seen.insert(&param.name) {
                issues.push(
                    ValidationIssue::error(
                        "E111",
                        format!(
                            "Duplicate parameter '{}' in system '{}'",
                            param.name, system.name
                        )
                    )
                    .with_span(param.span.clone())
                    .with_fix(format!("Remove or rename the duplicate '{}' parameter", param.name))
                );
            }
        }
    }

    /// E116: Validate no duplicate state names in machine
    fn validate_duplicate_states(&self, system: &SystemAst, issues: &mut Vec<ValidationIssue>) {
        if let Some(machine) = &system.machine {
            let mut seen = std::collections::HashMap::new();
            for state in &machine.states {
                if let Some(first_span) = seen.get(&state.name) {
                    issues.push(
                        ValidationIssue::error(
                            "E116",
                            format!(
                                "Duplicate state '{}' in system '{}'",
                                state.name, system.name
                            )
                        )
                        .with_span(state.span.clone())
                        .with_note(format!("State '{}' was first defined earlier", state.name))
                        .with_fix(format!("Remove or rename one of the '${}' states", state.name))
                    );
                } else {
                    seen.insert(state.name.clone(), state.span.clone());
                }
            }
        }
    }

    /// Validate state internals: E117 (duplicate handlers), E410 (duplicate state vars)
    fn validate_state_internals(&self, system: &SystemAst, issues: &mut Vec<ValidationIssue>) {
        if let Some(machine) = &system.machine {
            for state in &machine.states {
                self.validate_duplicate_handlers(system, state, issues);
                self.validate_duplicate_state_vars(system, state, issues);
            }
        }
    }

    /// E117: Validate no duplicate handlers in a state
    fn validate_duplicate_handlers(&self, system: &SystemAst, state: &crate::frame_c::v4::frame_ast::StateAst, issues: &mut Vec<ValidationIssue>) {
        let mut seen = std::collections::HashMap::new();
        for handler in &state.handlers {
            if let Some(_first_span) = seen.get(&handler.event) {
                issues.push(
                    ValidationIssue::error(
                        "E117",
                        format!(
                            "Duplicate handler '{}' in state '{}' of system '{}'",
                            handler.event, state.name, system.name
                        )
                    )
                    .with_span(handler.span.clone())
                    .with_note(format!("Handler '{}' was defined earlier in this state", handler.event))
                    .with_fix(format!("Remove one of the '{}' handlers", handler.event))
                );
            } else {
                seen.insert(handler.event.clone(), handler.span.clone());
            }
        }
    }

    /// E410: Validate no duplicate state variables in a state
    fn validate_duplicate_state_vars(&self, system: &SystemAst, state: &crate::frame_c::v4::frame_ast::StateAst, issues: &mut Vec<ValidationIssue>) {
        let mut seen = std::collections::HashMap::new();
        for var in &state.state_vars {
            if let Some(_first_span) = seen.get(&var.name) {
                issues.push(
                    ValidationIssue::error(
                        "E410",
                        format!(
                            "Duplicate state variable '$.{}' in state '{}' of system '{}'",
                            var.name, state.name, system.name
                        )
                    )
                    .with_span(var.span.clone())
                    .with_note(format!("State variable '$.{}' was declared earlier", var.name))
                    .with_fix(format!("Remove one of the '$.{}' declarations", var.name))
                );
            } else {
                seen.insert(var.name.clone(), var.span.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_c::v4::frame_ast::Span;

    fn empty_arcanum() -> Arcanum {
        Arcanum::new()
    }

    fn make_context() -> ValidationContext<'static> {
        static CONFIG: crate::frame_c::v4::validation::types::ValidationConfig =
            crate::frame_c::v4::validation::types::ValidationConfig {
                warnings_as_errors: false,
                suppress: Vec::new(),
                verbose: false,
                max_errors: 0,
            };
        ValidationContext::new(&CONFIG)
    }

    #[test]
    fn test_e113_section_order() {
        let pass = StructuralPass;
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.section_order = vec![
            SystemSectionKind::Machine,
            SystemSectionKind::Interface,  // Wrong - should come before machine
        ];

        let ast = FrameAst::System(system);
        let arcanum = empty_arcanum();
        let mut ctx = make_context();

        let issues = pass.run(&ast, &arcanum, &mut ctx);
        assert!(issues.iter().any(|i| i.code == "E113"));
    }

    #[test]
    fn test_e114_duplicate_section() {
        let pass = StructuralPass;
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.section_order = vec![
            SystemSectionKind::Machine,
            SystemSectionKind::Actions,
            SystemSectionKind::Machine,  // Duplicate!
        ];

        let ast = FrameAst::System(system);
        let arcanum = empty_arcanum();
        let mut ctx = make_context();

        let issues = pass.run(&ast, &arcanum, &mut ctx);
        assert!(issues.iter().any(|i| i.code == "E114"));
    }

    #[test]
    fn test_valid_section_order() {
        let pass = StructuralPass;
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.section_order = vec![
            SystemSectionKind::Operations,
            SystemSectionKind::Interface,
            SystemSectionKind::Machine,
            SystemSectionKind::Actions,
            SystemSectionKind::Domain,
        ];

        let ast = FrameAst::System(system);
        let arcanum = empty_arcanum();
        let mut ctx = make_context();

        let issues = pass.run(&ast, &arcanum, &mut ctx);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_e116_duplicate_state() {
        use crate::frame_c::v4::frame_ast::{MachineAst, StateAst};

        let pass = StructuralPass;
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        system.machine = Some(MachineAst {
            states: vec![
                StateAst::new("Idle".to_string(), Span::new(10, 20)),
                StateAst::new("Idle".to_string(), Span::new(30, 40)),  // Duplicate!
            ],
            span: Span::new(5, 50),
        });

        let ast = FrameAst::System(system);
        let arcanum = empty_arcanum();
        let mut ctx = make_context();

        let issues = pass.run(&ast, &arcanum, &mut ctx);
        assert!(issues.iter().any(|i| i.code == "E116"));
    }

    #[test]
    fn test_e117_duplicate_handler() {
        use crate::frame_c::v4::frame_ast::{MachineAst, StateAst, HandlerAst, HandlerBody};

        let pass = StructuralPass;
        let mut system = SystemAst::new("Test".to_string(), Span::new(0, 100));
        let mut state = StateAst::new("Idle".to_string(), Span::new(10, 50));
        state.handlers = vec![
            HandlerAst {
                event: "go".to_string(),
                params: vec![],
                return_type: None,
                body: HandlerBody::empty(Span::new(15, 20)),
                span: Span::new(12, 25),
            },
            HandlerAst {
                event: "go".to_string(),  // Duplicate!
                params: vec![],
                return_type: None,
                body: HandlerBody::empty(Span::new(30, 35)),
                span: Span::new(27, 40),
            },
        ];
        system.machine = Some(MachineAst {
            states: vec![state],
            span: Span::new(5, 55),
        });

        let ast = FrameAst::System(system);
        let arcanum = empty_arcanum();
        let mut ctx = make_context();

        let issues = pass.run(&ast, &arcanum, &mut ctx);
        assert!(issues.iter().any(|i| i.code == "E117"));
    }
}

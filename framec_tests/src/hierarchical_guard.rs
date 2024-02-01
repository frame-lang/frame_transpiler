// //! Test guarded transitions in hierarchical state machines.
//
// #![allow(clippy::nonminimal_bool)]
//
// type Log = Vec<String>;
// include!(concat!(env!("OUT_DIR"), "/", "hierarchical_guard.rs"));
//
// impl HierarchicalGuard {
//     pub fn log(&mut self, msg: String) {
//         self.tape.push(msg);
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     /// Test that basic conditional transitions work properly. In particular,
//     /// that control propagates to a parent handler if a child handler does
//     /// not transition and ends in a continue (`:>`).
//     fn propagate_to_parent() {
//         let mut sm = HierarchicalGuard::new();
//         sm.a(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S0);
//         sm.a(20);
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         assert_eq!(sm.tape, vec!["S0.A"]);
//
//         sm = HierarchicalGuard::new();
//         sm.a(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S0);
//         sm.a(-5);
//         assert_eq!(sm.state, HierarchicalGuardState::S0);
//         assert_eq!(sm.tape, vec!["S0.A", "S.A"]);
//
//         sm = HierarchicalGuard::new();
//         sm.a(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S0);
//         sm.b(-5);
//         assert_eq!(sm.state, HierarchicalGuardState::S1);
//         assert_eq!(sm.tape, vec!["S0.B"]);
//
//         sm = HierarchicalGuard::new();
//         sm.a(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S0);
//         sm.b(5);
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         assert_eq!(sm.tape, vec!["S0.B", "S.B"]);
//     }
//
//     #[test]
//     /// Test that control propagates across across multiple levels if a
//     /// transition is not initiated.
//     fn propagate_multiple_levels() {
//         let mut sm = HierarchicalGuard::new();
//         sm.b(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         sm.a(7);
//         assert_eq!(sm.state, HierarchicalGuardState::S3);
//         assert_eq!(sm.tape, vec!["S2.A", "S1.A"]);
//
//         sm = HierarchicalGuard::new();
//         sm.b(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         sm.a(-5);
//         assert_eq!(sm.state, HierarchicalGuardState::S0);
//         assert_eq!(sm.tape, vec!["S2.A", "S1.A", "S0.A", "S.A"]);
//     }
//
//     #[test]
//     /// Test that propagation of control skips levels that do not contain a
//     /// given handler.
//     fn propagate_skips_levels() {
//         let mut sm = HierarchicalGuard::new();
//         sm.b(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         sm.b(-5);
//         assert_eq!(sm.state, HierarchicalGuardState::S1);
//         assert_eq!(sm.tape, vec!["S2.B", "S0.B"]);
//
//         sm = HierarchicalGuard::new();
//         sm.b(0);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         sm.b(5);
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         assert_eq!(sm.tape, vec!["S2.B", "S0.B", "S.B"]);
//     }
//
//     #[test]
//     /// Test that conditional returns prevent propagation to parents.
//     fn conditional_return() {
//         let mut sm = HierarchicalGuard::new();
//         sm.b(20);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S3);
//         sm.a(5);
//         assert_eq!(sm.state, HierarchicalGuardState::S3);
//         assert_eq!(sm.tape, vec!["S3.A", "stop"]);
//
//         sm = HierarchicalGuard::new();
//         sm.b(20);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S3);
//         sm.a(-5);
//         assert_eq!(sm.state, HierarchicalGuardState::S0);
//         assert_eq!(sm.tape, vec!["S3.A", "continue", "S.A"]);
//
//         sm = HierarchicalGuard::new();
//         sm.b(20);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S3);
//         sm.b(-5);
//         assert_eq!(sm.state, HierarchicalGuardState::S3);
//         assert_eq!(sm.tape, vec!["S3.B", "stop"]);
//
//         sm = HierarchicalGuard::new();
//         sm.b(20);
//         sm.tape.clear();
//         assert_eq!(sm.state, HierarchicalGuardState::S3);
//         sm.b(5);
//         assert_eq!(sm.state, HierarchicalGuardState::S2);
//         assert_eq!(sm.tape, vec!["S3.B", "continue", "S.B"]);
//     }
// }

//! Test conditional expressions and the Boolean branching construct.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "branch.rs"));

impl Branch {
    pub fn log(&mut self, msg: String) {
        self.tape.push(msg);
    }
    pub fn transition_hook(&mut self, _current: BranchState, _next: BranchState) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_if_bool() {
        let mut sm = Branch::new();
        sm.a();
        sm.on_bool(true);
        assert_eq!(sm.state, BranchState::F1);
        assert_eq!(sm.tape, vec!["then 1", "then 2"]);
        sm = Branch::new();
        sm.a();
        sm.on_bool(false);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec!["else 1", "else 2"]);
    }

    #[test]
    fn simple_if_int() {
        let mut sm = Branch::new();
        sm.a();
        sm.on_int(7);
        assert_eq!(sm.state, BranchState::F1);
        assert_eq!(sm.tape, vec!["> 5", "< 10", "== 7"]);
        sm = Branch::new();
        sm.a();
        sm.on_int(-3);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec!["<= 5", "< 10", "!= 7"]);
        sm = Branch::new();
        sm.a();
        sm.on_int(12);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec!["> 5", ">= 10", "!= 7"]);
    }

    #[test]
    fn negated_if_bool() {
        let mut sm = Branch::new();
        sm.b();
        sm.on_bool(true);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec!["else 1", "else 2"]);
        sm = Branch::new();
        sm.b();
        sm.on_bool(false);
        assert_eq!(sm.state, BranchState::F1);
        assert_eq!(sm.tape, vec!["then 1", "then 2"]);
    }

    #[test]
    fn negated_if_int() {
        let mut sm = Branch::new();
        sm.b();
        sm.on_int(7);
        assert_eq!(sm.state, BranchState::F1);
        assert_eq!(sm.tape, vec![">= 5", "<= 10", "== 7"]);
        sm = Branch::new();
        sm.b();
        sm.on_int(5);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec![">= 5", "<= 10", "!= 7"]);
        sm = Branch::new();
        sm.b();
        sm.on_int(10);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec![">= 5", "<= 10", "!= 7"]);
        sm = Branch::new();
        sm.b();
        sm.on_int(0);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec!["< 5", "<= 10", "!= 7"]);
        sm = Branch::new();
        sm.b();
        sm.on_int(100);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec![">= 5", "> 10", "!= 7"]);
    }

    #[test]
    fn operator_precedence() {
        let mut sm = Branch::new();
        sm.c();
        sm.on_int(0);
        assert_eq!(sm.tape, vec!["then 1", "else 2", "then 3", "then 4"]);
        sm.tape.clear();
        sm.on_int(7);
        assert_eq!(sm.tape, vec!["else 1", "then 2", "else 3", "then 4"]);
        sm.tape.clear();
        sm.on_int(-3);
        assert_eq!(sm.tape, vec!["then 1", "else 2", "else 3", "else 4"]);
        sm.tape.clear();
        sm.on_int(12);
        assert_eq!(sm.tape, vec!["else 1", "else 2", "then 3", "else 4"]);
    }

    #[test]
    fn nested_if() {
        let mut sm = Branch::new();
        sm.d();
        sm.on_int(50);
        assert_eq!(sm.state, BranchState::F1);
        assert_eq!(sm.tape, vec!["> 0", "< 100"]);
        sm = Branch::new();
        sm.d();
        sm.on_int(200);
        assert_eq!(sm.state, BranchState::NestedIf);
        assert_eq!(sm.tape, vec!["> 0", ">= 100"]);
        sm = Branch::new();
        sm.d();
        sm.on_int(-5);
        assert_eq!(sm.state, BranchState::NestedIf);
        assert_eq!(sm.tape, vec!["<= 0", "> -10"]);
        sm = Branch::new();
        sm.d();
        sm.on_int(-10);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec!["<= 0", "<= -10"]);
    }

    #[test]
    #[ignore]
    /// Test that a transition within a conditional expression returns from
    /// the handler early. That is, only the first executed transition applies.
    fn transition_returns_early() {
        let mut sm = Branch::new();
        sm.e();
        sm.on_int(5);
        assert_eq!(sm.state, BranchState::F3);
        assert_eq!(sm.tape, vec!["-> $F3"]);
        sm = Branch::new();
        sm.e();
        sm.on_int(15);
        assert_eq!(sm.state, BranchState::F2);
        assert_eq!(sm.tape, vec!["-> $F2"]);
        sm = Branch::new();
        sm.e();
        sm.on_int(115);
        assert_eq!(sm.state, BranchState::F1);
        assert_eq!(sm.tape, vec!["-> $F1"]);
    }
}

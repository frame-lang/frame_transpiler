//! There are five different kinds of variables in Frame. Variables lower in
//! the following list shadow variables higher in the list. Frame uses a
//! variety of sigils to disambiguate potentially shadowed variables, which
//! are indicated in parentheses below.
//!
//!   * domain variables (`#.v`)
//!   * state parameters (`$[v]`)
//!   * state variables (`$.v`)
//!   * event handler parameters (`||[v]`)
//!   * event handler variables (`||.v`)
//!
//! This module tests that variable shadowing and the disambiguation sigils
//! work as expected.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "var_scope.rs"));

#[allow(dead_code)]
impl<'a> VarScope<'a> {
    pub fn log(&mut self, s: String) {
        self.tape.push(s);
    }

    pub fn do_nn(&mut self) {
        self.nn("|nn|[d]".to_string());
    }

    pub fn do_ny(&mut self) {
        self.ny("|ny|[d]".to_string());
    }

    pub fn do_yn(&mut self) {
        self.yn("|yn|[d]".to_string(), "|yn|[x]".to_string());
    }

    pub fn do_yy(&mut self) {
        self.yy("|yy|[d]".to_string(), "|yy|[x]".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected(state: &str, msg: &str, x: &str) -> Log {
        vec![
            "#.a".to_string(),
            format!("${}[b]", state).to_string(),
            format!("${}.c", state).to_string(),
            format!("|{}|[d]", msg).to_string(),
            format!("|{}|.e", msg).to_string(),
            x.to_string(),
        ]
    }

    #[test]
    fn no_shadowing() {
        let mut sm = VarScope::new();
        sm.to_nn();
        sm.do_nn();
        assert_eq!(sm.tape, expected("NN", "nn", "#.x"));
    }

    #[test]
    fn all_shadowing_scenarios() {
        let mut sm = VarScope::new();
        sm.to_nn();
        sm.do_ny();
        assert_eq!(sm.tape, expected("NN", "ny", "|ny|.x"));
        sm.tape.clear();
        sm.do_yn();
        assert_eq!(sm.tape, expected("NN", "yn", "|yn|[x]"));
        sm.tape.clear();
        sm.do_yy();
        assert_eq!(sm.tape, expected("NN", "yy", "|yy|.x"));

        sm = VarScope::new();
        sm.to_ny();
        sm.do_nn();
        assert_eq!(sm.tape, expected("NY", "nn", "$NY.x"));
        sm.tape.clear();
        sm.do_ny();
        assert_eq!(sm.tape, expected("NY", "ny", "|ny|.x"));
        sm.tape.clear();
        sm.do_yn();
        assert_eq!(sm.tape, expected("NY", "yn", "|yn|[x]"));
        sm.tape.clear();
        sm.do_yy();
        assert_eq!(sm.tape, expected("NY", "yy", "|yy|.x"));

        sm = VarScope::new();
        sm.to_yn();
        sm.do_nn();
        assert_eq!(sm.tape, expected("YN", "nn", "$YN[x]"));
        sm.tape.clear();
        sm.do_ny();
        assert_eq!(sm.tape, expected("YN", "ny", "|ny|.x"));
        sm.tape.clear();
        sm.do_yn();
        assert_eq!(sm.tape, expected("YN", "yn", "|yn|[x]"));
        sm.tape.clear();
        sm.do_yy();
        assert_eq!(sm.tape, expected("YN", "yy", "|yy|.x"));

        sm = VarScope::new();
        sm.to_yy();
        sm.do_nn();
        assert_eq!(sm.tape, expected("YY", "nn", "$YY.x"));
        sm.tape.clear();
        sm.do_ny();
        assert_eq!(sm.tape, expected("YY", "ny", "|ny|.x"));
        sm.tape.clear();
        sm.do_yn();
        assert_eq!(sm.tape, expected("YY", "yn", "|yn|[x]"));
        sm.tape.clear();
        sm.do_yy();
        assert_eq!(sm.tape, expected("YY", "yy", "|yy|.x"));
    }

    #[test]
    #[ignore]
    fn disambiguation() {
        let mut sm = VarScope::new();
        sm.to_nn();
        sm.sigils("foo".to_string());
        assert_eq!(sm.tape, vec!["#.x", "foo", "|sigils|.x"]);
        sm = VarScope::new();
        sm.to_ny();
        sm.sigils("foo".to_string());
        assert_eq!(sm.tape, vec!["#.x", "$NY.x", "foo", "|sigils|.x"]);
        sm = VarScope::new();
        sm.to_yn();
        sm.sigils("foo".to_string());
        assert_eq!(sm.tape, vec!["#.x", "$YN[x]", "foo", "|sigils|.x"]);
        sm = VarScope::new();
        sm.to_yy();
        sm.sigils("foo".to_string());
        assert_eq!(sm.tape, vec!["#.x", "$YY[x]", "$YY.x", "foo", "|sigils|.x"]);
    }
}

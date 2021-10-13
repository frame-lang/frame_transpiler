//! Test integer matching and string matching constructs. We're not testing the
//! floating point number matching for now since checking floats for equality
//! is usually not a good idea.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "match.rs"));

impl Match {
    pub fn log(&mut self, msg: String) {
        self.tape.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test matching the empty string.
    /// TODO: Matching the empty string currently only works in multi-string
    /// patterns. The pattern `//`, which should match only the empty string,
    /// instead produces a parse error.
    fn empty_string() {
        let mut sm = Match::new();
        sm.empty();
        sm.on_string("".to_string());
        assert_eq!(sm.tape, vec!["empty"]);
        sm.tape.clear();
        sm.on_string("hi".to_string());
        assert_eq!(sm.tape, vec!["?"]);
    }

    #[test]
    /// Test simple integer matching.
    fn integer_match() {
        let mut sm = Match::new();
        sm.simple();
        sm.on_int(0);
        assert_eq!(sm.tape, vec!["0"]);
        sm.tape.clear();
        sm.on_int(42);
        assert_eq!(sm.tape, vec!["42"]);
        sm.tape.clear();
        sm.on_int(-200);
        assert_eq!(sm.tape, vec!["-200"]);
        sm.tape.clear();
    }

    #[test]
    /// Test simple string matching.
    /// TODO: Testing revealed some limitations:
    ///  * Frame does not support UTF-8 graphemes larger than 1 byte, so we're
    ///    restricted to ASCII.
    ///  * Frame does not have a way to match the '/' or '|' characters,
    ///    which are part of the matching syntax.
    fn string_match() {
        let mut sm = Match::new();
        sm.simple();
        sm.on_string("hello".to_string());
        assert_eq!(sm.tape, vec!["hello"]);
        sm.tape.clear();
        sm.on_string("goodbye".to_string());
        assert_eq!(sm.tape, vec!["goodbye"]);
        sm.tape.clear();
        sm.on_string("Testing 1, 2, 3...".to_string());
        assert_eq!(sm.tape, vec!["testing"]);
        sm.tape.clear();
        sm.on_string("$10!".to_string());
        assert_eq!(sm.tape, vec!["money"]);
        sm.tape.clear();
        sm.on_string("missing".to_string());
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
        sm.on_string("Testing".to_string());
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
        sm.on_string("".to_string());
        assert_eq!(sm.tape, vec!["?"]);
    }

    #[test]
    /// Test the multiple match syntax for integers.
    fn integer_multi_match() {
        let mut sm = Match::new();
        sm.multi();
        sm.on_int(3);
        assert_eq!(sm.tape, vec!["3|-7"]);
        sm.tape.clear();
        sm.on_int(-7);
        assert_eq!(sm.tape, vec!["3|-7"]);
        sm.tape.clear();
        sm.on_int(-4);
        assert_eq!(sm.tape, vec!["-4|5|6"]);
        sm.tape.clear();
        sm.on_int(5);
        assert_eq!(sm.tape, vec!["-4|5|6"]);
        sm.tape.clear();
        sm.on_int(6);
        assert_eq!(sm.tape, vec!["-4|5|6"]);
        sm.tape.clear();
        sm.on_int(4);
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
        sm.on_int(0);
        assert_eq!(sm.tape, vec!["?"]);
    }

    #[test]
    /// Test the multiple match syntax for integers. Also tests matching
    /// whitespace-only strings.
    fn string_multi_match() {
        let mut sm = Match::new();
        sm.multi();
        sm.on_string("$10".to_string());
        assert_eq!(sm.tape, vec!["symbols"]);
        sm.tape.clear();
        sm.on_string("12.5%".to_string());
        assert_eq!(sm.tape, vec!["symbols"]);
        sm.tape.clear();
        sm.on_string("@#*!".to_string());
        assert_eq!(sm.tape, vec!["symbols"]);
        sm.tape.clear();
        sm.on_string(" ".to_string());
        assert_eq!(sm.tape, vec!["whitespace"]);
        sm.tape.clear();
        sm.on_string("  ".to_string());
        assert_eq!(sm.tape, vec!["whitespace"]);
        sm.tape.clear();
        sm.on_string("\t".to_string());
        assert_eq!(sm.tape, vec!["whitespace"]);
        sm.tape.clear();
        sm.on_string("\n".to_string());
        assert_eq!(sm.tape, vec!["whitespace"]);
        sm.tape.clear();
        sm.on_string("10".to_string());
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
        sm.on_string("#".to_string());
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
        sm.on_string("   ".to_string());
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
        sm.on_string("".to_string());
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
    }

    #[test]
    /// Test nested integer matching.
    fn integer_nested_match() {
        let mut sm = Match::new();
        sm.nested();
        sm.on_int(1);
        assert_eq!(sm.tape, vec!["1-3", "1"]);
        sm.tape.clear();
        sm.on_int(2);
        assert_eq!(sm.tape, vec!["1-3", "2"]);
        sm.tape.clear();
        sm.on_int(3);
        assert_eq!(sm.tape, vec!["1-3", "3"]);
        sm.tape.clear();
        sm.on_int(4);
        assert_eq!(sm.tape, vec!["4-5", "4"]);
        sm.tape.clear();
        sm.on_int(5);
        assert_eq!(sm.tape, vec!["4-5", "5"]);
        sm.tape.clear();
        sm.on_int(10);
        assert_eq!(sm.tape, vec!["too big"]);
        sm.tape.clear();
        sm.on_int(0);
        assert_eq!(sm.tape, vec!["too small"]);
    }

    #[test]
    /// Test nested string matching.
    fn string_nested_match() {
        let mut sm = Match::new();
        sm.nested();
        sm.on_string("hello".to_string());
        assert_eq!(sm.tape, vec!["greeting", "English"]);
        sm.tape.clear();
        sm.on_string("hola".to_string());
        assert_eq!(sm.tape, vec!["greeting", "Spanish"]);
        sm.tape.clear();
        sm.on_string("bonjour".to_string());
        assert_eq!(sm.tape, vec!["greeting", "French"]);
        sm.tape.clear();
        sm.on_string("goodbye".to_string());
        assert_eq!(sm.tape, vec!["farewell", "English"]);
        sm.tape.clear();
        sm.on_string("adios".to_string());
        assert_eq!(sm.tape, vec!["farewell", "Spanish"]);
        sm.tape.clear();
        sm.on_string("au revoir".to_string());
        assert_eq!(sm.tape, vec!["farewell", "French"]);
        sm.tape.clear();
        sm.on_string("hallo".to_string());
        assert_eq!(sm.tape, vec!["?"]);
        sm.tape.clear();
        sm.on_string("ciao".to_string());
        assert_eq!(sm.tape, vec!["?"]);
    }

    #[test]
    /// Test hierarchical integer matching.
    fn integer_hierarchical_match() {
        let mut sm = Match::new();
        sm.child();
        sm.on_int(0);
        assert_eq!(sm.state, MatchState::Final);
        assert!(sm.tape.is_empty());

        sm = Match::new();
        sm.child();
        sm.on_int(4);
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["4"]);

        sm.tape.clear();
        sm.on_int(5);
        assert_eq!(sm.state, MatchState::Final);
        assert_eq!(sm.tape, vec!["5"]);

        sm = Match::new();
        sm.child();
        sm.on_int(5);
        assert_eq!(sm.state, MatchState::Final);
        assert_eq!(sm.tape, vec!["5"]);

        sm = Match::new();
        sm.child();
        sm.on_int(3);
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["3", "?"]);

        sm.tape.clear();
        sm.on_int(42);
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["42 in child", "42"]);

        sm.tape.clear();
        sm.on_int(-200);
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["no match in child", "-200"]);

        sm.tape.clear();
        sm.on_int(100);
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["no match in child", "?"]);
    }

    #[test]
    /// Test hierarchical string matching.
    fn string_hierarchical_match() {
        let mut sm = Match::new();
        sm.child();
        sm.on_string("goodbye".to_string());
        assert_eq!(sm.state, MatchState::Final);
        assert!(sm.tape.is_empty());

        sm = Match::new();
        sm.child();
        sm.on_string("hello".to_string());
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["hello in child", "hello"]);

        sm.tape.clear();
        sm.on_string("Testing 1, 2, 3...".to_string());
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["testing in child"]);

        sm.tape.clear();
        sm.on_string("$10!".to_string());
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["no match in child", "money"]);

        sm.tape.clear();
        sm.on_string("testing 1, 2, 3...".to_string());
        assert_eq!(sm.state, MatchState::ChildMatch);
        assert_eq!(sm.tape, vec!["no match in child", "?"]);
    }
}

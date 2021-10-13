//! This file provides some basic tests of Frame's configuration system. There
//! are also many implicit tests of the configuration system throughout the
//! rest of the test suite since several of the test Frame specs set feature
//! options via an attribute at the top of the spec.
//!
//! The code generation options are definitely under-tested.
//!
//! Another aspect that is under-tested is overriding options in local
//! `config.yaml` files. The test suite include a `config.yaml` file that
//! overrides the `generate_action_impl` feature to be `false`, and this works,
//! but it's difficult to write a test for the absence of a trait... One could
//! tweak a user-observable code-generation option in `config.yaml` and test
//! for that, but I don't want to break the existing tests.

type Log = Vec<String>;
include!(concat!(env!("OUT_DIR"), "/", "config.rs"));

impl Config {
    pub fn oh_its_a_transition(&mut self, old_nation: ConfigNation, new_nation: ConfigNation) {
        let s = format!("{:?}->{:?}", old_nation, new_nation);
        self.tape.push(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the code compiles, works right, and the options we set in
    /// attributes are reflected in the generated code.
    #[test]
    fn custom_state_names() {
        let mut sm = Config::new();
        assert_eq!(sm.nation, ConfigNation::A);
        sm.next();
        assert_eq!(sm.nation, ConfigNation::B);
        sm.next();
        assert_eq!(sm.nation, ConfigNation::A);
    }

    /// Test that our customized transition hook method works.
    #[test]
    fn custom_transition_hook() {
        let mut sm = Config::new();
        sm.next();
        sm.next();
        sm.next();
        assert_eq!(sm.tape, vec!["A->B", "B->A", "A->B"]);
    }
}

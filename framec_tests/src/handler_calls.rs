//! Test directly invoking event handlers from within other event handlers.
//! Since event handlers may transition, we conservatively treat such calls
//! as terminating statements for the current handler.

// type Log = Vec<String>;
//include!(concat!(env!("OUT_DIR"), "/", "handler_calls.rs"));

// impl HandlerCalls {
//     pub fn log(&mut self, from: String, val: i32) {
//         self.tape.push(format!("{}({})", from, val));
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[test]
// /// Test that a handler call terminates the current handler.
// fn calls_terminate_handler() {
//     let mut sm = HandlerCalls::new();
//     sm.non_rec();
//     sm.foo(10);
//     assert!(!sm.tape.iter().any(|e| e == "Unreachable(0)"));
// }
//
// #[test]
// /// Test non-recursive handler calls.
// fn non_recursive() {
//     let mut sm = HandlerCalls::new();
//     sm.non_rec();
//     sm.foo(10);
//     assert_eq!(sm.tape, vec!["Foo(10)", "Bar(20)", "Final(30)"]);
// }
//
// #[test]
// /// Test self-recursive handler calls. Also tests calls in the then-branch
// /// of a conditional.
// fn self_recursive() {
//     let mut sm = HandlerCalls::new();
//     sm.self_rec();
//     sm.foo(10);
//     assert_eq!(
//         sm.tape,
//         vec!["Foo(10)", "Foo(20)", "Foo(40)", "Foo(80)", "Final(150)"]
//     );
// }
//
// #[test]
// /// Test self-recursive handler calls. Also tests calls in the else-branch
// /// of conditionals, and calls in integer matching constructs.
// fn mutually_recursive() {
//     let mut sm = HandlerCalls::new();
//     sm.mut_rec();
//     sm.foo(2);
//     assert_eq!(
//         sm.tape,
//         vec![
//             "Foo(2)",
//             "Bar(4)",
//             "Foo(4)",
//             "Bar(8)",
//             "Foo(16)",
//             "Bar(32)",
//             "Foo(96)",
//             "Final(162)"
//         ]
//     );
// }
//
// #[test]
// /// Test handler calls in string matching constructs.
// fn string_match_call() {
//     let mut sm = HandlerCalls::new();
//
//     sm.non_rec();
//     sm.call(String::from("Foo"), 5);
//     assert_eq!(sm.tape, vec!["Foo(5)", "Bar(10)", "Final(15)"]);
//     sm.tape.clear();
//
//     sm.non_rec();
//     sm.call(String::from("Bar"), 20);
//     assert_eq!(sm.tape, vec!["Bar(20)", "Final(20)"]);
//     sm.tape.clear();
//
//     sm.non_rec();
//     sm.call(String::from("Qux"), 37);
//     assert_eq!(sm.tape, vec!["Foo(1000)", "Bar(2000)", "Final(3000)"]);
// }
// }

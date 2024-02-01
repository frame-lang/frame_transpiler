//! Test transition parameters, i.e. arguments passed to the enter/exit handlers during a
//! transition.

// type Log = Vec<String>;
// include!(concat!(env!("OUT_DIR"), "/", "transition_params.rs"));

// impl TransitParams {
//     pub fn log(&mut self, msg: String) {
//         self.tape.push(msg);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use frame_runtime::*;
//     use std::sync::{Arc, Mutex};
//
//     #[test]
//     fn enter() {
//         let mut sm = TransitParams::new();
//         sm.next();
//         assert_eq!(sm.tape, vec!["hi A"]);
//     }
//
//     #[test]
//     fn enter_and_exit() {
//         let mut sm = TransitParams::new();
//         sm.next();
//         sm.tape.clear();
//         sm.next();
//         assert_eq!(sm.tape, vec!["bye A", "hi B", "42"]);
//         sm.tape.clear();
//         sm.next();
//         assert_eq!(sm.tape, vec!["true", "bye B", "hi again A"]);
//     }
//
//
//     fn change_state() {
//         let mut sm = TransitParams::new();
//         assert_eq!(sm.state, TransitParamsState::Init);
//         sm.change();
//         assert_eq!(sm.state, TransitParamsState::A);
//         sm.change();
//         assert_eq!(sm.state, TransitParamsState::B);
//         sm.change();
//         assert_eq!(sm.state, TransitParamsState::A);
//         assert!(sm.tape.is_empty());
//     }
//
//
//     fn change_and_transition() {
//         let mut sm = TransitParams::new();
//         sm.change();
//         assert_eq!(sm.state, TransitParamsState::A);
//         assert!(sm.tape.is_empty());
//         sm.next();
//         assert_eq!(sm.state, TransitParamsState::B);
//         assert_eq!(sm.tape, vec!["bye A", "hi B", "42"]);
//         sm.tape.clear();
//         sm.change();
//         assert_eq!(sm.state, TransitParamsState::A);
//         assert!(sm.tape.is_empty());
//         sm.change();
//         sm.next();
//         assert_eq!(sm.state, TransitParamsState::A);
//         assert_eq!(sm.tape, vec!["true", "bye B", "hi again A"]);
//     }
//
//     /// Test that transition callbacks get event arguments.
//     #[test]
//     fn callbacks_get_event_args() {
//         let mut sm = TransitParams::new();
//         let out = Arc::new(Mutex::new(String::new()));
//         let out_cb = out.clone();
//         sm.event_monitor_mut()
//             .add_transition_callback(Callback::new(
//                 "test",
//                 move |t: &Transition<TransitParams>| {
//                     let mut entry = String::new();
//                     let exit_args = t.exit_arguments();
//                     let enter_args = t.enter_arguments();
//                     if let Some(any) = exit_args.lookup("msg") {
//                         entry
//                             .push_str(&format!("msg: {}, ", any.downcast_ref::<String>().unwrap()));
//                     }
//                     if let Some(any) = exit_args.lookup("val") {
//                         entry.push_str(&format!("val: {}, ", any.downcast_ref::<bool>().unwrap()));
//                     }
//                     entry.push_str(&t.to_string());
//                     if let Some(any) = enter_args.lookup("msg") {
//                         entry
//                             .push_str(&format!(", msg: {}", any.downcast_ref::<String>().unwrap()));
//                     }
//                     if let Some(any) = enter_args.lookup("val") {
//                         entry.push_str(&format!(", val: {}", any.downcast_ref::<i16>().unwrap()));
//                     }
//                     *out_cb.lock().unwrap() = entry;
//                 },
//             ));
//         sm.next();
//         assert_eq!(*out.lock().unwrap(), "Init->A, msg: hi A");
//         sm.next();
//         assert_eq!(*out.lock().unwrap(), "A->B, msg: hi B, val: 42");
//         sm.next();
//         assert_eq!(
//             *out.lock().unwrap(),
//             "msg: bye B, val: true, B->A, msg: hi again A"
//         );
//         sm.change();
//         assert_eq!(*out.lock().unwrap(), "A->>B");
//         sm.change();
//         assert_eq!(*out.lock().unwrap(), "B->>A");
//     }
// }

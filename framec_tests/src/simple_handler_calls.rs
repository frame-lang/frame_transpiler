//! Test directly invoking event handlers from within other event handlers.
//! This module tests this feature for simple state machines not requiring a
//! state context. See `handler_calls.rs` for more interesting cases.

include!(concat!(env!("OUT_DIR"), "/", "simple_handler_calls.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test a basic handler call.
    fn simple_call() {
        let mut sm = SimpleHandlerCalls::new();
        sm.c();
        assert_eq!(sm.state, SimpleHandlerCallsState::A);
    }

    #[test]
    /// Test that a handler call terminates the current handler.
    fn calls_terminate_handler() {
        let mut sm = SimpleHandlerCalls::new();
        sm.d();
        assert_eq!(sm.state, SimpleHandlerCallsState::B);

        sm = SimpleHandlerCalls::new();
        sm.e();
        assert_eq!(sm.state, SimpleHandlerCallsState::B);
    }
}

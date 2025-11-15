@target rust

system RuntimeProtocol {
    interface:
        run() {
            // interface handler
        }

    actions:
        handle_command(message: &str) {
            use std::fmt::Write;
            let mut s = String::new();
            let _ = write!(&mut s, "{}", message);
        }
}

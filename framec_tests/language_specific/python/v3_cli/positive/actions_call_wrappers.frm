@target python_3
# @compile-expect: def _action_log\(
# @compile-expect: def log\(
# @compile-expect: def _action_handle\(
# @compile-expect: def handle\(

system CallMismatch {
    actions:
        log(message) {
            # log sink
            self.last = message
        }
        handle() {
            # Calls 'log' without _action_ prefix; wrappers should preserve FRM names
            self.log("hello")
        }
    machine:
        $S {
            e() { self.handle() }
        }
    domain:
        last = ""
}


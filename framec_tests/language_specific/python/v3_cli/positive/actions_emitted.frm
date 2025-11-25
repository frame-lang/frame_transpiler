@target python
# @compile-expect: def _action_helper\(
# @py-compile

system S {
    machine:
        $A {
            e() {
                self._action_helper()
            }
        }
    actions:
        helper() { pass }
}

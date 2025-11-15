@target python
# @compile-expect: def _action_helper\(

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


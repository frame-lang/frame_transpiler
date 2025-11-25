@target python
# @compile-expect: def e\(
# @compile-expect: def _action_doThing\(
# @py-compile

system S {
    interface:
        e()
    machine:
        $A {
            e() { pass }
        }
    actions:
        doThing() { pass }
}

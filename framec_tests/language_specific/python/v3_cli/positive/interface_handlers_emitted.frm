@target python
# @compile-expect: def e\(
# @compile-expect: def _action_doThing\(

system S {
    interface:
        e() { pass }
    actions:
        doThing() { pass }
}


@target python_3

# V3 capability fixture: operation attributes.

system AttrDemo {
    operations:
        @native
        helper(x) { return x }

    actions:
        doThing() { pass }
}

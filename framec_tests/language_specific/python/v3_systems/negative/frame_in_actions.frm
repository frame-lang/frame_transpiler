@target python

# Frame statements are not allowed in actions; expect policy error E401.
# Note: block order may also trigger E113, but this fixture is focused on E401.
# @expect: E401

system S {
    actions:
        bad() {
            => $^
        }
    machine:
        $A {
            e() { x() }
        }
}

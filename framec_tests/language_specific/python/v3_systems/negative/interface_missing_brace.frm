@target python

# Intentionally malformed: missing '{' after function-style header.
# We expect a structural E111 from the outline/semantic validator.
# @expect: E111

fn ev()  # missing '{' after header

system S {
    machine:
        $A { e() { x() } }
}

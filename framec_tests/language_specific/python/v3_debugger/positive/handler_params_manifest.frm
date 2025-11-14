@target python
# @debug-manifest-expect: system=S; states=A,B
# @debug-manifest-handler-expect: state=A; name=e; params=x,y

system S {
    machine:
        $A {
            e(x, y) {
                -> $B()
            }
        }
        $B { e() { pass } }
}


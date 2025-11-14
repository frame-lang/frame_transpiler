@target python
# @debug-manifest-expect: system=S; states=A,B
# @debug-manifest-handler-expect: state=A; name=e; params=p,q
# @debug-manifest-handler-expect: state=A; name=g; params=x

system S {
    machine:
        $A {
            e(p, q) { -> $B() }
            g(x) { pass }
        }
        $B { e() { pass } }
}


@target typescript
// @debug-manifest-expect: system=S; states=A,B
// @debug-manifest-handler-expect: state=A; name=e; params=x,y

system S {
    machine:
        $A {
            e(x: number, y: string) {
                -> $B()
            }
        }
        $B { e() { } }
}


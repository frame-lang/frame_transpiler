@target typescript
// @expect: E300 E302 E402 E400
// @expect-mode: equal

system S {
    machine:
        $A {
            e() {
                => $^ oops
                -> $ZZ() ; a()
                -> $ (1
            }
        }
}


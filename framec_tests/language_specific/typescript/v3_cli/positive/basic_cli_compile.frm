@target typescript
// @compile-expect: export class S \{

system S {
    machine:
        $A {
            e() {
                -> $B()
            }
        }
        $B { e() { } }
}

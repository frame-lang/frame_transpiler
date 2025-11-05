@target typescript

system NestedThreeLevelsPushPop {
    operations:
    interface:
    machine:
        $A {
            $B {
                $C {
                    start() {
                        -> $$[-]
                    }
                }
            }
        }
    actions:
    domain:
}

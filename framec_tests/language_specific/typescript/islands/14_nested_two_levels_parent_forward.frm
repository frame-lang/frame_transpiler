 

system NestedTwoLevels {
    operations:
    interface:
    machine:
        $Outer {
            start() {
                return
            }

            $Inner {
                start() {
                    => $^
                }
            }
        }
    actions:
    domain:
}

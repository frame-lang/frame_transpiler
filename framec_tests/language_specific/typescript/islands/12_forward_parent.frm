 

system ForwardParent {
    operations:
    interface:
    machine:
        $Outer {
            $Inner {
                start() {
                    => $^
                }
            }
        }
    actions:
    domain:
}

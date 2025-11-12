@target csharp

system S {
    machine:
        $P {
        }
        $A => $P {
            e() {
                => $^
            }
        }
        $B {
        }
}


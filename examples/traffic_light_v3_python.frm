@@target python_3

@@system TrafficLight {
    machine:
        $A {
            tick() {
                print("Red")
                -> $B()
            }
        }
        $B {
            tick() {
                print("Green")
                -> $C()
            }
        }
        $C {
            tick() {
                print("Yellow")
                -> $A()
            }
        }
}

fn main() {
    tl = TrafficLight()
    e = FrameEvent("tick", None)
    tl.tick(e, tl._compartment)
    tl.tick(e, tl._compartment)
    tl.tick(e, tl._compartment)
    tl.tick(e, tl._compartment)
}

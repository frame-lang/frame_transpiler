@target python
# @visitor-map-golden: origins=frame; min=2
# @debug-manifest-expect: system=S; states=A,B,C

system S {
    machine:
        $A {
            e() 
                                next_compartment = FrameCompartment("__S_state_B")
                self._frame_transition(next_compartment)
                return

            
        }
        $B {
            e() 
                y = 1
                                next_compartment = FrameCompartment("__S_state_C")
                next_compartment.state_args = [1, 2]
                self._frame_transition(next_compartment)
                return

            
            f() 
                pass
            
        }
        $C {
            e()  pass 
        }
}


/*#errors-json#
{"errors":[],"schemaVersion":1}
#errors-json#*/

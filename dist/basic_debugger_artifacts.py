@target python
# @visitor-map-golden: origins=frame; min=1
# @debug-manifest-expect: system=S; states=A,B

system S {
    machine:
        $A {
            e() 
                x = 1
                                next_compartment = FrameCompartment("__S_state_B")
                self._frame_transition(next_compartment)
                return

            
        }
        $B { e() { pass } }
}


/*#errors-json#
{"errors":[],"schemaVersion":1}
#errors-json#*/

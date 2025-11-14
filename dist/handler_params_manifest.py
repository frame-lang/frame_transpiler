@target python
# @debug-manifest-expect: system=S; states=A,B
# @debug-manifest-handler-expect: state=A; name=e; params=x,y

system S {
    machine:
        $A {
            e(x, y) 
                                next_compartment = FrameCompartment("__S_state_B")
                self._frame_transition(next_compartment)
                return

            
        }
        $B { e() { pass } }
}


/*#errors-json#
{"errors":[],"schemaVersion":1}
#errors-json#*/

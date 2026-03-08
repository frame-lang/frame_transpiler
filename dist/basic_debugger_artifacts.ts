@target typescript
// @visitor-map-golden: origins=frame; min=1
// @debug-manifest-expect: system=S; states=A,B

system S {
    machine:
        $A {
            e() 
                let x = 1;
                                const nextCompartment = new FrameCompartment("__S_state_B");
                this._frame_transition(nextCompartment);
                return;

            
        }
        $B { e() { } }
}


/*#frame-map#
{"map":[{"targetStart":166,"targetEnd":210,"origin":"native","sourceStart":1,"sourceEnd":45 },{"targetStart":210,"targetEnd":368,"origin":"frame","sourceStart":45,"sourceEnd":52 },{"targetStart":368,"targetEnd":381,"origin":"native","sourceStart":52,"sourceEnd":65 }] ,"version":1,"schemaVersion":1}
#frame-map#*/

/*#visitor-map#
{"mappings":[{"targetLine":8,"targetColumn":17,"sourceLine":1,"sourceColumn":2,"origin":"native"},{"targetLine":10,"targetColumn":17,"sourceLine":3,"sourceColumn":17,"origin":"frame"},{"targetLine":13,"targetColumn":1,"sourceLine":3,"sourceColumn":24,"origin":"native"}] ,"schemaVersion":2}
#visitor-map#*/

/*#debug-manifest#
{"system":"S","states":[{"name":"B","compiledId":"__S_state_B"},{"name":"A","compiledId":"__S_state_A"}],"handlers":[{"state":"A","name":"e","compiledId":"__S_state_A__handler_e"}],"schemaVersion":2}
#debug-manifest#*/

/*#errors-json#
{"errors":[],"schemaVersion":1}
#errors-json#*/

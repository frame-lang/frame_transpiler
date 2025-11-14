@target rust
// @run-expect: TRANSITION:

system S {
    machine:
        $A {
            e() 
                                let next_compartment = FrameCompartment { state: "__S_state_B", ..Default::default() };
                _frame_transition(&next_compartment);
                return;

            
        }
        $B { }
}


/*#frame-map#
{"map":[{"targetStart":95,"targetEnd":112,"origin":"native","sourceStart":1,"sourceEnd":18 },{"targetStart":112,"targetEnd":294,"origin":"frame","sourceStart":18,"sourceEnd":25 },{"targetStart":294,"targetEnd":307,"origin":"native","sourceStart":25,"sourceEnd":38 }] ,"version":1,"schemaVersion":1}
#frame-map#*/

/*#visitor-map#
{"mappings":[{"targetLine":7,"targetColumn":17,"sourceLine":1,"sourceColumn":2,"origin":"native"},{"targetLine":8,"targetColumn":17,"sourceLine":2,"sourceColumn":17,"origin":"frame"},{"targetLine":11,"targetColumn":1,"sourceLine":2,"sourceColumn":24,"origin":"native"}] ,"schemaVersion":2}
#visitor-map#*/

/*#debug-manifest#
{"system":"S","states":[{"name":"A","compiledId":"__S_state_A"},{"name":"B","compiledId":"__S_state_B"}],"handlers":[{"state":"A","name":"e","compiledId":"__S_state_A__handler_e"}],"schemaVersion":2}
#debug-manifest#*/

/*#errors-json#
{"errors":[],"schemaVersion":1}
#errors-json#*/

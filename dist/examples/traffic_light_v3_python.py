from frame_runtime_py import FrameEvent, FrameCompartment

class TrafficLight:
    def __init__(self):
        self._compartment = FrameCompartment("__TrafficLight_state_A")
    def _frame_transition(self, next_compartment: FrameCompartment):
        self._compartment = next_compartment
    def _frame_router(self, __e: FrameEvent, c: FrameCompartment=None):
        # Event router for parent-forward; dispatches by event name.
        compartment = c or self._compartment
        msg = getattr(__e, "_message", None)
        if msg is None:
            return
        # Dispatch to event-specific handler when available.
        if False:
            return  # placeholder for generated clauses
    def _frame_stack_push(self):
        pass
    def _frame_stack_pop(self):
        pass
    def tick(self, __e: FrameEvent, compartment: FrameCompartment):
        c = compartment or self._compartment
        if c.state == "__TrafficLight_state_A":
            
            print("Red")
            next_compartment = FrameCompartment("__TrafficLight_state_B")
            self._frame_transition(next_compartment)
            return
            
            
        elif c.state == "__TrafficLight_state_B":
            
            print("Green")
            next_compartment = FrameCompartment("__TrafficLight_state_C")
            self._frame_transition(next_compartment)
            return
            
            
        elif c.state == "__TrafficLight_state_C":
            
            print("Yellow")
            next_compartment = FrameCompartment("__TrafficLight_state_A")
            self._frame_transition(next_compartment)
            return
            
            

def main():
    
    tl = TrafficLight()
    e = FrameEvent("tick", None)
    tl.tick(e, tl._compartment)
    tl.tick(e, tl._compartment)
    tl.tick(e, tl._compartment)
    tl.tick(e, tl._compartment)

'''/*#frame-map#
{"map":[{"targetStart":85,"targetEnd":131,"origin":"native","sourceStart":1,"sourceEnd":47 },{"targetStart":131,"targetEnd":289,"origin":"frame","sourceStart":47,"sourceEnd":54 },{"targetStart":289,"targetEnd":302,"origin":"native","sourceStart":54,"sourceEnd":67 },{"targetStart":345,"targetEnd":393,"origin":"native","sourceStart":1,"sourceEnd":49 },{"targetStart":393,"targetEnd":551,"origin":"frame","sourceStart":49,"sourceEnd":56 },{"targetStart":551,"targetEnd":564,"origin":"native","sourceStart":56,"sourceEnd":69 },{"targetStart":607,"targetEnd":656,"origin":"native","sourceStart":1,"sourceEnd":50 },{"targetStart":656,"targetEnd":814,"origin":"frame","sourceStart":50,"sourceEnd":57 },{"targetStart":814,"targetEnd":827,"origin":"native","sourceStart":57,"sourceEnd":70 },{"targetStart":851,"targetEnd":1037,"origin":"native","sourceStart":1,"sourceEnd":187 }] ,"version":1,"schemaVersion":1}
#frame-map#*/'''

'''/*#visitor-map#
{"mappings":[{"targetLine":4,"targetColumn":7,"sourceLine":1,"sourceColumn":2,"origin":"native"},{"targetLine":5,"targetColumn":29,"sourceLine":3,"sourceColumn":17,"origin":"frame"},{"targetLine":8,"targetColumn":2,"sourceLine":3,"sourceColumn":24,"origin":"native"},{"targetLine":8,"targetColumn":58,"sourceLine":1,"sourceColumn":2,"origin":"native"},{"targetLine":9,"targetColumn":34,"sourceLine":3,"sourceColumn":17,"origin":"frame"},{"targetLine":13,"targetColumn":9,"sourceLine":3,"sourceColumn":24,"origin":"native"},{"targetLine":14,"targetColumn":46,"sourceLine":1,"sourceColumn":2,"origin":"native"},{"targetLine":16,"targetColumn":16,"sourceLine":3,"sourceColumn":17,"origin":"frame"},{"targetLine":21,"targetColumn":27,"sourceLine":3,"sourceColumn":24,"origin":"native"},{"targetLine":21,"targetColumn":64,"sourceLine":1,"sourceColumn":2,"origin":"native"}] ,"schemaVersion":2}
#visitor-map#*/'''

'''/*#debug-manifest#
{"system":"TrafficLight","states":[{"name":"A","compiledId":"__TrafficLight_state_A"},{"name":"B","compiledId":"__TrafficLight_state_B"},{"name":"C","compiledId":"__TrafficLight_state_C"}],"handlers":[{"state":"A","name":"tick","compiledId":"__TrafficLight_state_A__handler_tick"},{"state":"B","name":"tick","compiledId":"__TrafficLight_state_B__handler_tick"},{"state":"C","name":"tick","compiledId":"__TrafficLight_state_C__handler_tick"}],"schemaVersion":2}
#debug-manifest#*/'''

'''/*#errors-json#
{"errors":[],"schemaVersion":1}
#errors-json#*/'''

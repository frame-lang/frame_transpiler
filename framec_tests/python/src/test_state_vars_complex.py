#Emitted from framec_v0.30.0

from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None, state_vars=None, state_args=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment
        self.state_vars = state_vars or {}
        self.state_args = state_args or {}


def main():
    manager = StateManager()
    manager.start()
    manager.process()
    manager.process()
    manager.finish()
    manager.reset()
    manager.start()
    return
class StateManager:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__statemanager_state_Init', None, None, None, None, {'initCount': 0}, {})
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def start(self,):
        self.return_stack.append(None)
        __e = FrameEvent("start",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def process(self,):
        self.return_stack.append(None)
        __e = FrameEvent("process",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def finish(self,):
        self.return_stack.append(None)
        __e = FrameEvent("finish",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def reset(self,):
        self.return_stack.append(None)
        __e = FrameEvent("reset",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Init
    
    def __statemanager_state_Init(self, __e, compartment):
        if __e._message == "start":
            (compartment.state_vars["initCount"]) = compartment.state_vars["initCount"] + 1
            print("Starting system (attempt #" + str((compartment.state_vars["initCount"])) + ")")
            
            next_compartment = FrameCompartment('__statemanager_state_Working', None, None, None, None, {'itemsProcessed': 0, 'totalTime': 0.0}, {})
            next_compartment.state_vars["itemsProcessed"] = 0
            next_compartment.state_vars["totalTime"] = 0.0
            self.__transition(next_compartment)
            return
        elif __e._message == "reset":
            print("Already in init state, resetting init count")
            (compartment.state_vars["initCount"]) = 0
            return
    
    
    # ----------------------------------------
    # $Working
    
    def __statemanager_state_Working(self, __e, compartment):
        if __e._message == "process":
            (compartment.state_vars["itemsProcessed"]) = compartment.state_vars["itemsProcessed"] + 1
            (compartment.state_vars["totalTime"]) = compartment.state_vars["totalTime"] + 2.5
            print("Processed item #" + str((compartment.state_vars["itemsProcessed"])) + ", total time: " + str((compartment.state_vars["totalTime"])) + "s")
            return
        elif __e._message == "finish":
            print("Finishing work. Processed " + str((compartment.state_vars["itemsProcessed"])) + " items in " + str((compartment.state_vars["totalTime"])) + "s")
            
            next_compartment = FrameCompartment('__statemanager_state_Done', None, None, None, None, {'completionTime': "unknown"}, {})
            next_compartment.state_vars["completionTime"] = "unknown"
            self.__transition(next_compartment)
            return
        elif __e._message == "reset":
            print("Resetting from working state")
            
            next_compartment = FrameCompartment('__statemanager_state_Init', None, None, None, None, {'initCount': 0}, {})
            next_compartment.state_vars["initCount"] = 0
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Done
    
    def __statemanager_state_Done(self, __e, compartment):
        if __e._message == "$>":
            (compartment.state_vars["completionTime"]) = "2024-08-28T10:30:00Z"
            print("Work completed at: " + (compartment.state_vars["completionTime"]))
            return
        elif __e._message == "reset":
            print("Resetting from done state (completed at: " + (compartment.state_vars["completionTime"]) + ")")
            
            next_compartment = FrameCompartment('__statemanager_state_Init', None, None, None, None, {'initCount': 0}, {})
            next_compartment.state_vars["initCount"] = 0
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sInit(self, __e):
        return self.__statemanager_state_Init(__e, None)
    def _sWorking(self, __e):
        return self.__statemanager_state_Working(__e, None)
    def _sDone(self, __e):
        return self.__statemanager_state_Done(__e, None)
    
    # ==================== System Runtime =================== #
    
    def __kernel(self, __e):
        # send event to current state
        self.__router(__e)
        
        # loop until no transitions occur
        while self.__next_compartment != None:
            next_compartment = self.__next_compartment
            self.__next_compartment = None
            
            # exit current state
            self.__router(FrameEvent("<$", self.__compartment.exit_args))
            # change state
            self.__compartment = next_compartment
            
            if next_compartment.forward_event is None:
                # send normal enter event
                self.__router(FrameEvent("$>", self.__compartment.enter_args))
            else:
                # forwarded event
                if next_compartment.forward_event._message == "$>":
                    self.__router(next_compartment.forward_event)
                else:
                    self.__router(FrameEvent("$>", self.__compartment.enter_args))
                    self.__router(next_compartment.forward_event)
                next_compartment.forward_event = None
    
    def __router(self, __e, compartment=None):
        target_compartment = compartment or self.__compartment
        if target_compartment.state == '__statemanager_state_Init':
            self.__statemanager_state_Init(__e, target_compartment)
        elif target_compartment.state == '__statemanager_state_Working':
            self.__statemanager_state_Working(__e, target_compartment)
        elif target_compartment.state == '__statemanager_state_Done':
            self.__statemanager_state_Done(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()

#Emitted from framec_v0.30.0

from enum import Enum

class FrameEvent:
    def __init__(self, message, parameters):
        self._message = message
        self._parameters = parameters

class FrameCompartment:
    def __init__(self, state, forward_event=None, exit_args=None, enter_args=None, parent_compartment=None):
        self.state = state
        self.forward_event = forward_event
        self.exit_args = exit_args
        self.enter_args = enter_args
        self.parent_compartment = parent_compartment


def format_message(prefix,msg):
    print("=== " + prefix + ": " + msg + " ===")
    return prefix + "_" + msg
    return

def log_event(system_name,event_name):
    print("[LOG] System: " + system_name + ", Event: " + event_name)
    return

def main():
    print("Starting Multi-Entity Demo")
    print("---------------------------")
    result = format_message("TEST","helper_works")
    print("Result: " + result)
    counter = CounterSystem()
    counter.increment()
    counter.increment()
    count = counter.get_count()
    print("Counter value: " + str(count))
    counter.reset()
    toggle = ToggleSystem()
    toggle.switch()
    toggle.switch()
    toggle.switch()
    light = TrafficLight()
    light.next()
    light.next()
    light.next()
    light.emergency()
    print("---------------------------")
    print("Multi-Entity Demo Complete")
    return
class CounterSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__countersystem_state_Counting', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.count: int = 0
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def increment(self,):
        self.return_stack.append(None)
        __e = FrameEvent("increment",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def get_count(self,):
        self.return_stack.append(None)
        __e = FrameEvent("get_count",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def reset(self,):
        self.return_stack.append(None)
        __e = FrameEvent("reset",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Counting
    
    def __countersystem_state_Counting(self, __e, compartment):
        if __e._message == "increment":
            log_event("CounterSystem","increment")
            self.count = self.count + 1
            print("Count incremented to: " + str(self.count))
            return
        elif __e._message == "get_count":
            self.return_stack[-1] = self.count
            return
        elif __e._message == "reset":
            log_event("CounterSystem","reset")
            self.count = 0
            print("Counter reset to 0")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sCounting(self, __e):
        return self.__countersystem_state_Counting(__e, None)
    
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
        if target_compartment.state == '__countersystem_state_Counting':
            self.__countersystem_state_Counting(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class ToggleSystem:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__togglesystem_state_Off', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def switch(self,):
        self.return_stack.append(None)
        __e = FrameEvent("switch",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Off
    
    def __togglesystem_state_Off(self, __e, compartment):
        if __e._message == "switch":
            log_event("ToggleSystem","switch_to_on")
            print("Toggle: OFF -> ON")
            next_compartment = FrameCompartment('__togglesystem_state_On', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $On
    
    def __togglesystem_state_On(self, __e, compartment):
        if __e._message == "switch":
            log_event("ToggleSystem","switch_to_off")
            print("Toggle: ON -> OFF")
            next_compartment = FrameCompartment('__togglesystem_state_Off', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sOff(self, __e):
        return self.__togglesystem_state_Off(__e, None)
    def _sOn(self, __e):
        return self.__togglesystem_state_On(__e, None)
    
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
        if target_compartment.state == '__togglesystem_state_Off':
            self.__togglesystem_state_Off(__e, target_compartment)
        elif target_compartment.state == '__togglesystem_state_On':
            self.__togglesystem_state_On(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class TrafficLight:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__trafficlight_state_Green', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def next(self,):
        self.return_stack.append(None)
        __e = FrameEvent("next",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def emergency(self,):
        self.return_stack.append(None)
        __e = FrameEvent("emergency",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Green
    
    def __trafficlight_state_Green(self, __e, compartment):
        if __e._message == "next":
            log_event("TrafficLight","green_to_yellow")
            print("Traffic Light: GREEN -> YELLOW")
            next_compartment = FrameCompartment('__trafficlight_state_Yellow', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "emergency":
            print("EMERGENCY: Going to RED")
            next_compartment = FrameCompartment('__trafficlight_state_Red', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "$>":
            print("Light is now GREEN")
            return
    
    
    # ----------------------------------------
    # $Yellow
    
    def __trafficlight_state_Yellow(self, __e, compartment):
        if __e._message == "next":
            log_event("TrafficLight","yellow_to_red")
            print("Traffic Light: YELLOW -> RED")
            next_compartment = FrameCompartment('__trafficlight_state_Red', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "emergency":
            print("EMERGENCY: Going to RED")
            next_compartment = FrameCompartment('__trafficlight_state_Red', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "$>":
            print("Light is now YELLOW")
            return
    
    
    # ----------------------------------------
    # $Red
    
    def __trafficlight_state_Red(self, __e, compartment):
        if __e._message == "next":
            log_event("TrafficLight","red_to_green")
            print("Traffic Light: RED -> GREEN")
            next_compartment = FrameCompartment('__trafficlight_state_Green', None, None, None, None)
            self.__transition(next_compartment)
            return
        elif __e._message == "emergency":
            print("Already at RED - safe state")
            return
        elif __e._message == "$>":
            print("Light is now RED")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sGreen(self, __e):
        return self.__trafficlight_state_Green(__e, None)
    def _sYellow(self, __e):
        return self.__trafficlight_state_Yellow(__e, None)
    def _sRed(self, __e):
        return self.__trafficlight_state_Red(__e, None)
    
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
        if target_compartment.state == '__trafficlight_state_Green':
            self.__trafficlight_state_Green(__e, target_compartment)
        elif target_compartment.state == '__trafficlight_state_Yellow':
            self.__trafficlight_state_Yellow(__e, target_compartment)
        elif target_compartment.state == '__trafficlight_state_Red':
            self.__trafficlight_state_Red(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()

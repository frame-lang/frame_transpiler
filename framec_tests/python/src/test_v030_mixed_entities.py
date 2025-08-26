#Emitted from framec_v0.30.0


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


def main():
    worker = Worker("task1")
    monitor = Monitor()
    processor = Processor()# DEBUG_EXPR_TYPE: Discriminant(4)
    
    worker# DEBUG_EXPR_TYPE: Discriminant(4)
    
    monitor# DEBUG_EXPR_TYPE: Discriminant(4)
    
    processor# DEBUG_EXPR_TYPE: Discriminant(4)
    
    shared_utility("main")
    return

def shared_utility(source):# DEBUG_EXPR_TYPE: Discriminant(4)
    
    print("Called from: " + source)
    return

def calculate(x,y):
    return x * y + 5
    return
class Worker:
    def __init__(self, arg0):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__worker_state_Idle', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        # Initialize domain variables
        self.progress = 0
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def start(self,):
        self.return_stack.append(None)
        __e = FrameEvent("start",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def finish(self,):
        self.return_stack.append(None)
        __e = FrameEvent("finish",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Idle
    
    def __worker_state_Idle(self, __e, compartment):
        if __e._message == "start":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            shared_utility("Worker")
            result = calculate(10,3)# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Task: " + task_name + ", Result: " + str(result))# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__worker_state_Working', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Working
    
    def __worker_state_Working(self, __e, compartment):
        if __e._message == "finish":# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__worker_state_Done', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Done
    
    def __worker_state_Done(self, __e, compartment):
        pass
        
    
    # ===================== State Dispatchers =================== #
    
    def _sIdle(self, __e):
        return self.__worker_state_Idle(__e, None)
    def _sWorking(self, __e):
        return self.__worker_state_Working(__e, None)
    def _sDone(self, __e):
        return self.__worker_state_Done(__e, None)
    
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
        if target_compartment.state == '__worker_state_Idle':
            self.__worker_state_Idle(__e, target_compartment)
        elif target_compartment.state == '__worker_state_Working':
            self.__worker_state_Working(__e, target_compartment)
        elif target_compartment.state == '__worker_state_Done':
            self.__worker_state_Done(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class Monitor:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__monitor_state_Waiting', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def watch(self,):
        self.return_stack.append(None)
        __e = FrameEvent("watch",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def alert(self,):
        self.return_stack.append(None)
        __e = FrameEvent("alert",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Waiting
    
    def __monitor_state_Waiting(self, __e, compartment):
        if __e._message == "watch":# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__monitor_state_Monitoring', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Monitoring
    
    def __monitor_state_Monitoring(self, __e, compartment):
        if __e._message == "alert":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            shared_utility("Monitor")# DEBUG: TransitionStmt
            
            next_compartment = FrameCompartment('__monitor_state_Alerting', None, None, None, None)
            self.__transition(next_compartment)
            return
    
    
    # ----------------------------------------
    # $Alerting
    
    def __monitor_state_Alerting(self, __e, compartment):
        pass
        
    
    # ===================== State Dispatchers =================== #
    
    def _sWaiting(self, __e):
        return self.__monitor_state_Waiting(__e, None)
    def _sMonitoring(self, __e):
        return self.__monitor_state_Monitoring(__e, None)
    def _sAlerting(self, __e):
        return self.__monitor_state_Alerting(__e, None)
    
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
        if target_compartment.state == '__monitor_state_Waiting':
            self.__monitor_state_Waiting(__e, target_compartment)
        elif target_compartment.state == '__monitor_state_Monitoring':
            self.__monitor_state_Monitoring(__e, target_compartment)
        elif target_compartment.state == '__monitor_state_Alerting':
            self.__monitor_state_Alerting(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment
class Processor:
    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__processor_state_Ready', None, None, None, None)
        self.__next_compartment = None
        self.return_stack = [None]
        
        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)
    # ==================== Interface Block ================== #
    
    def execute(self,):
        self.return_stack.append(None)
        __e = FrameEvent("execute",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    def reset(self,):
        self.return_stack.append(None)
        __e = FrameEvent("reset",None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)
    
    # ===================== Machine Block =================== #
    
    
    # ----------------------------------------
    # $Ready
    
    def __processor_state_Ready(self, __e, compartment):
        if __e._message == "execute":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            shared_utility("Processor")# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Processing complete")
            return
        elif __e._message == "reset":# DEBUG_EXPR_TYPE: Discriminant(4)
            
            print("Reset processor")
            return
    
    # ===================== State Dispatchers =================== #
    
    def _sReady(self, __e):
        return self.__processor_state_Ready(__e, None)
    
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
        if target_compartment.state == '__processor_state_Ready':
            self.__processor_state_Ready(__e, target_compartment)
    
    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment

if __name__ == '__main__':
    main()
# Emitted from framec_v0.81.2


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

# Domain block
class MinimalDebugProtocol:

    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__minimaldebugprotocol_state_Disconnected', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]

        # Initialize domain variables
        self.debugPort = 0
        self.breakpoints = []
        self.currentLine = 0
        self.connectionAttempts = 0

        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)

    # ==================== Interface Block ==================

    def initialize(self, port):
        self.return_stack.append(None)
        __e = FrameEvent("initialize", {"port": port})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def connect(self,):
        self.return_stack.append(None)
        __e = FrameEvent("connect", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def disconnect(self,):
        self.return_stack.append(None)
        __e = FrameEvent("disconnect", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def handleContinue(self,):
        self.return_stack.append(None)
        __e = FrameEvent("handleContinue", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def handleStep(self,):
        self.return_stack.append(None)
        __e = FrameEvent("handleStep", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def handleBreakpoint(self, line):
        self.return_stack.append(None)
        __e = FrameEvent("handleBreakpoint", {"line": line})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def canExecuteCommand(self, command):
        self.return_stack.append(None)
        __e = FrameEvent("canExecuteCommand", {"command": command})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def getCurrentState(self,):
        self.return_stack.append(None)
        __e = FrameEvent("getCurrentState", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    # ===================== Machine Block ===================
    # Machine block
    # State: Disconnected

    def __handle_disconnected_initialize(self, __e, compartment):
        port = __e._parameters.get("port") if __e._parameters else None
        print(f"Initializing with port {port}")
        self.debugPort = port
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Connecting', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_disconnected_connect(self, __e, compartment):
        print("Cannot connect - not initialized")
        return

    def __handle_disconnected_handleContinue(self, __e, compartment):
        print("Cannot continue - not connected")
        return

    def __handle_disconnected_getCurrentState(self, __e, compartment):
        self.return_stack[-1] = "disconnected"
        return
    # State: Connecting

    def __handle_connecting_enter(self, __e, compartment):
        print(f"Attempting to connect to port {self.debugPort}")
        self.connectionAttempts = self.connectionAttempts + 1
        return

    def __handle_connecting_connect(self, __e, compartment):
        print("Connection established")
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Initializing', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_connecting_disconnect(self, __e, compartment):
        print("Aborting connection attempt")
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Disconnected', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_connecting_getCurrentState(self, __e, compartment):
        self.return_stack[-1] = "connecting"
        return
    # State: Initializing

    def __handle_initializing_enter(self, __e, compartment):
        print("Sending initialization data")
        return

    def __handle_initializing_handleContinue(self, __e, compartment):
        print("Starting execution")
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Running', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_initializing_handleBreakpoint(self, __e, compartment):
        line = __e._parameters.get("line") if __e._parameters else None
        print(f"Adding breakpoint at line {line}")
        self.breakpoints.append(line)
        return

    def __handle_initializing_getCurrentState(self, __e, compartment):
        self.return_stack[-1] = "initializing"
        return
    # State: Running

    def __handle_running_handleContinue(self, __e, compartment):
        print("Already running - ignoring continue")
        return

    def __handle_running_handleStep(self, __e, compartment):
        print("Cannot step while running")
        self.return_stack[-1] = False
        return

    def __handle_running_handleBreakpoint(self, __e, compartment):
        line = __e._parameters.get("line") if __e._parameters else None
        if line in self.breakpoints:
            print(f"Hit breakpoint at line {line}")
            self.currentLine = line
            next_compartment = FrameCompartment('__minimaldebugprotocol_state_Paused', None, None, None, None, {}, {})
            self.__transition(next_compartment)
        else:
            print(f"Line {line} is not a breakpoint")
        return

    def __handle_running_canExecuteCommand(self, __e, compartment):
        command = __e._parameters.get("command") if __e._parameters else None
        if command == "continue":
            self.return_stack[-1] = False
            return
        elif command == "step":
            self.return_stack[-1] = False
            return
        elif command == "pause":
            self.return_stack[-1] = True
            return
        else:
            self.return_stack[-1] = False
            return

    def __handle_running_getCurrentState(self, __e, compartment):
        self.return_stack[-1] = "running"
        return

    def __handle_running_disconnect(self, __e, compartment):
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Disconnecting', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return
    # State: Paused

    def __handle_paused_enter(self, __e, compartment):
        print(f"Paused at line {self.currentLine}")
        return

    def __handle_paused_handleContinue(self, __e, compartment):
        print("Resuming execution")
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Running', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_paused_handleStep(self, __e, compartment):
        print("Stepping to next line")
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Stepping', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_paused_canExecuteCommand(self, __e, compartment):
        command = __e._parameters.get("command") if __e._parameters else None
        if command in ["continue", "step", "stepOver", "stepOut"]:
            self.return_stack[-1] = True
            return
        elif command == "pause":
            self.return_stack[-1] = False
            return
        else:
            self.return_stack[-1] = True
            return

    def __handle_paused_getCurrentState(self, __e, compartment):
        self.return_stack[-1] = "paused"
        return

    def __handle_paused_disconnect(self, __e, compartment):
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Disconnecting', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return
    # State: Stepping

    def __handle_stepping_enter(self, __e, compartment):
        print("Executing step operation")
        self.currentLine = self.currentLine + 1
        return

    def __handle_stepping_handleBreakpoint(self, __e, compartment):
        line = __e._parameters.get("line") if __e._parameters else None
        self.currentLine = line
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Paused', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_stepping_handleContinue(self, __e, compartment):
        print("Step interrupted by continue")
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Running', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_stepping_canExecuteCommand(self, __e, compartment):
        command = __e._parameters.get("command") if __e._parameters else None
        self.return_stack[-1] = False
        return

    def __handle_stepping_getCurrentState(self, __e, compartment):
        self.return_stack[-1] = "stepping"
        return
    # State: Disconnecting

    def __handle_disconnecting_enter(self, __e, compartment):
        print("Closing connection")
        self.debugPort = 0
        self.breakpoints = []
        self.currentLine = 0
        return

    def __handle_disconnecting_disconnect(self, __e, compartment):
        print("Cleanup complete")
        next_compartment = FrameCompartment('__minimaldebugprotocol_state_Disconnected', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_disconnecting_getCurrentState(self, __e, compartment):
        self.return_stack[-1] = "disconnecting"
        return

    # ===================== State Dispatchers ===================

    # ----------------------------------------
    # $Disconnected

    def __minimaldebugprotocol_state_Disconnected(self, __e, compartment):
        if __e._message == "initialize":
            return self.__handle_disconnected_initialize(__e, compartment)
        elif __e._message == "connect":
            return self.__handle_disconnected_connect(__e, compartment)
        elif __e._message == "handleContinue":
            return self.__handle_disconnected_handleContinue(__e, compartment)
        elif __e._message == "getCurrentState":
            return self.__handle_disconnected_getCurrentState(__e, compartment)


    # ----------------------------------------
    # $Connecting

    def __minimaldebugprotocol_state_Connecting(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_connecting_enter(__e, compartment)
        elif __e._message == "connect":
            return self.__handle_connecting_connect(__e, compartment)
        elif __e._message == "disconnect":
            return self.__handle_connecting_disconnect(__e, compartment)
        elif __e._message == "getCurrentState":
            return self.__handle_connecting_getCurrentState(__e, compartment)


    # ----------------------------------------
    # $Initializing

    def __minimaldebugprotocol_state_Initializing(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_initializing_enter(__e, compartment)
        elif __e._message == "handleContinue":
            return self.__handle_initializing_handleContinue(__e, compartment)
        elif __e._message == "handleBreakpoint":
            return self.__handle_initializing_handleBreakpoint(__e, compartment)
        elif __e._message == "getCurrentState":
            return self.__handle_initializing_getCurrentState(__e, compartment)


    # ----------------------------------------
    # $Running

    def __minimaldebugprotocol_state_Running(self, __e, compartment):
        if __e._message == "handleContinue":
            return self.__handle_running_handleContinue(__e, compartment)
        elif __e._message == "handleStep":
            return self.__handle_running_handleStep(__e, compartment)
        elif __e._message == "handleBreakpoint":
            return self.__handle_running_handleBreakpoint(__e, compartment)
        elif __e._message == "canExecuteCommand":
            return self.__handle_running_canExecuteCommand(__e, compartment)
        elif __e._message == "getCurrentState":
            return self.__handle_running_getCurrentState(__e, compartment)
        elif __e._message == "disconnect":
            return self.__handle_running_disconnect(__e, compartment)


    # ----------------------------------------
    # $Paused

    def __minimaldebugprotocol_state_Paused(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_paused_enter(__e, compartment)
        elif __e._message == "handleContinue":
            return self.__handle_paused_handleContinue(__e, compartment)
        elif __e._message == "handleStep":
            return self.__handle_paused_handleStep(__e, compartment)
        elif __e._message == "canExecuteCommand":
            return self.__handle_paused_canExecuteCommand(__e, compartment)
        elif __e._message == "getCurrentState":
            return self.__handle_paused_getCurrentState(__e, compartment)
        elif __e._message == "disconnect":
            return self.__handle_paused_disconnect(__e, compartment)


    # ----------------------------------------
    # $Stepping

    def __minimaldebugprotocol_state_Stepping(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_stepping_enter(__e, compartment)
        elif __e._message == "handleBreakpoint":
            return self.__handle_stepping_handleBreakpoint(__e, compartment)
        elif __e._message == "handleContinue":
            return self.__handle_stepping_handleContinue(__e, compartment)
        elif __e._message == "canExecuteCommand":
            return self.__handle_stepping_canExecuteCommand(__e, compartment)
        elif __e._message == "getCurrentState":
            return self.__handle_stepping_getCurrentState(__e, compartment)


    # ----------------------------------------
    # $Disconnecting

    def __minimaldebugprotocol_state_Disconnecting(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_disconnecting_enter(__e, compartment)
        elif __e._message == "disconnect":
            return self.__handle_disconnecting_disconnect(__e, compartment)
        elif __e._message == "getCurrentState":
            return self.__handle_disconnecting_getCurrentState(__e, compartment)


    # ===================== Actions Block ===================
    # Actions block

    def __MinimalDebugProtocol__addBreakpoint(self, line):
        if line not in self.breakpoints:
            self.breakpoints.append(line)
            print(f"Breakpoint added at line {line}")


    def __MinimalDebugProtocol__removeBreakpoint(self, line):
        if line in self.breakpoints:
            self.breakpoints.remove(line)
            print(f"Breakpoint removed from line {line}")


    def __MinimalDebugProtocol__getBreakpoints(self):
        return self.breakpoints


    # ==================== System Runtime ===================

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
        if target_compartment.state == '__minimaldebugprotocol_state_Disconnected':
            self.__minimaldebugprotocol_state_Disconnected(__e, target_compartment)
        elif target_compartment.state == '__minimaldebugprotocol_state_Connecting':
            self.__minimaldebugprotocol_state_Connecting(__e, target_compartment)
        elif target_compartment.state == '__minimaldebugprotocol_state_Initializing':
            self.__minimaldebugprotocol_state_Initializing(__e, target_compartment)
        elif target_compartment.state == '__minimaldebugprotocol_state_Running':
            self.__minimaldebugprotocol_state_Running(__e, target_compartment)
        elif target_compartment.state == '__minimaldebugprotocol_state_Paused':
            self.__minimaldebugprotocol_state_Paused(__e, target_compartment)
        elif target_compartment.state == '__minimaldebugprotocol_state_Stepping':
            self.__minimaldebugprotocol_state_Stepping(__e, target_compartment)
        elif target_compartment.state == '__minimaldebugprotocol_state_Disconnecting':
            self.__minimaldebugprotocol_state_Disconnecting(__e, target_compartment)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment


# Module-level singleton instance for MinimalDebugProtocol
_minimaldebugprotocol_instance = None


def addBreakpoint(line):
    global _minimaldebugprotocol_instance
    if _minimaldebugprotocol_instance is None:
        _minimaldebugprotocol_instance = MinimalDebugProtocol()
    return _minimaldebugprotocol_instance._MinimalDebugProtocol__MinimalDebugProtocol__addBreakpoint(line)

def removeBreakpoint(line):
    global _minimaldebugprotocol_instance
    if _minimaldebugprotocol_instance is None:
        _minimaldebugprotocol_instance = MinimalDebugProtocol()
    return _minimaldebugprotocol_instance._MinimalDebugProtocol__MinimalDebugProtocol__removeBreakpoint(line)

def getBreakpoints():
    global _minimaldebugprotocol_instance
    if _minimaldebugprotocol_instance is None:
        _minimaldebugprotocol_instance = MinimalDebugProtocol()
    return _minimaldebugprotocol_instance._MinimalDebugProtocol__MinimalDebugProtocol__getBreakpoints()

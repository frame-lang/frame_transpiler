# Emitted from framec_v0.78.1


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
    print("=== Testing Frame Operations ===")
    foo()
    service = TestService()
    print("\n--- Testing external interface calls ---")
    service.performAction("external_action")
    result = service.calculate(10, 20)
    print("External calculate result: " + str(result))
    print("\n--- Testing internal operation calls via state transitions ---")
    service.process()
    print("\n--- Testing static operations ---")
    version = TestService.getVersion()
    print("Static getVersion: " + str(version))
    config = TestService.getDefaultConfig()
    print("Static getDefaultConfig: " + str(config))
    TestService.logMessage("Static operation test complete")
    print("\n--- Testing complex operations ---")
    data = {"key": "value", "count": 42}
    processed = service.processData(data)
    print("Processed data: " + str(processed))
    print("\n--- Testing operation error handling ---")
    service.testError()
    service.getStatus()
    print("\n=== All Operation Tests Complete ===")


def foo():
    print("foo")

class TestService:

    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__testservice_state_Start', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]

        # Initialize domain variables
        self.counter = 0
        self.lastAction = None
        self.config = None

        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)

    # ==================== Interface Block ==================

    def performAction(self, action):
        self.return_stack.append(None)
        __e = FrameEvent("performAction", {"action": action})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def calculate(self, x, y):
        self.return_stack.append(None)
        __e = FrameEvent("calculate", {"x": x, "y": y})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def process(self,):
        self.return_stack.append(None)
        __e = FrameEvent("process", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def processData(self, data):
        self.return_stack.append(None)
        __e = FrameEvent("processData", {"data": data})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def testError(self,):
        self.return_stack.append(None)
        __e = FrameEvent("testError", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def getStatus(self,):
        self.return_stack.append(None)
        __e = FrameEvent("getStatus", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    # ===================== Machine Block ===================

    def __handle_start_enter(self, __e, compartment):
        self._log("System initialized in Start state")
        self._incrementCounter()
        return

    def __handle_start_performAction(self, __e, compartment):
        action = __e._parameters.get("action") if __e._parameters else None
        isValid = self._validateAction(action)
        if isValid:
            self.lastAction = action
            self._log("Action performed: " + str(action))
            self._incrementCounter()
        else:
            self._log("Action rejected: " + str(action))
        return

    def __handle_start_calculate(self, __e, compartment):
        x = __e._parameters.get("x") if __e._parameters else None
        y = __e._parameters.get("y") if __e._parameters else None
        result = self._performCalculation(x, y)
        self._log((("Calculate called with: " + str(x)) + ", ") + str(y))
        self.return_stack[-1] = result
        return

    def __handle_start_process(self, __e, compartment):
        self._log("Processing started")
        self._incrementCounter()
        next_compartment = FrameCompartment('__testservice_state_Processing', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_start_processData(self, __e, compartment):
        data = __e._parameters.get("data") if __e._parameters else None
        self._log("Processing data externally")
        result = self._processInternally(data)
        self._incrementCounter()
        self.return_stack[-1] = result
        return

    def __handle_start_testError(self, __e, compartment):
        self._log("Testing error handling")
        try:
            x = 1 / 0
        except:
            self._log("Error caught and handled")
        return

    def __handle_start_getStatus(self, __e, compartment):
        print((("Status: Start state, counter=" + str(self.counter)) + ", lastAction=") + str(self.lastAction))
        return

    def __handle_processing_enter(self, __e, compartment):
        self._log("Entered Processing state")
        self._incrementCounter()
        calc = self._performCalculation(5, 10)
        self._log("Auto-calculation result: " + str(calc))
        next_compartment = FrameCompartment('__testservice_state_Ready', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_ready_enter(self, __e, compartment):
        self._log("System ready")
        self.config = TestService.getDefaultConfig()
        self._log("Config loaded: " + str(self.config))
        next_compartment = FrameCompartment('__testservice_state_Start', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_ready_getStatus(self, __e, compartment):
        print((("Status: Ready state, counter=" + str(self.counter)) + ", config=") + str(self.config))
        return

    # ===================== State Dispatchers ===================

    # ----------------------------------------
    # $Start

    def __testservice_state_Start(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_start_enter(__e, compartment)
        elif __e._message == "performAction":
            return self.__handle_start_performAction(__e, compartment)
        elif __e._message == "calculate":
            return self.__handle_start_calculate(__e, compartment)
        elif __e._message == "process":
            return self.__handle_start_process(__e, compartment)
        elif __e._message == "processData":
            return self.__handle_start_processData(__e, compartment)
        elif __e._message == "testError":
            return self.__handle_start_testError(__e, compartment)
        elif __e._message == "getStatus":
            return self.__handle_start_getStatus(__e, compartment)


    # ----------------------------------------
    # $Processing

    def __testservice_state_Processing(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_processing_enter(__e, compartment)


    # ----------------------------------------
    # $Ready

    def __testservice_state_Ready(self, __e, compartment):
        if __e._message == "$>":
            return self.__handle_ready_enter(__e, compartment)
        elif __e._message == "getStatus":
            return self.__handle_ready_getStatus(__e, compartment)


    # ==================== Operations Block =================

    def _incrementCounter(self):
        self.counter = self.counter + 1
        print("Counter incremented to: " + str(self.counter))


    def _performCalculation(self, a, b):
        result = (a + b) + self.counter
        print((((((("Internal calculation: " + str(a)) + " + ") + str(b)) + " + ") + str(self.counter)) + " = ") + str(result))
        return result


    def _validateAction(self, action):
        if action == None:
            print("Action validation failed: None")
            return False
        if action == "":
            print("Action validation failed: empty string")
            return False
        print("Action validated: " + str(action))
        return True


    def _processInternally(self, data):
        print("Processing internally: " + str(data))
        if "count" in data:
            data["count"] = data["count"] * 2
        data["processed"] = True
        data["processor"] = "TestService"
        return data


    def _log(self, message):
        print("[LOG] " + str(message))


    @staticmethod
    def getVersion():
        return "1.0.0"


    @staticmethod
    def getDefaultConfig():
        return {"timeout": 30, "retries": 3, "mode": "production"}


    @staticmethod
    def logMessage(msg):
        print("[STATIC LOG] " + str(msg))


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
        if target_compartment.state == '__testservice_state_Start':
            self.__testservice_state_Start(__e, target_compartment)
        elif target_compartment.state == '__testservice_state_Processing':
            self.__testservice_state_Processing(__e, target_compartment)
        elif target_compartment.state == '__testservice_state_Ready':
            self.__testservice_state_Ready(__e, target_compartment)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment


if __name__ == '__main__':
    main()


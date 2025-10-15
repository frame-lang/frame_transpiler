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


def main():
    checker = NoneChecker()
    checker.check(None)
    val = checker.getValue()
    checker.process()
    print("Got value: " + str(val))

class NoneChecker:

    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__nonechecker_state_Idle', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]

        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)

    # ==================== Interface Block ==================

    def check(self, value):
        self.return_stack.append(None)
        __e = FrameEvent("check", {"value": value})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def getValue(self,):
        self.return_stack.append(None)
        __e = FrameEvent("getValue", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def process(self,):
        self.return_stack.append(None)
        __e = FrameEvent("process", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    # ===================== Machine Block ===================
    # Machine block
    # State: Idle

    def __handle_idle_check(self, __e, compartment):
        value = __e._parameters.get("value") if __e._parameters else None
        if value == None:
            print("Value is None")
        else:
            print("Value is not None")
        return

    def __handle_idle_getValue(self, __e, compartment):
        self.return_stack[-1] = 42
        return

    def __handle_idle_process(self, __e, compartment):
        print("Processing")
        return

    # ===================== State Dispatchers ===================

    # ----------------------------------------
    # $Idle

    def __nonechecker_state_Idle(self, __e, compartment):
        if __e._message == "check":
            return self.__handle_idle_check(__e, compartment)
        elif __e._message == "getValue":
            return self.__handle_idle_getValue(__e, compartment)
        elif __e._message == "process":
            return self.__handle_idle_process(__e, compartment)


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
        if target_compartment.state == '__nonechecker_state_Idle':
            self.__nonechecker_state_Idle(__e, target_compartment)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment


if __name__ == '__main__':
    main()

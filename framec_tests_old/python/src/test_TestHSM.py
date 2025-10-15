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
    sys = ContinueTerminatorDemo()
    sys.passMe1()
    sys.passMe2()

class ContinueTerminatorDemo:

    def __init__(self):
        # Create and initialize start state compartment
        parent_compartment = FrameCompartment('__continueterminatordemo_state_Parent', None, None, None, None, {}, {})
        self.__compartment = FrameCompartment('__continueterminatordemo_state_Child', None, None, None, parent_compartment, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]

        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)

    # ==================== Interface Block ==================

    def passMe1(self,):
        self.return_stack.append(None)
        __e = FrameEvent("passMe1", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def passMe2(self,):
        self.return_stack.append(None)
        __e = FrameEvent("passMe2", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    # ===================== Machine Block ===================
    # Machine block
    # State: Child

    def __handle_child_passMe1(self, __e, compartment):
        self.__router(__e, compartment.parent_compartment)
        return

    def __handle_child_passMe2(self, __e, compartment):
        print("handled in $Child")
        self.__router(__e, compartment.parent_compartment)
        return
    # State: Parent

    def __handle_parent_passMe1(self, __e, compartment):
        print("handled in $Parent")
        return

    def __handle_parent_passMe2(self, __e, compartment):
        print("handled in $Parent")
        return

    # ===================== State Dispatchers ===================

    # ----------------------------------------
    # $Child

    def __continueterminatordemo_state_Child(self, __e, compartment):
        if __e._message == "passMe1":
            return self.__handle_child_passMe1(__e, compartment)
        elif __e._message == "passMe2":
            return self.__handle_child_passMe2(__e, compartment)


    # ----------------------------------------
    # $Parent

    def __continueterminatordemo_state_Parent(self, __e, compartment):
        if __e._message == "passMe1":
            return self.__handle_parent_passMe1(__e, compartment)
        elif __e._message == "passMe2":
            return self.__handle_parent_passMe2(__e, compartment)


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
        if target_compartment.state == '__continueterminatordemo_state_Child':
            self.__continueterminatordemo_state_Child(__e, target_compartment)
        elif target_compartment.state == '__continueterminatordemo_state_Parent':
            self.__continueterminatordemo_state_Parent(__e, target_compartment)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment


if __name__ == '__main__':
    main()

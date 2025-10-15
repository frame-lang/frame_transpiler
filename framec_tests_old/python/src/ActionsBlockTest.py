# Emitted from framec_v0.78.22


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
    hws = HelloWorldSystem()
    hws.sayHello()
    hws.sayWorld()

class HelloWorldSystem:

    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__helloworldsystem_state_Hello', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]

        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)

    # ==================== Interface Block ==================

    def sayHello(self,):
        self.return_stack.append(None)
        __e = FrameEvent("sayHello", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def sayWorld(self,):
        self.return_stack.append(None)
        __e = FrameEvent("sayWorld", None)
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    # ===================== Machine Block ===================

    def __handle_hello_sayHello(self, __e, compartment):
        self.__HelloWorldSystem__actionWriteHello()
        next_compartment = FrameCompartment('__helloworldsystem_state_World', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_world_sayWorld(self, __e, compartment):
        self.__HelloWorldSystem__actionWriteWorld()
        next_compartment = FrameCompartment('__helloworldsystem_state_Done', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    # ===================== State Dispatchers ===================

    # ----------------------------------------
    # $Hello

    def __helloworldsystem_state_Hello(self, __e, compartment):
        if __e._message == "sayHello":
            return self.__handle_hello_sayHello(__e, compartment)


    # ----------------------------------------
    # $World

    def __helloworldsystem_state_World(self, __e, compartment):
        if __e._message == "sayWorld":
            return self.__handle_world_sayWorld(__e, compartment)


    # ----------------------------------------
    # $Done

    def __helloworldsystem_state_Done(self, __e, compartment):
        pass

    # ===================== Actions Block ===================

    def __HelloWorldSystem__actionWriteHello(self):
        self.__HelloWorldSystem__actionWrite("Hello", " ")


    def __HelloWorldSystem__actionWriteWorld(self):
        self.__HelloWorldSystem__actionWrite("World!", "")


    def __HelloWorldSystem__actionWrite(self, msg, separator):
        print(msg, end = separator)


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
        if target_compartment.state == '__helloworldsystem_state_Hello':
            self.__helloworldsystem_state_Hello(__e, target_compartment)
        elif target_compartment.state == '__helloworldsystem_state_World':
            self.__helloworldsystem_state_World(__e, target_compartment)
        elif target_compartment.state == '__helloworldsystem_state_Done':
            self.__helloworldsystem_state_Done(__e, target_compartment)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment


# Module-level singleton instance for HelloWorldSystem
_helloworldsystem_instance = None


def actionWriteHello():
    global _helloworldsystem_instance
    if _helloworldsystem_instance is None:
        _helloworldsystem_instance = HelloWorldSystem()
    return _helloworldsystem_instance._HelloWorldSystem__HelloWorldSystem__actionWriteHello()

def actionWriteWorld():
    global _helloworldsystem_instance
    if _helloworldsystem_instance is None:
        _helloworldsystem_instance = HelloWorldSystem()
    return _helloworldsystem_instance._HelloWorldSystem__HelloWorldSystem__actionWriteWorld()

def actionWrite(msg, separator):
    global _helloworldsystem_instance
    if _helloworldsystem_instance is None:
        _helloworldsystem_instance = HelloWorldSystem()
    return _helloworldsystem_instance._HelloWorldSystem__HelloWorldSystem__actionWrite(msg, separator)

if __name__ == '__main__':
    main()

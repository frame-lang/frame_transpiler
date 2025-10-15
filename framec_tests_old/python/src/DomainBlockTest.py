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
    hws = HelloWorldWithDomainSystem()
    hws.sayHello()
    hws.sayWorld()

class HelloWorldWithDomainSystem:

    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__helloworldwithdomainsystem_state_Hello', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]

        # Initialize domain variables
        self.hello_txt = "Hello"
        self.world_txt = "World!"

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
        self.__HelloWorldWithDomainSystem__actionWriteHello()
        next_compartment = FrameCompartment('__helloworldwithdomainsystem_state_World', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    def __handle_world_sayWorld(self, __e, compartment):
        self.__HelloWorldWithDomainSystem__actionWriteWorld()
        next_compartment = FrameCompartment('__helloworldwithdomainsystem_state_Done', None, None, None, None, {}, {})
        self.__transition(next_compartment)
        return

    # ===================== State Dispatchers ===================

    # ----------------------------------------
    # $Hello

    def __helloworldwithdomainsystem_state_Hello(self, __e, compartment):
        if __e._message == "sayHello":
            return self.__handle_hello_sayHello(__e, compartment)


    # ----------------------------------------
    # $World

    def __helloworldwithdomainsystem_state_World(self, __e, compartment):
        if __e._message == "sayWorld":
            return self.__handle_world_sayWorld(__e, compartment)


    # ----------------------------------------
    # $Done

    def __helloworldwithdomainsystem_state_Done(self, __e, compartment):
        pass

    # ===================== Actions Block ===================

    def __HelloWorldWithDomainSystem__actionWriteHello(self):
        self.__HelloWorldWithDomainSystem__actionWrite(self.hello_txt, " ")


    def __HelloWorldWithDomainSystem__actionWriteWorld(self):
        self.__HelloWorldWithDomainSystem__actionWrite(self.world_txt, "")


    def __HelloWorldWithDomainSystem__actionWrite(self, msg, separator):
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
        if target_compartment.state == '__helloworldwithdomainsystem_state_Hello':
            self.__helloworldwithdomainsystem_state_Hello(__e, target_compartment)
        elif target_compartment.state == '__helloworldwithdomainsystem_state_World':
            self.__helloworldwithdomainsystem_state_World(__e, target_compartment)
        elif target_compartment.state == '__helloworldwithdomainsystem_state_Done':
            self.__helloworldwithdomainsystem_state_Done(__e, target_compartment)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment


# Module-level singleton instance for HelloWorldWithDomainSystem
_helloworldwithdomainsystem_instance = None


def actionWriteHello():
    global _helloworldwithdomainsystem_instance
    if _helloworldwithdomainsystem_instance is None:
        _helloworldwithdomainsystem_instance = HelloWorldWithDomainSystem()
    return _helloworldwithdomainsystem_instance._HelloWorldWithDomainSystem__HelloWorldWithDomainSystem__actionWriteHello()

def actionWriteWorld():
    global _helloworldwithdomainsystem_instance
    if _helloworldwithdomainsystem_instance is None:
        _helloworldwithdomainsystem_instance = HelloWorldWithDomainSystem()
    return _helloworldwithdomainsystem_instance._HelloWorldWithDomainSystem__HelloWorldWithDomainSystem__actionWriteWorld()

def actionWrite(msg, separator):
    global _helloworldwithdomainsystem_instance
    if _helloworldwithdomainsystem_instance is None:
        _helloworldwithdomainsystem_instance = HelloWorldWithDomainSystem()
    return _helloworldwithdomainsystem_instance._HelloWorldWithDomainSystem__HelloWorldWithDomainSystem__actionWrite(msg, separator)

if __name__ == '__main__':
    main()

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
    calc = Calculator()
    sum = calc.add(5, 3)
    print("Sum: " + str(sum))
    product = calc.multiply(4, 7)
    print("Product: " + str(product))
    quotient = calc.divide(10, 2)
    print("Quotient: " + str(quotient))

class Calculator:

    def __init__(self):
        # Create and initialize start state compartment
        self.__compartment = FrameCompartment('__calculator_state_Ready', None, None, None, None, {}, {})
        self.__next_compartment = None
        self.return_stack = [None]

        # Send system start event
        frame_event = FrameEvent("$>", None)
        self.__kernel(frame_event)

    # ==================== Interface Block ==================

    def add(self, a, b):
        self.return_stack.append(None)
        __e = FrameEvent("add", {"a": a, "b": b})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def multiply(self, x, y):
        self.return_stack.append(None)
        __e = FrameEvent("multiply", {"x": x, "y": y})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    def divide(self, n, d):
        self.return_stack.append(None)
        __e = FrameEvent("divide", {"n": n, "d": d})
        self.__kernel(__e)
        return self.return_stack.pop(-1)

    # ===================== Machine Block ===================
    # Machine block
    # State: Ready

    def __handle_ready_add(self, __e, compartment):
        a = __e._parameters.get("a") if __e._parameters else None
        b = __e._parameters.get("b") if __e._parameters else None
        result = a + b
        print((((("Adding: " + str(a)) + " + ") + str(b)) + " = ") + str(result))
        self.return_stack[-1] = result
        return

    def __handle_ready_multiply(self, __e, compartment):
        x = __e._parameters.get("x") if __e._parameters else None
        y = __e._parameters.get("y") if __e._parameters else None
        result = x * y
        print((((("Multiplying: " + str(x)) + " * ") + str(y)) + " = ") + str(result))
        self.return_stack[-1] = result
        return

    def __handle_ready_divide(self, __e, compartment):
        n = __e._parameters.get("n") if __e._parameters else None
        d = __e._parameters.get("d") if __e._parameters else None
        if d == 0:
            print("Error: Division by zero")
            self.return_stack[-1] = 0.0
        else:
            result = n / d
            print((((("Dividing: " + str(n)) + " / ") + str(d)) + " = ") + str(result))
            self.return_stack[-1] = result
        return

    # ===================== State Dispatchers ===================

    # ----------------------------------------
    # $Ready

    def __calculator_state_Ready(self, __e, compartment):
        if __e._message == "add":
            return self.__handle_ready_add(__e, compartment)
        elif __e._message == "multiply":
            return self.__handle_ready_multiply(__e, compartment)
        elif __e._message == "divide":
            return self.__handle_ready_divide(__e, compartment)


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
        if target_compartment.state == '__calculator_state_Ready':
            self.__calculator_state_Ready(__e, target_compartment)

    def __transition(self, next_compartment):
        self.__next_compartment = next_compartment


if __name__ == '__main__':
    main()

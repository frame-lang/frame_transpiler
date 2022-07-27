from handler_calls.handler_calls import HandlerCalls


class HandlerCallsController(HandlerCalls):

    def __init__(self):
        super().__init__()

    def log_do(self, through: str, val: int):
        self.tape.append(f'{through}({val})')


class TestHandlerCalls:
    """
    Test directly invoking event handlers from within other event handlers.
    Since event handlers may transition, we conservatively treat such calls
    as terminating statements for the current handler.
    """

    def test_calls_terminate_handler(self):
        """Test that a handler call terminates the current handler."""

        sm = HandlerCallsController()
        sm.NonRec()
        sm.Foo(10)
        assert "Unreachable(0)" not in sm.tape

    def test_non_recursive(self):
        """Test non-recursive handler calls."""
        sm = HandlerCallsController()
        sm.NonRec()
        sm.Foo(10)
        assert sm.tape == ["Foo(10)", "Bar(20)", "Final(30)"]

    def test_self_recursive(self):
        """Test self-recursive handler calls. Also tests calls in the then-branch
        of a conditional."""
        sm = HandlerCallsController()
        sm.SelfRec()
        sm.Foo(10)
        assert sm.tape == [
            "Foo(10)", "Foo(20)", "Foo(40)", "Foo(80)", "Final(150)"
        ]

    def test_mutually_recursive(self):
        """Test self-recursive handler calls. Also tests calls in the else-branch
        of conditionals, and calls in integer matching constructs."""

        sm = HandlerCallsController()
        sm.MutRec()
        sm.Foo(2)
        assert sm.tape == [
            "Foo(2)", "Bar(4)", "Foo(4)", "Bar(8)", "Foo(16)", "Bar(32)",
            "Foo(96)", "Final(162)"
        ]


    def test_string_match_calls(self):
        """Test handler calls in string matching constructs."""
        
        sm = HandlerCallsController()
        sm.NonRec()
        sm.Call("Foo", 5)
        assert sm.tape == ["Foo(5)", "Bar(10)", "Final(15)"]
        sm.tape.clear()

        sm.NonRec()
        sm.Call("Bar", 20)
        assert sm.tape == ["Bar(20)", "Final(20)"]
        sm.tape.clear()

        sm.NonRec()
        sm.Call("Qux", 37)
        assert sm.tape == ["Foo(1000)", "Bar(2000)", "Final(3000)"]


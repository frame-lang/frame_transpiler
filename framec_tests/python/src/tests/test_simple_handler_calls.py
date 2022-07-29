from simple_handler_calls.simple_handler_calls import SimpleHandlerCalls

def return_state_name(state: str) -> str:

    return f'__simplehandlercalls_state_{state}'

class TestSimpleHandlerCalls:

    def test_simple_calls(self):
        """Test a basic handler call."""
        sm = SimpleHandlerCalls()
        sm.C()
        assert sm.state_info() == return_state_name("A")

    def test_calls_terminate_handler(self):
        """Test that a handler call terminates the current handler."""
        sm = SimpleHandlerCalls()
        sm.D()
        assert sm.state_info() == return_state_name("B")

        sm = SimpleHandlerCalls()
        sm.E()
        assert sm.state_info() == return_state_name("B")

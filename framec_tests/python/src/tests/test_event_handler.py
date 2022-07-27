from event_handler.event_handler import EventHandler

class EventHandlerController(EventHandler):

    def __init__(self):
        super().__init__()
    
    def log_do(self, msg: str, val: int):
        return self.tape.append(f'{msg}={val}')


class TestEventHandler:

    def test_single_parameter(self):

        sm = EventHandlerController()
        sm.LogIt(2)
        assert sm.tape == ["x=2"]

    def test_compute_two_parameters(self):

        sm = EventHandlerController()
        sm.LogAdd(-3, 10)
        assert sm.tape == ["a=-3", "b=10", "a+b=7"]
    
    def test_return_local_variable(self):
        sm = EventHandlerController()
        ret = sm.LogReturn(13, 21)
        assert sm.tape == ["a=13", "b=21", "r=34"]
        assert ret == 34
    
    def test_pass_result(self):
        
        sm = EventHandlerController()
        sm.PassAdd(5, -12)
        assert sm.tape == ["p=-7"]

    def pass_and_return_result(self):

        sm = EventHandlerController()
        ret = sm.PassReturn(101, -59)
        assert sm.tape == ["r=42", "p=42"]
        assert ret == 42
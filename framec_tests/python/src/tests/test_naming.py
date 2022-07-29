from naming.naming import Naming

class NamingController(Naming):

    def snake_action_do(self, snake_param: int):
        self.snake_log.append(snake_param)
    
    def CamelAction_do(self, CamelParam: int):
        self.CamelLog.append(CamelParam)

    def action123_do(self, param123: int):
        self.log123.append(param123)
    
    def logFinal_do(self , r:int):
        self.finalLog.append(r)

def return_state_name(state: str) -> str:

    return f'__naming_state_{state}'


class TestNaming:

    def test_follow_naming_works(self):
        """Test that the generated state machine works and that events are
        named as expected."""

        sm = NamingController()
        sm.snake_event(1)
        assert sm.state_info() == return_state_name("snake_state")
        sm.snake_event(2)
        assert sm.state_info() == return_state_name("Init")
        sm.snake_event(1)
        assert sm.state_info() == return_state_name("snake_state")
        sm.CamelEvent(3)
        assert sm.state_info() == return_state_name("Init")
        sm.snake_event(1)
        assert sm.state_info() == return_state_name("snake_state")
        sm.event123(4)
        assert sm.state_info() == return_state_name("Init")
        assert sm.finalLog == [1103, 1104, 1105]
        sm.finalLog.clear()

        sm.CamelEvent(11)
        assert sm.state_info() == return_state_name("CamelState")
        sm.snake_event(2)
        assert sm.state_info() == return_state_name("Init")
        sm.CamelEvent(11)
        assert sm.state_info() == return_state_name("CamelState")
        sm.CamelEvent(3)
        assert sm.state_info() == return_state_name("Init")
        sm.CamelEvent(11)
        assert sm.state_info() == return_state_name("CamelState")
        sm.event123(4)
        assert sm.state_info() == return_state_name("Init")
        assert sm.finalLog == [1213, 1214, 1215]
        sm.finalLog.clear()

        sm.event123(21)
        assert sm.state_info() == return_state_name("state123")
        sm.snake_event(2)
        assert sm.state_info() == return_state_name("Init")
        sm.event123(21)
        assert sm.state_info() == return_state_name("state123")
        sm.CamelEvent(3)
        assert sm.state_info() == return_state_name("Init")
        sm.event123(21)
        assert sm.state_info() == return_state_name("state123")
        sm.event123(4)
        assert sm.state_info() == return_state_name("Init")
        assert sm.finalLog == [1323, 1324, 1325]

        assert sm.snake_log == [1103, 1213, 1323]
        assert sm.CamelLog == [1104, 1214, 1324]
        assert sm.log123 == [1105, 1215, 1325]

    def test_interface_calls(self):
        """Test that dynamic interface calls are renamed correctly."""
        
        sm = NamingController()
        sm.call("snake_event", 1)
        sm.call("CamelEvent", 2)
        sm.call("event123", 3)
        sm.call("snake_event", 4)
        sm.call("CamelEvent", 5)
        sm.call("event123", 6)
        assert sm.finalLog == [1103, 1307, 1211]
        assert sm.snake_log == [1307]
        assert sm.CamelLog == [1103]
        assert sm.log123 == [1211]
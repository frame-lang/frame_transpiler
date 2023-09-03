from forward_events.forward_events import ForwardEvents


class TestForwardEvents:

    def test_run(self):
        sm = ForwardEvents()
        sm.GotoS1()
        sm.ReturnFromS1()
        sm.GotoS2()
        sm.ReturnFromS2()
        assert sm.tape == ['Enter $S0', 'Recieved |GotoS1|', 'Exit $S0', 'Enter $S1', 'Exit $S1', 'Enter $S0', '|ReturnFromS1| Forwarded', 'Recieved |GotoS2|', 'Exit $S0', 'Enter $S2', 'Exit $S2', 'Enter $S0', '|ReturnFromS2| Forwarded']

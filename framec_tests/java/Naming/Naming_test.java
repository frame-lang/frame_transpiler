package framec_tests.java.Naming;

import java.util.*;
import static org.junit.Assert.assertEquals;
import org.junit.Test;

class NamingController extends Naming {

    NamingController() {
      super();
    }

protected void snake_action_do(int snake_param) {
    this.snake_log.add(snake_param);
}

protected void CamelAction_do(int CamelParam) {
    this.CamelLog.add(CamelParam);
}

protected void action123_do(int param123) {
    this.log123.add(param123);
}

protected void logFinal_do(int r) {
    this.finalLog.add(r);
}
}

public class Naming_test {
    /* Test that the generated state machine works and that events are
    named as expected. */
    @Test
    public void TestFollowingNamingWorks(){
        NamingController sm = new NamingController();
        sm.snake_event(1);
        assertEquals(sm.state_info(), "1");
        sm.snake_event(2);
        assertEquals(sm.state_info(), "0");
        sm.snake_event(1);
        assertEquals(sm.state_info(), "1");
        sm.CamelEvent(3);
        assertEquals(sm.state_info(), "0");
        sm.snake_event(1);
        assertEquals(sm.state_info(), "1");
        sm.event123(4);
        assertEquals(sm.state_info(), "0");
        ArrayList<Integer> expected = new ArrayList<Integer>();
        expected.add(1103);
        expected.add(1104);
        expected.add(1105);
        assertEquals(sm.finalLog, expected);
        sm.finalLog = new ArrayList<>();

        sm.CamelEvent(11);
        assertEquals(sm.state_info(), "2");
        sm.snake_event(2);
        assertEquals(sm.state_info(), "0");
        sm.CamelEvent(11);
        assertEquals(sm.state_info(), "2");
        sm.CamelEvent(3);
        assertEquals(sm.state_info(), "0");
        sm.CamelEvent(11);
        assertEquals(sm.state_info(), "2");
        sm.event123(4);
        assertEquals(sm.state_info(), "0");
        ArrayList<Integer> expected1 = new ArrayList<Integer>();
        expected1.add(1213);
        expected1.add(1214);
        expected1.add(1215);
        assertEquals(sm.finalLog, expected1);
        sm.finalLog = new ArrayList<>();

        sm.event123(21);
        assertEquals(sm.state_info(), "3");
        sm.snake_event(2);
        assertEquals(sm.state_info(), "0");
        sm.event123(21);
        assertEquals(sm.state_info(), "3");
        sm.CamelEvent(3);
        assertEquals(sm.state_info(), "0");
        sm.event123(21);
        assertEquals(sm.state_info(), "3");
        sm.event123(4);
        assertEquals(sm.state_info(), "0");
        ArrayList<Integer> expected2 = new ArrayList<Integer>();
        expected2.add(1323);
        expected2.add(1324);
        expected2.add(1325);
        assertEquals(sm.finalLog, expected2);
        ArrayList<Integer> expected3 = new ArrayList<Integer>();
        expected3.add(1103);
        expected3.add(1213);
        expected3.add(1323);
        assertEquals(sm.snake_log, expected3);
        ArrayList<Integer> expected4 = new ArrayList<Integer>();
        expected4.add(1104);
        expected4.add(1214);
        expected4.add(1324);
        assertEquals(sm.CamelLog, expected4);
        ArrayList<Integer> expected5 = new ArrayList<Integer>();
        expected5.add(1105);
        expected5.add(1215);
        expected5.add(1325);
        assertEquals(sm.log123, expected5);

    }
    /* Test that dynamic interface calls are renamed correctly. */
    @Test
    public void TestInterfaceCalls(){
        NamingController sm = new NamingController();

        sm.call("snake_event", 1);
        sm.call("CamelEvent", 2);
        sm.call("event123", 3);
        sm.call("snake_event", 4);
        sm.call("CamelEvent", 5);
        sm.call("event123", 6);

        ArrayList<Integer> expected = new ArrayList<Integer>();
        expected.add(1103);
        expected.add(1307);
        expected.add(1211);
        assertEquals(sm.finalLog, expected);
        ArrayList<Integer> expected1 = new ArrayList<Integer>();
        expected1.add(1307);
        assertEquals(sm.snake_log, expected1);
        ArrayList<Integer> expected2 = new ArrayList<Integer>();
        expected2.add(1103);
        assertEquals(sm.CamelLog, expected2);
        ArrayList<Integer> expected3 = new ArrayList<Integer>();
        expected3.add(1211);
        assertEquals(sm.log123, expected3);
    }
}


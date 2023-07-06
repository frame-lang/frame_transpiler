package framec_tests.java.StateContext;

import static org.junit.Assert.assertEquals;

import java.util.ArrayList;
import java.util.Arrays;

import org.junit.Test;

class StateContextSmController extends StateContextSm {

    StateContextSmController() {
      super();
    }

protected void log_do(String name, int val) {
    this.tape.add(name + "=" +String.valueOf(val));
}
}

public class StateContextTest {
    @Test
    public void TestInitialState(){
        StateContextSmController sm = new StateContextSmController();
        int r = sm.Inc();
        assertEquals(4, r);
        sm.LogState();
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("w=3");
        expected.add("w=4");
        expected.add("w=4");
        assertEquals(expected, sm.tape);

    }

    @Test
    public void TestTransition(){
        StateContextSmController sm = new StateContextSmController();
        sm.Inc();
        sm.Inc();
        sm.tape = new ArrayList<>();

        sm.Start();
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("a=3");
        expected.add("b=5");
        expected.add("x=15");
        assertEquals(sm.tape, expected);
        sm.tape = new ArrayList<>();

        sm.Inc();
        int r = sm.Inc();
        assertEquals(17, r);
        ArrayList<String> expected1 = new ArrayList<>();
        expected1.add("x=16");
        expected1.add("x=17");
        assertEquals(sm.tape, expected1);
        sm.tape = new ArrayList<>();

        sm.Next(3);
        ArrayList<String> expected2 = new ArrayList<String>();
        expected2.add("c=10");
        expected2.add("x=27");
        expected2.add("a=30");
        expected2.add("y=17");
        expected2.add("z=47");
        assertEquals(expected2, sm.tape);
        sm.tape = new ArrayList<>();

        sm.Inc();
        sm.Inc();
        r = sm.Inc();
        assertEquals(50, r);
        assertEquals(sm.tape, Arrays.asList("z=48", "z=49", "z=50"));

    }
    @Test
    public void TestChangeState(){
        StateContextSmController sm = new StateContextSmController();
        sm.Inc();
        sm.Inc();
        sm.Start();
        sm.tape = new ArrayList<>();

        sm.Inc();
        assertEquals(sm.tape, Arrays.asList("x=16"));
        sm.tape = new ArrayList<>();

        sm.Change(10);
        sm.LogState();
        assertEquals(sm.tape, Arrays.asList("y=26", "z=0"));
        sm.tape = new ArrayList<>();
        
        sm.Inc();
        sm.Change(100);
        sm.LogState();
        assertEquals(sm.state_info(), "0");
        assertEquals(sm.tape, Arrays.asList("z=1", "tmp=127", "w=0"));
    }
}


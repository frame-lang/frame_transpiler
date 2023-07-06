package framec_tests.java.StateContextStack;

import static org.junit.Assert.assertEquals;

import java.util.ArrayList;
import java.util.Arrays;

import org.junit.Test;

class StateContextStackController extends StateContextStack {

    StateContextStackController() {
      super();
    }

protected void log_do(String msg) {
    this.tape.add(msg);
}
}

public class StateContextStackTest {
   // Test that a pop restores a pushed state.
    @Test
    public void TestPushPop(){
        StateContextStackController sm = new StateContextStackController();
        assertEquals( "0", sm.state_info());
        sm.push();
        sm.to_b();
        assertEquals("1", sm.state_info());
        sm.pop();
        assertEquals("0", sm.state_info());
    }

    /* Test that multiple states can be pushed and subsequently restored by pops, LIFO style. */
    @Test
    public void TestMultiplePushPops(){
        StateContextStackController sm = new StateContextStackController();
        assertEquals("0", sm.state_info());
        sm.push();
        sm.to_c();
        sm.push();
        sm.to_a();
        sm.push();
        sm.push();
        sm.to_c(); // no push
        sm.to_b();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, B, A, A, C, A
        sm.to_a();
        assertEquals("0", sm.state_info());
        sm.pop();
        assertEquals("2", sm.state_info());
        sm.to_a();
        assertEquals("0", sm.state_info());
        sm.pop();
        assertEquals("1", sm.state_info());
        sm.pop();
        assertEquals("0", sm.state_info());
        sm.pop();
        assertEquals("0", sm.state_info());
        sm.pop();
        assertEquals("2", sm.state_info());
        sm.to_b();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, B, A
        sm.to_a();
        sm.to_b();
        assertEquals("1", sm.state_info());
        sm.pop();
        assertEquals("2", sm.state_info());
        sm.pop();
        assertEquals("1", sm.state_info());
        sm.pop();
        assertEquals("0", sm.state_info());

    }

    /*  Test that pop transitions trigger enter/exit events. */
    @Test
    public void TestPopTransitionEvents(){
        StateContextStackController sm = new StateContextStackController();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, A, B
        sm.to_a();
        sm.tape = new ArrayList<>();
        assertEquals("0", sm.state_info());
        sm.pop();
        assertEquals("2", sm.state_info());
        assertEquals(sm.tape, Arrays.asList("A:<", "C:>"));
        sm.tape = new ArrayList<>();
        sm.pop();
        sm.pop();
        assertEquals("1", sm.state_info());
        assertEquals(sm.tape, Arrays.asList("C:<", "A:>", "A:<", "B:>"));
  
    }
    /* Test that pop change-states do not trigger enter/exit events. */
    @Test
    public void TestPopChangesStateNoEvents(){
        StateContextStackController sm = new StateContextStackController();
        sm.to_b();
        sm.push();
        sm.to_a();
        sm.push();
        sm.to_c();
        sm.push(); // stack top-to-bottom: C, A, B
        sm.to_a();
        sm.tape = new ArrayList<>();
        assertEquals("0", sm.state_info());
        sm.pop_change();
        assertEquals("2", sm.state_info());
        assertEquals(0, sm.tape.size());
        sm.pop();
        sm.pop_change();
        assertEquals("1", sm.state_info());
        assertEquals(Arrays.asList("C:<", "A:>"), sm.tape);
    }
    /* Test that state variables are restored after pop. */
    @Test
    public void TestPopRestoresStateVariables(){
        StateContextStackController sm = new StateContextStackController();
        sm.inc();
        sm.inc();
        sm.push();
        assertEquals("0", sm.state_info());
        assertEquals(2, sm.value());
        sm.to_b();
        sm.inc();
        sm.push();
        assertEquals("1", sm.state_info());
        assertEquals(5, sm.value());
        sm.to_c();
        sm.inc();
        sm.inc();
        sm.inc();
        sm.push();
        assertEquals("2", sm.state_info());
        assertEquals(sm.value(), 30);
        sm.to_a();
        sm.inc();
        assertEquals("0", sm.state_info());
        assertEquals(1, sm.value());
        sm.pop();
        assertEquals("2", sm.state_info());
        assertEquals(30, sm.value());
        sm.pop();
        assertEquals("1", sm.state_info());
        assertEquals(5, sm.value());
        sm.to_a();
        sm.inc();
        sm.inc();
        sm.inc();
        sm.push();
        assertEquals("0", sm.state_info());
        assertEquals(3, sm.value());
        sm.to_c();
        sm.inc();
        assertEquals("2", sm.state_info());
        assertEquals(10, sm.value());
        sm.pop();
        assertEquals("0", sm.state_info());
        assertEquals(3, sm.value());
        sm.pop();
        assertEquals("0", sm.state_info());
        assertEquals(2, sm.value());
    }
    @Test
    public void TestPushStoresStateVariableSnapshot(){
        StateContextStackController sm = new StateContextStackController();
        sm.inc();
        sm.inc();
        sm.push();
        assertEquals("0", sm.state_info());
        assertEquals(2, sm.value());
        sm.inc();
        sm.inc();
        assertEquals(4, sm.value());

        sm.to_b();
        sm.inc();
        sm.push();
        assertEquals("1", sm.state_info());
        assertEquals(5, sm.value());
        sm.inc();
        sm.inc();
        assertEquals(15, sm.value()); // these changes should be forgotten

        sm.to_c();
        sm.inc();
        sm.inc();
        sm.inc();
        sm.push();
        assertEquals("2", sm.state_info());
        assertEquals(30, sm.value());
        sm.inc();
        assertEquals(40, sm.value()); // forgotten

        sm.to_a();
        sm.inc();
        assertEquals("0", sm.state_info());
        assertEquals(1, sm.value());

        sm.pop();
        assertEquals("2", sm.state_info());
        assertEquals(30, sm.value());

        sm.pop();
        assertEquals("1", sm.state_info());
        assertEquals(5, sm.value());

        sm.to_a();
        sm.inc();
        sm.inc();
        sm.inc();
        sm.push();
        assertEquals("0", sm.state_info());
        assertEquals(3, sm.value());
        sm.inc();
        assertEquals(4, sm.value()); //forgotten

        sm.to_c();
        sm.inc();
        assertEquals("2", sm.state_info());
        assertEquals(10, sm.value());

        sm.pop();
        assertEquals("0", sm.state_info());
        assertEquals(3, sm.value());

        sm.pop();
        assertEquals("0", sm.state_info());
        assertEquals(2, sm.value());
    }
    
}

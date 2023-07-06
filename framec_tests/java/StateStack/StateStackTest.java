package framec_tests.java.StateStack;

import static org.junit.Assert.assertEquals;

import java.util.ArrayList;
import java.util.Arrays;

import org.junit.Test;

class StateStackController extends StateStack {

	StateStackController() {
	  super();
	}

    protected void log_do(String msg) {
        this.tape.add(msg);
    }
}

public class StateStackTest {
    /// Test that a pop restores a pushed state.
    @Test
    public void TestPushPop(){
        StateStackController sm = new StateStackController();
        assertEquals("0", sm.state_info());
        sm.push();
        sm.to_b();
        assertEquals("1", sm.state_info());
        sm.pop();
        assertEquals("0", sm.state_info());

    }

    /// Test that multiple states can be pushed and subsequently restored by pops, LIFO style.
    @Test
    public void TestMultiplePushPops(){
        StateStackController sm = new StateStackController();
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

    /// Test that pop transitions trigger enter/exit events.
    @Test
    public void TestPopTransitionEvents(){
        StateStackController sm = new StateStackController();
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
        assertEquals(Arrays.asList("A:<", "C:>"), sm.tape);
        sm.tape = new ArrayList<>();
        sm.pop();
        sm.pop();
        assertEquals("1", sm.state_info());
        assertEquals(Arrays.asList("C:<", "A:>", "A:<", "B:>"), sm.tape);
    }

      // Test that pop change-states do not trigger enter/exit events.
    @Test
    public void TestPopChangeStateNoEvents(){
        StateStackController sm = new StateStackController();
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
        assertEquals(Arrays.asList("C:<", "A:>"), sm.tape);
    }

}

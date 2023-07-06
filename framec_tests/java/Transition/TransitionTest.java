package framec_tests.java.Transition;

import static org.junit.Assert.assertEquals;

import java.util.ArrayList;
import java.util.Arrays;

import org.junit.Test;

class TransitionSmController extends TransitionSm {

	TransitionSmController() {
	  super();
	}

    protected void enter_do(String state) {
        this.enters.add(state);
    }

    protected void exit_do(String state) {
        this.exits.add(state);
    }

    protected void clear_all(){
        this.enters = new ArrayList<>();
        this.exits = new ArrayList<>();
    }
}

public class TransitionTest {
    /// Test that transition works and triggers enter and exit events.
    @Test
    public void TestTransitionEvents(){
        TransitionSmController sm = new TransitionSmController();
        sm.clear_all();
        sm.transit();
        assertEquals("1", sm.state_info());
        assertEquals(Arrays.asList("S0"), sm.exits);
        assertEquals(Arrays.asList("S1"), sm.enters);
    }

    /// Test that change-state works and does not trigger events.
    @Test
    public void TestChangeStateNoEvents(){
        TransitionSmController sm = new TransitionSmController();
        sm.clear_all();
        sm.change();
        assertEquals("1", sm.state_info());
        sm.change();
        assertEquals("2", sm.state_info());
        sm.change();
        assertEquals("3", sm.state_info());
        sm.change();
        assertEquals("4", sm.state_info());
        assert(sm.exits.size()==0);
        assert(sm.enters.size()==0);
    }

    /// Test transition that triggers another transition in an enter event handler.
    @Test
    public void TestCascadingTransition(){
        TransitionSmController sm = new TransitionSmController();
        sm.change();
        sm.clear_all();
        assertEquals("1", sm.state_info());
        sm.transit();
        assertEquals("3", sm.state_info());
        assertEquals(Arrays.asList("S1", "S2"), sm.exits);
        assertEquals(Arrays.asList("S2", "S3"), sm.enters);

    }
    /// Test transition that triggers a change-state from an enter event handler.
    @Test
    public void TestCascadingChangeSheet(){
        TransitionSmController sm = new TransitionSmController();
        sm.change();
        sm.change();
        sm.change();
        sm.clear_all();
        assertEquals("3", sm.state_info());
        sm.transit();
        assertEquals("0", sm.state_info());
        assertEquals(Arrays.asList("S3"), sm.exits);
        assertEquals(Arrays.asList("S4"), sm.enters);
    }
}

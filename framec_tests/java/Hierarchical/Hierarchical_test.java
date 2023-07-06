package framec_tests.java.Hierarchical;

import java.util.*;
import static org.junit.Assert.assertEquals;

import org.junit.Test;

class HierarchicalController extends Hierarchical {
    HierarchicalController() {
      super();
    }
    protected void enter_do(String msg) {
      this.enters.add(msg);
    }
    protected void exit_do(String msg){
        this.exits.add(msg);
    }
    protected void log_do(String msg){
        this.tape.add(msg);
    }
  }

public class Hierarchical_test {
    /// Test that a continue (`:>`) in a child enter handler calls the parent enter handler.
    @Test
    public void TestEnterContinue(){
        HierarchicalController sm = new HierarchicalController();
        sm.enters = new ArrayList<>();
        sm.A();
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S0");
        expected.add("S");
        assertEquals(sm.enters, expected);
        sm.enters = new ArrayList<>();
        sm.C();
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S2");
        expected1.add("S0");
        expected1.add("S");
        assertEquals(sm.enters, expected1);
    }

    /// Test that a continue (`:>`) in a child exit handler calls the parent exit handler.
    @Test
    public void TestExitContinue(){
        HierarchicalController sm = new HierarchicalController();
        sm.A();
        sm.exits = new ArrayList<>();
        sm.C();
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S0");
        expected.add("S");
        assertEquals(sm.exits, expected);
        sm.exits = new ArrayList<>();
        sm.A();
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S2");
        expected1.add("S0");
        expected1.add("S");
        assertEquals(sm.exits, expected1);
    }
    /// Test that a return (`^`) in a child enter handler *does not* call the parent enter handler.
    @Test
    public void TestEnterReturn(){
        HierarchicalController sm = new HierarchicalController();
        sm.enters = new ArrayList<>();
        sm.B();
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S1");
        assertEquals(sm.enters, expected);
        sm = new HierarchicalController();
        sm.A();
        sm.A();
        assertEquals(sm.state_info(), "6");
        sm.enters = new ArrayList<>();
        sm.C();
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S3");
        expected1.add("S1");
        assertEquals(sm.enters, expected1);
    }

    /// Test that a return (`^`) in a child exit handler *does not* call the parent exit handler.
    @Test
    public void TestExitReturn(){
        HierarchicalController sm = new HierarchicalController();
        sm.B();
        assertEquals(sm.state_info(), "3");
        sm.exits = new ArrayList<>();
        sm.A();
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S1");
        assertEquals(sm.exits, expected);
        sm = new HierarchicalController();
        sm.A();
        sm.A();
        sm.C();
        assertEquals(sm.state_info(), "5");
        sm.exits = new ArrayList<>();
        sm.B();
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S3");
        expected1.add("S1");
        assertEquals(sm.exits, expected1);
    }

    /// Test that location in a hierarchical state is represented correctly. In this test, all
    /// state transitions are performed by the immediately matching handler.
    @Test
    public void TestCurrentStateSimple(){
        HierarchicalController sm = new HierarchicalController();
        assertEquals(sm.state_info(), "1");
        sm.A();
        assertEquals(sm.state_info(), "2");
        sm.A();
        assertEquals(sm.state_info(), "6");
        sm.C();
        assertEquals(sm.state_info(), "5");
        sm.B();
        assertEquals(sm.state_info(), "4");
    }

    /* Test that location in a hierarchical state is represented correctly. In this test, several
    state transitions propagate message handling to parents, either by implicit fall-through or
    explicit continues. */
    @Test
    public void TestCurrentStateWithPropagation(){
        HierarchicalController sm = new HierarchicalController();
        assertEquals(sm.state_info(), "1");
        sm.A();
        assertEquals(sm.state_info(), "2");
        sm.B();
        assertEquals(sm.state_info(), "3");
        sm.B();
        assertEquals(sm.state_info(), "3");
        sm.C();
        assertEquals(sm.state_info(), "3");
        sm.A();
        assertEquals(sm.state_info(), "2");
        sm.C();
        assertEquals(sm.state_info(), "4");
        sm.B();
        assertEquals(sm.state_info(), "3");
    }

    /* Test that a handler in a child overrides the parent handler if the child handler ends with
    a return. */
    @Test
    public void TestOverrideParentHandler(){
        HierarchicalController sm = new HierarchicalController();
        sm.A();
        sm.tape = new ArrayList<>();
        sm.A();
        assertEquals(sm.state_info(), "6");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S0.A");
        assertEquals(sm.tape, expected);
        sm.C();
        sm.tape = new ArrayList<>();
        sm.B();
        assertEquals(sm.state_info(), "4");
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S3.B");
        assertEquals(sm.tape, expected1);
    }
    // Test that a handler in a child propagates control to the parent handler if the child
    // handler ends with a continue.
    @Test
    public void TestBeforeParentHandle(){
        HierarchicalController sm = new HierarchicalController();
        sm.A();
        sm.tape = new ArrayList<>();
        sm.B();
        assertEquals(sm.state_info(), "3");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S0.B");
        expected.add("S.B");
        assertEquals(sm.tape, expected);
        sm.tape = new ArrayList<>();
        sm.exits = new ArrayList<>();
        sm.enters = new ArrayList<>();

        sm.B();
        assertEquals(sm.state_info(), "3");
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S1.B");
        expected1.add("S.B");
        assertEquals(sm.tape, expected1);
        ArrayList<String> expected2 = new ArrayList<String>();
        expected2.add("S1");
        assertEquals(sm.exits, expected2);
        ArrayList<String> expected3 = new ArrayList<String>();
        expected3.add("S1");
        assertEquals(sm.enters, expected3);

        sm = new HierarchicalController();
        sm.A();
        sm.C();
        assertEquals(sm.state_info(), "4");
        sm.tape = new ArrayList<>();
        sm.exits = new ArrayList<>();
        sm.enters = new ArrayList<>();
        sm.B();
        assertEquals(sm.state_info(), "3");
        ArrayList<String> expected4 = new ArrayList<String>();
        expected4.add("S2.B");
        expected4.add("S0.B");
        expected4.add("S.B");
        assertEquals(sm.tape, expected4);
        ArrayList<String> expected5 = new ArrayList<String>();
        expected5.add("S2");
        expected5.add("S0");
        expected5.add("S");
        assertEquals(sm.exits, expected5);
        ArrayList<String> expected6 = new ArrayList<String>();
        expected6.add("S1");
        assertEquals(sm.enters, expected6);
    }
    /* Test that missing event handlers in children automatically propagate to parents.
    */
    @Test
    public void TestDeferToParentHandler(){
        HierarchicalController sm = new HierarchicalController();
        sm.B();
        assertEquals(sm.state_info(), "3");
        sm.tape = new ArrayList<>();
        sm.A();
        assertEquals(sm.state_info(), "2");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S.A");
        assertEquals(sm.tape, expected);
        sm.A();
        sm.C();
        assertEquals(sm.state_info(), "5");
        sm.tape = new ArrayList<>();
        sm.A();
        assertEquals(sm.state_info(), "2");
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S.A");
        assertEquals(sm.tape, expected1);
    }
    /* Test that propagating control to a parent handler that doesn't handle the current message
    is a no-op. */
    @Test
    public void TestBeforeMissingHandler(){
        HierarchicalController sm = new HierarchicalController();
        sm.B();
        assertEquals(sm.state_info(), "3");
        sm.tape = new ArrayList<>();
        sm.enters = new ArrayList<>();
        sm.exits = new ArrayList<>();
        sm.C();
        assertEquals(sm.state_info(), "3");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S1.C");
        assertEquals(sm.tape, expected);
        assert(sm.exits.size()==0);
        assert(sm.enters.size()==0);
        
    } 
    /// Test that a continue after a transition statement is ignored.
    @Test
    public void TestContinueAfterTransitionIgnored(){
        HierarchicalController sm = new HierarchicalController();
        sm.A();
        sm.C();
        assertEquals(sm.state_info(), "4");
        sm.tape = new ArrayList<>();
        sm.enters = new ArrayList<>();
        sm.C();
        assertEquals(sm.state_info(), "6");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("T");
        assertEquals(sm.enters, expected);
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S2.C");
        assertEquals(sm.tape, expected1); 
    }
}

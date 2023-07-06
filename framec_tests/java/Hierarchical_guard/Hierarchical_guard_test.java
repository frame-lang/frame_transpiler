package framec_tests.java.Hierarchical_guard;

import java.util.*;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

class HierarchicalGuardController extends HierarchicalGuard {

    HierarchicalGuardController() {
      super();
    }

    protected void log_do(String msg) {
        this.tape.add(msg);
    }
}
public class Hierarchical_guard_test {

  /* Test that basic conditional transitions work properly. In particular,
that control propagates to a parent handler if a child handler does
not transition and ends in a continue (`:>`). */
    @Test
    public void TestPropogateToParent(){
        HierarchicalGuardController sm = new HierarchicalGuardController();
        sm.A(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "2");
        sm.A(20);
        assertEquals(sm.state_info(), "4");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S0.A");
        assertEquals(sm.tape, expected);

        sm = new HierarchicalGuardController();
        sm.A(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "2");
        sm.A(-5);
        assertEquals(sm.state_info(), "2");
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S0.A");
        expected1.add("S.A");
        assertEquals(sm.tape, expected1);

        sm = new HierarchicalGuardController();
        sm.A(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "2");
        sm.B(-5);
        assertEquals(sm.state_info(), "3");
        ArrayList<String> expected2 = new ArrayList<String>();
        expected2.add("S0.B");
        assertEquals(sm.tape, expected2);

        sm = new HierarchicalGuardController();
        sm.A(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "2");
        sm.B(5);
        assertEquals(sm.state_info(), "4");
    }
    
    /*
	Test that control propagates across across multiple levels if a
    transition is not initiated.
    */
    @Test
    public void TestPropogateMultipleLevels(){
        HierarchicalGuardController sm = new HierarchicalGuardController();
        sm.B(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "4");
        sm.A(7);
        assertEquals(sm.state_info(), "5");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S2.A");
        expected.add("S1.A");
        assertEquals(sm.tape, expected);

        sm = new HierarchicalGuardController();
        sm.B(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "4");
        sm.A(-5);
        assertEquals(sm.state_info(), "2");
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S2.A");
        expected1.add("S1.A");
        expected1.add("S0.A");
        expected1.add("S.A");
        assertEquals(sm.tape, expected1);
    }
    
/*
	Test that propagation of control skips levels that do not contain a

given handler.
*/
    @Test
    public void TestPropogateSkipsLevels(){
        HierarchicalGuardController sm = new HierarchicalGuardController();
        sm.B(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "4");
        sm.B(-5);
        assertEquals(sm.state_info(), "3");
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("S2.B");
        expected.add("S0.B");
        assertEquals(sm.tape, expected);

        sm = new HierarchicalGuardController();
        sm.B(0);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "4");
        sm.B(5);
        assertEquals(sm.state_info(), "4");
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("S2.B");
        expected1.add("S0.B");
        expected1.add("S.B");
        assertEquals(sm.tape, expected1);
    }
    /* Test that conditional returns prevent propagation to parents. */

    public void TestConditionalReturns(){
        HierarchicalGuardController sm = new HierarchicalGuardController();
        sm.B(20);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "5");
        sm.A(5);
        assertEquals(sm.state_info(), "5");
        ArrayList<String> expected = new ArrayList<>();
        expected.add("S3.A");
        expected.add("stop");
        assertEquals(sm.tape, expected);

        sm = new HierarchicalGuardController();
        sm.B(20);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "5");
        sm.A(-5);
        assertEquals(sm.state_info(), "2");
        ArrayList<String> expected1 = new ArrayList<>();
        expected1.add("S3.A");
        expected1.add("continue");
        expected1.add("S.A");
        assertEquals(sm.tape, expected1);

        sm = new HierarchicalGuardController();
        sm.B(20);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "5");
        sm.B(-5);
        assertEquals(sm.state_info(), "5");
        ArrayList<String> expected2 = new ArrayList<>();
        expected2.add("S3.B");
        expected2.add("stop");
        assertEquals(sm.tape, expected2);

        sm = new HierarchicalGuardController();
        sm.B(20);
        sm.tape = new ArrayList<>();
        assertEquals(sm.state_info(), "5");
        sm.A(-5);
        assertEquals(sm.state_info(), "4");
        ArrayList<String> expected3 = new ArrayList<>();
        expected3.add("S3.B");
        expected3.add("continue");
        expected3.add("S.B");
        assertEquals(sm.tape, expected3);
    }
}

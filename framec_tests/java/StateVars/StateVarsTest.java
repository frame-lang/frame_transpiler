package framec_tests.java.StateVars;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

class StateVarsController extends StateVars {

    StateVarsController() {
      super();
    }
}
public class StateVarsTest {
    @Test
    public void TestSingleVariable(){
        StateVarsController sm = new StateVarsController();
        assertEquals("1", sm.state_info());
        sm.X(); // increment x
        sm.X(); // increment x
        assertEquals(2, (int)sm._compartment_.stateVars.get("x"));
    }

    @Test
    public void TestMultipleVariables(){
        StateVarsController sm = new StateVarsController();
        sm.Y();
        assertEquals("2", sm.state_info());
        assertEquals((int)sm._compartment_.stateVars.get("y"), 10);
        assertEquals((int)sm._compartment_.stateVars.get("z"), 100);
        sm.Y();
        sm.Y();
        sm.Z();
        sm.Y();
        assertEquals((int)sm._compartment_.stateVars.get("y"), 13);
        assertEquals((int)sm._compartment_.stateVars.get("z"), 101);
    }

    @Test
    public void TestVariablesAreReset(){
        StateVarsController sm = new StateVarsController();
        sm.X(); // increment x
        sm.X(); // increment x
        assertEquals((int)sm._compartment_.stateVars.get("x"), 2);
        sm.Z(); // transition to B
        sm.Z(); // increment z
        sm.Y(); // increment y
        sm.Z(); // increment z
        assertEquals((int)sm._compartment_.stateVars.get("y"), 11);
        assertEquals((int)sm._compartment_.stateVars.get("z"), 102);
        sm.X(); // transition to A
        assertEquals((int)sm._compartment_.stateVars.get("x"), 0);
        sm.Y(); // transition to B
        assertEquals((int)sm._compartment_.stateVars.get("y"), 10);
        assertEquals((int)sm._compartment_.stateVars.get("z"), 100);
    }
}

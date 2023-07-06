package framec_tests.java.StateParams;

import static org.junit.Assert.assertEquals;

import java.util.Arrays;

import org.junit.Test;

class StateParamsController extends StateParams {

    StateParamsController() {
      super();
    }

protected void got_param_do(String name, int val) {
    this.param_log.add(name+"="+String.valueOf(val));
}
}

public class StateParamsTest {
    @Test
    public void TestSingleParameter(){
        StateParamsController sm = new StateParamsController();
        sm.Next();
        sm.Log();
        assertEquals(Arrays.asList("val=1"), sm.param_log);
    }

    @Test
    public void TestMultipleParameters(){
        StateParamsController sm = new StateParamsController();
        sm.Next();
        sm.Next();
        sm.Log();
        assertEquals(Arrays.asList("left=1", "right=2"), sm.param_log);
    }

    @Test
    public void TestSeveralPasses(){
        StateParamsController sm = new StateParamsController();

        sm.Next(); // val=1
        sm.Next(); // left=1 right=2
        sm.Next(); // val=3
        sm.Log();
        sm.Prev(); // left=4 right=3
        sm.Log();
        sm.Prev(); // val=12
        sm.Log();
        assertEquals(Arrays.asList("val=3", "left=4", "right=3", "val=12"), sm.param_log);
    }
}

package framec_tests.java.Simple_handler_calls;

import static org.junit.Assert.assertEquals;
import org.junit.Test;

class SimpleHandlerCallsController extends SimpleHandlerCalls {

    SimpleHandlerCallsController() {
      super();
    }
}

public class Simple_handler_calls_test {

    /* Test a basic handler call. */
    @Test
    public void TestSimpleCall(){
        SimpleHandlerCallsController sm = new SimpleHandlerCallsController();
        sm.C();
        assertEquals(sm.state_info(), "1");
    }

    /* Test that a handler call terminates the current handler. */
    @Test
    public void TestCallsTerminateHandler(){
        SimpleHandlerCallsController sm  = new SimpleHandlerCallsController();
        sm.D();
        assertEquals(sm.state_info(), "2");

        sm = new SimpleHandlerCallsController();
        sm.E();
        assertEquals(sm.state_info(), "2");
    }
    
}


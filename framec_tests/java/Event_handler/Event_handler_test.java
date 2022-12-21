package framec_tests.java.Event_handler;

import java.util.*;
import static org.junit.Assert.assertEquals;
import org.junit.Test;

class EventHandlerController extends EventHandler {

	EventHandlerController() {
	  super();
	}
	public void log_do(String msg, int val) {
        String value = msg + "=" + String.valueOf(val); 
		this.tape.add(value);
	}
};
public class Event_handler_test {
    @Test
    public void TestSingleParameter(){
        EventHandlerController sm = new EventHandlerController();
        sm.LogIt(2);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("x=2");
        assertEquals(sm.tape, expected);
    }

    @Test
    public void TestComputeTwoParameter(){
        EventHandlerController sm = new EventHandlerController();
        sm.LogAdd(-3, 10);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("a=-3");
        expected.add("b=10");
        expected.add("a+b=7");
        assertEquals(sm.tape, expected);  
    }

    @Test
    public void TestReturnLocalVariable(){
        EventHandlerController sm = new EventHandlerController();
        int ret = sm.LogReturn(13, 21);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("a=13");
        expected.add("b=21");
        expected.add("r=34");
        assertEquals(sm.tape, expected);
        assertEquals(ret, 34);
    }

    @Test
    public void TestPassResult(){
        EventHandlerController sm = new EventHandlerController();
        sm.PassAdd(5, -12);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("p=-7");
        assertEquals(sm.tape, expected);  
    }

    @Test
    public void TestPassAndReturnResult(){
        EventHandlerController sm = new EventHandlerController();
        int ret = sm.PassReturn(101, -59);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("r=42");
        expected.add("p=42");
        assertEquals(sm.tape, expected);
        assertEquals(ret, 42);
    }
}


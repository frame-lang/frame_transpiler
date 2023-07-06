package framec_tests.java.Handler_calls;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;

import java.util.ArrayList;

import org.junit.Test;


class HandlerCallsController extends HandlerCalls {
        HandlerCallsController() {
          super();
        }
        protected void log_do(String from, int val) {
          String value = from + "(" + String.valueOf(val) + ")"; 
          this.tape.add(value);
        }
      }
      
public class Handler_calls_test {
    @Test
    public void TestCallTerminateHandler(){
        HandlerCallsController sm = new HandlerCallsController();
        sm.NonRec();
        sm.Foo(10);
        assertNotEquals("Unreachable(0)","Handler calls unreachable statement", sm.tape);
    }

    @Test
    public void TestNonRecursive(){
        HandlerCallsController sm = new HandlerCallsController();
        sm.NonRec();
        sm.Foo(10);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("Foo(10)");
        expected.add("Bar(20)");
        expected.add("Final(30)");
        assertEquals(expected, sm.tape);
    }

    @Test
    public void TestSelfRecursive(){
        HandlerCallsController sm = new HandlerCallsController();
        sm.SelfRec();
        sm.Foo(10);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("Foo(10)");
        expected.add("Foo(20)");
        expected.add("Foo(40)");
        expected.add("Foo(80)");
        expected.add("Final(150)");
        assertEquals(expected, sm.tape);
    }

    @Test
    public void TestMutuallyRecursive(){
        HandlerCallsController sm = new HandlerCallsController();
        sm.MutRec();
        sm.Foo(2);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("Foo(2)");
        expected.add("Bar(4)");
        expected.add("Foo(4)");
        expected.add("Bar(8)");
        expected.add("Foo(16)");
        expected.add("Bar(32)");
        expected.add("Foo(96)");
        expected.add("Final(162)");
        assertEquals(expected, sm.tape);
    }

    @Test
    public void TestStringMatchCall(){
        HandlerCallsController sm = new HandlerCallsController();
        sm.NonRec();
        sm.Call("Foo", 5);
        ArrayList<String> expected = new ArrayList<String>();
        expected.add("Foo(5)");
        expected.add("Bar(10)");
        expected.add("Final(15)");
        assertEquals(expected, sm.tape);
        sm.tape = new ArrayList<>();

        sm.NonRec();
        sm.Call("Bar", 20);
        ArrayList<String> expected1 = new ArrayList<String>();
        expected1.add("Bar(20)");
        expected1.add("Final(20)");
        assertEquals(expected1, sm.tape);
        sm.tape = new ArrayList<>();

        sm.NonRec();
        sm.Call("Qux", 37);
        ArrayList<String> expected2 = new ArrayList<String>();
        expected2.add("Foo(1000)");
        expected2.add("Bar(2000)");
        expected2.add("Final(3000)");
        assertEquals(expected2, sm.tape);
    }

}


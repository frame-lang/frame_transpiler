package framec_tests.java.TransitionParams;

import static org.junit.Assert.assertEquals;

import java.util.ArrayList;
import java.util.Arrays;

import org.junit.Test;

class TransitParamsController extends TransitParams {

	TransitParamsController() {
	  super();
	}

    protected void log_do(String msg) {
        this.tape.add(msg);
    }
}

public class TransitionParamsTest {
    @Test
    public void TestEnter(){
        TransitParamsController sm = new TransitParamsController();
        sm.Next();
        assertEquals(Arrays.asList("hi A"), sm.tape);
    }

    @Test
    public void TestEnterAndExit(){
        TransitParamsController sm = new TransitParamsController();
        sm.Next();
        sm.tape = new ArrayList<>();
        sm.Next();
        assertEquals(Arrays.asList("bye A", "hi B", "42"), sm.tape);
        sm.tape = new ArrayList<>();
        sm.Next();
        assertEquals(Arrays.asList("true", "bye B", "hi again A"), sm.tape);
    }

    @Test
    public void TestChangeState(){
        TransitParamsController sm = new TransitParamsController();
        assertEquals("0", sm.state_info());
        sm.Change();
        assertEquals("1", sm.state_info());
        sm.Change();
        assertEquals("2", sm.state_info());
        sm.Change();
        assertEquals("1", sm.state_info());
        assert(sm.tape.size()==0);
    }

    @Test
    public void TestChangeAndTransition(){
        TransitParamsController sm = new TransitParamsController();
        sm.Change();
        assertEquals("1", sm.state_info());
        assert(sm.tape.size()==0);
        sm.Next();
        assertEquals("2", sm.state_info());
        assertEquals(Arrays.asList("bye A", "hi B", "42"), sm.tape);
        sm.tape = new ArrayList<>();
        sm.Change();
        assertEquals("1", sm.state_info());
        assert(sm.tape.size()==0);
        sm.Change();
        sm.Next();
        assertEquals("1", sm.state_info());
        assertEquals(Arrays.asList("true", "bye B", "hi again A"), sm.tape);
    }
}

package framec_tests.java.Basic;
import static org.junit.Assert.assertEquals;

import java.util.ArrayList;

import org.junit.Test;

class BasicController extends Basic {
	BasicController() throws Exception {
	  super();
	}
	protected void entered_do(String msg) {
		this.entry_log.add(msg);

	}

	protected void left_do(String msg){
	  this.exit_log.add(msg);
	  
	}
  
} 
  
public class Basic_test{
	
	@Test
	public void TestIntialEnterEvent() throws Exception{
  	BasicController sm = new BasicController();
	ArrayList<String> expected = new ArrayList<String>();
	expected.add("S0");
	assertEquals(expected, sm.entry_log);
	
	}

	@Test
	public void TestTransitionEnterEvents() throws Exception{
		BasicController sm = new BasicController();
		sm.entry_log = new ArrayList<>();
		sm.A();
		sm.B();
		ArrayList<String> expected = new ArrayList<String>();
		expected.add("S1");
        expected.add("S0");
		assertEquals(expected ,sm.entry_log);     	
	
	}
	@Test
	public void TestTransitionExitEvents() throws Exception{
		BasicController sm = new BasicController();
		sm.A();
		sm.B();

		ArrayList<String> expected = new ArrayList<String>();
		expected.add("S0");
		expected.add("S1");
		
    
		assertEquals(expected ,sm.exit_log);  		
	}

	@Test
	public void TestCurrentState() throws Exception{
		BasicController sm = new BasicController();
		assertEquals("0", sm.state_info());
		sm.A();
		assertEquals("1", sm.state_info());
		sm.B();
		assertEquals("0", sm.state_info());
	}

	
}

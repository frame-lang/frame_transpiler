package framec_tests.java.VarScope;

import static org.junit.Assert.assertEquals;

import java.util.ArrayList;

import org.junit.Test;

class VarScopeController extends VarScope {

	VarScopeController() {
	  super();
	}

    protected void log_do(String s) {
        this.tape.add(s);
    }
    protected void do_nn(){
        this.nn("|nn|[d]");
    }
    protected void do_ny(){
        this.ny("|ny|[d]");
    }
    protected void do_yn(){
        this.yn("|yn|[d]", "|yn|[x]");
    }
    protected void do_yy(){
        this.yy("|yy|[d]", "|yy|[x]");
    }
    
}

public class VarScopeTest {
    protected ArrayList<String> expected(String state, String msg, String x){
        ArrayList<String> result = new ArrayList<String>();
        result.add("#.a");
        result.add("$"+state+"[b]");
        result.add("$"+state+".c");
        result.add("|"+msg+"|"+"[d]");
        result.add("|"+msg+"|"+".e");
        result.add(x);
    
        return result;
    }
  @Test
  public void TestNoShadowing(){
    VarScopeController sm = new VarScopeController();
    sm.to_nn();
    sm.do_nn();
    assertEquals(expected("NN","nn", "#.x"), sm.tape);
  }

  @Test
  public void TestAllShadowingScenerios(){
    VarScopeController sm = new VarScopeController();
    sm.to_nn();
    sm.do_ny();
    assertEquals(expected("NN","ny", "|ny|.x"), sm.tape);
    sm.tape = new ArrayList<>();
    sm.do_yn();
    assertEquals(expected("NN","yn", "|yn|[x]"), sm.tape);
    sm.tape = new ArrayList<>();
    sm.do_yy();
    assertEquals(expected("NN","yy", "|yy|.x"), sm.tape);

    sm = new VarScopeController();
    sm.to_ny();
    sm.do_nn();
    assertEquals(expected("NY","nn", "$NY.x"), sm.tape);
    sm.tape = new ArrayList<>();
    sm.do_ny();
	assertEquals(expected("NY", "ny", "|ny|.x"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_yn();
	assertEquals(expected("NY", "yn", "|yn|[x]"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_yy();
	assertEquals(expected("NY", "yy", "|yy|.x"), sm.tape);

	sm = new VarScopeController();
	sm.to_yn();
	sm.do_nn();
	assertEquals(expected("YN", "nn", "$YN[x]"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_ny();
	assertEquals(expected("YN", "ny", "|ny|.x"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_yn();
	assertEquals(expected("YN", "yn", "|yn|[x]"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_yy();
	assertEquals(expected("YN", "yy", "|yy|.x"), sm.tape);

	sm = new VarScopeController();
	sm.to_yy();
	sm.do_nn();
	assertEquals(expected("YY", "nn", "$YY.x"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_ny();
	assertEquals(expected("YY", "ny", "|ny|.x"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_yn();
	assertEquals(expected("YY", "yn", "|yn|[x]"), sm.tape);
	sm.tape = new ArrayList<>();
	sm.do_yy();
	assertEquals(expected("YY", "yy", "|yy|.x"), sm.tape);

  }
}

package framec_tests.java.Branch;

import static org.junit.Assert.assertEquals;

import java.util.ArrayList;

import org.junit.Test;


class BranchController extends Branch {
  BranchController() {
    super();
  }
  protected void log_do(String msg) {
    this.tape.add(msg);
  }
}
public class Branch_test {
    
@Test
public void TestSimpleIfBool(){
    BranchController sm = new BranchController();
    sm.A();
    sm.OnBool(true);

    assertEquals("7", sm.state_info());
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("then 1");
    expected.add("then 2");

    assertEquals(sm.state_info(), "7");
    assertEquals(sm.tape, expected);
    sm = new BranchController();
    sm.A();
    sm.OnBool(false);

    assertEquals("8" ,sm.state_info());
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("else 1");
    expected1.add("else 2");
    assertEquals(sm.state_info(), "8");
    assertEquals(expected1, sm.tape);
  }

  @Test
  public void TestSimpleIfInit(){
    BranchController sm = new BranchController();
    sm.A();
    sm.OnInt(7);
    assertEquals(sm.state_info(), "7" );
    ArrayList<String> arr = new ArrayList<String>();
    arr.add("> 5");
    arr.add("< 10");
    arr.add("== 7");
    assertEquals(sm.tape, arr);
    sm = new BranchController();
    sm.A();
    sm.OnInt(-3);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> arr1 = new ArrayList<String>();
    arr1.add("<= 5");
    arr1.add("< 10");
    arr1.add("!= 7");
    assertEquals(sm.tape, arr1);
    sm = new BranchController();
    sm.A();
    sm.OnInt(12);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> arr2 = new ArrayList<String>();
    arr2.add("> 5");
    arr2.add(">= 10");
    arr2.add("!= 7");
    assertEquals(sm.tape, arr2);

  }

  @Test
  public void TestNegatedIfBool(){
    BranchController sm = new BranchController();
    sm.B();
    sm.OnBool(true);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected  = new ArrayList<String>();
    expected.add("else 1");
    expected.add("else 2");
    assertEquals(sm.tape, expected);
    sm = new BranchController();
    sm.B();
    sm.OnBool(false);
    assertEquals(sm.state_info(), "7");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("then 1");
    expected1.add("then 2");
    assertEquals(sm.tape, expected1);
  }

  @Test
  public void TestNegatedIfInt(){
    BranchController sm = new BranchController();
    sm.B();
    sm.OnInt(7);
    assertEquals(sm.state_info(), "7");
    ArrayList<String> expected = new ArrayList<>();
    expected.add(">= 5");
    expected.add("<= 10");
    expected.add("== 7");
    assertEquals(sm.tape, expected);

    sm = new BranchController();
    sm.B();
    sm.OnInt(5);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add(">= 5");
    expected1.add("<= 10");
    expected1.add("!= 7");
    assertEquals(sm.tape, expected1);

    sm = new BranchController();
    sm.B();
    sm.OnInt(10);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add(">= 5");
    expected2.add("<= 10");
    expected2.add("!= 7");
    assertEquals(sm.tape, expected2);

    sm = new BranchController();
    sm.B();
    sm.OnInt(0);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("< 5");
    expected3.add("<= 10");
    expected3.add("!= 7");
    assertEquals(sm.tape, expected3);

    sm = new BranchController();
    sm.B();
    sm.OnInt(100);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected4 = new ArrayList<String>();
    expected4.add(">= 5");
    expected4.add("> 10");
    expected4.add("!= 7");
    assertEquals(sm.tape, expected4);
  }

  @Test
  public void TestOperatorPrecedence(){
    BranchController sm = new BranchController();
    sm.C();
    sm.OnInt(0);
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("then 1");
    expected.add("else 2");
    expected.add("then 3");
    expected.add("then 4");
    assertEquals(expected, sm.tape);
    sm.tape = new ArrayList<>();
    sm.OnInt(7);
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("else 1");
    expected1.add("then 2");
    expected1.add("else 3");
    expected1.add("then 4");
    assertEquals(expected1, sm.tape);
    sm.tape = new ArrayList<>();
    sm.OnInt(-3);
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("then 1");
    expected2.add("else 2");
    expected2.add("else 3");
    expected2.add("else 4");
    assertEquals(expected2, sm.tape);
    sm.tape = new ArrayList<>();
    sm.OnInt(12);
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("else 1");
    expected3.add("else 2");
    expected3.add("then 3");
    expected3.add("else 4");
    assertEquals(expected3, sm.tape);
    sm.tape = new ArrayList<>();

  }

  @Test
  public void TestNestedIf(){
    BranchController sm = new BranchController();
    sm.D();
    sm.OnInt(50);
    assertEquals(sm.state_info(), "7");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("> 0");
    expected.add("< 100");
    assertEquals(sm.tape, expected);
    sm = new BranchController();
    sm.D();
    sm.OnInt(200);
    assertEquals(sm.state_info(), "4");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("> 0");
    expected1.add(">= 100");
    assertEquals(sm.tape, expected1);
    sm = new BranchController();
    sm.D();
    sm.OnInt(-5);
    assertEquals(sm.state_info(), "4");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("<= 0");
    expected2.add("> -10");
    assertEquals(sm.tape, expected2);
    sm = new BranchController();
    sm.D();
    sm.OnInt(-10);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("<= 0");
    expected3.add("<= -10");
    assertEquals(sm.tape, expected3);
    sm = new BranchController();
    
  }
  @Test
  public void TestGuardedTransition(){
    BranchController sm = new BranchController();
    sm.E();
    sm.OnInt(5);
    assertEquals(sm.state_info(), "9");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("-> $F3");
    assertEquals(sm.tape, expected);
    sm = new BranchController();
    sm.E();
    sm.OnInt(15);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("-> $F2");
    assertEquals(sm.tape, expected1);
    sm = new BranchController();
    sm.E();
    sm.OnInt(115);
    assertEquals(sm.state_info(), "7");
    ArrayList<String> expected2 = new ArrayList<>();
    expected2.add("-> $F1");
    assertEquals(sm.tape, expected2);
  }
  /*
	Test that a transition guarded by a nested conditional expression

triggers an early return from the handler, but this return doesn't

	apply to non-transitioned branches.
*/
  @Test
  public void TestNestedGuardedTransition(){
    BranchController sm = new BranchController();
    sm.F();
    sm.OnInt(5);
    assertEquals(sm.state_info(), "9");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("-> $F3");
    assertEquals(sm.tape, expected);
    sm = new BranchController();
    sm.tape = new ArrayList<>();
    sm.F();
    sm.OnInt(15);
    assertEquals(sm.state_info(), "8");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("-> $F2");
    assertEquals(sm.tape, expected1);
    sm = new BranchController();
    sm.F();
    sm.OnInt(65);
    assertEquals(sm.state_info(), "9");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("-> $F3");
    assertEquals(sm.tape, expected2);
    sm = new BranchController();
    sm.F();
    sm.OnInt(115);
    assertEquals(sm.state_info(), "7");
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("-> $F1");
    assertEquals(sm.tape, expected3);
    
  }
}
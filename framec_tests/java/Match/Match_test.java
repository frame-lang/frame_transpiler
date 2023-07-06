package framec_tests.java.Match;

import java.util.*;
import static org.junit.Assert.assertEquals;
import org.junit.Test;

class MatchController extends Match {

    MatchController() {
      super();
    }

    protected void log_do(String msg) {
        this.tape.add(msg);
    }
}
// Test matching the empty string.
// TODO: Matching the empty string currently only works in multi-string
// patterns. The pattern `//`, which should match only the empty string,
// instead produces a parse error.
public class Match_test {
  @Test
  public void TestEmptyString(){
    MatchController sm = new MatchController();
    sm.Empty();
    sm.OnString("");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("empty");
    assertEquals(sm.tape, expected);
    sm.tape = new ArrayList<>();
    sm.OnString("hi");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("?");
    assertEquals(sm.tape, expected1);
  }
  
  @Test
  public void TestIntegerMatch(){
    MatchController sm = new MatchController();
    sm.Simple();
    sm.OnInt(0);
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("0");
    assertEquals(sm.tape, expected);
    sm.tape = new ArrayList<>();
    sm.OnInt(42);
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("42");
    assertEquals(sm.tape, expected1);
    sm.tape = new ArrayList<>();
    sm.OnInt(-200);
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("-200");
    assertEquals(sm.tape, expected2);
  }

  /*
	Test simple string matching.

TODO: Testing revealed some limitations:
  - Frame does not support UTF-8 graphemes larger than 1 byte, so we're
    restricted to ASCII.
  - Frame does not have a way to match the '/' or '|' characters,
    which are part of the matching syntax.
*/

  @Test
  public void TestStringMatch(){
    MatchController sm = new MatchController();
    sm.Simple();
    sm.OnString("hello");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("hello");
    assertEquals(sm.tape, expected);
    sm.tape = new ArrayList<>();
    sm.OnString("goodbye");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("goodbye");
    assertEquals(sm.tape, expected1);
    sm.tape = new ArrayList<>();
    sm.OnString("Testing 1, 2, 3...");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("testing");
    assertEquals(sm.tape, expected2);
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("testing");
    assertEquals(sm.tape, expected3);
    sm.tape = new ArrayList<>();
    sm.OnString("$10!");
    ArrayList<String> expected4 = new ArrayList<String>();
    expected4.add("money");
    assertEquals(sm.tape, expected4);
    ArrayList<String> expected5 = new ArrayList<String>();
    expected5.add("money");
    assertEquals(sm.tape, expected5);
    sm.tape = new ArrayList<>();
    sm.OnString("missing");
    ArrayList<String> expected6 = new ArrayList<String>();
    expected6.add("?");
    assertEquals(sm.tape, expected6);
    ArrayList<String> expected7 = new ArrayList<String>();
    expected7.add("?");
    assertEquals(sm.tape, expected7);
    sm.tape = new ArrayList<>();
    sm.OnString("Testing");
    ArrayList<String> expected8 = new ArrayList<String>();
    expected8.add("?");
    assertEquals(sm.tape, expected8);
    ArrayList<String> expected9 = new ArrayList<String>();
    expected9.add("?");
    assertEquals(sm.tape, expected9);
    sm.tape = new ArrayList<>();
    sm.OnString("");
    ArrayList<String> expected10 = new ArrayList<String>();
    expected10.add("?");
    assertEquals(sm.tape, expected10);
    ArrayList<String> expected11 = new ArrayList<String>();
    expected11.add("?");
    assertEquals(sm.tape, expected11);

  }
  /* Test the multiple match syntax for integers. */

  @Test
  public void TestIntegerMultiMatch(){
    MatchController sm = new MatchController();
    sm.Multi();
    sm.OnInt(3);
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("3|-7");
    assertEquals(sm.tape, expected);
    sm.tape = new ArrayList<>();
    sm.OnInt(-7);
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("3|-7");
    assertEquals(sm.tape, expected1);
    sm.tape = new ArrayList<>();
    sm.OnInt(-4);
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("-4|5|6");
    assertEquals(sm.tape, expected2);
    sm.tape = new ArrayList<>();
    sm.OnInt(5);
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("-4|5|6");
    assertEquals(sm.tape, expected3);
    sm.tape = new ArrayList<>();
    sm.OnInt(6);
    ArrayList<String> expected4 = new ArrayList<String>();
    expected4.add("-4|5|6");
    assertEquals(sm.tape, expected4);
    sm.tape = new ArrayList<>();
    sm.OnInt(4);
    ArrayList<String> expected5 = new ArrayList<String>();
    expected5.add("?");
    assertEquals(sm.tape, expected5);
    sm.tape = new ArrayList<>();
    sm.OnInt(0);
    ArrayList<String> expected6 = new ArrayList<String>();
    expected6.add("?");
    assertEquals(sm.tape, expected6);
    sm.tape = new ArrayList<>();
  }

  // Test the multiple match syntax for integers. Also tests matching
// whitespace-only strings.

  @Test
  public void TestStringMultiMatch(){
    MatchController sm = new MatchController();
    sm.Multi();
    sm.OnString("$10");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("symbols");
    assertEquals(sm.tape, expected);
    sm.tape = new ArrayList<>();
    sm.OnString("12.5%");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("symbols");
    assertEquals(sm.tape, expected1);
    sm.tape = new ArrayList<>();
    sm.OnString("@#*!");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("symbols");
    assertEquals(sm.tape, expected2);
    sm.tape = new ArrayList<>();
    sm.OnString(" ");
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("whitespace");
    assertEquals(sm.tape, expected3);
    sm.tape = new ArrayList<>();
    sm.OnString(" ");
    ArrayList<String> expected4 = new ArrayList<String>();
    expected4.add("whitespace");
    assertEquals(sm.tape, expected4);
    sm.tape = new ArrayList<>();
    sm.OnString("\t");
    ArrayList<String> expected5 = new ArrayList<String>();
    expected5.add("whitespace");
    assertEquals(sm.tape, expected5);
    sm.tape = new ArrayList<>();
    sm.OnString("\n");
    ArrayList<String> expected6 = new ArrayList<String>();
    expected6.add("whitespace");
    assertEquals(sm.tape, expected6);
    sm.tape = new ArrayList<>();
    sm.OnString("10");
    ArrayList<String> expected7 = new ArrayList<String>();
    expected7.add("?");
    assertEquals(sm.tape, expected7);
    sm.tape = new ArrayList<>();
    sm.OnString("#");
    ArrayList<String> expected8 = new ArrayList<String>();
    expected8.add("?");
    assertEquals(sm.tape, expected8);
    sm.tape = new ArrayList<>();
    sm.OnString("   ");
    ArrayList<String> expected9 = new ArrayList<String>();
    expected9.add("?");
    assertEquals(sm.tape, expected9);
    sm.tape = new ArrayList<>();
    sm.OnString("");
    ArrayList<String> expected10 = new ArrayList<String>();
    expected10.add("?");
    assertEquals(sm.tape, expected10);
    sm.tape = new ArrayList<>();
  }
  /* Test nested integer matching. */

  @Test
  public void TestIntegerNestedMatch(){
    MatchController sm = new MatchController();
    sm.Nested();
    sm.OnInt(1);
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("1-3");
    expected.add("1");
    assertEquals(sm.tape, expected);
    sm.tape = new ArrayList<>();
    sm.OnInt(2);
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("1-3");
    expected1.add("2");
    assertEquals(sm.tape, expected1);
    sm.tape = new ArrayList<>();
    sm.OnInt(3);
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("1-3");
    expected2.add("3");
    assertEquals(sm.tape, expected2);
    sm.tape = new ArrayList<>();
    sm.OnInt(4);
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("4-5");
    expected3.add("4");
    assertEquals(sm.tape, expected3);
    sm.tape = new ArrayList<>();
    sm.OnInt(5);
    ArrayList<String> expected4 = new ArrayList<String>();
    expected4.add("4-5");
    expected4.add("5");
    assertEquals(sm.tape, expected4);
    sm.tape = new ArrayList<>();
    sm.OnInt(10);
    ArrayList<String> expected5 = new ArrayList<String>();
    expected5.add("too big");
    assertEquals(sm.tape, expected5);
    sm.tape = new ArrayList<>();
    sm.OnInt(0);
    ArrayList<String> expected6 = new ArrayList<String>();
    expected6.add("too small");
    assertEquals(sm.tape, expected6);
    sm.tape = new ArrayList<>();
  }

  /* Test nested string matching. */
  @Test
  public void TestStringNestedMatch(){
    MatchController sm = new MatchController();
    sm.Nested();
    sm.OnString("hello");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("greeting");
    expected.add("English");
    assertEquals(sm.tape, expected);
    sm.tape = new ArrayList<>();
    sm.OnString("hola");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("greeting");
    expected1.add("Spanish");
    assertEquals(sm.tape, expected1);
    sm.tape = new ArrayList<>();
    sm.OnString("bonjour");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("greeting");
    expected2.add("French");
    assertEquals(sm.tape, expected2);
    sm.tape = new ArrayList<>();
    sm.OnString("goodbye");
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("farewell");
    expected3.add("English");
    assertEquals(sm.tape, expected3);
    sm.tape = new ArrayList<>();
    sm.OnString("adios");
    ArrayList<String> expected4 = new ArrayList<String>();
    expected4.add("farewell");
    expected4.add("Spanish");
    assertEquals(sm.tape, expected4);
    sm.tape = new ArrayList<>();
    sm.OnString("au revoir");
    ArrayList<String> expected5 = new ArrayList<String>();
    expected5.add("farewell");
    expected5.add("French");
    assertEquals(sm.tape, expected5);
    sm.tape = new ArrayList<>();
    sm.OnString("hallo");
    ArrayList<String> expected6 = new ArrayList<String>();
    expected6.add("?");
    assertEquals(sm.tape, expected6);
    sm.tape = new ArrayList<>();
    sm.OnString("ciao");
    ArrayList<String> expected7 = new ArrayList<String>();
    expected7.add("?");
    assertEquals(sm.tape, expected7);
    sm.tape = new ArrayList<>();
  }
  /* Test hierarchical integer matching. */

  @Test
  public void TestIntegerHierarchicalMatch(){
    MatchController sm = new MatchController();
    sm.Child();
    sm.OnInt(0);
    assertEquals(sm.state_info(), "6");
    sm.tape = new ArrayList<>();

    sm = new MatchController();
    sm.Child();
    sm.OnInt(4);
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("4");
    assertEquals(sm.tape, expected);

    sm.tape = new ArrayList<String>();
    sm.OnInt(5);
    assertEquals(sm.state_info(), "6");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("5");
    assertEquals(sm.tape, expected1);
    
    sm = new MatchController();
    sm.tape = new ArrayList<String>();
    sm.Child();
    sm.OnInt(3);
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("3");
    expected2.add("?");
    assertEquals(sm.tape, expected2);

    sm.tape = new ArrayList<String>();
    sm.OnInt(42);
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("42 in child");
    expected3.add("42");
    assertEquals(sm.tape, expected3);

    sm.tape = new ArrayList<String>();
    sm.OnInt(-200);
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected4 = new ArrayList<String>();
    expected4.add("no match in child");
    expected4.add("-200");
    assertEquals(sm.tape, expected4);

    sm.tape = new ArrayList<String>();
    sm.OnInt(100);
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected5 = new ArrayList<String>();
    expected5.add("no match in child");
    expected5.add("?");
    assertEquals(sm.tape, expected5);
  }

  /* Test hierarchical string matching. */
  @Test
  public void TestStringHierarchicalMatch(){
    MatchController sm = new MatchController();
    sm.Child();
    sm.OnString("goodbye");
    assertEquals(sm.state_info(), "6");
    sm.tape = new ArrayList<>();

    sm = new MatchController();
    sm.Child();
    sm.OnString("hello");
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected = new ArrayList<String>();
    expected.add("hello in child");
    expected.add("hello");
    assertEquals(sm.tape, expected);

    sm.tape = new ArrayList<>();
    sm.OnString("Testing 1, 2, 3...");
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected1 = new ArrayList<String>();
    expected1.add("testing in child");
    assertEquals(sm.tape, expected1);

    sm.tape = new ArrayList<>();
    sm.OnString("$10!");
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected2 = new ArrayList<String>();
    expected2.add("no match in child");
    expected2.add("money");
    assertEquals(sm.tape, expected2);

    sm.tape = new ArrayList<>();
    sm.OnString("testing 1, 2, 3...");
    assertEquals(sm.state_info(), "5");
    ArrayList<String> expected3 = new ArrayList<String>();
    expected3.add("no match in child");
    expected3.add("?");
    assertEquals(sm.tape, expected3);
  }
}


const { describe, it } = require("mocha");
const assert = require("assert");
const Match = require("./match");
const returnStateName = require("../utils/state_info/returnStateName");

class MatchController extends Match {
  constructor() {
    super();
  }
  log_do(msg) {
    this.tape.push(msg);
  }
}

describe("Match", () => {
  /// Test matching the empty string.
  /// TODO: Matching the empty string currently only works in multi-string
  /// patterns. The pattern `//`, which should match only the empty string,
  /// instead produces a parse error.
  it("Empty string", () => {
    let sm = new MatchController();
    sm.Empty();
    sm.OnString("");
    assert.deepStrictEqual(sm.tape, ["empty"]);
    sm.tape = [];
    sm.OnString("hi");
    assert.deepStrictEqual(sm.tape, ["?"]);
  });

  /// Test simple integer matching.
  it("Integer match", () => {
    let sm = new MatchController();
    sm.Simple();
    sm.OnInt(0);
    assert.deepStrictEqual(sm.tape, ["0"]);
    sm.tape = [];
    sm.OnInt(42);
    assert.deepStrictEqual(sm.tape, ["42"]);
    sm.tape = [];
    sm.OnInt(-200);
    assert.deepStrictEqual(sm.tape, ["-200"]);
    sm.tape = [];
  });

  /// Test simple string matching.
  /// TODO: Testing revealed some limitations:
  ///  * Frame does not support UTF-8 graphemes larger than 1 byte, so we're
  ///    restricted to ASCII.
  ///  * Frame does not have a way to match the '/' or '|' characters,
  ///    which are part of the matching syntax.
  it("String match", () => {
    let sm = new MatchController();
    sm.Simple();
    sm.OnString("hello");
    assert.deepStrictEqual(sm.tape, ["hello"]);
    sm.tape = [];
    sm.OnString("goodbye");
    assert.deepStrictEqual(sm.tape, ["goodbye"]);
    sm.tape = [];
    sm.OnString("Testing 1, 2, 3...");
    assert.deepStrictEqual(sm.tape, ["testing"]);
    sm.tape = [];
    sm.OnString("$10!");
    assert.deepStrictEqual(sm.tape, ["money"]);
    sm.tape = [];
    sm.OnString("missing");
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
    sm.OnString("Testing");
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
    sm.OnString("");
    assert.deepStrictEqual(sm.tape, ["?"]);
  });

  /// Test the multiple match syntax for integers.
  it("Integer multi match", () => {
    let sm = new MatchController();
    sm.Multi();
    sm.OnInt(3);
    assert.deepStrictEqual(sm.tape, ["3|-7"]);
    sm.tape = [];
    sm.OnInt(-7);
    assert.deepStrictEqual(sm.tape, ["3|-7"]);
    sm.tape = [];
    sm.OnInt(-4);
    assert.deepStrictEqual(sm.tape, ["-4|5|6"]);
    sm.tape = [];
    sm.OnInt(5);
    assert.deepStrictEqual(sm.tape, ["-4|5|6"]);
    sm.tape = [];
    sm.OnInt(6);
    assert.deepStrictEqual(sm.tape, ["-4|5|6"]);
    sm.tape = [];
    sm.OnInt(4);
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
    sm.OnInt(0);
    assert.deepStrictEqual(sm.tape, ["?"]);
  });

  /// Test the multiple match syntax for integers. Also tests matching
  /// whitespace-only strings.
  it("String multi match", () => {
    let sm = new MatchController();
    sm.Multi();
    sm.OnString("$10");
    assert.deepStrictEqual(sm.tape, ["symbols"]);
    sm.tape = [];
    sm.OnString("12.5%");
    assert.deepStrictEqual(sm.tape, ["symbols"]);
    sm.tape = [];
    sm.OnString("@#*!");
    assert.deepStrictEqual(sm.tape, ["symbols"]);
    sm.tape = [];
    sm.OnString(" ");
    assert.deepStrictEqual(sm.tape, ["whitespace"]);
    sm.tape = [];
    sm.OnString("  ");
    assert.deepStrictEqual(sm.tape, ["whitespace"]);
    sm.tape = [];
    sm.OnString("\t");
    assert.deepStrictEqual(sm.tape, ["whitespace"]);
    sm.tape = [];
    sm.OnString("\n");
    assert.deepStrictEqual(sm.tape, ["whitespace"]);
    sm.tape = [];
    sm.OnString("10");
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
    sm.OnString("#");
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
    sm.OnString("   ");
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
    sm.OnString("");
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
  });
  /// Test nested integer matching.

  it("Integer nested match", () => {
    let sm = new MatchController();
    sm.Nested();
    sm.OnInt(1);
    assert.deepStrictEqual(sm.tape, ["1-3", "1"]);
    sm.tape = [];
    sm.OnInt(2);
    assert.deepStrictEqual(sm.tape, ["1-3", "2"]);
    sm.tape = [];
    sm.OnInt(3);
    assert.deepStrictEqual(sm.tape, ["1-3", "3"]);
    sm.tape = [];
    sm.OnInt(4);
    assert.deepStrictEqual(sm.tape, ["4-5", "4"]);
    sm.tape = [];
    sm.OnInt(5);
    assert.deepStrictEqual(sm.tape, ["4-5", "5"]);
    sm.tape = [];
    sm.OnInt(10);
    assert.deepStrictEqual(sm.tape, ["too big"]);
    sm.tape = [];
    sm.OnInt(0);
    assert.deepStrictEqual(sm.tape, ["too small"]);
  });
  /// Test nested string matching.

  it("String nested match", () => {
    let sm = new MatchController();
    sm.Nested();
    sm.OnString("hello");
    assert.deepStrictEqual(sm.tape, ["greeting", "English"]);
    sm.tape = [];
    sm.OnString("hola");
    assert.deepStrictEqual(sm.tape, ["greeting", "Spanish"]);
    sm.tape = [];
    sm.OnString("bonjour");
    assert.deepStrictEqual(sm.tape, ["greeting", "French"]);
    sm.tape = [];
    sm.OnString("goodbye");
    assert.deepStrictEqual(sm.tape, ["farewell", "English"]);
    sm.tape = [];
    sm.OnString("adios");
    assert.deepStrictEqual(sm.tape, ["farewell", "Spanish"]);
    sm.tape = [];
    sm.OnString("au revoir");
    assert.deepStrictEqual(sm.tape, ["farewell", "French"]);
    sm.tape = [];
    sm.OnString("hallo");
    assert.deepStrictEqual(sm.tape, ["?"]);
    sm.tape = [];
    sm.OnString("ciao");
    assert.deepStrictEqual(sm.tape, ["?"]);
  });
  /// Test hierarchical integer matching.

  it("Integer hierarchical match", () => {
    let sm = new MatchController();
    sm.Child();
    sm.OnInt(0);
    assert.deepStrictEqual(sm.state_info(), returnStateName("Final"))
    assert(sm.tape.length === 0);

    sm = new MatchController();
    sm.Child();
    sm.OnInt(4);
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["4"]);

    sm.tape = [];
    sm.OnInt(5);
    assert.deepStrictEqual(sm.state_info(), returnStateName("Final"))
    assert.deepStrictEqual(sm.tape, ["5"]);

    sm = new MatchController();
    sm.Child();
    sm.OnInt(5);
    assert.deepStrictEqual(sm.state_info(), returnStateName("Final"))
    assert.deepStrictEqual(sm.tape, ["5"]);

    sm = new MatchController();
    sm.Child();
    sm.OnInt(3);
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["3", "?"]);

    sm.tape = [];
    sm.OnInt(42);
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["42 in child", "42"]);

    sm.tape = [];
    sm.OnInt(-200);
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["no match in child", "-200"]);

    sm.tape = [];
    sm.OnInt(100);
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["no match in child", "?"]);
  });

  /// Test hierarchical string matching.
  it("String hierarchical match", () => {
    let sm = new MatchController();
    sm.Child();
    sm.OnString("goodbye");
    assert.deepStrictEqual(sm.state_info(), returnStateName("Final"))
    assert(sm.tape.length === 0);

    sm = new MatchController();
    sm.Child();
    sm.OnString("hello");
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["hello in child", "hello"]);

    sm.tape = [];
    sm.OnString("Testing 1, 2, 3...");
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["testing in child"]);

    sm.tape = [];
    sm.OnString("$10!");
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["no match in child", "money"]);

    sm.tape = [];
    sm.OnString("testing 1, 2, 3...");
    assert.deepStrictEqual(sm.state_info(), returnStateName("ChildMatch"))
    assert.deepStrictEqual(sm.tape, ["no match in child", "?"]);
  });
});

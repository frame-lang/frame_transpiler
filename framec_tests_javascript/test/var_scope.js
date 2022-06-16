const { describe, it } = require("mocha");
const assert = require("assert");
const VarScope = require("../output/var_scope");

class VarScopeController extends VarScope {
  constructor() {
    super();
  }
  log_do(s) {
    this.tape.push(s);
  }

  do_nn() {
    this.nn("|nn|[d]");
  }

  do_ny() {
    this.ny("|ny|[d]");
  }

  do_yn() {
    this.yn("|yn|[d]", "|yn|[x]");
  }

  do_yy() {
    this.yy("|yy|[d]", "|yy|[x]");
  }
}

// Functions for testing purpose not related to Frame spec

const expected = (state, msg, x) => {
  let result = [];

  result.push("#.a");
  result.push(`$${state}[b]`);
  result.push(`$${state}.c`);
  result.push(`|${msg}|[d]`);
  result.push(`|${msg}|.e`);
  result.push(x);

  return result;
};

describe("Var scope", () => {
  it("No shadowing", () => {
    let sm = new VarScopeController();
    sm.to_nn();
    sm.do_nn();
    assert.deepStrictEqual(sm.tape, expected("NN", "nn", "#.x"));
  });

  it("All shadowing scenarios", () => {
    let sm = new VarScopeController();
    sm.to_nn();
    sm.do_ny();
    assert.deepStrictEqual(sm.tape, expected("NN", "ny", "|ny|.x"));
    sm.tape = [];
    sm.do_yn();
    assert.deepStrictEqual(sm.tape, expected("NN", "yn", "|yn|[x]"));
    sm.tape = [];
    sm.do_yy();
    assert.deepStrictEqual(sm.tape, expected("NN", "yy", "|yy|.x"));

    sm = new VarScopeController();
    sm.to_ny();
    sm.do_nn();
    assert.deepStrictEqual(sm.tape, expected("NY", "nn", "$NY.x"));
    sm.tape = [];
    sm.do_ny();
    assert.deepStrictEqual(sm.tape, expected("NY", "ny", "|ny|.x"));
    sm.tape = [];
    sm.do_yn();
    assert.deepStrictEqual(sm.tape, expected("NY", "yn", "|yn|[x]"));
    sm.tape = [];
    sm.do_yy();
    assert.deepStrictEqual(sm.tape, expected("NY", "yy", "|yy|.x"));

    sm = new VarScopeController();
    sm.to_yn();
    sm.do_nn();
    assert.deepStrictEqual(sm.tape, expected("YN", "nn", "$YN[x]"));
    sm.tape = [];
    sm.do_ny();
    assert.deepStrictEqual(sm.tape, expected("YN", "ny", "|ny|.x"));
    sm.tape = [];
    sm.do_yn();
    assert.deepStrictEqual(sm.tape, expected("YN", "yn", "|yn|[x]"));
    sm.tape = [];
    sm.do_yy();
    assert.deepStrictEqual(sm.tape, expected("YN", "yy", "|yy|.x"));

    sm = new VarScopeController();
    sm.to_yy();
    sm.do_nn();
    assert.deepStrictEqual(sm.tape, expected("YY", "nn", "$YY.x"));
    sm.tape = [];
    sm.do_ny();
    assert.deepStrictEqual(sm.tape, expected("YY", "ny", "|ny|.x"));
    sm.tape = [];
    sm.do_yn();
    assert.deepStrictEqual(sm.tape, expected("YY", "yn", "|yn|[x]"));
    sm.tape = [];
    sm.do_yy();
    assert.deepStrictEqual(sm.tape, expected("YY", "yy", "|yy|.x"));
  });
});

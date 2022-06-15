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

let sm = new VarScopeController()
sm.to_nn();
sm.do_nn();

// Functions for testing purpose not related to Frame spec

const expected = (state, msg, x) => {
  let result = [];

  result.push("#.a");
  result.push(`${state}[b]`);
  result.push(`${state}.c`);
  result.push(`|${msg}|[d]`);
  result.push(`|${msg}|.e`);
  result.push(x);

  return result
};


// describe("Var scope", ()=>{
//     it("No shadowing", ()=>{
//         let sm = new VarScopeController()
//         sm.to_nn();
//         sm.do_nn();
//         assert.deepStrictEqual(sm.tape, expected("NN", "nn", "#.x"));
//     })

// })
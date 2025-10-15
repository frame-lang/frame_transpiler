const { describe, it } = require("mocha");
const assert = require("assert");
const TransitParams = require("./transition_params");
const returnStateName = require("../utils/state_info/returnStateName");

class TransitParamsController extends TransitParams {
  constructor() {
    super();
  }
  log_do(msg) {
    this.tape.push(msg);
  }
}

describe("Transition Params", () => {
  it("Enter", () => {
    let sm = new TransitParamsController();
    sm.Next();
    assert.deepStrictEqual(sm.tape, ["hi A"]);
  });

  it("Enter and exit", () => {
    let sm = new TransitParamsController();
    sm.Next();
    sm.tape = [];
    sm.Next();
    assert.deepStrictEqual(sm.tape, ["bye A", "hi B", "42"]);
    sm.tape = [];
    sm.Next();
    assert.deepStrictEqual(sm.tape, ["true", "bye B", "hi again A"]);
  });

  it("Change state", () => {
    let sm = new TransitParamsController();
    assert.deepStrictEqual(sm.state_info(), returnStateName("Init"));
    sm.Change();
    assert.deepStrictEqual(sm.state_info(), returnStateName("A"));
    sm.Change();
    assert.deepStrictEqual(sm.state_info(), returnStateName("B"));
    sm.Change();
    assert.deepStrictEqual(sm.state_info(), returnStateName("A"));
    assert.deepEqual(sm.tape.length, 0);
  });

  it("Change and transition", () => {
    let sm = new TransitParamsController();
    sm.Change();
    assert.deepStrictEqual(sm.state_info(), returnStateName("A"));
    assert.deepEqual(sm.tape.length, 0)
    sm.Next();
    assert.deepStrictEqual(sm.state_info(), returnStateName("B"));
    assert.deepStrictEqual(sm.tape, ["bye A", "hi B", "42"]);
    sm.tape = []
    sm.Change()
    assert.deepStrictEqual(sm.state_info(), returnStateName("A"));
    assert.deepEqual(sm.tape.length, 0)
    sm.Change()
    sm.Next()
    assert.deepStrictEqual(sm.state_info(), returnStateName("A"));
    assert.deepStrictEqual(sm.tape, ["true", "bye B", "hi again A"]);
  });

});
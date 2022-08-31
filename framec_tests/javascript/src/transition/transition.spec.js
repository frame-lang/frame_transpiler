const { describe, it } = require("mocha");
const assert = require("assert");
const TransitionSm = require("./transition");
const returnStateName = require("../utils/state_info/returnStateName");

class TransitionSmController extends TransitionSm {
  constructor() {
    super();
  }
  enter_do(state) {
    this.enters.push(state);
  }
  exit_do(state) {
    this.exits.push(state);
  }

  clear_all() {
    this.enters = [];
    this.exits = [];
  }
}

describe("Transition", () => {

  // Test that transition works and triggers enter and exit events.
  it("Transition events", () => {
    let sm = new TransitionSmController();
    sm.clear_all();
    sm.transit();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    assert.deepStrictEqual(sm.exits, ["S0"]);
    assert.deepStrictEqual(sm.enters, ["S1"]);
  });

  // Test that change-state works and does not trigger events.
  it("Change state no events", () => {
    let sm = new TransitionSmController();
    
    sm.clear_all()
    sm.change()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"))
    sm.change()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S2"))
    sm.change()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S3"))
    sm.change()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S4"))
    assert.deepEqual(sm.exits.length , 0)
    assert.deepEqual(sm.enters.length , 0)

  });

  // Test transition that triggers another transition in an enter event handler.
  it("Cascading transition", () => {
    let sm = new TransitionSmController();
    sm.change()
    sm.clear_all()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"))
    sm.transit()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S3"))
    assert.deepStrictEqual(sm.exits, ["S1", "S2"])
    assert.deepStrictEqual(sm.enters, ["S2", "S3"])
  });

  // Test transition that triggers a change-state from an enter event handler.
  it("Cascading change state", () => {
    let sm = new TransitionSmController();
    sm.change()
    sm.change()
    sm.change()
    sm.clear_all()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S3"))
    sm.transit()
    assert.deepStrictEqual(sm.state_info(), returnStateName("S0"))
    assert.deepStrictEqual(sm.exits, ["S3"])
    assert.deepStrictEqual(sm.enters, ["S4"])
  });
});


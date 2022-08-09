const { describe, it } = require("mocha");
const assert = require("assert");

const Hierarchical = require("./hierarchical");
const returnStateName = require("../utils/state_info/returnStateName");

class HierarchicalController extends Hierarchical {
  constructor() {
    super();
  }
  enter_do(msg) {
    this.enters.push(msg);
  }
  exit_do(msg) {
    this.exits.push(msg);
  }
  log_do(msg) {
    this.tape.push(msg);
  }
}

describe("Hierarchical", () => {
  /// Test that a continue (`:>`) in a child enter handler calls the parent enter handler.
  it("Enter continue", () => {
    let sm = new HierarchicalController();
    sm.enters = [];
    sm.A();
    assert.deepStrictEqual(sm.enters, ["S0", "S"]);
    sm.enters = [];
    sm.C();
    assert.deepStrictEqual(sm.enters, ["S2", "S0", "S"]);
  });

  /// Test that a continue (`:>`) in a child exit handler calls the parent exit handler.
  it("Exit continue", () => {
    let sm = new HierarchicalController();
    sm.A();
    sm.exits = [];
    sm.C();
    assert.deepStrictEqual(sm.exits, ["S0", "S"]);
    sm.exits = [];
    sm.A();
    assert.deepStrictEqual(sm.exits, ["S2", "S0", "S"]);
  });

  /// Test that a return (`^`) in a child enter handler *does not* call the parent enter handler.
  it("Enter return", () => {
    let sm = new HierarchicalController();
    sm.enters = [];
    sm.B();
    assert.deepStrictEqual(sm.enters, ["S1"]);
    sm = new HierarchicalController();
    sm.A();
    sm.A();
    assert.deepStrictEqual(sm.state_info(), returnStateName("T"));
    sm.enters = [];
    sm.C();
    assert.deepStrictEqual(sm.enters, ["S3", "S1"]);
  });

  /// Test that a return (`^`) in a child exit handler *does not* call the parent exit handler.
  it("Exit return", () => {
    let sm = new HierarchicalController();
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    sm.exits = [];
    sm.A();
    assert.deepStrictEqual(sm.exits, ["S1"]);
    sm = new HierarchicalController();
    sm.A();
    sm.A();
    sm.C();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S3"));
    sm.exits = [];
    sm.B();
    assert.deepStrictEqual(sm.exits, ["S3", "S1"]);
  });

  /// Test that location in a hierarchical state is represented correctly. In this test, all
  /// state transitions are performed by the immediately matching handler.
  it("Current state simple", () => {
    let sm = new HierarchicalController();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S"));
    sm.A();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S0"));
    sm.A();
    assert.deepStrictEqual(sm.state_info(), returnStateName("T"));
    sm.C();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S3"));
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S2"));
  });

  /// Test that location in a hierarchical state is represented correctly. In this test, several
    /// state transitions propagate message handling to parents, either by implicit fall-through or
    /// explicit continues.
    it( "Current state with propagation",  ()=> {
        let sm = new HierarchicalController();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S"));
        sm.A();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S0"));
        sm.B();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
        sm.B();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
        sm.C();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
        sm.A();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S0"));
        sm.C();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S2"));
        sm.B();
        assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    })


  /// Test that a handler in a child overrides the parent handler if the child handler ends with
  /// a return.
  it("Overide parent handler", () => {
    let sm = new HierarchicalController();
    sm.A();
    sm.tape = [];
    sm.A();
    assert.deepStrictEqual(sm.state_info(), returnStateName("T"));
    assert.deepStrictEqual(sm.tape, ["S0.A"]);

    sm.C();
    sm.tape = [];
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S2"));
    assert.deepStrictEqual(sm.tape, ["S3.B"]);
  });

  /// Test that a handler in a child propagates control to the parent handler if the child
  /// handler ends with a continue.
  it("Before parent handler", () => {
    let sm = new HierarchicalController();
    sm.A();
    sm.tape = [];
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    assert.deepStrictEqual(sm.tape, ["S0.B", "S.B"]);
    sm.tape = [];
    sm.exits = [];
    sm.enters = [];
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    assert.deepStrictEqual(sm.tape, ["S1.B", "S.B"]);
    assert.deepStrictEqual(sm.exits, ["S1"]);
    assert.deepStrictEqual(sm.enters, ["S1"]);

    sm = new HierarchicalController();
    sm.A();
    sm.C();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S2"));
    sm.tape = [];
    sm.exits = [];
    sm.enters = [];
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    assert.deepStrictEqual(sm.tape, ["S2.B", "S0.B", "S.B"]);
    assert.deepStrictEqual(sm.exits, ["S2", "S0", "S"]);
    assert.deepStrictEqual(sm.enters, ["S1"]);
  });

  /// Test that missing event handlers in children automatically propagate to parents.
  it("Defer to parent handler", () => {
    let sm = new HierarchicalController();
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    sm.tape = [];
    sm.A();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S0"));
    assert.deepStrictEqual(sm.tape, ["S.A"]);
    sm.A();
    sm.C();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S3"));
    sm.tape = [];
    sm.A();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S0"));
    assert.deepStrictEqual(sm.tape, ["S.A"]);
  });

  /// Test that propagating control to a parent handler that doesn't handle the current message
  /// is a no-op.
  it("Before missing handler", () => {
    let sm = new HierarchicalController();
    sm.B();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    sm.tape = [];
    sm.exits = [];
    sm.enters = [];
    sm.C();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S1"));
    assert.deepStrictEqual(sm.tape, ["S1.C"]);
    assert(sm.exits.length === 0);
    assert(sm.enters.length === 0);
  });

  /// Test that a continue after a transition statement is ignored.
  it("Continue after transition ignored", () => {
    let sm = new HierarchicalController();
    sm.A();
    sm.C();
    assert.deepStrictEqual(sm.state_info(), returnStateName("S2"));
    sm.tape = [];
    sm.enters = [];
    sm.C();
    assert.deepStrictEqual(sm.state_info(), returnStateName("T"));
    assert.deepStrictEqual(sm.enters, ["T"]);
    assert.deepStrictEqual(sm.tape, ["S2.C"]);
  });
});

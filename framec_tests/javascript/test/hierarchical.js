const { describe, it } = require("mocha");
const assert = require("assert");

const Hierarchical = require("../output/hierarchical");

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
    sm.enters = [];
    sm.C();
    assert.deepStrictEqual(sm.enters, ["S3", "S1"]);
  });

  /// Test that a return (`^`) in a child exit handler *does not* call the parent exit handler.
  it("Exit return", () => {
    let sm = new HierarchicalController();
    sm.B();
    sm.exits = [];
    sm.A();
    assert.deepStrictEqual(sm.exits, ["S1"]);
    sm = new HierarchicalController();
    sm.A();
    sm.A();
    sm.C();
    sm.exits = [];
    sm.B();
    assert.deepStrictEqual(sm.exits, ["S3", "S1"]);
  });

  /// Test that a handler in a child overrides the parent handler if the child handler ends with
  /// a return.
  it("Overide parent handler", () => {
    let sm = new HierarchicalController();
    sm.A();
    sm.tape = [];
    sm.A();
    assert.deepStrictEqual(sm.tape, ["S0.A"]);

    sm.C();
    sm.tape = [];
    sm.B();

    assert.deepStrictEqual(sm.tape, ["S3.B"]);
  });

  /// Test that a handler in a child propagates control to the parent handler if the child
  /// handler ends with a continue.
  it("Before parent handler", () => {
    let sm = new HierarchicalController();
    sm.A();
    sm.tape = [];
    sm.B();
    assert.deepStrictEqual(sm.tape, ["S0.B", "S.B"]);
    sm.tape = [];
    sm.exits = [];
    sm.enters = [];
    sm.B();
    assert.deepStrictEqual(sm.tape, ["S1.B", "S.B"]);
    assert.deepStrictEqual(sm.exits, ["S1"]);
    assert.deepStrictEqual(sm.enters, ["S1"]);

    sm = new HierarchicalController();
    sm.A();
    sm.C();
    sm.tape = [];
    sm.exits = [];
    sm.enters = [];
    sm.B();
    assert.deepStrictEqual(sm.tape, ["S2.B", "S0.B", "S.B"]);
    assert.deepStrictEqual(sm.exits, ["S2", "S0", "S"]);
    assert.deepStrictEqual(sm.enters, ["S1"]);
  });

  /// Test that missing event handlers in children automatically propagate to parents.
  it("Defer to parent handler", () => {
    let sm = new HierarchicalController();
    sm.B();
    sm.tape = [];
    sm.A();
    assert.deepStrictEqual(sm.tape, ["S.A"]);
    sm.A();
    sm.C();
    sm.tape = [];
    sm.A();
    assert.deepStrictEqual(sm.tape, ["S.A"]);
  });

  /// Test that propagating control to a parent handler that doesn't handle the current message
  /// is a no-op.
  it("Before missing handler", () => {
    let sm = new HierarchicalController();
    sm.B();
    sm.tape = [];
    sm.exits = [];
    sm.enters = [];
    sm.C();
    assert.deepStrictEqual(sm.tape, ["S1.C"]);
    assert(sm.exits.length === 0);
    assert(sm.enters.length === 0);
  });

  /// Test that a continue after a transition statement is ignored.
  it("Continue after transition ignored", () => {
    let sm = new HierarchicalController();
    sm.A();
    sm.C();
    sm.tape = [];
    sm.enters = [];
    sm.C();
    assert.deepStrictEqual(sm.enters, ["T"]);
    assert.deepStrictEqual(sm.tape, ["S2.C"]);
  });
});

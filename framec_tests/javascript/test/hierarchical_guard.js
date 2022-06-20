const { describe, it } = require("mocha");
const assert = require("assert");
const HierarchicalGuard = require("../output/hierarchical_guard");

class HierarchicalGuardController extends HierarchicalGuard {
  constructor() {
    super();
  }
  log_do(msg) {
    this.tape.push(msg);
  }
}

describe("Hierarchical Guard", () => {
  it("Propagate to parent", () => {
    let sm = new HierarchicalGuardController();

    sm.A(0);
    sm.tape = [];
    sm.A(20);
    console.log();
    assert.deepStrictEqual(sm.tape, ["S0.A"]);

    sm = new HierarchicalGuardController();
    sm.A(0);
    sm.tape = [];
    sm.A(-5);
    assert.deepStrictEqual(sm.tape, ["S0.A", "S.A"]);

    sm = new HierarchicalGuardController();
    sm.A(0);
    sm.tape = [];
    sm.B(-5);
    assert.deepStrictEqual(sm.tape, ["S0.B"]);

    sm = new HierarchicalGuardController();
    sm.A(0);
    sm.tape = [];
    sm.B(5);
    assert.deepStrictEqual(sm.tape, ["S0.B", "S.B"]);
  });

  it("Propagate multiple levels", () => {
    let sm = new HierarchicalGuardController();
    sm.B(0);
    sm.tape = [];
    sm.A(7);
    assert.deepStrictEqual(sm.tape, ["S2.A", "S1.A"]);

    sm = new HierarchicalGuardController();
    sm.B(0);
    sm.tape = [];
    sm.A(-5);

    assert.deepStrictEqual(sm.tape, ["S2.A", "S1.A", "S0.A", "S.A"]);
  });

  it("Propagate skips levels", () => {
    let sm = new HierarchicalGuardController();
    sm.B(0);
    sm.tape = [];
    sm.B(-5);
    assert.deepStrictEqual(sm.tape, ["S2.B", "S0.B"]);

    sm = new HierarchicalGuardController();
    sm.B(0);
    sm.tape = [];
    sm.B(5);
    assert.deepStrictEqual(sm.tape, ["S2.B", "S0.B", "S.B"]);
  });

  it("Conditional return", () => {
    let sm = new HierarchicalGuardController();
    sm.B(20);
    sm.tape = [];
    sm.A(5);
    assert.deepStrictEqual(sm.tape, ["S3.A", "stop"]);

    sm = new HierarchicalGuardController();
    sm.B(20);
    sm.tape = [];
    sm.A(-5);
    assert.deepStrictEqual(sm.tape, ["S3.A", "continue", "S.A"]);

    sm = new HierarchicalGuardController();
    sm.B(20);
    sm.tape = [];
    sm.B(-5);
    assert.deepStrictEqual(sm.tape, ["S3.B", "stop"]);

    sm = new HierarchicalGuardController();
    sm.B(20);
    sm.tape = [];
    sm.B(5);
    assert.deepStrictEqual(sm.tape, ["S3.B", "continue", "S.B"]);
  });
});

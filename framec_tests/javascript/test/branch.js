const { describe, it } = require("mocha");
const assert = require("assert");

const Branch = require("../output/branch");

class BranchController extends Branch {
  constructor() {
    super();
  }
  log_do(msg) {
    this.tape.push(msg);
  }
}

describe("Branch", () => {
  it("Simple if bool", () => {
    let sm = new BranchController();
    sm.A();
    sm.OnBool(true);

    assert.deepStrictEqual(sm.tape, ["then 1", "then 2"]);

    sm = new BranchController();
    sm.A();
    sm.OnBool(false);

    assert.deepStrictEqual(sm.tape, ["else 1", "else 2"]);
  });

  it("Simple if int", () => {
    let sm = new BranchController();

    sm.A();
    sm.OnInt(7);

    assert.deepStrictEqual(sm.tape, ["> 5", "< 10", "== 7"]);

    sm = new BranchController();

    sm.A();
    sm.OnInt(-3);

    assert.deepStrictEqual(sm.tape, ["<= 5", "< 10", "!= 7"]);

    sm = new BranchController();

    sm.A();
    sm.OnInt(12);

    assert.deepStrictEqual(sm.tape, ["> 5", ">= 10", "!= 7"]);
  });

  it("Negated if boo", () => {
    let sm = new BranchController();

    sm.B();
    sm.OnBool(true);

    assert.deepStrictEqual(sm.tape, ["else 1", "else 2"]);

    sm = new BranchController();

    sm.B();
    sm.OnBool(false);

    assert.deepStrictEqual(sm.tape, ["then 1", "then 2"]);
  });

  it("Negated if int", () => {
    let sm = new BranchController();

    sm.B();
    sm.OnInt(7);

    assert.deepStrictEqual(sm.tape, [">= 5", "<= 10", "== 7"]);

    sm = new BranchController();

    sm.B();
    sm.OnInt(5);

    assert.deepStrictEqual(sm.tape, [">= 5", "<= 10", "!= 7"]);

    sm = new BranchController();

    sm.B();
    sm.OnInt(10);

    assert.deepStrictEqual(sm.tape, [">= 5", "<= 10", "!= 7"]);

    sm = new BranchController();

    sm.B();
    sm.OnInt(0);

    assert.deepStrictEqual(sm.tape, ["< 5", "<= 10", "!= 7"]);

    sm = new BranchController();

    sm.B();
    sm.OnInt(100);

    assert.deepStrictEqual(sm.tape, [">= 5", "> 10", "!= 7"]);
  });

  it("Operator precedence", () => {
    let sm = new BranchController();

    sm.C();
    sm.OnInt(0);
    assert.deepStrictEqual(sm.tape, ["then 1", "else 2", "then 3", "then 4"]);
    sm.tape = [];
    sm.OnInt(7);
    assert.deepStrictEqual(sm.tape, ["else 1", "then 2", "else 3", "then 4"]);
    sm.tape = [];
    sm.OnInt(-3);
    assert.deepStrictEqual(sm.tape, ["then 1", "else 2", "else 3", "else 4"]);
    sm.tape = [];
    sm.OnInt(12);
    assert.deepStrictEqual(sm.tape, ["else 1", "else 2", "then 3", "else 4"]);
  });

  it("Nested if", () => {
    let sm = new BranchController();

    sm.D();
    sm.OnInt(50);
    assert.deepStrictEqual(sm.tape, ["> 0", "< 100"]);

    sm = new BranchController();
    sm.D();
    sm.OnInt(200);
    assert.deepStrictEqual(sm.tape, ["> 0", ">= 100"]);

    sm = new BranchController();
    sm.D();
    sm.OnInt(-5);
    assert.deepStrictEqual(sm.tape, ["<= 0", "> -10"]);

    sm = new BranchController();
    sm.D();
    sm.OnInt(-10);
    assert.deepStrictEqual(sm.tape, ["<= 0", "<= -10"]);
  });

  it("Guarded transition", () => {
    let sm = new BranchController();
    sm.E();
    sm.OnInt(5);
    assert.deepStrictEqual(sm.tape, ["-> $F3"]);

    sm = new BranchController();
    sm.E();
    sm.OnInt(15);
    assert.deepStrictEqual(sm.tape, ["-> $F2"]);

    sm = new BranchController();
    sm.E();
    sm.OnInt(115);
    assert.deepStrictEqual(sm.tape, ["-> $F1"]);
  });

  it("Nested guarded transition", () => {
    let sm = new BranchController();
    sm.F();
    sm.OnInt(5);
    assert.deepStrictEqual(sm.tape, ["-> $F3"]);
    sm = new BranchController();
    sm.F();
    sm.OnInt(15);
    assert.deepStrictEqual(sm.tape, ["-> $F2"]);
    sm = new BranchController();
    sm.F();
    sm.OnInt(65);
    assert.deepStrictEqual(sm.tape, ["-> $F3"]);
    sm = new BranchController();
    sm.F();
    sm.OnInt(115);
    assert.deepStrictEqual(sm.tape, ["-> $F1"]);
  });
});

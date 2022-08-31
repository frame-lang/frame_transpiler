const { describe, it } = require("mocha");
const assert = require("assert");
const StateContextStack = require("./state_context_stack");
const returnStateName = require("../utils/state_info/returnStateName");

class StateContextStackController extends StateContextStack {
  constructor() {
    super();
  }
  log_do(msg) {
    this.tape.push(msg);
  }
}

describe("State context stack", () => {
  // Test that a pop restores a pushed state.
  it("Push pop", () => {
    let sm = new StateContextStackController();
    assert.equal(sm.state_info(), returnStateName("A"));
    sm.push()
    sm.to_b()
    assert.equal(sm.state_info(), returnStateName("B"))
    sm.pop()
    assert.equal(sm.state_info(), returnStateName("A"))
  });

  // Test that multiple states can be pushed and subsequently restored by pops, LIFO style.
  it("Mulitple push pop", () => {
    let sm = new StateContextStackController();
    assert.equal(sm.state_info(), returnStateName("A"));
    sm.push();
    sm.to_c();
    sm.push();
    sm.to_a();
    sm.push();
    sm.push();
    sm.to_c(); // no push
    sm.to_b();
    sm.push();
    sm.to_c();
    sm.push(); // stack top-to-bottom: C, B, A, A, C, A
    sm.to_a();
    assert.equal(sm.state_info(), returnStateName("A"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("C"));
    sm.to_a();
    assert.equal(sm.state_info(), returnStateName("A"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("B"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("A"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("A"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("C"));
    sm.to_b();
    sm.push();
    sm.to_c();
    sm.push(); // stack top-to-bottom: C, B, A
    sm.to_a();
    sm.to_b();
    assert.equal(sm.state_info(), returnStateName("B"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("C"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("B"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("A"));
  });

  // Test that pop transitions trigger enter/exit events.
  it("Pop transition events", () => {
    let sm = new StateContextStackController();
    sm.to_b();
    sm.push();
    sm.to_a();
    sm.push();
    sm.to_c();
    sm.push(); // stack top-to-bottom: C, A, B
    sm.to_a();
    sm.tape = [];
    assert.equal(sm.state_info(), returnStateName("A"));
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("C"));
    assert.deepStrictEqual(sm.tape, ["A:<", "C:>"]);
    sm.tape = [];
    sm.pop();
    sm.pop();
    assert.equal(sm.state_info(), returnStateName("B"));
    assert.deepStrictEqual(sm.tape, ["C:<", "A:>", "A:<", "B:>"]);
  });

  // Test that pop change-states do not trigger enter/exit events.
  it("Pop change state no events", () => {
    let sm = new StateContextStackController();
    sm.to_b()
    sm.push()
    sm.to_a()
    sm.push()
    sm.to_c()
    sm.push() // stack top-to-bottom: C, A, B
    sm.to_a()
    sm.tape = []
    assert.equal(sm.state_info(), returnStateName("A"))
    sm.pop_change()
    assert.equal(sm.state_info(), returnStateName("C"))
    assert.equal(sm.tape.length, 0)
    sm.pop()
    sm.pop_change()
    assert.equal(sm.state_info(), returnStateName("B"))
    assert.deepStrictEqual(sm.tape,["C:<", "A:>"])
  });

  //  Test that state variables are restored after pop.
  it("Pop restores state variables", () => {
    let sm = new StateContextStackController();
    sm.inc()
    sm.inc()
    sm.push()
    assert.equal(sm.state_info(), returnStateName("A"))
    assert.equal(sm.value(), 2)
    sm.to_b()
    sm.inc()
    sm.push()
    assert.equal(sm.state_info(), returnStateName("B"))
    assert.equal(sm.value(), 5)
    sm.to_c()
    sm.inc()
    sm.inc()
    sm.inc()
    sm.push()
    assert.equal(sm.state_info(), returnStateName("C"))
    assert.equal(sm.value(), 30)
    sm.to_a()
    sm.inc()
    assert.equal(sm.state_info(), returnStateName("A"))
    assert.equal(sm.value(), 1)
    sm.pop()
    assert.equal(sm.state_info(), returnStateName("C"))
    assert.equal(sm.value(), 30)
    sm.pop()
    assert.equal(sm.state_info(), returnStateName("B"))
    assert.equal(sm.value(),5)
    sm.to_a()
    sm.inc()
    sm.inc()
    sm.inc()
    sm.push()
    assert.equal(sm.state_info(), returnStateName("A"))
    assert.equal(sm.value(),3)
    sm.to_c()
    sm.inc()
    assert.equal(sm.state_info(), returnStateName("C"))
    assert.equal(sm.value(),10)
    sm.pop()
    assert.equal(sm.state_info(), returnStateName("A"))
    assert.equal(sm.value(),3)
    sm.pop()
    assert.equal(sm.state_info(), returnStateName("A"))
    assert.equal(sm.value(),2)
  });

  it("Push stores state variable snapshot", () => {
    let sm = new StateContextStackController();
    sm.inc();
    sm.inc();
    sm.push();
    assert.equal(sm.state_info(), returnStateName("A"));
    assert.equal(sm.value(), 2);
    sm.inc();
    sm.inc();
    assert.equal(sm.value(), 4);

    sm.to_b();
    sm.inc();
    sm.push();
    assert.equal(sm.state_info(), returnStateName("B"));
    assert.equal(sm.value(), 5);
    sm.inc();
    sm.inc();
    assert.equal(sm.value(), 15); // // these changes should be forgotten

    sm.to_c();
    sm.inc();
    sm.inc();
    sm.inc();
    sm.push();
    sm.state_info();
    assert.equal(sm.state_info(), returnStateName("C"));
    assert.equal(sm.value(), 30);
    sm.inc();
    assert.equal(sm.value(), 40); // forgotten

    sm.to_a();
    sm.inc();
    assert.equal(sm.state_info(), returnStateName("A"));
    assert.equal(sm.value(), 1);

    sm.pop();
    assert.equal(sm.state_info(), returnStateName("C"));
    assert.equal(sm.value(), 30);

    sm.pop();
    assert.equal(sm.state_info(), returnStateName("B"));
    assert.equal(sm.value(), 5);

    sm.to_a();
    sm.inc();
    sm.inc();
    sm.inc();
    sm.push();
    assert.equal(sm.state_info(), returnStateName("A"));
    assert.equal(sm.value(), 3);
    sm.inc();
    assert.equal(sm.value(), 4); //forgotten

    sm.to_c();
    sm.inc();
    assert.equal(sm.state_info(), returnStateName("C"));
    assert.equal(sm.value(), 10);

    sm.pop();
    assert.equal(sm.state_info(), returnStateName("A"));
    assert.equal(sm.value(), 3);

    sm.pop();
    assert.equal(sm.state_info(), returnStateName("A"));
    assert.equal(sm.value(), 2);
  });
});

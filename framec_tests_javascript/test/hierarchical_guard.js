const { describe, it } = require("mocha");
const assert = require("assert");
const HierarchicalGuard = require("../output/hierarchical_guard");

class HierarchicalGuardController extends HierarchicalGuard {

	constructor() {
	  super()
	}
	log_do(msg) {
        this.tape.push(msg)
    }
};


describe("Hierarchical Guard", ()=>{

    it("Propagate to parent", ()=>{
        let sm = new HierarchicalGuardController()

        sm.A(0)
        sm.tape = []
        sm.A(20)
        console.log()
        assert.deepStrictEqual(sm.tape, ["S0.A"])

        sm = new HierarchicalGuardController()
        sm.A(0)
        sm.tape = []
        sm.A(-5)
        assert.deepStrictEqual(sm.tape, ["S0.A", "S.A"])

        sm = new HierarchicalGuardController()
        sm.A(0)
        sm.tape = []
        sm.B(-5)
        assert.deepStrictEqual(sm.tape, ["S0.B"])

        sm = new HierarchicalGuardController()
        sm.A(0)
        sm.tape = []
        sm.B(5)
        assert.deepStrictEqual(sm.tape, ["S0.B", "S.B"])

    })

    it(propagate_multiple_levels)
})

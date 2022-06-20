const { describe, it } = require("mocha");
const assert = require("assert");
const StateContextSm = require("../output/state_context_runtime");

class StateContextSmController extends StateContextSm {

	constructor() {
	  super()
	}

	log_do(name,val) {
        this.tape.push(`${name}=${val}`)
    }
};


describe("State context", ()=>{

    it("Initial state", ()=>{
        let sm = new StateContextSmController();
        let r = sm.Inc();
        assert(r, 4);
        sm.LogState();
        assert.deepStrictEqual(sm.tape, ["w=3", "w=4", "w=4"])
    })

    it( "Transition", () =>  {
        let sm = new StateContextSmController();
        sm.Inc();
        sm.Inc();
        sm.tape = [];

        sm.Start();
        assert.deepStrictEqual(sm.tape, ["a=3", "b=5", "x=15"]);
        sm.tape = [];

        sm.Inc();
        let r = sm.Inc();
        assert(r, 17);
        assert.deepStrictEqual(sm.tape, ["x=16", "x=17"]);
        sm.tape = [];

        sm.Next(3);
        assert.deepStrictEqual(sm.tape, ["c=10", "x=27", "a=30", "y=17", "z=47"]);
        sm.tape = [];

        sm.Inc();
        sm.Inc();
        r = sm.Inc();
        assert(r, 50);
        assert.deepStrictEqual(sm.tape, ["z=48", "z=49", "z=50"]);
    })

    it("Change state", ()=>{
        let sm = new StateContextSmController()
        sm.Inc();
        sm.Inc();
        sm.Start();
        sm.tape = [];

        sm.Inc();
        assert.deepStrictEqual(sm.tape, ["x=16"]);
        sm.tape = [];

        sm.Change(10);
        sm.LogState();
        assert.deepStrictEqual(sm.tape, ["y=26", "z=0"]);
        sm.tape = [];

        sm.Inc();
        sm.Change(100);
        sm.LogState();
        assert.deepStrictEqual(sm.tape, ["z=1", "tmp=127", "w=0"]);
    })
})
const { describe, it } = require("mocha");
const assert = require("assert");
const StateVars = require("./state_vars");
const returnStateName = require("../utils/state_info/returnStateName")

class StateVarsController extends StateVars {

    constructor() {
        super()
    }
}

describe("State Vars", ()=>{

    it("Single variable", ()=>{
        let sm = new StateVarsController()
        assert.deepStrictEqual(sm.state_info(), returnStateName("A"))
        assert.deepStrictEqual(sm.compartment_info().StateVars['x'], 0)
        sm.X() // increment x
        sm.X() // increment x
        assert.deepStrictEqual(sm.compartment_info().StateVars['x'], 2)
    })

    it("Multiple variable", ()=>{
        let sm = new StateVarsController()
        sm.Y() // transition to B
        assert.deepStrictEqual( sm.state_info() , returnStateName("B"))
        assert.deepStrictEqual( sm.compartment_info().StateVars['y'] ,10)
        assert.deepStrictEqual( sm.compartment_info().StateVars['z'] ,100)
        sm.Y(); // increment y
        sm.Y(); // increment y
        sm.Z(); // increment z
        sm.Y(); // increment y
        assert.deepStrictEqual( sm.compartment_info().StateVars['y'] ,13)
        assert.deepStrictEqual( sm.compartment_info().StateVars['z'] ,101)
    })

    it("Variables are reset", ()=>{
        let sm = new StateVarsController()
        sm.X() // increment x
        sm.X(); // increment x
        assert.deepStrictEqual(sm.compartment_info().StateVars['x'], 2)
        sm.Z(); // transition to B
        sm.Z(); // increment z
        sm.Y(); // increment y
        sm.Z(); // increment z
        assert.deepStrictEqual(sm.compartment_info().StateVars['y'], 11)
        assert.deepStrictEqual(sm.compartment_info().StateVars['z'], 102)
        sm.X() // transition to A
        assert.deepStrictEqual(sm.compartment_info().StateVars['x'], 0)
        sm.Y() // transition to B
        assert.deepStrictEqual(sm.compartment_info().StateVars['y'], 10)
        assert.deepStrictEqual(sm.compartment_info().StateVars['z'], 100)
    })


})
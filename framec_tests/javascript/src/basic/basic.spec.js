const Basic = require('./basic')

const assert = require('assert');
const { it , describe} = require('mocha');

class BasicController extends Basic {
  constructor(){
    super()
  }

  entered_do(msg) {
    this.entry_log.push(msg)
  }

  left_do(msg) {
    this.exit_log.push(msg)}
}

describe("Basic",  function () {

   it("Intial enter event", ()=>{
    let sm = new BasicController()
    assert.deepStrictEqual(sm.entry_log, ["S0"] , "Enter event is sent for entering the initial state on startup")
   })

   it("Transition enter events", ()=>{
    let sm = new BasicController()
    sm.entry_log = []
    sm.A()
    sm.B()
    assert.deepStrictEqual(sm.entry_log, ["S1", "S0"], "Enter events are sent to the new state on transition")
   })

   it("Transition exit events", ()=>{
    let sm = new BasicController()
    sm.A()
    sm.B()
    assert.deepStrictEqual(sm.exit_log, ["S0", "S1"], "Exit events are sent to the old state on transition")
   })

   it("Current state", ()=> {
     let sm = new BasicController()
     assert(sm.state, '#sS0_')
     sm.A()
     assert(sm.state, '#sS1_')
     sm.B()
     assert(sm.state, '#sS0_')
   })
    
  })
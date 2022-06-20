const { describe, it } = require("mocha");
const assert = require("assert");

const EventHandler = require("../output/event_handler");

class EventHandlerController extends EventHandler {

	constructor() {
	  super()
	}
	log_do(msg, val) {
		this.tape.push(`${msg}=${val}`)
	}
};

describe("Event handler", ()=>{

	it("Single parameter", () => {
		let sm = new EventHandlerController();
		sm.LogIt(2);
		assert.deepStrictEqual(sm.tape, ["x=2"])

	})

	it("Compute two parameters", ()=>{
		let sm = new EventHandlerController()
		sm.LogAdd(-3, 10)
		assert.deepStrictEqual(sm.tape, ["a=-3", "b=10", "a+b=7"]);
	})
	
	it("Return local variable", ()=> {
		let sm = new EventHandlerController()
		let ret = sm.LogReturn(13, 21);
		assert.deepStrictEqual(sm.tape, ["a=13", "b=21", "r=34"]);
		assert.deepStrictEqual(ret, 34);
	})

	it("Pass result", ()=>{
		let sm = new EventHandlerController()
		sm.PassAdd(5, -12);
		assert.deepStrictEqual(sm.tape, ["p=-7"])
	})

	it("Pass and return result", ()=> {
		let sm = new EventHandlerController()
        let ret = sm.PassReturn(101, -59);
        assert.deepStrictEqual(sm.tape, ["r=42", "p=42"]);
        assert.deepStrictEqual(ret, 42);
	})
})
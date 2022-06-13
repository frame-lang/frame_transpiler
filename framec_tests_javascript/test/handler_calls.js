const { describe, it } = require("mocha");
const assert = require("assert");
const HandlerCalls = require("../output/handler_calls");

class HandlerCallsConstroller extends HandlerCalls {

    constructor(){
        super()
    }

    log_do(from, val) {
        this.tape.push(`${from}(${val})`)
    }
}


describe("Handler calls", ()=>{

    it("Calls terminate handler", ()=> {
        let sm = new HandlerCallsConstroller()
        sm.NonRec()
        sm.Foo(10);
        assert.strictEqual(sm.tape.includes("Unreachable(0)"), false, "Handler calls unreachable statement")
    })

    it("Non Recursive", ()=> {
        let sm = new HandlerCallsConstroller()
        sm.NonRec()
        sm.Foo(10);
        assert.deepStrictEqual(sm.tape, ["Foo(10)", "Bar(20)", "Final(30)"]);
    })

    it("Self Recursive", ()=> {
        let sm = new HandlerCallsConstroller()
        sm.SelfRec()
        sm.Foo(10);
        assert.deepStrictEqual(sm.tape, ["Foo(10)", "Foo(20)", "Foo(40)", "Foo(80)", "Final(150)"]);
    })

    it("Mutually Recursive", ()=> {
        let sm = new HandlerCallsConstroller()
        sm.MutRec()
        sm.Foo(2);
        assert.deepStrictEqual(sm.tape,[
            "Foo(2)",
            "Bar(4)",
            "Foo(4)",
            "Bar(8)",
            "Foo(16)",
            "Bar(32)",
            "Foo(96)",
            "Final(162)"
        ]);
    })

    it("String match calls", ()=> {
        let sm = new HandlerCallsConstroller()
        sm.NonRec()
        sm.Call("Foo", 5);
        assert.deepStrictEqual(sm.tape,["Foo(5)", "Bar(10)", "Final(15)"]);
        sm.tape = []

        sm.NonRec()
        sm.Call("Bar", 20)
        assert.deepStrictEqual(sm.tape, ["Bar(20)", "Final(20)"])
        sm.tape = []
        
        sm.NonRec()
        sm.Call("Qux", 37)
        assert.deepStrictEqual(sm.tape, ["Foo(1000)", "Bar(2000)", "Final(3000)"])

    })
})
// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class HandlerCalls {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new HandlerCallsCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    NonRec() {
        let e = FrameEvent("NonRec",null);
        this.#mux(e);
    }
    
    SelfRec() {
        let e = FrameEvent("SelfRec",null);
        this.#mux(e);
    }
    
    MutRec() {
        let e = FrameEvent("MutRec",null);
        this.#mux(e);
    }
    
    Call(event,arg) {
        let e = FrameEvent("Call",{"event":event,"arg":arg});
        this.#mux(e);
    }
    
    Foo(arg) {
        let e = FrameEvent("Foo",{"arg":arg});
        this.#mux(e);
    }
    
    Bar(arg) {
        let e = FrameEvent("Bar",{"arg":arg});
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sInit_:
                this.#sInit_(e);
                break;
            case this.#sNonRecursive_:
                this.#sNonRecursive_(e);
                break;
            case this.#sSelfRecursive_:
                this.#sSelfRecursive_(e);
                break;
            case this.#sMutuallyRecursive_:
                this.#sMutuallyRecursive_(e);
                break;
            case this.#sFinal_:
                this.#sFinal_(e);
                break;
        }
        
        if( this.#nextCompartment != null) {
            let nextCompartment = this.#nextCompartment
            this.#nextCompartment = null
            if (nextCompartment._forwardEvent != null && 
               nextCompartment._forwardEvent._message == ">") {
                this.#mux(FrameEvent( "<", this.#compartment.ExitArgs))
                this.#compartment = nextCompartment
                this.#mux(nextCompartment._forwardEvent)
            } else {
                this.#doTransition(nextCompartment)
                if (nextCompartment._forwardEvent != null) {
                    this.#mux(nextCompartment._forwardEvent)
                }
            }
            nextCompartment._forwardEvent = null
        }
    }
    
    //===================== Machine Block ===================//
    
    #sInit_(e) {
        switch (e._message) {
            case "NonRec":
                {
                let compartment =  new HandlerCallsCompartment(this.#sNonRecursive_);
                
                compartment.StateVars["counter"] = this.#compartment.StateVars["counter"] + e._parameters["arg"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "SelfRec":
                {
                let compartment =  new HandlerCallsCompartment(this.#sSelfRecursive_);
                
                compartment.StateVars["counter"] = this.#compartment.StateVars["counter"] + e._parameters["arg"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "MutRec":
                {
                let compartment =  new HandlerCallsCompartment(this.#sMutuallyRecursive_);
                
                compartment.StateVars["counter"] = this.#compartment.StateVars["counter"] + e._parameters["arg"];
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sNonRecursive_(e) {
        switch (e._message) {
            case "Foo":
                {
                this.log_do("Foo",(e._parameters["arg"]));
                (this.#compartment.StateVars["counter"]) = (this.#compartment.StateVars["counter"]) + (e._parameters["arg"]);
                should;
                report;
                the;
                next;
                line;
                as;
                a;
                static;
                error;
                this.log_do("Unreachable",0);
                
                return;
                }
                
            case "Bar":
                {
                this.log_do("Bar",(e._parameters["arg"]));
                (this.#compartment.StateVars["counter"]) = (this.#compartment.StateVars["counter"]) + (e._parameters["arg"]);
                let compartment =  new HandlerCallsCompartment(this.#sFinal_);
                
                compartment.StateArgs["counter"] = this.#compartment.StateVars["counter"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Call":
                {
                if (((e._parameters["event"])) {
                    this.Foo((e._parameters["arg"]));
                    return;
                } else if (((e._parameters["event"])) {
                    this.Bar((e._parameters["arg"]));
                    return;
                } else {
                    this.Call("Foo",1000);
                    return;
                }
                
                return;
                }
                
        }
    }
    
    #sSelfRecursive_(e) {
        switch (e._message) {
            case "Foo":
                {
                this.log_do("Foo",(e._parameters["arg"]));
                (this.#compartment.StateVars["counter"]) = (this.#compartment.StateVars["counter"]) + (e._parameters["arg"]);
                if ((this.#compartment.StateVars["counter"]) < 100) {
                    this.Foo((e._parameters["arg"]) * 2);
                    return;
                } else {
                    let compartment =  new HandlerCallsCompartment(this.#sFinal_);
                    
                    compartment.StateArgs["counter"] = this.#compartment.StateVars["counter"];
                    
                    this.#transition(compartment);
                    return;
                }
                
                return;
                }
                
            case "Bar":
                {
                this.log_do("Bar",(e._parameters["arg"]));
                (this.#compartment.StateVars["counter"]) = (this.#compartment.StateVars["counter"]) + (e._parameters["arg"]);
                let compartment =  new HandlerCallsCompartment(this.#sFinal_);
                
                compartment.StateArgs["counter"] = this.#compartment.StateVars["counter"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Call":
                {
                if (((e._parameters["event"])) {
                    this.Foo((e._parameters["arg"]));
                    return;
                } else if (((e._parameters["event"])) {
                    this.Bar((e._parameters["arg"]));
                    return;
                } else {
                }
                
                return;
                }
                
        }
    }
    
    #sMutuallyRecursive_(e) {
        switch (e._message) {
            case "Foo":
                {
                this.log_do("Foo",(e._parameters["arg"]));
                (this.#compartment.StateVars["counter"]) = (this.#compartment.StateVars["counter"]) + (e._parameters["arg"]);
                if ((this.#compartment.StateVars["counter"]) > 100) {
                    let compartment =  new HandlerCallsCompartment(this.#sFinal_);
                    
                    compartment.StateArgs["counter"] = this.#compartment.StateVars["counter"];
                    
                    this.#transition(compartment);
                    return;
                } else {
                    this.Bar((e._parameters["arg"]) * 2);
                    return;
                }
                
                return;
                }
                
            case "Bar":
                {
                this.log_do("Bar",(e._parameters["arg"]));
                (this.#compartment.StateVars["counter"]) = (this.#compartment.StateVars["counter"]) + (e._parameters["arg"]);
                if (((e._parameters["arg"]) == 4)) {
                    this.Foo((e._parameters["arg"]));
                    return;
                } else if (((e._parameters["arg"]) == 8)) {
                    this.Foo((e._parameters["arg"]) * 2);
                    return;
                } else {
                    this.Foo((e._parameters["arg"]) * 3);
                    return;
                }
                
                return;
                }
                
            case "Call":
                {
                if (((e._parameters["event"])) {
                    this.Foo((e._parameters["arg"]));
                    return;
                } else if (((e._parameters["event"])) {
                    this.Bar((e._parameters["arg"]));
                    return;
                } else {
                }
                
                return;
                }
                
        }
    }
    
    #sFinal_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do("Final",(this.#compartment.StateArgs["counter"]));
                let compartment =  new HandlerCallsCompartment(this.#sInit_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    log_do(from,val) { throw new Error('Action not implemented.'); }
    
    //=============== Machinery and Mechanisms ==============//
    
    #transition(compartment) {
        this.#nextCompartment = compartment;
    }
    
    #doTransition(nextCompartment) {
        this.#mux(FrameEvent("<", this.#compartment.ExitArgs));
        this.#compartment = nextCompartment;
        this.#mux(FrameEvent(">", this.#compartment.EnterArgs));
    }
    
    state_info() {
        return this.#compartment.state.name;
    }
    
    
};

//=============== Compartment ==============//

class HandlerCallsCompartment {

    constructor(state) {
        this.state = state
    }
    
    StateArgs = {};
    StateVars = {};
    EnterArgs = {};
    ExitArgs = {};
    _forwardEvent = FrameEvent.call(this)
}


/********************

class HandlerCallsController extends HandlerCalls {

	constructor() {
	  super()
	}
	log_do(from,val) {}
};

********************/

module.exports = HandlerCalls

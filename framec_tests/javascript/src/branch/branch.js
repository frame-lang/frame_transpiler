// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class Branch {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sI_;
        this.#compartment = new BranchCompartment(this.#state);
        this.#nextCompartment = null;
        this.state = this.#compartment.state.name
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    A() {
        let e = FrameEvent("A",null);
        this.#mux(e);
    }
    
    B() {
        let e = FrameEvent("B",null);
        this.#mux(e);
    }
    
    C() {
        let e = FrameEvent("C",null);
        this.#mux(e);
    }
    
    D() {
        let e = FrameEvent("D",null);
        this.#mux(e);
    }
    
    E() {
        let e = FrameEvent("E",null);
        this.#mux(e);
    }
    
    F() {
        let e = FrameEvent("F",null);
        this.#mux(e);
    }
    
    OnBool(b) {
        let e = FrameEvent("OnBool",{"b":b});
        this.#mux(e);
    }
    
    OnInt(i) {
        let e = FrameEvent("OnInt",{"i":i});
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sI_:
                this.#sI_(e);
                break;
            case this.#sSimpleIf_:
                this.#sSimpleIf_(e);
                break;
            case this.#sNegatedIf_:
                this.#sNegatedIf_(e);
                break;
            case this.#sPrecedence_:
                this.#sPrecedence_(e);
                break;
            case this.#sNestedIf_:
                this.#sNestedIf_(e);
                break;
            case this.#sGuardedTransition_:
                this.#sGuardedTransition_(e);
                break;
            case this.#sNestedGuardedTransition_:
                this.#sNestedGuardedTransition_(e);
                break;
            case this.#sF1_:
                this.#sF1_(e);
                break;
            case this.#sF2_:
                this.#sF2_(e);
                break;
            case this.#sF3_:
                this.#sF3_(e);
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
    
    #sI_(e) {
        switch (e._message) {
            case "A":
                {
                let compartment =  new BranchCompartment(this.#sSimpleIf_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "B":
                {
                let compartment =  new BranchCompartment(this.#sNegatedIf_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "C":
                {
                let compartment =  new BranchCompartment(this.#sPrecedence_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "D":
                {
                let compartment =  new BranchCompartment(this.#sNestedIf_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "E":
                {
                let compartment =  new BranchCompartment(this.#sGuardedTransition_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "F":
                {
                let compartment =  new BranchCompartment(this.#sNestedGuardedTransition_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sSimpleIf_(e) {
        switch (e._message) {
            case "OnBool":
                {
                if ((e._parameters["b"])) {
                    this.log_do("then 1");
                } else {
                }
                if ((e._parameters["b"])) {
                } else {
                    this.log_do("else 1");
                }
                if ((e._parameters["b"])) {
                    this.log_do("then 2");
                } else {
                    this.log_do("else 2");
                }
                if ((e._parameters["b"])) {
                    let compartment =  new BranchCompartment(this.#sF1_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                    let compartment =  new BranchCompartment(this.#sF2_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                return;
                }
                
            case "OnInt":
                {
                if ((e._parameters["i"]) > 5) {
                    this.log_do("> 5");
                } else {
                    this.log_do("<= 5");
                }
                if ((e._parameters["i"]) < 10) {
                    this.log_do("< 10");
                } else {
                    this.log_do(">= 10");
                }
                if ((e._parameters["i"]) == 7) {
                    this.log_do("== 7");
                    let compartment =  new BranchCompartment(this.#sF1_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                    this.log_do("!= 7");
                    let compartment =  new BranchCompartment(this.#sF2_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                return;
                }
                
        }
    }
    
    #sNegatedIf_(e) {
        switch (e._message) {
            case "OnBool":
                {
                if (!((e._parameters["b"]))) {
                    this.log_do("then 1");
                } else {
                }
                if (!((e._parameters["b"]))) {
                } else {
                    this.log_do("else 1");
                }
                if (!((e._parameters["b"]))) {
                    this.log_do("then 2");
                } else {
                    this.log_do("else 2");
                }
                if (!((e._parameters["b"]))) {
                    let compartment =  new BranchCompartment(this.#sF1_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                    let compartment =  new BranchCompartment(this.#sF2_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                return;
                }
                
            case "OnInt":
                {
                if (!((e._parameters["i"]) >= 5)) {
                    this.log_do("< 5");
                } else {
                    this.log_do(">= 5");
                }
                if (!((e._parameters["i"]) <= 10)) {
                    this.log_do("> 10");
                } else {
                    this.log_do("<= 10");
                }
                if (!((e._parameters["i"]) != 7)) {
                    this.log_do("== 7");
                    let compartment =  new BranchCompartment(this.#sF1_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                    this.log_do("!= 7");
                    let compartment =  new BranchCompartment(this.#sF2_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                return;
                }
                
        }
    }
    
    #sPrecedence_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if (-(e._parameters["i"]) >= 0 && -(e._parameters["i"]) <= 5) {
                    this.log_do("then 1");
                } else {
                    this.log_do("else 1");
                }
                if (!((e._parameters["i"]) >= -5 && (e._parameters["i"]) <= 5) && ((e._parameters["i"]) >= -10 && (e._parameters["i"]) <= 10)) {
                    this.log_do("then 2");
                } else {
                    this.log_do("else 2");
                }
                if ((e._parameters["i"]) >= 0 && (e._parameters["i"]) <= 5 || (e._parameters["i"]) >= 10 && (e._parameters["i"]) <= 20) {
                    this.log_do("then 3");
                } else {
                    this.log_do("else 3");
                }
                if (!(((e._parameters["i"]) < 0 || (e._parameters["i"]) > 10) && (e._parameters["i"]) + 5 < 20)) {
                    this.log_do("then 4");
                } else {
                    this.log_do("else 4");
                }
                
                return;
                }
                
        }
    }
    
    #sNestedIf_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if ((e._parameters["i"]) > 0) {
                    this.log_do("> 0");
                    if ((e._parameters["i"]) < 100) {
                        this.log_do("< 100");
                        let compartment =  new BranchCompartment(this.#sF1_);
                        
                        
                        this.#transition(compartment);
                        return;
                    } else {
                        this.log_do(">= 100");
                    }
                } else {
                    this.log_do("<= 0");
                    if ((e._parameters["i"]) > -10) {
                        this.log_do("> -10");
                    } else {
                        this.log_do("<= -10");
                        let compartment =  new BranchCompartment(this.#sF2_);
                        
                        
                        this.#transition(compartment);
                        return;
                    }
                }
                
                return;
                }
                
        }
    }
    
    #sGuardedTransition_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if ((e._parameters["i"]) > 100) {
                    this.log_do("-> $F1");
                    let compartment =  new BranchCompartment(this.#sF1_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                }
                if (!((e._parameters["i"]) > 10)) {
                } else {
                    this.log_do("-> $F2");
                    let compartment =  new BranchCompartment(this.#sF2_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                this.log_do("-> $F3");
                let compartment =  new BranchCompartment(this.#sF3_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sNestedGuardedTransition_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if ((e._parameters["i"]) > 10) {
                    if ((e._parameters["i"]) > 100) {
                        this.log_do("-> $F1");
                        let compartment =  new BranchCompartment(this.#sF1_);
                        
                        
                        this.#transition(compartment);
                        return;
                    } else {
                    }
                    if ((e._parameters["i"]) > 50) {
                    } else {
                        this.log_do("-> $F2");
                        let compartment =  new BranchCompartment(this.#sF2_);
                        
                        
                        this.#transition(compartment);
                        return;
                    }
                } else {
                }
                this.log_do("-> $F3");
                let compartment =  new BranchCompartment(this.#sF3_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sF1_(e) {
        switch (e._message) {
        }
    }
    
    #sF2_(e) {
        switch (e._message) {
        }
    }
    
    #sF3_(e) {
        switch (e._message) {
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    log_do(msg) { throw new Error('Action not implemented.'); }
    
    //=============== Machinery and Mechanisms ==============//
    
    #transition(compartment) {
        this.#nextCompartment = compartment;
    }
    
    #doTransition(nextCompartment) {
        this.#mux(FrameEvent("<", this.#compartment.ExitArgs));
        this.#compartment = nextCompartment;
        this.#mux(FrameEvent(">", this.#compartment.EnterArgs));
    }
    
    
    
};

//=============== Compartment ==============//

class BranchCompartment {

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

class BranchController extends Branch {

	constructor() {
	  super()
	}
	log_do(msg) {}
};

********************/

module.exports = Branch

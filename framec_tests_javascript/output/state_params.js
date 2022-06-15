// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

const FrameEvent = require("../framelang/FrameEvent")

class StateParams {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new StateParamsCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.param_log = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    Next() {
        let e = FrameEvent("Next",null);
        this.#mux(e);
    }
    
    Prev() {
        let e = FrameEvent("Prev",null);
        this.#mux(e);
    }
    
    Log() {
        let e = FrameEvent("Log",null);
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sInit_:
                this.#sInit_(e);
                break;
            case this.#sSplit_:
                this.#sSplit_(e);
                break;
            case this.#sMerge_:
                this.#sMerge_(e);
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
            case "Next":
                {
                let compartment =  new StateParamsCompartment(this.#sSplit_);
                
                compartment.StateArgs["val"] = 1;
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sSplit_(e) {
        switch (e._message) {
            case "Next":
                {
                let compartment =  new StateParamsCompartment(this.#sMerge_);
                
                compartment.StateArgs["left"] = this.#compartment.StateArgs["val"];
                compartment.StateArgs["right"] = this.#compartment.StateArgs["val"] + 1;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Prev":
                {
                let compartment =  new StateParamsCompartment(this.#sMerge_);
                
                compartment.StateArgs["left"] = this.#compartment.StateArgs["val"] + 1;
                compartment.StateArgs["right"] = this.#compartment.StateArgs["val"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Log":
                {
                this.got_param_do("val",(this.#compartment.StateArgs["val"]));
                
                return;
                }
                
        }
    }
    
    #sMerge_(e) {
        switch (e._message) {
            case "Next":
                {
                let compartment =  new StateParamsCompartment(this.#sSplit_);
                
                compartment.StateArgs["val"] = this.#compartment.StateArgs["left"] + this.#compartment.StateArgs["right"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Prev":
                {
                let compartment =  new StateParamsCompartment(this.#sSplit_);
                
                compartment.StateArgs["val"] = this.#compartment.StateArgs["left"] * this.#compartment.StateArgs["right"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Log":
                {
                this.got_param_do("left",(this.#compartment.StateArgs["left"]));
                this.got_param_do("right",(this.#compartment.StateArgs["right"]));
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    got_param_do(name,val) { throw new Error('Action not implemented.'); }
    
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

class StateParamsCompartment {

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

class StateParamsController extends StateParams {

	constructor() {
	  super()
	}
	got_param_do(name,val) {}
};

********************/

module.exports = StateParams
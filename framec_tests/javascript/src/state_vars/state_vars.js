// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class StateVars {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new StateVarsCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    X() {
        let e = FrameEvent("X",null);
        this.#mux(e);
    }
    
    Y() {
        let e = FrameEvent("Y",null);
        this.#mux(e);
    }
    
    Z() {
        let e = FrameEvent("Z",null);
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sInit_:
                this.#sInit_(e);
                break;
            case this.#sA_:
                this.#sA_(e);
                break;
            case this.#sB_:
                this.#sB_(e);
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
            case ">":
                {
                let compartment =  new StateVarsCompartment(this.#sA_);
                
                compartment.StateVars["x"] = this.#compartment.StateVars["x"] + 1;
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sA_(e) {
        switch (e._message) {
            case "X":
                {
                (this.#compartment.StateVars["x"]) = (this.#compartment.StateVars["x"]) + 1;
                
                return;
                }
                
            case "Y":
                {
                let compartment =  new StateVarsCompartment(this.#sB_);
                
                compartment.StateVars["y"] = this.#compartment.StateVars["y"] + 1;
                compartment.StateVars["z"] = this.#compartment.StateVars["z"] + 1;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Z":
                {
                let compartment =  new StateVarsCompartment(this.#sB_);
                
                compartment.StateVars["y"] = this.#compartment.StateVars["y"] + 1;
                compartment.StateVars["z"] = this.#compartment.StateVars["z"] + 1;
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sB_(e) {
        switch (e._message) {
            case "X":
                {
                let compartment =  new StateVarsCompartment(this.#sA_);
                
                compartment.StateVars["x"] = this.#compartment.StateVars["x"] + 1;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Y":
                {
                (this.#compartment.StateVars["y"]) = (this.#compartment.StateVars["y"]) + 1;
                
                return;
                }
                
            case "Z":
                {
                (this.#compartment.StateVars["z"]) = (this.#compartment.StateVars["z"]) + 1;
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    
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
    compartment_info() {
        return this.#compartment
    }
    
    
};

//=============== Compartment ==============//

class StateVarsCompartment {

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

class StateVarsController extends StateVars {

	constructor() {
	  super()
	}
};

********************/

module.exports = StateVars

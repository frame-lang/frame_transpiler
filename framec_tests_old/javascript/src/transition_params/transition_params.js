// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class TransitParams {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new TransitParamsCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    Next() {
        let e = FrameEvent("Next",null);
        this.#mux(e);
    }
    
    Change() {
        let e = FrameEvent("Change",null);
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
            case "Next":
                {
                let compartment =  new TransitParamsCompartment(this.#sA_);
                
                compartment.EnterArgs["msg"] = "hi A";
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Change":
                {
                
                return;
                }
                
        }
    }  // ->> $A
	
    
    #sA_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do((e._parameters["msg"]));
                
                return;
                }
                
            case "<":
                {
                this.log_do("bye A");
                
                return;
                }
                
            case "Next":
                {
                let compartment =  new TransitParamsCompartment(this.#sB_);
                
                compartment.EnterArgs["msg"] = "hi B";
                compartment.EnterArgs["val"] = 42;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Change":
                {
                
                return;
                }
                
        }
    }  // ->> $B
	
    
    #sB_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do((e._parameters["msg"]));
                this.log_do((e._parameters["val"]).toString());
                
                return;
                }
                
            case "<":
                {
                this.log_do((e._parameters["val"]).toString());
                this.log_do((e._parameters["msg"]));
                
                return;
                }
                
            case "Next":
                {
                this.#compartment.ExitArgs["val"] = true;
                this.#compartment.ExitArgs["msg"] = "bye B";
                let compartment =  new TransitParamsCompartment(this.#sA_);
                
                compartment.EnterArgs["msg"] = "hi again A";
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Change":
                {
                
                return;
                }
                
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
    
    state_info() {
        return this.#compartment.state.name;
    }
    
      // ->> $A
	
    
};

//=============== Compartment ==============//

class TransitParamsCompartment {

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

class TransitParamsController extends TransitParams {

	constructor() {
	  super()
	}
	log_do(msg) {}
};

********************/

module.exports = TransitParams

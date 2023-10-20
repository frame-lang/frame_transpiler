// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class TransitionSm {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sS0_;
        this.#compartment = new TransitionSmCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.enters = [];
        this.exits = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    transit() {
        let e = FrameEvent("transit",null);
        this.#mux(e);
    }
    
    change() {
        let e = FrameEvent("change",null);
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sS0_:
                this.#sS0_(e);
                break;
            case this.#sS1_:
                this.#sS1_(e);
                break;
            case this.#sS2_:
                this.#sS2_(e);
                break;
            case this.#sS3_:
                this.#sS3_(e);
                break;
            case this.#sS4_:
                this.#sS4_(e);
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
    
    #sS0_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S0");
                
                return;
                }
                
            case "<":
                {
                this.exit_do("S0");
                
                return;
                }
                
            case "transit":
                {
                let compartment =  new TransitionSmCompartment(this.#sS1_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "change":
                {
                
                return;
                }
                
        }
    }  // ->> $S1
	
    
    #sS1_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S1");
                
                return;
                }
                
            case "<":
                {
                this.exit_do("S1");
                
                return;
                }
                
            case "transit":
                {
                let compartment =  new TransitionSmCompartment(this.#sS2_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "change":
                {
                
                return;
                }
                
        }
    }  // ->> $S2
	
    
    #sS2_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S2");
                let compartment =  new TransitionSmCompartment(this.#sS3_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "<":
                {
                this.exit_do("S2");
                
                return;
                }
                
            case "transit":
                {
                let compartment =  new TransitionSmCompartment(this.#sS3_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "change":
                {
                
                return;
                }
                
        }
    }  // ->> $S3
	
    
    #sS3_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S3");
                
                return;
                }
                
            case "<":
                {
                this.exit_do("S3");
                
                return;
                }
                
            case "transit":
                {
                let compartment =  new TransitionSmCompartment(this.#sS4_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "change":
                {
                
                return;
                }
                
        }
    }  // ->> $S4
	
    
    #sS4_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S4");
                
                return;
                }
                
              // ->> $S0
			case "<":
                {
                this.exit_do("S4");
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    enter_do(state) { throw new Error('Action not implemented.'); }
    exit_do(state) { throw new Error('Action not implemented.'); }
    
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

class TransitionSmCompartment {

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

class TransitionSmController extends TransitionSm {

	constructor() {
	  super()
	}
	enter_do(state) {}
	exit_do(state) {}
};

********************/

module.exports = TransitionSm

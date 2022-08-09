// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class Basic {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sS0_;
        this.#compartment = new BasicCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.entry_log = [];
        this.exit_log = [];
        
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
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sS0_:
                this.#sS0_(e);
                break;
            case this.#sS1_:
                this.#sS1_(e);
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
                this.entered_do("S0");
                
                return;
                }
                
            case "<":
                {
                this.left_do("S0");
                
                return;
                }
                
            case "A":
                {
                // ooh
                let compartment =  new BasicCompartment(this.#sS1_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sS1_(e) {
        switch (e._message) {
            case ">":
                {
                this.entered_do("S1");
                
                return;
                }
                
            case "<":
                {
                this.left_do("S1");
                
                return;
                }
                
            case "B":
                {
                // aah
                let compartment =  new BasicCompartment(this.#sS0_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    entered_do(msg) { throw new Error('Action not implemented.'); }
    left_do(msg) { throw new Error('Action not implemented.'); }
    
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

class BasicCompartment {

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

class BasicController extends Basic {

	constructor() {
	  super()
	}
	entered_do(msg) {}
	left_do(msg) {}
};

********************/

module.exports = Basic

// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

const FrameEvent = require("../framelang/FrameEvent")

class SimpleHandlerCalls {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new SimpleHandlerCallsCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        
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
            case "A":
                {
                let compartment =  new SimpleHandlerCallsCompartment(this.#sA_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "B":
                {
                let compartment =  new SimpleHandlerCallsCompartment(this.#sB_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "C":
                {
                this.A();
                return;
                
                return;
                }
                
            case "D":
                {
                this.B();
                return;
                let compartment =  new SimpleHandlerCallsCompartment(this.#sA_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "E":
                {
                this.D();
                return;
                this.C();
                return;
                
                return;
                }
                
        }
    }
    
    #sA_(e) {
        switch (e._message) {
        }
    }
    
    #sB_(e) {
        switch (e._message) {
        }
    }
    
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

class SimpleHandlerCallsCompartment {

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

class SimpleHandlerCallsController extends SimpleHandlerCalls {

	constructor() {
	  super()
	}
};

********************/

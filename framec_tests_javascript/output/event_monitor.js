// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

const FrameEvent = require("../framelang/FrameEvent")

class EventMonitorSm {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sA_;
        this.#compartment = new EventMonitorSmCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    change() {
        let e = FrameEvent("change",null);
        this.#mux(e);
        return e._return;
    }
    
    transit(x) {
        let e = FrameEvent("transit",{"x":x});
        this.#mux(e);
    }
    
    mult(a,b) {
        let e = FrameEvent("mult",{"a":a,"b":b});
        this.#mux(e);
        return e._return;
    }
    
    reset() {
        let e = FrameEvent("reset",null);
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sA_:
                this.#sA_(e);
                break;
            case this.#sB_:
                this.#sB_(e);
                break;
            case this.#sC_:
                this.#sC_(e);
                break;
            case this.#sD_:
                this.#sD_(e);
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
    
    #sA_(e) {
        switch (e._message) {
            case "<":
                {
                
                return;
                }
                
            case "change":
                {
                this.#changeState(this.#sB_);
                e._return = 2
                return;
                }
                
            case "transit":
                {
                this.#compartment.ExitArgs["a_out"] = 3;
                let compartment =  new EventMonitorSmCompartment(this.#sB_);
                
                compartment.EnterArgs["b_in"] = 4;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "mult":
                {
                let out = e._parameters["a"] * e._parameters["b"];
                e._return = out
                return;
                }
                
        }
    }
    
    #sB_(e) {
        switch (e._message) {
            case ">":
                {
                this.transit(11);
                return;
                
                return;
                }
                
            case "<":
                {
                
                return;
                }
                
            case "change":
                {
                this.#changeState(this.#sC_);
                e._return = 12
                return;
                }
                
            case "transit":
                {
                this.#compartment.ExitArgs["b_out"] = 13;
                let compartment =  new EventMonitorSmCompartment(this.#sC_);
                
                compartment.EnterArgs["c_in"] = 14;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "mult":
                {
                let out = e._parameters["a"] * e._parameters["b"];
                e._return = out
                return;
                }
                
            case "reset":
                {
                this.#changeState(this.#sA_);
                
                return;
                }
                
        }
    }
    
    #sC_(e) {
        switch (e._message) {
            case ">":
                {
                this.transit(21);
                return;
                
                return;
                }
                
            case "<":
                {
                
                return;
                }
                
            case "change":
                {
                this.#changeState(this.#sD_);
                e._return = 22
                return;
                }
                
            case "transit":
                {
                this.#compartment.ExitArgs["c_out"] = 23;
                let compartment =  new EventMonitorSmCompartment(this.#sD_);
                
                compartment.EnterArgs["d_in"] = 24;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "mult":
                {
                let out = e._parameters["a"] * e._parameters["b"];
                e._return = out
                return;
                }
                
            case "reset":
                {
                this.#changeState(this.#sA_);
                
                return;
                }
                
        }
    }
    
    #sD_(e) {
        switch (e._message) {
            case ">":
                {
                this.change();
                return;
                
                return;
                }
                
            case "<":
                {
                
                return;
                }
                
            case "change":
                {
                this.#changeState(this.#sA_);
                e._return = 32
                return;
                }
                
            case "transit":
                {
                this.#compartment.ExitArgs["d_out"] = 33;
                let compartment =  new EventMonitorSmCompartment(this.#sA_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "mult":
                {
                let out = e._parameters["a"] * e._parameters["b"];
                e._return = out
                return;
                }
                
            case "reset":
                {
                this.#changeState(this.#sA_);
                
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
    
    #changeState(compartment) {
        this.#compartment = compartment;
    }
    
    
    
};

//=============== Compartment ==============//

class EventMonitorSmCompartment {

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

class EventMonitorSmController extends EventMonitorSm {

	constructor() {
	  super()
	}
};

********************/
module.exports = Monitor
// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class EventHandler {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sS1_;
        this.#compartment = new EventHandlerCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    LogIt(x) {
        let e = FrameEvent("LogIt",{"x":x});
        this.#mux(e);
    }
    
    LogAdd(a,b) {
        let e = FrameEvent("LogAdd",{"a":a,"b":b});
        this.#mux(e);
    }
    
    LogReturn(a,b) {
        let e = FrameEvent("LogReturn",{"a":a,"b":b});
        this.#mux(e);
        return e._return
    }
    
    PassAdd(a,b) {
        let e = FrameEvent("PassAdd",{"a":a,"b":b});
        this.#mux(e);
    }
    
    PassReturn(a,b) {
        let e = FrameEvent("PassReturn",{"a":a,"b":b});
        this.#mux(e);
        return e._return
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sS1_:
                this.#sS1_(e);
                break;
            case this.#sS2_:
                this.#sS2_(e);
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
    
    #sS1_(e) {
        switch (e._message) {
            case "LogIt":
                {
                this.log_do("x",(e._parameters["x"]));
                
                return;
                }
                
            case "LogAdd":
                {
                this.log_do("a",(e._parameters["a"]));
                this.log_do("b",(e._parameters["b"]));
                this.log_do("a+b",(e._parameters["a"]) + (e._parameters["b"]));
                
                return;
                }
                
            case "LogReturn":
                {
                this.log_do("a",(e._parameters["a"]));
                this.log_do("b",(e._parameters["b"]));
                let r = e._parameters["a"] + e._parameters["b"];
                this.log_do("r",r);
                let compartment =  new EventHandlerCompartment(this.#sS2_);
                
                
                this.#transition(compartment);
                e._return = r
                return;
                }
                
            case "PassAdd":
                {
                let compartment =  new EventHandlerCompartment(this.#sS2_);
                
                compartment.StateArgs["p"] = e._parameters["a"] + e._parameters["b"];
                
                this.#transition(compartment);
                
                return;
                }
                
            case "PassReturn":
                {
                let r = e._parameters["a"] + e._parameters["b"];
                this.log_do("r",r);
                let compartment =  new EventHandlerCompartment(this.#sS2_);
                
                compartment.StateArgs["p"] = r;
                
                this.#transition(compartment);
                e._return = r
                return;
                }
                
        }
    }
    
    #sS2_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do("p",(this.#compartment.StateArgs["p"]));
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    log_do(msg,val) { throw new Error('Action not implemented.'); }
    
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

class EventHandlerCompartment {

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

class EventHandlerController extends EventHandler {

	constructor() {
	  super()
	}
	log_do(msg,val) {}
};

********************/

module.exports = EventHandler

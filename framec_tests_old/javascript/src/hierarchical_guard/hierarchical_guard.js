// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class HierarchicalGuard {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sI_;
        this.#compartment = new HierarchicalGuardCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    A(i) {
        let e = FrameEvent("A",{"i":i});
        this.#mux(e);
    }
    
    B(i) {
        let e = FrameEvent("B",{"i":i});
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sI_:
                this.#sI_(e);
                break;
            case this.#sS_:
                this.#sS_(e);
                break;
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
    
    #sI_(e) {
        switch (e._message) {
            case ">":
                {
                let compartment =  new HierarchicalGuardCompartment(this.#sS_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sS_(e) {
        switch (e._message) {
            case "A":
                {
                this.log_do("S.A");
                if ((e._parameters["i"]) < 10) {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS0_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS1_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                return;
                }
                
            case "B":
                {
                this.log_do("S.B");
                if ((e._parameters["i"]) < 10) {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS2_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS3_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                return;
                }
                
        }
    }
    
    #sS0_(e) {
        switch (e._message) {
            case "A":
                {
                this.log_do("S0.A");
                if ((e._parameters["i"]) > 0) {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS2_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                }
                
                break;
                }
                
              // fall through else branch
			case "B":
                {
                this.log_do("S0.B");
                if ((e._parameters["i"]) > 0) {
                } else {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS1_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                break;
                }
                
        }
        this.#sS_(e);
        
    }  // fall through then branch
	
    
    #sS1_(e) {
        switch (e._message) {
            case "A":
                {
                this.log_do("S1.A");
                if ((e._parameters["i"]) > 5) {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS3_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                }
                
                break;
                }
                
        }
        this.#sS0_(e);
        
    }  // fall through else branch
	
    
    #sS2_(e) {
        switch (e._message) {
            case "A":
                {
                this.log_do("S2.A");
                if ((e._parameters["i"]) > 10) {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS4_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                }
                
                break;
                }
                
              // fall through then branch
			case "B":
                {
                this.log_do("S2.B");
                if (!((e._parameters["i"]) > 10)) {
                } else {
                    let compartment =  new HierarchicalGuardCompartment(this.#sS4_);
                    
                    
                    this.#transition(compartment);
                    return;
                }
                
                break;
                }
                
        }
        this.#sS1_(e);
        
    }  // fall through then branch
	
    
    #sS3_(e) {
        switch (e._message) {
            case "A":
                {
                this.log_do("S3.A");
                if ((e._parameters["i"]) > 0) {
                    this.log_do("stop");
                    
                    return;
                } else {
                    this.log_do("continue");
                }
                
                break;
                }
                
            case "B":
                {
                this.log_do("S3.B");
                if ((e._parameters["i"]) > 0) {
                    this.log_do("continue");
                } else {
                    this.log_do("stop");
                    
                    return;
                }
                
                break;
                }
                
        }
        this.#sS_(e);
        
    }
    
    #sS4_(e) {
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
    
    state_info() {
        return this.#compartment.state.name;
    }
    
    
};

//=============== Compartment ==============//

class HierarchicalGuardCompartment {

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

class HierarchicalGuardController extends HierarchicalGuard {

	constructor() {
	  super()
	}
	log_do(msg) {}
};

********************/

module.exports = HierarchicalGuard

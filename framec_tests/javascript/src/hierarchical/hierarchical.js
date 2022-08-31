// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class Hierarchical {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sI_;
        this.#compartment = new HierarchicalCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.enters = [];
        this.exits = [];
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
            case this.#sT_:
                this.#sT_(e);
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
                let compartment =  new HierarchicalCompartment(this.#sS_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sS_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S");
                
                return;
                }
                
            case "<":
                {
                this.exit_do("S");
                
                return;
                }
                
            case "A":
                {
                this.log_do("S.A");
                let compartment =  new HierarchicalCompartment(this.#sS0_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "B":
                {
                this.log_do("S.B");
                let compartment =  new HierarchicalCompartment(this.#sS1_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sS0_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S0");
                
                break;
                }
                
            case "<":
                {
                this.exit_do("S0");
                
                break;
                }
                
              //  override parent handler
			case "A":
                {
                this.log_do("S0.A");
                let compartment =  new HierarchicalCompartment(this.#sT_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
              //  do this, then parent handler
			case "B":
                {
                this.log_do("S0.B");
                
                break;
                }
                
              //  extend parent handler
			case "C":
                {
                this.log_do("S0.C");
                let compartment =  new HierarchicalCompartment(this.#sS2_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
        this.#sS_(e);
        
    }
    
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
                
              //  defer to parent for A
			  //  do this, then parent, which transitions here
			case "B":
                {
                this.log_do("S1.B");
                
                break;
                }
                
              //  propagate message not handled by parent
			case "C":
                {
                this.log_do("S1.C");
                
                break;
                }
                
        }
        this.#sS_(e);
        
    }
    
    #sS2_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S2");
                
                break;
                }
                
            case "<":
                {
                this.exit_do("S2");
                
                break;
                }
                
              //  will propagate to S0 and S
			case "B":
                {
                this.log_do("S2.B");
                
                break;
                }
                
            case "C":
                {
                this.log_do("S2.C");
                let compartment =  new HierarchicalCompartment(this.#sT_);
                
                
                this.#transition(compartment);
                
                return;
                break;
                }
                
        }
        this.#sS0_(e);
        
    }  //  continue after transition (should be ignored)
	
    
    #sS3_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("S3");
                
                break;
                }
                
            case "<":
                {
                this.exit_do("S3");
                
                break;
                }
                
              //  defer to grandparent for A
			  //  override and move to sibling
			case "B":
                {
                this.log_do("S3.B");
                let compartment =  new HierarchicalCompartment(this.#sS2_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
        this.#sS1_(e);
        
    }
    
    #sT_(e) {
        switch (e._message) {
            case ">":
                {
                this.enter_do("T");
                
                return;
                }
                
            case "<":
                {
                this.exit_do("T");
                
                return;
                }
                
            case "A":
                {
                this.log_do("T.A");
                let compartment =  new HierarchicalCompartment(this.#sS_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "B":
                {
                this.log_do("T.B");
                let compartment =  new HierarchicalCompartment(this.#sS2_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "C":
                {
                this.log_do("T.C");
                let compartment =  new HierarchicalCompartment(this.#sS3_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    enter_do(msg) { throw new Error('Action not implemented.'); }
    exit_do(msg) { throw new Error('Action not implemented.'); }
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

class HierarchicalCompartment {

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

class HierarchicalController extends Hierarchical {

	constructor() {
	  super()
	}
	enter_do(msg) {}
	exit_do(msg) {}
	log_do(msg) {}
};

********************/

module.exports = Hierarchical

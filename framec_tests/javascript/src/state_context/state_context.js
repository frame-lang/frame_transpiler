// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class StateContextSm {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new StateContextSmCompartment(this.#state);
        this.#nextCompartment = null;
        this.#compartment.StateVars["w"] = this.#compartment.StateVars["w"] + 1;
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    Start() {
        let e = FrameEvent("Start",null);
        this.#mux(e);
    }
    
    LogState() {
        let e = FrameEvent("LogState",null);
        this.#mux(e);
    }
    
    Inc() {
        let e = FrameEvent("Inc",null);
        this.#mux(e);
        return e._return
    }
    
    Next(arg) {
        let e = FrameEvent("Next",{"arg":arg});
        this.#mux(e);
    }
    
    Change(arg) {
        let e = FrameEvent("Change",{"arg":arg});
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sInit_:
                this.#sInit_(e);
                break;
            case this.#sFoo_:
                this.#sFoo_(e);
                break;
            case this.#sBar_:
                this.#sBar_(e);
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
                (this.#compartment.StateVars["w"]) = 3;
                this.log_do("w",(this.#compartment.StateVars["w"]));
                
                return;
                }
                
            case "Inc":
                {
                (this.#compartment.StateVars["w"]) = (this.#compartment.StateVars["w"]) + 1;
                this.log_do("w",(this.#compartment.StateVars["w"]));
                e._return = (this.#compartment.StateVars["w"])
                return;
                }
                
            case "LogState":
                {
                this.log_do("w",(this.#compartment.StateVars["w"]));
                
                return;
                }
                
            case "Start":
                {
                let compartment =  new StateContextSmCompartment(this.#sFoo_);
                
                compartment.EnterArgs["a"] = 3;
                compartment.EnterArgs["b"] = this.#compartment.StateVars["w"];
                compartment.StateVars["x"] = this.#compartment.StateVars["x"] + 1;
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sFoo_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do("a",(e._parameters["a"]));
                this.log_do("b",(e._parameters["b"]));
                (this.#compartment.StateVars["x"]) = (e._parameters["a"]) * (e._parameters["b"]);
                this.log_do("x",(this.#compartment.StateVars["x"]));
                
                return;
                }
                
            case "<":
                {
                this.log_do("c",(e._parameters["c"]));
                (this.#compartment.StateVars["x"]) = (this.#compartment.StateVars["x"]) + (e._parameters["c"]);
                this.log_do("x",(this.#compartment.StateVars["x"]));
                
                return;
                }
                
            case "LogState":
                {
                this.log_do("x",(this.#compartment.StateVars["x"]));
                
                return;
                }
                
            case "Inc":
                {
                (this.#compartment.StateVars["x"]) = (this.#compartment.StateVars["x"]) + 1;
                this.log_do("x",(this.#compartment.StateVars["x"]));
                e._return = (this.#compartment.StateVars["x"])
                return;
                }
                
            case "Next":
                {
                let tmp = e._parameters["arg"] * 10;
                this.#compartment.ExitArgs["c"] = 10;
                let compartment =  new StateContextSmCompartment(this.#sBar_);
                
                compartment.EnterArgs["a"] = tmp;
                compartment.StateArgs["y"] = this.#compartment.StateVars["x"];
                compartment.StateVars["z"] = this.#compartment.StateVars["z"] + 1;
                
                this.#transition(compartment);
                
                return;
                }
                
              // FIXME: Swapping this to 10 * arg causes a parse error!
			case "Change":
                {
                let tmp = this.#compartment.StateVars["x"] + e._parameters["arg"];
                
                return;
                }
                
        }
    }  // ->> $Bar(tmp)
	
    
    #sBar_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do("a",(e._parameters["a"]));
                this.log_do("y",(this.#compartment.StateArgs["y"]));
                (this.#compartment.StateVars["z"]) = (e._parameters["a"]) + (this.#compartment.StateArgs["y"]);
                this.log_do("z",(this.#compartment.StateVars["z"]));
                
                return;
                }
                
            case "LogState":
                {
                this.log_do("y",(this.#compartment.StateArgs["y"]));
                this.log_do("z",(this.#compartment.StateVars["z"]));
                
                return;
                }
                
            case "Inc":
                {
                (this.#compartment.StateVars["z"]) = (this.#compartment.StateVars["z"]) + 1;
                this.log_do("z",(this.#compartment.StateVars["z"]));
                e._return = (this.#compartment.StateVars["z"])
                return;
                }
                
            case "Change":
                {
                let tmp = this.#compartment.StateArgs["y"] + this.#compartment.StateVars["z"] + e._parameters["arg"];
                this.log_do("tmp",tmp);
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    log_do(name,val) { throw new Error('Action not implemented.'); }
    
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
    
      // ->> $Init
	
    
};

//=============== Compartment ==============//

class StateContextSmCompartment {

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

class StateContextSmController extends StateContextSm {

	constructor() {
	  super()
	}
	log_do(name,val) {}
};

********************/

module.exports = StateContextSm

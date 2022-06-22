// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

const FrameEvent = require("../framelang/FrameEvent")

class RustNaming {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new RustNamingCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.snake_domain_var = 300;
        this.CamelDomainVar = 550;
        this.domainVar123 = 150;
        this.snake_log = [];
        this.CamelLog = [];
        this.log123 = [];
        this.finalLog = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    snake_event(snake_param) {
        let e = FrameEvent("snake_event",{"snake_param":snake_param});
        this.#mux(e);
    }
    
    CamelEvent(CamelParam) {
        let e = FrameEvent("CamelEvent",{"CamelParam":CamelParam});
        this.#mux(e);
    }
    
    event123(param123) {
        let e = FrameEvent("event123",{"param123":param123});
        this.#mux(e);
    }
    
    call(event,param) {
        let e = FrameEvent("call",{"event":event,"param":param});
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sInit_:
                this.#sInit_(e);
                break;
            case this.#ssnake_state_:
                this.#ssnake_state_(e);
                break;
            case this.#sCamelState_:
                this.#sCamelState_(e);
                break;
            case this.#sstate123_:
                this.#sstate123_(e);
                break;
            case this.#sFinal_:
                this.#sFinal_(e);
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
            case "snake_event":
                {
                let compartment =  new RustNamingCompartment(this.#ssnake_state_);
                
                compartment.StateArgs["snake_state_param"] = e._parameters["snake_param"];
                compartment.StateVars["snake_state_var"] = this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 100;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "CamelEvent":
                {
                let compartment =  new RustNamingCompartment(this.#sCamelState_);
                
                compartment.StateArgs["CamelStateParam"] = e._parameters["CamelParam"];
                compartment.StateVars["CamelStateVar"] = this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 200;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "event123":
                {
                let compartment =  new RustNamingCompartment(this.#sstate123_);
                
                compartment.StateArgs["stateParam123"] = e._parameters["param123"];
                compartment.StateVars["stateVar123"] = this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 300;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "call":
                {
                if (((e._parameters["event"]) == "snake_event")) {
                    this.snake_event((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "CamelEvent")) {
                    this.CamelEvent((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "event123")) {
                    this.event123((e._parameters["param"]));
                    return;
                } else {
                }
                
                return;
                }
                
        }
    }
    
    #ssnake_state_(e) {
        switch (e._message) {
              //  1100
			case "snake_event":
                {
                let snake_local_var = this.#compartment.StateVars["snake_state_var"] + this.#compartment.StateArgs["snake_state_param"] + e._parameters["snake_param"];
                this.snake_action_do(snake_local_var);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = snake_local_var;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "CamelEvent":
                {
                let CamelLocalVar = this.#compartment.StateVars["snake_state_var"] + this.#compartment.StateArgs["snake_state_param"] + e._parameters["CamelParam"];
                this.CamelAction_do(CamelLocalVar);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = CamelLocalVar;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "event123":
                {
                let localVar123 = this.#compartment.StateVars["snake_state_var"] + this.#compartment.StateArgs["snake_state_param"] + e._parameters["param123"];
                this.action123_do(localVar123);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = localVar123;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "call":
                {
                if (((e._parameters["event"]) == "snake_event")) {
                    this.snake_event((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "CamelEvent")) {
                    this.CamelEvent((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "event123")) {
                    this.event123((e._parameters["param"]));
                    return;
                } else {
                }
                
                return;
                }
                
        }
    }
    
    #sCamelState_(e) {
        switch (e._message) {
              //  1200
			case "snake_event":
                {
                let snake_local_var = this.#compartment.StateVars["CamelStateVar"] + this.#compartment.StateArgs["CamelStateParam"] + e._parameters["snake_param"];
                this.snake_action_do(snake_local_var);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = snake_local_var;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "CamelEvent":
                {
                let CamelLocalVar = this.#compartment.StateVars["CamelStateVar"] + this.#compartment.StateArgs["CamelStateParam"] + e._parameters["CamelParam"];
                this.CamelAction_do(CamelLocalVar);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = CamelLocalVar;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "event123":
                {
                let localVar123 = this.#compartment.StateVars["CamelStateVar"] + this.#compartment.StateArgs["CamelStateParam"] + e._parameters["param123"];
                this.action123_do(localVar123);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = localVar123;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "call":
                {
                if (((e._parameters["event"]) == "snake_event")) {
                    this.snake_event((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "CamelEvent")) {
                    this.CamelEvent((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "event123")) {
                    this.event123((e._parameters["param"]));
                    return;
                } else {
                }
                
                return;
                }
                
        }
    }
    
    #sstate123_(e) {
        switch (e._message) {
              //  1300
			case "snake_event":
                {
                let snake_local_var = this.#compartment.StateVars["stateVar123"] + this.#compartment.StateArgs["stateParam123"] + e._parameters["snake_param"];
                this.snake_action_do(snake_local_var);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = snake_local_var;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "CamelEvent":
                {
                let CamelLocalVar = this.#compartment.StateVars["stateVar123"] + this.#compartment.StateArgs["stateParam123"] + e._parameters["CamelParam"];
                this.CamelAction_do(CamelLocalVar);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = CamelLocalVar;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "event123":
                {
                let localVar123 = this.#compartment.StateVars["stateVar123"] + this.#compartment.StateArgs["stateParam123"] + e._parameters["param123"];
                this.action123_do(localVar123);
                let compartment =  new RustNamingCompartment(this.#sFinal_);
                
                compartment.StateArgs["result"] = localVar123;
                
                this.#transition(compartment);
                
                return;
                }
                
            case "call":
                {
                if (((e._parameters["event"]) == "snake_event")) {
                    this.snake_event((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "CamelEvent")) {
                    this.CamelEvent((e._parameters["param"]));
                    return;
                } else if (((e._parameters["event"]) == "event123")) {
                    this.event123((e._parameters["param"]));
                    return;
                } else {
                }
                
                return;
                }
                
        }
    }
    
    #sFinal_(e) {
        switch (e._message) {
            case ">":
                {
                this.logFinal_do((this.#compartment.StateArgs["result"]));
                let compartment =  new RustNamingCompartment(this.#sInit_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    snake_action_do(snake_param) { throw new Error('Action not implemented.'); }
    CamelAction_do(CamelParam) { throw new Error('Action not implemented.'); }
    action123_do(param123) { throw new Error('Action not implemented.'); }
    logFinal_do(r) { throw new Error('Action not implemented.'); }
    
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

class RustNamingCompartment {

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

class RustNamingController extends RustNaming {

	constructor() {
	  super()
	}
	snake_action_do(snake_param) {}
	CamelAction_do(CamelParam) {}
	action123_do(param123) {}
	logFinal_do(r) {}
};

********************/

module.exports = RustNaming
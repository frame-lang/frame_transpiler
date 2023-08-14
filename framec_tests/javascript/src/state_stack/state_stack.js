// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class StateStack {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    #stateStack
    
    constructor () {
        
        // Create state stack.
        
        this.#stateStack = [];
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sA_;
        this.#compartment = new StateStackCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    to_a() {
        let e = FrameEvent("to_a",null);
        this.#mux(e);
    }
    
    to_b() {
        let e = FrameEvent("to_b",null);
        this.#mux(e);
    }
    
    to_c() {
        let e = FrameEvent("to_c",null);
        this.#mux(e);
    }
    
    push() {
        let e = FrameEvent("push",null);
        this.#mux(e);
    }
    
    pop() {
        let e = FrameEvent("pop",null);
        this.#mux(e);
    }
    
    pop_change() {
        let e = FrameEvent("pop_change",null);
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
            case ">":
                {
                this.log_do("A:>");
                
                return;
                }
                
            case "<":
                {
                this.log_do("A:<");
                
                return;
                }
                
            case "to_a":
                {
                let compartment =  new StateStackCompartment(this.#sA_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_b":
                {
                let compartment =  new StateStackCompartment(this.#sB_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_c":
                {
                let compartment =  new StateStackCompartment(this.#sC_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "push":
                {
                this.#stateStack_push(this.#compartment);
                
                return;
                }
                
            case "pop":
                {
                let compartment = this.#stateStack_pop()
                this.#transition(compartment)
                
                return;
                }
                
            case "pop_change":
                {
                let compartment = this.#stateStack_pop()
                this.#changeState(compartment)
                
                return;
                }
                
        }
    }
    
    #sB_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do("B:>");
                
                return;
                }
                
            case "<":
                {
                this.log_do("B:<");
                
                return;
                }
                
            case "to_a":
                {
                let compartment =  new StateStackCompartment(this.#sA_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_b":
                {
                let compartment =  new StateStackCompartment(this.#sB_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_c":
                {
                let compartment =  new StateStackCompartment(this.#sC_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "push":
                {
                this.#stateStack_push(this.#compartment);
                
                return;
                }
                
            case "pop":
                {
                let compartment = this.#stateStack_pop()
                this.#transition(compartment)
                
                return;
                }
                
            case "pop_change":
                {
                let compartment = this.#stateStack_pop()
                this.#changeState(compartment)
                
                return;
                }
                
        }
    }
    
    #sC_(e) {
        switch (e._message) {
            case ">":
                {
                this.log_do("C:>");
                
                return;
                }
                
            case "<":
                {
                this.log_do("C:<");
                
                return;
                }
                
            case "to_a":
                {
                let compartment =  new StateStackCompartment(this.#sA_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_b":
                {
                let compartment =  new StateStackCompartment(this.#sB_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_c":
                {
                let compartment =  new StateStackCompartment(this.#sC_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "push":
                {
                this.#stateStack_push(this.#compartment);
                
                return;
                }
                
            case "pop":
                {
                let compartment = this.#stateStack_pop()
                this.#transition(compartment)
                
                return;
                }
                
            case "pop_change":
                {
                let compartment = this.#stateStack_pop()
                this.#changeState(compartment)
                
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
    
    #stateStack_push(compartment) {
        this.#stateStack.push(this.#deepClone(compartment));
    }
    
    #stateStack_pop(){
        return this.#stateStack.pop();
    }
    
    // deepcopy function for compartment
    #deepClone(target) {
        let copy = {}
        walk(target, copy);
        return copy;
        
        function walk(target, copy) {
            for (let key in target) {
                let obj = target[key];
                if (obj instanceof Function) {
                    let value = obj;
                    add(copy, key, value);
                } else if (obj instanceof Array) {
                    let value = [];
                    let last = add(copy, key, value);
                    walk(obj, last);
                } else if (obj instanceof Object) {
                    let value = {};
                    let last = add(copy, key, value);
                    walk(obj, last);
                } else {
                    let value = obj;
                    add(copy, key, value);
                }
                
            }
        }
        
        function add(copy, key, value) {
            copy[key] = value;
            return copy[key];
        }
    }
    
    #changeState(compartment) {
        this.#compartment = compartment;
    }
    
    state_info() {
        return this.#compartment.state.name;
    }
    
    
};

//=============== Compartment ==============//

class StateStackCompartment {

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

class StateStackController extends StateStack {

	constructor() {
	  super()
	}
	log_do(msg) {}
};

********************/

module.exports = StateStack

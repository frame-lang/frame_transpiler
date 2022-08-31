// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {

    var that = {};
    that._message = message;
    that._parameters = parameters;
    that._return = null;
    return that;
    
}

class Match {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new MatchCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    Empty() {
        let e = FrameEvent("Empty",null);
        this.#mux(e);
    }
    
    Simple() {
        let e = FrameEvent("Simple",null);
        this.#mux(e);
    }
    
    Multi() {
        let e = FrameEvent("Multi",null);
        this.#mux(e);
    }
    
    Nested() {
        let e = FrameEvent("Nested",null);
        this.#mux(e);
    }
    
    Child() {
        let e = FrameEvent("Child",null);
        this.#mux(e);
    }
    
    OnInt(i) {
        let e = FrameEvent("OnInt",{"i":i});
        this.#mux(e);
    }
    
    OnString(s) {
        let e = FrameEvent("OnString",{"s":s});
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sInit_:
                this.#sInit_(e);
                break;
            case this.#sEmptyMatch_:
                this.#sEmptyMatch_(e);
                break;
            case this.#sSimpleMatch_:
                this.#sSimpleMatch_(e);
                break;
            case this.#sMultiMatch_:
                this.#sMultiMatch_(e);
                break;
            case this.#sNestedMatch_:
                this.#sNestedMatch_(e);
                break;
            case this.#sChildMatch_:
                this.#sChildMatch_(e);
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
            case "Empty":
                {
                let compartment =  new MatchCompartment(this.#sEmptyMatch_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Simple":
                {
                let compartment =  new MatchCompartment(this.#sSimpleMatch_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Multi":
                {
                let compartment =  new MatchCompartment(this.#sMultiMatch_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Nested":
                {
                let compartment =  new MatchCompartment(this.#sNestedMatch_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
            case "Child":
                {
                let compartment =  new MatchCompartment(this.#sChildMatch_);
                
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sEmptyMatch_(e) {
        switch (e._message) {
            case "OnString":
                {
                if (((e._parameters["s"]) == "") || ((e._parameters["s"]) == "foo")) {
                    this.log_do("empty");
                } else {
                    this.log_do("?");
                }
                
                return;
                }
                
        }
    }  //  TODO: matching only the empty string is broken
	
    
    #sSimpleMatch_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if (((e._parameters["i"]) == 0)) {
                    this.log_do("0");
                } else if (((e._parameters["i"]) == 42)) {
                    this.log_do("42");
                } else if (((e._parameters["i"]) == 42)) {
                    this.log_do("!!!");
                } else if (((e._parameters["i"]) == -200)) {
                    this.log_do("-200");
                } else {
                    this.log_do("?");
                }
                
                return;
                }
                
            case "OnString":
                {
                if (((e._parameters["s"]) == "hello")) {
                    this.log_do("hello");
                } else if (((e._parameters["s"]) == "hello")) {
                    this.log_do("!!!");
                } else if (((e._parameters["s"]) == "goodbye")) {
                    this.log_do("goodbye");
                } else if (((e._parameters["s"]) == "Testing 1, 2, 3...")) {
                    this.log_do("testing");
                } else if (((e._parameters["s"]) == "$10!")) {
                    this.log_do("money");
                } else {
                    this.log_do("?");
                }
                
                return;
                }
                
        }
    }
    
    #sMultiMatch_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if (((e._parameters["i"]) == 3) || ((e._parameters["i"]) == -7)) {
                    this.log_do("3|-7");
                } else if (((e._parameters["i"]) == -4) || ((e._parameters["i"]) == 5) || ((e._parameters["i"]) == 6)) {
                    this.log_do("-4|5|6");
                } else {
                    this.log_do("?");
                }
                
                return;
                }
                
            case "OnString":
                {
                if (((e._parameters["s"]) == "$10") || ((e._parameters["s"]) == "12.5%") || ((e._parameters["s"]) == "@#*!")) {
                    this.log_do("symbols");
                } else if (((e._parameters["s"]) == " ") || ((e._parameters["s"]) == "  ") || ((e._parameters["s"]) == "\t") || ((e._parameters["s"]) == "\n")) {
                    this.log_do("whitespace");
                } else {
                    this.log_do("?");
                }
                
                return;
                }
                
        }
    }
    
    #sNestedMatch_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if ((e._parameters["i"]) > 0) {
                    if (((e._parameters["i"]) == 1) || ((e._parameters["i"]) == 2) || ((e._parameters["i"]) == 3)) {
                        this.log_do("1-3");
                        if (((e._parameters["i"]) == 1)) {
                            this.log_do("1");
                        } else if (((e._parameters["i"]) == 2)) {
                            this.log_do("2");
                        } else {
                            this.log_do("3");
                        }
                    } else if (((e._parameters["i"]) == 4) || ((e._parameters["i"]) == 5)) {
                        this.log_do("4-5");
                        if ((e._parameters["i"]) == 4) {
                            this.log_do("4");
                        } else {
                            this.log_do("5");
                        }
                    } else {
                        this.log_do("too big");
                    }
                } else {
                    this.log_do("too small");
                }
                
                return;
                }
                
            case "OnString":
                {
                if (((e._parameters["s"]) == "hello") || ((e._parameters["s"]) == "hola") || ((e._parameters["s"]) == "bonjour")) {
                    this.log_do("greeting");
                    if (((e._parameters["s"]) == "hello")) {
                        this.log_do("English");
                    } else if (((e._parameters["s"]) == "hola")) {
                        this.log_do("Spanish");
                    } else {
                        this.log_do("French");
                    }
                } else if (((e._parameters["s"]) == "goodbye") || ((e._parameters["s"]) == "adios") || ((e._parameters["s"]) == "au revoir")) {
                    this.log_do("farewell");
                    if (((e._parameters["s"]) == "goodbye")) {
                        this.log_do("English");
                    } else if (((e._parameters["s"]) == "adios")) {
                        this.log_do("Spanish");
                    } else {
                        this.log_do("French");
                    }
                } else {
                    this.log_do("?");
                }
                
                return;
                }
                
        }
    }
    
    #sChildMatch_(e) {
        switch (e._message) {
            case "OnInt":
                {
                if (((e._parameters["i"]) == 0)) {
                    let compartment =  new MatchCompartment(this.#sFinal_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else if (((e._parameters["i"]) == 3)) {
                    this.log_do("3");
                } else if (((e._parameters["i"]) == 4)) {
                    this.log_do("4");
                    
                    return;
                } else if (((e._parameters["i"]) == 42)) {
                    this.log_do("42 in child");
                } else if (((e._parameters["i"]) == 5)) {
                    this.log_do("5");
                    let compartment =  new MatchCompartment(this.#sFinal_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else {
                    this.log_do("no match in child");
                }
                
                break;
                }
                
            case "OnString":
                {
                if (((e._parameters["s"]) == "hello")) {
                    this.log_do("hello in child");
                } else if (((e._parameters["s"]) == "goodbye")) {
                    let compartment =  new MatchCompartment(this.#sFinal_);
                    
                    
                    this.#transition(compartment);
                    return;
                } else if (((e._parameters["s"]) == "Testing 1, 2, 3...")) {
                    this.log_do("testing in child");
                    
                    return;
                } else {
                    this.log_do("no match in child");
                }
                
                break;
                }
                
        }
        this.#sSimpleMatch_(e);
        
    }
    
    #sFinal_(e) {
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

class MatchCompartment {

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

class MatchController extends Match {

	constructor() {
	  super()
	}
	log_do(msg) {}
};

********************/

module.exports = Match

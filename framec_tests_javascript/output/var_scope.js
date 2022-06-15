// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

const FrameEvent = require("../framelang/FrameEvent")

class VarScope {
    
    // creating private properties
    
    #state
    #compartment
    #nextCompartment
    
    
    constructor () {
        
        // Create and intialize start state compartment.
        
        this.#state = this.#sInit_;
        this.#compartment = new VarScopeCompartment(this.#state);
        this.#nextCompartment = null;
        
        // Initialize domain
        this.a = "#.a";
        this.x = "#.x";
        this.tape = [];
        
        // Send system start event
        const frameEvent = FrameEvent(">", null);
        this.#mux(frameEvent);
    }
    
    //===================== Interface Block ===================//
    
    to_nn() {
        let e = FrameEvent("to_nn",null);
        this.#mux(e);
    }
    
    to_ny() {
        let e = FrameEvent("to_ny",null);
        this.#mux(e);
    }
    
    to_yn() {
        let e = FrameEvent("to_yn",null);
        this.#mux(e);
    }
    
    to_yy() {
        let e = FrameEvent("to_yy",null);
        this.#mux(e);
    }
    
    nn(d) {
        let e = FrameEvent("nn",{"d":d});
        this.#mux(e);
    }
    
    ny(d) {
        let e = FrameEvent("ny",{"d":d});
        this.#mux(e);
    }
    
    yn(d,x) {
        let e = FrameEvent("yn",{"d":d,"x":x});
        this.#mux(e);
    }
    
    yy(d,x) {
        let e = FrameEvent("yy",{"d":d,"x":x});
        this.#mux(e);
    }
    
    sigils(x) {
        let e = FrameEvent("sigils",{"x":x});
        this.#mux(e);
    }
    
    //====================== Multiplexer ====================//
    
    #mux(e) {
        switch (this.#compartment.state) {
            case this.#sInit_:
                this.#sInit_(e);
                break;
            case this.#sNN_:
                this.#sNN_(e);
                break;
            case this.#sNY_:
                this.#sNY_(e);
                break;
            case this.#sYN_:
                this.#sYN_(e);
                break;
            case this.#sYY_:
                this.#sYY_(e);
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
            case "to_nn":
                {
                let compartment =  new VarScopeCompartment(this.#sNN_);
                
                compartment.StateArgs["b"] = "$NN[b]";
                compartment.StateVars["c"] = "$NN.c";
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_ny":
                {
                let compartment =  new VarScopeCompartment(this.#sNY_);
                
                compartment.StateArgs["b"] = "$NY[b]";
                compartment.StateVars["c"] = "$NY.c";
                compartment.StateVars["x"] = "$NY.x";
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_yn":
                {
                let compartment =  new VarScopeCompartment(this.#sYN_);
                
                compartment.StateArgs["b"] = "$YN[b]";
                compartment.StateArgs["x"] = "$YN[x]";
                compartment.StateVars["c"] = "$YN.c";
                
                this.#transition(compartment);
                
                return;
                }
                
            case "to_yy":
                {
                let compartment =  new VarScopeCompartment(this.#sYY_);
                
                compartment.StateArgs["b"] = "$YY[b]";
                compartment.StateArgs["x"] = "$YY[x]";
                compartment.StateVars["c"] = "$YY.c";
                compartment.StateVars["x"] = "$YY.x";
                
                this.#transition(compartment);
                
                return;
                }
                
        }
    }
    
    #sNN_(e) {
        switch (e._message) {
            case "nn":
                {
                let e = "|nn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do(e._parameters["d"]);
                this.log_do(e);
                this.log_do(this.x);
                
                return;
                }
                
            case "ny":
                {
                let e = "|ny|.e";
                let x = "|ny|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "yn":
                {
                let e = "|yn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do((e._parameters["x"]));
                
                return;
                }
                
            case "yy":
                {
                let e = "|yy|.e";
                let x = "|yy|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "sigils":
                {
                let x = "|sigils|.x";
                this.log_do(this.x);
                
                return;
                }
                
        }
    }  //  log(||[x])
	  //  log(||.x)
	
    
    #sNY_(e) {
        switch (e._message) {
            case "nn":
                {
                let e = "|nn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do((this.#compartment.StateVars["x"]));
                
                return;
                }
                
            case "ny":
                {
                let e = "|ny|.e";
                let x = "|ny|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "yn":
                {
                let e = "|yn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do((e._parameters["x"]));
                
                return;
                }
                
            case "yy":
                {
                let e = "|yy|.e";
                let x = "|yy|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "sigils":
                {
                let x = "|sigils|.x";
                this.log_do(this.x);
                
                return;
                }
                
        }
    }  //  log($.x)
	  //  log(||[x])
	  //  log(||.x)
	
    
    #sYN_(e) {
        switch (e._message) {
            case "nn":
                {
                let e = "|nn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do((this.#compartment.StateArgs["x"]));
                
                return;
                }
                
            case "ny":
                {
                let e = "|ny|.e";
                let x = "|ny|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "yn":
                {
                let e = "|yn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do((e._parameters["x"]));
                
                return;
                }
                
            case "yy":
                {
                let e = "|yy|.e";
                let x = "|yy|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "sigils":
                {
                let x = "|sigils|.x";
                this.log_do(this.x);
                
                return;
                }
                
        }
    }  //  log($[x])
	  //  log(||[x])
	  //  log(||.x)
	
    
    #sYY_(e) {
        switch (e._message) {
            case "nn":
                {
                let e = "|nn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do((this.#compartment.StateVars["x"]));
                
                return;
                }
                
            case "ny":
                {
                let e = "|ny|.e";
                let x = "|ny|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "yn":
                {
                let e = "|yn|.e";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do((e._parameters["x"]));
                
                return;
                }
                
            case "yy":
                {
                let e = "|yy|.e";
                let x = "|yy|.x";
                this.log_do(this.a);
                this.log_do((this.#compartment.StateArgs["b"]));
                this.log_do((this.#compartment.StateVars["c"]));
                this.log_do((e._parameters["d"]));
                this.log_do(e);
                this.log_do(x);
                
                return;
                }
                
            case "sigils":
                {
                let x = "|sigils|.x";
                this.log_do(this.x);
                
                return;
                }
                
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    log_do(s) { throw new Error('Action not implemented.'); }
    
    //=============== Machinery and Mechanisms ==============//
    
    #transition(compartment) {
        this.#nextCompartment = compartment;
    }
    
    #doTransition(nextCompartment) {
        this.#mux(FrameEvent("<", this.#compartment.ExitArgs));
        this.#compartment = nextCompartment;
        this.#mux(FrameEvent(">", this.#compartment.EnterArgs));
    }
    
    
      //  log($[x])
	  //  log($.x)
	  //  log(||[x])
	  //  log(||.x)
	
    
};

//=============== Compartment ==============//

class VarScopeCompartment {

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

class VarScopeController extends VarScope {

	constructor() {
	  super()
	}
	log_do(s) {}
};

********************/

module.exports = VarScope
// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

function FrameEvent(message, parameters) {
  var that = {};
  that._message = message;
  that._parameters = parameters;
  that._return = null;
  return that;
}

class StateContextStack {
  // creating private properties

  #state;
  #compartment;
  #nextCompartment;
  #stateStack;

  constructor() {
    // Create state stack.

    this.#stateStack = [];

    // Create and intialize start state compartment.

    this.#state = this.#sA_;
    this.#compartment = new StateContextStackCompartment(this.#state);
    this.#nextCompartment = null;
    this.#compartment.StateVars["x"] = 0;

    // Initialize domain
    this.tape = [];

    // Send system start event
    const frameEvent = FrameEvent(">", null);
    this.#mux(frameEvent);
  }

  //===================== Interface Block ===================//

  to_a() {
    let e = FrameEvent("to_a", null);
    this.#mux(e);
  }

  to_b() {
    let e = FrameEvent("to_b", null);
    this.#mux(e);
  }

  to_c() {
    let e = FrameEvent("to_c", null);
    this.#mux(e);
  }

  inc() {
    let e = FrameEvent("inc", null);
    this.#mux(e);
  }

  value() {
    let e = FrameEvent("value", null);
    this.#mux(e);
    return e._return;
  }

  push() {
    let e = FrameEvent("push", null);
    this.#mux(e);
  }

  pop() {
    let e = FrameEvent("pop", null);
    this.#mux(e);
  }

  pop_change() {
    let e = FrameEvent("pop_change", null);
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

    if (this.#nextCompartment != null) {
      let nextCompartment = this.#nextCompartment;
      this.#nextCompartment = null;
      if (
        nextCompartment._forwardEvent != null &&
        nextCompartment._forwardEvent._message == ">"
      ) {
        this.#mux(FrameEvent("<", this.#compartment.ExitArgs));
        this.#compartment = nextCompartment;
        this.#mux(nextCompartment._forwardEvent);
      } else {
        this.#doTransition(nextCompartment);
        if (nextCompartment._forwardEvent != null) {
          this.#mux(nextCompartment._forwardEvent);
        }
      }
      nextCompartment._forwardEvent = null;
    }
  }

  //===================== Machine Block ===================//

  #sA_(e) {
    switch (e._message) {
      case ">": {
        this.log_do("A:>");

        return;
      }

      case "<": {
        this.log_do("A:<");

        return;
      }

      case "inc": {
        this.#compartment.StateVars["x"] = this.#compartment.StateVars["x"] + 1;

        return;
      }

      case "value": {
        e._return = this.#compartment.StateVars["x"];
        return;
      }

      case "to_a": {
        let compartment = new StateContextStackCompartment(this.#sA_);

        compartment.StateVars["x"] = 0;

        this.#transition(compartment);

        return;
      }

      case "to_b": {
        let compartment = new StateContextStackCompartment(this.#sB_);

        compartment.StateVars["y"] = 0;

        this.#transition(compartment);

        return;
      }

      case "to_c": {
        let compartment = new StateContextStackCompartment(this.#sC_);

        compartment.StateVars["z"] = 0;

        this.#transition(compartment);

        return;
      }

      case "push": {
        this.#stateStack_push(this.#compartment);

        return;
      }

      case "pop": {
        let compartment = this.#stateStack_pop();
        this.#transition(compartment);

        return;
      }

      case "pop_change": {
        let compartment = this.#stateStack_pop();
        this.#changeState(compartment);

        return;
      }
    }
  }

  #sB_(e) {
    switch (e._message) {
      case ">": {
        this.log_do("B:>");

        return;
      }

      case "<": {
        this.log_do("B:<");

        return;
      }

      case "inc": {
        this.#compartment.StateVars["y"] = this.#compartment.StateVars["y"] + 5;

        return;
      }

      case "value": {
        e._return = this.#compartment.StateVars["y"];
        return;
      }

      case "to_a": {
        let compartment = new StateContextStackCompartment(this.#sA_);

        compartment.StateVars["x"] = 0;

        this.#transition(compartment);

        return;
      }

      case "to_b": {
        let compartment = new StateContextStackCompartment(this.#sB_);

        compartment.StateVars["y"] = 0;

        this.#transition(compartment);

        return;
      }

      case "to_c": {
        let compartment = new StateContextStackCompartment(this.#sC_);

        compartment.StateVars["z"] = 0;

        this.#transition(compartment);

        return;
      }

      case "push": {
        this.#stateStack_push(this.#compartment);

        return;
      }

      case "pop": {
        let compartment = this.#stateStack_pop();
        this.#transition(compartment);

        return;
      }

      case "pop_change": {
        let compartment = this.#stateStack_pop();
        this.#changeState(compartment);

        return;
      }
    }
  }

  #sC_(e) {
    switch (e._message) {
      case ">": {
        this.log_do("C:>");

        return;
      }

      case "<": {
        this.log_do("C:<");

        return;
      }

      case "inc": {
        this.#compartment.StateVars["z"] =
          this.#compartment.StateVars["z"] + 10;

        return;
      }

      case "value": {
        e._return = this.#compartment.StateVars["z"];
        return;
      }

      case "to_a": {
        let compartment = new StateContextStackCompartment(this.#sA_);

        compartment.StateVars["x"] = 0;

        this.#transition(compartment);

        return;
      }

      case "to_b": {
        let compartment = new StateContextStackCompartment(this.#sB_);

        compartment.StateVars["y"] = 0;

        this.#transition(compartment);

        return;
      }

      case "to_c": {
        let compartment = new StateContextStackCompartment(this.#sC_);

        compartment.StateVars["z"] = 0;

        this.#transition(compartment);

        return;
      }

      case "push": {
        this.#stateStack_push(this.#compartment);

        return;
      }

      case "pop": {
        let compartment = this.#stateStack_pop();
        this.#transition(compartment);

        return;
      }

      case "pop_change": {
        let compartment = this.#stateStack_pop();
        this.#changeState(compartment);

        return;
      }
    }
  }

  //===================== Actions Block ===================//

  // Unimplemented Actions

  log_do(msg) {
    throw new Error("Action not implemented.");
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

  #stateStack_push(compartment) {
    this.#stateStack.push(cloneObj(compartment));
  }

  #stateStack_pop() {
    return this.#stateStack.pop();
  }

  #changeState(compartment) {
    this.#compartment = compartment;
  }

  state_info() {
    return this.#compartment.state.name;
  }
}

//=============== Compartment ==============//

class StateContextStackCompartment {
  constructor(state) {
    this.state = state;
  }

  StateArgs = {};
  StateVars = {};
  EnterArgs = {};
  ExitArgs = {};
  _forwardEvent = FrameEvent.call(this);
}

const cloneObj = (obj) => {
  if (Object(obj) !== obj) return obj;
  else if (Array.isArray(obj)) return obj.map(cloneObj);

  return Object.fromEntries(
    Object.entries(obj).map(([k, v]) => {
      if (typeof v == "function") {
        return [k, v];
      }
      return [k, cloneObj(v)];
    })
  );
};
/********************

class StateContextStackController extends StateContextStack {

	constructor() {
	  super()
	}
	log_do(msg) {}
};

********************/

module.exports = StateContextStack;

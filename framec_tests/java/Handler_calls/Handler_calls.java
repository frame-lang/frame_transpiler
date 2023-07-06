package framec_tests.java.Handler_calls;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class HandlerCalls {

    private HandlerCallsCompartment _compartment_;
    private HandlerCallsCompartment _nextCompartment_;



    HandlerCalls() {

        // Create and intialize start state compartment.
        this._compartment_ = new HandlerCallsCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);

    }

    // states enum
    private enum HandlerCallsState {

        INIT(0),
        NONRECURSIVE(1),
        SELFRECURSIVE(2),
        MUTUALLYRECURSIVE(3),
        FINAL(4);

        public final int value;

        private HandlerCallsState(int value) {
            this.value=value;
        }

        public int getValue() {
            return value;
        }
    }

    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == HandlerCallsState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == HandlerCallsState.NONRECURSIVE.getValue()) {
            this._sNonRecursive_(e);
        }else if(this._compartment_.state == HandlerCallsState.SELFRECURSIVE.getValue()) {
            this._sSelfRecursive_(e);
        }else if(this._compartment_.state == HandlerCallsState.MUTUALLYRECURSIVE.getValue()) {
            this._sMutuallyRecursive_(e);
        }else if(this._compartment_.state == HandlerCallsState.FINAL.getValue()) {
            this._sFinal_(e);
        }


        if(this._nextCompartment_ != null) {
            HandlerCallsCompartment nextCompartment = this._nextCompartment_;
            this._nextCompartment_ = null;
            if(nextCompartment._forwardEvent != null &&
             nextCompartment._forwardEvent._message == ">") {
                this._mux_(new FrameEvent( "<", this._compartment_.exitArgs));
                this._compartment_ = nextCompartment;
                this._mux_(nextCompartment._forwardEvent);
            } else {
                this._doTransition_(nextCompartment);
                if(nextCompartment._forwardEvent != null) {
                    this._mux_(nextCompartment._forwardEvent);
                }
            }
            nextCompartment._forwardEvent = null;
        }
    }

    //===================== Interface Block ===================//

    public void NonRec() {
        FrameEvent e = new FrameEvent("NonRec", null);
        this._mux_(e);
    }

    public void SelfRec() {
        FrameEvent e = new FrameEvent("SelfRec", null);
        this._mux_(e);
    }

    public void MutRec() {
        FrameEvent e = new FrameEvent("MutRec", null);
        this._mux_(e);
    }

    public void Call(String event,int arg) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("event", event);

        parameters.put("arg", arg);

        FrameEvent e = new FrameEvent("Call", parameters);
        this._mux_(e);
    }

    public void Foo(int arg) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("arg", arg);

        FrameEvent e = new FrameEvent("Foo", parameters);
        this._mux_(e);
    }

    public void Bar(int arg) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("arg", arg);

        FrameEvent e = new FrameEvent("Bar", parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    private void _sInit_(FrameEvent e) {
        if(e._message == "NonRec") {
            HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.NONRECURSIVE.getValue());
            compartment.stateVars.put("counter", 0);

            this._transition_(compartment);

            return;
        }
        else if(e._message == "SelfRec") {
            HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.SELFRECURSIVE.getValue());
            compartment.stateVars.put("counter", 0);

            this._transition_(compartment);

            return;
        }
        else if(e._message == "MutRec") {
            HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.MUTUALLYRECURSIVE.getValue());
            compartment.stateVars.put("counter", 0);

            this._transition_(compartment);

            return;
        }
    }

    private void _sNonRecursive_(FrameEvent e) {
        if(e._message == "Foo") {
            log_do("Foo",((int) e._parameters.get("arg")));
            this._compartment_.stateVars.put("counter",(int) this._compartment_.stateVars.get("counter") + (int) e._parameters.get("arg"));
            print_do(String.valueOf(((int) this._compartment_.stateVars.get("counter"))));
            Bar(((int) e._parameters.get("arg")) * 2);
            return;

            //return;
        }
          //  the front-end should report the next line as a static error
          //  need to handle the case for unreachable code
          // log("Unreachable" 0)
        else if(e._message == "Bar") {
            log_do("Bar",((int) e._parameters.get("arg")));
            this._compartment_.stateVars.put("counter",(int) this._compartment_.stateVars.get("counter") + (int) e._parameters.get("arg"));
            print_do(String.valueOf(((int) this._compartment_.stateVars.get("counter"))));
            HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.FINAL.getValue());
            compartment.stateArgs.put("counter", (int) this._compartment_.stateVars.get("counter"));

            this._transition_(compartment);

            return;
        }
        else if(e._message == "Call") {
            if ((((String) e._parameters.get("event")) == "Foo")) {
                Foo(((int) e._parameters.get("arg")));
                return;
            } else if ((((String) e._parameters.get("event")) == "Bar")) {
                Bar(((int) e._parameters.get("arg")));
                return;
            } else {
                Call("Foo",1000);
                return;
            }

            //return;
        }
    }

    private void _sSelfRecursive_(FrameEvent e) {
        if(e._message == "Foo") {
            log_do("Foo",((int) e._parameters.get("arg")));
            this._compartment_.stateVars.put("counter",(int) this._compartment_.stateVars.get("counter") + (int) e._parameters.get("arg"));
            print_do(String.valueOf(((int) this._compartment_.stateVars.get("counter"))));
            if (((int) this._compartment_.stateVars.get("counter")) < 100) {
                Foo(((int) e._parameters.get("arg")) * 2);
                return;
            } else {
                HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.FINAL.getValue());
                compartment.stateArgs.put("counter", (int) this._compartment_.stateVars.get("counter"));

                this._transition_(compartment);
            }

            return;
        }
        else if(e._message == "Bar") {
            log_do("Bar",((int) e._parameters.get("arg")));
            this._compartment_.stateVars.put("counter",(int) this._compartment_.stateVars.get("counter") + (int) e._parameters.get("arg"));
            print_do(String.valueOf(((int) this._compartment_.stateVars.get("counter"))));
            HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.FINAL.getValue());
            compartment.stateArgs.put("counter", (int) this._compartment_.stateVars.get("counter"));

            this._transition_(compartment);

            return;
        }
        else if(e._message == "Call") {
            if ((((String) e._parameters.get("event")) == "Foo")) {
                Foo(((int) e._parameters.get("arg")));
                return;
            } else if ((((String) e._parameters.get("event")) == "Bar")) {
                Bar(((int) e._parameters.get("arg")));
                return;
            } else {
            }

            return;
        }
    }

    private void _sMutuallyRecursive_(FrameEvent e) {
        if(e._message == "Foo") {
            log_do("Foo",((int) e._parameters.get("arg")));
            this._compartment_.stateVars.put("counter",(int) this._compartment_.stateVars.get("counter") + (int) e._parameters.get("arg"));
            print_do(String.valueOf(((int) this._compartment_.stateVars.get("counter"))));
            if (((int) this._compartment_.stateVars.get("counter")) > 100) {
                HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.FINAL.getValue());
                compartment.stateArgs.put("counter", (int) this._compartment_.stateVars.get("counter"));

                this._transition_(compartment);
            } else {
                Bar(((int) e._parameters.get("arg")) * 2);
                return;
            }

            return;
        }
        else if(e._message == "Bar") {
            log_do("Bar",((int) e._parameters.get("arg")));
            this._compartment_.stateVars.put("counter",(int) this._compartment_.stateVars.get("counter") + (int) e._parameters.get("arg"));
            print_do(String.valueOf(((int) this._compartment_.stateVars.get("counter"))));
            if ((((int) e._parameters.get("arg")) == 4)) {
                Foo(((int) e._parameters.get("arg")));
                return;
            } else if ((((int) e._parameters.get("arg")) == 8)) {
                Foo(((int) e._parameters.get("arg")) * 2);
                return;
            } else {
                Foo(((int) e._parameters.get("arg")) * 3);
                //return;
            }

            return;
        }
        else if(e._message == "Call") {
            if ((((String) e._parameters.get("event")) == "Foo")) {
                Foo(((int) e._parameters.get("arg")));
                return;
            } else if ((((String) e._parameters.get("event")) == "Bar")) {
                Bar(((int) e._parameters.get("arg")));
                return;
            } else {
            }

            return;
        }
    }

    private void _sFinal_(FrameEvent e) {
        if(e._message == ">") {
            log_do("Final",((int) this._compartment_.stateArgs.get("counter")));
            HandlerCallsCompartment compartment =  new HandlerCallsCompartment(HandlerCallsState.INIT.getValue());

            this._transition_(compartment);

            return;
        }
    }

    //===================== Actions Block ===================//

    protected void print_do(String s) {
        System.out.println(s);
    }

    // Unimplemented Actions

    protected void log_do(String from, int val) { throw new UnsupportedOperationException(); }

    //===================== Domain Block ===================//

    public ArrayList<String> tape  = new ArrayList<String>();

    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(HandlerCallsCompartment compartment) {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(HandlerCallsCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }

    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }

}

//=============== Compartment ==============//

class HandlerCallsCompartment {

    public int getState() {
        return state;
    }

    public void setState(int state) {
        this.state = state;
    }

    public HashMap<String, Object> getStateArgs() {
        return stateArgs;
    }

    public void setStateArgs(HashMap<String, Object> stateArgs) {
        this.stateArgs = stateArgs;
    }

    public HashMap<String, Object> getStateVars() {
        return stateVars;
    }

    public void setStateVars(HashMap<String, Object> stateVars) {
        this.stateVars = stateVars;
    }

    public HashMap<String, Object> getEnterArgs() {
        return enterArgs;
    }

    public void setEnterArgs(HashMap<String, Object> enterArgs) {
        this.enterArgs = enterArgs;
    }

    public HashMap<String, Object> getExitArgs() {
        return exitArgs;
    }

    public void setExitArgs(HashMap<String, Object> exitArgs) {
        this.exitArgs = exitArgs;
    }

    public FrameEvent get_forwardEvent() {
        return _forwardEvent;
    }

    public void set_forwardEvent(FrameEvent _forwardEvent) {
        this._forwardEvent = _forwardEvent;
    }
    int state;
    HandlerCallsCompartment(){

    }
    HandlerCallsCompartment(int state) {
        this.state = state;
    }

    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();

    public HandlerCallsCompartment(int state, HashMap<String, Object> stateArgs, HashMap<String, Object> stateVars,
            HashMap<String, Object> enterArgs, HashMap<String, Object> exitArgs, FrameEvent _forwardEvent) {
        this.state = state;
        this.stateArgs = stateArgs;
        this.stateVars = stateVars;
        this.enterArgs = enterArgs;
        this.exitArgs = exitArgs;
        this._forwardEvent = _forwardEvent;
    }
}


/********************

class HandlerCallsController extends HandlerCalls {

        HandlerCallsController() {
          super();
        }

    protected void log_do(String from, int val) {}
}

********************/
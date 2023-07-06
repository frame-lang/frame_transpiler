// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.Simple_handler_calls;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class SimpleHandlerCalls {

    private SimpleHandlerCallsCompartment _compartment_;
    private SimpleHandlerCallsCompartment _nextCompartment_;



    SimpleHandlerCalls() {

        // Create and intialize start state compartment.
        this._compartment_ = new SimpleHandlerCallsCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);

    }

    // states enum
    private enum SimpleHandlerCallsState {

        INIT(0),
        A(1),
        B(2);

        public final int value;

        private SimpleHandlerCallsState(int value) {
            this.value=value;
        }

        public int getValue() {
            return value;
        }
    }

    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == SimpleHandlerCallsState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == SimpleHandlerCallsState.A.getValue()) {
            this._sA_(e);
        }else if(this._compartment_.state == SimpleHandlerCallsState.B.getValue()) {
            this._sB_(e);
        }


        if(this._nextCompartment_ != null) {
            SimpleHandlerCallsCompartment nextCompartment = this._nextCompartment_;
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

    public void A() {
        FrameEvent e = new FrameEvent("A", null);
        this._mux_(e);
    }

    public void B() {
        FrameEvent e = new FrameEvent("B", null);
        this._mux_(e);
    }

    public void C() {
        FrameEvent e = new FrameEvent("C", null);
        this._mux_(e);
    }

    public void D() {
        FrameEvent e = new FrameEvent("D", null);
        this._mux_(e);
    }

    public void E() {
        FrameEvent e = new FrameEvent("E", null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    private void _sInit_(FrameEvent e) {
        if(e._message == "A") {
            SimpleHandlerCallsCompartment compartment =  new SimpleHandlerCallsCompartment(SimpleHandlerCallsState.A.getValue());

            this._transition_(compartment);

            return;
        }
        else if(e._message == "B") {
            SimpleHandlerCallsCompartment compartment =  new SimpleHandlerCallsCompartment(SimpleHandlerCallsState.B.getValue());

            this._transition_(compartment);

            return;
        }
        else if(e._message == "C") {
            A();
            return;

        
        }
        else if(e._message == "D") {
            B();
            if(true){
            return;
            }
            SimpleHandlerCallsCompartment compartment =  new SimpleHandlerCallsCompartment(SimpleHandlerCallsState.A.getValue());

            this._transition_(compartment);

            return;
        }
        else if(e._message == "E") {
            D();
            if(true){
            return;
            }
            C();
            return;

        }
    }

    private void _sA_(FrameEvent e) {
    }

    private void _sB_(FrameEvent e) {
    }

    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(SimpleHandlerCallsCompartment compartment) {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(SimpleHandlerCallsCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }

    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }

}

//=============== Compartment ==============//

class SimpleHandlerCallsCompartment {

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
    SimpleHandlerCallsCompartment(){

    }
    SimpleHandlerCallsCompartment(int state) {
        this.state = state;
    }

    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();

    public SimpleHandlerCallsCompartment(int state, HashMap<String, Object> stateArgs, HashMap<String, Object> stateVars,
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

class SimpleHandlerCallsController extends SimpleHandlerCalls {

        SimpleHandlerCallsController() {
          super();
        }
}

********************/
    
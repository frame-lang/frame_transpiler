// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.StateVars;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class StateVars {
    
    StateVarsCompartment _compartment_;
    private StateVarsCompartment _nextCompartment_;
    
    
    
    StateVars() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new StateVarsCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum StateVarsState {
        
        INIT(0),
        A(1),
        B(2);
        
        public final int value;
        
        private StateVarsState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == StateVarsState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == StateVarsState.A.getValue()) {
            this._sA_(e);
        }else if(this._compartment_.state == StateVarsState.B.getValue()) {
            this._sB_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            StateVarsCompartment nextCompartment = this._nextCompartment_;
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
    
    public void X() {
        FrameEvent e = new FrameEvent("X", null);
        this._mux_(e);
    }
    
    public void Y() {
        FrameEvent e = new FrameEvent("Y", null);
        this._mux_(e);
    }
    
    public void Z() {
        FrameEvent e = new FrameEvent("Z", null);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sInit_(FrameEvent e) {
        if(e._message == ">") {
            StateVarsCompartment compartment =  new StateVarsCompartment(StateVarsState.A.getValue());
            compartment.stateVars.put("x", 0);
            
            this._transition_(compartment);
            return;
        }
    }
    
    private void _sA_(FrameEvent e) {
        if(e._message == "X") {
            this._compartment_.stateVars.put("x",(int) this._compartment_.stateVars.get("x") + 1);
            return;
        }
        else if(e._message == "Y") {
            StateVarsCompartment compartment =  new StateVarsCompartment(StateVarsState.B.getValue());
            compartment.stateVars.put("y", 10);
            compartment.stateVars.put("z", 100);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "Z") {
            StateVarsCompartment compartment =  new StateVarsCompartment(StateVarsState.B.getValue());
            compartment.stateVars.put("y", 10);
            compartment.stateVars.put("z", 100);
            
            this._transition_(compartment);
            return;
        }
    }
    
    private void _sB_(FrameEvent e) {
        if(e._message == "X") {
            StateVarsCompartment compartment =  new StateVarsCompartment(StateVarsState.A.getValue());
            compartment.stateVars.put("x", 0);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "Y") {
            this._compartment_.stateVars.put("y",(int) this._compartment_.stateVars.get("y") + 1);
            return;
        }
        else if(e._message == "Z") {
            this._compartment_.stateVars.put("z",(int) this._compartment_.stateVars.get("z") + 1);
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    
    //===================== Domain Block ===================//
    
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(StateVarsCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(StateVarsCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
    }
}
    
    //=============== Compartment ==============//
    
    class StateVarsCompartment {
    
        int state;
        
        StateVarsCompartment(int state) {
            this.state = state;
        }
        
        HashMap<String, Object> stateArgs = new HashMap<String, Object>();
        HashMap<String, Object> stateVars = new HashMap<String, Object>();
        HashMap<String, Object> enterArgs = new HashMap<String, Object>();
        HashMap<String, Object> exitArgs = new HashMap<String, Object>();
        FrameEvent _forwardEvent = new FrameEvent();
    }
    
    
    /********************

    class StateVarsController extends StateVars {

    	StateVarsController() {
    	  super();
    	}
    }
    
********************/
    

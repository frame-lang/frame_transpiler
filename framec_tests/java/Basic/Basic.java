// emitted fro
package framec_tests.java.Basic;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class Basic {
    
    private BasicCompartment _compartment_;
    private BasicCompartment _nextCompartment_;
    
    
    
    Basic() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new BasicCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum BasicState {
        
        S0(0),
        S1(1);
        
        public final int value;
        
        private BasicState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == BasicState.S0.getValue()) {
            this._sS0_(e);
        }else if(this._compartment_.state == BasicState.S1.getValue()) {
            this._sS1_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            BasicCompartment nextCompartment = this._nextCompartment_;
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
    
    
    //===================== Machine Block ===================//
    
    private void _sS0_(FrameEvent e) {
        if(e._message == ">") {
            entered_do("S0");
            return;
        }
        else if(e._message == "<") {
            left_do("S0");
            return;
        }
        else if(e._message == "A") {
            // ooh
            BasicCompartment compartment =  new BasicCompartment(BasicState.S1.getValue());
            
            this._transition_(compartment);
            return;
        }
    }
    
    private void _sS1_(FrameEvent e) {
        if(e._message == ">") {
            entered_do("S1");
            return;
        }
        else if(e._message == "<") {
            left_do("S1");
            return;
        }
        else if(e._message == "B") {
            // aah
            BasicCompartment compartment =  new BasicCompartment(BasicState.S0.getValue());
            
            this._transition_(compartment);
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void entered_do(String msg) { throw new UnsupportedOperationException(); }
    protected void left_do(String msg) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> entry_log  = new ArrayList<String>();
    public ArrayList<String> exit_log  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(BasicCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(BasicCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
    }   
}

//=============== Compartment ==============//

class BasicCompartment {

    int state;
    
    BasicCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
}


/********************

class BasicController extends Basic {

	BasicController() {
	  super();
	}

    protected void entered_do(String msg) {}

    protected void left_do(String msg) {}
}

********************/

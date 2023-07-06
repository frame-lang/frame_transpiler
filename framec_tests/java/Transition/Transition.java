// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.Transition;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class TransitionSm {
    
    private TransitionSmCompartment _compartment_;
    private TransitionSmCompartment _nextCompartment_;
    
    
    
    TransitionSm() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new TransitionSmCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum TransitionSmState {
        
        S0(0),
        S1(1),
        S2(2),
        S3(3),
        S4(4);
        
        public final int value;
        
        private TransitionSmState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == TransitionSmState.S0.getValue()) {
            this._sS0_(e);
        }else if(this._compartment_.state == TransitionSmState.S1.getValue()) {
            this._sS1_(e);
        }else if(this._compartment_.state == TransitionSmState.S2.getValue()) {
            this._sS2_(e);
        }else if(this._compartment_.state == TransitionSmState.S3.getValue()) {
            this._sS3_(e);
        }else if(this._compartment_.state == TransitionSmState.S4.getValue()) {
            this._sS4_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            TransitionSmCompartment nextCompartment = this._nextCompartment_;
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
    
    public void transit() {
        FrameEvent e = new FrameEvent("transit", null);
        this._mux_(e);
    }
    
    public void change() {
        FrameEvent e = new FrameEvent("change", null);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sS0_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S0");
            
            return;
        }
        else if(e._message == "<") {
            exit_do("S0");
            
            return;
        }
        else if(e._message == "transit") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S1.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "change") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S1.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sS1_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S1");
            
            return;
        }
        else if(e._message == "<") {
            exit_do("S1");
            
            return;
        }
        else if(e._message == "transit") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S2.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "change") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S2.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sS2_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S2");
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S3.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "<") {
            exit_do("S2");
            
            return;
        }
        else if(e._message == "transit") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S3.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "change") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S3.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sS3_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S3");
            
            return;
        }
        else if(e._message == "<") {
            exit_do("S3");
            
            return;
        }
        else if(e._message == "transit") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S4.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "change") {
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S4.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sS4_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S4");
            TransitionSmCompartment compartment =  new TransitionSmCompartment(TransitionSmState.S0.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
        else if(e._message == "<") {
            exit_do("S4");
            
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void enter_do(String state) { throw new UnsupportedOperationException(); }
    protected void exit_do(String state) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> enters  = new ArrayList<String>();
    public ArrayList<String> exits  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(TransitionSmCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(TransitionSmCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    private void _changeState_(TransitionSmCompartment compartment) {
        this._compartment_ = compartment;
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }
        
}

//=============== Compartment ==============//

class TransitionSmCompartment {

    int state;
    
    TransitionSmCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
}


/********************

class TransitionSmController extends TransitionSm {

	TransitionSmController() {
	  super();
	}

    protected void enter_do(String state) {}

    protected void exit_do(String state) {}
}

********************/


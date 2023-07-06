// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.TransitionParams;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class TransitParams {
    
    private TransitParamsCompartment _compartment_;
    private TransitParamsCompartment _nextCompartment_;
    
    
    
    TransitParams() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new TransitParamsCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum TransitParamsState {
        
        INIT(0),
        A(1),
        B(2);
        
        public final int value;
        
        private TransitParamsState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == TransitParamsState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == TransitParamsState.A.getValue()) {
            this._sA_(e);
        }else if(this._compartment_.state == TransitParamsState.B.getValue()) {
            this._sB_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            TransitParamsCompartment nextCompartment = this._nextCompartment_;
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
    
    public void Next() {
        FrameEvent e = new FrameEvent("Next", null);
        this._mux_(e);
    }
    
    public void Change() {
        FrameEvent e = new FrameEvent("Change", null);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sInit_(FrameEvent e) {
        if(e._message == "Next") {
            TransitParamsCompartment compartment =  new TransitParamsCompartment(TransitParamsState.A.getValue());
            compartment.enterArgs.put("msg", "hi A");
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "Change") {
            TransitParamsCompartment compartment =  new TransitParamsCompartment(TransitParamsState.A.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sA_(FrameEvent e) {
        if(e._message == ">") {
            log_do(((String) e._parameters.get("msg")));
            
            return;
        }
        else if(e._message == "<") {
            log_do("bye A");
            
            return;
        }
        else if(e._message == "Next") {
            TransitParamsCompartment compartment =  new TransitParamsCompartment(TransitParamsState.B.getValue());
            compartment.enterArgs.put("msg", "hi B");
            compartment.enterArgs.put("val", 42);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "Change") {
            TransitParamsCompartment compartment =  new TransitParamsCompartment(TransitParamsState.B.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sB_(FrameEvent e) {
        if(e._message == ">") {
            log_do(((String) e._parameters.get("msg")));
            log_do(String.valueOf(((int) e._parameters.get("val"))));
            
            return;
        }
        else if(e._message == "<") {
            log_do(((Boolean) e._parameters.get("val")).toString());
            log_do(((String) e._parameters.get("msg")));
            
            return;
        }
        else if(e._message == "Next") {
            this._compartment_.exitArgs.put("val", true);
            this._compartment_.exitArgs.put("msg", "bye B");
            TransitParamsCompartment compartment =  new TransitParamsCompartment(TransitParamsState.A.getValue());
            compartment.enterArgs.put("msg", "hi again A");
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "Change") {
            TransitParamsCompartment compartment =  new TransitParamsCompartment(TransitParamsState.A.getValue());
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void log_do(String msg) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> tape  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(TransitParamsCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(TransitParamsCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    private void _changeState_(TransitParamsCompartment compartment) {
        this._compartment_ = compartment;
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }
        
}

//=============== Compartment ==============//

class TransitParamsCompartment {

    int state;
    
    TransitParamsCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
}


/********************

class TransitParamsController extends TransitParams {

	TransitParamsController() {
	  super();
	}

    protected void log_do(String msg) {}
}

********************/


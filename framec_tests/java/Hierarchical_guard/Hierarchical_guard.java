// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.Hierarchical_guard;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class HierarchicalGuard {
    
    private HierarchicalGuardCompartment _compartment_;
    private HierarchicalGuardCompartment _nextCompartment_;
    
    
    
    HierarchicalGuard() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new HierarchicalGuardCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum HierarchicalGuardState {
        
        I(0),
        S(1),
        S0(2),
        S1(3),
        S2(4),
        S3(5),
        S4(6);
        
        public final int value;
        
        private HierarchicalGuardState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == HierarchicalGuardState.I.getValue()) {
            this._sI_(e);
        }else if(this._compartment_.state == HierarchicalGuardState.S.getValue()) {
            this._sS_(e);
        }else if(this._compartment_.state == HierarchicalGuardState.S0.getValue()) {
            this._sS0_(e);
        }else if(this._compartment_.state == HierarchicalGuardState.S1.getValue()) {
            this._sS1_(e);
        }else if(this._compartment_.state == HierarchicalGuardState.S2.getValue()) {
            this._sS2_(e);
        }else if(this._compartment_.state == HierarchicalGuardState.S3.getValue()) {
            this._sS3_(e);
        }else if(this._compartment_.state == HierarchicalGuardState.S4.getValue()) {
            this._sS4_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            HierarchicalGuardCompartment nextCompartment = this._nextCompartment_;
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
    
    public void A(int i) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("i", i);

        FrameEvent e = new FrameEvent("A", parameters);
        this._mux_(e);
    }
    
    public void B(int i) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("i", i);

        FrameEvent e = new FrameEvent("B", parameters);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sI_(FrameEvent e) {
        if(e._message == ">") {
            HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S.getValue());
            
            this._transition_(compartment);
            return;
        }
    }
    
    private void _sS_(FrameEvent e) {
        if(e._message == "A") {
            log_do("S.A");
            if (((int) e._parameters.get("i")) < 10) {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S0.getValue());
                
                this._transition_(compartment);
                return;
            } else {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S1.getValue());
                
                this._transition_(compartment);
            }
            return;
        }
        else if(e._message == "B") {
            log_do("S.B");
            if (((int) e._parameters.get("i")) < 10) {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S2.getValue());
                
                this._transition_(compartment);
                return;
            } else {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S3.getValue());
                
                this._transition_(compartment);
            }
            return;
        }
    }
    
    private void _sS0_(FrameEvent e) {
        if(e._message == "A") {
            log_do("S0.A");
            if (((int) e._parameters.get("i")) > 0) {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S2.getValue());
                
                this._transition_(compartment);
                return;
            } else {
            }
            
        }
          //  fall through else branch
        else if(e._message == "B") {
            log_do("S0.B");
            if (((int) e._parameters.get("i")) > 0) {
            } else {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S1.getValue());
                
                this._transition_(compartment);
                return;
            }
            
        }
        _sS_(e);
        
    }  //  fall through then branch
    
    
    private void _sS1_(FrameEvent e) {
        if(e._message == "A") {
            log_do("S1.A");
            if (((int) e._parameters.get("i")) > 5) {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S3.getValue());
                
                this._transition_(compartment);
                return;
            } else {
            }
            
        }
        _sS0_(e);
        
    }  //  fall through else branch
    
    
    private void _sS2_(FrameEvent e) {
        if(e._message == "A") {
            log_do("S2.A");
            if (((int) e._parameters.get("i")) > 10) {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S4.getValue());
                
                this._transition_(compartment);
                return;
            } else {
            }
            
        }
          //  fall through then branch
        else if(e._message == "B") {
            log_do("S2.B");
            if (!(((int) e._parameters.get("i")) > 10)) {
            } else {
                HierarchicalGuardCompartment compartment =  new HierarchicalGuardCompartment(HierarchicalGuardState.S4.getValue());
                
                this._transition_(compartment);
                return;
            }
            
        }
        _sS1_(e);
        
    }  //  fall through then branch
    
    
    private void _sS3_(FrameEvent e) {
        if(e._message == "A") {
            log_do("S3.A");
            if (((int) e._parameters.get("i")) > 0) {
                log_do("stop");
                return;
            } else {
                log_do("continue");
            }
            
        }
        else if(e._message == "B") {
            log_do("S3.B");
            if (((int) e._parameters.get("i")) > 0) {
                log_do("continue");
            } else {
                log_do("stop");
                return;
            }
            
        }
        _sS_(e);
        
    }
    
    private void _sS4_(FrameEvent e) {
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void log_do(String msg) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> tape  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(HierarchicalGuardCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(HierarchicalGuardCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
            }
            
    }
    
    //=============== Compartment ==============//
    
    class HierarchicalGuardCompartment {
    
        int state;
        
        HierarchicalGuardCompartment(int state) {
            this.state = state;
        }
        
        HashMap<String, Object> stateArgs = new HashMap<String, Object>();
        HashMap<String, Object> stateVars = new HashMap<String, Object>();
        HashMap<String, Object> enterArgs = new HashMap<String, Object>();
        HashMap<String, Object> exitArgs = new HashMap<String, Object>();
        FrameEvent _forwardEvent = new FrameEvent();
    }
    
    
    /********************

    class HierarchicalGuardController extends HierarchicalGuard {

    	HierarchicalGuardController() {
    	  super();
    	}
    
    protected void log_do(String msg) {}
    }
    
********************/
    
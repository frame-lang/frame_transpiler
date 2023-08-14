// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package framec_tests.java.Branch;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class Branch {
    
    private BranchCompartment _compartment_;
    private BranchCompartment _nextCompartment_;
    
    
    
    Branch() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new BranchCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum BranchState {
        
        I(0),
        SIMPLEIF(1),
        NEGATEDIF(2),
        PRECEDENCE(3),
        NESTEDIF(4),
        GUARDEDTRANSITION(5),
        NESTEDGUARDEDTRANSITION(6),
        F1(7),
        F2(8),
        F3(9);
        
        public final int value;
        
        private BranchState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == BranchState.I.getValue()) {
            this._sI_(e);
        }else if(this._compartment_.state == BranchState.SIMPLEIF.getValue()) {
            this._sSimpleIf_(e);
        }else if(this._compartment_.state == BranchState.NEGATEDIF.getValue()) {
            this._sNegatedIf_(e);
        }else if(this._compartment_.state == BranchState.PRECEDENCE.getValue()) {
            this._sPrecedence_(e);
        }else if(this._compartment_.state == BranchState.NESTEDIF.getValue()) {
            this._sNestedIf_(e);
        }else if(this._compartment_.state == BranchState.GUARDEDTRANSITION.getValue()) {
            this._sGuardedTransition_(e);
        }else if(this._compartment_.state == BranchState.NESTEDGUARDEDTRANSITION.getValue()) {
            this._sNestedGuardedTransition_(e);
        }else if(this._compartment_.state == BranchState.F1.getValue()) {
            this._sF1_(e);
        }else if(this._compartment_.state == BranchState.F2.getValue()) {
            this._sF2_(e);
        }else if(this._compartment_.state == BranchState.F3.getValue()) {
            this._sF3_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            BranchCompartment nextCompartment = this._nextCompartment_;
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
    
    public void F() {
        FrameEvent e = new FrameEvent("F", null);
        this._mux_(e);
    }
    
    public void OnBool(Boolean b) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("b", b);

        FrameEvent e = new FrameEvent("OnBool", parameters);
        this._mux_(e);
    }
    
    public void OnInt(int i) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("i", i);

        FrameEvent e = new FrameEvent("OnInt", parameters);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sI_(FrameEvent e) {
        if(e._message == "A") {
            BranchCompartment compartment =  new BranchCompartment(BranchState.SIMPLEIF.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "B") {
            BranchCompartment compartment =  new BranchCompartment(BranchState.NEGATEDIF.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "C") {
            BranchCompartment compartment =  new BranchCompartment(BranchState.PRECEDENCE.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "D") {
            BranchCompartment compartment =  new BranchCompartment(BranchState.NESTEDIF.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "E") {
            BranchCompartment compartment =  new BranchCompartment(BranchState.GUARDEDTRANSITION.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "F") {
            BranchCompartment compartment =  new BranchCompartment(BranchState.NESTEDGUARDEDTRANSITION.getValue());
            
            this._transition_(compartment);
            
            return;
        }
    }
    
    private void _sSimpleIf_(FrameEvent e) {
        if(e._message == "OnBool") {
            if (((Boolean) e._parameters.get("b"))) {
                log_do("then 1");
            } else {
            }
            if (((Boolean) e._parameters.get("b"))) {
            } else {
                log_do("else 1");
            }
            if (((Boolean) e._parameters.get("b"))) {
                log_do("then 2");
            } else {
                log_do("else 2");
            }
            if (((Boolean) e._parameters.get("b"))) {
                BranchCompartment compartment =  new BranchCompartment(BranchState.F1.getValue());
                
                this._transition_(compartment);
            } else {
                BranchCompartment compartment =  new BranchCompartment(BranchState.F2.getValue());
                
                this._transition_(compartment);
            }
            
            return;
        }
        else if(e._message == "OnInt") {
            if (((int) e._parameters.get("i")) > 5) {
                log_do("> 5");
            } else {
                log_do("<= 5");
            }
            if (((int) e._parameters.get("i")) < 10) {
                log_do("< 10");
            } else {
                log_do(">= 10");
            }
            if (((int) e._parameters.get("i")) == 7) {
                log_do("== 7");
                BranchCompartment compartment =  new BranchCompartment(BranchState.F1.getValue());
                
                this._transition_(compartment);
            } else {
                log_do("!= 7");
                BranchCompartment compartment =  new BranchCompartment(BranchState.F2.getValue());
                
                this._transition_(compartment);
            }
            
            return;
        }
    }
    
    private void _sNegatedIf_(FrameEvent e) {
        if(e._message == "OnBool") {
            if (!(((Boolean) e._parameters.get("b")))) {
                log_do("then 1");
            } else {
            }
            if (!(((Boolean) e._parameters.get("b")))) {
            } else {
                log_do("else 1");
            }
            if (!(((Boolean) e._parameters.get("b")))) {
                log_do("then 2");
            } else {
                log_do("else 2");
            }
            if (!(((Boolean) e._parameters.get("b")))) {
                BranchCompartment compartment =  new BranchCompartment(BranchState.F1.getValue());
                
                this._transition_(compartment);
            } else {
                BranchCompartment compartment =  new BranchCompartment(BranchState.F2.getValue());
                
                this._transition_(compartment);
            }
            
            return;
        }
        else if(e._message == "OnInt") {
            if (((int) e._parameters.get("i")) < 5) {
                log_do("< 5");
            } else {
                log_do(">= 5");
            }
            if (((int) e._parameters.get("i")) > 10) {
                log_do("> 10");
            } else {
                log_do("<= 10");
            }
            if (((int) e._parameters.get("i")) == 7) {
                log_do("== 7");
                BranchCompartment compartment =  new BranchCompartment(BranchState.F1.getValue());
                
                this._transition_(compartment);
            } else {
                log_do("!= 7");
                BranchCompartment compartment =  new BranchCompartment(BranchState.F2.getValue());
                
                this._transition_(compartment);
            }
            
            return;
        }
    }
    
    private void _sPrecedence_(FrameEvent e) {
        if(e._message == "OnInt") {
            if (-((int) e._parameters.get("i")) >= 0 && -((int) e._parameters.get("i")) <= 5) {
                log_do("then 1");
            } else {
                log_do("else 1");
            }
            if ((((int) e._parameters.get("i")) > -5 && ((int) e._parameters.get("i")) > 5) && (((int) e._parameters.get("i")) > -10 && ((int) e._parameters.get("i")) < 10)) {
                log_do("then 2");
            } else {
                log_do("else 2");
            }
            if (((int) e._parameters.get("i")) >= 0 && ((int) e._parameters.get("i")) <= 5 || ((int) e._parameters.get("i")) >= 10 && ((int) e._parameters.get("i")) <= 20) {
                log_do("then 3");
            } else {
                log_do("else 3");
            }
            if ((((int) e._parameters.get("i")) >= 0 && ((int) e._parameters.get("i")) < 10) && ((int) e._parameters.get("i")) + 5 < 20) {
                log_do("then 4");
            } else {
                log_do("else 4");
            }
            
            return;
        }
    }
    
    private void _sNestedIf_(FrameEvent e) {
        if(e._message == "OnInt") {
            if (((int) e._parameters.get("i")) > 0) {
                log_do("> 0");
                if (((int) e._parameters.get("i")) < 100) {
                    log_do("< 100");
                    BranchCompartment compartment =  new BranchCompartment(BranchState.F1.getValue());
                    
                    this._transition_(compartment);
                } else {
                    log_do(">= 100");
                }
            } else {
                log_do("<= 0");
                if (((int) e._parameters.get("i")) > -10) {
                    log_do("> -10");
                } else {
                    log_do("<= -10");
                    BranchCompartment compartment =  new BranchCompartment(BranchState.F2.getValue());
                    
                    this._transition_(compartment);
                }
            }
            
            return;
        }
    }
    
    private void _sGuardedTransition_(FrameEvent e) {
        if(e._message == "OnInt") {
            if (((int) e._parameters.get("i")) > 100) {
                log_do("-> $F1");
                BranchCompartment compartment =  new BranchCompartment(BranchState.F1.getValue());
                
                this._transition_(compartment);
                
                return;
            } else {
            }
            if (((int) e._parameters.get("i")) < 10) {
            } else {
                log_do("-> $F2");
                BranchCompartment compartment =  new BranchCompartment(BranchState.F2.getValue());
                
                this._transition_(compartment);
                
                return;
            }
            log_do("-> $F3");
            BranchCompartment compartment =  new BranchCompartment(BranchState.F3.getValue());
            
            this._transition_(compartment);
            
            return;
        }
    }
    
    private void _sNestedGuardedTransition_(FrameEvent e) {
        if(e._message == "OnInt") {
            if (((int) e._parameters.get("i")) > 10) {
                if (((int) e._parameters.get("i")) > 100) {
                    log_do("-> $F1");
                    BranchCompartment compartment =  new BranchCompartment(BranchState.F1.getValue());
                    
                    this._transition_(compartment);
                    
                    return;
                } else {
                }
                if (((int) e._parameters.get("i")) > 50) {
                } else {
                    log_do("-> $F2");
                    BranchCompartment compartment =  new BranchCompartment(BranchState.F2.getValue());
                    
                    this._transition_(compartment);
                    
                    return;
                }
            } else {
            }
            log_do("-> $F3");
            BranchCompartment compartment =  new BranchCompartment(BranchState.F3.getValue());
            
            this._transition_(compartment);
            
            return;
        }
    }
    
    private void _sF1_(FrameEvent e) {
    }
    
    private void _sF2_(FrameEvent e) {
    }
    
    private void _sF3_(FrameEvent e) {
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void log_do(String msg) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> tape  = new ArrayList<String>();;
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(BranchCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(BranchCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }
        
}

//=============== Compartment ==============//

class BranchCompartment {

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
    BranchCompartment(){
    
    }
    BranchCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
    
    public BranchCompartment(int state, HashMap<String, Object> stateArgs, HashMap<String, Object> stateVars,
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

class BranchController extends Branch {

	BranchController() {
	  super();
	}

    protected void log_do(String msg) {}
}

********************/

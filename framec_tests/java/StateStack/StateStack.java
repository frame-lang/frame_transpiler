// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.StateStack;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class StateStack {
    
    private StateStackCompartment _compartment_;
    private StateStackCompartment _nextCompartment_;
    
    
    
    StateStack() {
        
        // Create state stack.
        
        this._stateStack_ = new Stack<StateStackCompartment>();
        
        // Create and intialize start state compartment.
        this._compartment_ = new StateStackCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum StateStackState {
        
        A(0),
        B(1),
        C(2);
        
        public final int value;
        
        private StateStackState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == StateStackState.A.getValue()) {
            this._sA_(e);
        }else if(this._compartment_.state == StateStackState.B.getValue()) {
            this._sB_(e);
        }else if(this._compartment_.state == StateStackState.C.getValue()) {
            this._sC_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            StateStackCompartment nextCompartment = this._nextCompartment_;
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
    
    public void to_a() {
        FrameEvent e = new FrameEvent("to_a", null);
        this._mux_(e);
    }
    
    public void to_b() {
        FrameEvent e = new FrameEvent("to_b", null);
        this._mux_(e);
    }
    
    public void to_c() {
        FrameEvent e = new FrameEvent("to_c", null);
        this._mux_(e);
    }
    
    public void push() {
        FrameEvent e = new FrameEvent("push", null);
        this._mux_(e);
    }
    
    public void pop() {
        FrameEvent e = new FrameEvent("pop", null);
        this._mux_(e);
    }
    
    public void pop_change() {
        FrameEvent e = new FrameEvent("pop_change", null);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sA_(FrameEvent e) {
        if(e._message == ">") {
            log_do("A:>");
            
            return;
        }
        else if(e._message == "<") {
            log_do("A:<");
            
            return;
        }
        else if(e._message == "to_a") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.A.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_b") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.B.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_c") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.C.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "push") {
            _stateStack_push_(this._compartment_);
            
            return;
        }
        else if(e._message == "pop") {
            StateStackCompartment compartment = this._stateStack_pop_();
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "pop_change") {
            StateStackCompartment compartment = this._stateStack_pop_();
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sB_(FrameEvent e) {
        if(e._message == ">") {
            log_do("B:>");
            
            return;
        }
        else if(e._message == "<") {
            log_do("B:<");
            
            return;
        }
        else if(e._message == "to_a") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.A.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_b") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.B.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_c") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.C.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "push") {
            _stateStack_push_(this._compartment_);
            
            return;
        }
        else if(e._message == "pop") {
            StateStackCompartment compartment = this._stateStack_pop_();
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "pop_change") {
            StateStackCompartment compartment = this._stateStack_pop_();
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sC_(FrameEvent e) {
        if(e._message == ">") {
            log_do("C:>");
            
            return;
        }
        else if(e._message == "<") {
            log_do("C:<");
            
            return;
        }
        else if(e._message == "to_a") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.A.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_b") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.B.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_c") {
            StateStackCompartment compartment =  new StateStackCompartment(StateStackState.C.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "push") {
            _stateStack_push_(this._compartment_);
            
            return;
        }
        else if(e._message == "pop") {
            StateStackCompartment compartment = this._stateStack_pop_();
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "pop_change") {
            StateStackCompartment compartment = this._stateStack_pop_();
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
    
    private void _transition_(StateStackCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(StateStackCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    private Stack<StateStackCompartment> _stateStack_ = new Stack<>();
    
    private void _stateStack_push_(StateStackCompartment compartment) {
        _stateStack_.push(compartment);
    }
    
    private StateStackCompartment _stateStack_pop_() {
        return _stateStack_.pop();
    }
    
    private void _changeState_(StateStackCompartment compartment) {
        this._compartment_ = compartment;
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }
        
}

//=============== Compartment ==============//

class StateStackCompartment {

    int state;
    
    StateStackCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
}


/********************

class StateStackController extends StateStack {

	StateStackController() {
	  super();
	}

    protected void log_do(String msg) {}
}

********************/



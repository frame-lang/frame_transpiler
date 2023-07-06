// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.StateContextStack;
import java.util.*;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonMappingException;
import com.fasterxml.jackson.databind.ObjectMapper;

import framec_tests.java.FrameLang.FrameEvent;

class StateContextStack {
    
    private StateContextStackCompartment _compartment_;
    private StateContextStackCompartment _nextCompartment_;
    
    
    
    StateContextStack() {
        
        // Create state stack.
        
        this._stateStack_ = new Stack<StateContextStackCompartment>();
        
        // Create and intialize start state compartment.
        this._compartment_ = new StateContextStackCompartment(this._state_);
        this._nextCompartment_ = null;
        this._compartment_.stateVars.put("x", 0);
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum StateContextStackState {
        
        A(0),
        B(1),
        C(2);
        
        public final int value;
        
        private StateContextStackState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }

    // public void push(StateContextStackCompartment compartment) {
    //     StateContextStackCompartment copyCompartment = deepCopyCompartment(_compartment_);
    //     _stateStack_.push(copyCompartment);
    // }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == StateContextStackState.A.getValue()) {
            this._sA_(e);
        }else if(this._compartment_.state == StateContextStackState.B.getValue()) {
            this._sB_(e);
        }else if(this._compartment_.state == StateContextStackState.C.getValue()) {
            this._sC_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            StateContextStackCompartment nextCompartment = this._nextCompartment_;
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
    
    public void inc() {
        FrameEvent e = new FrameEvent("inc", null);
        this._mux_(e);
    }
    
    public int value() {
        FrameEvent e = new FrameEvent("value", null);
        this._mux_(e);
        return (int) e._return;
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
        else if(e._message == "inc") {
            this._compartment_.stateVars.put("x",(int) this._compartment_.stateVars.get("x") + 1);
            
            return;
        }
        else if(e._message == "value") {
            e._return = ((int) this._compartment_.stateVars.get("x"));
            
            return;
            
        }
        else if(e._message == "to_a") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.A.getValue());
            compartment.stateVars.put("x", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_b") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.B.getValue());
            compartment.stateVars.put("y", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_c") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.C.getValue());
            compartment.stateVars.put("z", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "push") {
            _stateStack_push_(this._compartment_);
            
            return;
        }
        else if(e._message == "pop") {
            StateContextStackCompartment compartment = this._stateStack_pop_();
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "pop_change") {
            StateContextStackCompartment compartment = this._stateStack_pop_();
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
        else if(e._message == "inc") {
            this._compartment_.stateVars.put("y",(int) this._compartment_.stateVars.get("y") + 5);
            
            return;
        }
        else if(e._message == "value") {
            e._return = ((int) this._compartment_.stateVars.get("y"));
            
            return;
            
        }
        else if(e._message == "to_a") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.A.getValue());
            compartment.stateVars.put("x", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_b") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.B.getValue());
            compartment.stateVars.put("y", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_c") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.C.getValue());
            compartment.stateVars.put("z", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "push") {
            _stateStack_push_(this._compartment_);
            
            return;
        }
        else if(e._message == "pop") {
            StateContextStackCompartment compartment = this._stateStack_pop_();
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "pop_change") {
            StateContextStackCompartment compartment = this._stateStack_pop_();
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
        else if(e._message == "inc") {
            this._compartment_.stateVars.put("z",(int) this._compartment_.stateVars.get("z") + 10);
            
            return;
        }
        else if(e._message == "value") {
            e._return = ((int) this._compartment_.stateVars.get("z"));
            
            return;
            
        }
        else if(e._message == "to_a") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.A.getValue());
            compartment.stateVars.put("x", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_b") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.B.getValue());
            compartment.stateVars.put("y", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_c") {
            StateContextStackCompartment compartment =  new StateContextStackCompartment(StateContextStackState.C.getValue());
            compartment.stateVars.put("z", 0);
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "push") {
            _stateStack_push_(this._compartment_);
            
            return;
        }
        else if(e._message == "pop") {
            StateContextStackCompartment compartment = this._stateStack_pop_();
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "pop_change") {
            StateContextStackCompartment compartment = this._stateStack_pop_();
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
    
    private void _transition_(StateContextStackCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(StateContextStackCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    private Stack<StateContextStackCompartment> _stateStack_ = new Stack<>();
    
    public void _stateStack_push_(StateContextStackCompartment compartment){
        StateContextStackCompartment copyCompartment = deepCopyCompartment(_compartment_);
        this._stateStack_.push(copyCompartment);
    }
    
    private StateContextStackCompartment _stateStack_pop_() {
        return _stateStack_.pop();
    }
    
    private StateContextStackCompartment deepCopyCompartment(StateContextStackCompartment c){
    
        //Create a new compartment to hold a deep copy
        ObjectMapper mapper = new ObjectMapper();
        StateContextStackCompartment copyCompartment;
        try {
            copyCompartment = mapper.readValue(mapper.writeValueAsString(c), StateContextStackCompartment.class);
            return copyCompartment;
        } catch (JsonMappingException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
            throw new UnsupportedOperationException();
        } catch (JsonProcessingException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
            throw new UnsupportedOperationException();
        }
       
    }

    private void _changeState_(StateContextStackCompartment compartment) {
        this._compartment_ = compartment;
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }  
        
}

//=============== Compartment ==============//

class StateContextStackCompartment {


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
    StateContextStackCompartment(){

    }
    StateContextStackCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();

    
    public StateContextStackCompartment(int state, HashMap<String, Object> stateArgs, HashMap<String, Object> stateVars,
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

class StateContextStackController extends StateContextStack {

	StateContextStackController() {
	  super();
	}

    protected void log_do(String msg) {}
}

********************/


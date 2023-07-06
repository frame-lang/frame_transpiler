// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.StateParams;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class StateParams {
    
    private StateParamsCompartment _compartment_;
    private StateParamsCompartment _nextCompartment_;
    
    
    
    StateParams() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new StateParamsCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum StateParamsState {
        
        INIT(0),
        SPLIT(1),
        MERGE(2);
        
        public final int value;
        
        private StateParamsState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == StateParamsState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == StateParamsState.SPLIT.getValue()) {
            this._sSplit_(e);
        }else if(this._compartment_.state == StateParamsState.MERGE.getValue()) {
            this._sMerge_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            StateParamsCompartment nextCompartment = this._nextCompartment_;
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
    
    public void Prev() {
        FrameEvent e = new FrameEvent("Prev", null);
        this._mux_(e);
    }
    
    public void Log() {
        FrameEvent e = new FrameEvent("Log", null);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sInit_(FrameEvent e) {
        if(e._message == "Next") {
            StateParamsCompartment compartment =  new StateParamsCompartment(StateParamsState.SPLIT.getValue());
            compartment.stateArgs.put("val", 1);
            
            this._transition_(compartment);
            return;
        }
    }
    
    private void _sSplit_(FrameEvent e) {
        if(e._message == "Next") {
            StateParamsCompartment compartment =  new StateParamsCompartment(StateParamsState.MERGE.getValue());
            compartment.stateArgs.put("left", (int) this._compartment_.stateArgs.get("val"));
            compartment.stateArgs.put("right", (int) this._compartment_.stateArgs.get("val") + 1);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "Prev") {
            StateParamsCompartment compartment =  new StateParamsCompartment(StateParamsState.MERGE.getValue());
            compartment.stateArgs.put("left", (int) this._compartment_.stateArgs.get("val") + 1);
            compartment.stateArgs.put("right", (int) this._compartment_.stateArgs.get("val"));
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "Log") {
            got_param_do("val",((int) this._compartment_.stateArgs.get("val")));
            return;
        }
    }
    
    private void _sMerge_(FrameEvent e) {
        if(e._message == "Next") {
            StateParamsCompartment compartment =  new StateParamsCompartment(StateParamsState.SPLIT.getValue());
            compartment.stateArgs.put("val", (int) this._compartment_.stateArgs.get("left") + (int) this._compartment_.stateArgs.get("right"));
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "Prev") {
            StateParamsCompartment compartment =  new StateParamsCompartment(StateParamsState.SPLIT.getValue());
            compartment.stateArgs.put("val", (int) this._compartment_.stateArgs.get("left") * (int) this._compartment_.stateArgs.get("right"));
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "Log") {
            got_param_do("left",((int) this._compartment_.stateArgs.get("left")));
            got_param_do("right",((int) this._compartment_.stateArgs.get("right")));
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void got_param_do(String name, int val) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> param_log  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(StateParamsCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(StateParamsCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
            }
            
    }
    
    //=============== Compartment ==============//
    
    class StateParamsCompartment {
    
        int state;
        
        StateParamsCompartment(int state) {
            this.state = state;
        }
        
        HashMap<String, Object> stateArgs = new HashMap<String, Object>();
        HashMap<String, Object> stateVars = new HashMap<String, Object>();
        HashMap<String, Object> enterArgs = new HashMap<String, Object>();
        HashMap<String, Object> exitArgs = new HashMap<String, Object>();
        FrameEvent _forwardEvent = new FrameEvent();
    }
    
    
    /********************

    class StateParamsController extends StateParams {

    	StateParamsController() {
    	  super();
    	}
    
    protected void got_param_do(String name, int val) {}
    }
    
********************/
    

// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.StateContext;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class StateContextSm {
    
    private StateContextSmCompartment _compartment_;
    private StateContextSmCompartment _nextCompartment_;
    
    
    
    StateContextSm() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new StateContextSmCompartment(this._state_);
        this._nextCompartment_ = null;
        this._compartment_.stateVars.put("w", 0);
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum StateContextSmState {
        
        INIT(0),
        FOO(1),
        BAR(2);
        
        public final int value;
        
        private StateContextSmState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == StateContextSmState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == StateContextSmState.FOO.getValue()) {
            this._sFoo_(e);
        }else if(this._compartment_.state == StateContextSmState.BAR.getValue()) {
            this._sBar_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            StateContextSmCompartment nextCompartment = this._nextCompartment_;
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
    
    public void Start() {
        FrameEvent e = new FrameEvent("Start", null);
        this._mux_(e);
    }
    
    public void LogState() {
        FrameEvent e = new FrameEvent("LogState", null);
        this._mux_(e);
    }
    
    public int Inc() {
        FrameEvent e = new FrameEvent("Inc", null);
        this._mux_(e);
        return (int) e._return;
    }
    
    public void Next(int arg) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("arg", arg);

        FrameEvent e = new FrameEvent("Next", parameters);
        this._mux_(e);
    }
    
    public void Change(int arg) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("arg", arg);

        FrameEvent e = new FrameEvent("Change", parameters);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sInit_(FrameEvent e) {
        if(e._message == ">") {
            this._compartment_.stateVars.put("w",3);
            log_do("w",((int) this._compartment_.stateVars.get("w")));
            
            return;
        }
        else if(e._message == "Inc") {
            this._compartment_.stateVars.put("w",(int) this._compartment_.stateVars.get("w") + 1);
            log_do("w",((int) this._compartment_.stateVars.get("w")));
            e._return = ((int) this._compartment_.stateVars.get("w"));
            
            return;
            
        }
        else if(e._message == "LogState") {
            log_do("w",((int) this._compartment_.stateVars.get("w")));
            
            return;
        }
        else if(e._message == "Start") {
            StateContextSmCompartment compartment =  new StateContextSmCompartment(StateContextSmState.FOO.getValue());
            compartment.enterArgs.put("a", 3);
            compartment.enterArgs.put("b", (int) this._compartment_.stateVars.get("w"));
            compartment.stateVars.put("x", 0);
            
            this._transition_(compartment);
            
            return;
        }
    }
    
    private void _sFoo_(FrameEvent e) {
        if(e._message == ">") {
            log_do("a",((int) e._parameters.get("a")));
            log_do("b",((int) e._parameters.get("b")));
            this._compartment_.stateVars.put("x",(int) e._parameters.get("a") * (int) e._parameters.get("b"));
            log_do("x",((int) this._compartment_.stateVars.get("x")));
            
            return;
        }
        else if(e._message == "<") {
            log_do("c",((int) e._parameters.get("c")));
            this._compartment_.stateVars.put("x",(int) this._compartment_.stateVars.get("x") + (int) e._parameters.get("c"));
            log_do("x",((int) this._compartment_.stateVars.get("x")));
            
            return;
        }
        else if(e._message == "LogState") {
            log_do("x",((int) this._compartment_.stateVars.get("x")));
            
            return;
        }
        else if(e._message == "Inc") {
            this._compartment_.stateVars.put("x",(int) this._compartment_.stateVars.get("x") + 1);
            log_do("x",((int) this._compartment_.stateVars.get("x")));
            e._return = ((int) this._compartment_.stateVars.get("x"));
            
            return;
            
        }
        else if(e._message == "Next") {
            int tmp  = (int) e._parameters.get("arg") * 10;
            this._compartment_.exitArgs.put("c", 10);
            StateContextSmCompartment compartment =  new StateContextSmCompartment(StateContextSmState.BAR.getValue());
            compartment.enterArgs.put("a", tmp);
            compartment.stateArgs.put("y", (int) this._compartment_.stateVars.get("x"));
            compartment.stateVars.put("z", 0);
            
            this._transition_(compartment);
            
            return;
        }
          //  FIXME: Swapping this to 10 * arg causes a parse error!
        else if(e._message == "Change") {
            int tmp  = (int) this._compartment_.stateVars.get("x") + (int) e._parameters.get("arg");
            StateContextSmCompartment compartment =  new StateContextSmCompartment(StateContextSmState.BAR.getValue());
            compartment.stateArgs.put("y", tmp);
            compartment.stateVars.put("z", 0);
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    private void _sBar_(FrameEvent e) {
        if(e._message == ">") {
            log_do("a",((int) e._parameters.get("a")));
            log_do("y",((int) this._compartment_.stateArgs.get("y")));
            this._compartment_.stateVars.put("z",(int) e._parameters.get("a") + (int) this._compartment_.stateArgs.get("y"));
            log_do("z",((int) this._compartment_.stateVars.get("z")));
            
            return;
        }
        else if(e._message == "LogState") {
            log_do("y",((int) this._compartment_.stateArgs.get("y")));
            log_do("z",((int) this._compartment_.stateVars.get("z")));
            
            return;
        }
        else if(e._message == "Inc") {
            this._compartment_.stateVars.put("z",(int) this._compartment_.stateVars.get("z") + 1);
            log_do("z",((int) this._compartment_.stateVars.get("z")));
            e._return = ((int) this._compartment_.stateVars.get("z"));
            
            return;
            
        }
        else if(e._message == "Change") {
            int tmp  = (int) this._compartment_.stateArgs.get("y") + (int) this._compartment_.stateVars.get("z") + (int) e._parameters.get("arg");
            log_do("tmp",tmp);
            StateContextSmCompartment compartment =  new StateContextSmCompartment(StateContextSmState.INIT.getValue());
            compartment.stateVars.put("w", 0);
            
            this._changeState_(compartment);
            
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void log_do(String name, int val) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> tape  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(StateContextSmCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(StateContextSmCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    private void _changeState_(StateContextSmCompartment compartment) {
        this._compartment_ = compartment;
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
            }
            
    }
    
    //=============== Compartment ==============//
    
    class StateContextSmCompartment {
    
        int state;
        
        StateContextSmCompartment(int state) {
            this.state = state;
        }
        
        HashMap<String, Object> stateArgs = new HashMap<String, Object>();
        HashMap<String, Object> stateVars = new HashMap<String, Object>();
        HashMap<String, Object> enterArgs = new HashMap<String, Object>();
        HashMap<String, Object> exitArgs = new HashMap<String, Object>();
        FrameEvent _forwardEvent = new FrameEvent();
    }
    
    
    /********************

    class StateContextSmController extends StateContextSm {

    	StateContextSmController() {
    	  super();
    	}
    
    protected void log_do(String name, int val) {}
    }
    
********************/
    

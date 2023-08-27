// emitted from framec_v0.11.0
// get include files at https://github.com/frame-lang/frame-ancillary-files
package framec_tests.java.Event_handler;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class EventHandler {
    
    private EventHandlerCompartment _compartment_;
    private EventHandlerCompartment _nextCompartment_;
    
    
    
    EventHandler() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new EventHandlerCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum EventHandlerState {
        
        S1(0),
        S2(1);
        
        public final int value;
        
        private EventHandlerState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == EventHandlerState.S1.getValue()) {
            this._sS1_(e);
        }else if(this._compartment_.state == EventHandlerState.S2.getValue()) {
            this._sS2_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            EventHandlerCompartment nextCompartment = this._nextCompartment_;
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
    
    public void LogIt(int x) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("x", x);

        FrameEvent e = new FrameEvent("LogIt", parameters);
        this._mux_(e);
    }
    
    public void LogAdd(int a,int b) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("a", a);

        parameters.put("b", b);

        FrameEvent e = new FrameEvent("LogAdd", parameters);
        this._mux_(e);
    }
    
    public int LogReturn(int a,int b) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("a", a);

        parameters.put("b", b);

        FrameEvent e = new FrameEvent("LogReturn", parameters);
        this._mux_(e);
        return (int) e._return;
    }
    
    public void PassAdd(int a,int b) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("a", a);

        parameters.put("b", b);

        FrameEvent e = new FrameEvent("PassAdd", parameters);
        this._mux_(e);
    }
    
    public int PassReturn(int a,int b) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("a", a);

        parameters.put("b", b);

        FrameEvent e = new FrameEvent("PassReturn", parameters);
        this._mux_(e);
        return (int) e._return;
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sS1_(FrameEvent e) {
        if(e._message == "LogIt") {
            log_do("x",((int) e._parameters.get("x")));
            
            return;
        }
        else if(e._message == "LogAdd") {
            log_do("a",((int) e._parameters.get("a")));
            log_do("b",((int) e._parameters.get("b")));
            log_do("a+b",((int) e._parameters.get("a")) + ((int) e._parameters.get("b")));
            
            return;
        }
        else if(e._message == "LogReturn") {
            log_do("a",((int) e._parameters.get("a")));
            log_do("b",((int) e._parameters.get("b")));
            int r  = (int) e._parameters.get("a") + (int) e._parameters.get("b");
            log_do("r",r);
            e._return = r;
            
            return;
            
        }
        else if(e._message == "PassAdd") {
            EventHandlerCompartment compartment =  new EventHandlerCompartment(EventHandlerState.S2.getValue());
            compartment.stateArgs.put("p", (int) e._parameters.get("a") + (int) e._parameters.get("b"));
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "PassReturn") {
            int r  = (int) e._parameters.get("a") + (int) e._parameters.get("b");
            log_do("r",r);
            EventHandlerCompartment compartment =  new EventHandlerCompartment(EventHandlerState.S2.getValue());
            compartment.stateArgs.put("p", r);
            
            this._transition_(compartment);
            e._return = r;
            
            return;
            
        }
    }
    
    private void _sS2_(FrameEvent e) {
        if(e._message == ">") {
            log_do("p",((int) this._compartment_.stateArgs.get("p")));
            
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void log_do(String msg, int val) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> tape  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(EventHandlerCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(EventHandlerCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }
        
}

//=============== Compartment ==============//

class EventHandlerCompartment {

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
    EventHandlerCompartment(){
    
    }
    EventHandlerCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
    
    public EventHandlerCompartment(int state, HashMap<String, Object> stateArgs, HashMap<String, Object> stateVars,
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

class EventHandlerController extends EventHandler {

	EventHandlerController() {
	  super();
	}

    protected void log_do(String msg, int val) {}
}

********************/

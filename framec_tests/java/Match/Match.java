// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.Match;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class Match {
    
    private MatchCompartment _compartment_;
    private MatchCompartment _nextCompartment_;
    
    
    
    Match() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new MatchCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum MatchState {
        
        INIT(0),
        EMPTYMATCH(1),
        SIMPLEMATCH(2),
        MULTIMATCH(3),
        NESTEDMATCH(4),
        CHILDMATCH(5),
        FINAL(6);
        
        public final int value;
        
        private MatchState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == MatchState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == MatchState.EMPTYMATCH.getValue()) {
            this._sEmptyMatch_(e);
        }else if(this._compartment_.state == MatchState.SIMPLEMATCH.getValue()) {
            this._sSimpleMatch_(e);
        }else if(this._compartment_.state == MatchState.MULTIMATCH.getValue()) {
            this._sMultiMatch_(e);
        }else if(this._compartment_.state == MatchState.NESTEDMATCH.getValue()) {
            this._sNestedMatch_(e);
        }else if(this._compartment_.state == MatchState.CHILDMATCH.getValue()) {
            this._sChildMatch_(e);
        }else if(this._compartment_.state == MatchState.FINAL.getValue()) {
            this._sFinal_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            MatchCompartment nextCompartment = this._nextCompartment_;
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
    
    public void Empty() {
        FrameEvent e = new FrameEvent("Empty", null);
        this._mux_(e);
    }
    
    public void Simple() {
        FrameEvent e = new FrameEvent("Simple", null);
        this._mux_(e);
    }
    
    public void Multi() {
        FrameEvent e = new FrameEvent("Multi", null);
        this._mux_(e);
    }
    
    public void Nested() {
        FrameEvent e = new FrameEvent("Nested", null);
        this._mux_(e);
    }
    
    public void Child() {
        FrameEvent e = new FrameEvent("Child", null);
        this._mux_(e);
    }
    
    public void OnInt(int i) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("i", i);

        FrameEvent e = new FrameEvent("OnInt", parameters);
        this._mux_(e);
    }
    
    public void OnString(String s) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("s", s);

        FrameEvent e = new FrameEvent("OnString", parameters);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sInit_(FrameEvent e) {
        if(e._message == "Empty") {
            MatchCompartment compartment =  new MatchCompartment(MatchState.EMPTYMATCH.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "Simple") {
            MatchCompartment compartment =  new MatchCompartment(MatchState.SIMPLEMATCH.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "Multi") {
            MatchCompartment compartment =  new MatchCompartment(MatchState.MULTIMATCH.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "Nested") {
            MatchCompartment compartment =  new MatchCompartment(MatchState.NESTEDMATCH.getValue());
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "Child") {
            MatchCompartment compartment =  new MatchCompartment(MatchState.CHILDMATCH.getValue());
            
            this._transition_(compartment);
            
            return;
        }
    }
    
    private void _sEmptyMatch_(FrameEvent e) {
        if(e._message == "OnString") {
            if ((((String) e._parameters.get("s")) == "") || (((String) e._parameters.get("s")) == "foo")) {
                log_do("empty");
            } else {
                log_do("?");
            }
            
            return;
        }
    }  //  TODO: matching only the empty string is broken
    
    
    private void _sSimpleMatch_(FrameEvent e) {
        if(e._message == "OnInt") {
            if ((((int) e._parameters.get("i")) == 0)) {
                log_do("0");
            } else if ((((int) e._parameters.get("i")) == 42)) {
                log_do("42");
            } else if ((((int) e._parameters.get("i")) == 42)) {
                log_do("!!!");
            } else if ((((int) e._parameters.get("i")) == -200)) {
                log_do("-200");
            } else {
                log_do("?");
            }
            
            return;
        }
        else if(e._message == "OnString") {
            if ((((String) e._parameters.get("s")) == "hello")) {
                log_do("hello");
            } else if ((((String) e._parameters.get("s")) == "hello")) {
                log_do("!!!");
            } else if ((((String) e._parameters.get("s")) == "goodbye")) {
                log_do("goodbye");
            } else if ((((String) e._parameters.get("s")) == "Testing 1, 2, 3...")) {
                log_do("testing");
            } else if ((((String) e._parameters.get("s")) == "$10!")) {
                log_do("money");
            } else {
                log_do("?");
            }
            
            return;
        }
    }
    
    private void _sMultiMatch_(FrameEvent e) {
        if(e._message == "OnInt") {
            if ((((int) e._parameters.get("i")) == 3) || (((int) e._parameters.get("i")) == -7)) {
                log_do("3|-7");
            } else if ((((int) e._parameters.get("i")) == -4) || (((int) e._parameters.get("i")) == 5) || (((int) e._parameters.get("i")) == 6)) {
                log_do("-4|5|6");
            } else {
                log_do("?");
            }
            
            return;
        }
        else if(e._message == "OnString") {
            if ((((String) e._parameters.get("s")) == "$10") || (((String) e._parameters.get("s")) == "12.5%") || (((String) e._parameters.get("s")) == "@#*!")) {
                log_do("symbols");
            } else if ((((String) e._parameters.get("s")) == " ") || (((String) e._parameters.get("s")) == "  ") || (((String) e._parameters.get("s")) == "\t") || (((String) e._parameters.get("s")) == "\n")) {
                log_do("whitespace");
            } else {
                log_do("?");
            }
            
            return;
        }
    }
    
    private void _sNestedMatch_(FrameEvent e) {
        if(e._message == "OnInt") {
            if (((int) e._parameters.get("i")) > 0) {
                if ((((int) e._parameters.get("i")) == 1) || (((int) e._parameters.get("i")) == 2) || (((int) e._parameters.get("i")) == 3)) {
                    log_do("1-3");
                    if ((((int) e._parameters.get("i")) == 1)) {
                        log_do("1");
                    } else if ((((int) e._parameters.get("i")) == 2)) {
                        log_do("2");
                    } else {
                        log_do("3");
                    }
                } else if ((((int) e._parameters.get("i")) == 4) || (((int) e._parameters.get("i")) == 5)) {
                    log_do("4-5");
                    if (((int) e._parameters.get("i")) == 4) {
                        log_do("4");
                    } else {
                        log_do("5");
                    }
                } else {
                    log_do("too big");
                }
            } else {
                log_do("too small");
            }
            
            return;
        }
        else if(e._message == "OnString") {
            if ((((String) e._parameters.get("s")) == "hello") || (((String) e._parameters.get("s")) == "hola") || (((String) e._parameters.get("s")) == "bonjour")) {
                log_do("greeting");
                if ((((String) e._parameters.get("s")) == "hello")) {
                    log_do("English");
                } else if ((((String) e._parameters.get("s")) == "hola")) {
                    log_do("Spanish");
                } else {
                    log_do("French");
                }
            } else if ((((String) e._parameters.get("s")) == "goodbye") || (((String) e._parameters.get("s")) == "adios") || (((String) e._parameters.get("s")) == "au revoir")) {
                log_do("farewell");
                if ((((String) e._parameters.get("s")) == "goodbye")) {
                    log_do("English");
                } else if ((((String) e._parameters.get("s")) == "adios")) {
                    log_do("Spanish");
                } else {
                    log_do("French");
                }
            } else {
                log_do("?");
            }
            
            return;
        }
    }
    
    private void _sChildMatch_(FrameEvent e) {
        if(e._message == "OnInt") {
            if ((((int) e._parameters.get("i")) == 0)) {
                MatchCompartment compartment =  new MatchCompartment(MatchState.FINAL.getValue());
                
                this._transition_(compartment);
            } else if ((((int) e._parameters.get("i")) == 3)) {
                log_do("3");
            } else if ((((int) e._parameters.get("i")) == 4)) {
                log_do("4");
                
                return;
            } else if ((((int) e._parameters.get("i")) == 42)) {
                log_do("42 in child");
            } else if ((((int) e._parameters.get("i")) == 5)) {
                log_do("5");
                MatchCompartment compartment =  new MatchCompartment(MatchState.FINAL.getValue());
                
                this._transition_(compartment);
                
                return;
            } else {
                log_do("no match in child");
            }
            
        }
        else if(e._message == "OnString") {
            if ((((String) e._parameters.get("s")) == "hello")) {
                log_do("hello in child");
            } else if ((((String) e._parameters.get("s")) == "goodbye")) {
                MatchCompartment compartment =  new MatchCompartment(MatchState.FINAL.getValue());
                
                this._transition_(compartment);
            } else if ((((String) e._parameters.get("s")) == "Testing 1, 2, 3...")) {
                log_do("testing in child");
                
                return;
            } else {
                log_do("no match in child");
            }
            
        }
        _sSimpleMatch_(e);
        
    }
    
    private void _sFinal_(FrameEvent e) {
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void log_do(String msg) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> tape  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(MatchCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(MatchCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }
        
}

//=============== Compartment ==============//

class MatchCompartment {

    int state;
    
    MatchCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
}


/********************

class MatchController extends Match {

	MatchController() {
	  super();
	}

    protected void log_do(String msg) {}
}

********************/


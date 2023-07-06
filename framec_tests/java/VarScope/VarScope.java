// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.VarScope;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class VarScope {
    
    private VarScopeCompartment _compartment_;
    private VarScopeCompartment _nextCompartment_;
    
    
    
    VarScope() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new VarScopeCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum VarScopeState {
        
        INIT(0),
        NN(1),
        NY(2),
        YN(3),
        YY(4);
        
        public final int value;
        
        private VarScopeState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == VarScopeState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == VarScopeState.NN.getValue()) {
            this._sNN_(e);
        }else if(this._compartment_.state == VarScopeState.NY.getValue()) {
            this._sNY_(e);
        }else if(this._compartment_.state == VarScopeState.YN.getValue()) {
            this._sYN_(e);
        }else if(this._compartment_.state == VarScopeState.YY.getValue()) {
            this._sYY_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            VarScopeCompartment nextCompartment = this._nextCompartment_;
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
    
    public void to_nn() {
        FrameEvent e = new FrameEvent("to_nn", null);
        this._mux_(e);
    }
    
    public void to_ny() {
        FrameEvent e = new FrameEvent("to_ny", null);
        this._mux_(e);
    }
    
    public void to_yn() {
        FrameEvent e = new FrameEvent("to_yn", null);
        this._mux_(e);
    }
    
    public void to_yy() {
        FrameEvent e = new FrameEvent("to_yy", null);
        this._mux_(e);
    }
    
    public void nn(String d) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("d", d);

        FrameEvent e = new FrameEvent("nn", parameters);
        this._mux_(e);
    }
    
    public void ny(String d) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("d", d);

        FrameEvent e = new FrameEvent("ny", parameters);
        this._mux_(e);
    }
    
    public void yn(String d,String x) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("d", d);

        parameters.put("x", x);

        FrameEvent e = new FrameEvent("yn", parameters);
        this._mux_(e);
    }
    
    public void yy(String d,String x) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("d", d);

        parameters.put("x", x);

        FrameEvent e = new FrameEvent("yy", parameters);
        this._mux_(e);
    }
    
    public void sigils(String x) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("x", x);

        FrameEvent e = new FrameEvent("sigils", parameters);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sInit_(FrameEvent e) {
        if(e._message == "to_nn") {
            VarScopeCompartment compartment =  new VarScopeCompartment(VarScopeState.NN.getValue());
            compartment.stateArgs.put("b", "$NN[b]");
            compartment.stateVars.put("c", "$NN.c");
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_ny") {
            VarScopeCompartment compartment =  new VarScopeCompartment(VarScopeState.NY.getValue());
            compartment.stateArgs.put("b", "$NY[b]");
            compartment.stateVars.put("c", "$NY.c");
            compartment.stateVars.put("x", "$NY.x");
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_yn") {
            VarScopeCompartment compartment =  new VarScopeCompartment(VarScopeState.YN.getValue());
            compartment.stateArgs.put("b", "$YN[b]");
            compartment.stateArgs.put("x", "$YN[x]");
            compartment.stateVars.put("c", "$YN.c");
            
            this._transition_(compartment);
            
            return;
        }
        else if(e._message == "to_yy") {
            VarScopeCompartment compartment =  new VarScopeCompartment(VarScopeState.YY.getValue());
            compartment.stateArgs.put("b", "$YY[b]");
            compartment.stateArgs.put("x", "$YY[x]");
            compartment.stateVars.put("c", "$YY.c");
            compartment.stateVars.put("x", "$YY.x");
            
            this._transition_(compartment);
            
            return;
        }
    }
    
    private void _sNN_(FrameEvent e) {
        if(e._message == "nn") {
            String et  = "|nn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(this.x);
            
            return;
        }
        else if(e._message == "ny") {
            String et  = "|ny|.e";
            String x  = "|ny|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "yn") {
            String et  = "|yn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(((String) e._parameters.get("x")));
            
            return;
        }
        else if(e._message == "yy") {
            String et  = "|yy|.e";
            String x  = "|yy|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "sigils") {
            log_do(this.x);
            
            return;
        }
    }  //  var x:String = "|sigils|.x"
      //  log(||[x])
      //  log(||.x)
    
    
    private void _sNY_(FrameEvent e) {
        if(e._message == "nn") {
            String et  = "|nn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(((String) this._compartment_.stateVars.get("x")));
            
            return;
        }
        else if(e._message == "ny") {
            String et  = "|ny|.e";
            String x  = "|ny|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "yn") {
            String et  = "|yn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(((String) e._parameters.get("x")));
            
            return;
        }
        else if(e._message == "yy") {
            String et  = "|yy|.e";
            String x  = "|yy|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "sigils") {
            log_do(this.x);
            
            return;
        }
    }  //  var x:String = "|sigils|.x"
      //  log($.x)
      //  log(||[x])
      //  log(||.x)
    
    
    private void _sYN_(FrameEvent e) {
        if(e._message == "nn") {
            String et  = "|nn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(((String) this._compartment_.stateArgs.get("x")));
            
            return;
        }
        else if(e._message == "ny") {
            String et  = "|ny|.e";
            String x  = "|ny|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "yn") {
            String et  = "|yn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(((String) e._parameters.get("x")));
            
            return;
        }
        else if(e._message == "yy") {
            String et  = "|yy|.e";
            String x  = "|yy|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "sigils") {
            log_do(this.x);
            
            return;
        }
    }  //  var x:String = "|sigils|.x"
      //  log($[x])
      //  log(||[x])
      //  log(||.x)
    
    
    private void _sYY_(FrameEvent e) {
        if(e._message == "nn") {
            String et  = "|nn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(((String) this._compartment_.stateVars.get("x")));
            
            return;
        }
        else if(e._message == "ny") {
            String et  = "|ny|.e";
            String x  = "|ny|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "yn") {
            String et  = "|yn|.e";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(((String) e._parameters.get("x")));
            
            return;
        }
        else if(e._message == "yy") {
            String et  = "|yy|.e";
            String x  = "|yy|.x";
            log_do(this.a);
            log_do(((String) this._compartment_.stateArgs.get("b")));
            log_do(((String) this._compartment_.stateVars.get("c")));
            log_do(((String) e._parameters.get("d")));
            log_do(et);
            log_do(x);
            
            return;
        }
        else if(e._message == "sigils") {
            log_do(this.x);
            
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void log_do(String s) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public String a  = "#.a";
    public String x  = "#.x";
    public ArrayList<String> tape  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(VarScopeCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(VarScopeCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
        }
        
}

//=============== Compartment ==============//

class VarScopeCompartment {

    int state;
    
    VarScopeCompartment(int state) {
        this.state = state;
    }
    
    HashMap<String, Object> stateArgs = new HashMap<String, Object>();
    HashMap<String, Object> stateVars = new HashMap<String, Object>();
    HashMap<String, Object> enterArgs = new HashMap<String, Object>();
    HashMap<String, Object> exitArgs = new HashMap<String, Object>();
    FrameEvent _forwardEvent = new FrameEvent();
}


/********************

class VarScopeController extends VarScope {

	VarScopeController() {
	  super();
	}

    protected void log_do(String s) {}
}

********************/


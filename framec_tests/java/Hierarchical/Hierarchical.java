// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.Hierarchical;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class Hierarchical {
    
    private HierarchicalCompartment _compartment_;
    private HierarchicalCompartment _nextCompartment_;
    
    
    
    Hierarchical() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new HierarchicalCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum HierarchicalState {
        
        I(0),
        S(1),
        S0(2),
        S1(3),
        S2(4),
        S3(5),
        T(6);
        
        public final int value;
        
        private HierarchicalState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == HierarchicalState.I.getValue()) {
            this._sI_(e);
        }else if(this._compartment_.state == HierarchicalState.S.getValue()) {
            this._sS_(e);
        }else if(this._compartment_.state == HierarchicalState.S0.getValue()) {
            this._sS0_(e);
        }else if(this._compartment_.state == HierarchicalState.S1.getValue()) {
            this._sS1_(e);
        }else if(this._compartment_.state == HierarchicalState.S2.getValue()) {
            this._sS2_(e);
        }else if(this._compartment_.state == HierarchicalState.S3.getValue()) {
            this._sS3_(e);
        }else if(this._compartment_.state == HierarchicalState.T.getValue()) {
            this._sT_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            HierarchicalCompartment nextCompartment = this._nextCompartment_;
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
    
    
    //===================== Machine Block ===================//
    
    private void _sI_(FrameEvent e) {
        if(e._message == ">") {
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S.getValue());
            
            this._transition_(compartment);
            return;
        }
    }
    
    private void _sS_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S");
            return;
        }
        else if(e._message == "<") {
            exit_do("S");
            return;
        }
        else if(e._message == "A") {
            log_do("S.A");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S0.getValue());
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "B") {
            log_do("S.B");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S1.getValue());
            
            this._transition_(compartment);
            return;
        }
    }
    
    private void _sS0_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S0");
            
        }
        else if(e._message == "<") {
            exit_do("S0");
            
        }
          //  override parent handler
        else if(e._message == "A") {
            log_do("S0.A");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.T.getValue());
            
            this._transition_(compartment);
            return;
        }
          //  do this, then parent handler
        else if(e._message == "B") {
            log_do("S0.B");
            
        }
          //  extend parent handler
        else if(e._message == "C") {
            log_do("S0.C");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S2.getValue());
            
            this._transition_(compartment);
            return;
        }
        _sS_(e);
        
    }
    
    private void _sS1_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S1");
            return;
        }
        else if(e._message == "<") {
            exit_do("S1");
            return;
        }
          //  defer to parent for A
          //  do this, then parent, which transitions here
        else if(e._message == "B") {
            log_do("S1.B");
            
        }
          //  propagate message not handled by parent
        else if(e._message == "C") {
            log_do("S1.C");
            
        }
        _sS_(e);
        
    }
    
    private void _sS2_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S2");
            
        }
        else if(e._message == "<") {
            exit_do("S2");
            
        }
          //  will propagate to S0 and S
        else if(e._message == "B") {
            log_do("S2.B");
            
        }
        else if(e._message == "C") {
            log_do("S2.C");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.T.getValue());
            
            this._transition_(compartment);
            return;
        }
        _sS0_(e);
        
    }  //  continue after transition (should be ignored)
    
    
    private void _sS3_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("S3");
            
        }
        else if(e._message == "<") {
            exit_do("S3");
            
        }
          //  defer to grandparent for A
          //  override and move to sibling
        else if(e._message == "B") {
            log_do("S3.B");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S2.getValue());
            
            this._transition_(compartment);
            return;
        }
        _sS1_(e);
        
    }
    
    private void _sT_(FrameEvent e) {
        if(e._message == ">") {
            enter_do("T");
            return;
        }
        else if(e._message == "<") {
            exit_do("T");
            return;
        }
        else if(e._message == "A") {
            log_do("T.A");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S.getValue());
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "B") {
            log_do("T.B");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S2.getValue());
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "C") {
            log_do("T.C");
            HierarchicalCompartment compartment =  new HierarchicalCompartment(HierarchicalState.S3.getValue());
            
            this._transition_(compartment);
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void enter_do(String msg) { throw new UnsupportedOperationException(); }
    protected void exit_do(String msg) { throw new UnsupportedOperationException(); }
    protected void log_do(String msg) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public ArrayList<String> enters  = new ArrayList<String>();
    public ArrayList<String> exits  = new ArrayList<String>();
    public ArrayList<String> tape  = new ArrayList<String>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(HierarchicalCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(HierarchicalCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
            }
            
    }
    
    //=============== Compartment ==============//
    
    class HierarchicalCompartment {
    
        int state;
        
        HierarchicalCompartment(int state) {
            this.state = state;
        }
        
        HashMap<String, Object> stateArgs = new HashMap<String, Object>();
        HashMap<String, Object> stateVars = new HashMap<String, Object>();
        HashMap<String, Object> enterArgs = new HashMap<String, Object>();
        HashMap<String, Object> exitArgs = new HashMap<String, Object>();
        FrameEvent _forwardEvent = new FrameEvent();
    }
    
    
    /********************

    class HierarchicalController extends Hierarchical {

    	HierarchicalController() {
    	  super();
    	}
    
    protected void enter_do(String msg) {}
    
    protected void exit_do(String msg) {}
    
    protected void log_do(String msg) {}
    }
    
********************/
    
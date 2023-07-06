// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

package framec_tests.java.Naming;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;

class Naming {
    
    private NamingCompartment _compartment_;
    private NamingCompartment _nextCompartment_;
    
    
    
    Naming() {
        
        // Create and intialize start state compartment.
        this._compartment_ = new NamingCompartment(this._state_);
        this._nextCompartment_ = null;
        
        
        // Send system start event
        FrameEvent frame_event = new FrameEvent(">", null);
        this._mux_(frame_event);
    
    }
    
    // states enum
    private enum NamingState {
        
        INIT(0),
        SNAKE_STATE(1),
        CAMELSTATE(2),
        STATE123(3),
        FINAL(4);
        
        public final int value;
        
        private NamingState(int value) {
            this.value=value;
        }
        
        public int getValue() {
            return value;
        }
    }
    
    //====================== Multiplexer ====================//
    
    private void _mux_(FrameEvent e) {
        if(this._compartment_.state == NamingState.INIT.getValue()) {
            this._sInit_(e);
        }else if(this._compartment_.state == NamingState.SNAKE_STATE.getValue()) {
            this._ssnake_state_(e);
        }else if(this._compartment_.state == NamingState.CAMELSTATE.getValue()) {
            this._sCamelState_(e);
        }else if(this._compartment_.state == NamingState.STATE123.getValue()) {
            this._sstate123_(e);
        }else if(this._compartment_.state == NamingState.FINAL.getValue()) {
            this._sFinal_(e);
        }
        
        
        if(this._nextCompartment_ != null) {
            NamingCompartment nextCompartment = this._nextCompartment_;
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
    
    public void snake_event(int snake_param) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("snake_param", snake_param);

        FrameEvent e = new FrameEvent("snake_event", parameters);
        this._mux_(e);
    }
    
    public void CamelEvent(int CamelParam) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("CamelParam", CamelParam);

        FrameEvent e = new FrameEvent("CamelEvent", parameters);
        this._mux_(e);
    }
    
    public void event123(int param123) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("param123", param123);

        FrameEvent e = new FrameEvent("event123", parameters);
        this._mux_(e);
    }
    
    public void call(String event,int param) {
        HashMap<String,Object> parameters = new HashMap<String,Object>();
        parameters.put("event", event);

        parameters.put("param", param);

        FrameEvent e = new FrameEvent("call", parameters);
        this._mux_(e);
    }
    
    
    //===================== Machine Block ===================//
    
    private void _sInit_(FrameEvent e) {
        if(e._message == "snake_event") {
            NamingCompartment compartment =  new NamingCompartment(NamingState.SNAKE_STATE.getValue());
            compartment.stateArgs.put("snake_state_param", (int) e._parameters.get("snake_param"));
            compartment.stateVars.put("snake_state_var", this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 100);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "CamelEvent") {
            NamingCompartment compartment =  new NamingCompartment(NamingState.CAMELSTATE.getValue());
            compartment.stateArgs.put("CamelStateParam", (int) e._parameters.get("CamelParam"));
            compartment.stateVars.put("CamelStateVar", this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 200);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "event123") {
            NamingCompartment compartment =  new NamingCompartment(NamingState.STATE123.getValue());
            compartment.stateArgs.put("stateParam123", (int) e._parameters.get("param123"));
            compartment.stateVars.put("stateVar123", this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 300);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "call") {
            if (((String) e._parameters.get("event")) == "snake_event") {
                snake_event(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "CamelEvent") {
                CamelEvent(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "event123") {
                event123(((int) e._parameters.get("param")));
            } else {
            }
            return;
        }
    }
    
    private void _ssnake_state_(FrameEvent e) {
          //  1100
        if(e._message == "snake_event") {
            int snake_local_var  = (int) this._compartment_.stateVars.get("snake_state_var") + (int) this._compartment_.stateArgs.get("snake_state_param") + (int) e._parameters.get("snake_param");
            snake_action_do(snake_local_var);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", snake_local_var);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "CamelEvent") {
            int CamelLocalVar  = (int) this._compartment_.stateVars.get("snake_state_var") + (int) this._compartment_.stateArgs.get("snake_state_param") + (int) e._parameters.get("CamelParam");
            CamelAction_do(CamelLocalVar);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", CamelLocalVar);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "event123") {
            int localVar123  = (int) this._compartment_.stateVars.get("snake_state_var") + (int) this._compartment_.stateArgs.get("snake_state_param") + (int) e._parameters.get("param123");
            action123_do(localVar123);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", localVar123);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "call") {
            if (((String) e._parameters.get("event")) == "snake_event") {
                snake_event(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "CamelEvent") {
                CamelEvent(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "event123") {
                event123(((int) e._parameters.get("param")));
            } else {
            }
            return;
        }
    }
    
    private void _sCamelState_(FrameEvent e) {
          //  1200
        if(e._message == "snake_event") {
            int snake_local_var  = (int) this._compartment_.stateVars.get("CamelStateVar") + (int) this._compartment_.stateArgs.get("CamelStateParam") + (int) e._parameters.get("snake_param");
            snake_action_do(snake_local_var);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", snake_local_var);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "CamelEvent") {
            int CamelLocalVar  = (int) this._compartment_.stateVars.get("CamelStateVar") + (int) this._compartment_.stateArgs.get("CamelStateParam") + (int) e._parameters.get("CamelParam");
            CamelAction_do(CamelLocalVar);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", CamelLocalVar);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "event123") {
            int localVar123  = (int) this._compartment_.stateVars.get("CamelStateVar") + (int) this._compartment_.stateArgs.get("CamelStateParam") + (int) e._parameters.get("param123");
            action123_do(localVar123);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", localVar123);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "call") {
            if (((String) e._parameters.get("event")) == "snake_event") {
                snake_event(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "CamelEvent") {
                CamelEvent(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "event123") {
                event123(((int) e._parameters.get("param")));
            } else {
            }
            return;
        }
    }
    
    private void _sstate123_(FrameEvent e) {
          //  1300
        if(e._message == "snake_event") {
            int snake_local_var  = (int) this._compartment_.stateVars.get("stateVar123") + (int) this._compartment_.stateArgs.get("stateParam123") + (int) e._parameters.get("snake_param");
            snake_action_do(snake_local_var);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", snake_local_var);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "CamelEvent") {
            int CamelLocalVar  = (int) this._compartment_.stateVars.get("stateVar123") + (int) this._compartment_.stateArgs.get("stateParam123") + (int) e._parameters.get("CamelParam");
            CamelAction_do(CamelLocalVar);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", CamelLocalVar);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "event123") {
            int localVar123  = (int) this._compartment_.stateVars.get("stateVar123") + (int) this._compartment_.stateArgs.get("stateParam123") + (int) e._parameters.get("param123");
            action123_do(localVar123);
            NamingCompartment compartment =  new NamingCompartment(NamingState.FINAL.getValue());
            compartment.stateArgs.put("result", localVar123);
            
            this._transition_(compartment);
            return;
        }
        else if(e._message == "call") {
            if (((String) e._parameters.get("event")) == "snake_event") {
                snake_event(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "CamelEvent") {
                CamelEvent(((int) e._parameters.get("param")));
            } else if (((String) e._parameters.get("event")) == "event123") {
                event123(((int) e._parameters.get("param")));
            } else {
            }
            return;
        }
    }
    
    private void _sFinal_(FrameEvent e) {
        if(e._message == ">") {
            logFinal_do(((int) this._compartment_.stateArgs.get("result")));
            NamingCompartment compartment =  new NamingCompartment(NamingState.INIT.getValue());
            
            this._transition_(compartment);
            return;
        }
    }
    
    //===================== Actions Block ===================//
    
    // Unimplemented Actions
    
    protected void snake_action_do(int snake_param) { throw new UnsupportedOperationException(); }
    protected void CamelAction_do(int CamelParam) { throw new UnsupportedOperationException(); }
    protected void action123_do(int param123) { throw new UnsupportedOperationException(); }
    protected void logFinal_do(int r) { throw new UnsupportedOperationException(); }
    
    //===================== Domain Block ===================//
    
    public int snake_domain_var  = 300;
    public int CamelDomainVar  = 550;
    public int domainVar123  = 150;
    public ArrayList<Integer> snake_log  = new ArrayList<Integer>();
    public ArrayList<Integer> CamelLog  = new ArrayList<Integer>();
    public ArrayList<Integer> log123  = new ArrayList<Integer>();
    public ArrayList<Integer> finalLog  = new ArrayList<Integer>();
    
    //=============== Machinery and Mechanisms ==============//
    
    private int _state_;
    
    private void _transition_(NamingCompartment compartment) {
        this._nextCompartment_ = compartment;
    }
    
    private void _doTransition_(NamingCompartment nextCompartment) {
        this._mux_(new FrameEvent("<", this._compartment_.exitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.enterArgs));
    }
    
    public String state_info(){
        return String.valueOf(this._compartment_.state);
            }
            
    }
    
    //=============== Compartment ==============//
    
    class NamingCompartment {
    
        int state;
        
        NamingCompartment(int state) {
            this.state = state;
        }
        
        HashMap<String, Object> stateArgs = new HashMap<String, Object>();
        HashMap<String, Object> stateVars = new HashMap<String, Object>();
        HashMap<String, Object> enterArgs = new HashMap<String, Object>();
        HashMap<String, Object> exitArgs = new HashMap<String, Object>();
        FrameEvent _forwardEvent = new FrameEvent();
    }
    
    
    /********************

    class NamingController extends Naming {

    	NamingController() {
    	  super();
    	}
    
    protected void snake_action_do(int snake_param) {}
    
    protected void CamelAction_do(int CamelParam) {}
    
    protected void action123_do(int param123) {}
    
    protected void logFinal_do(int r) {}
    }
    
********************/
    
using FrameLang;
#nullable disable
namespace Naming
{

class Naming
{

    private NamingCompartment _compartment_;
    private NamingCompartment _nextCompartment_;



    public Naming(){


        // Create and intialize start state compartment.

        this._state_ = (int)NamingState.INIT;
        this._compartment_ = new NamingCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum NamingState
    {

        INIT = 0,
        SNAKE_STATE = 1,
        CAMELSTATE = 2,
        STATE123 = 3,
        FINAL = 4
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)NamingState.INIT:
                this._sInit_(e);
                break;
            case (int)NamingState.SNAKE_STATE:
                this._ssnake_state_(e);
                break;
            case (int)NamingState.CAMELSTATE:
                this._sCamelState_(e);
                break;
            case (int)NamingState.STATE123:
                this._sstate123_(e);
                break;
            case (int)NamingState.FINAL:
                this._sFinal_(e);
                break;
        }

        if( this._nextCompartment_ != null)
        {
            var nextCompartment = this._nextCompartment_;
            this._nextCompartment_ = null;
            if (nextCompartment._forwardEvent != null &&
               nextCompartment._forwardEvent._message == ">")
            {
                this._mux_(new FrameEvent( "<", this._compartment_.ExitArgs));
                this._compartment_ = nextCompartment;
                this._mux_(nextCompartment._forwardEvent);
            }
            else
            {
                this._doTransition_(nextCompartment);
                if (nextCompartment._forwardEvent != null)
                {
                    this._mux_(nextCompartment._forwardEvent);
                }
            }
            nextCompartment._forwardEvent = null;
        }
    }

    //===================== Interface Block ===================//

    public void snake_event(int snake_param) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["snake_param"] = snake_param;

        FrameEvent e = new FrameEvent("snake_event",parameters);
        this._mux_(e);
    }

    public void CamelEvent(int CamelParam) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["CamelParam"] = CamelParam;

        FrameEvent e = new FrameEvent("CamelEvent",parameters);
        this._mux_(e);
    }

    public void event123(int param123) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["param123"] = param123;

        FrameEvent e = new FrameEvent("event123",parameters);
        this._mux_(e);
    }

    public void call(string eventStr,int param) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["eventStr"] = eventStr;

        parameters["param"] = param;

        FrameEvent e = new FrameEvent("call",parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    NamingCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == "snake_event")
        {


            compartment =  new NamingCompartment((int)NamingState.SNAKE_STATE);

            compartment.StateArgs["snake_state_param"] = (int)e._parameters["snake_param"];

            compartment.StateVars["snake_state_var"] = this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 100;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "CamelEvent")
        {


            compartment =  new NamingCompartment((int)NamingState.CAMELSTATE);

            compartment.StateArgs["CamelStateParam"] = (int)e._parameters["CamelParam"];

            compartment.StateVars["CamelStateVar"] = this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 200;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "event123")
        {


            compartment =  new NamingCompartment((int)NamingState.STATE123);

            compartment.StateArgs["stateParam123"] = (int)e._parameters["param123"];

            compartment.StateVars["stateVar123"] = this.snake_domain_var + this.CamelDomainVar + this.domainVar123 + 300;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "call")
        {
            if (((string)e._parameters["eventStr"] == "snake_event")) {
                snake_event((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "CamelEvent")) {
                CamelEvent((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "event123")) {
                event123((int)e._parameters["param"]);
                return;
            } else {
            }

            return;
        }
    }

    private void _ssnake_state_(FrameEvent e)
    {
          //  1100
        if (e._message == "snake_event")
        {
            int snake_local_var  = (int)this._compartment_.StateVars["snake_state_var"] + (int)this._compartment_.StateArgs["snake_state_param"] + (int)e._parameters["snake_param"];
            this.snake_action_do(snake_local_var);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = snake_local_var;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "CamelEvent")
        {
            int CamelLocalVar  = (int)this._compartment_.StateVars["snake_state_var"] + (int)this._compartment_.StateArgs["snake_state_param"] + (int)e._parameters["CamelParam"];
            this.CamelAction_do(CamelLocalVar);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = CamelLocalVar;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "event123")
        {
            int localVar123  = (int)this._compartment_.StateVars["snake_state_var"] + (int)this._compartment_.StateArgs["snake_state_param"] + (int)e._parameters["param123"];
            this.action123_do(localVar123);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = localVar123;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "call")
        {
            if (((string)e._parameters["eventStr"] == "snake_event")) {
                snake_event((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "CamelEvent")) {
                CamelEvent((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "event123")) {
                event123((int)e._parameters["param"]);
                return;
            } else {
            }

            return;
        }
    }

    private void _sCamelState_(FrameEvent e)
    {
          //  1200
        if (e._message == "snake_event")
        {
            int snake_local_var  = (int)this._compartment_.StateVars["CamelStateVar"] + (int)this._compartment_.StateArgs["CamelStateParam"] + (int)e._parameters["snake_param"];
            this.snake_action_do(snake_local_var);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = snake_local_var;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "CamelEvent")
        {
            int CamelLocalVar  = (int)this._compartment_.StateVars["CamelStateVar"] + (int)this._compartment_.StateArgs["CamelStateParam"] + (int)e._parameters["CamelParam"];
            this.CamelAction_do(CamelLocalVar);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = CamelLocalVar;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "event123")
        {
            int localVar123  = (int)this._compartment_.StateVars["CamelStateVar"] + (int)this._compartment_.StateArgs["CamelStateParam"] + (int)e._parameters["param123"];
            this.action123_do(localVar123);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = localVar123;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "call")
        {
            if (((string)e._parameters["eventStr"] == "snake_event")) {
                snake_event((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "CamelEvent")) {
                CamelEvent((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "event123")) {
                event123((int)e._parameters["param"]);
                return;
            } else {
            }

            return;
        }
    }

    private void _sstate123_(FrameEvent e)
    {
          //  1300
        if (e._message == "snake_event")
        {
            int snake_local_var  = (int)this._compartment_.StateVars["stateVar123"] + (int)this._compartment_.StateArgs["stateParam123"] + (int)e._parameters["snake_param"];
            this.snake_action_do(snake_local_var);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = snake_local_var;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "CamelEvent")
        {
            int CamelLocalVar  = (int)this._compartment_.StateVars["stateVar123"] + (int)this._compartment_.StateArgs["stateParam123"] + (int)e._parameters["CamelParam"];
            this.CamelAction_do(CamelLocalVar);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = CamelLocalVar;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "event123")
        {
            int localVar123  = (int)this._compartment_.StateVars["stateVar123"] + (int)this._compartment_.StateArgs["stateParam123"] + (int)e._parameters["param123"];
            this.action123_do(localVar123);


            compartment =  new NamingCompartment((int)NamingState.FINAL);

            compartment.StateArgs["result"] = localVar123;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "call")
        {
            if (((string)e._parameters["eventStr"] == "snake_event")) {
                snake_event((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "CamelEvent")) {
                CamelEvent((int)e._parameters["param"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "event123")) {
                event123((int)e._parameters["param"]);
                return;
            } else {
            }

            return;
        }
    }

    private void _sFinal_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.logFinal_do((int)this._compartment_.StateArgs["result"]);


            compartment =  new NamingCompartment((int)NamingState.INIT);


            this._transition_(compartment);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void snake_action_do(int snake_param)
    {
        this.snake_log.Add(snake_param);
    }

    public void CamelAction_do(int CamelParam)
    {
        this.CamelLog.Add(CamelParam);
    }

    public void action123_do(int param123)
    {
        this.log123.Add(param123);
    }

    public void logFinal_do(int r)
    {
        this.finalLog.Add(r);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public int snake_domain_var  = 300;
    public int CamelDomainVar  = 550;
    public int domainVar123  = 150;
    public List<int> snake_log  = new List<int>();
    public List<int> CamelLog  = new List<int>();
    public List<int> log123  = new List<int>();
    public List<int> finalLog  = new List<int>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(NamingCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(NamingCompartment nextCompartment)
    {
        this._mux_(new FrameEvent("<", this._compartment_.ExitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.EnterArgs));
    }

    public string state_info(){
        return this._compartment_.state.ToString();
        }

}

//=============== Compartment ==============//

class NamingCompartment
{

    public int state;

    public NamingCompartment(int state)
    {
        this.state = state;
    }

    public Dictionary<string, object> StateArgs { get; set; } = new Dictionary<string, object>();
    public Dictionary<string, object> StateVars { get; set; } = new Dictionary<string, object>();
    public Dictionary<string, object> EnterArgs { get; set; } = new Dictionary<string, object>();
    public Dictionary<string, object> ExitArgs { get; set; } = new Dictionary<string, object>();
    public FrameEvent _forwardEvent = new FrameEvent();
}


/********************

class NamingController : Naming
{
        public NamingController() : base()
        {
        }
}

********************/
}
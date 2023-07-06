using FrameLang;
#nullable disable
namespace HandlerCalls
{

class HandlerCalls
{

    private HandlerCallsCompartment _compartment_;
    private HandlerCallsCompartment _nextCompartment_;



    public HandlerCalls(){


        // Create and intialize start state compartment.


        this._state_ = (int)HandlerCallsState.INIT;
        this._compartment_ = new HandlerCallsCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum HandlerCallsState
    {

        INIT = 0,
        NONRECURSIVE = 1,
        SELFRECURSIVE = 2,
        MUTUALLYRECURSIVE = 3,
        FINAL = 4
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)HandlerCallsState.INIT:
                this._sInit_(e);
                break;
            case (int)HandlerCallsState.NONRECURSIVE:
                this._sNonRecursive_(e);
                break;
            case (int)HandlerCallsState.SELFRECURSIVE:
                this._sSelfRecursive_(e);
                break;
            case (int)HandlerCallsState.MUTUALLYRECURSIVE:
                this._sMutuallyRecursive_(e);
                break;
            case (int)HandlerCallsState.FINAL:
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

    public void NonRec() {
        FrameEvent e = new FrameEvent("NonRec",null);
        this._mux_(e);
    }

    public void SelfRec() {
        FrameEvent e = new FrameEvent("SelfRec",null);
        this._mux_(e);
    }

    public void MutRec() {
        FrameEvent e = new FrameEvent("MutRec",null);
        this._mux_(e);
    }

    public void Call(string eventStr,int arg) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["eventStr"] = eventStr;

        parameters["arg"] = arg;

        FrameEvent e = new FrameEvent("Call",parameters);
        this._mux_(e);
    }

    public void Foo(int arg) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["arg"] = arg;

        FrameEvent e = new FrameEvent("Foo",parameters);
        this._mux_(e);
    }

    public void Bar(int arg) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["arg"] = arg;

        FrameEvent e = new FrameEvent("Bar",parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    HandlerCallsCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == "NonRec")
        {


            compartment =  new HandlerCallsCompartment((int)HandlerCallsState.NONRECURSIVE);


            compartment.StateVars["counter"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "SelfRec")
        {


            compartment =  new HandlerCallsCompartment((int)HandlerCallsState.SELFRECURSIVE);


            compartment.StateVars["counter"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "MutRec")
        {


            compartment =  new HandlerCallsCompartment((int)HandlerCallsState.MUTUALLYRECURSIVE);


            compartment.StateVars["counter"] = 0;

            this._transition_(compartment);

            return;
        }
    }

    private void _sNonRecursive_(FrameEvent e)
    {
        if (e._message == "Foo")
        {
            this.log_do("Foo",(int)e._parameters["arg"]);
            this._compartment_.StateVars["counter"] = ((int)this._compartment_.StateVars["counter"]) + (int)e._parameters["arg"];
            Bar((int)e._parameters["arg"] * 2);
            return;
            this.log_do("Unreachable",0);

            return;
        }
          //  the front-end should report the next line as a static error
        else if (e._message == "Bar")
        {
            this.log_do("Bar",(int)e._parameters["arg"]);
            this._compartment_.StateVars["counter"] = ((int)this._compartment_.StateVars["counter"]) + (int)e._parameters["arg"];


            compartment =  new HandlerCallsCompartment((int)HandlerCallsState.FINAL);

            compartment.StateArgs["counter"] = (int)this._compartment_.StateVars["counter"];

            this._transition_(compartment);

            return;
        }
        else if (e._message == "Call")
        {
            if (((string)e._parameters["eventStr"] == "Foo")) {
                Foo((int)e._parameters["arg"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "Bar")) {
                Bar((int)e._parameters["arg"]);
                return;
            } else {
                Call("Foo",1000);
                return;
            }

            return;
        }
    }

    private void _sSelfRecursive_(FrameEvent e)
    {
        if (e._message == "Foo")
        {
            this.log_do("Foo",(int)e._parameters["arg"]);
            this._compartment_.StateVars["counter"] = ((int)this._compartment_.StateVars["counter"]) + (int)e._parameters["arg"];
            if (((int)this._compartment_.StateVars["counter"]) < 100) {
                Foo((int)e._parameters["arg"] * 2);
                return;
            } else {


                compartment =  new HandlerCallsCompartment((int)HandlerCallsState.FINAL);

                compartment.StateArgs["counter"] = (int)this._compartment_.StateVars["counter"];

                this._transition_(compartment);
                return;
            }

            return;
        }
        else if (e._message == "Bar")
        {
            this.log_do("Bar",(int)e._parameters["arg"]);
            this._compartment_.StateVars["counter"] = ((int)this._compartment_.StateVars["counter"]) + (int)e._parameters["arg"];


            compartment =  new HandlerCallsCompartment((int)HandlerCallsState.FINAL);

            compartment.StateArgs["counter"] = (int)this._compartment_.StateVars["counter"];

            this._transition_(compartment);

            return;
        }
        else if (e._message == "Call")
        {
            if (((string)e._parameters["eventStr"] == "Foo")) {
                Foo((int)e._parameters["arg"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "Bar")) {
                Bar((int)e._parameters["arg"]);
                return;
            } else {
            }

            return;
        }
    }

    private void _sMutuallyRecursive_(FrameEvent e)
    {
        if (e._message == "Foo")
        {
            this.log_do("Foo",(int)e._parameters["arg"]);
            this._compartment_.StateVars["counter"] = ((int)this._compartment_.StateVars["counter"]) + (int)e._parameters["arg"];
            if (((int)this._compartment_.StateVars["counter"]) > 100) {


                compartment =  new HandlerCallsCompartment((int)HandlerCallsState.FINAL);

                compartment.StateArgs["counter"] = (int)this._compartment_.StateVars["counter"];

                this._transition_(compartment);
                return;
            } else {
                Bar((int)e._parameters["arg"] * 2);
                return;
            }

            return;
        }
        else if (e._message == "Bar")
        {
            this.log_do("Bar",(int)e._parameters["arg"]);
            this._compartment_.StateVars["counter"] = ((int)this._compartment_.StateVars["counter"]) + (int)e._parameters["arg"];
            if (((int)e._parameters["arg"] == 4)) {
                Foo((int)e._parameters["arg"]);
                return;
            } else if (((int)e._parameters["arg"] == 8)) {
                Foo((int)e._parameters["arg"] * 2);
                return;
            } else {
                Foo((int)e._parameters["arg"] * 3);
                return;
            }

            return;
        }
        else if (e._message == "Call")
        {
            if (((string)e._parameters["eventStr"] == "Foo")) {
                Foo((int)e._parameters["arg"]);
                return;
            } else if (((string)e._parameters["eventStr"] == "Bar")) {
                Bar((int)e._parameters["arg"]);
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
            this.log_do("Final",(int)this._compartment_.StateArgs["counter"]);


            compartment =  new HandlerCallsCompartment((int)HandlerCallsState.INIT);


            this._transition_(compartment);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void log_do(string from, int val)
    {

            string value = from + "(" + val.ToString() + ")";
            this.tape.Add(value);

    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> tape  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(HandlerCallsCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(HandlerCallsCompartment nextCompartment)
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

class HandlerCallsCompartment
{

    public int state;

    public HandlerCallsCompartment(int state)
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

class HandlerCallsController : HandlerCalls
{
        public HandlerCallsController() : base()
        {
        }
}

********************/
}
using FrameLang;
#nullable disable
namespace TransitionParams
{

class TransitParams
{

    public TransitParamsCompartment _compartment_;
    public TransitParamsCompartment _nextCompartment_;



    public TransitParams(){


        // Create and intialize start state compartment.

        this._state_ = (int)TransitParamsState.INIT;
        this._compartment_ = new TransitParamsCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum TransitParamsState
    {

        INIT = 0,
        A = 1,
        B = 2
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)TransitParamsState.INIT:
                this._sInit_(e);
                break;
            case (int)TransitParamsState.A:
                this._sA_(e);
                break;
            case (int)TransitParamsState.B:
                this._sB_(e);
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

    public void Next() {
        FrameEvent e = new FrameEvent("Next",null);
        this._mux_(e);
    }

    public void Change() {
        FrameEvent e = new FrameEvent("Change",null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    TransitParamsCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == "Next")
        {


            compartment =  new TransitParamsCompartment((int)TransitParamsState.A);

            compartment.EnterArgs["msg"]  = "hi A";

            this._transition_(compartment);

            return;
        }
        else if (e._message == "Change")
        {
            compartment =  new TransitParamsCompartment((int)TransitParamsState.A);

            this._changeState_(compartment);

            return;
        }
    }

    private void _sA_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do((string)e._parameters["msg"]);

            return;
        }
        else if (e._message == "<")
        {
            this.log_do("bye A");

            return;
        }
        else if (e._message == "Next")
        {


            compartment =  new TransitParamsCompartment((int)TransitParamsState.B);

            compartment.EnterArgs["msg"]  = "hi B";
            compartment.EnterArgs["val"]  = 42;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "Change")
        {
            compartment =  new TransitParamsCompartment((int)TransitParamsState.B);

            this._changeState_(compartment);
            
            return;
        }
    }

    private void _sB_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do((string)e._parameters["msg"]);
            //this.log_do((int)e._parameters["val"].ToString());
            this.log_do(e._parameters["val"].ToString());

            return;
        }
        else if (e._message == "<")
        {
            //this.log_do((bool)e._parameters["val"].ToString());
            this.log_do(e._parameters["val"].ToString());
            this.log_do((string)e._parameters["msg"]);

            return;
        }
        else if (e._message == "Next")
        {

            this._compartment_.ExitArgs["val"] = true;
            this._compartment_.ExitArgs["msg"] = "bye B";

            compartment =  new TransitParamsCompartment((int)TransitParamsState.A);

            compartment.EnterArgs["msg"]  = "hi again A";

            this._transition_(compartment);

            return;
        }
        else if (e._message == "Change")
        {
            compartment =  new TransitParamsCompartment((int)TransitParamsState.A);

            this._changeState_(compartment);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void log_do(string msg)
    {
        this.tape.Add(msg);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> tape  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(TransitParamsCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(TransitParamsCompartment nextCompartment)
    {
        this._mux_(new FrameEvent("<", this._compartment_.ExitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.EnterArgs));
    }

    private void _changeState_(TransitParamsCompartment compartment)
    {
        this._compartment_ = compartment;
    }

    public string state_info(){
        return this._compartment_.state.ToString();
        }

}

//=============== Compartment ==============//

class TransitParamsCompartment
{

    public int state;

    public TransitParamsCompartment(int state)
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

class TransitParamsController : TransitParams
{
        public TransitParamsController() : base()
        {
        }
}

********************/
}
using FrameLang;
#nullable disable
namespace Transition
{

class TransitionSm
{

    private TransitionSmCompartment _compartment_;
    private TransitionSmCompartment _nextCompartment_;



    public TransitionSm(){


        // Create and intialize start state compartment.

        this._state_ = (int)TransitionSmState.S0;
        this._compartment_ = new TransitionSmCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum TransitionSmState
    {

        S0 = 0,
        S1 = 1,
        S2 = 2,
        S3 = 3,
        S4 = 4
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)TransitionSmState.S0:
                this._sS0_(e);
                break;
            case (int)TransitionSmState.S1:
                this._sS1_(e);
                break;
            case (int)TransitionSmState.S2:
                this._sS2_(e);
                break;
            case (int)TransitionSmState.S3:
                this._sS3_(e);
                break;
            case (int)TransitionSmState.S4:
                this._sS4_(e);
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

    public void transit() {
        FrameEvent e = new FrameEvent("transit",null);
        this._mux_(e);
    }

    public void change() {
        FrameEvent e = new FrameEvent("change",null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    TransitionSmCompartment compartment;


    private void _sS0_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S0");

            return;
        }
        else if (e._message == "<")
        {
            this.exit_do("S0");

            return;
        }
        else if (e._message == "transit")
        {


            compartment =  new TransitionSmCompartment((int)TransitionSmState.S1);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "change")
        {
            compartment =  new TransitionSmCompartment((int)TransitionSmState.S1);

            this._changeState_(compartment);

            return;
        }
    }

    private void _sS1_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S1");

            return;
        }
        else if (e._message == "<")
        {
            this.exit_do("S1");

            return;
        }
        else if (e._message == "transit")
        {


            compartment =  new TransitionSmCompartment((int)TransitionSmState.S2);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "change")
        {
            compartment =  new TransitionSmCompartment((int)TransitionSmState.S2);

            this._changeState_(compartment);

            return;
        }
    }

    private void _sS2_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S2");


            compartment =  new TransitionSmCompartment((int)TransitionSmState.S3);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "<")
        {
            this.exit_do("S2");

            return;
        }
        else if (e._message == "transit")
        {


            compartment =  new TransitionSmCompartment((int)TransitionSmState.S3);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "change")
        {
            compartment =  new TransitionSmCompartment((int)TransitionSmState.S3);

            this._changeState_(compartment);

            return;
        }
    }

    private void _sS3_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S3");

            return;
        }
        else if (e._message == "<")
        {
            this.exit_do("S3");

            return;
        }
        else if (e._message == "transit")
        {


            compartment =  new TransitionSmCompartment((int)TransitionSmState.S4);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "change")
        {
            compartment =  new TransitionSmCompartment((int)TransitionSmState.S4);

            this._changeState_(compartment);

            return;
        }
    }

    private void _sS4_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S4");
            compartment =  new TransitionSmCompartment((int)TransitionSmState.S0);

            this._changeState_(compartment);

            return;
        }
        else if (e._message == "<")
        {
            this.exit_do("S4");

            return;
        }
    }

    //===================== Actions Block ===================//

    public void enter_do(string state)
    {
        this.enters.Add(state);
    }

    public void exit_do(string state)
    {
        this.exits.Add(state);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> enters  = new List<string>();
    public List<string> exits  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(TransitionSmCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(TransitionSmCompartment nextCompartment)
    {
        this._mux_(new FrameEvent("<", this._compartment_.ExitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.EnterArgs));
    }

    private void _changeState_(TransitionSmCompartment compartment)
    {
        this._compartment_ = compartment;
    }

    public string state_info(){
        return this._compartment_.state.ToString();
        }

}

//=============== Compartment ==============//

class TransitionSmCompartment
{

    public int state;

    public TransitionSmCompartment(int state)
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

class TransitionSmController : TransitionSm
{
        public TransitionSmController() : base()
        {
        }
}

********************/
}
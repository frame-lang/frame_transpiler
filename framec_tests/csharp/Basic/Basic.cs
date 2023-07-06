using FrameLang;
#nullable disable
namespace Basic
{

class Basic
{

    private BasicCompartment _compartment_;
    private BasicCompartment _nextCompartment_;



    public Basic(){


        // Create and intialize start state compartment.


        this._state_ = (int)BasicState.S0;
        this._compartment_ = new BasicCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum BasicState
    {

        S0 = 0,
        S1 = 1
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)BasicState.S0:
                this._sS0_(e);
                break;
            case (int)BasicState.S1:
                this._sS1_(e);
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

    public void A() {
        FrameEvent e = new FrameEvent("A",null);
        this._mux_(e);
    }

    public void B() {
        FrameEvent e = new FrameEvent("B",null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    BasicCompartment compartment;


    private void _sS0_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.entered_do("S0");

            return;
        }
        else if (e._message == "<")
        {
            this.left_do("S0");

            return;
        }
        else if (e._message == "A")
        {

            // ooh

            compartment =  new BasicCompartment((int)BasicState.S1);


            this._transition_(compartment);

            return;
        }
    }

    private void _sS1_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.entered_do("S1");

            return;
        }
        else if (e._message == "<")
        {
            this.left_do("S1");

            return;
        }
        else if (e._message == "B")
        {

            // aah

            compartment =  new BasicCompartment((int)BasicState.S0);


            this._transition_(compartment);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void entered_do(string msg)
    {
        entry_log.Add(msg);
    }

    public void left_do(string msg)
    {
        exit_log.Add(msg);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> entry_log  = new List<string>();
    public List<string> exit_log  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(BasicCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(BasicCompartment nextCompartment)
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

class BasicCompartment
{

    public int state;

    public BasicCompartment(int state)
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

class BasicController : Basic
{
        public BasicController() : base()
        {
        }
}

********************/
}
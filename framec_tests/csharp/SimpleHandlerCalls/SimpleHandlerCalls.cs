using FrameLang;
#nullable disable
namespace SimpleHandlerCalls
{

class SimpleHandlerCalls
{

    private SimpleHandlerCallsCompartment _compartment_;
    private SimpleHandlerCallsCompartment _nextCompartment_;



    public SimpleHandlerCalls(){


        // Create and intialize start state compartment.

        this._state_ = (int)SimpleHandlerCallsState.INIT;
        this._compartment_ = new SimpleHandlerCallsCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum SimpleHandlerCallsState
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
            case (int)SimpleHandlerCallsState.INIT:
                this._sInit_(e);
                break;
            case (int)SimpleHandlerCallsState.A:
                this._sA_(e);
                break;
            case (int)SimpleHandlerCallsState.B:
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

    public void A() {
        FrameEvent e = new FrameEvent("A",null);
        this._mux_(e);
    }

    public void B() {
        FrameEvent e = new FrameEvent("B",null);
        this._mux_(e);
    }

    public void C() {
        FrameEvent e = new FrameEvent("C",null);
        this._mux_(e);
    }

    public void D() {
        FrameEvent e = new FrameEvent("D",null);
        this._mux_(e);
    }

    public void E() {
        FrameEvent e = new FrameEvent("E",null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    SimpleHandlerCallsCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == "A")
        {


            compartment =  new SimpleHandlerCallsCompartment((int)SimpleHandlerCallsState.A);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "B")
        {


            compartment =  new SimpleHandlerCallsCompartment((int)SimpleHandlerCallsState.B);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "C")
        {
            A();
            return;

            return;
        }
        else if (e._message == "D")
        {
            B();
            return;


            compartment =  new SimpleHandlerCallsCompartment((int)SimpleHandlerCallsState.A);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "E")
        {
            D();
            return;
            C();
            return;

            return;
        }
    }

    private void _sA_(FrameEvent e)
    {
    }

    private void _sB_(FrameEvent e)
    {
    }


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(SimpleHandlerCallsCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(SimpleHandlerCallsCompartment nextCompartment)
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

class SimpleHandlerCallsCompartment
{

    public int state;

    public SimpleHandlerCallsCompartment(int state)
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

class SimpleHandlerCallsController : SimpleHandlerCalls
{
        public SimpleHandlerCallsController() : base()
        {
        }
}

********************/
}
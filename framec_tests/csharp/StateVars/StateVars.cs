using FrameLang;
#nullable disable
namespace StateVars
{

class StateVars
{

    public StateVarsCompartment _compartment_;
    public StateVarsCompartment _nextCompartment_;



    public StateVars(){


        // Create and intialize start state compartment.

        this._state_ = (int)StateVarsState.INIT;
        this._compartment_ = new StateVarsCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum StateVarsState
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
            case (int)StateVarsState.INIT:
                this._sInit_(e);
                break;
            case (int)StateVarsState.A:
                this._sA_(e);
                break;
            case (int)StateVarsState.B:
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

    public void X() {
        FrameEvent e = new FrameEvent("X",null);
        this._mux_(e);
    }

    public void Y() {
        FrameEvent e = new FrameEvent("Y",null);
        this._mux_(e);
    }

    public void Z() {
        FrameEvent e = new FrameEvent("Z",null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    StateVarsCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == ">")
        {


            compartment =  new StateVarsCompartment((int)StateVarsState.A);


            compartment.StateVars["x"] = 0;

            this._transition_(compartment);

            return;
        }
    }

    private void _sA_(FrameEvent e)
    {
        if (e._message == "X")
        {
            this._compartment_.StateVars["x"] = ((int)this._compartment_.StateVars["x"]) + 1;

            return;
        }
        else if (e._message == "Y")
        {


            compartment =  new StateVarsCompartment((int)StateVarsState.B);


            compartment.StateVars["y"] = 10;

            compartment.StateVars["z"] = 100;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "Z")
        {


            compartment =  new StateVarsCompartment((int)StateVarsState.B);


            compartment.StateVars["y"] = 10;

            compartment.StateVars["z"] = 100;

            this._transition_(compartment);

            return;
        }
    }

    private void _sB_(FrameEvent e)
    {
        if (e._message == "X")
        {


            compartment =  new StateVarsCompartment((int)StateVarsState.A);


            compartment.StateVars["x"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "Y")
        {
            this._compartment_.StateVars["y"] = ((int)this._compartment_.StateVars["y"]) + 1;

            return;
        }
        else if (e._message == "Z")
        {
            this._compartment_.StateVars["z"] = ((int)this._compartment_.StateVars["z"]) + 1;

            return;
        }
    }

    //===================== Actions Block ===================//

    // Unimplemented Actions


    //===================== Domain Block ===================//



    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(StateVarsCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(StateVarsCompartment nextCompartment)
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

class StateVarsCompartment
{

    public int state;

    public StateVarsCompartment(int state)
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

class StateVarsController : StateVars
{
        public StateVarsController() : base()
        {
        }
}

********************/
}
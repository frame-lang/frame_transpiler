using FrameLang;
#nullable disable
namespace StateParams
{

class StateParams
{

    private StateParamsCompartment _compartment_;
    private StateParamsCompartment _nextCompartment_;



    public StateParams(){


        // Create and intialize start state compartment.


        this._state_ = (int)StateParamsState.INIT;
        this._compartment_ = new StateParamsCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum StateParamsState
    {

        INIT = 0,
        SPLIT = 1,
        MERGE = 2
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)StateParamsState.INIT:
                this._sInit_(e);
                break;
            case (int)StateParamsState.SPLIT:
                this._sSplit_(e);
                break;
            case (int)StateParamsState.MERGE:
                this._sMerge_(e);
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

    public void Prev() {
        FrameEvent e = new FrameEvent("Prev",null);
        this._mux_(e);
    }

    public void Log() {
        FrameEvent e = new FrameEvent("Log",null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    StateParamsCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == "Next")
        {


            compartment =  new StateParamsCompartment((int)StateParamsState.SPLIT);

            compartment.StateArgs["val"] = 1;

            this._transition_(compartment);
            return;
        }
    }

    private void _sSplit_(FrameEvent e)
    {
        if (e._message == "Next")
        {


            compartment =  new StateParamsCompartment((int)StateParamsState.MERGE);

            compartment.StateArgs["left"] = (int)this._compartment_.StateArgs["val"];
            compartment.StateArgs["right"] = (int)this._compartment_.StateArgs["val"] + 1;

            this._transition_(compartment);
            return;
        }
        else if (e._message == "Prev")
        {


            compartment =  new StateParamsCompartment((int)StateParamsState.MERGE);

            compartment.StateArgs["left"] = (int)this._compartment_.StateArgs["val"] + 1;
            compartment.StateArgs["right"] = (int)this._compartment_.StateArgs["val"];

            this._transition_(compartment);
            return;
        }
        else if (e._message == "Log")
        {
            this.got_param_do("val",((int)this._compartment_.StateArgs["val"]));
            return;
        }
    }

    private void _sMerge_(FrameEvent e)
    {
        if (e._message == "Next")
        {


            compartment =  new StateParamsCompartment((int)StateParamsState.SPLIT);

            compartment.StateArgs["val"] = (int)this._compartment_.StateArgs["left"] + (int)this._compartment_.StateArgs["right"];

            this._transition_(compartment);
            return;
        }
        else if (e._message == "Prev")
        {


            compartment =  new StateParamsCompartment((int)StateParamsState.SPLIT);

            compartment.StateArgs["val"] = (int)this._compartment_.StateArgs["left"] * (int)this._compartment_.StateArgs["right"];

            this._transition_(compartment);
            return;
        }
        else if (e._message == "Log")
        {
            this.got_param_do("left",((int)this._compartment_.StateArgs["left"]));
            this.got_param_do("right",((int)this._compartment_.StateArgs["right"]));
            return;
        }
    }

    //===================== Actions Block ===================//

    public void got_param_do(string name, int val)
    {
        this.param_log.Add(name+"="+(val).ToString());
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> param_log = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(StateParamsCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(StateParamsCompartment nextCompartment)
    {
        this._mux_(new FrameEvent("<", this._compartment_.ExitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.EnterArgs));
    }

    public String state_info(){
        return this._compartment_.state.ToString();
        }

}

//=============== Compartment ==============//

class StateParamsCompartment
{

    public int state;

    public StateParamsCompartment(int state)
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

class StateParamsController : StateParams
{
        public StateParamsController() : base()
        {
        }
}

********************/
}
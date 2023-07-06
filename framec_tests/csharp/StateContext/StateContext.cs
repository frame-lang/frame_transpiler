using FrameLang;
#nullable disable
namespace StateContext
{

class StateContextSm
{

    public StateContextSmCompartment _compartment_;
    public StateContextSmCompartment _nextCompartment_;



    public StateContextSm(){


        // Create and intialize start state compartment.

        this._state_ = (int)StateContextSmState.INIT;
        this._compartment_ = new StateContextSmCompartment(this._state_);
        this._nextCompartment_ = null;
        this._compartment_.StateVars["w"] = 0;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum StateContextSmState
    {

        INIT = 0,
        FOO = 1,
        BAR = 2
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)StateContextSmState.INIT:
                this._sInit_(e);
                break;
            case (int)StateContextSmState.FOO:
                this._sFoo_(e);
                break;
            case (int)StateContextSmState.BAR:
                this._sBar_(e);
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

    public void Start() {
        FrameEvent e = new FrameEvent("Start",null);
        this._mux_(e);
    }

    public void LogState() {
        FrameEvent e = new FrameEvent("LogState",null);
        this._mux_(e);
    }

    public int Inc() {
        FrameEvent e = new FrameEvent("Inc",null);
        this._mux_(e);
        return (int) e._return;
    }

    public void Next(int arg) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["arg"] = arg;

        FrameEvent e = new FrameEvent("Next",parameters);
        this._mux_(e);
    }

    public void Change(int arg) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["arg"] = arg;

        FrameEvent e = new FrameEvent("Change",parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    StateContextSmCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this._compartment_.StateVars["w"] = 3;
            this.log_do("w",((int)this._compartment_.StateVars["w"]));

            return;
        }
        else if (e._message == "Inc")
        {
            this._compartment_.StateVars["w"] = ((int)this._compartment_.StateVars["w"]) + 1;
            this.log_do("w",((int)this._compartment_.StateVars["w"]));
            e._return = ((int)this._compartment_.StateVars["w"]);

            return;

        }
        else if (e._message == "LogState")
        {
            this.log_do("w",((int)this._compartment_.StateVars["w"]));

            return;
        }
        else if (e._message == "Start")
        {


            compartment =  new StateContextSmCompartment((int)StateContextSmState.FOO);

            compartment.EnterArgs["a"]  = 3;
            compartment.EnterArgs["b"]  = (int)this._compartment_.StateVars["w"];

            compartment.StateVars["x"] = 0;

            this._transition_(compartment);

            return;
        }
    }

    private void _sFoo_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do("a",(int)e._parameters["a"]);
            this.log_do("b",(int)e._parameters["b"]);
            this._compartment_.StateVars["x"] = (int)e._parameters["a"] * (int)e._parameters["b"];
            this.log_do("x",((int)this._compartment_.StateVars["x"]));

            return;
        }
        else if (e._message == "<")
        {
            this.log_do("c",(int)e._parameters["c"]);
            this._compartment_.StateVars["x"] = ((int)this._compartment_.StateVars["x"]) + (int)e._parameters["c"];
            this.log_do("x",((int)this._compartment_.StateVars["x"]));

            return;
        }
        else if (e._message == "LogState")
        {
            this.log_do("x",((int)this._compartment_.StateVars["x"]));

            return;
        }
        else if (e._message == "Inc")
        {
            this._compartment_.StateVars["x"] = ((int)this._compartment_.StateVars["x"]) + 1;
            this.log_do("x",((int)this._compartment_.StateVars["x"]));
            e._return = ((int)this._compartment_.StateVars["x"]);

            return;

        }
        else if (e._message == "Next")
        {
            int tmp  = (int)e._parameters["arg"] * 10;

            this._compartment_.ExitArgs["c"] = 10;

            compartment =  new StateContextSmCompartment((int)StateContextSmState.BAR);

            compartment.EnterArgs["a"]  = tmp;
            compartment.StateArgs["y"] = (int)this._compartment_.StateVars["x"];

            compartment.StateVars["z"] = 0;

            this._transition_(compartment);

            return;
        }
          //  FIXME: Swapping this to 10 * arg causes a parse error!
        else if (e._message == "Change")
        {
            int tmp  = (int)this._compartment_.StateVars["x"] + (int)e._parameters["arg"];
            compartment =  new StateContextSmCompartment((int)StateContextSmState.BAR);
            compartment.StateArgs["y"] = tmp;
            compartment.StateVars["z"] = 0;

            this._changeState_(compartment);

            return;
        }
    }

    private void _sBar_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do("a",(int)e._parameters["a"]);
            this.log_do("y",(int)this._compartment_.StateArgs["y"]);
            this._compartment_.StateVars["z"] = (int)e._parameters["a"] + (int)this._compartment_.StateArgs["y"];
            this.log_do("z",((int)this._compartment_.StateVars["z"]));

            return;
        }
        else if (e._message == "LogState")
        {
            this.log_do("y",(int)this._compartment_.StateArgs["y"]);
            this.log_do("z",((int)this._compartment_.StateVars["z"]));

            return;
        }
        else if (e._message == "Inc")
        {
            this._compartment_.StateVars["z"] = ((int)this._compartment_.StateVars["z"]) + 1;
            this.log_do("z",((int)this._compartment_.StateVars["z"]));
            e._return = ((int)this._compartment_.StateVars["z"]);

            return;

        }
        else if (e._message == "Change")
        {
            int tmp  = (int)this._compartment_.StateArgs["y"] + (int)this._compartment_.StateVars["z"] + (int)e._parameters["arg"];
            this.log_do("tmp",tmp);
            compartment =  new StateContextSmCompartment((int)StateContextSmState.INIT);
            compartment.StateVars["w"] = 0;

            this._changeState_(compartment);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void log_do(String name, int val)
    {
        this.tape.Add(name + "=" +val.ToString());
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> tape  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(StateContextSmCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(StateContextSmCompartment nextCompartment)
    {
        this._mux_(new FrameEvent("<", this._compartment_.ExitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.EnterArgs));
    }

    private void _changeState_(StateContextSmCompartment compartment)
    {
        this._compartment_ = compartment;
    }

    public string state_info(){
        return this._compartment_.state.ToString();
        }

}

//=============== Compartment ==============//

class StateContextSmCompartment
{

    public int state;

    public StateContextSmCompartment(int state)
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

class StateContextSmController : StateContextSm
{
        public StateContextSmController() : base()
        {
        }
}

********************/
}
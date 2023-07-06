using FrameLang;
#nullable disable
namespace StateContextStack
{

class StateContextStack
{

    public StateContextStackCompartment _compartment_;
    public StateContextStackCompartment _nextCompartment_;



    public StateContextStack(){


        // Create state stack.

        this._stateStack_ = new Stack<StateContextStackCompartment>();

        // Create and intialize start state compartment.

        this._state_ = (int)StateContextStackState.A;
        this._compartment_ = new StateContextStackCompartment(this._state_);
        this._nextCompartment_ = null;
        this._compartment_.StateVars["x"] = 0;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum StateContextStackState
    {

        A = 0,
        B = 1,
        C = 2
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)StateContextStackState.A:
                this._sA_(e);
                break;
            case (int)StateContextStackState.B:
                this._sB_(e);
                break;
            case (int)StateContextStackState.C:
                this._sC_(e);
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

    public void to_a() {
        FrameEvent e = new FrameEvent("to_a",null);
        this._mux_(e);
    }

    public void to_b() {
        FrameEvent e = new FrameEvent("to_b",null);
        this._mux_(e);
    }

    public void to_c() {
        FrameEvent e = new FrameEvent("to_c",null);
        this._mux_(e);
    }

    public void inc() {
        FrameEvent e = new FrameEvent("inc",null);
        this._mux_(e);
    }

    public int value() {
        FrameEvent e = new FrameEvent("value",null);
        this._mux_(e);
        return (int) e._return;
    }

    public void push() {
        FrameEvent e = new FrameEvent("push",null);
        this._mux_(e);
    }

    public void pop() {
        FrameEvent e = new FrameEvent("pop",null);
        this._mux_(e);
    }

    public void pop_change() {
        FrameEvent e = new FrameEvent("pop_change",null);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    StateContextStackCompartment compartment;


    private void _sA_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do("A:>");

            return;
        }
        else if (e._message == "<")
        {
            this.log_do("A:<");

            return;
        }
        else if (e._message == "inc")
        {
            this._compartment_.StateVars["x"] = ((int)this._compartment_.StateVars["x"]) + 1;

            return;
        }
        else if (e._message == "value")
        {
            e._return = ((int)this._compartment_.StateVars["x"]);
            
            return;

        }
        else if (e._message == "to_a")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.A);


            compartment.StateVars["x"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_b")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.B);


            compartment.StateVars["y"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_c")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.C);


            compartment.StateVars["z"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "push")
        {
            _stateStack_push_(_compartment_);

            return;
        }
        else if (e._message == "pop")
        {
            StateContextStackCompartment stateContext = _stateStack_pop_();
            _transition_(stateContext);

            return;
        }
        else if (e._message == "pop_change")
        {
            StateContextStackCompartment compartment = this._stateStack_pop_();
            this._changeState_(compartment);

            return;
        }
    }

    private void _sB_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do("B:>");

            return;
        }
        else if (e._message == "<")
        {
            this.log_do("B:<");

            return;
        }
        else if (e._message == "inc")
        {
            this._compartment_.StateVars["y"] = ((int)this._compartment_.StateVars["y"]) + 5;

            return;
        }
        else if (e._message == "value")
        {
            e._return = ((int)this._compartment_.StateVars["y"]);

            return;

        }
        else if (e._message == "to_a")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.A);


            compartment.StateVars["x"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_b")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.B);


            compartment.StateVars["y"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_c")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.C);


            compartment.StateVars["z"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "push")
        {
            _stateStack_push_(_compartment_);

            return;
        }
        else if (e._message == "pop")
        {
            StateContextStackCompartment stateContext = _stateStack_pop_();
            _transition_(stateContext);

            return;
        }
        else if (e._message == "pop_change")
        {
            StateContextStackCompartment compartment = this._stateStack_pop_();
            this._changeState_(compartment);

            return;
        }
    }

    private void _sC_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do("C:>");

            return;
        }
        else if (e._message == "<")
        {
            this.log_do("C:<");

            return;
        }
        else if (e._message == "inc")
        {
            this._compartment_.StateVars["z"] = ((int)this._compartment_.StateVars["z"]) + 10;

            return;
        }
        else if (e._message == "value")
        {
            e._return = ((int)this._compartment_.StateVars["z"]);

            return;

        }
        else if (e._message == "to_a")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.A);


            compartment.StateVars["x"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_b")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.B);


            compartment.StateVars["y"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_c")
        {


            compartment =  new StateContextStackCompartment((int)StateContextStackState.C);


            compartment.StateVars["z"] = 0;

            this._transition_(compartment);

            return;
        }
        else if (e._message == "push")
        {
            _stateStack_push_(_compartment_);

            return;
        }
        else if (e._message == "pop")
        {
            StateContextStackCompartment stateContext = _stateStack_pop_();
            _transition_(stateContext);

            return;
        }
        else if (e._message == "pop_change")
        {
            StateContextStackCompartment compartment = this._stateStack_pop_();
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

    private void _transition_(StateContextStackCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(StateContextStackCompartment nextCompartment)
    {
        this._mux_(new FrameEvent("<", this._compartment_.ExitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.EnterArgs));
    }

    private Stack<StateContextStackCompartment> _stateStack_ = new Stack<StateContextStackCompartment>();

    private void _stateStack_push_(StateContextStackCompartment compartment) {
        _stateStack_.Push(this.DeepCopyCompartment(compartment));
    }

    private StateContextStackCompartment DeepCopyCompartment(StateContextStackCompartment c)
    {
        StateContextStackCompartment copyCompartment = new StateContextStackCompartment(c.state)
        {
            state = c.state,
            StateArgs = c.StateArgs == null ? new Dictionary<string, object>() : new Dictionary<string, object>(c.StateArgs),
            StateVars = c.StateVars == null ? new Dictionary<string, object>() : new Dictionary<string, object>(c.StateVars),
            EnterArgs = c.EnterArgs == null ? new Dictionary<string, object>() : new Dictionary<string, object>(c.EnterArgs),
            ExitArgs = c.ExitArgs == null ? new Dictionary<string, object>() : new Dictionary<string, object>(c.ExitArgs)
        };

        if (c._forwardEvent != null)
        {
            FrameEvent forwardEventCopy = new FrameEvent()
            {
                _message = c._forwardEvent._message,
                _return = c._forwardEvent._return
            };
            if(c._forwardEvent._parameters!=null){
                forwardEventCopy._parameters=new Dictionary<string, object>(c._forwardEvent._parameters);
            }
            copyCompartment._forwardEvent = forwardEventCopy;
        }

        return copyCompartment;
    }

    private StateContextStackCompartment _stateStack_pop_() {
        return _stateStack_.Pop();
    }

    private void _changeState_(StateContextStackCompartment compartment)
    {
        this._compartment_ = compartment;
    }

    public string state_info(){
        return this._compartment_.state.ToString();
        }

}

//=============== Compartment ==============//

class StateContextStackCompartment
{

    public int state;

    public StateContextStackCompartment(int state)
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

class StateContextStackController : StateContextStack
{
        public StateContextStackController() : base()
        {
        }
}

********************/
}
using FrameLang;
#nullable disable
namespace StateStack
{

class StateStack
{

    public StateStackCompartment _compartment_;
    public StateStackCompartment _nextCompartment_;



    public StateStack(){


        // Create state stack.

        this._stateStack_ = new Stack<StateStackCompartment>();

        // Create and intialize start state compartment.

        this._state_ = (int)StateStackState.A;
        this._compartment_ = new StateStackCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum StateStackState
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
            case (int)StateStackState.A:
                this._sA_(e);
                break;
            case (int)StateStackState.B:
                this._sB_(e);
                break;
            case (int)StateStackState.C:
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

    StateStackCompartment compartment;


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
        else if (e._message == "to_a")
        {


            compartment =  new StateStackCompartment((int)StateStackState.A);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_b")
        {


            compartment =  new StateStackCompartment((int)StateStackState.B);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_c")
        {


            compartment =  new StateStackCompartment((int)StateStackState.C);


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
            StateStackCompartment compartment = _stateStack_pop_();
            _transition_(compartment);

            return;
        }
        else if (e._message == "pop_change")
        {
            StateStackCompartment compartment = this._stateStack_pop_();
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
        else if (e._message == "to_a")
        {


            compartment =  new StateStackCompartment((int)StateStackState.A);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_b")
        {


            compartment =  new StateStackCompartment((int)StateStackState.B);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_c")
        {


            compartment =  new StateStackCompartment((int)StateStackState.C);


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
            StateStackCompartment compartment = _stateStack_pop_();
            _transition_(compartment);

            return;
        }
        else if (e._message == "pop_change")
        {
            StateStackCompartment compartment = this._stateStack_pop_();
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
        else if (e._message == "to_a")
        {


            compartment =  new StateStackCompartment((int)StateStackState.A);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_b")
        {


            compartment =  new StateStackCompartment((int)StateStackState.B);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_c")
        {


            compartment =  new StateStackCompartment((int)StateStackState.C);


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
            StateStackCompartment compartment = _stateStack_pop_();
            _transition_(compartment);

            return;
        }
        else if (e._message == "pop_change")
        {
            StateStackCompartment compartment = this._stateStack_pop_();
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

    private void _transition_(StateStackCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(StateStackCompartment nextCompartment)
    {
        this._mux_(new FrameEvent("<", this._compartment_.ExitArgs));
        this._compartment_ = nextCompartment;
        this._mux_(new FrameEvent(">", this._compartment_.EnterArgs));
    }

    private Stack<StateStackCompartment> _stateStack_ = new Stack<StateStackCompartment>();

    private void _stateStack_push_(StateStackCompartment compartment) {
        _stateStack_.Push(compartment);
    }

    private StateStackCompartment _stateStack_pop_() {
        return _stateStack_.Pop();
    }

    private void _changeState_(StateStackCompartment compartment)
    {
        this._compartment_ = compartment;
    }

    public string state_info(){
        return this._compartment_.state.ToString();
        }

}

//=============== Compartment ==============//

class StateStackCompartment
{

    public int state;

    public StateStackCompartment(int state)
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

class StateStackController : StateStack
{
        public StateStackController() : base()
        {
        }
}

********************/
}
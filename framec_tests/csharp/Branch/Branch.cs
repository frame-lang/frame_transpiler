using FrameLang;
#nullable disable
namespace Branch
{

class Branch
{

    private BranchCompartment _compartment_;
    private BranchCompartment _nextCompartment_;



    public Branch(){


        // Create and intialize start state compartment.


        this._state_ = (int)BranchState.I;
        this._compartment_ = new BranchCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum BranchState
    {

        I = 0,
        SIMPLEIF = 1,
        NEGATEDIF = 2,
        PRECEDENCE = 3,
        NESTEDIF = 4,
        GUARDEDTRANSITION = 5,
        NESTEDGUARDEDTRANSITION = 6,
        F1 = 7,
        F2 = 8,
        F3 = 9
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)BranchState.I:
                this._sI_(e);
                break;
            case (int)BranchState.SIMPLEIF:
                this._sSimpleIf_(e);
                break;
            case (int)BranchState.NEGATEDIF:
                this._sNegatedIf_(e);
                break;
            case (int)BranchState.PRECEDENCE:
                this._sPrecedence_(e);
                break;
            case (int)BranchState.NESTEDIF:
                this._sNestedIf_(e);
                break;
            case (int)BranchState.GUARDEDTRANSITION:
                this._sGuardedTransition_(e);
                break;
            case (int)BranchState.NESTEDGUARDEDTRANSITION:
                this._sNestedGuardedTransition_(e);
                break;
            case (int)BranchState.F1:
                this._sF1_(e);
                break;
            case (int)BranchState.F2:
                this._sF2_(e);
                break;
            case (int)BranchState.F3:
                this._sF3_(e);
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

    public void F() {
        FrameEvent e = new FrameEvent("F",null);
        this._mux_(e);
    }

    public void OnBool(bool b) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["b"] = b;

        FrameEvent e = new FrameEvent("OnBool",parameters);
        this._mux_(e);
    }

    public void OnInt(int i) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["i"] = i;

        FrameEvent e = new FrameEvent("OnInt",parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    BranchCompartment compartment;


    private void _sI_(FrameEvent e)
    {
        if (e._message == "A")
        {


            compartment =  new BranchCompartment((int)BranchState.SIMPLEIF);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "B")
        {


            compartment =  new BranchCompartment((int)BranchState.NEGATEDIF);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "C")
        {


            compartment =  new BranchCompartment((int)BranchState.PRECEDENCE);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "D")
        {


            compartment =  new BranchCompartment((int)BranchState.NESTEDIF);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "E")
        {


            compartment =  new BranchCompartment((int)BranchState.GUARDEDTRANSITION);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "F")
        {


            compartment =  new BranchCompartment((int)BranchState.NESTEDGUARDEDTRANSITION);


            this._transition_(compartment);

            return;
        }
    }

    private void _sSimpleIf_(FrameEvent e)
    {
        if (e._message == "OnBool")
        {
            if (((bool)e._parameters["b"])) {
                this.log_do("then 1");
            } else {
            }
            if (((bool)e._parameters["b"])) {
            } else {
                this.log_do("else 1");
            }
            if (((bool)e._parameters["b"])) {
                this.log_do("then 2");
            } else {
                this.log_do("else 2");
            }
            if (((bool)e._parameters["b"])) {


                compartment =  new BranchCompartment((int)BranchState.F1);


                this._transition_(compartment);
                return;
            } else {


                compartment =  new BranchCompartment((int)BranchState.F2);


                this._transition_(compartment);
                return;
            }

            return;
        }
        else if (e._message == "OnInt")
        {
            if (((int)e._parameters["i"]) > 5) {
                this.log_do("> 5");
            } else {
                this.log_do("<= 5");
            }
            if (((int)e._parameters["i"]) < 10) {
                this.log_do("< 10");
            } else {
                this.log_do(">= 10");
            }
            if (((int)e._parameters["i"]) == 7) {
                this.log_do("== 7");


                compartment =  new BranchCompartment((int)BranchState.F1);


                this._transition_(compartment);
                return;
            } else {
                this.log_do("!= 7");


                compartment =  new BranchCompartment((int)BranchState.F2);


                this._transition_(compartment);
                return;
            }

            return;
        }
    }

    private void _sNegatedIf_(FrameEvent e)
    {
        if (e._message == "OnBool")
        {
            if (!(((bool)e._parameters["b"]))) {
                this.log_do("then 1");
            } else {
            }
            if (!(((bool)e._parameters["b"]))) {
            } else {
                this.log_do("else 1");
            }
            if (!(((bool)e._parameters["b"]))) {
                this.log_do("then 2");
            } else {
                this.log_do("else 2");
            }
            if (!(((bool)e._parameters["b"]))) {


                compartment =  new BranchCompartment((int)BranchState.F1);


                this._transition_(compartment);
                return;
            } else {


                compartment =  new BranchCompartment((int)BranchState.F2);


                this._transition_(compartment);
                return;
            }

            return;
        }
        else if (e._message == "OnInt")
        {
            if (!(((int)e._parameters["i"]) >= 5)) {
                this.log_do("< 5");
            } else {
                this.log_do(">= 5");
            }
            if (!(((int)e._parameters["i"]) <= 10)) {
                this.log_do("> 10");
            } else {
                this.log_do("<= 10");
            }
            if (!(((int)e._parameters["i"]) != 7)) {
                this.log_do("== 7");


                compartment =  new BranchCompartment((int)BranchState.F1);


                this._transition_(compartment);
                return;
            } else {
                this.log_do("!= 7");


                compartment =  new BranchCompartment((int)BranchState.F2);


                this._transition_(compartment);
                return;
            }

            return;
        }
    }

    private void _sPrecedence_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if (-((int)e._parameters["i"]) >= 0 && -((int)e._parameters["i"]) <= 5) {
                this.log_do("then 1");
            } else {
                this.log_do("else 1");
            }
            if (!(((int)e._parameters["i"]) >= -5 && ((int)e._parameters["i"]) <= 5) && (((int)e._parameters["i"]) >= -10 && ((int)e._parameters["i"]) <= 10)) {
                this.log_do("then 2");
            } else {
                this.log_do("else 2");
            }
            if (((int)e._parameters["i"]) >= 0 && ((int)e._parameters["i"]) <= 5 || ((int)e._parameters["i"]) >= 10 && ((int)e._parameters["i"]) <= 20) { 
                this.log_do("then 3");
            } else {
                this.log_do("else 3");
            }
            if (!((((int)e._parameters["i"]) < 0 || ((int)e._parameters["i"]) > 10) && ((int)e._parameters["i"]) + 5 < 20)) {
                this.log_do("then 4");
            } else {
                this.log_do("else 4");
            }

            return;
        }
    }

    private void _sNestedIf_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if (((int)e._parameters["i"]) > 0) {
                this.log_do("> 0");
                if (((int)e._parameters["i"]) < 100) {
                    this.log_do("< 100");


                    compartment =  new BranchCompartment((int)BranchState.F1);


                    this._transition_(compartment);
                    return;
                } else {
                    this.log_do(">= 100");
                }
            } else {
                this.log_do("<= 0");
                if (((int)e._parameters["i"]) > -10) {
                    this.log_do("> -10");
                } else {
                    this.log_do("<= -10");


                    compartment =  new BranchCompartment((int)BranchState.F2);


                    this._transition_(compartment);
                    return;
                }
            }

            return;
        }
    }

    private void _sGuardedTransition_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if (((int)e._parameters["i"]) > 100) {
                this.log_do("-> $F1");


                compartment =  new BranchCompartment((int)BranchState.F1);


                this._transition_(compartment);
                return;
            } else {
            }
            if (!(((int)e._parameters["i"]) > 10)) {
            } else {
                this.log_do("-> $F2");


                compartment =  new BranchCompartment((int)BranchState.F2);


                this._transition_(compartment);
                return;
            }
            this.log_do("-> $F3");


            compartment =  new BranchCompartment((int)BranchState.F3);


            this._transition_(compartment);

            return;
        }
    }

    private void _sNestedGuardedTransition_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if (((int)e._parameters["i"]) > 10) {
                if (((int)e._parameters["i"]) > 100) {
                    this.log_do("-> $F1");


                    compartment =  new BranchCompartment((int)BranchState.F1);


                    this._transition_(compartment);
                    return;
                } else {
                }
                if (((int)e._parameters["i"]) > 50) {
                } else {
                    this.log_do("-> $F2");


                    compartment =  new BranchCompartment((int)BranchState.F2);


                    this._transition_(compartment);
                    return;
                }
            } else {
            }
            this.log_do("-> $F3");


            compartment =  new BranchCompartment((int)BranchState.F3);


            this._transition_(compartment);

            return;
        }
    }

    private void _sF1_(FrameEvent e)
    {
    }

    private void _sF2_(FrameEvent e)
    {
    }

    private void _sF3_(FrameEvent e)
    {
    }

    //===================== Actions Block ===================//

    public void log_do(string msg)
    {
        this.tape.Add(msg);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> tape = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(BranchCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(BranchCompartment nextCompartment)
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

class BranchCompartment
{

    public int state;

    public BranchCompartment(int state)
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

class BranchController : Branch
{
        public BranchController() : base()
        {
        }
}

********************/
}
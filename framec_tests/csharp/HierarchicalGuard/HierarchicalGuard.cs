using FrameLang;
#nullable disable
namespace HierarchicalGuard
{

class HierarchicalGuard
{

    private HierarchicalGuardCompartment _compartment_;
    private HierarchicalGuardCompartment _nextCompartment_;



    public HierarchicalGuard(){


        // Create and intialize start state compartment.


        this._state_ = (int)HierarchicalGuardState.I;
        this._compartment_ = new HierarchicalGuardCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum HierarchicalGuardState
    {

        I = 0,
        S = 1,
        S0 = 2,
        S1 = 3,
        S2 = 4,
        S3 = 5,
        S4 = 6
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)HierarchicalGuardState.I:
                this._sI_(e);
                break;
            case (int)HierarchicalGuardState.S:
                this._sS_(e);
                break;
            case (int)HierarchicalGuardState.S0:
                this._sS0_(e);
                break;
            case (int)HierarchicalGuardState.S1:
                this._sS1_(e);
                break;
            case (int)HierarchicalGuardState.S2:
                this._sS2_(e);
                break;
            case (int)HierarchicalGuardState.S3:
                this._sS3_(e);
                break;
            case (int)HierarchicalGuardState.S4:
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

    public void A(int i) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["i"] = i;

        FrameEvent e = new FrameEvent("A",parameters);
        this._mux_(e);
    }

    public void B(int i) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["i"] = i;

        FrameEvent e = new FrameEvent("B",parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    HierarchicalGuardCompartment compartment;


    private void _sI_(FrameEvent e)
    {
        if (e._message == ">")
        {


            compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S);


            this._transition_(compartment);

            return;
        }
    }

    private void _sS_(FrameEvent e)
    {
        if (e._message == "A")
        {
            this.log_do("S.A");
            if ((int)e._parameters["i"] < 10) {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S0);


                this._transition_(compartment);
                return;
            } else {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S1);


                this._transition_(compartment);
                return;
            }

            return;
        }
        else if (e._message == "B")
        {
            this.log_do("S.B");
            if ((int)e._parameters["i"] < 10) {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S2);


                this._transition_(compartment);
                return;
            } else {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S3);


                this._transition_(compartment);
                return;
            }

            return;
        }
    }

    private void _sS0_(FrameEvent e)
    {
        if (e._message == "A")
        {
            this.log_do("S0.A");
            if ((int)e._parameters["i"] > 0) {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S2);


                this._transition_(compartment);
                return;
            } else {
            }

        }
          //  fall through else branch
        else if (e._message == "B")
        {
            this.log_do("S0.B");
            if ((int)e._parameters["i"] > 0) {
            } else {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S1);


                this._transition_(compartment);
                return;
            }

        }
        _sS_(e);

    }  //  fall through then branch


    private void _sS1_(FrameEvent e)
    {
        if (e._message == "A")
        {
            this.log_do("S1.A");
            if ((int)e._parameters["i"] > 5) {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S3);


                this._transition_(compartment);
                return;
            } else {
            }

        }
        _sS0_(e);

    }  //  fall through else branch


    private void _sS2_(FrameEvent e)
    {
        if (e._message == "A")
        {
            this.log_do("S2.A");
            if ((int)e._parameters["i"] > 10) {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S4);


                this._transition_(compartment);
                return;
            } else {
            }

        }
          //  fall through then branch
        else if (e._message == "B")
        {
            this.log_do("S2.B");
            if (!((int)e._parameters["i"] > 10)) {
            } else {


                compartment =  new HierarchicalGuardCompartment((int)HierarchicalGuardState.S4);


                this._transition_(compartment);
                return;
            }

        }
        _sS1_(e);

    }  //  fall through then branch


    private void _sS3_(FrameEvent e)
    {
        if (e._message == "A")
        {
            this.log_do("S3.A");
            if ((int)e._parameters["i"] > 0) {
                this.log_do("stop");

                return;
            } else {
                this.log_do("continue");
            }

        }
        else if (e._message == "B")
        {
            this.log_do("S3.B");
            if ((int)e._parameters["i"] > 0) {
                this.log_do("continue");
            } else {
                this.log_do("stop");

                return;
            }

        }
        _sS_(e);

    }

    private void _sS4_(FrameEvent e)
    {
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

    private void _transition_(HierarchicalGuardCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(HierarchicalGuardCompartment nextCompartment)
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

class HierarchicalGuardCompartment
{

    public int state;

    public HierarchicalGuardCompartment(int state)
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

class HierarchicalGuardController : HierarchicalGuard
{
        public HierarchicalGuardController() : base()
        {
        }
}

********************/
}
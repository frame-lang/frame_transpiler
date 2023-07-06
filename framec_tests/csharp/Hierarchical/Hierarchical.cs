using FrameLang;
#nullable disable
namespace Hierarchical
{

class Hierarchical
{

    private HierarchicalCompartment _compartment_;
    private HierarchicalCompartment _nextCompartment_;



    public Hierarchical(){


        // Create and intialize start state compartment.


        this._state_ = (int)HierarchicalState.I;
        this._compartment_ = new HierarchicalCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum HierarchicalState
    {

        I = 0,
        S = 1,
        S0 = 2,
        S1 = 3,
        S2 = 4,
        S3 = 5,
        T = 6
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)HierarchicalState.I:
                this._sI_(e);
                break;
            case (int)HierarchicalState.S:
                this._sS_(e);
                break;
            case (int)HierarchicalState.S0:
                this._sS0_(e);
                break;
            case (int)HierarchicalState.S1:
                this._sS1_(e);
                break;
            case (int)HierarchicalState.S2:
                this._sS2_(e);
                break;
            case (int)HierarchicalState.S3:
                this._sS3_(e);
                break;
            case (int)HierarchicalState.T:
                this._sT_(e);
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


    //===================== Machine Block ===================//

    HierarchicalCompartment compartment;


    private void _sI_(FrameEvent e)
    {
        if (e._message == ">")
        {


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S);


            this._transition_(compartment);

            return;
        }
    }

    private void _sS_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S");

            return;
        }
        else if (e._message == "<")
        {
            this.exit_do("S");

            return;
        }
        else if (e._message == "A")
        {
            this.log_do("S.A");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S0);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "B")
        {
            this.log_do("S.B");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S1);


            this._transition_(compartment);

            return;
        }
    }

    private void _sS0_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S0");

        }
        else if (e._message == "<")
        {
            this.exit_do("S0");

        }
          //  override parent handler
        else if (e._message == "A")
        {
            this.log_do("S0.A");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.T);


            this._transition_(compartment);

            return;
        }
          //  do this, then parent handler
        else if (e._message == "B")
        {
            this.log_do("S0.B");

        }
          //  extend parent handler
        else if (e._message == "C")
        {
            this.log_do("S0.C");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S2);


            this._transition_(compartment);

            return;
        }
        _sS_(e);

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
          //  defer to parent for A
          //  do this, then parent, which transitions here
        else if (e._message == "B")
        {
            this.log_do("S1.B");

        }
          //  propagate message not handled by parent
        else if (e._message == "C")
        {
            this.log_do("S1.C");

        }
        _sS_(e);

    }

    private void _sS2_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S2");

        }
        else if (e._message == "<")
        {
            this.exit_do("S2");

        }
          //  will propagate to S0 and S
        else if (e._message == "B")
        {
            this.log_do("S2.B");

        }
        else if (e._message == "C")
        {
            this.log_do("S2.C");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.T);


            this._transition_(compartment);

            return;
        }
        _sS0_(e);

    }  //  continue after transition (should be ignored)


    private void _sS3_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("S3");

        }
        else if (e._message == "<")
        {
            this.exit_do("S3");

        }
          //  defer to grandparent for A
          //  override and move to sibling
        else if (e._message == "B")
        {
            this.log_do("S3.B");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S2);


            this._transition_(compartment);

            return;
        }
        _sS1_(e);

    }

    private void _sT_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.enter_do("T");

            return;
        }
        else if (e._message == "<")
        {
            this.exit_do("T");

            return;
        }
        else if (e._message == "A")
        {
            this.log_do("T.A");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "B")
        {
            this.log_do("T.B");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S2);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "C")
        {
            this.log_do("T.C");


            compartment =  new HierarchicalCompartment((int)HierarchicalState.S3);


            this._transition_(compartment);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void enter_do(string msg)
    {
        this.enters.Add(msg);
    }

    public void exit_do(string msg)
    {
        this.exits.Add(msg);
    }

    public void log_do(string msg)
    {
        this.tape.Add(msg);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> enters  = new List<string>();
    public List<string> exits  = new List<string>();
    public List<string> tape  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(HierarchicalCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(HierarchicalCompartment nextCompartment)
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

class HierarchicalCompartment
{

    public int state;

    public HierarchicalCompartment(int state)
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

class HierarchicalController : Hierarchical
{
        public HierarchicalController() : base()
        {
        }
}

********************/
}
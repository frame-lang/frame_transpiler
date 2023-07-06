using FrameLang;
#nullable disable
namespace VarScope
{

class VarScope
{

    public VarScopeCompartment _compartment_;
    public VarScopeCompartment _nextCompartment_;



    public VarScope(){


        // Create and intialize start state compartment.

        this._state_ = (int)VarScopeState.INIT;
        this._compartment_ = new VarScopeCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum VarScopeState
    {

        INIT = 0,
        NN = 1,
        NY = 2,
        YN = 3,
        YY = 4
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)VarScopeState.INIT:
                this._sInit_(e);
                break;
            case (int)VarScopeState.NN:
                this._sNN_(e);
                break;
            case (int)VarScopeState.NY:
                this._sNY_(e);
                break;
            case (int)VarScopeState.YN:
                this._sYN_(e);
                break;
            case (int)VarScopeState.YY:
                this._sYY_(e);
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

    public void to_nn() {
        FrameEvent e = new FrameEvent("to_nn",null);
        this._mux_(e);
    }

    public void to_ny() {
        FrameEvent e = new FrameEvent("to_ny",null);
        this._mux_(e);
    }

    public void to_yn() {
        FrameEvent e = new FrameEvent("to_yn",null);
        this._mux_(e);
    }

    public void to_yy() {
        FrameEvent e = new FrameEvent("to_yy",null);
        this._mux_(e);
    }

    public void nn(string d) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["d"] = d;

        FrameEvent e = new FrameEvent("nn",parameters);
        this._mux_(e);
    }

    public void ny(string d) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["d"] = d;

        FrameEvent e = new FrameEvent("ny",parameters);
        this._mux_(e);
    }

    public void yn(string d,string x) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["d"] = d;

        parameters["x"] = x;

        FrameEvent e = new FrameEvent("yn",parameters);
        this._mux_(e);
    }

    public void yy(string d,string x) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["d"] = d;

        parameters["x"] = x;

        FrameEvent e = new FrameEvent("yy",parameters);
        this._mux_(e);
    }

    public void sigils(string x) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["x"] = x;

        FrameEvent e = new FrameEvent("sigils",parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    VarScopeCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == "to_nn")
        {


            compartment =  new VarScopeCompartment((int)VarScopeState.NN);

            compartment.StateArgs["b"] = "$NN[b]";

            compartment.StateVars["c"] = "$NN.c";

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_ny")
        {


            compartment =  new VarScopeCompartment((int)VarScopeState.NY);

            compartment.StateArgs["b"] = "$NY[b]";

            compartment.StateVars["c"] = "$NY.c";

            compartment.StateVars["x"] = "$NY.x";

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_yn")
        {


            compartment =  new VarScopeCompartment((int)VarScopeState.YN);

            compartment.StateArgs["b"] = "$YN[b]";
            compartment.StateArgs["x"] = "$YN[x]";

            compartment.StateVars["c"] = "$YN.c";

            this._transition_(compartment);

            return;
        }
        else if (e._message == "to_yy")
        {


            compartment =  new VarScopeCompartment((int)VarScopeState.YY);

            compartment.StateArgs["b"] = "$YY[b]";
            compartment.StateArgs["x"] = "$YY[x]";

            compartment.StateVars["c"] = "$YY.c";

            compartment.StateVars["x"] = "$YY.x";

            this._transition_(compartment);

            return;
        }
    }

    private void _sNN_(FrameEvent e)
    {
        if (e._message == "nn")
        {
            string et  = "|nn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(this.x);

            return;
        }
        else if (e._message == "ny")
        {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "yn")
        {
            string et  = "|yn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do((string)e._parameters["x"]);

            return;
        }
        else if (e._message == "yy")
        {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "sigils")
        {
            this.log_do(this.x);

            return;
        }
    }  //  var x:string = "|sigils|.x"
      //  log(||[x])
      //  log(||.x)


    private void _sNY_(FrameEvent e)
    {
        if (e._message == "nn")
        {
            string et  = "|nn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(((string)this._compartment_.StateVars["x"]));

            return;
        }
        else if (e._message == "ny")
        {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "yn")
        {
            string et  = "|yn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do((string)e._parameters["x"]);

            return;
        }
        else if (e._message == "yy")
        {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "sigils")
        {
            this.log_do(this.x);

            return;
        }
    }  //  var x:string = "|sigils|.x"
      //  log($.x)
      //  log(||[x])
      //  log(||.x)


    private void _sYN_(FrameEvent e)
    {
        if (e._message == "nn")
        {
            string et  = "|nn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do((string)this._compartment_.StateArgs["x"]);

            return;
        }
        else if (e._message == "ny")
        {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "yn")
        {
            string et  = "|yn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do((string)e._parameters["x"]);

            return;
        }
        else if (e._message == "yy")
        {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "sigils")
        {
            this.log_do(this.x);

            return;
        }
    }  //  var x:string = "|sigils|.x"
      //  log($[x])
      //  log(||[x])
      //  log(||.x)


    private void _sYY_(FrameEvent e)
    {
        if (e._message == "nn")
        {
            string et  = "|nn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(((string)this._compartment_.StateVars["x"]));

            return;
        }
        else if (e._message == "ny")
        {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "yn")
        {
            string et  = "|yn|.e";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do((string)e._parameters["x"]);

            return;
        }
        else if (e._message == "yy")
        {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            this.log_do(this.a);
            this.log_do((string)this._compartment_.StateArgs["b"]);
            this.log_do(((string)this._compartment_.StateVars["c"]));
            this.log_do((string)e._parameters["d"]);
            this.log_do(et);
            this.log_do(x);

            return;
        }
        else if (e._message == "sigils")
        {
            this.log_do(this.x);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void log_do(string s)
    {
        this.tape.Add(s);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public string a  = "#.a";
    public string x  = "#.x";
    public List<string> tape  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(VarScopeCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(VarScopeCompartment nextCompartment)
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

class VarScopeCompartment
{

    public int state;

    public VarScopeCompartment(int state)
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

class VarScopeController : VarScope
{
        public VarScopeController() : base()
        {
        }
}

********************/
}
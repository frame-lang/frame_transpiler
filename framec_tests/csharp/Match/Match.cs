using FrameLang;
#nullable disable
namespace Match
{

class Match
{

    private MatchCompartment _compartment_;
    private MatchCompartment _nextCompartment_;



    public Match(){


        // Create and intialize start state compartment.


        this._state_ = (int)MatchState.INIT;
        this._compartment_ = new MatchCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum MatchState
    {

        INIT = 0,
        EMPTYMATCH = 1,
        SIMPLEMATCH = 2,
        MULTIMATCH = 3,
        NESTEDMATCH = 4,
        CHILDMATCH = 5,
        FINAL = 6
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)MatchState.INIT:
                this._sInit_(e);
                break;
            case (int)MatchState.EMPTYMATCH:
                this._sEmptyMatch_(e);
                break;
            case (int)MatchState.SIMPLEMATCH:
                this._sSimpleMatch_(e);
                break;
            case (int)MatchState.MULTIMATCH:
                this._sMultiMatch_(e);
                break;
            case (int)MatchState.NESTEDMATCH:
                this._sNestedMatch_(e);
                break;
            case (int)MatchState.CHILDMATCH:
                this._sChildMatch_(e);
                break;
            case (int)MatchState.FINAL:
                this._sFinal_(e);
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

    public void Empty() {
        FrameEvent e = new FrameEvent("Empty",null);
        this._mux_(e);
    }

    public void Simple() {
        FrameEvent e = new FrameEvent("Simple",null);
        this._mux_(e);
    }

    public void Multi() {
        FrameEvent e = new FrameEvent("Multi",null);
        this._mux_(e);
    }

    public void Nested() {
        FrameEvent e = new FrameEvent("Nested",null);
        this._mux_(e);
    }

    public void Child() {
        FrameEvent e = new FrameEvent("Child",null);
        this._mux_(e);
    }

    public void OnInt(int i) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["i"] = i;

        FrameEvent e = new FrameEvent("OnInt",parameters);
        this._mux_(e);
    }

    public void OnString(string s) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["s"] = s;

        FrameEvent e = new FrameEvent("OnString",parameters);
        this._mux_(e);
    }


    //===================== Machine Block ===================//

    MatchCompartment compartment;


    private void _sInit_(FrameEvent e)
    {
        if (e._message == "Empty")
        {


            compartment =  new MatchCompartment((int)MatchState.EMPTYMATCH);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "Simple")
        {


            compartment =  new MatchCompartment((int)MatchState.SIMPLEMATCH);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "Multi")
        {


            compartment =  new MatchCompartment((int)MatchState.MULTIMATCH);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "Nested")
        {


            compartment =  new MatchCompartment((int)MatchState.NESTEDMATCH);


            this._transition_(compartment);

            return;
        }
        else if (e._message == "Child")
        {


            compartment =  new MatchCompartment((int)MatchState.CHILDMATCH);


            this._transition_(compartment);

            return;
        }
    }

    private void _sEmptyMatch_(FrameEvent e)
    {
        if (e._message == "OnString")
        {
            if (((string)e._parameters["s"] == "") || ((string)e._parameters["s"] == "foo")) {
                this.log_do("empty");
            } else {
                this.log_do("?");
            }

            return;
        }
    }  //  TODO: matching only the empty string is broken


    private void _sSimpleMatch_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if (((int)e._parameters["i"] == 0)) {
                this.log_do("0");
            } else if (((int)e._parameters["i"] == 42)) {
                this.log_do("42");
            } else if (((int)e._parameters["i"] == 42)) {
                this.log_do("!!!");
            } else if (((int)e._parameters["i"] == -200)) {
                this.log_do("-200");
            } else {
                this.log_do("?");
            }

            return;
        }
        else if (e._message == "OnString")
        {
            if (((string)e._parameters["s"] == "hello")) {
                this.log_do("hello");
            } else if (((string)e._parameters["s"] == "hello")) {
                this.log_do("!!!");
            } else if (((string)e._parameters["s"] == "goodbye")) {
                this.log_do("goodbye");
            } else if (((string)e._parameters["s"] == "Testing 1, 2, 3...")) {
                this.log_do("testing");
            } else if (((string)e._parameters["s"] == "$10!")) {
                this.log_do("money");
            } else {
                this.log_do("?");
            }

            return;
        }
    }

    private void _sMultiMatch_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if (((int)e._parameters["i"] == 3) || ((int)e._parameters["i"] == -7)) {
                this.log_do("3|-7");
            } else if (((int)e._parameters["i"] == -4) || ((int)e._parameters["i"] == 5) || ((int)e._parameters["i"] == 6)) {
                this.log_do("-4|5|6");
            } else {
                this.log_do("?");
            }

            return;
        }
        else if (e._message == "OnString")
        {
            if (((string)e._parameters["s"] == "$10") || ((string)e._parameters["s"] == "12.5%") || ((string)e._parameters["s"] == "@#*!")) {
                this.log_do("symbols");
            } else if (((string)e._parameters["s"] == " ") || ((string)e._parameters["s"] == "  ") || ((string)e._parameters["s"] == "\t") || ((string)e._parameters["s"] == "\n")) {
                this.log_do("whitespace");
            } else {
                this.log_do("?");
            }

            return;
        }
    }

    private void _sNestedMatch_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if ((int)e._parameters["i"] > 0) {
                if (((int)e._parameters["i"] == 1) || ((int)e._parameters["i"] == 2) || ((int)e._parameters["i"] == 3)) {
                    this.log_do("1-3");
                    if (((int)e._parameters["i"] == 1)) {
                        this.log_do("1");
                    } else if (((int)e._parameters["i"] == 2)) {
                        this.log_do("2");
                    } else {
                        this.log_do("3");
                    }
                } else if (((int)e._parameters["i"] == 4) || ((int)e._parameters["i"] == 5)) {
                    this.log_do("4-5");
                    if ((int)e._parameters["i"] == 4) {
                        this.log_do("4");
                    } else {
                        this.log_do("5");
                    }
                } else {
                    this.log_do("too big");
                }
            } else {
                this.log_do("too small");
            }

            return;
        }
        else if (e._message == "OnString")
        {
            if (((string)e._parameters["s"] == "hello") || ((string)e._parameters["s"] == "hola") || ((string)e._parameters["s"] == "bonjour")) {
                this.log_do("greeting");
                if (((string)e._parameters["s"] == "hello")) {
                    this.log_do("English");
                } else if (((string)e._parameters["s"] == "hola")) {
                    this.log_do("Spanish");
                } else {
                    this.log_do("French");
                }
            } else if (((string)e._parameters["s"] == "goodbye") || ((string)e._parameters["s"] == "adios") || ((string)e._parameters["s"] == "au revoir")) {
                this.log_do("farewell");
                if (((string)e._parameters["s"] == "goodbye")) {
                    this.log_do("English");
                } else if (((string)e._parameters["s"] == "adios")) {
                    this.log_do("Spanish");
                } else {
                    this.log_do("French");
                }
            } else {
                this.log_do("?");
            }

            return;
        }
    }

    private void _sChildMatch_(FrameEvent e)
    {
        if (e._message == "OnInt")
        {
            if (((int)e._parameters["i"] == 0)) {


                compartment =  new MatchCompartment((int)MatchState.FINAL);


                this._transition_(compartment);
                return;
            } else if (((int)e._parameters["i"] == 3)) {
                this.log_do("3");
            } else if (((int)e._parameters["i"] == 4)) {
                this.log_do("4");

                return;
            } else if (((int)e._parameters["i"] == 42)) {
                this.log_do("42 in child");
            } else if (((int)e._parameters["i"] == 5)) {
                this.log_do("5");


                compartment =  new MatchCompartment((int)MatchState.FINAL);


                this._transition_(compartment);
                return;
            } else {
                this.log_do("no match in child");
            }

        }
        else if (e._message == "OnString")
        {
            if (((string)e._parameters["s"] == "hello")) {
                this.log_do("hello in child");
            } else if (((string)e._parameters["s"] == "goodbye")) {


                compartment =  new MatchCompartment((int)MatchState.FINAL);


                this._transition_(compartment);
                return;
            } else if (((string)e._parameters["s"] == "Testing 1, 2, 3...")) {
                this.log_do("testing in child");

                return;
            } else {
                this.log_do("no match in child");
            }

        }
        _sSimpleMatch_(e);

    }

    private void _sFinal_(FrameEvent e)
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

    private void _transition_(MatchCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(MatchCompartment nextCompartment)
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

class MatchCompartment
{

    public int state;

    public MatchCompartment(int state)
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

class MatchController : Match
{
        public MatchController() : base()
        {
        }
}

********************/
}
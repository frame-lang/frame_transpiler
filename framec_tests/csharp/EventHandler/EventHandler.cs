using FrameLang;
#nullable disable
namespace EventHandler
{

class EventHandler
{

    private EventHandlerCompartment _compartment_;
    private EventHandlerCompartment _nextCompartment_;



    public EventHandler(){


        // Create and intialize start state compartment.


        this._state_ = (int)EventHandlerState.S1;
        this._compartment_ = new EventHandlerCompartment(this._state_);
        this._nextCompartment_ = null;


        // Send system start event
        FrameEvent frameEvent = new FrameEvent(">", null);
        this._mux_(frameEvent);

    }

    // states enum
    private enum EventHandlerState
    {

        S1 = 0,
        S2 = 1
    }
    //====================== Multiplexer ====================//

    private void _mux_(FrameEvent e)
    {
        switch (this._compartment_.state)
        {
            case (int)EventHandlerState.S1:
                this._sS1_(e);
                break;
            case (int)EventHandlerState.S2:
                this._sS2_(e);
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

    public void LogIt(int x) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["x"] = x;

        FrameEvent e = new FrameEvent("LogIt",parameters);
        this._mux_(e);
    }

    public void LogAdd(int a,int b) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["a"] = a;

        parameters["b"] = b;

        FrameEvent e = new FrameEvent("LogAdd",parameters);
        this._mux_(e);
    }

    public int LogReturn(int a,int b) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["a"] = a;

        parameters["b"] = b;

        FrameEvent e = new FrameEvent("LogReturn",parameters);
        this._mux_(e);
        return (int) e._return;
    }

    public void PassAdd(int a,int b) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["a"] = a;

        parameters["b"] = b;

        FrameEvent e = new FrameEvent("PassAdd",parameters);
        this._mux_(e);
    }

    public int PassReturn(int a,int b) {
        Dictionary<string,object> parameters = new Dictionary<string,object>();
        parameters["a"] = a;

        parameters["b"] = b;

        FrameEvent e = new FrameEvent("PassReturn",parameters);
        this._mux_(e);
        return (int) e._return;
    }


    //===================== Machine Block ===================//

    EventHandlerCompartment compartment;


    private void _sS1_(FrameEvent e)
    {
        if (e._message == "LogIt")
        {
            this.log_do("x",(int)e._parameters["x"]);

            return;
        }
        else if (e._message == "LogAdd")
        {
            this.log_do("a",(int)e._parameters["a"]);
            this.log_do("b",(int)e._parameters["b"]);
            this.log_do("a+b",(int)e._parameters["a"] + (int)e._parameters["b"]);
            
            return;
        }
        else if (e._message == "LogReturn")
        {
            this.log_do("a",(int)e._parameters["a"]);
            this.log_do("b",(int)e._parameters["b"]);
            int r  = (int)e._parameters["a"] + (int)e._parameters["b"];
            this.log_do("r",r);
            e._return = r;

            return;

        }
        else if (e._message == "PassAdd")
        {


            compartment =  new EventHandlerCompartment((int)EventHandlerState.S2);

            compartment.StateArgs["p"] = (int)e._parameters["a"] + (int)e._parameters["b"];

            this._transition_(compartment);

            return;
        }
        else if (e._message == "PassReturn")
        {
            int r  = (int)e._parameters["a"] + (int)e._parameters["b"];
            this.log_do("r",r);


            compartment =  new EventHandlerCompartment((int)EventHandlerState.S2);

            compartment.StateArgs["p"] = r;

            this._transition_(compartment);
            e._return = r;

            return;

        }
    }

    private void _sS2_(FrameEvent e)
    {
        if (e._message == ">")
        {
            this.log_do("p",(int)this._compartment_.StateArgs["p"]);

            return;
        }
    }

    //===================== Actions Block ===================//

    public void log_do(string msg, int val)
    {

        string value = msg + "=" + val.ToString();
        this.tape.Add(value);
    }

    // Unimplemented Actions


    //===================== Domain Block ===================//

    public List<string> tape  = new List<string>();


    //=============== Machinery and Mechanisms ==============//

    private int _state_;

    private void _transition_(EventHandlerCompartment compartment)
    {
        this._nextCompartment_ = compartment;
    }

    private void _doTransition_(EventHandlerCompartment nextCompartment)
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

class EventHandlerCompartment
{

    public int state;

    public EventHandlerCompartment(int state)
    {
        this.state = state;
    }

    public Dictionary<string, object> StateArgs { get; set; } = new Dictionary<string, object>();
    public Dictionary<string, object> StateVars { get; set; } = new Dictionary<string, object>();
    public Dictionary<string, object> EnterArgs { get; set; } = new Dictionary<string, object>();
    public Dictionary<string, object> ExitArgs { get; set; } = new Dictionary<string, object>();
    public FrameEvent _forwardEvent = new FrameEvent();
}


// class EventHandlerController : EventHandler
// {
//         public EventHandlerController() : base()
//         {
//         }
// }
}
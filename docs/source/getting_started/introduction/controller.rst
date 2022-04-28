==================
System Controllers
==================

We will now bring all the pieces together as well as explore
the structure of the generated code. Here is the complete ``#Lamp`` spec:

.. code-block::

    #Lamp

    -interface-

    turnOn
    turnOff
    setColor [color:string]
    getColor : string

    -machine-

    $Off
        |turnOn|
            -> $On ^
        |getColor| : string
            ^(color)
        |setColor| [color:string]
            #.color = color ^

    $On
        |>|
            closeSwitch() ^
        |<|
            openSwitch() ^
        |turnOff|
            -> $Off ^
        |getColor| : string
            ^(color)
        |setColor| [color:string]
            #.color = color ^

    -actions-

    closeSwitch
    openSwitch

    -domain-

    var color:string = "white"

    ##

Now lets explore the code this spec generates.
This output is known as *the controller*.

The Start State
---------------
The first thing that must be initialized in the system is the start state. This can be
done by direct member variable initialization or in a constructor depending
on the particular capabilities of the language. Here we see it done in the
constructor.

.. code-block::

    public Lamp() {
        _state_ = _sOff_;
    }


The Interface
-------------

The interface methods perform the following functions:

#. Create a FrameEvent and initialize its message and parameters
#. Send the event to the state machine
#. Return the message's return value to the caller

.. code-block::

     //===================== Interface Block ===================//

     public void turnOn() {
         FrameEvent e = new FrameEvent("turnOn",null);
         _mux_(e);
     }

     public void turnOff() {
         FrameEvent e = new FrameEvent("turnOff",null);
         _mux_(e);
     }

     public void setColor(string color) {
         Dictionary<String,object> parameters = new Dictionary<String,object>();
         parameters["color"] = color;

         FrameEvent e = new FrameEvent("setColor",parameters);
         _mux_(e);
     }

     public string getColor() {
         FrameEvent e = new FrameEvent("getColor",null);
         _mux_(e);
         return (string) e._return;
     }

The Mux
-------

The Mux, or Multiplexer, is the private method containing the switch statement
that routes the incoming event to the current state method - in other words,
the state machine:

.. code-block::

    //====================== Multiplexer ====================//

    func (m *lampStruct) _mux_(e *framelang.FrameEvent) {
        switch m._compartment_.State {
        case LampState_Off:
            m._LampState_Off_(e)
        case LampState_On:
            m._LampState_On_(e)

        ...
    }

.. note::

    There is more to the Multiplexer than is shown in the snippet above.
    See :ref:`multiplexer` for details.


The Machine Block
-----------------

The Machine Block contains a method for each state. Inside of each state
method is a simple if-elseif or switch block that matches the event message
and routes it to the correct behavior for the message.

.. code-block::

    //===================== Machine Block ===================//

    private void _sOff_(FrameEvent e) {
        if (e._message.Equals("turnOn")) {
            _transition_(_sOn_);
            return;
        }
        else if (e._message.Equals("getColor")) {
            e._return = this.color;
            return;

        }
        else if (e._message.Equals("setColor")) {
            this.color = ((string) e._parameters["color"]);
            return;
        }
    }

    private void _sOn_(FrameEvent e) {
        if (e._message.Equals(">")) {
            turnOnLamp_do();
            return;
        }
        else if (e._message.Equals("<")) {
            turnOffLamp_do();
            return;
        }
        else if (e._message.Equals("turnOff")) {
            _transition_(_sOff_);
            return;
        }
        else if (e._message.Equals("getColor")) {
            e._return = this.color;
            return;

        }
        else if (e._message.Equals("setColor")) {
            this.color = ((string) e._parameters["color"]);
            return;
        }
    }


The Actions Block
-----------------

By default the Actions Block contains non-public stub methods, if appropriate
for the language, for the actions. Alternatives for embedding native code in actions
will be discussed later.

.. code-block::

    //===================== Actions Block ===================//

    protected virtual void turnOnLamp_do() { throw new NotImplementedException(); }
    protected virtual void turnOffLamp_do() { throw new NotImplementedException(); }

The Domain Block
----------------

The Domain Block contains the initialized system variables.

.. code-block::

    //===================== Domain Block ===================//

    string color = "white";

The Transition Machinery
------------------------

Frame generates supporting runtime code for the system mechanisms
as appropriate for the target language.

.. code-block::

    //=============== Machinery and Mechanisms ==============//

    private Compartment _compartment_;

    func (m *lampStruct) _transition_(compartment *LampCompartment) {
        m._nextCompartment_ = compartment
    }

    func (m *lampStruct) _do_transition_(nextCompartment *LampCompartment) {
        m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
        m._compartment_ = nextCompartment
        m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
    }

Here we can see the declaration of the compartment runtime variable,
the deferred transition method and the transition execution method. These, together
with the multiplexer, comprise the bulk of the Frame controller machinery.

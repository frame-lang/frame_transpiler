.. _compartments_sec:

Compartments
============

Frame controllers are the generated code from Frame specs. As we have seen,
the controller's current state is tracked in a Frame managed runtime variable:

.. code-block::

    var _state_ = OFF

However, transition parameters, state paramenters and state variables
now mean there is more data associated with a
state than just the state variable. To accomplish this, Frame introduces
the idea of the **Compartment**. A compartment is, in essence, a *state closure*.

Closures are a concept from programming languages that tie references to anonymous functions
to the environment that existed when they were created. Frame compartments
are a similar concept, but instead of a function carrying its associated environment
with it, compartments enable instances of states to maintain their own environments.

While closures sound mysterious, at the end of the day they are basically lookup tables
for variables. Therefore compartments are simply a data structure consisting of
a state variable and the state's environment:

.. code-block::

    type SomeCompartment struct {
        State SomeState                      // - state variable
        StateArgs map[string]interface{}     // - map of state arguments
        StateVars map[string]interface{}     // - map of state variables
        EnterArgs map[string]interface{}     // - map of transition enter arguments
        ExitArgs map[string]interface{}      // - map of transition exit arguments
        _forwardEvent_ *framelang.FrameEvent // - system runtime data member for Event Forwarding feature
    }

Let us now discuss each member in detail.

The State Member
----------------------------

The state variable holds the identifier for the state and is immutable so
should be implemented as a const if supported by the language.

The StateArgs Member
--------------------------------

The StateArgs are a map containing the data passed as part of a transition
to a new state:

.. code-block::

    -> $NextState(<state_args>)


The StateVars Member
--------------------------------

The StateVars member is a map containing the initializer data for each
state variable member:


.. code-block::

    $JoeName
        var name:string = "Joe"

The EnterArgs Member
--------------------------------

The EnterArgs member is a map containing the initializer data for each
member of the enter event parameters for a transition:

.. code-block::

    -> ("Mark") $PrintName

The ExitArgs Member
-------------------------------

The ExitArgs member is a map containing the initializer data for each
member of the exit event parameters for a transition:

.. code-block::

    $OuttaHere
        |gottaGo|
            ("cya") -> $NextState ^
        |<| [exitMsg:string]
            print(exitMsg) ^ --- prints "cya"


The _forwardEvent_ Member
-------------------------------------

The _forwardEvent_ member is used by the system runtime code to support event
forwarding semantics:

.. code-block::

    $WrongStateToHandleE1
        |e1|
            -> => $RightStateToHandleE1State ^

    $RightStateToHandleE1State
        |e1| --- This is the same e1 event object!
            handleE1Now(@) ^


Compartment Initialization
--------------------------

The Framepiler generates code for both the system compartment structure declaration
as well as a factory function for creating new ones. So for this system spec:

``Frame``

.. code-block::

    #MySystem
    ##

this code would be generated:

.. code-block::

    type MySystemCompartment struct {
        State MySystemState
        StateArgs map[string]interface{}
        StateVars map[string]interface{}
        EnterArgs map[string]interface{}
        ExitArgs map[string]interface{}
        _forwardEvent_ *framelang.FrameEvent
    }

    func NewMySystemCompartment(state MySystemState) *MySystemCompartment {
        c := &MySystemCompartment{State: state}
        c.StateArgs = make(map[string]interface{})
        c.StateVars = make(map[string]interface{})
        c.EnterArgs = make(map[string]interface{})
        c.ExitArgs = make(map[string]interface{})
        return c
    }

Compartments are allocated and initialized in event handlers as the first stage
of a transition. Lets explore the controller code for a basic transition:

``Frame``

.. code-block::

    #TransitionExample

    -machine-

    $From
        |>| -> $To ^

    $To

    ##

The spec generates the controller class/struct that contains two runtime
data members related to compartments:

.. code-block::

    type transitionExampleStruct struct {
        _compartment_ *TransitionExampleCompartment
        _nextCompartment_ *TransitionExampleCompartment
    }

``_compartment_`` variable always holds a reference to the current compartment while
the ``_nextCompartment_`` holds a reference to the next compartment
and is a key runtime mechanism for the :ref:`deferred transitions <deferred_transitions>`
capability of Frame controllers.

The following code related to the transition from ``$From`` to ``$To``:

.. code-block::

    func (m *transitionExampleStruct) _TransitionExampleState_From_(e *framelang.FrameEvent) {
        switch e.Msg {
        case ">":
            compartment := NewTransitionExampleCompartment(TransitionExampleState_To)
            m._transition_(compartment)
            return
        }
    }

In this simple case, the ``NewTransitionCompartmentCompartment`` factory simply takes the id of the
state being transitioned to, in this case ``$To``.

.. code-block::

    //=============== Machinery and Mechanisms ==============//

    func (m *transitionExampleStruct) _transition_(compartment *TransitionExampleCompartment) {
        m._nextCompartment_ = compartment
    }

    ...

As we can see, the ``_transition_`` method simply caches off a reference to
the newly constructed transition and then returns.

If a transition occurs in an event handler, the event handler is required to
return immediately to the ``multiplexer``:

.. code-block::


    //====================== Multiplexer ====================//

    func (m *transitionExampleStruct) _mux_(e *framelang.FrameEvent) {
        switch m._compartment_.State {
        case TransitionExampleState_From:
            m._TransitionExampleState_From_(e)
        case TransitionExampleState_To:
            m._TransitionExampleState_To_(e)
        }

        // NOTE: this is a simplified version of the _do_transition_() logic
        if m._nextCompartment_ != nil {
            m._do_transition_(m._nextCompartment_)
        }
    }

The ``m._TransitionCompartmentState_From_(e)`` state function call returns
and then tests if a transition occurred by seeing if ``m._nextCompartment_`` is
set:

.. code-block::

    // NOTE: this is a simplified version of the _do_transition_() logic
    if m._nextCompartment_ != nil {
        m._do_transition_(m._nextCompartment_)
    }

If so, it performs the transition:

.. code-block::

    //=============== Machinery and Mechanisms ==============//

    ...

    func (m *transitionExampleStruct) _do_transition_(nextCompartment *TransitionExampleCompartment) {
        m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
        m._compartment_ = nextCompartment
        m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
    }

This is the basic pattern for a "simple" transition. Let's take a look at
a transition with all of the data passing in place:

.. code-block::

    #TransitionExampleWithDataPassing

    -machine-

    $From
        |>| ("exitParam") -> ("enterParam") $To("stateParam") ^
        |<| [exitParam:string] ^

    $To [stateParam:string]
    	|>| [enterParam:string] ^

    ##

.. code-block::

    //===================== Machine Block ===================//

    func (m *transitionExampleWithDataPassingStruct) _TransitionExampleWithDataPassingState_From_(e *framelang.FrameEvent) {
        switch e.Msg {
        case ">":
            m._compartment_.ExitArgs["exitParam"] = "exitParam"
            compartment := NewTransitionExampleWithDataPassingCompartment(TransitionExampleWithDataPassingState_To)
            compartment.EnterArgs["enterParam"] = "enterParam"
            compartment.StateArgs["stateParam"] = "stateParam"

            m._transition_(compartment)
            return
        case "<":
            return
        }
    }

Above we can see the following steps happen with regards to data passing:

#. Set the exit parameter on the *current* state compartment
#. Create the next state compartment and initialize the state variable
#. Initialize the next state's enter parameters
#. Initialize the next state's state parameters
#. Do (deferred) transition and return

So each transition is simply proceeded by code that creates and provisions the compartments
as appropriate.

Conclusion
----------

This section explained the mechanisms of compartments for data
passing between states. We will next explore their role in facilitating a number of
advanced or nuanced scenarios.

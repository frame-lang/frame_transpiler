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
as well as a factory function for creating new ones.

So for this system spec:

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


Compartments are allocated and intialized in event handlers as the first stage
of a transition. Lets explore the controller code for a basic transition:

``Frame``

.. code-block::

    #TransitionCompartment

    -machine-

    $From
        |>| -> $To ^

    $To

    ##

This trivial spec generates the following code related to the transition:

The spec generates the controller class/struct that contains two runtime
data members related to compartments:

.. code-block::

    type transitionCompartmentStruct struct {
        _compartment_ *TransitionCompartmentCompartment
        _nextCompartment_ *TransitionCompartmentCompartment
    }

``_compartment_`` variable always holds a reference to the current compartment while
the ``_nextCompartment_`` sometimes holds a reference to the next compartment
when the transition is also :ref:`forwarding an event <event_forwarding>`.

This trivial spec generates the following code related to the transition:

.. code-block::

    func (m *transitionCompartmentStruct) _TransitionCompartmentState_From_(e *framelang.FrameEvent) {
        switch e.Msg {
        case ">":
            compartment := NewTransitionCompartmentCompartment(TransitionCompartmentState_To)
            m._transition_(compartment)
            return
        }
    }

The ``NewTransitionCompartmentCompartment`` factory simply takes the id of the
state being transitioned to, in this case ``$To``.

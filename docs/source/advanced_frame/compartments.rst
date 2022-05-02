.. _compartments_sec:

Compartments
============


An Introduction to Compartments
-------------------------------

Frame controllers are the generated code from Frame specs. As we have seen,
the controller's current state is tracked in a Frame managed runtime variable:

.. code-block::

    var _state_ = OFF

However, transition parameters now mean there is more data associated with a
state than just the state variable. To accomplish this, Frame introduces
the idea of the **Compartment**. A compartment is, in essence, a *state closure*.

Closures are a concept from programming languages that tie references to anonymous functions
to the environment that existed when they were created. Frame compartments
are a similar concept, but instead of a function carrying its associated environment
with it, compartments enable instances of states to maintain their own environments.

Compartments are simply a data structure consisting of a state variable and
its environment:

.. code-block::

    struct Compartment {
        State _state_;
        EnterArgs map[string]interface{}
        ExitArgs map[string]interface{}
        ...
    }

.. note::

    To focus on just the transition related parameters, the
    data structure above only shows a partial
    inventory of a full Compartment.


In the sections on transition and state parameters, as well as state variables,
no details were given as
to how this data is passed to the state, initialized and/or preserved.
The answer to those questions is a new idea for state machines called
the **compartment**.

Compartments are a `closure <https://en.wikipedia.org/wiki/Closure_(computer_programming)>`
concept for states that preserve the state's context. This context includes
the state identifier, the data from the
various state local scopes event handlers can access (so not including the domain)
as well as the system runtime data needed for the machinery to implement the
Frame language semantics.

A compartment is implemented simply enough:

.. code-block::

    type SomeCompartment struct {
        State SomeState
        StateArgs map[string]interface{}
        StateVars map[string]interface{}
        EnterArgs map[string]interface{}
        ExitArgs map[string]interface{}
        _forwardEvent_ *framelang.FrameEvent
    }

and has the following data members:

* State     - state variable
* StateArgs - map of state arguments
* StateVars - map of state variables
* EnterArgs - map of transition enter arguments
* ExitArgs  - map of transition exit arguments
*  _forwardEvent_ - system runtime data member for Event Forwarding feature

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

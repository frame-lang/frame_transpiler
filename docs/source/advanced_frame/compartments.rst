Compartments
============

In the sections on transition and state parameters, as well as state variables,
no details were given as
to how this data is passed to the state, initialized and/or preserved.
The answer to those questions is a new concept for state machines called
the **compartment**.

Compartments are a closure concept for
states that preserve the state itself, the data from the
various scopes event handlers can access (but not including the domain)
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

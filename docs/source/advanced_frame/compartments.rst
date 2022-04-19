Compartments
============

In the sections on transition and state parameters, as well as state variables,
no details were given as
to how this data is passed to the state, intialized and/or preserved.
The answer to those questions is a new concept for state machines called
the **compartment**.

Compartments, or *state compartments*, are a closure concept for
states that preserve the state itself, the data from the
various scopes as well as runtime data
needed for the Frame machine semantics.

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

The compartment manifest is:

* State     - state identifier
* StateArgs - state arguments
* StateVars - state variables
* EnterArgs - transition enter arguments
* ExitArgs  - transition exit arguments
*  _forwardEvent_ - runtime data member for Event Forwarding feature

.. code-block::

    #CompartmentDemo $[state_param:string] >[enter_param:string] [domain_param:string]

    -machine-

    $StartState [p1:string]
        var v1

    ##

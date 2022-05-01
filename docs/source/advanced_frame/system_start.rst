Starting A System
=================

Frame specifications mask over a number of important complexities
about how to start a system running. This
section will explore the Frame syntax and implementation details related
to "booting up" a Frame system.

The Start State
---------------

State machines conceptually begin operation already in a **start state**.
In Frame this is always the first state listed in the spec.

``Frame``

.. code-block::

    #InTheBeginning

    -machine-

    $StartHere

An important implementation detail is that the start state is different from
all other states as it is the only one
that is not first entered via a state change or transition. This is important
as transitions are the means by which all other states are initialized. The
state initialization steps during a transition are:

#. Initialize the state parameters
#. Initialize the state variables
#. Initialize the enter event parameters
#. Send state the enter event and trigger the enter event handler

For instance:

``Frame``

.. code-block::

    #TransitionInit

        -machine-

        $S1
            |next|
                -> ("Hello")            // <--- Initialize enter event params
                    $S2("state $S2")    // <--- Intialize state params
                    ^

        $S2 [who:string]                // <--- State param
            var separator:string = " "  // <--- Intialize state variable

            |>| [greeting:string]       // <--- Enter event handler and params
                print(who + separator + greeting) ^
    ##

The Frame transition mechanisms support the activities outlined above.
Upon creation, however, the system does not do a transition into the start state.
Instead it simply starts there, so none of the usual transition mechanisms
are used in the very beginning. Therefore
another mechanism must exist to provide these parameters to the start state.
To do so, Frame provides **system initializer lists**.

These lists are optional, but if present must be in the following order:

#. State parameter initializer list - $[<params>]
#. Enter event parameter initializer list - >[<params>]
#. Domain variable override list - [<params>]

The state parameters are initialized with the first two lists:

.. code-block::

    #StartSystem1 $[state_param:string] >[enter_param:string]

    -machine-

    $StartState [state_param:string]
        |>| [enter_param:string] ^

    ##

Above we can see that the state parameters and enter event parameters can be
provided via two lists after the system declaration identifier. The ``StartState``
will now work upon boot just like it will if a state transitions back into it.

Additionally, the system has one additional initializer list to override the
default initialization of the domain variables:

.. code-block::

    #StartSystem2 [domain_param:string]

    -domain-

    var domain_param:string = nil

    ##


The System Factory
------------------

To facilitate proper use of this feature, the Framepiler generates
a convenience factory function to return a properly initialized system.

State Parameter Initialization
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.. code-block::

    #StartSystem3 $[state_param:string] >[enter_param:string] [domain_param:string]

    -machine-

    $StartState [state_param:string]
        |>| [enter_param:string] ^

    -domain-

    var domain_param:string = nil

    ##

This specification generates the following factory code:

``Go``

.. code-block::


    func NewStartSystem3(state_param string,enter_param string,domain_param string) StartSystem3 {
        m := &startSystem3Struct{}

        // Validate interfaces
        var _ StartSystem3 = m

        m._compartment_ = NewStartSystem3Compartment(StartSystem3State_StartState)
        m._compartment_.StateArgs["state_param"] = state_param

        // Initialize domain
        m.domain_param = domain_param

        // Send system start event
        params := make(map[string]interface{})
        params["enter_param"] = enter_param
        e := framelang.FrameEvent{Msg:">", Params:params}
        m._mux_(&e)
        return m
    }

Enter Event Parameter Initialization
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.. code-block::

    #StartSystem3 >[enter_param:string]

    -machine-

    $StartState
        |>| [enter_param:string] ^

    -domain-

    var domain_param:string = nil

    ##


Domain Variable Override Initialization
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.. code-block::

    #StartSystem3 [domain_param:string]

    -domain-

    var domain_param:string = nil

    ##

The steps for proper system initialization are:

#. Create the system and initialize the domain
#. Create the compartment for the first state
#. Set the machine compartment to be the new compartment
#. Initialize the compartment with state parameters
#. Initialize compartment with enter parameters
#. Send an enter event to the multiplexer and pass the compartment enter parameters

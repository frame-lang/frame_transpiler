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
that is not first entered by a transition. This is important
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


State Parameter List
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The system state parameters are initialized with the $[<params>] list:

.. code-block::

    #StartSystem1 $[stateParam:string]

    -machine-

    $StartState [stateParam:string]

    ##

Enter Event Parameter List
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The system enter event parameters are initialized with the >[<params>] list:

.. code-block::

    #StartSystem3 [domainParam:string]

    -machine-

    $StartState
        |>| [enterParam:string] ^

    ##


Domain Variable Override List
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The system domain variables must be initialized by default, but can be overriden
using the domain variable initialization list:


.. code-block::

    #StartSystem3 [domainParam:string]

    -domain-

    var domainParam:string = nil

    ##



These lists are optional, but if present must be in the following order:

#. State parameter initializer list
#. Enter event parameter initializer list
#. Domain variable override list


The System Factory
------------------

To facilitate proper use of these feature, the Framepiler generates
a convenience factory function to return a properly initialized system.

State Parameter Initialization
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block::

    #StartSystem3 $[stateParam:string] >[enterParam:string] [domainParam:string]

    -machine-

    $StartState [stateParam:string]
        |>| [enterParam:string] ^

    -domain-

    var domainParam:string = nil

    ##

This specification generates the following factory code:

``Go``

.. code-block::


    func NewStartSystem3(stateParam string, enterParam string, domainParam string) StartSystem3 {
        m := &startSystem3Struct{}

        // Validate interfaces
        var _ StartSystem3 = m

        m._compartment_ = NewStartSystem3Compartment(StartSystem3State_StartState)
        m._compartment_.StateArgs["stateParam"] = stateParam

        // Initialize domain
        m.domainParam = domainParam

        // Send system start event
        params := make(map[string]interface{})
        params["enterParam"] = enterParam
        e := framelang.FrameEvent{Msg:">", Params:params}
        m._mux_(&e)
        return m
    }


The steps for proper system initialization are:

#. Create the system and initialize the domain
#. Create the compartment for the first state
#. Set the machine compartment to be the new compartment
#. Initialize the compartment with state parameters
#. Initialize compartment with enter parameters
#. Send an enter event to the multiplexer and pass the compartment enter parameters

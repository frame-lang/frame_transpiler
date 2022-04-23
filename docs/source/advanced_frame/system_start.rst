Starting A System
=================

State machines begin operation in a **start state** which is always the
first state listed in the Frame spec.


``Frame``

.. code-block::

    #InTheBeginning

    -machine-

    $Start


The start state is different from all the other states as it is the only one
that is not first entered via a state change or transition. In Frame, transitions
perform three important state initialization activities:

#. Send enter event
#. Set enter event parameters
#. Set state parameters

For instance:

``Frame``

.. code-block::

    #TransitionInit

        -machine-

        $S1
            |next| -> ("Hello") $S2("state $S2")  ^

        $S2 [who:string]
            var separator:string = " "

            |>| [greeting:string]
                print(who + separator + greeting) ^
    ##

Upon creation, the system does not do a transition into the start state, so
another mechanism must exist to provide these parameters to the start state.
To do so, Frame provides system initializer lists:

.. code-block::

    #StartSystem1 $[state_param:string] >[enter_param:string]

    -machine-

    $StartState [state_param:string]
        |>| [enter_param:string] ^

    ##

Above we can see that the state parameters and enter event parameters can be
provided via two lists after the system declaration identifier. The `StartState`
will now work upon boot just like it will if a state transitions back into it.

Additionally, the system has one additional initializer list to override the
default initialization of the domain variables:

.. code-block::

    #StartSystem2 [domain_param:string]

    -domain-

    var domain_param:string = nil

    ##

These lists are optional, but if present must be in the following order:

#. State parameter initializer list - $[<params>]
#. Enter event parameter initializer list - >[<params>]
#. Domain variable override list - [<params>]

The System Factory
------------------

To facilitate proper use of this feature, the Framepiler generates
a convenience factory function to return a properly initialized system

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
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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
#. Send an enter event to the mux and pass the compartment enter parameters

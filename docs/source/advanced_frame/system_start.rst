Starting A System
=================

State machines begin operation in a **start state** which is always the
first state listed in the Frame spec.

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

#. State parameter initializer list
#. Enter event parameter initializer list
#. Domain variable override list

.. _transition_parameters::

Transition Parameters
=====================

Transition parameters allow system designers to specify that data that should
be sent to enter ``|>|`` and exit ``|<|`` event handlers during a transition.

This capability greatly simplifies data passing between states and parameterizing
exit event behavior.

.. _exit_event_parameters::

Enter Event Parameters
----------------------

Frame provides notation to directly pass arguments to the new state as part of
a transition:

.. code-block::

    -> (<enter_argument_list>) $NewState

The state being transitioned to receives the arguments on the ``|>|`` event:

.. code-block::

    $NewState
        |>| [<enter_argument_list>] ...

For instance:

``Frame``

.. code-block::

    #EnterEventParameters

        -interface-

        start @(|>>|)

        -machine-

        $Begin
            |>>| -> ("Hello $State") $Print ^ // <--- "Hello State" sent to $Print

        $Print
            |>| [greeting:string]  // <--- greeting parameter is "Hello State"
                print(greeting) ^  // <--- greeting printed

        -actions-

        print[message:string]

    ##

Exit Event Parameters
---------------------

Though not as common an operation as sending data forward to the next state,
Frame also enables sending data to the exit event hander of the current state as well:

.. code-block::

    (<exit_argument_list>) -> $NewState

For instance:

.. code-block::

    ("cya") -> $NextState

as in

``Frame``

.. code-block::

    $OuttaHere
        |<| [exitMsg:string]
            print(exitMsg) ^

        |gottaGo|
            ("cya") -> $NextState ^

State Parameters
----------------

In addition to parameterizing the transition operator, Frame enables passing
arguments to states themselves. State arguments are passed in an expression
list after the target state identifier:

.. code-block::

    -> $NextState(<state_args>)

State parameters are declared as a parameter list for the state:

``Frame``

.. code-block::

    #StateParameters

        -interface-

        stop

        -machine-

        $Begin
            |>| -> $State("Hi! I am $State :)")  ^

        $State [stateNameTag:string]
            |>|  print(stateNameTag) ^
            |<|  print(stateNameTag) ^
            |stop|
                 print(stateNameTag)
                 -> $End ^

        $End

        -actions-

        printAll[message:string]

        -domain-

        var systemName = "#Variables"
    ##

Above we see that the stateNameTag is accessible in the enter, exit and
stop event handlers. It will also be in scope for all other event handlers for
the state as well.

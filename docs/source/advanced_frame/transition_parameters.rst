.. _transition_parameters::

Much of Frame's innovation is linked to enhancing the concept of what a state
is and can do. Statecharts introduced key functional advancements by
introducing the idea of enter and exit events.

``Frame``

.. code-block::

    -machine-

    $S1
        |someEvent| -> $S2 ^ // <--- 1) transition is triggered
        |<| exitS1() ^       // <--- 2) exit event happens

    $S2
        |>| enterS2() ^      // <--- 3) enter event happens

We have already seen the
Frame mechanism for implementing these features during a transition
- the transition method:

``C#``

.. code-block::

    private void _transition_(FrameState newState) {
        _state_(new FrameEvent("<",null));  // <--- send Exit Event
        _state_ = newState;                 // <--- change of state
        _state_(new FrameEvent(">",null));  // <--- send Enter Event
    }

While extremely useful for intializing and cleaning up the states, there is
a challenge with passing data to the various system components during a transition.

For instance what if we wanted to send data from ``$S1`` to ``$S2`` during
the transition? One approach is simply to save a value in a special domain
variable:

.. code-block::

    -machine-

    $S1
        |e1|
            meaning_of_life = 42 // <--- 1) save a value in the domain
            -> $S2 ^             // <--- 2) transition
        |<| exitS1() ^           // <--- 3) exit event handler


    $S2
        |>|
            enterS2(meaning_of_life) // <--- 4) use cached data
            ^

    -domain-

    var meaning_of_life:int = 0

This works, but is cumbersome and requires special data members whose only
purpose is to transfer data between states.
We will now explore how Frame innovates to allow a more sophisticated data
flow between states during transitions.

Transition Parameters
=====================

Transition parameters allow system designers to specify that data that should
be sent to enter ``|>|`` and exit ``|<|`` event handlers during a transition.

This capability greatly simplifies data passing between states and parameterizing
exit event behavior.

.. _enter_event_parameters::

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

        -machine-

        $Begin
            |>| -> ("Hello $State") $Print ^ // <--- "Hello State" sent to $Print

        $Print
            |>| [greeting:string]  // <--- greeting parameter is "Hello State"
                print(greeting) ^  // <--- greeting printed

        -actions-

        print[message:string]

    ##

The ability to pass data directly to the new state via the transition is
a significant improvement over allocating single use domain variables.

Let us now see if there is something to do on the other side of the transition.

Exit Event Parameters
---------------------

Though not as common an operation as sending data forward to the next state,
Frame also enables sending data to the exit event hander of the current state:

.. code-block::

    (<exit_argument_list>) -> $NewState

For instance:

.. code-block::

    ("cya") -> $NextState

In context,

``Frame``

.. code-block::

    $OuttaHere
        |<| [exitMsg:string]        // <--- exit event parameters
            print(exitMsg) ^

        |gottaGo|
            ("cya") -> $NextState ^ // <--- initialize exit event parameters

This ability can be useful when distinguishing different exit contexts:


.. code-block::

    $OuttaHere
        |<| [exitMsg:string]          // <--- "walk" or "run" depending on...
            print(exitMsg) ^

        |yellow_alert|
            ("walk") -> $NextState ^  // <--- "show "walk" message

        |red_alert|
            ("run!!") -> $NextState ^ // <--- show "run" message

The enter and exit events provide a pleasing symmetry to the data flows
involving transitions. And while the syntax is simple to specify them,
how they are implemented is significantly more involved.

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
        Map stateArgs;
        Map stateArgs;
        StateVars map[string]interface{}
        EnterArgs map[string]interface{}
        ExitArgs map[string]interface{}
        _forwardEvent_ *framelang.FrameEvent
    }

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
            |>| -> $State("Hi! My name is $State :)")  ^

        $State [stateNameTag:string]
            |>|  print(stateNameTag) ^
            |<|  print(stateNameTag) ^
            |stop|
                 print(stateNameTag)
                 -> $End ^

        $End

        -actions-

        print[message:string]

    ##

Above we see that the stateNameTag is accessible in the enter, exit and
stop event handlers. It will also be in scope for all other event handlers for
the state as well.

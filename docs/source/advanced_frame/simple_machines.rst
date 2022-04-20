Simple Machines
===============

Before explaining the details of the Frame architecture for state machine
controllers, let us take a look at some simple alternatives to better
understand why certain choices were made.

However let us start at an even more basic level and answer a very basic
question - what is a state?

The Essence Of a State
----------------------

States are, at their core, a function. This function takes, at its most
basic incarnation, two inputs and outputs a "behavior".

.. code-block::

    (Event,State) -> Behavior

Alternatively the function signature could be this:

.. code-block::

    (State, Event) -> Behavior


From a mathematical perspective, this reordering of inputs makes no difference.
However, from a code organization standpoint this difference results in
*extremely* different code structure. And that turns out to be very important.

Let us now take a look an example of each to see why.

Event Oriented State Machines
-----------------------------

An important point about why state machines are interesting is that
all code really is part of a state machine, whether it looks like it or not.

Below we have pseuedocode for a state machine for a very simple lamp that simply
 looks like a normal object oriented class:

.. code-block::

    class VerySimpleLamp {

        enum {OFF,ON} state;

        function turnOn() {

            if state == OFF {
                state = ON;
                closeSwitch();
            } else if state == ON {
                // nop
            }
        }

        function turnOff() {

            if state == OFF {
                // nop
            } else if state == ON {
                openSwitch();
                state = OFF;
            }
        }
    }

The class is simple, containing only a state variable, two states (OFF and ON),
and two event
handlers `turnOn()` and `turnOff()`. In each handler this class simply
tests to see if the machine is in a state that responds to that event. So,
in a sense, the machine is "event-oriented" in that it first considers what
the event is (an event handler is called) and then considers what state it is
in (boolean test for state performed). It then triggers some behavior if
the (event x state) -> behavior.


.. table:: Event-Oriented State Machine Table
    :widths: auto

    +-------------+-----------------+--------------+
    |Event\\State |   OFF           |   ON         |
    +=============+=================+==============+
    ||TURN_ON     | |state = ON     ||             |
    ||            | |               ||closeSwitch()|
    +-------------+-----------------+--------------+
    |TURN_OFF     |                 | openSwitch() |
    |             |                 | state = OFF  |
    +-------------+-----------------+--------------+


.. table:: Event-Oriented State Machine Table
    :widths: auto

    =============  ===============  ===============
    Event\\State    OFF              ON
    =============  ===============  ===============
    TURN_ON        state = ON       \n
                                    closeSwitch()
    TURN_OFF                        openSwitch()\n
                                    state = OFF
    =============  ===============  ===============

The

.. code-block::

    class SimpleLamp {

        enum {OFF,ON} state;

        function handleEvent(e Event) {
            switch state {
                case OFF:
                    if e.msg == "TURN_ON" {
                        state = ON;
                        closeSwitch();
                        return;
                    }
                    break;
                case ON:
                    if e.msg == "TURN_OFF" {
                        openSwitch();
                        state = OFF;
                        return;
                    }
                    break;
            }
        }
    }

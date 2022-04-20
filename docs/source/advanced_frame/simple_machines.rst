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

    +-------------+-----------------+---------------+
    |Event\\State |   OFF           |   ON          |
    +=============+=================+===============+
    || TURN_ON    || state = ON     ||              |
    ||            ||                || closeSwitch()|
    +-------------+-----------------+---------------+
    || TURN_OFF   |                 || openSwitch() |
    ||            |                 || state = OFF  |
    +-------------+-----------------+---------------+

Notice how the `closeSwitch()` and `openSwitch()` calls happen in the context
of the state machine being in the `ON` state. It is a subtle point, which
is actually the point. The context activity occurs should be more obvious.

The example below improves the situation, but is still not completely
well structured from a logical standpoint.

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
                    } else {
                        // nop
                    }
                    break;
                case ON:
                    if e.msg == "TURN_OFF" {
                        openSwitch();
                        state = OFF;
                        return;
                    } else {
                        // nop
                    }
                    break;
            }
        }
    }

This version of a Lamp state machine has one major improvement - it is now
*state oriented* in that the state is considered first (in the switch)
and then the event is inspected. What this accomplishes is that now **all
code related to a logical state is in one physical location in the code**.

In the event-oriented state machine, the developer would have to look in both
event handlers to see the code related to a given state. This is called
**logical state fragmentation** and is one of the worst flaws of event-oriented
state machines.

However, this approach is still not semantically perfect.

.. code-block::

    case OFF:
        if e.msg == "TURN_ON" {
            state = ON;
            closeSwitch();
            return;
        } else {
            // nop
        }
        break;

The code above still has one subtle, logical problem. It shows the code
related to the `OFF` state, therefore it is reasonable so assume that all
code there is actually exectuted in the context of being `OFF`. However,
that is not the case. The problem happens on these lines:

.. code-block::

    state = ON;
    closeSwitch();

Here, inside of `OFF`, the machine changes state **and then proceeds do
do an action**. The problem is that `closeSwitch()` happens in the actual
context of being in the `ON` state - you can see that we just changed state in
the line above. However all this code is inside the `case OFF` block, which
it is reasonable to assume only contains code related to being OFF. But as we
have just shown, the `closeSwitch()` call is decidedly called when being `ON`.

The result is that we have an entanglement of the two states in the same block.
This
entanglement makes it subtle and potentially confusing what exactly is
happening. This subtlety is why this approach to implementing state machines
is flawed as it is very easy to lose track as to what is happening in which
state. 

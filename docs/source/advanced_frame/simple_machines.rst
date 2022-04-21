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
    || turnOn     || state = ON     ||              |
    ||            ||                || closeSwitch()|
    +-------------+-----------------+---------------+
    || turnOff    |                 || openSwitch() |
    ||            |                 || state = OFF  |
    +-------------+-----------------+---------------+

Notice how the `closeSwitch()` and `openSwitch()` calls happen in the context
of the state machine being in the `ON` state. It is a subtle point, which
is actually the point. The context activity occurs should be more obvious.

State Oriented Machines
-----------------------

To address some of the deficiencies of the event-oriented machine architecture
we will try to restructure the state machine to pull together all the code related
to a single logical state in one place.
The example below improves the situation, but is still not completely
well structured from the perspective of compartmentalizing logical state.

.. code-block::

    class SimpleLamp {

        enum {OFF,ON} state;

        function handleEvent(e Event) {
            switch state {
                case OFF:
                    if e.msg == "turnOn" {
                        state = ON;
                        closeSwitch();
                        return;
                    } else {
                        // nop
                    }
                    break;
                case ON:
                    if e.msg == "turnOff" {
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
and then the event is inspected. The goal with that reorganization is
to get the code related to a logical state is in one physical location
in the file. And it *looks* like we have but, in fact, that is *not* the case.

Let's take a closer look at the code block for the `Off` state:

.. code-block::

    case OFF:
        if e.msg == "turnOn" {
            state = ON;
            closeSwitch();
            return;
        } else {
            // nop
        }
        break;

The code above still has one subtle, logical problem. The problem happens
on these lines:

.. code-block::

    state = ON;
    closeSwitch();

Here, inside of `OFF`, the machine changes state to `ON` **and then proceeds do
do an action**. Therefore `closeSwitch()` is being executed in the
context of `ON` state despite both of those lines being inside the
`case OFF` block. Essentially a sliver of
 `ON` state functionality is inside of a
code block that is supposedly code related to being `OFF`.

The result is an **entanglement** of the two states.  This
entanglement is a subtle, and potentially very confusing, overlap of logical
states. And it certainly isn't very tidy.

State Function Machine Architecture
-----------------------------------

Statecharts introduced the concept of enter and exit events, which were
explored earlier. These system generated (as opposed to coming from an
external client) events are supremely valuable as mechanisms to initialize and
cleanup states. How are these ideas represented in the state machine
implementations above. The answer to that question precisely intersects
 the entanglement problem that was just discussed.

 The Enter Event and State Structure in Frame
---------------------------------------------

Let us take another look at the last, entangled state example:

.. code-block::

    case OFF:
        if e.msg == "turnOn" {
            state = ON;    // <---- Change of state
            closeSwitch(); // <---- ON State enter behavior
            return;
        } else {
            // nop
        }
        break;

The comments identify what is actually happening in the entangled portion
of the state machine. The code is changing state and then **executing the
enter state behavior**. Although this is a perfectly viable way to construct state machines,
but can be confusing for the reasons discussed above as well as not being
as powerful as will be discussed in the advanced Frame features later.

The Frame approach to solving this problem is to use state functions to
hold all state event handlers and behavior and to introduce a `_transition_()`
method to do the change of state mechanics:

.. code-block::

    private void _sOff_(FrameEvent e) {
        if (e._message.Equals("turnOn")) {
            _transition_(_sOn_);
            return;
        }
        ...
    }

    private void _sOn_(FrameEvent e) {
        if (e._message.Equals(">")) {
            closeSwitch_do();
            return;
        }
        ...
    }

    private void _transition_(FrameState newState) {
        FrameEvent exitEvent = new FrameEvent("<",null);
        _state_(exitEvent);  // <--- send Exit Event

        _state_ = newState;  // <--- change state

        FrameEvent enterEvent = new FrameEvent(">",null);
        _state_(enterEvent); // <--- send Enter Event
    }

As we can see above, the `OFF` state uses the `_transition_()` to perform three
key operations necessary for basic Statechart functionality:

#. Send the Exit Event to the current state
#. Change the current state to the new state
#. Send the Enter Event to the (new) current state

The Frame spec that would generate the code above is very simple:

``Frame``

.. code-block::

    $Off
        |turnOn| -> $On ^
    $On
        |>| closeSwitch() ^
        |<| openSwitch() ^

What we can see this approach also accomplishes is consolidating all behavior related
to the `ON` state in the `ON` state function. The logical behavior of the
state machine is now properly compartmentalized in the correct state function.

It is arguable that the state function approach necessitates more code to
accomplish the goal of complete disentanglement, which may be considered
bad form. The perspective of the author is that the complete compartmentalization
of code related to logical states is tremendously simpler from an organizational
perspective and the benefits vastly outweigh any other concerns. This approach
 also provides the infrastructure to build far more sophisticated
mechanisms for state machine architectures than would be reasonably possible
without this approach.

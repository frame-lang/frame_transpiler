Machine Evolution
=================

The Frame language evolved from years of studying the literature and
experimenting with writing
state machines by hand in many different programming languages.
Although the basic coding pattern the Framepiler generates today quickly
emerged as the preferred implementation, it took much longer to understand why
it was instinctually preferred.

It just so happened that this approach lent itself to both a pleasing (to
the author in any case) domain specific language for expressing the pattern
as well as unlocking new concepts that could make the generated state machines
more powerful and easier to work with.

This section will explore that evolutionary path with the goal of making
the choices for Frame's syntax and implementation clearer.

We will begin with a very basic question about the atomic unit of a state
machine - what is a state?

The Essence Of a State
----------------------

States are, at their core, form of mathematical function.
And in its simplest
incarnation this function takes two inputs and outputs a "behavior".

.. code-block::

    (Event,State) -> Behavior

Alternatively the function signature could be this:

.. code-block::

    (State, Event) -> Behavior


From a mathematical perspective, this reordering of inputs makes no difference.
However, from a code organization standpoint this difference results in
*extremely* different code structure. And that turns out to be very important.

Let us now take a look an example of each to see why.

Event Oriented Machines
-----------------------------

An important point about why state machines are interesting is that
all code really is part of a state machine, whether it looks like it or not.

Below we have pseuedocode for a state machine for a very simple lamp that simply
 looks like a normal object oriented class:

.. code-block::

    class VerySimpleLamp {

        enum {OFF,ON} state;

        function turnOn() {

            if state == OFF {       // <--- "OFF" state
                state = ON;
                closeSwitch();
            } else if state == ON { // <--- "ON" state
                print("Already on");
            }
        }

        function turnOff() {

            if state == OFF {       // <--- more "OFF" state
                print("Already off");
            } else if state == ON { // <--- more "ON" state
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

Here is a table that illustrates the behavior mapping function:

.. table:: Event-Oriented State Machine Table
    :widths: auto

    +-------------+-----------------------+----------------------+
    |Event\\State |   OFF                 |   ON                 |
    +=============+=======================+======================+
    || turnOn     || state = ON           || print("Already On") |
    ||            || closeSwitch()        ||                     |
    +-------------+-----------------------+----------------------+
    || turnOff    || print("Already Off") || openSwitch()        |
    ||            |                       || state = OFF         |
    +-------------+-----------------------+----------------------+

The problem with this approach is that **each event handler has a block
of code for each state**. By organizing the class by event we necessarily
must sort out the small chunks of each state that are related to the event.
This organization for a state machine results in **state fragmentation**.

People mentally organize the world around logical context - logical state.
State fragmentation makes it much harder to understand what is happening in
any give logical context because the logical context is exploded throughout
the software, as we have just seen. It is, in fact, a bizarre way to structure
software. However, it is also the practically universal way of doing it.

Let us now take a look at the start of an alternative.

State Oriented Machines
-----------------------

To address some of the deficiencies of the event-oriented machine architecture
we will try to restructure the state machine to pull together all the code related
to a single logical state in one place.
The example below improves the situation, but is still not completely
water-tight from the perspective of compartmentalizing logical state.

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
                    } else if e.msg == "turnOff" {
                        print("Already off");
                    }
                    break;
                case ON:
                    if e.msg == "turnOff" {
                        openSwitch();
                        state = OFF;
                        return;
                    } else if e.msg == "turnOn" {
                        print("Already on");
                    }
                    break;
            }
        }
    }

This version of a Lamp state machine has one major improvement - it is now
*state oriented* in that the state is considered first (in the switch)
and then the event is inspected. The goal with that reorganization is
to get the code related to a logical state is in one physical location
in the file. And it *looks* like we have. Unfortunately, it's not true.

Let's take a closer look at the code block for the `OFF` state:

.. code-block::

    case OFF: // <--- code block for "OFF" state
        if e.msg == "turnOn" {
            state = ON;    // <---- change of state
            closeSwitch(); // <---- enter behavior for "ON" state
            return;
        } else if e.msg == "turnOff" {
            print("Already off");
        }
        break;

The code above is better still has one subtle, logical problem. The problem happens
on these lines:

.. code-block::

    state = ON;    // <---- change of state.
    // ----------------------------------//
    // This code is run in the ON state!!
    closeSwitch(); // <---- enter behavior for "ON" state

Here, inside of the `OFF` state code block, the machine changes state to
`ON` **and then proceeds do
do an action**. Therefore `closeSwitch()` is being executed **in the
context of `ON` state** despite both of those lines being inside the
`case OFF` block. Essentially a sliver of
 `ON` state functionality is subtly embedded in a
code block that is supposedly code related to being `OFF`.

The result is an **entanglement** of the two states.  State entanglement is a
subtle, and potentially very confusing, overlap of logical
states. And it certainly isn't very tidy.

Let's see how this can be addressed.

State Function Machine Architecture
-----------------------------------

Statecharts introduced the concept of enter and exit events, which were
explored earlier. These system generated (as opposed to coming from an
external client) events are supremely valuable as mechanisms to initialize and
cleanup states. How are these ideas represented in the state machine
implementations above. The answer to that question precisely intersects
 the entanglement problem that was just discussed.

 The Enter Event and State Mechanism in Frame
---------------------------------------------

Let us take another look at the last, entangled state example:

.. code-block::

    case OFF: // <--- code block for "OFF" state
        if e.msg == "turnOn" {
            state = ON;    // <---- change of state
            closeSwitch(); // <---- enter behavior for "ON" state
            return;
        } else if e.msg == "turnOff" {
            print("Already off");
        }
        break;

The comments identify what is actually happening in the entangled portion
of the machine. The code is changing state and then **executing the
enter state behavior**. This is a perfectly viable way to construct state machines,
but suffers from two problems. First, it can be very confusing. But second,
it is not as powerful or flexible as it could be.

The Frame approach to solving this problem is to use **state functions** to
hold all state event handlers and behavior and to introduce a `_transition_()`
method to do the mechanics of changing the state. Here is snippet of a Frame spec
for the lamp:

``Frame``

.. code-block::

    $Off
        |turnOn| -> $On ^
        |turnOff| print ("Already off") ^
    $On
        |>| closeSwitch() ^

And the generated code:

``C#``

.. code-block::

    private void Off(FrameEvent e) {
        if (e._message.Equals("turnOn")) {
            _transition_(On);
            return;
        } else if (e._message.Equals("turnOff")) {
            print("Already off");
            return;
        }
        ...
    }

    private void On(FrameEvent e) {
        if (e._message.Equals(">")) {
            closeSwitch();
            return;
        }
        ...
    }

    private void _transition_(FrameState newState) {
        _state_(new FrameEvent("<",null));  // <--- send Exit Event
        _state_ = newState;                 // <--- change of state
        _state_(new FrameEvent(">",null));  // <--- send Enter Event
    }

As we can see above, the `OFF` state uses `_transition_()` to perform three
key operations necessary for basic Statechart enter/exit functionality:

#. Send the Exit Event to the current state
#. Change the current state to the new state
#. Send the Enter Event to the (new) current state

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

Conclusion
----------

This article was a quick overview of common approaches to implementing
state machines. These examples showed the functional and logical gaps that motivate
the more complex, but more powerful, state function architecture employed by
Frame.

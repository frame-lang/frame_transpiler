=============
Machine Block
=============

The Machine Block is the heart of the system and defines the system's state
machine. This section will introduce the core features of the
machine notation.

State Machines
--------------

State machines, or more accurately, Turing Machines, were invented by Alan
Turing in his `seminal 1936 paper <https://plato.stanford.edu/entries/turing-machine/>`_.

Although the paper is a complex mathematical proof, the logical device
he used to solve it is actually quite simple to understand. In fact, state machines
structure software much more intelligibly than the popular programming languages.

More on that later, but, in essence, state machines directly expose the
logical problem being solved rather than obscuring it. A simple example should
make this assertion clearer.

States
------

Let us start by exploring a defective lamp as our most basic state machine.

.. code-block::

    #BrokenLamp

    -machine-

    $Off

    ##

Although rather useless to read by, the #BrokenLamp does illuminate an important
point - a state machine can have just one state. State identifiers in Frame are
indicated by a `$` prefix.

To make States do something, they need to be sent events. States handle events
with... event handlers.

.. code-block::

    #BrokenLamp

    -machine-

    $Off
        |turnOn|
            print("I'm broken.") ^

    ##

Event handlers start with a *message selector* (|msg|) and end with either a
*return* (**^** token) or *continue* (**>**) token.

Here we see that the $Off state handles the |turnOn| event by calling the
print function and then returning. In general, states can be described as
mapping events to **behavior**. Behavior comes in two big categories -
**taking action** and **transitioning**.

Taking Action
-------------

Taking action means executing general imperative behaviors
like calling external code, sending messages, changing data etc. In the 
broadest sense, "doing something" that doesn't include transitioning to a new
state. In this example, "doing something" is printing a message.

Next we will explore the other category of behavior - transitioning to a new
state.

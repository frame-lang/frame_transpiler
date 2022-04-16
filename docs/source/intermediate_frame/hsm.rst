===========================
Hierarchical State Machines
===========================

In the Getting Started introduction to Frame we developed a simple spec with
just two states for controlling a Lamp. However, with even this simple system
we start to see issues with redundancy in it's structure.

Classic state machines can suffer from significant duplication of behavior
between states. As is always this case, factoring code into shared components
is a best practice so as to only modify one place rather than multiple
when changes occur. Additionally, with state machines, this redundancy
is especially impactful to the visual modeling of them as the number of lines
in the diagram can go up dramatically for shared transitions.

To address this situation, Hierarchical State Machines (HSMs) were invented
by Dr David Harel in his
1987 paper on Statecharts. Statechart notation is used in both UML software
modeling and the SYSML systems modeling languages as the standard flavor of
state machine visual languages.

Reviewing our #Lamp spec, we can see that two states share a significant
amount of identical functionality with the |getColor| and |setColor| handlers:

.. code-block::

    #Lamp

    -interface-

    turnOn
    turnOff
    setColor [color:string]
    getColor : string

    -machine-

    $Off
        |turnOn|
            -> $On ^
        |getColor| : string
            ^(color)
        |setColor| [color:string]
            #.color = color ^

    $On
        |>|
            turnOnLamp() ^
        |<|
            turnOffLamp() ^
        |turnOff|
            -> $Off ^
        |getColor| : string
            ^(color)
        |setColor| [color:string]
            #.color = color ^

    -actions-

    turnOnLamp
    turnOffLamp

    -domain-

    var color:string = "white"

    ##

To remove the redundancy, HSMs factor out shared functionality into parent
states which is then inherited by child states.
For our #Lamp we will create a new `$Base` state and move those handlers into
it.

.. code-block:: language

    #Lamp

    ...

    -machine-

    $Off => $ColorBehavior
        |turnOn|
            -> $On ^

    $On => $ColorBehavior
        |>|
            turnOnLamp() ^
        |<|
            turnOffLamp() ^
        |turnOff|
            -> $Off ^

    $ColorBehavior
        |getColor| : string
            ^(color)
        |setColor| [color:string]
            #.color = color ^

Here we can see the `$Off` and `$On` states now inherit their behavior from
`$ColorBehavior` state using the `=>` dispatch operator.

Easily supporting HSM semantics is one of the major reasons for the use of
FrameEvents in the architecture as it enables a very simple way to inherit
behavior between states using call chains:

.. code-block::

    //===================== Machine Block ===================//

    private void _sOff_(FrameEvent e) {
        if (e._message.Equals("turnOn")) {
            _transition_(_sOn_);
            return;
        }
        _sColorBehavior_(e);
    }

    private void _sOn_(FrameEvent e) {
        if (e._message.Equals(">")) {
            turnOnLamp_do();
            return;
        }
        else if (e._message.Equals("<")) {
            turnOffLamp_do();
            return;
        }
        else if (e._message.Equals("turnOff")) {
            _transition_(_sOff_);
            return;
        }
        _sColorBehavior_(e);
    }

    private void _sColorBehavior_(FrameEvent e) {
        if (e._message.Equals("getColor")) {
            e._return = this.color;
            return;

        }
        else if (e._message.Equals("setColor")) {
            this.color = ((string) e._parameters["color"]);
            return;
        }
    }

Above we can see that the `$ColorBehavior` state now contains the get/setColor
event handlers and the `$Off` and `$On` states forward the FrameEvent
to it for any functionality they do not handle.

Here is the full HSM implementation of our #Lamp:

.. code-block::

    #Lamp

    -interface-

    turnOn
    turnOff
    setColor [color:string]
    getColor : string

    -machine-

    $Off => $ColorBehavior
        |turnOn|
            -> $On ^

    $On => $ColorBehavior
        |>|
            turnOnLamp() ^
        |<|
            turnOffLamp() ^
        |turnOff|
            -> $Off ^

    $ColorBehavior
        |getColor| : string
            ^(color)
        |setColor| [color:string]
            #.color = color ^

    -actions-

    turnOnLamp
    turnOffLamp

    -domain-

    var color:string = "white"

    ##

Statecharts are the current gold standard for state machine modeling. We will
next explore another powerful Statechart innovation - the history mechanism.

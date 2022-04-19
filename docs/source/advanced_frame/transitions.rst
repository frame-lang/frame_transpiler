State Changes and Transitions
=============================

Frame, at its most advanced, introduces a significant amount of complex "machinery" to
implement the sophisticated capabilities of the language.

The 

Changes of state in Frame come in two types - state changes and transitions.
State changes are trivial - the only activity is to update the state
variable to the new state. Transitions, however, become increasingly more
sophisticated depending on the language features used in a spec.

State Changes
-------------

Sometimes it is desirable to change states without triggering the full
enter/exit machinery involved with a transition. The state change operator
enables this capability:

->> $NewState

A simple filter system is a good example of when changing state is more appropriate than a full transition. Here we can see that the #Filter system simply oscillates between the $Off and $On states. It’s only behaivor is enabling and disabling transmission of objects through it so no state resource management is necessary:
.. code-block:: language

    <code>


Transitions are about three activites:

#.
Simple state machine implememntations
Frame

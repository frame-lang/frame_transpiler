State Changes and Transitions
=============================

Frame, at its most advanced, introduces a significant amount of complex "machinery" to
implement the sophisticated capabilities of the language.

The 

Changes of state in Frame come in two types - state changes and transitions.
State changes are trivial - the only activity is to update the state
variable to the new state. Transitions, however, become increasingly more
sophisticated depending on the language features used in a spec.

State Changes (Deprecated)
--------------------------

.. note::
   The state change operator (`->>`) has been deprecated in Frame V4.
   All state changes should now use the standard transition operator (`->`)
   which includes proper enter/exit lifecycle handling.

In older Frame versions, it was possible to change states without triggering the full
enter/exit machinery using the state change operator. This capability has been
removed to ensure consistent state lifecycle behavior.
.. code-block:: language

    <code>


Transitions are about three activites:

#.
Simple state machine implememntations
Frame

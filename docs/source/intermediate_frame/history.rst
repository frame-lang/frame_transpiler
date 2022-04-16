=============
State History
=============

Classical state machines (Turing machines for sticklers for accuracy)
permit, but do not define, any specific mechanisms for remembering historical
state transitions. This special case is common in use cases
where multiple states can transition to a particular state and then want
to return to the prior state.

An example of this would be a state that
manages a dialog that is useful in many different situations. Once it has
been dismissed the user wants to go back to whatever the prior context was.

To address this kind of scenario Statecharts introduce the “history” mechanism.


History 101
-----------

.. code-block::

    #History101

      -machine-

        $A
            |gotoC| -> $C ^

        $B
            |gotoC| -> $C ^

        $C
            |return| ^

    ##

.. image:: ../images/intermediate_frame/history101.png

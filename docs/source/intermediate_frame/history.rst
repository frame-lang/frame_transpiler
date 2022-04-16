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

The following spec illustrates the limitation of state machines with regards
to history. Below we see states `$A` and `$B` both transitioning into, and
dead ending in, state `$C`.

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

Here we see that $C has no way to know what state preceded it. To solve this
problem for a pure state machine we would have to do something like this:

.. code-block::

    #History102

      -machine-

        $A
            |gotoC| -> $Ca ^

        $B
            |gotoC| -> $Cb ^

        $Ca
            |return| -> $A ^

        $Cb
            |return| -> $B ^

    ##


.. image:: ../images/intermediate_frame/history102.png

$Ca and $Cb would be identical except for the response to the |return| message.
This is obviously inefficient.

The Solution
------------

Automata theory holds that there are are three levels of increasing complexity
and capability for abstract machines:

#. Finite State Machines
#. Pushdown Automata
#. Turing Machines

Pushdown Automata and Turning Machines share the trait of being able to store
information for future use. Pushdown Automata specifically use a stack for
storing history while Turning Machines theoretically have a “tape” to store
information on. In reality if a system can store off data and access it later
to make a decision it is effectively a Turing Machine.

For our problem with remembering the last state, a stack will do nicely thus
giving us the power of a Pushdown Automata. To support this, Frame has two
special operators:

.. list-table:: State Stack Operators
    :widths: 25 25
    :header-rows: 1

    * - Operator
      - Name
    * - $$[+]
      - State Stack Push
    * - $$[-]
      - State Stack Pop

Let’s see how these are used:

.. code-block::

    #History201

      -machine-

        $A
            |gotoC| $$[+] -> "$$[+]" $C ^

        $B
            |gotoC| $$[+] -> "$$[+]" $C ^

        $C
            |return| -> "$$[-]" $$[-] ^

    ##

.. image:: ../images/intermediate_frame/history201.png

What we see above is that the state stack push token precedes a transition to a
new state:

.. code-block::

    $$[+] -> $NewState
===========================
Define a System
===========================

Systems Engineering methodology describes two broad categories of aspects to a system -
**structure** and **behavior**.

Frame is a **Domain Specific Language (DSL)** for for defining system behavior.
In Frame notation a Frame system specification starts with the `#` token and the name of the system
and terminated with the `##` token:

``Frame``

.. code-block::

    #MySystem
    ##

`#MySystem` is an empty system and has no behavior. However, when sent to the Framepiler it still generates code:

``C#``

.. code-block::

    public partial class MySystem {
    }

As we can see, Frame simply software class file. For programming languages that don't have the
concept of a class, Frame generates other targets to implement system
behavior.

The Framepiler currently generates 8 programming languages. Here is the
JavaScript version of the same spec:

``JavaScript``

.. code-block::

    let MySystem = function () {

        let that = {};
        that.constructor = MySystem;

        return that;

    };

### Blocks

Frame specs are organized internally into four *blocks* that are all optional,
as we just saw, but if present must be implemented in a specified order.

.. code-block::

    #MySystem

    -interface-
    -machine-
    -actions-
    -domain-

    ##

We will next investigate each of these blocks, starting with the domain and
working back to the interface.

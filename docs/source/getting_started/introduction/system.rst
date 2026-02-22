===========================
Define a System
===========================

Systems Engineering methodology describes two broad categories of aspects to a system -
**structure** and **behavior**.

Frame is a **Domain Specific Language (DSL)** for defining system behavior.
In Frame notation a Frame system specification uses the ``@@system`` directive
followed by the name of the system in braces:

``Frame``

.. code-block::

    @@system Lamp {
    }

`@@system Lamp` is an empty system spec and has no behavior. However, when sent to the
Framepiler it still generates code:

``Python``

.. code-block:: python

    class Lamp:
        pass

As we can see, Frame simply generates a class. For programming languages
that don't have the
concept of a class, Frame generates other targets to implement system
behavior.

The Framepiler currently generates multiple programming languages including Python,
TypeScript, and Rust.

Blocks
======

Frame specs are organized internally into four *blocks* that are all optional,
as we just saw, but if present must be implemented in a specified order.

.. code-block::

    @@system Lamp {
        interface:

        machine:

        actions:

        domain:
    }

We will next investigate each of these blocks, starting with the domain and
working back to the interface.

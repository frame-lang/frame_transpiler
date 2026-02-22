============
Frame Events
============

Frame Events are essential to the notation and the implementation of Frame system controllers. Frame notation assumes three mandatory fields for a FrameEvent:

#. A message object (String, enumeration, other)
#. A parameters key/value lookup object
#. A return object

Here is a basic implementation of this class:

``Python``

.. code-block:: python

    class FrameEvent:
        def __init__(self, message, parameters=None):
            self.message = message
            self.parameters = parameters or {}
            self._return = None

Frame uses reserved event names for special operations:

.. _system_events:

============== ===========
Event Name     Meaning
============== ===========
$>             Enter state
$<             Exit state
============== ===========


The semantics of the `$>()` and `$<()` events are understood by the Framepiler
and functionally supported. These are the enter and exit event handlers
that are called during state transitions.

============
Frame Events
============

Frame Events are essential to the notation and the implementation of Frame system controllers. Frame notation assumes three mandatory fields for a FrameEvent:

#. A message object (String, enumeration, other)
#. A parameters key/value lookup object
#. A return object

Here is a basic implementation of this class:

`C#`

.. code-block:: csharp

    public class FrameEvent {
        public FrameEvent(String message, Dictionary<String,object> parameters) {
            this._message = message;
            this._parameters = parameters;
        }
        public String _message;
        public Dictionary<String,Object> _parameters;
        public Object _return;
    }

Frame notation uses the `@` symbol to identify a FrameEvent. Each of the three
FrameEvent attributes has its own accessor symbol as well:

.. list-table:: Frame Event Syntax
    :widths: 25 25
    :header-rows: 1

    * - Symbol
      - Meaning/Usage
    * - @
      - frameEvent
    * - @||
      - frameEvent._message
    * - @[]
      - frameEvent._parameters
    * - @[“foo”]
      - frameEvent._parameters[“foo”]
    * - @^
      - frameEvent._return
    * - ^(value)
      - frameEvent._return = value; return;

Frame has two special reserved messages for important operations:

table here:

=====  =====  =======
A      B      A and B
=====  =====  =======
False  False  False
True   False  False
False  True   False
True   True   True
=====  =====  =======

end table

======= ===========  =================
Message Symbol	     Meaning Mandatory
------- -----------  -----------------
>	    Enter state	 Yes
<	    Exit state	 Yes
======= ===========  =================

The semantics of the |>| and |<| events are understood by the Framepiler and functionally supported. The remaining messages are optional may be unused or replaced by other messages with the same semantics if desired.

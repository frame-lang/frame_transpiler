Interface Block
===============

The notation for the interface is very similar to the actions declaration
notation as they both (mainly) define method calls. However, the interface
methods are public while the action methods are private.

The most basic interface method has no parameters and no return values:

.. code-block::

    #Lamp

    -interface-

    turnOn
    turnOff
    ...

To accept parameters, interface methods take parameter lists:

.. code-block::

    #Lamp

    -interface-

    ...
    setColor [color:string]
    ...

And finally to return values to the caller, interface methods specify a
return type:

.. code-block::

    #Lamp

    -interface-

    ...
    getColor : string

    ##

Here is the full interface specification for our lamp:

.. code-block::

    #Lamp

    -interface-

    turnOn
    turnOff
    setColor [color:string]
    getColor : string

    ##

The interface declares four publicly accessible methods.  ``turnOn`` and ``turnOff``
do not take any parameters or return a value. ``setColor`` takes a color string
and ``getColor`` returns a string.


That is all that is needed for our ``#Lamp`` to become a working appliance!

Message Aliases
---------------

Interface methods differ from actions in that their defined behavior is to
send messages into the state machine and the return any value on the
event return object reference. Interface methods, by default, send events
with messages with the same name of the interface method. To change the
message to a custom setting a **message alias** can be used.

.. code-block::

    #MessageAliasDemo

    -interface-

    foo @(|bar|)

    -machine-

    |bar| print("foo called") ^


Next we will investigate the code these four blocks generate to make a
functioning system controller for our Lamp.

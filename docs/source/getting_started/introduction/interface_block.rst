==================
Interface Block
==================

The notation for the interface is very similar to the actions declaration
notation - for the very reason they both define method calls.

The most basic interface method has no parameters and no return values:

.. code-block::

    system Lamp {
        interface:
            turnOn()
            turnOff()
            setColor(color: string)
            getColor(): string
    }

The interface declares four publicly accessible methods.  `turnOn` and `turnOff`
do not take any parameters or return a value. `setColor` takes a color string
and `getColor` returns a string. 


That is all that is needed for our Lamp system to become a working appliance!

Next we will investigate the code these four blocks generate to make a
functioning controller for a lamp.

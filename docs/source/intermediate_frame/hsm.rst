===========================
Hierarchical State Machines
===========================

Hierarchical State Machines (HSMs) were invented by Dr David Harel in his
1987 paper on Statecharts, a visual formalism for software system design.

Classic state machines can suffer from a lot of redundant behavior between
states. This is especially impactful when visually interacting with state
machines as the number of lines

.. code-block::

    #Lamp

    -interface-

    turnOn
    turnOff
    getLocation : string
    -machine-

    $Off
        |turnOn|
            -> $On ^
        |getLampLocation| : string
            ^(getLampLocation())

    $On
        |>|
            turnOnLamp() ^
        |<|
            turnOffLamp() ^
        |turnOff|
            -> $Off ^
        |getLampLocation| : string 
            ^(getLampLocation())
    -actions-

    turnOnLamp
    turnOffLamp
    getLampLocation : string

    -domain-

    var location:string = "bedroom"


    ##

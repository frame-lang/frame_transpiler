Frame v0.11.0 Documentation 
=======================================================
Welcome! Here you will find the official v0.11.0 documentation for the **Frame Language**.

What is Frame?
--------------
Frame is a "metaprogramming" language for both designing and programming state machines (technically Turning Machines). 
Frame is a "metaprogramming" language in the sense that Frame programs are intended to be transpiled to other languages
as well as into documentation. Currently Frame supports Python as its only target language and Statechart visual notation 
for documentation. 

In the future Frame will expand its target language set beyond Python to include JavaScript, Java, C#, C++, Golang and Rust. Other languages 
will follow as the project gains support and adoption. 

The Framepiler
--------------

The Frame transpiler (Framepiler) is an open source CLI tool that turns Frame "specs" into UML or code. 
It is written in Rust and is straightforward to modify to add new target outputs for documentation and source code generation. You can experiment with Frame online `here <https://framepiler.frame-lang.org>`_.


System Design with Frame
------------------------

Frame notation promotes three concepts as first class aspects of the language that don't exist in mainstream programming languages:

#. Systems 
#. States
#. Events


Systems
^^^^^^^

A System in Frame is indicated by an identifier preceeded by the '#' token and terminated by the '##' token:


.. code-block::
    :caption: Empty System

    #Lamp
        // Frame comment follows C style
        // Empty system
    ##


Above we see a Frame *specification* for a lamp "system". Currently this system does absolutely nothing. 

States
^^^^^^^

To improve our lamp, lets start by adding two states - **$Off** and **$On** - to our spec.

.. code-block::
    :caption: Machine Block and States

    #Lamp

        -machine-

        $Off

        $On

    ##

As with "#" for systems, Frame uses a special token "$"  to indicate that an identifier is a state. Frame systems
have optional "blocks" that provide the structure for a system spec. States must live inside the "-machine-" block. 

However these states don't do anything. Let's fix that.

Events
^^^^^^^

Events drive activity in Frame systems. To do so, we must add **event handlers** to our states to provide 
behavior for our lamp.

.. code-block::
    :caption: Event Handlers

    #Lamp

        -machine-

        $Off                    // $Off state
            |turnOn|            // event selector for 'turnOff' event message
                -> $On          // transition to $On state
                ^               // return from event handler

        $On                     // $On state
            |turnOff|           // event selector for 'turnOff' event message
                -> $Off         // transition to $Off state
                ^               // return from event handler

    ##

Let's explore each aspect of the event handlers. 

Event Handlers
~~~~~~~~~~~~~~

Event handlers always begin with an **event selector** for an event message **|msg|** and end with an event handler terminator 
which, in this case, is a return token **^**. 

.. code-block::
    :caption: Event Selector

    |msg|  ^ // Simplest event handler
    

Event handlers contain the behavior of the system. Currently the only behavior our event handlers have
are to **transiton** between the states. Frame transitions use the transition operator '->' which references the
target state the machine will transition to.

.. code-block::
    :caption: Transitions

    -> $TargetState ^

With this level of capability, we have defined a simple Lamp system **state machine** with two states. 
Frame's notation makes it easy to 
understand the purpose and behavior of each state and how they respond to events. 

The Interface Block
~~~~~~~~~~~~~~

Despite having a simple lamp state machine defined, there is currently no way to send an event to the machine
to make it do anything. To enable that capability we add an **-interface-** block containing two public interface methods 
which will generate the events we need to drive the machine activity:

.. code-block::
    :caption: Interface Block and Methods

    #Lamp

        -interface-

        turnOn      // Interface method that sends 'turnOn' event to the machine
        turnOff     // Interface method that sends 'turnOff' event to the machine

        -machine-

        $Off                   
            |turnOn|            
                -> $On  ^              

        $On                      
            |turnOff|           
                -> $Off  ^           

    ##

Identifiers in the `-interface` block generate public methods for the system. So now an external client of the 
system can interact with it and make it do something. 

When `turnOn` and `turnOff` methods are called, Frame generates an event with the same name as the method and sends 
it into the machine which, in turn, will respond if it is in a state that handles that event type. If the 
current state does not handle the event it will simply be ignored. 

Enter and Exit Events
~~~~~~~~~~~~

Even though our system now switches between states, those states don't *really* do anything. For this simple demo we 
will simply log that we have entered and exited our **$Off** and **$On** states. 

To do so we will utilize special events that Frame generates when a system transitions from one state to another. 

.. code-block::
    :caption: State Enter and Exit events

    $Off   
        ...

        |<|  // Exit Event
            print("Exiting $Off") ^

        |turnOn|            
            -> $On  ^              

    $On  
        |>|  // Enter Event 
            print("Entering $On") ^ 

        ...

When a transition occurs Frame sends two special events. In the example above, if the system is in the `$Off` state 
and receives the `|turnOn|` event it will transition to `$On`. In doing so, the system will first send an exit event ``<``
to `$Off` which will print "Exiting $Off". Next the system will update the state to  ``$On`` and subsequently send 
an enter event ``>`` to `$On` which will print "Entering $On".

Enter and exit events provide "hooks" that can be used to initialize and clean up states. This capability is a powerful tool for 
better coding practices and often makes reasoning about complex system behavior much easier. 

.. code-block::
    :caption: Lamp System

    #Lamp

        -interface-

        turnOn      
        turnOff

        -machine-

        $Off   
            |>| print("Entering $Off") ^ 
            |<| print("Exiting $Off") ^

            |turnOn|            
                -> $On  ^              

        $On  
            |>| print("Entering $On") ^ 
            |<| print("Exiting $On") ^
            
            |turnOff|           
                -> $Off  ^           

    ##

So now we have specified a model for a lamp system, but how do we actually run it? Let's explore how to create
a complete Python program to run our Lamp. 

Frame Programs
^^^^^^^^^^^^^^

Frame, like other languages, provides a special entry point for execution called the `main` function. In main we will instantiate 
our Lamp and turn it on and off. 

.. code-block::
    :caption: Lamp Program

    fn main {
        var lamp:# = #Lamp()
        lamp.turnOn()
        lamp.turnOff()
    }

Frame's syntax for `main` does not have an argument list (e.g. `main(a,b)`) if no environment variables are passed 
to the program. 

We also see that a system controller is instantiated using `#Lamp()` which indicates this is a Frame system spec being
created.

.. code-block::
    :caption: Lamp Controller Instantiation

    var lamp:# = #Lamp()

Frame  uses the `var` keyword to declare variables and `:#` is a special Frame type for a system controller instance. 

After instantiation the lamp controller is told to turn itself on and then back off:

.. code-block::
    :caption: Lamp Operations

    lamp.turnOn()
    lamp.turnOff()

However, although this program will successfully transpile, it still won't run. That is because `print()` is not actually 
included in the runtime of the program. It will successfully transpile because Frame, as a metaprogamming language,
 assumes that undeclared
variables and function calls will be somehow be available at compile time or runtime depending on the nature of the 
target language. However, that is not yet true for our Lamp program as `print()` isn't yet included.

Let's see how to fix that. 

Metaprogramming
^^^^^^^^^^^^^^
To solve a wide range of compatibility issues with target languages, Frame supports **superstrings**. 
Superstrings are enclosed in backticks, the contents
of which are pasted directly into the target language code. 

Here we can see how to add a Python import using a superstring: 

.. code-block::
    :caption: Including Python Modules with Frame Superstring

    `import sys` // Frame superstring to inject Python import code

    fn main {
        ...
    }

This import will provide the needed Python library containing `print()`. With that final addition, we have a complete 
and working Frame program for a Lamp system in Python. 


Executing Frame Programs
^^^^^^^^^^^^^^

Frame makes designing, developing and testing state machines easy and intuitive.  Here is our final Lamp program:

.. code-block::
    :caption: Complete Lamp Program


    `import sys`

    fn main {
        var lamp:# = #Lamp()
        lamp.turnOn()
        lamp.turnOff()
    }

    #Lamp

        -interface-

        turnOn      
        turnOff

        -machine-

        $Off   
            |>| print("Entering $Off") ^ 
            |<| print("Exiting $Off") ^

            |turnOn|            
                -> $On  ^              

        $On  
            |>| print("Entering $On") ^ 
            |<| print("Exiting $On") ^
            
            |turnOff|           
                -> $Off  ^               

    ##

Here you can try running the program_.

.. _program: https://onlinegdb.com/fcVOr4FgpB

As a bonus, Frame also can generate system documentation as well: 

``UML``

.. image:: https://www.plantuml.com/plantuml/png/SoWkIImgAStDuG8oIb8L_DFI5AgvQc6yF30dMYjMGLVN3YJ91SGWDaZAIa5DsT38nBgaj2ZFFm_2vWAAGvMYo0FvK0KEgNafGFi0


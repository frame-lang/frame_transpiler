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

A System in Frame is indicated by an identifier preceeded by the '#' token:

.. code-block::

    #Lamp 

    foo

    ##

.. code-block::
    :caption: Machine Block and States

    #Lamp

        foo

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

        $Off                     // $Off state
            |turnOn|             // handle 'turnOn' event
                -> $On          // transition to $On state
                ^               // return from event handler

        $On                      // $On state
            |turnOff|            // handle 'turnOff' event
                -> $Off         // transition to $Off state
                ^               // return from event handler

    ##


Systems
^^^^^^^


..  code-block::


   #Lamp

       -interface-

       turnOn
       turnOff

       -machine-

       $Off
           |turnOn| -> $On ^

       $On
           |turnOff| -> $Off ^
   ##

The true power of Frame, however, is realized by the ability to generate both documentation and code from Frame specification documents:

``UML``

.. image:: https://www.plantuml.com/plantuml/png/SoWkIImgAStDuG8oIb8L_DFI5AgvQc6yF30dMYjMGLVN3YJ91SGWDaZAIa5DsT38nBgaj2ZFFm_2vWAAGvMYo0FvK0KEgNafGFi0

``C#``

..  code-block:: C#

    public partial class Lamp {
        public Lamp() {

            _state_ = _sOff_;
        }

        //===================== Interface Block ===================//

        public void turnOn() {
            FrameEvent e = new FrameEvent("turnOn",null);
            _state_(e);
        }

        public void turnOff() {
            FrameEvent e = new FrameEvent("turnOff",null);
            _state_(e);
        }


        //===================== Machine Block ===================//

        private void _sOff_(FrameEvent e) {
            if (e.Msg.Equals("turnOn")) {
                _transition_(_sOn_);
                return;
            }
        }

        private void _sOn_(FrameEvent e) {
            if (e.Msg.Equals("turnOff")) {
                _transition_(_sOff_);
                return;
            }
        }

        //=========== Machinery and Mechanisms ===========//

        private delegate void FrameState(FrameEvent e);
        private FrameState _state_;

        private void _transition_(FrameState newState) {
            FrameEvent exitEvent = new FrameEvent("<",null);
            _state_(exitEvent);
            _state_ = newState;
            FrameEvent enterEvent = new FrameEvent(">",null);
            _state_(enterEvent);
        }

    }

.. toctree::
    :caption: General
    :name: sec-about

    source/about/introduction

.. toctree::
    :caption: Getting Started
    :name: sec-getting-started

    source/getting_started/introduction/index

.. toctree::
    :caption: Intermediate Frame
    :name: sec-intermediate-frame

    source/intermediate_frame/index

.. toctree::
    :caption: Advanced Frame
    :name: sec-advanced-frame

    source/advanced_frame/index

.. toctree::
    :caption: Language
    :name: sec-language

    source/language/index

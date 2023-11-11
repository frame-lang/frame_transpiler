Frame v0.10.0 Documentation
=======================================================
Welcome! Here you will find the official v0.10.0 documentation for the **Frame System Design Language**.

What is Frame?
--------------
At the highest level, Frame is language for defining the behavior of systems in Frame specification docs, or specs. In practice, system designers and programmers can define Frame specifications that generate both UML documentation as well as code in (currently) 8 different programming languages, with many more to come.

The Frame transpiler is the open source CLI tool that turns Frame specs into UML or code. The transpiler, or Framepiler, is written in Rust and is straightforward to modify to add new target outputs for documentation and source code generation. You can experiment with Frame online `here <https://framepiler.frame-lang.org>`_.


A Markdown Language For System Designers
----------------------------------------

UML and other modeling specifications promote a visual-first paradigm. However this approach to system design requires (sometimes expensive) diagramming and modeling tools. Additionally - let’s just say it - working with boxes and lines to code can be a pain when systems get complex.

With Frame, anyone with a text editor can quickly sketch out a system design concept - notepad is just fine!



``Frame``

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

    getting_started/introduction/index

.. toctree::
    :caption: Intermediate Frame
    :name: sec-intermediate-frame

    intermediate_frame/index

.. toctree::
    :caption: Advanced Frame
    :name: sec-advanced-frame

    advanced_frame/index

.. toctree::
    :caption: Language
    :name: sec-language

    language/index

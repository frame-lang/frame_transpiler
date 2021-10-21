
Frame v0.7.0 Documentation
=======================================================
Welcome! Here you will find the official v0.7.0 documentation for the **Frame Software Architecture Language**.

What is Frame?
--------------
Frame is an easy to learn textual markdown language for defining system specifications that can generate both UML documentation as well as code in 7 langauges.

Frame is a simple yet powerful textual language for defining the dynamic behavior of systems, enabling software architicts and developers to quickly design - and code - state machines that comply with core UML statechart concepts.

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

Contents
--------

.. toctree::
   :maxdepth: 1
   :caption: General
   :name: sec-general

   about/index

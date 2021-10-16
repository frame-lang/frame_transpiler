
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




.. |codeblk1| code-block:: 

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

.. |img1| image:: https://www.plantuml.com/plantuml/png/SoWkIImgAStDuG8oIb8L_DFI5AgvQc6yF30dMYjMGLVN3YJ91SGWDaZAIa5DsT38nBgaj2ZFFm_2vWAAGvMYo0FvK0KEgNafGFi0



======
table
====== 
.. |code| literalinclude:: code.frm
=====

Contents
--------

.. toctree::
   :maxdepth: 1
   :caption: General
   :name: sec-general

   about/index
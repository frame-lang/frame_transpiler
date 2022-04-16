==================
Actions Block
==================

Declaring Actions
-----------------

Actions are declared in the -actions- block and observe all of the method
declaration syntax discussed in the :ref:`methods` section:

.. code-block::

    #SystemActions

      -actions-

      simpleActionDecl
      actionWithParams [p1:T p2:T]
      actionWithReturn : RetType
      theWorks [p1:T] : RetType

    ##

The corresponding C# code is generated:

`C#`

.. code-block::

    public partial class SystemActions {

        //===================== Actions Block ===================//

        protected virtual void simpleActionDecl_do() { throw new NotImplementedException(); }
        protected virtual void actionWithParams_do(T p1,T p2) { throw new NotImplementedException(); }
        protected virtual RetType actionWithReturn_do() { throw new NotImplementedException(); }
        protected virtual RetType theWorks_do(T p1) { throw new NotImplementedException(); }

    }

Currently Frame does not have a complete language for implementing actions, therefore the
Framepiler only generates stub code (or as appropriate for the
target language). For object oriented languages the generated controller can
serve as a base class from and the stubs implemented in a derived class.

In addition, there is also
the option to inline target language code
into Frame specs, which we will explore in later sections. This capability
addresses many simple use cases, but does tie the spec to a particular target language.

For our Lamp, we simply need two actions to drive the switch:

.. code-block::

    #Lamp

      -actions-

      turnOnLamp
      turnOffLamp

      -domain-

      var color:string = "white"

    ##

Next we will look at how these actions are called to implement behavior.

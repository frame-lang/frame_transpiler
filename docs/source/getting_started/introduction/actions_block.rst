==================
Actions Block
==================

Declaring Actions
-----------------

Actions are declared in the -actions- block and observe all of the method
declaration syntax discussed in the :ref:`methods` section:

.. code-block:: Frame

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

Currently Frame does not have a robust language for implementing actions, therefore the
Framepiler only generates stub code by default. The developer can then override or
augment the generated stubs with action definitions, depending on the
nature of the target language.

Although Frame does not yet have a full implementation language, in the advanced
section we will explore how to developers *can* inline target language code
into Frame specs. This capability addresses many use cases.

==================
Actions Block
==================

Declaring Actions
-----------------

Actions are declared in the -actions- block and observe all of the method
declaration syntax discussed above. However actions do not support the alias
notation:

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

===========================
Define a System
===========================

Systems Engineering methodology describes two broad categories of aspects to a system -
**structure** and **behavior**.

Frame is a domain specific language for for defining system behavior in specification
documents. A system type is identified by the `#` token followed by an identifier:

``Frame``

.. code-block::

    #MySystem
    ##

System specs are terminated with the `##` token. This is an empty system and
has no behavior. However, it still generates code. Using the Framepiler,
the `#MySystem` spec will generate the following JavaScript:

``JavaScript``

.. code-block::

    let MySystem = function () {

        let that = {};
        that.constructor = MySystem;


        return that;

    };

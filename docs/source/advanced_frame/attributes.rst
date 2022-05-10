Attributes
==========

Frame supports spec level configurability for a number of aspects of the controller
code generation. Frame adheres to the Rust language specifications outlined
`here <https://doc.rust-lang.org/reference/attributes.html>`.

Controller Language Override Attribute
--------------------------------------

Frame specs can override the command line language for the generated controller
using the following ``language`` attribute:

#[language="rust"]

Serialization
-------------

Frame controllers can be made serializable by adding the ``Marshall`` attribute
to the system spec:

#[derive(Marshal)]

Managed
-------

Frame supports a manager relationship between machines using the ``managed``
attribute, which accepts the name of the manager type as a parameter:

.. code-block::

    #[managed(ManagerTypeName)]

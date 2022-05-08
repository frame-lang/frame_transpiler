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

Frame supports a manager relationship between machines by the ``Managed``
attribute combined with the ``mom`` attribute. ``mom`` stands for Machine Operating
Machine. To have Frame autogenerate a member variable for a manager, use these
type attributes together:

.. code-block::

    #[derive(Managed)]
    #[mom="MomTypeName"]

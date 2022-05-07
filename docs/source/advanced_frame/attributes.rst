Attributes
==========

Frame supports spec level configurability for a number of aspects of the controller
code generation. Frame adheres to the Rust language specifications outlined
`here <https://doc.rust-lang.org/reference/attributes.html>`.

Controller Language Override Attribute
--------------------------------------

#[language="rust"]

Serialization
-------------

#[derive(Marshal)]

Managed
-------

#[derive(Managed)]
#[mom="TrafficLightMom"]

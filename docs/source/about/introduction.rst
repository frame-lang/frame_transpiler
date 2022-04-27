Introduction
============

Frame is a Domain Specific Language (DSL) for creating behavioral
specifications for systems and has
evolved from years of low-level experimentation with UML Statecharts and
how to structure object-oriented classes to implement them.

Why is this an interesting thing to do? The answer is that software is
**always** built out of state machines. However, they are usually structured
*badly*.

Frame is about easily defining state machines correctly with the additional
benefits of being able to generate them in multiple target languages
(8 so far) as well as generating UML documentation.

This manual is structured to present an initial introduction to the language
itself and then progress into explaining the how and why of Frame's
state machine code generation. Along the way we will explore Frame's
alignment with and deviations from UML Statecharts
and finally delve into the advanced features that differentiate Frame from
other approaches to state machines and visual software design.

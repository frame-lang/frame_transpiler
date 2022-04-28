
Intermediate Frame
==================

Frame's syntax is inspired by and supports key concepts from Statecharts, a state machine visual
language invented by Dr. David Harel in his
`1987 paper <https://www.sciencedirect.com/science/article/pii/0167642387900359>`_.
Statechart notation is used in both UML software
modeling and the SYSML systems modeling languages as the standard flavor of
state machine diagrams.

Although Frame has some deviations from a strict interpretation
of some of the more complex aspects of the Statechart semantics, these
differences enable a simplified implementation of the generated controller code
as well as new capabilities
not addressed in Statecharts. These advanced features will be discussed in
detail in the advanced section.

We will explore the four key big ideas of Statecharts and see how the Frame
language supports them. For context we will start with a short,
highly selective and very opinionated history of state machine implementations.

.. toctree::
    :maxdepth: 1
    :name: toc-intermediate-frame-introduction

    machine_evolution
    hsm
    history
    orthogonality

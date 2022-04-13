============
Domain Block
============

The domain block contains the internal system data.

.. code-block::

    #Light

    -domain-

    var location:string = nil

    ##

Domain variables follow the general declaration syntax discussed in the
:ref:`variable_declarations` section.

Domain variables can be disambiguated from variables with the same name in
different scopes by prefixing it with `#.`. For example `#.location` would
reference the domain variable shown above.

============
Domain Block
============

The domain block contains the system data. All actions and event handlers
can access the domain data.

.. code-block::

    #Item

    -domain-

    var location:string = "warehouse" --- typed variable
    const sku = 12345                 --- untyped constant

    ##

Domain variables follow the general declaration syntax discussed in the
:ref:`variable_declarations` section.

Domain Scope Prefix
-------------------
Domain variables can be disambiguated from variables with the same name in
different scopes by prefixing it with `#.`. For example `#.location` would
reference the domain variable shown above. Variables from other scopes also
have scope prefixes which will be discussed in context.

Our lamp will have just one data member - the color of the light.

.. code-block::

    #Lamp

    -domain-

    var color:string = "white"

    ##

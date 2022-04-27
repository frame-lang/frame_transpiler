============
Domain Block
============

The domain block contains the system data which are in scope for actions and
event handlers. For instance an item in an e-commerce site
might have a few member variables.

.. code-block::

    #ThingForSale

    -domain-

    var item_id:i64 = newId()         --- expression intitializer
    var location:string = "warehouse" --- typed variable
    const sku = 12345                 --- untyped constant

    ##

Domain variables follow the general declaration syntax discussed in the
:ref:`variable_declarations` section.

Domain Scope Prefix
-------------------
Domain variables can be disambiguated from variables with the same name in
different scopes by prefixing it with `#.<domain_var>`. For example

.. code-block::

    print("The domain variable location value is " + #.location)

would reference the domain variable declared above. Variables from other scopes
also have scope prefixes which will be discussed in context.

Returning to our `#Lamp` project, we will add a single data member to it -
the color of the light:

.. code-block::

    #Lamp

    -domain-

    var color:string = "white"

    ##

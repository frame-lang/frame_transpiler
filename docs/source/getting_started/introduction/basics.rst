==========
The Basics
==========

This section will discuss aspects to the language that are common throughout
a system specification.

Comments
--------

Comments are single line and indicated by three (3) dashes:

.. code-block::

    --- this is a single line comment
    --- so is this


Parameter Declarations
----------------------

Parameters are declared as follows and separated by whitespace:

.. code-block::

    param:type

The name is required but the :type is optional. Parameter lists are one or
more parameter declarations enclosed in brackets:

.. code-block::

    [<param_list>]

Therefore parameter lists can be declared either of these two ways:

.. code-block::

    [param1 param2]

    --- or ---

    [param1:type1 param2:type2]

.. _variable_declarations:

Variable Declarations
---------------------

Variable and constant declarations have the following format:

.. code-block::

    <var | const> <name> : <type_opt> = <intializer>

    var x:int = 1
    const name = "Steve"

The type is optional but the initializer is required.

Frame will transpile into the closest semantic equivalents in the target
language. At this time Frame does not enforce mutability itself but instead
relies on the underlying language to do so.

It may be necessary have an arbitrary string as a type. To do so, use the
string literal syntax ``weirdVar:`MyWeirdType```.

.. code-block:: language

    -interface-

    foo [p1:`WeirdParamType`] : `WeirdRetVal`

    -machine-

    $Bar
        |foo| [p1:`WeirdParamType`] : `WeirdRetVal` 


If you transpile into a language that requires a type and you don’t provide one,
a token such as `<?>` is substituted. Conversely, if you add a type and transpile
into a language that doesn’t require one, the Framepiler ignores it.

.. _methods:

Methods
-------

All methods (for all blocks) have a similar syntax:

.. code-block::

    <method-name> <parameters-opt> <return-value-opt>

As implied above, the parameters and return value are optional. Here are the
permutations for method declarations:

.. code-block::

    method_name
    method_name [param]
    method_name [param:type]
    method_name [param1 param2]
    method_name [param1:type param2:type]
    method_name : return_value
    method_name [param1:type param2:type] : return_value

Whitespace separators
---------------------

One important difference between Frame and other languages is the lack of any
commas or semicolons as separators. Instead Frame relies on whitespace to
delineate tokens:

.. code-block:: language

    --- lists ---

    [x y]
    [x:int y:string]
    (a b c)
    (d() e() f())

    --- statements ---

    a() b() c()
    var x:int = 1

Unlike other languages where structured whitespace is significant (e.g. Python),
Frame’s use of whitespace is unstructured. Frame only separates tokens with
whitespace and does not insist on any pattern of use.

The esthetic goal is to be as spare and clean as possible, but it may take some
getting used to.

Lists
-----

List come in two flavors - *parameter lists* and *expression lists*.

Frame uses square brackets to denote parameter lists:

.. code-block::

    [x y]
    [x:int y:string]

Expression lists are parenthetical lists of expressions that currently
only hold arguments for function or method calls:

.. code-block::

    foo(b a r)

Next
----

Now that you have a basic introduction to some common syntax, we are now ready
to explore a central concept in the Frame architecture - the
**FrameEvent**.

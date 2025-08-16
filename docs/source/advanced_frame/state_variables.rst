===============
State Variables
===============

State variables are variables scoped to a state and are initialized upon
entry to a state. As with state parameters, they go out of scope when
the state compartment does.

State variables are declared after the state declaration and before any
event handlers:

``Frame``

.. code-block::

    system StateVariableDemo {

        interface:
            print()
            updateName(newName: string)
            forgetMe()

        machine:
            $JoeName {
                var name: string = "Joe"

                print() {
                    print(name)
                    return
                }
                updateName(newName: string) {
                    name = newName
                    return
                }
                forgetMe() {
                    -> $ResetName
                    return
                }
            }

            $ResetName {
                $>() {
                    -> $JoeName
                    return
                }
            }

        actions:
            print(msg: string) {}
    }

The `StateVariableDemo` system start state, `$JoeName`, initializes the `name`
state variable to "Joe". The `updateName()` handler will update the state
variable such that the next `print()` event will print the new name.

When `forgetMe()` is handled, the machine will cycle through the `$ResetName`
state, losing reference to the previous state compartment and creating a
new one upon reentry. This reentry will reset the state variable to "Joe".

State variables are always reset upon reentry to a state except in one important
situation - the return of the machine to a historical state. We will see
how state compartments facilitate that capability.

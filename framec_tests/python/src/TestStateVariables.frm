fn main() {
    var demo = StateVariableDemo()
}

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
        print(msg: string) {
            // Print implementation
        }
}
@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    demo = StateVariableDemo()
}

system StateVariableDemo {

    interface:
        print()
        updateName(newName: string)
        forgetMe()

    machine:
        $JoeName {
            var name:string = "Joe"

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
            }
        }

        $ResetName {
            $>() {
                -> $JoeName
            }
        }

    actions:
        print(msg: string) {}
}

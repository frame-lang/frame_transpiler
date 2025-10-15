# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system DomainTypedTest {

    machine:
        $Ready {
            displayName() {
                print("My name is " + name)
                return
            }
        }

    actions:
        printName() {
            print("My name is " + name) 
        }

    domain:
        var item_id = 42                                      # untyped variable
        var name:string = "Boris"                             # typed variable
        var s = [2, 3, 5, 7, 11, 13]   # list type
}
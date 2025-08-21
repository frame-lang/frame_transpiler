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
        var item_id = 42                                      // untyped variable
        var name:string = "Boris"                             // typed variable
        var s:`[]int` = `[6]int{2, 3, 5, 7, 11, 13}[1:4]`   // custom type
}
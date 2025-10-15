# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system Test {
    machine:
        $Start {
            test() {
                if true {
                    print("if")
                } elif false {
                    print("elif")
                }
            }
        }
}
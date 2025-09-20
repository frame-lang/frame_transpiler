# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system Test {
    machine:
        $Start {
            test() {
                if true {
                    return "if"
                } elif false {
                    return "elif"
                }
                return "none"
            }
        }
}
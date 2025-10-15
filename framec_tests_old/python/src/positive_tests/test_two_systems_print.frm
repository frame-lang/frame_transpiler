# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system Test1 {
    machine:
        $Start {
            $>() {
                print("Test1 Enter")
            }
        }
}

system Test2 {
    machine:
        $Start {
            $>() {
                print("Test2 Enter")
            }
        }
}
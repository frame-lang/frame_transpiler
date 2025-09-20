# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system Simple {
    machine:
        $Start {
            $>() {
                print("Hello")
            }
        }
}
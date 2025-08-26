// Basic module scope test without variables

system TestBasic {
    machine:
        $Start {
            $>() {
                print("Basic module scope test")
            }
        }
}
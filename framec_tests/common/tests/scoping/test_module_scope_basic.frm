# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Basic module scope test without variables

system TestBasic {
    machine:
        $Start {
            $>() {
                print("Basic module scope test")
            }
        }
}
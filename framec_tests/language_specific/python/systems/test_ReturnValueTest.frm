@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system ReturnValueTest {

    interface:
        getValue() : String

    machine:
        $Start {
            getValue() : String {
                return "hello"
            }
        }
}

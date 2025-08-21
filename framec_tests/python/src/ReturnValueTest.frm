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
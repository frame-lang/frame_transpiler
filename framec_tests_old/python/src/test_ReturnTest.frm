# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
system ReturnTest {

    interface:
        testReturn()

    machine:
        $Start {
            testReturn() {
                return
            }
        }
}
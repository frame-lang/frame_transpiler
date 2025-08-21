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
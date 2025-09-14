system A {

    actions:

        testPythonStyle() {
            if x > 5:
                doSomething()
            elif y < 10:
                doOther()
            else:
                doDefault()
        }

        testBracedStyle() {
            if x > 5 {
                doSomething()
                doMore()
            } elif y < 10 {
                doOther()
                doAnother()
            } else {
                doDefault()
                doFinal()
            }
        }

        testMixed() {
            if simpleCondition:
                singleStatement()
            elif complexCondition {
                firstStatement()
                secondStatement()
            } else:
                fallbackStatement()
        }

}
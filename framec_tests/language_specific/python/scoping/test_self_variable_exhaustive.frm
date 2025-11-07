# Python native bodies — self.variable exhaustive tests

system SelfVariableExhaustive {
    operations:
        process(val: int) {
            print("N4: process(" + str(val) + ")")
        }

        calculate(val: int): int {
            print("N5: calculate(" + str(val) + ")")
            return val * 10
        }

    interface:
        run_tests()

    machine:
        $Start {
            run_tests() {
                print("=== EXHAUSTIVE self.variable TESTS ===")
                print("")

                # LVALUE TESTS
                print("--- LVALUE TESTS ---")
                self.x = 100
                print("L1: self.x = 100 -> " + str(self.x))
                self.x = 50 + 25
                print("L2: self.x = 50 + 25 -> " + str(self.x))
                temp = 200
                self.x = temp
                print("L3: self.x = temp -> " + str(self.x))
                self.y = 300
                self.x = self.y
                print("L4: self.x = self.y -> " + str(self.x))
                self.x = self.y + 100
                print("L5: self.x = self.y + 100 -> " + str(self.x))
                self.x = (self.y * 2) + (50 - 10)
                print("L6: self.x = (self.y * 2) + (50 - 10) -> " + str(self.x))
                self.msg = "Hello"
                print("L7: self.msg = 'Hello' -> " + self.msg)
                self.msg = "Hello " + "World"
                print("L8: self.msg = 'Hello ' + 'World' -> " + self.msg)

                print("")
                # RVALUE TESTS
                print("--- RVALUE TESTS ---")
                r1 = self.x
                print("R1: var r1 = self.x -> " + str(r1))
                r2 = self.x + 50
                print("R2: var r2 = self.x + 50 -> " + str(r2))
                r3 = self.x * 2
                print("R3: var r3 = self.x * 2 -> " + str(r3))
                r4 = self.x / 10
                print("R4: var r4 = self.x / 10 -> " + str(r4))
                r5 = 1000 - self.x
                print("R5: var r5 = 1000 - self.x -> " + str(r5))
                r6 = self.x + self.y
                print("R6: var r6 = self.x + self.y -> " + str(r6))
                if self.x > 500:
                    print("R7: self.x > 500 -> true")
                else:
                    print("R7: self.x > 500 -> false")
                if self.x == self.y:
                    print("R8: self.x == self.y -> true")
                else:
                    print("R8: self.x == self.y -> false")
                r9 = self.msg
                print("R9: var r9 = self.msg -> " + r9)
                r10 = self.msg + " Extended"
                print("R10: var r10 = self.msg + ' Extended' -> " + r10)

                print("")
                # NESTED EXPRESSION TESTS
                print("--- NESTED EXPRESSION TESTS ---")
                n1 = self.x
                print("N1: self.x -> " + str(n1))
                n2 = self.x + 10 * 2
                print("N2: self.x + 10 * 2 -> " + str(n2))
                n3 = self.x * 2 + self.y / 3 - 100
                print("N3: self.x * 2 + self.y / 3 - 100 -> " + str(n3))
                self.process(self.x)
                self.process(self.calculate(self.x))
                if (self.x > 100) and (self.y < 500):
                    print("N6: (self.x > 100) and (self.y < 500) -> true")
                else:
                    print("N6: (self.x > 100) and (self.y < 500) -> false")
                n7 = self.x if self.x > self.y else self.y
                print("N7: max(self.x, self.y) -> " + str(n7))

                print("")
                # EDGE CASES
                print("--- EDGE CASES ---")
                self.x = self.x
                print("E1: self.x = self.x -> " + str(self.x))
                self.x = self.x + self.x
                print("E2: self.x = self.x + self.x -> " + str(self.x))
                self.z = 50
                self.y = self.z
                self.x = self.y
                print("E3: Chain assignment -> x=" + str(self.x) + ", y=" + str(self.y) + ", z=" + str(self.z))
                e4 = "Value: " + str(self.x)
                print("E4: 'Value: ' + str(self.x) -> " + e4)
                print("")
                print("=== ALL TESTS COMPLETED ===")
                return
            }
        }

    domain:
        x: int = 0
        y: int = 0
        z: int = 0
        msg: str = ""
}

fn main() {
    test = SelfVariableExhaustive()
    test.run_tests()
}


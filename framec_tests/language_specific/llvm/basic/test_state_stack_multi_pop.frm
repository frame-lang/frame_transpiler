# Validates multiple consecutive state stack pops restore prior compartments.

system StateStackMultiPop {
    interface:
        advance()
        retreat()

    machine:
        $A {
            advance() {
                print("A advance")
                $$[+]
                -> $B
            }

            retreat() {
                print("A retreat noop")
            }
        }

        $B {
            $>() {
                print("B enter")
                return
            }

            advance() {
                print("B advance")
                $$[+]
                -> $C
            }

            retreat() {
                print("B retreat pop")
                -> $$[-]
            }
        }

        $C {
            $>() {
                print("C enter")
                return
            }

            retreat() {
                print("C retreat pop")
                -> $$[-]
            }

            advance() {
                print("C advance noop")
            }
        }
}

fn main() {
    var runner = StateStackMultiPop()
    runner.advance()
    runner.advance()
    runner.retreat()
    runner.retreat()
}

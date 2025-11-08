# TS override: History102 cleanup

system History102 {
    interface:
        gotoB()
        gotoC()
        gotoD()
        ret()

    machine:
        $A {
            gotoB() { -> $B }
            gotoC() { -> $C }
        }
        $B { gotoD() { -> $D("B") } }
        $C { gotoD() { -> $D("C") } }
        $D(previous_state) {
            ret() {
                if (previous_state == "B") { -> $B }
                else if (previous_state == "C") { -> $C }
            }
        }
}


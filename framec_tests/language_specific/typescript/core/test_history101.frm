# TS override: History101 cleanup

system History101 {
    interface:
        gotoB()
        gotoC()
        gotoD()

    machine:
        $A {
            gotoB() { -> $B }
            gotoC() { -> $C }
        }
        $B { gotoD() { -> $D } }
        $C { gotoD() { -> $D } }
        $D { }
}


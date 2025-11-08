# TS override: History104 (reentry vars)

system History104 {
    interface:
        gotoB()
        gotoC()
        gotoD()
        retToB()
        retToC()

    machine:
        $A {
            $>() { console.log("In $A"); return }
            gotoB() { -> $B }
        }
        $B {
            var b = 0
            $>() { console.log("Entering $B. b = " + String(b)); return }
            gotoC() { console.log("--------------"); console.log("Going to $C."); console.log("--------------"); -> $C }
            gotoD() { b = 1; console.log("Going to $D. b = " + String(b)); -> $D }
        }
        $C {
            var c = 0
            $>() { console.log("Entering $C. c = " + String(c)); return }
            gotoD() { c = 1; console.log("Going to $D. c = " + String(c)); $$[+] -> $D }
        }
        $D {
            $>() { console.log("In $D"); return }
            retToB() { console.log("Returning to $B"); -> $B }
            retToC() { console.log("Returning to $C"); $$[-] }
        }
}


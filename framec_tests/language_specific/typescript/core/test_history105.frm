# TS override: History105 (push/pop tracking)

system History105 {
    interface:
        gotoB()
        gotoC()
        ret()

    machine:
        $A {
            var a = 0
            $>() { console.log("In $A. a = " + String(a)); return }
            gotoB() { console.log("Transitioning to $B"); -> $B }
            gotoC() { a = a + 1; console.log("Incrementing a to " + String(a)); $$[+] -> $C }
        }
        $B {
            var b = 0
            $>() { console.log("In $B. b = " + String(b)); return }
            gotoC() { b = b + 1; console.log("Incrementing b to " + String(b)); $$[+] -> $C }
        }
        $C {
            $>() { console.log("In $C"); return }
            ret() { console.log("Return to previous state"); $$[-] }
        }
}


# TS override: History103 (B/C/C pop)

system History103 {
    interface:
        gotoC()
        ret()

    machine:
        $A { gotoC() { $$[+] -> $C } }
        $B { gotoC() { $$[+] -> $C } }
        $C { ret() { $$[-] } }
}


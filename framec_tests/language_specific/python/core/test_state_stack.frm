# Python-specific: convert domain var to native assignment

system StateContextStack {
    interface:
        to_a()
        to_b()
        to_c()
        inc()
        value() : int
        push()
        pop()

    machine:
        $A {
            var x:int = 0
            $>() { log("A:>"); return }
            <$() { log("A:<"); return }
            inc() { x = x + 1; return }
            value() : int { return x }
            to_a() { -> $A }
            to_b() { -> $B }
            to_c() { -> $C }
            push() { $$[+]; return }
            pop() { -> $$[-] }
        }
        $B {
            var y:int = 0
            $>() { log("B:>"); return }
            <$() { log("B:<"); return }
            inc() { y = y + 5; return }
            value() : int { return y }
            to_a() { -> $A }
            to_b() { -> $B }
            to_c() { -> $C }
            push() { $$[+]; return }
            pop() { -> $$[-] }
        }
        $C {
            var z:int = 0
            $>() { log("C:>"); return }
            <$() { log("C:<"); return }
            inc() { z = z + 10; return }
            value() : int { return z }
            to_a() { -> $A }
            to_b() { -> $B }
            to_c() { -> $C }
            push() { $$[+]; return }
            pop() { -> $$[-] }
        }

    actions:
        log(msg:str) {}

    domain:
        tape = []
}


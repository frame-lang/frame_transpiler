fn main() {
    var sys = HSM1()
    sys.a()
    sys.b()
}

system HSM1 {

    interface:
        a()
        b()

    machine:
        $S0 {
            b() {
                -> "b" $S3
                return
            }
        }

        $S1 => $S0 {
            a() {
                -> "a" $S2
                return
            }
        }

        $S2 => $S0 {
            a() {
                -> "a" $S1
                return
            }
        }
        
        $S3 {}
}
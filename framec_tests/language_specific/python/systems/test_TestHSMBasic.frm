@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = HSM1()
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
            }
        }

        $S1 => $S0 {
            a() {
                -> "a" $S2
            }
        }

        $S2 => $S0 {
            a() {
                -> "a" $S1
            }
        }
        
        $S3 {}
}

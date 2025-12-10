@target python_3

# @core
# @run-expect: start
# @run-expect: done

system AsyncDemoPy {
    machine:
        $A {
            $>($enter) {
                # Minimal async-parity smoke: print markers on entry.
                print("start")
                print("done")
            }
        }
}

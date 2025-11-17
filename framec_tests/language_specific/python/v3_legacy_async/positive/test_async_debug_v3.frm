@target python_3

# Port of legacy test_async_debug.frm to V3 syntax.

system AsyncDebug {
    interface:
        getData(id)
        normalMethod(x)

    machine:
        $Ready {
            getData(id) {
                print("ID: " + str(id))
                -> $Processing
            }
            normalMethod(x) {
                print("X: " + str(x))
                return x
            }
        }
        $Processing {
            $>() { }
        }
}

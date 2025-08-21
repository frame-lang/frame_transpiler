
#[codegen.python.code.public_state_info:bool="true"]


system StateContextSm {
    interface:
        Start()
        LogState()
        Inc() : int
        Next(arg:int)
        // Change(arg:int)

    machine:
        $Init {
            var w:int = 0

            $>() {
                w = 3
                log("w", w)
                return
            }

            Inc() : int {
                w = w + 1
                log("w", w)
                return w
            }

            LogState() {
                log("w", w)
                return
            }

            Start() {
                -> (3, w) $Foo
                return
            }
        }

        $Foo {
            var x:int = 0

            $>(a:int, b:int) {
                log("a", a)
                log("b", b)
                x = a * b
                log("x", x)
                return
            }

            <$(c:int) {
                log("c", c)
                x = x + c
                log("x", x)
                return
            }

            LogState() {
                log("x", x)
                return
            }

            Inc() : int {
                x = x + 1
                log("x", x)
                return x
            }

            Next(arg:int) {
                var tmp = arg * 10  // FIXME: Swapping this to 10 * arg causes a parse error!
                (10) -> (tmp) $Bar(x)
                return
            }

            // Change(arg:int) {
            //     var tmp = x + arg
            //     -> $Bar(tmp)
            //     return
            // }
        }

        $Bar(y:int) {
            var z:int = 0

            $>(a:int) {
                log("a", a)
                log("y", y)
                z = a + y
                log("z", z)
                return
            }

            LogState() {
                log("y", y)
                log("z", z)
                return
            }

            Inc() : int {
                z = z + 1
                log("z", z)
                return z
            }

            // Change(arg:int) {
            //     var tmp = y + z + arg
            //     log("tmp", tmp)
            //     ->> $Init
            //     return
            // }
        }

    actions:
        log(name:str, val:int) {
        }

    domain:
        var tape = `[]`
}

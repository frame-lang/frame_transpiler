
#[codegen.python.code.public_state_info:bool="true"]


#StateContextSm
    -interface-
    Start
    LogState
    Inc : int
    Next [arg:int]
    // Change [arg:int]

    -machine-
    $Init
        var w:int = 0

        |>|
            w = 3
            log("w", w)
            ^

        |Inc|:int
            w = w + 1
            log("w", w)
            ^(w)

        |LogState|
            log("w", w)
            ^

        |Start|
            -> (3, w) $Foo
            ^

    $Foo
        var x:int = 0

        |>| [a:int, b:int]
            log("a", a)
            log("b", b)
            x = a * b
            log("x", x)
            ^

        |<| [c:int]
            log("c", c)
            x = x + c
            log("x", x)
            ^

        |LogState|
            log("x", x)
            ^

        |Inc|:int
            x = x + 1
            log("x", x)
            ^(x)

        |Next| [arg:int]
            var tmp = arg * 10  // FIXME: Swapping this to 10 * arg causes a parse error!
            (10) -> (tmp) $Bar(x)
            ^

        // |Change| [arg:int]
        //     var tmp = x + arg
        //     -> $Bar(tmp)
        //     ^

    $Bar [y:int]

        var z:int = 0

        |>| [a:int]
            log("a", a)
            log("y", y)
            z = a + y
            log("z", z)
            ^

        |LogState|
            log("y", y)
            log("z", z)
            ^

        |Inc|:int
            z = z + 1
            log("z", z)
            ^(z)

        // |Change| [arg:int]
        //     var tmp = y + z + arg
        //     log("tmp", tmp)
        //     ->> $Init
        //     ^

    -actions-
    log [name:str, val:int]

    -domain-
    var tape = `[]`
##

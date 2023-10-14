#[codegen.rust.features.generate_action_impl:bool="true"]
#[codegen.rust.features.runtime_support:bool="true"]
#StateContextSm
    -interface-
    Start
    LogState
    Inc : i32
    Next [arg:i32]
    Change [arg:i32]

    -machine-
    $Init
        var w:i32 = 0

        |>|
            w = 3
            log("w", w)
            ^

        |Inc|: i32
            w = w + 1
            log("w", w)
            ^(w)

        |LogState|
            log("w", w)
            ^

        |Start|
            -> (3, w) "transition 1" $Foo
            ^

    $Foo
        var x:i32 = 0

        |>| [a:i32, b:i32]
            log("a", a)
            log("b", b)
            x = a * b
            log("x", x)
            ^

        |<| [c:i32]
            log("c", c)
            x = x + c
            log("x", x)
            ^

        |LogState|
            log("x", x)
            ^

        |Inc|: i32
            x = x + 1
            log("x", x)
            ^(x)

        |Next| [arg:i32]
            var tmp = arg * 10  --- FIXME: Swapping this to 10 * arg causes a parse error!
            (10) -> (tmp) "transition 2" $Bar(x)
            ^

        |Change| [arg:i32]
            var tmp = x + arg
            --- ->> "change-state 1" $Bar(tmp)
            ^

    $Bar [y:i32]

        var z:i32 = 0

        |>| [a:i32]
            log("a", a)
            log("y", y)
            z = a + y
            log("z", z)
            ^

        |LogState|
            log("y", y)
            log("z", z)
            ^

        |Inc|: i32
            z = z + 1
            log("z", z)
            ^(z)

        |Change| [arg:i32]
            var tmp = y + z + arg
            log("tmp", tmp)
            --- ->> "change-state 2" $Init
            ^

    -actions-
    log [name:String, val:i32]

    -domain-
    var tape:Log = `vec![]`
##

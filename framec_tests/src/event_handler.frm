#EventHandler
    -interface-
    LogIt [x:i32]
    LogAdd [a:i32 b:i32]
    LogReturn [a:i32 b:i32] : i32
    PassAdd [a:i32 b:i32]
    PassReturn [a:i32 b:i32] : i32

    -machine-
    $S1
        |LogIt| [x:i32]
            log("x" x) ^

        |LogAdd| [a:i32 b:i32]
            log("a" a)
            log("b" b)
            log("a+b" a+b) ^

        |LogReturn| [a:i32 b:i32] : i32
            log("a" a)
            log("b" b)
            var r = a + b
            log("r" r)
            -> ^(r)

        |PassAdd| [a:i32 b:i32]
            -> $S2(a+b) ^

        |PassReturn| [a:i32 b:i32]: i32
            var r = a + b
            log("r" r)
            -> $S2(r) ^(r)

    $S2 [p:i32]

        |>|
            log("p" p) ^

    -actions-
    log [val:i32]

    -domain-
    var tape:Log = `vec![]`
##

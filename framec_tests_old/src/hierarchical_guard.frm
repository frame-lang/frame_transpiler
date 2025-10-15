#HierarchicalGuard
    -interface-
    A [i:i16]
    B [i:i16]

    -machine-
    $I  |>| -> $S ^

    $S
        |A| [i:i16]
            log("S.A")
            i < 10 ?
                -> $S0
            :
                -> $S1
            :: ^

        |B| [i:i16]
            log("S.B")
            i < 10 ?
                -> $S2
            :
                -> $S3
            :: ^

    $S0 => $S
        |A| [i:i16]
            log("S0.A")
            i > 0 ?
                -> $S2
            :            // fall through else branch
            :: :>

        |B| [i:i16]
            log("S0.B")
            i > 0 ?
            :            // fall through then branch
                -> $S1
            :: :>

    $S1 => $S0
        |A| [i:i16]
            log("S1.A")
            i > 5 ?
                -> $S3
            :            // fall through else branch
            :: :>

    $S2 => $S1
        |A| [i:i16]
            log("S2.A")
            i > 10 ?
                -> $S4
            :            // fall through then branch
            :: :>

        |B| [i:i16]
            log("S2.B")
            i > 10 ?!
            :            // fall through then branch
                -> $S4
            :: :>

    $S3 => $S
        |A| [i:i16]
            log("S3.A")
            i > 0 ?
                log("stop") ^
            :
                log("continue")
            :: :>

        |B| [i:i16]
            log("S3.B")
            i > 0 ?
                log("continue")
            :
                log("stop") ^
            :: :>

    $S4

    -actions-
    log [msg:String]

    -domain-
    var tape:Log = `vec![]`
##

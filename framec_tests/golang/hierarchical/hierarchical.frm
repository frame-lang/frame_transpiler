```
package hierarchical

import "golang/framelang"

```
#Hierarchical
    -interface-
    A
    B
    C
    -machine-
    $I  |>| -> $S ^

    $S
        |>| enter("S") ^
        |<| exit("S") ^
        |A| log("S.A")
            -> $S0 ^
        |B| log("S.B")
            -> $S1 ^

    $S0 => $S
        |>| enter("S0") :>
        |<| exit("S0") :>
        |A| log("S0.A")     --- override parent handler
            -> $T ^
        |B| log("S0.B") :>  --- do this, then parent handler
        |C| log("S0.C")     --- extend parent handler
            -> $S2 ^

    $S1 => $S
        |>| enter("S1") ^
        |<| exit("S1") ^
                            --- defer to parent for A
        |B| log("S1.B") :>  --- do this, then parent, which transitions here
        |C| log("S1.C") :>  --- propagate message not handled by parent

    $S2 => $S0
        |>| enter("S2") :>
        |<| exit("S2") :>
        |B| log("S2.B") :>  --- will propagate to S0 and S
        |C| log("S2.C")
            -> $T :>        --- continue after transition (should be ignored)

    $S3 => $S1
        |>| enter("S3") :>
        |<| exit("S3") :>
                            --- defer to grandparent for A
        |B| log("S3.B")     --- override and move to sibling
            -> $S2 ^

    $T
        |>| enter("T") ^
        |<| exit("T") ^
        |A| log("T.A")
            -> $S ^
        |B| log("T.B")
            -> $S2 ^
        |C| log("T.C")
            -> $S3 ^

    -actions-
    enter [msg:string]
    exit [msg:string]
    log [msg:string]

    -domain-
    var enters:`[]string` = `[]string{}`
    var exits:`[]string` = `[]string{}`
    var tape:`[]string` = `[]string{}`
##

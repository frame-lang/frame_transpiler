#Basic
    -interface-
    A
    B
    -machine-

    $S0
        |>| entered("S0") ^
        |<| left("S0") ^
        |A| -> $S1 ^

    $S1
        |>| entered("S1") ^
        |<| left("S1") ^
        |B| -> $S0 ^

    -actions-
    entered[msg:&String]
    left[msg:&String]

    -domain-
    var entry_log:Log = `vec![]`
    var exit_log:Log = `vec![]`
##

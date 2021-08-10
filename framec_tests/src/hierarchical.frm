#Hierarchical
    -interface-
    A
    B
    C
    -machine-
    
    $I
        |>| entered("I") ^
        |<| left("I") ^
        |A| -> $S0 ^
    
    $S
        |>| entered("S") ^
        |<| left("S") ^
        |C| -> $I ^

    $S0 => $S
        |>| entered("S0") ^
        |<| left("S0") ^
        |B| -> $S1 ^

    $S1 => $S
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

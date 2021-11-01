#[codegen.rust.features.generate_action_impl:bool="true"]
#[codegen.rust.features.runtime_support:bool="true"]
#Basic
    -interface-
    A
    B
    -machine-

    $S0
        |>| entered("S0") ^
        |<| left("S0") ^
        |A| -> "ooh" $S1 ^

    $S1
        |>| entered("S1") ^
        |<| left("S1") ^
        |B| -> "aah" $S0 ^

    -actions-
    entered[msg:&String]
    left[msg:&String]

    -domain-
    var entry_log:Log = `vec![]`
    var exit_log:Log = `vec![]`
##

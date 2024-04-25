
#[codegen.python.code.public_state_info:bool="true"]

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
    entered[msg:str]
    left[msg:str]

    -domain-
    var entry_log = `[]`
    var exit_log = `[]`
##

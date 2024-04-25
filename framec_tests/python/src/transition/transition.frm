
#[codegen.python.code.public_state_info:bool="true"]

#TransitionSm
    -interface-
    transit
    change

    -machine-
    $S0
        |<| exit("S0") ^
        |transit|
            -> $S1 ^

    $S1
        |>| enter("S1") ^
        |change|
            -> $S2
            ^

    $S2
        |<| exit("S2") ^
        |transit|
            -> $S3 ^

    $S3
        |>| enter("S3") ^
        |<| exit("S3") ^
        |transit|
            -> $S4 ^


    $S4
        |>| enter("S4")
            -> $S0
            ^


    -actions-
    enter [state:str]
    exit [state:str]

    -domain-
    var enters = `[]`
    var exits = `[]`
##

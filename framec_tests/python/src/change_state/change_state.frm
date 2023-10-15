#![generate_frame_event]
#[codegen.python.code.public_state_info:bool="true"]

#ChangeStateSm
    -interface-

    change

    -machine-
    $S0
        |change|
            ->> $S1
            ^

    $S1
        |change|
            ->> $S2
            ^

    $S2
        |change|
            ->> $S3
            ^

    $S3
        |change|
            ->> $S4
            ^

    $S4
        |change|
            ->> $S0
            ^

##

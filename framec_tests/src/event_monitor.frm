#[codegen.rust.features.runtime_support:bool="true"]
#[codegen.rust.runtime.event_history_length:int="5"]
#[codegen.rust.runtime.transition_history_length:int="3"]
#EventMonitorSm
    -interface-
    change : u32
    transit [x:u32]
    mult [a:i32 b:i32] : i32
    reset

    -machine-
    $A
        |<| [a_out:u32] ^

        |change| : u32
            ->> $B ^(2)

        |transit| [x:u32]
            (3) -> (4) $B ^

        |mult| [a:i32 b:i32] : i32
            var out = a * b
            ^(out)

    $B
        |>| [b_in:u32]
            transit (11) ^

        |<| [b_out:u32] ^

        |change| : u32
            ->> $C ^(12)

        |transit| [x:u32]
            (13) -> (14) $C ^

        |mult| [a:i32 b:i32] : i32
            var out = a * b
            ^(out)

        |reset|
            ->> $A ^

    $C
        |>| [c_in:u32]
            transit (21) ^

        |<| [c_out:u32] ^

        |change| : u32
            ->> $D ^(22)

        |transit| [x:u32]
            (23) -> (24) $D ^

        |mult| [a:i32 b:i32] : i32
            var out = a * b
            ^(out)

        |reset|
            ->> $A ^

    $D
        |>| [d_in:u32]
            change () ^

        |<| [d_out:u32] ^

        |change| : u32
            ->> $A ^(32)

        |transit| [x:u32]
            (33) -> $A ^

        |mult| [a:i32 b:i32] : i32
            var out = a * b
            ^(out)

        |reset|
            ->> $A ^

    -actions-
    -domain-
##

#TransitParams
    -interface-
    Next
    
    -machine-
    $Init
        |>|
            -> ("hi A" 1) $A ^

    $A
        |>| [msg:String val:i16]
            entered(msg.clone() val) ^
        |Next|
            -> ("hi B" 2) $B ^

    $B
        |>| [msg:String val:i16]
            entered(msg.clone() val) ^
        |<| [val:bool msg:String]
            exited(val msg.clone()) ^
        |Next|
            (true "bye B") -> ("hi again A" 3) $A ^

    -actions-
    entered [msg:String val:i16]
    exited [val:bool msg:String]

    -domain-
    var enter_log:Log = `vec![]`
    var exit_log:Log = `vec![]`
##

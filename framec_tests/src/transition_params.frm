#TransitParams
    -interface-
    Next

    -machine-
    $Init
        |Next|
            -> ("hi A") $A ^

    $A
        |>| [msg:String]
            log(msg.clone()) ^
        
        |<| 
            log("bye A") ^
        
        |Next|
            -> ("hi B" 42) $B ^

    $B
        |>| [msg:String val:i16]
            log(msg.clone())
            log(val.to_string()) ^
        
        |<| [val:bool msg:String]
            log(val.to_string())
            log(msg.clone()) ^
        
        |Next|
            (true "bye B") -> ("hi again A") $A ^

    -actions-
    log [msg:String]

    -domain-
    var tape:Log = `vec![]`
##

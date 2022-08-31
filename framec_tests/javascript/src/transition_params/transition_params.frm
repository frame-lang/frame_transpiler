#[codegen.javascript.code.public_domain:bool="true"]
#[codegen.javascript.code.public_state_info:bool="true"]
#[codegen.javascript.code.generate_import_export:bool="true"]

#TransitParams
    -interface-
    Next
    Change

    -machine-
    $Init
        |Next|
            -> ("hi A") $A ^
        |Change|
            ->> $A ^

    $A
        |>| [msg:str]
            log(msg) ^

        |<|
            log("bye A") ^

        |Next|
            -> ("hi B" 42) $B ^

        |Change|
            ->> $B ^

    $B
        |>| [msg:str val:int]
            log(msg)
            log(val.toString()) ^

        |<| [val:bool msg:str]
            log(val.toString())
            log(msg) ^

        |Next|
            (true "bye B") -> ("hi again A") $A ^

        |Change|
            ->> $A ^

    -actions-
    log [msg:str]

    -domain-
    var tape = `[]`
##

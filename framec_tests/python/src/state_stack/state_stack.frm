
#[codegen.python.code.public_state_info:bool="true"]

#StateStack
    -interface-
    to_a
    to_b
    to_c
    push
    pop


    -machine-
    $A
        |>|
            log("A:>") ^
        |<|
            log("A:<") ^
        |to_a|
            -> $A ^
        |to_b|
            -> $B ^
        |to_c|
            -> $C ^
        |push|
            $$[+] ^
        |pop|
            -> $$[-] ^

    $B
        |>|
            log("B:>") ^
        |<|
            log("B:<") ^
        |to_a|
            -> $A ^
        |to_b|
            -> $B ^
        |to_c|
            -> $C ^
        |push|
            $$[+] ^
        |pop|
            -> $$[-] ^

    $C
        |>|
            log("C:>") ^
        |<|
            log("C:<") ^
        |to_a|
            -> $A ^
        |to_b|
            -> $B ^
        |to_c|
            -> $C ^
        |push|
            $$[+] ^
        |pop|
            -> $$[-] ^

    -actions-
    log [msg:str]

    -domain-
    var tape = `[]`
##

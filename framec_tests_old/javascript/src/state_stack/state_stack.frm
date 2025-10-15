#[codegen.javascript.code.public_domain:bool="true"]
#[codegen.javascript.code.public_state_info:bool="true"]
#[codegen.javascript.code.generate_import_export:bool="true"]

#StateStack
    -interface-
    to_a
    to_b
    to_c
    push
    pop
    pop_change

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
        |pop_change|
            // ->> $$[-]
            ^

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
        |pop_change|
            // ->> $$[-]
            ^

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
        |pop_change|
            // ->> $$[-]
            ^

    -actions-
    log [msg:string]

    -domain-
    var tape = `[]`
##


#[codegen.python.code.public_state_info:bool="true"]

#StateContextStack
    -interface-
    to_a
    to_b
    to_c
    inc
    value:int
    push
    pop

    -machine-
    $A
        var x:int = 0
        |>|
            log("A:>") ^
        |<|
            log("A:<") ^
        |inc|
            x = x + 1 ^
        |value|:int
            ^(x)
        |to_a|
            -> $A ^
        |to_b|
            -> $B ^
        |to_c|
            -> $C ^
        |push|
            $$[+] ^
        |pop|
            -> $$[-]
            ^

    $B
        var y:int = 0
        |>|
            log("B:>") ^
        |<|
            log("B:<") ^
        |inc|
            y = y + 5 ^
        |value|:int
            ^(y)
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
        var z:int = 0
        |>|
            log("C:>") ^
        |<|
            log("C:<") ^
        |inc|
            z = z + 10 ^
        |value|:int
            ^(z)
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

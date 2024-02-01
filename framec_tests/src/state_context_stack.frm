#[codegen.rust.features.runtime_support:bool="true"]
#StateContextStack
    -interface-
    to_a
    to_b
    to_c
    inc
    value:i32
    push
    pop
    pop_change

    -machine-
    $A
        var x:i32 = 0
        |>|
            log("A:>") ^
        |<|
            log("A:<") ^
        |inc|
            x = x + 1 ^
        |value|:i32
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
            -> $$[-] ^
        |pop_change|
            // ->> $$[-]
            ^

    $B
        var y:i32 = 0
        |>|
            log("B:>") ^
        |<|
            log("B:<") ^
        |inc|
            y = y + 5 ^
        |value|:i32
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
        |pop_change|
            // ->> $$[-]
            ^

    $C
        var z:i32 = 0
        |>|
            log("C:>") ^
        |<|
            log("C:<") ^
        |inc|
            z = z + 10 ^
        |value|:i32
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
        |pop_change|
            // ->> $$[-]
            ^

    -actions-
    log [msg:String]

    -domain-
    var tape:Log = `vec![]`
##

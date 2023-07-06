```
package framec_tests.java.StateContextStack;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
#StateContextStack
    -interface-
    to_a
    to_b
    to_c
    inc
    value:int
    push
    pop
    pop_change

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
            -> $$[-] ^
        |pop_change|
            ->> $$[-] ^

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
        |pop_change|
            ->> $$[-] ^

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
        |pop_change|
            ->> $$[-] ^

    -actions-
    log [msg:String]

    -domain-
    var tape:`ArrayList<String>` = `new ArrayList<String>()`
##

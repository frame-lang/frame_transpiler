```
#include <vector>
#include <any>
#include <stack>
using namespace std;
#include "../FrameLang/FrameLang.h"
```
#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]
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
            ->> $$[-] ^

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
            ->> $$[-] ^

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
            ->> $$[-] ^

    -actions-
    log [msg:`const std::string&`]{`tape.push_back(msg);`}

    -domain-
    var tape:`std::vector<std::string>` =``
##

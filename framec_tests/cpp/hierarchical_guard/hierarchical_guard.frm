```
#include <unordered_map>
#include <stdexcept>
#include <string>
#include <iostream>
#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"
```

#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]

#HierarchicalGuard
    -interface-
    A [i:int]
    B [i:int]

    -machine-
    $I  |>| -> $S ^

    $S
        |A| [i:int]
            log("S.A")
            i < 10 ?
                -> $S0^
            :
                -> $S1
            :: ^

        |B| [i:int]
            log("S.B")
            i < 10 ?
                -> $S2^
            :
                -> $S3
            :: ^

    $S0 => $S
        |A| [i:int]
            log("S0.A")
            i > 0 ?
                -> $S2^
            :            --- fall through else branch
            :: :>

        |B| [i:int]
            log("S0.B")
            i > 0 ?
            :            --- fall through then branch
                -> $S1^
            :: :>

    $S1 => $S0
        |A| [i:int]
            log("S1.A")
            i > 5 ?
                -> $S3^
            :            --- fall through else branch
            :: :>

    $S2 => $S1
        |A| [i:int]
            log("S2.A")
            i > 10 ?
                -> $S4^
            :            --- fall through then branch
            :: :>

        |B| [i:int]
            log("S2.B")
            i > 10 ?!
            :            --- fall through then branch
                -> $S4^
            :: :>

    $S3 => $S
        |A| [i:int]
            log("S3.A")
            i > 0 ?
                log("stop") ^
            :
                log("continue")
            :: :>

        |B| [i:int]
            log("S3.B")
            i > 0 ?
                log("continue")
            :
                log("stop") ^
            :: :>

    $S4

    -actions-
    log [msg:`const std::string&`]{`tape.push_back(msg);`}

    -domain-
    var tape:`std::vector<std::string>` =``
##

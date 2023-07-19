```
#include <vector>
#include <any>
#include <string>
using namespace std;
#include "../FrameLang/FrameLang.h"
```
#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]

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
        |>| [msg:string]
            log(msg) ^

        |<|
            log("bye A") ^

        |Next|
            -> ("hi B" 42) $B ^

        |Change|
            ->> $B ^

    $B
        |>| [msg:string val:int]
            log(msg)
            log(`std::`to_string(val)) ^

        |<| [val:bool msg:string]
            log(`std::`to_string(val))
            log(msg) ^

        |Next|
            (true "bye B") -> ("hi again A") $A ^

        |Change|
            ->> $A ^

    -actions-
    log [msg:`const std::string&`]{`tape.push_back(msg);`}

    -domain-
    var tape:`std::vector<std::string>` =``
##

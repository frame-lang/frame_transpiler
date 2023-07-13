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
#EventHandler
    -interface-
    LogIt [x:int]
    LogAdd [a:int b:int]
    LogReturn [a:int b:int] : int
    PassAdd [a:int b:int]
    PassReturn [a:int b:int] : int

    -machine-
    $S1
        |LogIt| [x:int]
            log("x" x) ^

        |LogAdd| [a:int b:int]
            log("a" a)
            log("b" b)
            log("a+b" a+b) ^

        |LogReturn| [a:int b:int] : int
            log("a" a)
            log("b" b)
            var r:int = a + b
            log("r" r)
            -> ^(r)

        |PassAdd| [a:int b:int]
            -> $S2(a+b) ^

        |PassReturn| [a:int b:int]: int
            var r:int = a + b
            log("r" r)
            -> $S2(r) ^(r)

    $S2 [p:int]

        |>|
            log("p" p) ^

    -actions-
    log [msg:`const std::string&` val:int] {
        `std::string value = msg + "=" + std::to_string(val);
        tape.push_back(value);`
    }

    -domain-
    var tape:`std::vector<std::string>` =``
##

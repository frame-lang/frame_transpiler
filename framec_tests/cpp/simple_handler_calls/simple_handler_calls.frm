```
#include <unordered_map>
#include <vector>
using namespace std;
#include "../FrameLang/FrameLang.h"
```
#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]
#SimpleHandlerCalls
    -interface-
    A
    B
    C
    D
    E

    -machine-
    $Init
        |A| -> $A ^

        |B| -> $B ^

        |C| A() ^

        |D|
            B()
            -> $A ^

        |E|
            D()
            C() ^

    $A
    $B
##

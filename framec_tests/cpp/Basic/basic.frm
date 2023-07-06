```
#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>
#include "../FrameLang/FrameEvent.h"
using namespace std;
```
#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]
#Basic
    -interface-
    A
    B
    -machine-

    $S0
        |>| entered("S0") ^
        |<| left("S0") ^
        |A| -> "ooh" $S1 ^

    $S1
        |>| entered("S1") ^
        |<| left("S1") ^
        |B| -> "aah" $S0 ^

    -actions-
    entered[msg:string] {`entry_log.push_back(msg);`}
    left[msg:string] {`exit_log.push_back(msg);`}

    -domain-
    
    var entry_log:`vector<string>` = `{}`
    var exit_log:`vector<string>` = `{}`
##

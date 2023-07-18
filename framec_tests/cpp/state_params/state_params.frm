```
#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"
```
#[codegen.cpp.code.public_domain:bool="true"]
#[codegen.cpp.code.public_state_info:bool="true"]
#[codegen.cpp.code.generate_import_export:bool="true"]
#StateParams
    -interface-
    Next
    Prev
    Log

    -machine-
    $Init
        |Next| -> $Split(1) ^

    $Split [val:int]
        |Next| -> $Merge(val val+1) ^
        |Prev| -> $Merge(val+1 val) ^
        |Log| got_param("val" val) ^

    $Merge [left:int right:int]
        |Next| -> $Split(left+right) ^
        |Prev| -> $Split(left*right) ^
        |Log|
            got_param("left" left)
            got_param("right" right)
            ^

    -actions-
    got_param [name:`const std::string&` val:int]{`param_log.push_back(name + "=" + std::to_string(val));`}

    -domain-
    var param_log:`std::vector<std::string>` =``
##

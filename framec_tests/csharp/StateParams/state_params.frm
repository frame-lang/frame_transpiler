```
using FrameLang;
#nullable disable
namespace StateParams
```
#[codegen.csharp.code.public_domain:bool="true"]
#[codegen.csharp.code.public_state_info:bool="true"]
#[codegen.csharp.code.generate_import_export:bool="true"]
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
    got_param [name:string val:int] {`this.param_log.Add(name+"="+(val).ToString());`}

    -domain-
    var param_log: `List<string>` = `new List<string>()`
##

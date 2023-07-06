```
using FrameLang;
#nullable disable
namespace Basic
```
#[codegen.csharp.code.public_domain:bool="true"]
#[codegen.csharp.code.public_state_info:bool="true"]
#[codegen.csharp.code.generate_import_export:bool="true"]
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
    entered[msg:string] {`entry_log.Add(msg);`}
    left[msg:string] {`exit_log.Add(msg);`}

    -domain-
    var entry_log:`List<string>` = `new List<string>()`
    var exit_log:`List<string>` = `new List<string>()`
##

```
using FrameLang;
#nullable disable
namespace Transition
```
#[codegen.csharp.code.public_domain:bool="true"]
#[codegen.csharp.code.public_state_info:bool="true"]
#[codegen.csharp.code.generate_import_export:bool="true"]

#TransitionSm
    -interface-
    transit
    change

    -machine-
    $S0
        |>| enter("S0") ^
        |<| exit("S0") ^
        |transit|
            -> $S1 ^
        |change|
            ->> $S1 ^

    $S1
        |>| enter("S1") ^
        |<| exit("S1") ^
        |transit|
            -> $S2 ^
        |change|
            ->> $S2 ^

    $S2
        |>| enter("S2")
            -> $S3 ^
        |<| exit("S2") ^
        |transit|
            -> $S3 ^
        |change|
            ->> $S3 ^

    $S3
        |>| enter("S3") ^
        |<| exit("S3") ^
        |transit|
            -> $S4 ^
        |change|
            ->> $S4 ^

    $S4
        |>| enter("S4")
            ->> $S0 ^
        |<| exit("S4") ^

    -actions-
    enter [state:string] {`this.enters.Add(state);`}
    exit [state:string] {`this.exits.Add(state);`}

    -domain-
    var enters:`List<string>` = `new List<string>()`
    var exits:`List<string>` = `new List<string>()`
##

```
using FrameLang;
#nullable disable
namespace EventHandler
```
#[codegen.csharp.code.public_domain:bool="true"]
#[codegen.csharp.code.public_state_info:bool="true"]
#[codegen.csharp.code.generate_import_export:bool="true"]
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
    log [msg:string val:int] {`
        string value = msg + "=" + val.ToString(); 
        this.tape.Add(value);`
        }

    -domain-
    var tape:`List<string>` = `new List<string>()`
##
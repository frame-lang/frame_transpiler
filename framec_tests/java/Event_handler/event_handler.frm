```
package framec_tests.java.Event_handler;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
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
    log [msg:String val:int]

    -domain-
    var tape:`ArrayList<String>` = `new ArrayList<String>()`
##

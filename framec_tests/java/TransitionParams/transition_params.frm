```
package framec_tests.java.TransitionParams;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]

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
        |>| [msg:String]
            log(msg) ^

        |<|
            log("bye A") ^

        |Next|
            -> ("hi B" 42) $B ^

        |Change|
            ->> $B ^

    $B
        |>| [msg:String val:int]
            log(msg)
            log(String.valueOf(val)) ^

        |<| [val:Boolean msg:String]
            log(val.toString())
            log(msg) ^

        |Next|
            (true "bye B") -> ("hi again A") $A ^

        |Change|
            ->> $A ^

    -actions-
    log [msg:String]

    -domain-
    var tape:`ArrayList<String>` = `new ArrayList<String>()`
##

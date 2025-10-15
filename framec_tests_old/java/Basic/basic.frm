```
package framec_tests.java.Basic;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```

#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
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
    entered[msg:String]
    left[msg:String] 

    -domain-
    var entry_log:`ArrayList<String>` = `new ArrayList<String>()`
    var exit_log:`ArrayList<String>` = `new ArrayList<String>()`
##
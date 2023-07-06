```
package framec_tests.java.StateVars;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.public_compartment:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
#StateVars
    -interface-
    X
    Y
    Z

    -machine-
    $Init
        |>| -> $A ^

    $A
        var x:int = 0
        |X| x = x + 1 ^
        |Y| -> $B ^
        |Z| -> $B ^

    $B
        var y:int = 10
        var z:int = 100
        |X| -> $A ^
        |Y| y = y + 1 ^
        |Z| z = z + 1 ^

    -actions-

    -domain-
##

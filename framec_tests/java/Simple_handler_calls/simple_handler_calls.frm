```
package framec_tests.java.Simple_handler_calls;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
#SimpleHandlerCalls
    -interface-
    A
    B
    C
    D
    E

    -machine-
    $Init
        |A| -> $A ^

        |B| -> $B ^

        |C| A() ^

        |D|
            B()
            -> $A ^

        |E|
            D()
            C() ^

    $A
    $B
##

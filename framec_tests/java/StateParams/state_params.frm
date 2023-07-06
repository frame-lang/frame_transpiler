```
package framec_tests.java.StateParams;
import java.util.*;
import framec_tests.java.FrameLang.FrameEvent;
```
#[codegen.java.code.public_domain:bool="true"]
#[codegen.java.code.public_state_info:bool="true"]
#[codegen.java.code.generate_import_export:bool="true"]
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
    got_param [name:String val:int]

    -domain-
    var param_log:`ArrayList<String>` = `new ArrayList<String>()`
##

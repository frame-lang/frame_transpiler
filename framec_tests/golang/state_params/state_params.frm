```
package state_params

import "golang/framelang"

```
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
    got_param [name:string val:int]

    -domain-
    var param_log: `[]string` = `[]string{}`
##

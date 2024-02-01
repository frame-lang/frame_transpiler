```
from framelang.framelang import FrameEvent
from enum import Enum
```
#[codegen.python.code.public_state_info:bool="true"]
#EnumTest

    -interface-
    A
    B
    -machine-

    $SUN
        |A| entered("SUNDAY", Days.SUNDAY) ^
        |B| left("MONDAY", Days.MONDAY) 
           -> $MON ^

    $MON
        |A| entered("MONDAY", Days.MONDAY) ^
        |B| left("TUESDAY", Days.TUESDAY) 
           -> $TUE ^

    $TUE
        |A| entered("TUESDAY", Days.TUESDAY) ^
        |B| left("WEDNESDAY", Days.WEDNESDAY) 
           -> $WED ^

    $WED
        |A| entered("WEDNESDAY", Days.WEDNESDAY) ^
        |B| left("THURSDAY", Days.THURSDAY) 
           -> $THR ^

    $THR
        |A| entered("THURSDAY", Days.THURSDAY) ^
        |B| left("FRIDAY", Days.FRIDAY) 
           -> $FRI ^

    $FRI
        |A| entered("FRIDAY", Days.FRIDAY) ^
        |B| left("SUNDAY", Days.SUNDAY) 
           -> $SUN ^
    
    -actions-

    entered[msg:str, val:int] 
    
    left[msg:str, val:int]

    -domain-

    var days = `[]`
    enum Days {
        SUNDAY 
        MONDAY
        TUESDAY 
        WEDNESDAY 
        THURSDAY
        FRIDAY 
    }

##
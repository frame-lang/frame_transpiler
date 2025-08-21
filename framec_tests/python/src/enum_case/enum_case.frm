```
from enum import Enum
```

system EnumTest {
    interface:
        A()
        B()

    machine:
        $SUN {
            A() {
                entered("SUNDAY", Days.SUNDAY)
                return
            }
            B() {
                left("MONDAY", Days.MONDAY)
                -> $MON
                return
            }
        }

        $MON {
            A() {
                entered("MONDAY", Days.MONDAY)
                return
            }
            B() {
                left("TUESDAY", Days.TUESDAY)
                -> $TUE
                return
            }
        }

        $TUE {
            A() {
                entered("TUESDAY", Days.TUESDAY)
                return
            }
            B() {
                left("WEDNESDAY", Days.WEDNESDAY)
                -> $WED
                return
            }
        }

        $WED {
            A() {
                entered("WEDNESDAY", Days.WEDNESDAY)
                return
            }
            B() {
                left("THURSDAY", Days.THURSDAY)
                -> $THR
                return
            }
        }

        $THR {
            A() {
                entered("THURSDAY", Days.THURSDAY)
                return
            }
            B() {
                left("FRIDAY", Days.FRIDAY)
                -> $FRI
                return
            }
        }

        $FRI {
            A() {
                entered("FRIDAY", Days.FRIDAY)
                return
            }
            B() {
                left("SUNDAY", Days.SUNDAY)
                -> $SUN
                return
            }
        }
    
    actions:
        entered(msg:str, val:int) {
        }
        
        left(msg:str, val:int) {
        }

    domain:
        var days = `[]`
        enum Days {
            SUNDAY 
            MONDAY
            TUESDAY 
            WEDNESDAY 
            THURSDAY
            FRIDAY 
        }
}
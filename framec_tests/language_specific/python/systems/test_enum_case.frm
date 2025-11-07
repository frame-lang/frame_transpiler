# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

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
            }
        }
    
    actions:
        entered(msg:str, val:int) {
        }
        
        left(msg:str, val:int) {
        }

    domain:
        days = []
        enum Days {
            SUNDAY 
            MONDAY
            TUESDAY 
            WEDNESDAY 
            THURSDAY
            FRIDAY 
        }
}
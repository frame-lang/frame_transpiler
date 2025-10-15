# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Calculator system for main example

import MathUtils from "./test_multifile_utils.frm"

system Calculator {
    interface:
        add(a, b) : str
        multiply(a, b) : str
        getCircleArea(r) : str
        checkEven(n) : bool
    
    machine:
        $Ready {
            add(a, b) {
                var result = MathUtils::add(a, b)
                system.return = str(result)
                return
            }
            
            multiply(a, b) {
                var result = MathUtils::multiply(a, b)
                system.return = str(result)
                return
            }
            
            getCircleArea(r) {
                var area = MathUtils::circleArea(r)
                system.return = str(area)
                return
            }
            
            checkEven(n) {
                system.return = MathUtils::isEven(n)
                return
            }
        }
}
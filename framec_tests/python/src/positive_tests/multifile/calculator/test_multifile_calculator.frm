# Multi-file test: Calculator system
# This module imports utilities and implements a calculator

import MathUtils from "./test_multifile_utils.frm"
import { formatNumber, isEven } from "./test_multifile_utils.frm"

system Calculator {
    interface:
        add(a, b) : str
        multiply(a, b) : str
        getCircleArea(r) : str
        checkEven(n) : bool
    
    machine:
        $Ready {
            add(a, b) {
                var result = MathUtils.add(a, b)
                system.return = formatNumber(result)
                return
            }
            
            multiply(a, b) {
                var result = MathUtils.multiply(a, b)
                system.return = formatNumber(result)
                return
            }
            
            getCircleArea(r) {
                var area = MathUtils.circleArea(r)
                system.return = formatNumber(area)
                return
            }
            
            checkEven(n) {
                system.return = isEven(n)
                return
            }
        }
}
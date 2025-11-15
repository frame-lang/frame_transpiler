@target python
# @compile-expect: def _operation_calc\(

system S {
    machine:
        $A {
            e() {
                self._operation_calc(1, 2)
            }
        }
    operations:
        calc(x, y) { pass }
}


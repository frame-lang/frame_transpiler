@target python
# @compile-expect: def _operation_calc\(
# @py-compile

system S {
    operations:
        calc(x, y) { pass }
    machine:
        $A {
            e() {
                self._operation_calc(1, 2)
            }
        }
}

@target python_3

# V3 capability fixture: header type/default segment (`: Type = default`).
# This exercises `type_and_default` on interface/actions/operations headers.

system TypeAndDefaultDemo {
    operations:
        @native
        helper(x): Result = None {
            return x
        }

    interface:
        compute(x, y): Result = None

    machine:
        $A {
            e() {
                # Call action and operation to ensure headers are wired.
                self.log("ok")
                self._operation_helper(1)
            }
        }

    actions:
        log(message): Result = None {
            print(message)
        }
}

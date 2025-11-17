@target python_3
# @expect: E407

# Negative: system.return is not allowed in operations.

system BadSystemReturn {
    operations:
        op() {
            system.return = "bad"
        }

    interface:
        status()

    machine:
        $A {
            status() {
                system.return = "ok"
            }
        }
}


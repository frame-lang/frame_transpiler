@target typescript
# @expect: E407

# Negative: system.return is not allowed in operations (TypeScript).

system BadSystemReturnTs {
    operations:
        op() {
            system.return = "bad";
        }

    interface:
        status()

    machine:
        $A {
            status() {
                system.return = "ok";
            }
        }
}


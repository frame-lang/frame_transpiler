@target python

system PyFStringNested {
    operations:
        op() {
            a = 1
            name = "world"
            s1 = f"hello {name.upper()}"
            s2 = f"tokens -> $Next and $$[+] and => $^ inside fstring"
            s3 = f"nested {f\"level2 {a}\"} end"
        }
    machine:
        $Init { op() { } }
        $Next {}
}

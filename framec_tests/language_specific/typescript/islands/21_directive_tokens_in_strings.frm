@target typescript

system DirectiveTokensInStringsTS {
    operations:
    op1() {
        const a = "string mentions -> $Next and $$[-]";
        const b = `template mentions => $^ and -> $Other`;
        const c = 'single quotes $$[+] here';
        -> $Next
    }
    machine:
        $Init { op1() { } }
        $Next {}
}


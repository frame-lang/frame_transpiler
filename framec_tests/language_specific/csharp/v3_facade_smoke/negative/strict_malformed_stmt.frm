@target csharp

system S {
    machine:
        $A {
            e() {
                => $^
                int x = ; // malformed native statement
            }
        }
}


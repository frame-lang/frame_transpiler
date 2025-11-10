@target csharp

system S {
    machine:
        $A {
            e() {
                => $^ /* inline block ok */
            }
        }
}


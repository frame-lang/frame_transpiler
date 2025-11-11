@target csharp

system S {
    actions:
        fn bad() {
            -> $B()
        }
}

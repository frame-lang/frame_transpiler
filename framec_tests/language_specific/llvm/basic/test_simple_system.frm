# Phase 1 LLVM backend smoke test

system Simple {
    interface:
        greet()

    machine:
        $Start {
            greet() {
                print("Hello from LLVM backend 2")
            }
        }
}

fn main() {
    var simple = Simple()
    simple.greet()
    return
}

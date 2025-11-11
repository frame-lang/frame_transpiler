@target cpp

system S {
    machine:
        $A {
            e() {
                -> $B();
                std::vector<int x; // missing closing '>'
            }
        }
        $B {
        }
}


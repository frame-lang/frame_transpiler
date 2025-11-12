@target cpp

system S {
    machine:
        $A {
            e() {
                auto s = R"(raw with -> $B() and => $^)";
                native();
            }
        }
}


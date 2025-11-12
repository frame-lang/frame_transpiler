@target java

system S {
    machine:
        $A {
            e() {
                String t = """
                    text block with -> $B() and => $^
                    still inside text block
                """;
                native();
            }
        }
}


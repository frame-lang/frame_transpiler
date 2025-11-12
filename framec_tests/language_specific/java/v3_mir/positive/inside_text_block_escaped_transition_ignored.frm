@target java

system S {
    machine:
        $A {
            e() {
                String t = """
                    text block with escaped like \" -> $B() and => $^ \"
                    still inside text block
                """;
                native();
            }
        }
}


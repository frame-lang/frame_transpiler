@target typescript

system S {
    machine:
        $A {
            e() {
                try {
                    => $^; tryBlock();
                } catch (e) {
                    => $^; handle(e);
                } finally {
                    => $^; cleanup();
                }
            }
        }
}


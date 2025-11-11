@target typescript

system S {
    machine:
        $A => $P {
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
        $P { }
}

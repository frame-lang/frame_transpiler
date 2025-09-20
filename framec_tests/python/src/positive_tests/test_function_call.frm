# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn logTransition(fromState, toState) {
    print("Transition: " + fromState + " -> " + toState)
}

system TestSystem {
    machine:
        $StateA {
            $>() {
                logTransition("", "StateA")
            }
        }
}
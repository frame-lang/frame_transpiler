fn main() {
    var sys = Simple()
    // Trigger enter event
    sys._sStart(FrameEvent("$>", []))
}

system Simple {
    machine:
        $Start {
            $>() {
                print("Hello from Simple system!")
            }
        }
}
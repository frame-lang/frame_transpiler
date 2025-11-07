# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = Simple()
    # Trigger enter event
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
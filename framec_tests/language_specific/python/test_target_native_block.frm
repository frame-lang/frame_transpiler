@target python
system NativePython {
    interface:
        start()

    machine:
        $Init {
            start() {
                print("native python block start")
                for idx in range(2):
                    print(f"idx = {idx}")
                self.log("native complete")
                return
            }
        }
    actions:
        log(message) {
            print("LOG: " + message)
            return
        }
}

fn helper() {
    print("helper native")
    return 42
}

fn main() {
    sys = NativePython()
    sys.start()
}

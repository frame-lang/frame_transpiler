system NativePython {
    interface:
        start()

    machine:
        $Init {
            start() {
                #[target: python]
                print("native python block start")
                for idx in range(2):
                    print(f"idx = {idx}")
                self.log("native complete")
                return
            }
        }

    actions:
        log(message) {
            #[target: python]
            print("LOG: " + message)
            return
        }
    }

fn helper() {
    #[target: python]
    print("helper native")
    return 42
}

fn main() {
    var sys = NativePython()
    sys.start()
}

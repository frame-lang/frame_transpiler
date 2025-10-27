# Validates parent event forwarding (=> $^) in the LLVM backend.

system ParentForward {
    interface:
        test()

    machine:
        $Child => $Parent {
            test() {
                print("child handling")
                => $^
            }
        }

        $Parent {
            test() {
                print("parent handling")
            }
        }
}

fn main() {
    var sys = ParentForward()
    sys.test()
}

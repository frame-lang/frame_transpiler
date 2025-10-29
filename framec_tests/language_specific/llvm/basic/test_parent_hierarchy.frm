# Multi-level parent dispatch with typed and inferred domain fields.

system ParentHierarchy {
    interface:
        trigger()

    machine:
        $Grandchild => $Child {
            trigger() {
                print("grandchild: start")
                note = "updated by grandchild"
                => $^
                print("grandchild: done")
            }
        }

        $Child => $Parent {
            trigger() {
                print("child: before parent")
                => $^
                print("child: after parent")
                print(note)
            }
        }

        $Parent {
            trigger() {
                print("parent: handling")
                counter = 4
                status = "parent updated"
                print(status)
            }
        }

    domain:
        var counter: int = 3
        var status: string = "idle"
        var note = "unset"
}

fn main() {
    var hierarchy = ParentHierarchy()
    hierarchy.trigger()
}

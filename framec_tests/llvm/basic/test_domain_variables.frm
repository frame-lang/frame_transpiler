# Domain variables exercise for the LLVM backend

system DomainExample {
    interface:
        show()

    machine:
        $Start {
            show() {
                print(label)
            }
        }

    domain:
        var count: int = 42
        var label: string = "Domain example ready"
        var enabled: bool = true
}

fn main() {
    var example = DomainExample()
    example.show()
}

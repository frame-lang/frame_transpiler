fn main() {
    var sys = TestReturnAssign()
    print(sys.getValue())
}

system TestReturnAssign {
    interface:
        getValue() ^("default")

    machine:
        $Start {
            getValue() {
                return = "modified"
                return
            }
        }
}
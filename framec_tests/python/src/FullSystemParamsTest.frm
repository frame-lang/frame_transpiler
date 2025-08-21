fn main() {
    var sys = SystemInitializationDemo("a", "b", "c", "d", "e", "f")
}

system SystemInitializationDemo ($(A,B), $>(C,D), E, F) {
    machine:
        $Start(A,B) {
            $>(C,D) {
                print(A + B + C + D + E + F)
                return
            }
        }
    
    domain:
        var E = nil
        var F = nil 
}
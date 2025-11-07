# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = SystemInitializationDemo("a", "b", "c", "d", "e", "f")
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
        E = None
        F = None 
}
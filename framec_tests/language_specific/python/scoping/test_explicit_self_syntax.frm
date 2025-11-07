# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    sys = TestSystem()
    sys.process()
}

system TestSystem {
    interface:
        process()
        
    machine:
        $Ready {
            process() {
                self.doWork()
                return
            }
        }
        
    actions:
        doWork() {
            print("Working with total: " + str(self.total))
        }
        
    domain:
        total: int = 0
}
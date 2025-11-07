# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    worker = Worker()
    worker.start()
}

system Worker {
    interface:
        start()
        
    machine:
        $Ready {
            start() {
                self.doWork()
                return
            }
        }
        
    actions:
        doWork() {
            print("Working with underscore prefix...")
        }
}
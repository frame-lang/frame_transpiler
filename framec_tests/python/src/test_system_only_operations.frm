system TestOperations {
    operations:
        run() {
            print("running")
            run_internal()
        }
        
        run_internal() {
            print("internal")
        }
    
    machine:
        $Start {
        }
}
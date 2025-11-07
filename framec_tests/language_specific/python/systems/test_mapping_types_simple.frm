system TestMappingTypes {
    interface:
        start()
    
    machine:
        $Init {
            start() {
                print("Hello World")
                var x = 42
                doAction()
            }
        }
    
    actions:
        doAction() {
            print("Action called")
        }
    
    domain:
        counter = 0
}
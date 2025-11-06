@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Minimal test for async interface methods
system AsyncInterface {
    interface:
        async getData(id)
        
    machine:
        $Ready {
            getData(id) {
                print("Getting data")
                return "data"
            }
        }
}
}

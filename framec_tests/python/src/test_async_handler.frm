// Test async interface methods with proper event handlers
from fsl import str

system AsyncHandler {
    interface:
        async getData(id)
        normalMethod(x)
        
    machine:
        $Ready {
            getData(id) {
                print("Getting data for id: " + str(id))
                // In real code, this would await an async operation
                var result = "data_" + str(id)
                return result
            }
            
            normalMethod(x) {
                print("Normal method: " + str(x))
                return x * 2
            }
        }
}
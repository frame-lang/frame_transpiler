// test_dict_in_system.frm
// Test dictionary literals in systems


system DictSystem {
    interface:
        configure(config)
        getSettings()
    
    machine:
        $Ready {
            configure(config) {
                // Use the config dict parameter
                print("Received config: " + str(config))
                
                // Create and return a new dict
                var result = {"status": "ok", "data": config}
                return = result
            }
            
            getSettings() {
                // Return a dictionary literal
                return = {"mode": "test", "version": "1.0"}
            }
        }
}

fn main() {
    print("Testing dictionary in system...")
    var sys = DictSystem()
    
    // Pass dictionary literal to interface method
    var response = sys.configure({"mode": "production", "debug": false})
    print("Configure response: " + str(response))
    
    // Get dictionary from interface method
    var settings = sys.getSettings()
    print("Settings: " + str(settings))
    
    print("System dictionary tests completed!")
    return
}
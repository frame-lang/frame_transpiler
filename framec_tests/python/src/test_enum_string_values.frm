// Test string enum values

fn main() {
    var config = ConfigManager()
    config.testStringEnums()
}

system ConfigManager {
    interface:
        testStringEnums()
    
    machine:
        $Ready {
            testStringEnums() {
                // Test explicit string values
                var red = Color.Red
                var green = Color.Green
                var blue = Color.Blue
                
                print("Red color: " + red.value)
                print("Green color: " + green.value)
                print("Blue color: " + blue.value)
                
                if red.value == "red" {
                    print("Red value is correctly 'red'")
                }
                
                // Test auto string values (uses member name)
                var debug = LogLevel.Debug
                var info = LogLevel.Info
                var error = LogLevel.Error
                
                print("Debug level: " + debug.value)
                print("Info level: " + info.value)
                print("Error level: " + error.value)
                
                if debug.value == "Debug" {
                    print("Debug auto-value is correctly 'Debug'")
                }
                
                // Test database states
                var disconnected = DbState.Disconnected
                var connected = DbState.Connected
                
                print("Disconnected state: " + disconnected.value)
                print("Connected state: " + connected.value)
                
                return
            }
        }
    
    domain:
        // String enum with explicit values
        enum Color : string {
            Red = "red"
            Green = "green"
            Blue = "blue"
        }
        
        // String enum with auto values (uses member name)
        enum LogLevel : string {
            Debug    // "Debug"
            Info     // "Info"
            Warning  // "Warning"
            Error    // "Error"
            Critical // "Critical"
        }
        
        // Mixed explicit and auto string values
        enum DbState : string {
            Disconnected = "disconnected"
            Connecting   // "Connecting" (auto)
            Connected = "connected"
            Error = "error"
            Retrying     // "Retrying" (auto)
        }
}
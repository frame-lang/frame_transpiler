# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

# Simple async validation test for Frame v0.37
import asyncio

# Basic async function
async fn fetch_data(id) {
    print("Fetching data for id: " + str(id))
    # Removed backticks - await asyncio.sleep(0.1)
    return "data_" + str(id)
}

# Test system with async interface
system AsyncTest {
    interface:
        async getData(id)
        setStatus(status)
        
    machine:
        $Ready {
            async getData(id) {
                print("Getting data for: " + str(id))
                data = await fetch_data(id)
                print("Received: " + data)
                self.last_data = data
                system.return = data
            }
            setStatus(status) {
                print("Setting status: " + status)
                self.status = status
            }
        }
        }
        domain:
        last_data = ""
        status = "ready"
}

# Main async test function
async fn run_test() {
    print("=== Async Validation Test ===")
    
    # Test basic async function
    result = await fetch_data(100)
    print("Direct call result: " + result)
    
    # Test async state machine
    test = AsyncTest()
    
    # Call async interface method
    data = await test.getData(42)
    print("Interface call result: " + data)
    
    # Call sync interface method
    test.setStatus("complete")
    
    print("=== Test Complete ===")
}

fn main() {
    print("Starting Frame v0.37 Async Validation")
    # Removed backticks - asyncio.run(run_test())
}

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

# Proof of Concept: Event-handlers-as-functions architecture
# This demonstrates the proposed v0.36 architecture where each event
# handler becomes an independent function with fine-grained async control

# Mock async function
async fn fetch_data(id) {
    print("Fetching data for id: " + str(id))
    return "data_" + str(id)
}

# Mock sync function  
fn process_sync(data) {
    print("Sync processing: " + data)
    return "processed_" + data
}

# Mock async processing
async fn process_async(data) {
    print("Async processing: " + data)
    var extra = await fetch_data(999)
    return "async_" + data + "_with_" + extra
}

# System demonstrating mixed sync/async handlers in same state
system MixedHandlerDemo {
    interface:
        # Sync interface method
        getData(id)
        
        # Async interface methods
        async fetchRemote(id)
        async processData(data)
        
        # Another sync method
        getStatus()
        
    machine:
        $Ready {
            # Sync event handler - should stay sync
            getData(id) {
                print("Sync handler: getData")
                self.lastId = id
                self.data = "cached_" + str(id)
                system.return = self.data
            }
            
            # Async event handler - needs async due to await
            async fetchRemote(id) {
                print("Async handler: fetchRemote")
                data = await fetch_data(id)
                self.data = data
                -> $Processing
            }
            
            # Another async handler with await
            async processData(data) {
                print("Async handler: processData")
                var result = await process_async(data)
                system.return = result
            }
            
            # Sync handler without await
            getStatus() {
                print("Sync handler: getStatus")
                system.return = "ready"
            }
        }
        
        $Processing {
            # Async enter event with await
            async $>() {
                print("Processing state entered")
                processed = await process_async(self.data)
                print("Processing complete: " + processed)
                -> $Ready
            }
            
            # Sync handler in Processing state
            getStatus() {
                print("Sync handler in Processing: getStatus")
                system.return = "processing"
            }
            # No-op handlers to satisfy interface coverage in this state
            getData(id) { return }
            async fetchRemote(id) { return }
            async processData(data) { return }
        }
        
    domain:
        lastId = 0
        data = ""
}

# Test function to demonstrate mixed sync/async usage
# Note: All interface methods become async when any are async
async fn test_mixed_handlers() {
    print("=== Testing Mixed Handler Demo ===")
    var demo = MixedHandlerDemo()
    
    # Since system has async methods, all interface methods are async
    await demo.async_start()  # Initialize async system
    
    # Call getData (async due to system having async methods)
    var data = await demo.getData(123)
    print("Got sync data: " + data)
    
    # Get status (also async)
    var status = await demo.getStatus()
    print("Status: " + status)
    
    print("=== Test Complete ===")
}

# Async test function
async fn test_async_handlers() {
    print("=== Testing Async Handlers ===")
    var demo = MixedHandlerDemo()
    await demo.async_start()  # Initialize async system
    
    # Call async method
    var remote = await demo.fetchRemote(456)
    print("Got remote data: " + str(remote))
    
    # Process data async
    var result = await demo.processData("test_data")
    print("Processed result: " + result)
    
    # Get status after async operations (also async)
    var status = await demo.getStatus()
    print("Final status: " + status)
    
    print("=== Async Test Complete ===")
}

# Main entry point - needs to run async functions
async fn main() {
    print("Event-Handlers-as-Functions PoC")
    # Run async test functions
    await test_mixed_handlers()
    await test_async_handlers()
}

# Call main to run the tests
import asyncio
asyncio.run(main())

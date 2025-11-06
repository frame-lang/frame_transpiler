@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

# Test basic async/await functionality in Frame v0.35
# This tests async functions, async operations, and async interface methods

# Mock async function to simulate network call
async fn fetch_data(url) {
    # In real code, this would be an actual async call
    print("Fetching from " + url)
    return "data from " + url
}

# Async module-level function
async fn process_data(data) {
    print("Processing: " + data)
    result = await fetch_data("api.example.com/process")
    return "processed " + data + " with " + result
}

# System with async interface methods
system AsyncService {
    operations:
        async fetchRemote(endpoint) {
            print("Fetching from endpoint: " + endpoint)
            response = await fetch_data(endpoint)
            return response
        }
        
    interface:
        async getData(id)
        async processItem(item)
        
    machine:
        $Ready {
            async getData(id) {
                print("Getting data for id: " + str(id))
                data = await fetch_data("api.example.com/item/" + str(id))
                print("Received: " + data)
                # Store data in domain for processing state
                self.lastData = data
                -> $Processing
            }
            async processItem(item) {
                result = await process_data(item)
                print("Result: " + result)
                return result
            }
        }
        $Processing {
            async $>() {
                print("Now processing: " + self.lastData)
                processed = await process_data(self.lastData)
                print("Processing complete: " + processed)
                -> $Ready
            }
        }
        
    actions:
        logMessage(msg) {
            print("LOG: " + msg)
        
    domain:
        lastData = None
}

# Regular function that can't use await
fn main() {
    print("Starting async test")
    service = AsyncService()
    # Can't await here since main is not async
    # But we can call the async methods - they'll return coroutines
    service.getData(123)
    service.processItem("test item")
    print("Async test complete")
}

# Async entry point
async fn async_main() {
    print("Starting async main")
    service = AsyncService()
    
    # Here we can properly await
    data = await service.getData(456)
    print("Got data: " + data)
    
    result = await service.processItem("async item")
    print("Processed: " + result)
    
    # Call async operation
    remote = await service.fetchRemote("api.example.com/remote")
    print("Remote data: " + remote)
    
    print("Async main complete")
}

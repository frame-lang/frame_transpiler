# Frame Async Capability Module
# Universal async/await functionality across all target languages
# Follows Frame v0.35-v0.37 async grammar specification

module AsyncCapabilities {
    # Universal HTTP GET operation
    async fn httpGet(url) {
        # Python: await aiohttp.ClientSession().get(url) 
        # TypeScript: await FrameAsync.httpGet(url)
        # C#: await httpClient.GetAsync(url)
        print("HTTP GET: " + url)
        
        # Simulate async operation with await
        await sleep(10)  # Small delay to simulate network
        
        var response = {
            "status": 200,
            "body": "simulated response from " + url,
            "headers": {}
        }
        return response
    }
    
    # Universal HTTP POST operation  
    async fn httpPost(url, data) {
        print("HTTP POST: " + url + " with data: " + str(data))
        
        await sleep(15)  # Simulate network delay
        
        var response = {
            "status": 201, 
            "body": "posted to " + url,
            "headers": {}
        }
        return response
    }
    
    # Universal sleep/delay operation
    async fn sleep(milliseconds) {
        # Python: await asyncio.sleep(milliseconds/1000)
        # TypeScript: await new Promise(r => setTimeout(r, milliseconds))
        # C#: await Task.Delay(milliseconds)
        print("Sleeping for " + str(milliseconds) + " milliseconds")
    }
    
    # Execute operations in parallel
    async fn parallel(tasks) {
        # Python: await asyncio.gather(*tasks)
        # TypeScript: await Promise.all(tasks)
        # C#: await Task.WhenAll(tasks)
        print("Executing " + str(len(tasks)) + " tasks in parallel")
        
        var results = []
        for task in tasks {
            var result = await task  # Each task should be awaitable
            results.append(result)
        }
        return results
    }
    
    # Execute operations in sequence
    async fn sequence(tasks) {
        print("Executing " + str(len(tasks)) + " tasks in sequence")
        
        var results = []
        for task in tasks {
            var result = await task
            results.append(result)
        }
        return results
    }
    
    # Create an async task wrapper
    fn createTask(taskFunc) {
        # Return the function to be called later with await
        return taskFunc
    }
    
    # Wait for all tasks to complete (alias for parallel)
    async fn waitAll(tasks) {
        return await parallel(tasks)
    }
}

# Test system demonstrating async capabilities
system AsyncTester {
    interface:
        async runTest()
        async fetchAndProcess(url)
    
    machine:
        $Ready {
            async runTest() {
                print("Starting async test...")
                
                # Test basic async operations
                await AsyncCapabilities.sleep(100)
                print("Basic sleep completed")
                
                # Test HTTP operations
                var response = await AsyncCapabilities.httpGet("https://api.example.com/data")
                print("HTTP GET response: " + str(response["status"]))
                
                # Test parallel operations
                var tasks = [
                    AsyncCapabilities.sleep(50),
                    AsyncCapabilities.sleep(75), 
                    AsyncCapabilities.sleep(25)
                ]
                
                await AsyncCapabilities.parallel(tasks)
                print("Parallel operations completed")
                
                print("Async test completed successfully!")
                system.return = "test_passed"
            }
            
            async fetchAndProcess(url) {
                try {
                    var data = await AsyncCapabilities.httpGet(url)
                    var processed = await AsyncCapabilities.httpPost("/process", data)
                    system.return = processed
                } except Exception as e {
                    print("Error in fetch and process: " + str(e))
                    system.return = None
                }
            }
        }
}

# Main execution function
async fn main() {
    var tester = AsyncTester()
    var result = await tester.runTest()
    print("Test result: " + str(result))
    
    var processResult = await tester.fetchAndProcess("https://data.example.com")
    print("Process result: " + str(processResult))
}
from fsl import str, int, float, list

// Simplified async stress test for Frame v0.37
// Tests core async functionality without complex backtick blocks

import asyncio
import time

// Simple async functions
async fn async_work(work_id, delay) {
    print("Starting work " + str(work_id))
    `await asyncio.sleep(delay)`
    print("Completed work " + str(work_id))
    return "Result_" + str(work_id)
}

async fn parallel_work(count) {
    print("Running " + str(count) + " parallel tasks")
    var start = `time.time()`
    
    // Create tasks
    var tasks = []
    for i in [0, 1, 2, 3, 4] {
        `tasks.append(async_work(i, 0.1))`
    }
    
    // Wait for all
    `results = await asyncio.gather(*tasks)`
    
    var elapsed = `time.time() - start`
    print("Completed in " + str(elapsed) + " seconds")
    return `results`
}

// Async system with mixed handlers
system AsyncStressTest {
    interface:
        async runTest(test_id)
        getStatus()
        async processItems(items)
        
    machine:
        $Idle {
            runTest(test_id) {
                print("Starting test " + str(test_id))
                self.current_test = test_id
                -> $Running
            }
            
            getStatus() {
                return = "idle"
            }
            
            processItems(items) {
                self.items_to_process = items
                -> $Processing
            }
        }
        
        $Running {
            $>() {
                print("Test running...")
                var result = await async_work(self.current_test, 0.5)
                print("Test result: " + result)
                self.last_result = result
                -> $Complete
            }
            
            getStatus() {
                return = "running test " + str(self.current_test)
            }
        }
        
        $Processing {
            $>() {
                print("Processing " + str(`len(self.items_to_process)`) + " items")
                
                // Process each item
                for item in self.items_to_process {
                    var result = await async_work(item, 0.1)
                    print("Processed: " + result)
                }
                
                -> $Complete
            }
            
            getStatus() {
                return = "processing"
            }
        }
        
        $Complete {
            $>() {
                print("All tasks complete")
            }
            
            getStatus() {
                return = "complete"
            }
            
            runTest(test_id) {
                self.current_test = test_id
                -> $Running
            }
        }
        
    domain:
        var current_test = 0
        var last_result = ""
        var items_to_process = []
}

// Error handling system
system AsyncErrorTest {
    interface:
        async tryOperation(should_fail)
        handleError(error_msg)
        
    machine:
        $Ready {
            tryOperation(should_fail) {
                if should_fail {
                    self.error_count = self.error_count + 1
                    -> $Error
                } else {
                    var result = await async_work(1, 0.2)
                    self.last_success = result
                    return = result
                }
            }
            
            handleError(error_msg) {
                print("Handling error: " + error_msg)
                self.last_error = error_msg
                self.error_count = self.error_count + 1
                if self.error_count > 3 {
                    -> $Error  
                }
            }
        }
        
        $Error {
            $>() {
                print("Error state entered. Count: " + str(self.error_count))
                // Auto-recovery after delay
                `await asyncio.sleep(1)`
                print("Attempting recovery...")
                self.error_count = 0
                -> $Ready
            }
            
            handleError(error_msg) {
                return = "Already in error state"
            }
        }
        
    domain:
        var error_count = 0
        var last_error = ""
        var last_success = ""
}

// Main test function
async fn run_stress_test() {
    print("=== Frame v0.37 Async Stress Test ===")
    print("")
    
    // Test 1: Basic async work
    print("Test 1: Basic Async Work")
    print("-" * 30)
    var result = await async_work(100, 0.5)
    print("Result: " + result)
    print("")
    
    // Test 2: Parallel execution
    print("Test 2: Parallel Execution")
    print("-" * 30)
    var results = await parallel_work(5)
    print("Got " + str(`len(results)`) + " results")
    print("")
    
    // Test 3: Async state machine
    print("Test 3: Async State Machine")
    print("-" * 30)
    var machine = AsyncStressTest()
    
    // Run a test
    await machine.runTest(42)
    var status = machine.getStatus()
    print("Status: " + status)
    
    // Process items
    await machine.processItems([10, 20, 30])
    status = machine.getStatus()
    print("Final status: " + status)
    print("")
    
    // Test 4: Error handling
    print("Test 4: Error Handling")
    print("-" * 30)
    var error_test = AsyncErrorTest()
    
    // Successful operation
    var success = await error_test.tryOperation(false)
    print("Success: " + success)
    
    // Failed operation
    await error_test.tryOperation(true)
    
    // Multiple errors
    error_test.handleError("Network error")
    error_test.handleError("Timeout")
    print("")
    
    print("=== All Tests Complete ===")
}

// Benchmark function
async fn benchmark() {
    print("=== Performance Benchmark ===")
    var start = `time.time()`
    
    // Create many tasks
    var tasks = []
    for i in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
        `tasks.append(async_work(i, 0.1))`
    }
    
    `results = await asyncio.gather(*tasks)`
    
    var elapsed = `time.time() - start`
    print("10 parallel tasks completed in " + str(elapsed) + " seconds")
    print("Expected ~0.1s for parallel, would be ~1.0s if serial")
}

fn main() {
    print("Frame v0.37 Async Stress Test")
    print("=" * 40)
    
    // Run async tests
    `asyncio.run(run_stress_test())`
    
    print("")
    `asyncio.run(benchmark())`
}
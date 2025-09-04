from fsl import str, int, float, list

// Comprehensive async stress test for Frame v0.37
// Tests real web downloads, parallel processing, error handling, timeouts

import aiohttp
import asyncio
import json
from time import time

// Mock async functions for testing without actual web calls
async fn mock_download(url) {
    print("Mock downloading: " + url)
    `await asyncio.sleep(0.1)`  // Simulate network delay
    return "{'data': 'Mock response from " + url + "'}"
}

async fn mock_process(data) {
    `await asyncio.sleep(0.05)`  // Simulate processing time
    return "Processed: " + data
}

// Real async download function (with fallback to mock)
async fn download_data(url) {
    try {
        `
        async with aiohttp.ClientSession() as session:
            async with session.get(url, timeout=5) as response:
                return await response.text()
        `
    } except {
        // Fallback to mock for testing
        return await mock_download(url)
    }
}

// Parallel download function
async fn download_parallel(urls) {
    `
    tasks = [download_data(url) for url in urls]
    results = await asyncio.gather(*tasks, return_exceptions=True)
    return results
    `
}

// CPU-intensive async work simulation
async fn compute_heavy(n) {
    print("Starting heavy computation for n=" + str(n))
    `
    # Simulate CPU-bound work with async sleep to allow other tasks
    result = 0
    for i in range(n):
        if i % 1000 == 0:
            await asyncio.sleep(0.001)  # Yield to other tasks
        result += i * i
    `
    print("Computation complete for n=" + str(n))
    return `result`
}

// Async timeout wrapper
async fn with_timeout(coro, timeout_sec) {
    try {
        `
        result = await asyncio.wait_for(coro, timeout=timeout_sec)
        return result
        `
    } except {
        return "TIMEOUT"
    }
}

// Complex async data pipeline system
system AsyncDataPipeline {
    interface:
        // Async methods for data processing pipeline
        async fetchBatch(urls)
        async processBatch(batch_id)
        async getStatus()
        async runPipeline(config)
        
        // Sync method that needs to work with async runtime
        configure(settings)
        
    machine:
        $Idle {
            fetchBatch(urls) {
                print("Starting batch fetch for " + str(`len(urls)`) + " URLs")
                self.current_urls = urls
                self.batch_data = []
                -> $Downloading
            }
            
            configure(settings) {
                print("Configuring pipeline: " + `str(settings)`)
                self.config = settings
                return = "configured"
            }
            
            getStatus() {
                return = "idle"
            }
            
            runPipeline(config) {
                self.pipeline_config = config
                -> $PipelineRunning
            }
        }
        
        $Downloading {
            $>() {
                // Parallel download on enter
                var start_time = `time()`
                self.batch_data = await download_parallel(self.current_urls)
                var elapsed = `time() - start_time`
                print("Downloaded " + str(`len(self.batch_data)`) + " items in " + str(elapsed) + "s")
                -> $Processing
            }
            
            getStatus() {
                return = "downloading"
            }
        }
        
        $Processing {
            $>() {
                // Process each downloaded item
                self.processed_data = []
                var tasks = []
                
                for item in self.batch_data {
                    var processed = await mock_process(`str(item)[:50]`)  // Process first 50 chars
                    `self.processed_data.append(processed)`
                }
                
                print("Processed " + str(`len(self.processed_data)`) + " items")
                -> $Complete
            }
            
            processBatch(batch_id) {
                print("Processing batch: " + str(batch_id))
                // Simulate batch processing with timeout
                var result = await with_timeout(compute_heavy(1000), 2.0)
                return = "Batch " + str(batch_id) + " result: " + str(result)
            }
            
            getStatus() {
                return = "processing"
            }
        }
        
        $Complete {
            $>() {
                print("Pipeline complete. Processed " + str(`len(self.processed_data)`) + " items")
            }
            
            getStatus() {
                return = "complete: " + str(`len(self.processed_data)`) + " items"
            }
            
            fetchBatch(urls) {
                // Can start new batch
                self.current_urls = urls
                self.batch_data = []
                -> $Downloading
            }
        }
        
        $PipelineRunning {
            $>() {
                // Complex pipeline with multiple async stages
                print("Running full pipeline")
                
                // Stage 1: Fetch multiple data sources in parallel
                var urls = self.pipeline_config`["urls"]`
                var data = await download_parallel(urls)
                print("Stage 1 complete: " + str(`len(data)`) + " sources fetched")
                
                // Stage 2: Process data with concurrency limit
                var processed = await self._process_with_limit(data)
                self.pipeline_result = processed
                print("Stage 2 complete: " + str(`len(processed)`) + " items processed")
                
                // Stage 3: Heavy computation with timeout
                var compute_tasks = []
                for i in [1000, 2000, 3000] {
                    var result = await with_timeout(compute_heavy(i), 1.0)
                    compute_tasks.append(result)
                }
                print("Stage 3 complete: " + str(compute_tasks))
                
                -> $Complete
            }
            
            getStatus() {
                return = "pipeline running"
            }
        }
        
    actions:
        _process_with_limit(data) {
            `
            import asyncio
            semaphore = asyncio.Semaphore(3)  # Limit to 3 concurrent tasks
            
            async def process_with_limit(item):
                async with semaphore:
                    return await mock_process(str(item)[:100])
            
            tasks = [process_with_limit(item) for item in data]
            return await asyncio.gather(*tasks)
            `
        }
        
    domain:
        var current_urls = []
        var batch_data = []
        var processed_data = []
        var config = None
        var pipeline_config = None
        var pipeline_result = None
}

// Async worker pool system
system AsyncWorkerPool {
    interface:
        async submitTask(task_id, work_amount)
        async submitBatch(task_ids)
        async getResults()
        async shutdown()
        
    machine:
        $Ready {
            submitTask(task_id, work_amount) {
                print("Worker processing task: " + str(task_id))
                
                // Simulate async work
                var result = await compute_heavy(work_amount)
                self.results[str(task_id)] = result
                
                return = "Task " + str(task_id) + " complete"
            }
            
            submitBatch(task_ids) {
                print("Processing batch of " + str(`len(task_ids)`) + " tasks")
                
                // Process all tasks in parallel
                await self._process_batch(task_ids)
                
                return = "Batch complete: " + str(`len(task_ids)`) + " tasks"
            }
            
            getResults() {
                return = self.results
            }
            
            shutdown() {
                print("Worker pool shutting down")
                -> $Shutdown
            }
        }
        
        $Shutdown {
            $>() {
                print("Worker pool shut down")
                self.results = `{}`
            }
            
            getResults() {
                return = "Pool is shut down"
            }
        }
        
    actions:
        _process_batch(task_ids) {
            `
            async def process_task(tid):
                result = await compute_heavy(tid * 100)
                return (tid, result)
            
            tasks = [process_task(tid) for tid in task_ids]
            results = await asyncio.gather(*tasks)
            
            for tid, result in results:
                self.results[str(tid)] = result
            `
        }
        
    domain:
        var results = `{}`
}

// Test state machine with mixed sync/async and error handling
system AsyncErrorHandler {
    interface:
        async fetchWithRetry(url, max_retries)
        handleError(error_type)
        async processWithFallback(data)
        
    machine:
        $Operational {
            fetchWithRetry(url, max_retries) {
                var attempts = 0
                var result = None
                
                while attempts < max_retries {
                    try {
                        print("Attempt " + str(attempts + 1) + " for " + url)
                        result = await download_data(url)
                        print("Success on attempt " + str(attempts + 1))
                        break
                    } except {
                        attempts = attempts + 1
                        if attempts < max_retries {
                            `await asyncio.sleep(0.5 * attempts)`  // Exponential backoff
                        }
                    }
                }
                
                if result == None {
                    self.error_count = self.error_count + 1
                    return = "Failed after " + str(max_retries) + " attempts"
                    -> $ErrorState
                } else {
                    return = result
                }
            }
            
            handleError(error_type) {
                print("Handling error: " + error_type)
                self.last_error = error_type
                self.error_count = self.error_count + 1
                
                if self.error_count > 3 {
                    -> $ErrorState
                }
                return = "Error handled"
            }
            
            processWithFallback(data) {
                try {
                    // Try primary processing
                    var result = await mock_process(data)
                    return = result
                } except {
                    // Fallback processing
                    print("Primary processing failed, using fallback")
                    `await asyncio.sleep(0.1)`
                    return = "Fallback: " + data
                }
            }
        }
        
        $ErrorState {
            $>() {
                print("System in error state. Errors: " + str(self.error_count))
                print("Last error: " + self.last_error)
                
                // Auto-recovery after delay
                `await asyncio.sleep(2)`
                print("Attempting auto-recovery...")
                self.error_count = 0
                -> $Operational
            }
            
            handleError(error_type) {
                print("Already in error state. New error: " + error_type)
                return = "In error recovery"
            }
        }
        
    domain:
        var error_count = 0
        var last_error = ""
}

// Main async stress test
async fn stress_test_async() {
    print("=== Starting Async Stress Test ===")
    print("")
    
    print("1. Testing AsyncDataPipeline")
    print("-" * 40)
    var pipeline = AsyncDataPipeline()
    
    // Test sync method with async runtime
    var config_result = pipeline.configure(`{"max_batch": 10}`)
    print("Config result: " + config_result)
    
    // Test parallel downloads
    var test_urls = [
        "https://api.github.com/users/github",
        "https://api.github.com/users/torvalds", 
        "https://api.github.com/users/gvanrossum"
    ]
    await pipeline.fetchBatch(test_urls)
    
    // Process batch
    var batch_result = await pipeline.processBatch(1)
    print("Batch result: " + batch_result)
    
    // Get status
    var status = await pipeline.getStatus()
    print("Pipeline status: " + status)
    
    // Run full pipeline
    await pipeline.runPipeline(`{"urls": test_urls}`)
    print("")
    
    print("2. Testing AsyncWorkerPool")
    print("-" * 40)
    var pool = AsyncWorkerPool()
    
    // Submit individual tasks
    var task1 = await pool.submitTask(1, 500)
    print(task1)
    
    // Submit batch of tasks
    var batch = await pool.submitBatch([10, 20, 30, 40, 50])
    print(batch)
    
    // Get all results
    var results = await pool.getResults()
    print("Worker results: " + str(`len(results)`) + " tasks completed")
    
    await pool.shutdown()
    print("")
    
    print("3. Testing AsyncErrorHandler")
    print("-" * 40)
    var handler = AsyncErrorHandler()
    
    // Test retry logic
    var retry_result = await handler.fetchWithRetry("https://fake-url-that-fails.com", 3)
    print("Retry result: " + retry_result)
    
    // Test error handling
    handler.handleError("NetworkError")
    handler.handleError("TimeoutError")
    
    // Test fallback processing
    var fallback = await handler.processWithFallback("test data")
    print("Fallback result: " + fallback)
    print("")
    
    print("=== Async Stress Test Complete ===")
}

// Performance benchmark
async fn benchmark_async() {
    print("=== Async Performance Benchmark ===")
    
    var start = `time()`
    
    // Run multiple async operations in parallel
    `
    tasks = []
    
    # Create 100 async tasks
    for i in range(100):
        if i % 3 == 0:
            tasks.append(mock_download(f"url_{i}"))
        elif i % 3 == 1:
            tasks.append(mock_process(f"data_{i}"))
        else:
            tasks.append(compute_heavy(100))
    
    # Run all tasks in parallel
    results = await asyncio.gather(*tasks, return_exceptions=True)
    `
    
    var elapsed = `time() - start`
    print("Completed 100 async tasks in " + str(elapsed) + " seconds")
    
    // Test concurrency limits
    print("\nTesting with concurrency limit (max 5 concurrent):")
    start = `time()`
    
    `
    semaphore = asyncio.Semaphore(5)
    
    async def limited_task(i):
        async with semaphore:
            if i % 2 == 0:
                return await mock_download(f"limited_url_{i}")
            else:
                return await compute_heavy(50)
    
    tasks = [limited_task(i) for i in range(50)]
    results = await asyncio.gather(*tasks)
    `
    
    elapsed = `time() - start`
    print("Completed 50 tasks with concurrency limit in " + str(elapsed) + " seconds")
}

fn main() {
    print("Frame v0.37 Async Stress Test Suite")
    print("=" * 50)
    
    // Run async stress test
    `asyncio.run(stress_test_async())`
    
    // Run performance benchmark
    print("")
    `asyncio.run(benchmark_async())`
}
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Comprehensive async stress test for Frame v0.37 - Fixed version
# Tests parallel processing, error handling, timeouts with mock functions

import asyncio
from time import time

# Mock async functions for testing without actual web calls
async fn mock_download(url) {
    print("Mock downloading: " + url)
    await asyncio.sleep(0.1)  # Simulate network delay
    return "{'data': 'Mock response from " + url + "'}"
}

async fn mock_process(data) {
    await asyncio.sleep(0.05)  # Simulate processing time
    return "Processed: " + data
}

# Async download function - now could use async with!
async fn download_data(url) {
    try:
        # With async with support, we could now do:
        # async with aiohttp.ClientSession() as session {
        #     async with session.get(url) as response {
        #         return await response.text()
        #     }
        # }
        # But for testing without network, using mock:
        return await mock_download(url)
    except:
        # Fallback to mock for testing
        return await mock_download(url)

# Parallel download function
async fn download_parallel(urls) {
    tasks = []
    for url in urls:
        tasks.append(download_data(url))
    results = await asyncio.gather(*tasks, return_exceptions=True)
    return results

# CPU-intensive async work simulation
async fn compute_heavy(n) {
    print("Starting heavy computation for n=" + str(n))
    result = 0
    for i in range(n):
        if i % 1000 == 0:
            await asyncio.sleep(0.001)
        result = result + (i * i)
    print("Computation complete for n=" + str(n))
    return result
}

# Async timeout wrapper
async fn with_timeout(coro, timeout_sec) {
    try:
        result = await asyncio.wait_for(coro, timeout=timeout_sec)
        return result
    except:
        return "TIMEOUT"

# Simple async data pipeline system for testing
system AsyncDataPipeline {
    interface:
        # Async methods for data processing pipeline
        async fetchBatch(urls)
        async processBatch(batch_id)
        async getStatus()
        async runPipeline(config)
        
        # Sync method that needs to work with async runtime
        configure(settings)
        
    machine:
        $Idle {
            async fetchBatch(urls) {
                print("Starting batch fetch for " + str(len(urls)) + " URLs")
                self.current_urls = urls
                self.batch_data = []
                -> $Downloading
            configure(settings) {
                print("Configuring pipeline: " + str(settings))
                self.config = settings
                system.return = "configured"
            }
            async getStatus() {
                system.return = "idle"
            }
            async runPipeline(config) {
                self.pipeline_config = config
                -> $PipelineRunning
            }
        }
        $Downloading {
            async $>() {  # Explicitly mark as async handler
                # Parallel download on enter
                start_time = time()
                self.batch_data = await download_parallel(self.current_urls)
                elapsed = time() - start_time
                print("Downloaded " + str(len(self.batch_data)) + " items in " + str(elapsed) + "s")
                -> $Processing
            }
            async getStatus() {
                system.return = "downloading"
            }
        }
        $Processing {
            async $>() {  # Explicitly mark as async handler
                # Process each downloaded item
                self.processed_data = []
                
                for item in self.batch_data:
                    processed = await mock_process(str(item))
                    self.processed_data.append(processed)
                
                print("Processed " + str(len(self.processed_data)) + " items")
                -> $Complete
            }
            async processBatch(batch_id) {  # Mark as async since it uses await
                print("Processing batch: " + str(batch_id))
                # Simulate batch processing with timeout
                result = await with_timeout(compute_heavy(1000), 2.0)
                system.return = "Batch " + str(batch_id) + " result: " + str(result)
            }
            async getStatus() {
                system.return = "processing"
            }
        }
        $Complete {
            async $>() {  # Must be async - entered from async PipelineRunning state
                print("Pipeline complete. Processed " + str(len(self.processed_data)) + " items")
            }
            async getStatus() {
                system.return = "complete: " + str(len(self.processed_data)) + " items"
            }
            async fetchBatch(urls) {
                # Can start new batch
                self.current_urls = urls
                self.batch_data = []
                -> $Downloading
            
            async processBatch(batch_id) {  # Mark as async since it uses await
                print("Processing batch: " + str(batch_id) + " (in complete state)")
                # Can still process batches even when complete
                result = await with_timeout(compute_heavy(500), 2.0)
                system.return = "Batch " + str(batch_id) + " result: " + str(result)
            }
        }
        $PipelineRunning {
            async $>() {  # Explicitly mark as async handler
                # Complex pipeline with multiple async stages
                print("Running full pipeline")
                
                # Stage 1: Fetch multiple data sources in parallel (config is now urls directly)
                urls = self.pipeline_config
                data = await download_parallel(urls)
                print("Stage 1 complete: " + str(len(data)) + " sources fetched")
                
                # Stage 2: Process data with concurrency limit
                processed = await self._process_with_limit(data)
                self.pipeline_result = processed
                print("Stage 2 complete: " + str(len(processed)) + " items processed")
                
                # Stage 3: Heavy computation with timeout
                compute_tasks = []
                for i in [1000, 2000, 3000]:
                    result = await with_timeout(compute_heavy(i), 1.0)
                    compute_tasks.append(result)
                print("Stage 3 complete: " + str(compute_tasks))
                
                -> $Complete
            }
            async getStatus() {
                system.return = "pipeline running"
            }
        }
    }
    actions:
        async _process_with_limit(data) {
            # Need semaphore support - temporary simplified version
            results = []
            for item in data:
                result = await mock_process(str(item))
                results.append(result)
            return results
        
    domain:
        current_urls = []
        batch_data = []
        processed_data = []
        config = None
        pipeline_config = None
        pipeline_result = None
}

# Main async stress test - simplified version
async fn stress_test_async() {
    print("=== Starting Async Stress Test ===")
    print("")
    
    print("1. Testing AsyncDataPipeline")
    print("-" * 40)
    pipeline = AsyncDataPipeline()
    
    # Test sync method with async runtime (simplified - no dict literal)
    config_result = pipeline.configure("max_batch_10")
    print("Config result: " + str(config_result))
    
    # Test parallel downloads
    test_urls = [
        "https://api.github.com/users/github",
        "https://api.github.com/users/torvalds", 
        "https://api.github.com/users/gvanrossum"
    ]
    await pipeline.fetchBatch(test_urls)
    
    # Process batch
    batch_result = await pipeline.processBatch(1)
    print("Batch result: " + str(batch_result))
    
    # Get status
    status = await pipeline.getStatus()
    print("Pipeline status: " + str(status))
    
    # Run full pipeline (passing urls directly - no dict literal)
    await pipeline.runPipeline(test_urls)
    print("")
    
    print("=== Async Stress Test Complete ===")
}

# Performance benchmark
async fn benchmark_async() {
    print("=== Async Performance Benchmark ===")
    
    start = time()
    
    # Run multiple async operations in parallel
    tasks = []
    
    # Create 100 async tasks
    for i in range(100):
        if i % 3 == 0:
            tasks.append(mock_download("url_" + str(i)))
        elif i % 3 == 1:
            tasks.append(mock_process("data_" + str(i)))
        else:
            tasks.append(compute_heavy(100))
    
    # Run all tasks in parallel
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    elapsed = time() - start
    print("Completed 100 async tasks in " + str(elapsed) + " seconds")

fn main() {
    print("Frame v0.37 Async Stress Test Suite - Fixed")
    print("=" * 50)
    
    # Run async stress test
    asyncio.run(stress_test_async())
    
    # Run performance benchmark
    print("")
    asyncio.run(benchmark_async())
}

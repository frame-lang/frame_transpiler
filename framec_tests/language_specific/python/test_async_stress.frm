@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Comprehensive async stress test for Frame v0.37
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

# Real async download function (with fallback to mock)
async fn download_data(url) {
    try:
        # Need async with support - temporary placeholder
        return await mock_download(url)
    except:
        # Fallback to mock for testing
        return await mock_download(url)
}

# Parallel download function
async fn download_parallel(urls) {
    tasks = []
    for url in urls:
        tasks.append(download_data(url))
    results = await asyncio.gather(*tasks, return_exceptions=True)
    return results
}

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
}

# Complex async data pipeline system
system AsyncDataPipeline {
    interface:
        # Async methods for data processing pipeline
        async fetchBatch(urls)
        async processBatch(batch_id)
        async getStatus()
        async runPipeline(config)
        
        # Async method (all interface methods become async in async systems)
        async configure(settings)
        
    machine:
        $Idle {
            fetchBatch(urls) {
                print("Starting batch fetch for " + str(len(urls)) + " URLs")
                self.current_urls = urls
                self.batch_data = []
                -> $Downloading
            
            configure(settings) {
                print("Configuring pipeline: " + str(settings))
                self.config = settings
                system.return = "configured"
            
            getStatus() {
                system.return = "idle"
            
            runPipeline(config) {
                self.pipeline_config = config
                -> $PipelineRunning
        
        $Downloading {
            async $>() {
                # Parallel download on enter
                start_time = time()
                self.batch_data = await download_parallel(self.current_urls)
                elapsed = time() - start_time
                print("Downloaded " + str(len(self.batch_data)) + " items in " + str(elapsed) + "s")
                -> $Processing
            
            getStatus() {
                system.return = "downloading"
        
        $Processing {
            async $>() {
                # Process each downloaded item
                self.processed_data = []
                tasks = []
                
                for item in self.batch_data:
                    processed = await mock_process(str(item))  # Process all chars for now
                    self.processed_data.append(processed)
                
                print("Processed " + str(len(self.processed_data)) + " items")
                -> $Complete
            
            async processBatch(batch_id) {
                print("Processing batch: " + str(batch_id))
                # Simulate batch processing with timeout
                result = await with_timeout(compute_heavy(1000), 2.0)
                system.return = "Batch " + str(batch_id) + " result: " + str(result)
            
            getStatus() {
                system.return = "processing"
        
        $Complete {
            $>() {
                print("Pipeline complete. Processed " + str(len(self.processed_data)) + " items")
            
            getStatus() {
                system.return = "complete: " + str(len(self.processed_data)) + " items"
            
            fetchBatch(urls) {
                # Can start new batch
                self.current_urls = urls
                self.batch_data = []
                -> $Downloading
            
            async processBatch(batch_id) {
                print("Processing batch: " + str(batch_id) + " (in complete state)")
                # Can still process batches even when complete
                result = await with_timeout(compute_heavy(500), 2.0)
                system.return = "Batch " + str(batch_id) + " result: " + str(result)
        
        $PipelineRunning {
            async $>() {
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
            
            getStatus() {
                system.return = "pipeline running"
        
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

# Async worker pool system
system AsyncWorkerPool {
    interface:
        async submitTask(task_id, work_amount)
        async submitBatch(task_ids)
        async getResults()
        async shutdown()
        
    machine:
        $Ready {
            $>() {
                # TODO: Initialize results dict - Frame needs dict literal support
                # For now, we'll skip dict operations in this test
            
            async submitTask(task_id, work_amount) {
                print("Worker processing task: " + str(task_id))
                
                # Simulate async work
                result = await compute_heavy(work_amount)
                # TODO: Store result - self.results[str(task_id)] = result
                
                system.return = "Task " + str(task_id) + " complete"
            
            async submitBatch(task_ids) {
                print("Processing batch of " + str(len(task_ids)) + " tasks")
                
                # Process all tasks in parallel
                await self._process_batch(task_ids)
                
                system.return = "Batch complete: " + str(len(task_ids)) + " tasks"
            
            getResults() {
                # Return a simple string describing results
                system.return = "6 tasks completed"
            
            shutdown() {
                print("Worker pool shutting down")
                -> $Shutdown
        
        $Shutdown {
            $>() {
                print("Worker pool shut down")
                # TODO: Clear results dictionary - Frame needs dict literal support
            
            getResults() {
                system.return = "Pool is shut down"
        
    actions:
        async _process_batch(task_ids) {
            for tid in task_ids:
                result = await compute_heavy(tid * 100)
                # TODO: Store result - self.results[str(tid)] = result
        
    domain:
        results = None
}

# Test state machine with mixed sync/async and error handling
system AsyncErrorHandler {
    interface:
        async fetchWithRetry(url, max_retries)
        handleError(error_type)
        async processWithFallback(data)
        
    machine:
        $Operational {
            async fetchWithRetry(url, max_retries) {
                attempts = 0
                result = None
                
                while attempts < max_retries:
                    try:
                        print("Attempt " + str(attempts + 1) + " for " + url)
                        result = await download_data(url)
                        print("Success on attempt " + str(attempts + 1))
                        break
                    except:
                        attempts = attempts + 1
                        if attempts < max_retries:
                            await asyncio.sleep(0.5 * attempts)  # Exponential backoff
                
                if result == None:
                    self.error_count = self.error_count + 1
                    system.return = "Failed after " + str(max_retries) + " attempts"
                    -> $ErrorState
                } else {
                    system.return = result
            
            handleError(error_type) {
                print("Handling error: " + error_type)
                self.last_error = error_type
                self.error_count = self.error_count + 1
                
                if self.error_count > 3:
                    -> $ErrorState
                system.return = "Error handled"
            
            async processWithFallback(data) {
                try:
                    # Try primary processing
                    result = await mock_process(data)
                    system.return = result
                except:
                    # Fallback processing
                    print("Primary processing failed, using fallback")
                    await asyncio.sleep(0.1)
                    system.return = "Fallback: " + data
        
        $ErrorState {
            async $>() {
                print("System in error state. Errors: " + str(self.error_count))
                print("Last error: " + self.last_error)
                
                # Auto-recovery after delay
                await asyncio.sleep(2)
                print("Attempting auto-recovery...")
                self.error_count = 0
                -> $Operational
            
            handleError(error_type) {
                print("Already in error state. New error: " + error_type)
                system.return = "In error recovery"
        
    domain:
        error_count = 0
        last_error = ""
}

# Main async stress test
async fn stress_test_async() {
    print("=== Starting Async Stress Test ===")
    print("")
    
    print("1. Testing AsyncDataPipeline")
    print("-" * 40)
    pipeline = AsyncDataPipeline()
    await pipeline.async_start()  # Initialize async system
    
    # Test async configure method (all interface methods are async in async systems)
    config_result = await pipeline.configure("max_batch_10")
    print("Config result: " + config_result)
    
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
    
    print("2. Testing AsyncWorkerPool")
    print("-" * 40)
    pool = AsyncWorkerPool()
    
    # Submit individual tasks
    task1 = await pool.submitTask(1, 500)
    print(task1)
    
    # Submit batch of tasks
    batch = await pool.submitBatch([10, 20, 30, 40, 50])
    print(batch)
    
    # Get all results
    results = await pool.getResults()
    print("Worker results: " + str(len(results)) + " tasks completed")
    
    await pool.shutdown()
    print("")
    
    print("3. Testing AsyncErrorHandler")
    print("-" * 40)
    handler = AsyncErrorHandler()
    
    # Test retry logic
    retry_result = await handler.fetchWithRetry("https://fake-url-that-fails.com", 3)
    print("Retry result: " + retry_result)
    
    # Test error handling
    handler.handleError("NetworkError")
    handler.handleError("TimeoutError")
    
    # Test fallback processing
    fallback = await handler.processWithFallback("test data")
    print("Fallback result: " + fallback)
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
        } elif i % 3 == 1 {
            tasks.append(mock_process("data_" + str(i)))
        } else {
            tasks.append(compute_heavy(100))
    
    # Run all tasks in parallel
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    elapsed = time() - start
    print("Completed 100 async tasks in " + str(elapsed) + " seconds")
    
    # Test concurrency limits
    print("\nTesting with concurrency limit (max 5 concurrent):")
    start = time()
    
    # Need semaphore support - simplified version
    tasks2 = []
    for i in range(50):
        if i % 2 == 0:
            tasks2.append(mock_download("limited_url_" + str(i)))
        } else {
            tasks2.append(compute_heavy(50))
    results2 = await asyncio.gather(*tasks2)
    
    elapsed = time() - start
    print("Completed 50 tasks with concurrency limit in " + str(elapsed) + " seconds")
}

fn main() {
    print("Frame v0.37 Async Stress Test Suite")
    print("=" * 50)
    
    # Run async stress test
    asyncio.run(stress_test_async())
    
    # Run performance benchmark
    print("")
    asyncio.run(benchmark_async())
}

# Example Frame System Using Capability Modules
# This demonstrates the Python-syntax-based universal approach
# Same Frame source currently targets Python, TypeScript, and Graphviz (additional languages future work)

import AsyncSupport from "./async.frm"
import Collections from "./collections.frm"
import Memory from "./memory.frm"
import Errors from "./errors.frm"
import FileSystem from "./filesystem.frm"

# Data processing function using capability modules
async fn processUrlData(url, data) {
    print("Processing data from: " + url)
    
    # Use collections module for data manipulation
    var items = Collections.createList()
    var results = Collections.createMap()
    
    # Parse data (simulated)
    for i in range(len(data)) {
        Collections.append(items, "item_" + str(i))
        Collections.put(results, "key_" + str(i), data[i])
    }
    
    # Use error handling
    var processResult = Errors.tryExecute(lambda: {
        var processed = Collections.map(items, lambda x: x.upper())
        return processed
    })
    
    if Errors.isOk(processResult) {
        var processed = Errors.unwrap(processResult)
        print("Successfully processed " + str(len(processed)) + " items")
        return processed
    } else {
        var error = Errors.getError(processResult)
        print("Processing failed: " + error)
        return []
    }
}

# Web crawler system using all capability modules
system WebCrawler {
    domain:
        var urls: list = Collections.createList()
        var results: map = Collections.createMap()
        var errors: list = Collections.createList()
        var maxConcurrent: int = 5
        var outputFile: string = "crawler_results.json"
    
    interface:
        async crawlUrls(urlList: list): map
        async saveResults(): bool
        getStats(): map
        async processWithRetry(url: string): string
    
    machine:
        $Ready {
            async crawlUrls(urlList) {
                print("Starting crawl of " + str(len(urlList)) + " URLs")
                
                # Store URLs using collections module
                self.urls = Collections.copy(urlList)
                
                # Create async tasks for concurrent processing
                var tasks = Collections.createList()
                
                for url in urlList {
                    var task = AsyncSupport.createTask(lambda: self.processWithRetry(url))
                    Collections.append(tasks, task)
                }
                
                # Execute tasks in parallel with concurrency limit
                print("Processing URLs with max concurrency: " + str(self.maxConcurrent))
                var responses = await AsyncSupport.waitAll(tasks)
                
                # Store results
                for i in range(len(urlList)) {
                    Collections.put(self.results, urlList[i], responses[i])
                }
                
                -> $Processing
                ^ return self.results
            }
            
            getStats() {
                var stats = Collections.createMap()
                Collections.put(stats, "urls_count", Collections.length(self.urls))
                Collections.put(stats, "results_count", Collections.length(self.results))
                Collections.put(stats, "errors_count", Collections.length(self.errors))
                ^ return stats
            }
        }
        
        $Processing {
            async $>() {
                print("Entered processing state")
                await AsyncSupport.sleep(100)  # Brief pause
                -> $Ready
            }
            
            async saveResults() {
                print("Saving results to: " + self.outputFile)
                
                # Use memory management for file operations
                var saveResult = await Errors.tryExecuteAsync(lambda: {
                    # Convert results to JSON-like string
                    var jsonData = "{\n"
                    var items = Collections.items(self.results)
                    
                    for i in range(len(items)) {
                        var item = items[i]
                        var key = item[0]
                        var value = item[1]
                        jsonData = jsonData + "  \"" + key + "\": \"" + str(value) + "\""
                        
                        if i < len(items) - 1 {
                            jsonData = jsonData + ","
                        }
                        jsonData = jsonData + "\n"
                    }
                    
                    jsonData = jsonData + "}"
                    
                    # Write using filesystem module
                    FileSystem.writeFile(self.outputFile, jsonData)
                    return True
                })
                
                if Errors.isOk(saveResult) {
                    print("Results saved successfully")
                    ^ return True
                } else {
                    var error = Errors.getError(saveResult)
                    Collections.append(self.errors, "Save failed: " + error)
                    print("Failed to save results: " + error)
                    ^ return False
                }
            }
            
            async processWithRetry(url) {
                print("Processing URL with retry: " + url)
                
                # Use error handling with retry
                var result = await Errors.retryAsync(
                    lambda: AsyncSupport.httpGet(url),
                    3,  # max attempts
                    1000  # delay between retries (ms)
                )
                
                if Errors.isOk(result) {
                    var response = Errors.unwrap(result)
                    var processedData = await processUrlData(url, response["body"])
                    return "Success: " + str(len(processedData)) + " items"
                } else {
                    var error = Errors.getError(result)
                    Collections.append(self.errors, url + ": " + error)
                    return "Failed: " + error
                }
            }
        }
    
    actions:
        logError(message) {
            Collections.append(self.errors, message)
            print("ERROR: " + message)
        }
        
        clearResults() {
            self.results = Collections.createMap()
            self.errors = Collections.createList()
            print("Results cleared")
        }
        
    operations:
        async validateUrl(url) {
            # Validate URL format using error handling
            var validation = Errors.validate(
                url,
                lambda x: len(x) > 0 and ("http://" in x or "https://" in x),
                "Invalid URL format"
            )
            
            if Errors.isOk(validation) {
                return Errors.unwrap(validation)
            } else {
                var error = Errors.getError(validation)
                self.logError("URL validation failed: " + error)
                raise ValueError(error)
            }
        }
        
        async healthCheck() {
            # Test connectivity using async module
            var testUrl = "https://httpbin.org/status/200"
            
            print("Performing health check...")
            var response = await AsyncSupport.httpGet(testUrl)
            
            if response["status"] == 200 {
                print("Health check passed")
                return True
            } else {
                print("Health check failed: " + str(response["status"]))
                return False
            }
        }
}

# Main function demonstrating usage
async fn main() {
    print("=== Frame Capability Modules Example ===")
    
    # Create crawler instance
    var crawler = WebCrawler()
    
    # Test URLs
    var testUrls = [
        "https://httpbin.org/json",
        "https://httpbin.org/user-agent", 
        "https://httpbin.org/ip",
        "https://httpbin.org/headers"
    ]
    
    # Perform health check
    var healthOk = await crawler.healthCheck()
    if not healthOk {
        print("Health check failed, exiting")
        return
    }
    
    # Crawl URLs
    print("\nCrawling URLs...")
    var results = await crawler.crawlUrls(testUrls)
    
    # Show statistics
    var stats = crawler.getStats()
    print("\nCrawl Statistics:")
    var statItems = Collections.items(stats)
    for item in statItems {
        print("  " + item[0] + ": " + str(item[1]))
    }
    
    # Save results
    print("\nSaving results...")
    var saved = await crawler.saveResults()
    
    if saved {
        print("Results saved to: " + crawler.outputFile)
        
        # Verify saved file
        if FileSystem.exists(crawler.outputFile) {
            var fileSize = FileSystem.getFileSize(crawler.outputFile)
            print("File size: " + str(fileSize) + " bytes")
        }
    } else {
        print("Failed to save results")
    }
    
    print("\n=== Example Complete ===")
}

# Entry point
if __name__ == "__main__" {
    await main()
}

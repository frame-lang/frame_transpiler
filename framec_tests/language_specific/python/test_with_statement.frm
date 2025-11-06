@target python
# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test file for with statement support in Frame v0.37
import asyncio
import aiohttp
import os

# Test regular with statement for file operations
fn test_with_file() {
    script_dir = os.path.dirname(os.path.abspath(__file__))
    test_file = os.path.join(script_dir, "test.txt")
    input_file_path = os.path.join(script_dir, "input.txt")
    output_file_path = os.path.join(script_dir, "output.txt")
    
    with open(test_file, "r") as f:
        content = f.read()
        print("File content: " + content)
    
    # Nested with statements
    with open(input_file_path, "r") as input_file:
        with open(output_file_path, "w") as output_file:
            data = input_file.read()
            output_file.write(data.upper())
}

# Test async with statement for network operations
async fn test_async_with() {
    # Using async with for aiohttp session
    async with aiohttp.ClientSession() as session:
        async with session.get("https://api.github.com") as response:
            text = await response.text()
            print("Response length: " + str(len(text)))
}

# Test with statement in a system
system FileProcessor {
    interface:
        processFile(filename)
        
    machine:
        $Idle {
            processFile(filename) {
                # Use with statement inside event handler
                with open(filename, "r") as file:
                    self.content = file.read()
                    print("Read " + str(len(self.content)) + " bytes")
                -> $Processing
            }
        }
        
        $Processing {
            $>() {
                print("Processing file content...")
                # Transform content to uppercase
                self.processed = self.content.upper()
                -> $Done
            }
        }
        
        $Done {
            $>() {
                print("Processing complete")
            }
        }
        
    domain:
        content = ""
        processed = ""
}

# Test async with in async system methods
system AsyncDataFetcher {
    interface:
        async fetchData(url)
        
    machine:
        $Ready {
            fetchData(url) {
                print("Fetching from: " + url)
                
                # Use async with for resource management
                async with aiohttp.ClientSession() as session:
                    async with session.get(url) as response:
                        self.data = await response.text()
                        self.status_code = response.status
                
                print("Fetched " + str(len(self.data)) + " bytes")
                print("Status: " + str(self.status_code))
                system.return = self.data
            }
        }
        
    domain:
        data = ""
        status_code = 0
}

# Async function to test everything
async fn test_all() {
    print("Testing with statement support in Frame v0.37")
    print("=" * 50)
    
    # Test regular with
    print("\n1. Testing regular with statement:")
    test_with_file()
    
    # Test async with
    print("\n2. Testing async with statement:")
    await test_async_with()
    
    # Test with in system
    print("\n3. Testing with in system:")
    script_dir = os.path.dirname(os.path.abspath(__file__))
    test_file = os.path.join(script_dir, "test.txt")
    processor = FileProcessor()
    processor.processFile(test_file)
    
    # Test async with in system
    print("\n4. Testing async with in system:")
    fetcher = AsyncDataFetcher()
    result = await fetcher.fetchData("https://api.github.com")
    # Note: String slicing not yet supported in Frame
    print("Got result from API")
}

# Entry point - will be called from __main__
fn main() {
    asyncio.run(test_all())
}

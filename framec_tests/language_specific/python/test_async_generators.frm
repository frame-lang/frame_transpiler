# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test async generators in Frame v0.42
# This demonstrates Frame's support for async generators combining v0.35 async and v0.42 generators

import asyncio

# Basic async generator
async fn async_range(n) {
    i = 0
    while i < n:
        await asyncio.sleep(0.01)  # Simulate async work
        yield i
        i = i + 1
}

# Async generator that yields from async operations
async fn fetch_data_generator() {
    yield "Starting data fetch"
    await asyncio.sleep(0.01)
    yield "Fetched item 1"
    await asyncio.sleep(0.01)
    yield "Fetched item 2"
    await asyncio.sleep(0.01)
    yield "Complete"
}

# Test function using async generator
async fn test_async_gen() {
    print("Testing async generator:")
    
    # Create async generator
    gen = async_range(3)
    
    # Manual iteration (Python's async for not yet in Frame)
    # In Python this would be: async for val in async_range(3): print(val)
    try:
        while True:
            val = await gen.__anext__()
            print("Generated: " + str(val))
    except StopAsyncIteration:
        print("Generator exhausted")
    
    print("\nTesting fetch generator:")
    fetch_gen = fetch_data_generator()
    try:
        while True:
            msg = await fetch_gen.__anext__()
            print("Status: " + msg)
    except StopAsyncIteration:
        print("Fetch complete")

# Main async function
async fn async_main() {
    await test_async_gen()
    print("All async generator tests complete!")

fn main() {
    # Module initialization - demonstrates async generator creation
    print("=== Frame v0.42: Async Generator Support ===")
    print("Creating async generators with 'async fn' + 'yield'")
    
    # Note: To run, Python needs: asyncio.run(async_main())
    # This would typically be added in a runner script

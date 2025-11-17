@target python_3

async fn fetch_data(url) {
    print("Fetching from " + url)
    return "data from " + url
}

async fn process_data(data) {
    print("Processing: " + data)
    result = await fetch_data("api.example.com/process")
    return "processed " + data + " with " + result
}

fn main() {
    import asyncio

    async def runner():
        print("=== async basic V3 ===")
        data = await fetch_data("api.example.com/item/123")
        print("Got:", data)
        result = await process_data("test payload")
        print("Result:", result)

    asyncio.run(runner())
}

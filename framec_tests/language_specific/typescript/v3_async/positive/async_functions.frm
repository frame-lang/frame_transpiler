@target typescript

// @core
// @run-expect: start
// @run-expect: data from api.example.com/item/123
// @run-expect: processed payload with data from api.example.com/process
// @run-expect: done

system AsyncDemoTs {
    operations:
        async fn fetchData(url: string): Promise<string> {
            console.log(`fetching ${url}`);
            return `data from ${url}`;
        }

        async fn processData(data: string): Promise<string> {
            const result = await fetchData("api.example.com/process");
            return `processed ${data} with ${result}`;
        }

    machine:
        $A {
            fn main(): void {
                console.log("start");
                (async () => {
                    const data = await fetchData("api.example.com/item/123");
                    console.log(data);
                    const result = await processData("payload");
                    console.log(result);
                    console.log("done");
                })();
            }
        }
}

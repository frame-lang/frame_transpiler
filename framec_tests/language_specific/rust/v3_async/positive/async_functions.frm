@target rust

// @core
// @run-expect: start
// @run-expect: data from api.example.com/item/123
// @run-expect: processed payload with data from api.example.com/process
// @run-expect: done
system AsyncDemoRs {
    operations:
        async fn fetch_data(url: String) -> String {
            println!("fetching {}", url);
            format!("data from {}", url)
        }

        async fn process_data(data: String) -> String {
            let result = fetch_data("api.example.com/process".to_string()).await;
            format!("processed {} with {}", data, result)
        }

    machine:
        $A {
            async fn main_entry() {
                println!("start");
                let data = fetch_data("api.example.com/item/123".to_string()).await;
                println!("{}", data);
                let result = process_data("payload".to_string()).await;
                println!("{}", result);
                println!("done");
            }

            async fn e() {
                main_entry().await;
                -> $A()
            }
        }
}

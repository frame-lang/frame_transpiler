// Minimal test to isolate scope issue

fn main() {
    var local_var = "test"
    print(local_var)  // This should generate: print(local_var), NOT: print(self)
}
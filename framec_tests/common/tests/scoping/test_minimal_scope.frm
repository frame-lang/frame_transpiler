# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Minimal test to isolate scope issue

fn main() {
    var local_var = "test"
    print(local_var)  # This should generate: print(local_var), NOT: print(self)
}
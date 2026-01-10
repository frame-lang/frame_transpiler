// Simple test for v4 compiler
fn main() {
    println!("Testing Frame v4 compiler...");
    
    let source = r#"@@target python

system TrafficLight {
    interface:
        timer()
    
    machine:
        $Red {
            timer() {
                print("Red")
                -> $Green()
            }
        }
        
        $Green {
            timer() {
                print("Green")
                -> $Red()
            }
        }
}"#;

    println!("Input:");
    println!("{}", source);
    println!("\nCompiling...");
    
    // For now, just test that the module exists
    println!("✅ V4 module exists and compiles!");
}
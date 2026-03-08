// Debug scanner output
use framec::frame_c::v4::scanner;

fn main() {
    let source = r#"@@target python

system Test {
    machine:
        $Red {
            timer() {
                print("hello")
                -> $Green()
            }
        }
}"#;

    match scanner::scan(source, "test.fpy") {
        Ok(tokens) => {
            println!("Tokens:");
            for token in &tokens {
                println!("  {:?}: '{}'", token.token_type, token.lexeme);
            }
        }
        Err(e) => {
            println!("Scanner error: {:?}", e);
        }
    }
}
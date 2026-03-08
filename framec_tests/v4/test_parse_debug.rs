// Debug what the parser is extracting
use framec::frame_c::v4::*;

fn main() {
    let source = r#"@@target python
system Test {
    machine:
        $Start {
            go() {
                print("Before transition")
                -> $End()
                print("After transition - unreachable")
            }
        }
        $End {}
}"#;

    let tokens = scanner::scan(source, "test.fpy").unwrap();
    let ast = parser::parse(tokens, source).unwrap();
    
    println!("=== Parse Debug ===\n");
    
    if let Some(machine) = &ast.machine {
        for state in &machine.states {
            println!("State: {}", state.name);
            for handler in &state.handlers {
                if let Some(name) = &handler.name {
                    println!("  Handler: {}", name);
                    println!("    Native code: '{}'", handler.native_code);
                    println!("    Frame statements: {:?}", handler.frame_statements);
                }
            }
        }
    }
}
// Test v4 code generation
use framec::frame_c::v4::*;
use std::fs;

fn main() {
    // Read the traffic light Frame file
    let source = fs::read_to_string("framec_tests/v4/traffic_light.fpy")
        .expect("Failed to read traffic_light.fpy");
    
    println!("=== Frame v4 Code Generation Test ===\n");
    println!("Input file: traffic_light.fpy");
    
    // Scan
    let tokens = match scanner::scan(&source, "traffic_light.fpy") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Scanner error: {:?}", e);
            return;
        }
    };
    
    println!("✓ Scanning complete: {} tokens", tokens.len());
    
    // Parse
    let ast = match parser::parse(tokens, &source) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            return;
        }
    };
    
    println!("✓ Parsing complete: system '{}'", ast.name);
    
    // Validate
    if let Err(e) = validator::validate(&ast) {
        eprintln!("Validation error: {:?}", e);
        return;
    }
    
    println!("✓ Validation passed");
    
    // Generate Python code
    let code = match codegen::generate(&ast, TargetLanguage::Python) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Code generation error: {:?}", e);
            return;
        }
    };
    
    println!("✓ Code generation complete\n");
    println!("=== Generated Python Code ===\n");
    println!("{}", code);
    
    // Write to file
    fs::write("framec_tests/v4/traffic_light.py", &code)
        .expect("Failed to write output");
    
    println!("\n=== Output written to traffic_light.py ===");
    println!("\nTo test the generated code:");
    println!("  cd framec_tests/v4");
    println!("  python3 -c \"from traffic_light import TrafficLight; t = TrafficLight(); t.timer(); print(t.get_color())\"");
}
//! Debug module for Frame parser

use super::frame_parser::*;
use super::frame_ast::*;

pub fn debug_parse() {
    let source = r#"@@system Test {
    machine:
        $Start {
        }
}"#;
    
    eprintln!("=== Starting parse debug ===");
    eprintln!("Source:\n{}", source);
    
    let mut parser = FrameParser::new(source.as_bytes(), TargetLanguage::Python3);
    
    match parser.parse_module() {
        Ok(ast) => {
            eprintln!("Parse succeeded!");
            match ast {
                FrameAst::System(sys) => {
                    eprintln!("System: {}", sys.name);
                    if let Some(machine) = sys.machine {
                        eprintln!("  Machine with {} states", machine.states.len());
                    }
                }
                FrameAst::Module(_) => eprintln!("Module"),
            }
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_debug_minimal() {
        debug_parse();
    }
}
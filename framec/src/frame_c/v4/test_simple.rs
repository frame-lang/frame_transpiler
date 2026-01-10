// Simple test to debug v4
#[cfg(test)]
mod test {
    use crate::frame_c::v4::*;
    use crate::frame_c::v4::scanner::TokenType;
    
    #[test]
    fn test_simple_scan() {
        let source = "system Test { }";
        let tokens = scanner::scan(source, "test.fpy").unwrap();
        
        println!("Tokens for '{}': ", source);
        for token in &tokens {
            println!("  {:?}", token.token_type);
        }
        
        assert!(tokens.len() > 0);
    }
    
    #[test]
    fn test_handler_parse() {
        let source = r#"@@target python
system Test {
    machine:
        $Start {
            go() {
                print("hello")
                -> $End()
            }
        }
        $End {}
}"#;
        
        let tokens = scanner::scan(source, "test.fpy").unwrap();
        println!("\nTokens:");
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?} = '{}'", i, token.token_type, token.lexeme);
        }
        
        let ast = parser::parse(tokens, source);
        match ast {
            Ok(a) => {
                println!("\nAST created: {}", a.name);
                if let Some(machine) = &a.machine {
                    for state in &machine.states {
                        println!("  State: {}", state.name);
                        for handler in &state.handlers {
                            if let Some(name) = &handler.name {
                                println!("    Handler: {}", name);
                                println!("      Native code: '{}'", handler.native_code);
                                println!("      Frame statements: {:?}", handler.frame_statements);
                            }
                        }
                    }
                }
            }
            Err(e) => println!("\nParse error: {:?}", e),
        }
    }
    
    #[test]
    fn test_simple_parse() {
        let source = r#"@@target python
system Test {
    machine:
        $Start {}
}"#;
        
        let tokens = scanner::scan(source, "test.fpy").unwrap();
        println!("\nTokens:");
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?} = '{}'", i, token.token_type, token.lexeme);
        }
        
        let ast = parser::parse(tokens, source);
        match ast {
            Ok(a) => println!("\nAST created: {}", a.name),
            Err(e) => println!("\nParse error: {:?}", e),
        }
    }
}
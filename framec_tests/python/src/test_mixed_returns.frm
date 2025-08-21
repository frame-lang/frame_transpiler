// Test: Mixed usage of return and return = in different contexts
fn main() {
    var analyzer = TextAnalyzer()
    
    // Test different text inputs
    var result1 = analyzer.analyze("")
    print("Empty: " + result1)
    
    var result2 = analyzer.analyze("hello")
    print("hello: " + result2)
    
    var result3 = analyzer.analyze("HELLO WORLD")
    print("HELLO WORLD: " + result3)
    
    var result4 = analyzer.analyze("Frame v0.20 is great!")
    print("Frame v0.20 is great!: " + result4)
}

system TextAnalyzer {
    interface:
        analyze(text: str): str
        
    machine:
        $Analyzing {
            analyze(text: str): str {
                // Use early returns with return = for interface value
                if text == "" {
                    return = "empty input"
                    return  // Exit handler
                }
                
                // Call action for processing
                var category = categorizeText(text)
                print("Category from action: " + category)
                
                // Set final interface return based on category
                if category == "short" {
                    return = "short text: " + text
                } elif category == "caps" {
                    return = "LOUD TEXT: " + text
                } else {
                    return = "normal text: " + text
                }
                
                return  // Exit handler
            }
        }
        
    actions:
        categorizeText(text: str): str {
            // This action uses regular returns
            if len(text) < 10 {
                return "short"  // Return to caller
            }
            
            if text == text.upper() && text != text.lower() {
                return "caps"   // Return to caller
            }
            
            return "normal"     // Return to caller
        }
        
        // Helper action
        len(s: str): int {
            // Simple return of computed value
            var count = 0
            for c in s {
                count = count + 1
            }
            return count
        }
}
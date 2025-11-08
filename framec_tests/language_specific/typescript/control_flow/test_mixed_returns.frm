# TS override: Mixed usage of return and system.return in handlers/actions
fn main() {
    var analyzer = TextAnalyzer()
    // Test different text inputs
    var result1 = analyzer.analyze("")
    console.log("Empty: " + result1);

    var result2 = analyzer.analyze("hello")
    console.log("hello: " + result2);

    var result3 = analyzer.analyze("HELLO WORLD")
    console.log("HELLO WORLD: " + result3);

    var result4 = analyzer.analyze("Frame v0.20 is great!")
    console.log("Frame v0.20 is great!: " + result4);
}

system TextAnalyzer {
    interface:
        analyze(text: str): str

    machine:
        $Analyzing {
            analyze(text: str): str {
                // Early return path with system.return
                if (text == "") {
                    system.return = "empty input";
                    return;
                }

                // Call action for processing
                var category = categorizeText(text);
                console.log("Category from action: " + category);

                // Set final interface return based on category
                if (category == "short") {
                    system.return = "short text: " + text;
                } else if (category == "caps") {
                    system.return = "LOUD TEXT: " + text;
                } else {
                    system.return = "normal text: " + text;
                }

                return;
            }
        }

    actions:
        categorizeText(text: str): str {
            // Regular returns in actions
            if (len(text) < 10) {
                return "short";
            }
            if (text == text.toUpperCase() && text != text.toLowerCase()) {
                return "caps";
            }
            return "normal";
        }

        // Helper action
        len(s: str): int {
            var count = 0
            for c in s {
                count = count + 1
            }
            return count
        }
}


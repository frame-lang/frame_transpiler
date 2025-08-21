// Test: Complex if/elif/else chains with returns in event handlers
// This was the original issue that was fixed in v0.20
fn main() {
    var grader = GradeSystem()
    
    // Test various scores
    var scores = [95, 85, 75, 65, 55, -5, 105]
    
    print("=== Grade System Tests ===")
    for score in scores {
        var grade = grader.calculateGrade(score)
        print("Score " + str(score) + " -> Grade: " + grade)
    }
    
    print("\n=== Category Tests ===")
    grader.testCategories()
}

system GradeSystem {
    interface:
        calculateGrade(score: int): str
        testCategories()
        
    machine:
        $Processing {
            calculateGrade(score: int): str {
                // Complex nested if/elif/else with returns - this was broken before v0.20
                if score < 0 {
                    return = "Invalid: negative score"
                    return
                } elif score > 100 {
                    return = "Invalid: score too high"
                    return
                }
                
                // Grade calculation with exhaustive if/elif/else
                if score >= 90 {
                    if score >= 95 {
                        return = "A+"
                        return
                    } elif score >= 93 {
                        return = "A"
                        return
                    } else {
                        return = "A-"
                        return
                    }
                } elif score >= 80 {
                    if score >= 87 {
                        return = "B+"
                        return
                    } elif score >= 83 {
                        return = "B"
                        return
                    } else {
                        return = "B-"
                        return
                    }
                } elif score >= 70 {
                    if score >= 77 {
                        return = "C+"
                        return
                    } elif score >= 73 {
                        return = "C"
                        return
                    } else {
                        return = "C-"
                        return
                    }
                } elif score >= 60 {
                    return = "D"
                    return
                } else {
                    return = "F"
                    return
                }
            }
            
            testCategories() {
                // Test different category patterns
                var testValue = 42
                
                if testValue < 25 {
                    print("Low value")
                    return
                } elif testValue < 50 {
                    print("Medium value: " + str(testValue))
                    return
                } elif testValue < 75 {
                    print("High value")
                    return
                } else {
                    print("Very high value")
                    return
                }
            }
        }
}
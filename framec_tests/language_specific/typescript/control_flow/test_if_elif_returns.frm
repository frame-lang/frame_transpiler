# Language-specific (TypeScript) version of the complex if/else chains test

fn main() {
    var grader = GradeSystem()
    
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
                if (score < 0) {
                    system.return = "Invalid: negative score"
                    return
                } else if (score > 100) {
                    system.return = "Invalid: score too high"
                    return
                }
                
                if (score >= 90) {
                    if (score >= 95) {
                        system.return = "A+"
                        return
                    } else if (score >= 93) {
                        system.return = "A"
                        return
                    } else {
                        system.return = "A-"
                        return
                    }
                } else if (score >= 80) {
                    if (score >= 87) {
                        system.return = "B+"
                        return
                    } else if (score >= 83) {
                        system.return = "B"
                        return
                    } else {
                        system.return = "B-"
                        return
                    }
                } else if (score >= 70) {
                    if (score >= 77) {
                        system.return = "C+"
                        return
                    } else if (score >= 73) {
                        system.return = "C"
                        return
                    } else {
                        system.return = "C-"
                        return
                    }
                } else if (score >= 60) {
                    system.return = "D"
                    return
                } else {
                    system.return = "F"
                    return
                }
            }
            
            testCategories() {
                var testValue = 42
                
                if (testValue < 25) {
                    print("Low value")
                    return
                } else if (testValue < 50) {
                    print("Medium value: " + str(testValue))
                    return
                } else if (testValue < 75) {
                    print("High value")
                    return
                } else {
                    print("Very high value")
                    return
                }
            }
        }
}


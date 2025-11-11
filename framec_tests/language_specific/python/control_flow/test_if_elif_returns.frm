@target python

# Language-specific (Python) version of the complex if/elif/else chains test

fn main() {
    grader = GradeSystem()
    
    # Test various scores
    scores = [95, 85, 75, 65, 55, -5, 105]
    
    print("=== Grade System Tests ===")
    for score in scores:
        grade = grader.calculateGrade(score)
        print("Score " + str(score) + " -> Grade: " + grade)
    
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
                # Native Python if/elif with returns
                if score < 0:
                    system.return = "Invalid: negative score"
                    return
                elif score > 100:
                    system.return = "Invalid: score too high"
                    return
                
                # Grade calculation with exhaustive if/elif/else
                if score >= 90:
                    if score >= 95:
                        system.return = "A+"
                        return
                    elif score >= 93:
                        system.return = "A"
                        return
                    else:
                        system.return = "A-"
                        return
                elif score >= 80:
                    if score >= 87:
                        system.return = "B+"
                        return
                    elif score >= 83:
                        system.return = "B"
                        return
                    else:
                        system.return = "B-"
                        return
                elif score >= 70:
                    if score >= 77:
                        system.return = "C+"
                        return
                    elif score >= 73:
                        system.return = "C"
                        return
                    else:
                        system.return = "C-"
                        return
                elif score >= 60:
                    system.return = "D"
                    return
                else:
                    system.return = "F"
                    return
            }
            
            testCategories() {
                # Test different category patterns (native Python)
                testValue = 42
                
                if testValue < 25:
                    print("Low value")
                    return
                elif testValue < 50:
                    print("Medium value: " + str(testValue))
                    return
                elif testValue < 75:
                    print("High value")
                    return
                else:
                    print("Very high value")
                    return
            }
        }
}


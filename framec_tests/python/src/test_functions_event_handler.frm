fn main() {
    var grader = GradeProcessor()
    var grade = grader.processScore(85)
    print("Grade: " + grade)
}

system GradeProcessor {
    interface:
        processScore(score: int): string

    machine:
        $Start {
            processScore(score: int): string {
                // Validate input
                if score < 0 {
                    return "Invalid"
                } elif score > 100 {
                    return "Invalid"
                }
                
                // Calculate letter grade
                if score >= 90 {
                    return "A"
                } elif score >= 80 {
                    return "B"
                } elif score >= 70 {
                    return "C"
                } elif score >= 60 {
                    return "D"
                } else {
                    return "F"
                }
            }
        }
}
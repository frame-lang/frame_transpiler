fn main() {
    var result = add(5, 3)
    print("5 + 3 = " + str(result))
    
    var category = categorizeNumber(42)
    print("42 is " + category)
}

system Utils {
    actions:
        add(x: int, y: int): int {
            return x + y
        }

        categorizeNumber(num: int): string {
            if num < 0 {
                return "negative"
            } elif num == 0 {
                return "zero"
            } elif num < 10 {
                return "single digit"
            } elif num < 100 {
                return "double digit"
            } else {
                return "large number"
            }
        }
}
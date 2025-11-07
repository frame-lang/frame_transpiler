# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
fn main() {
    result = Utils.add(5, 3)
    print("5 + 3 = " + str(result))
    
    category = Utils.categorizeNumber(42)
    print("42 is " + category)
}

system Utils {
    operations:
        @staticmethod
        add(x: int, y: int): int {
            return x + y
        }

        @staticmethod
        categorizeNumber(num: int): string {
            if num < 0:
                return "negative"
            elif num == 0:
                return "zero"
            elif num < 10:
                return "single digit"
            elif num < 100:
                return "double digit"
            else:
                return "large number"
            }
        }
}

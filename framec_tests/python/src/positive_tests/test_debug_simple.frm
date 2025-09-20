


# Comprehensive debugging test - covers ALL statements and expressions (except classes/systems)
# Tests various permutations and edge cases for complete debugging coverage

fn main() {
    print("=== Starting Comprehensive Debug Test ===")
    
    # ========== VARIABLE OPERATIONS ==========
    # Basic variable declarations
    var x = 101
    print("Variable declaration: x = " + str(x))
    
    var y = 20
    var z = 30.5
    var flag = true
    var name = "Frame"
    print("Multiple types: y=" + str(y) + ", z=" + str(z) + ", flag=" + str(flag) + ", name=" + name)
    
    # Multiple variable assignment
    var a, b = divmod_custom(17, 5)
    print("Multiple assignment: a=" + str(a) + ", b=" + str(b))
    
    # Tuple unpacking
    var t1, t2, t3 = unpack_three()
    print("Tuple unpacking: t1=" + str(t1) + ", t2=" + str(t2) + ", t3=" + str(t3))
    
    # Reassignment
    x = 42
    print("Reassignment: x now = " + str(x))
    
    # ========== ARITHMETIC EXPRESSIONS ==========
    # Basic arithmetic
    var sum = x + y
    var diff = x - y  
    var prod = x * y
    var quot = x / y
    var mod = x % y
    var power = x ** 2
    var floor_div = x // y
    print("Arithmetic: sum=" + str(sum) + ", diff=" + str(diff) + ", prod=" + str(prod))
    print("More math: quot=" + str(quot) + ", mod=" + str(mod) + ", power=" + str(power) + ", floor=" + str(floor_div))
    
    # Compound expressions
    var complex1 = (x + y) * (x - y)
    var complex2 = x + y * z - power / 2
    var complex3 = ((x + y) * z) / (a + b)
    print("Complex expressions: c1=" + str(complex1) + ", c2=" + str(complex2) + ", c3=" + str(complex3))
    
    # ========== COMPARISON OPERATIONS ==========
    var gt = x > y
    var lt = x < y
    var gte = x >= y
    var lte = x <= y
    var eq = x == y
    var neq = x != y
    print("Comparisons: x>y=" + str(gt) + ", x<y=" + str(lt) + ", x>=y=" + str(gte))
    print("More comparisons: x<=y=" + str(lte) + ", x==y=" + str(eq) + ", x!=y=" + str(neq))
    
    # Chained comparisons
    var chained = 10 < x < 100
    print("Chained comparison 10 < x < 100: " + str(chained))
    
    # ========== LOGICAL OPERATIONS ==========
    var and1 = true and true
    var and2 = true and false
    var or1 = false or true
    var or2 = false or false
    var not1 = not true
    var not2 = not false
    print("Logic: and(T,T)=" + str(and1) + ", and(T,F)=" + str(and2) + ", or(F,T)=" + str(or1))
    print("More logic: or(F,F)=" + str(or2) + ", not(T)=" + str(not1) + ", not(F)=" + str(not2))
    
    # Complex logical expressions
    var complex_logic = (x > y) and (not flag or (z < 100))
    print("Complex logic expression: " + str(complex_logic))
    
    # ========== BITWISE OPERATIONS ==========
    var bit_and = 5 & 3  # 101 & 011 = 001
    var bit_or = 5 | 3   # 101 | 011 = 111
    var bit_xor = 5 ^ 3  # 101 ^ 011 = 110
    var bit_not = ~5     # ~101 = ...11111010 (two's complement)
    var bit_left = 5 << 1  # 101 << 1 = 1010
    var bit_right = 5 >> 1 # 101 >> 1 = 10
    print("Bitwise: 5&3=" + str(bit_and) + ", 5|3=" + str(bit_or) + ", 5^3=" + str(bit_xor))
    print("More bitwise: ~5=" + str(bit_not) + ", 5<<1=" + str(bit_left) + ", 5>>1=" + str(bit_right))
    
    # ========== STRING OPERATIONS ==========
    var str1 = "Hello"
    var str2 = "World"
    var concat = str1 + " " + str2
    var repeated = str1 * 3
    print("String concat: " + concat)
    print("String repeat: " + repeated)
    
    # String formatting
    var formatted = "x=" + str(x) + ", y=" + str(y)
    print("Formatted string: " + formatted)
    
    # ========== LIST OPERATIONS ==========
    var myList = [1, 2, 3, 4, 5]
    print("Original list: " + str(myList))
    
    # List access
    var first = myList[0]
    var last = myList[-1]
    print("List access: first=" + str(first) + ", last=" + str(last))
    
    # List slicing
    var slice1 = myList[1:3]
    var slice2 = myList[::2]
    var slice3 = myList[::-1]
    print("Slices: [1:3]=" + str(slice1) + ", [::2]=" + str(slice2) + ", [::-1]=" + str(slice3))
    
    # List modification
    myList.append(6)
    print("After append(6): " + str(myList))
    
    myList[0] = 100
    print("After myList[0]=100: " + str(myList))
    
    # List comprehension-like operations
    var doubled = []
    for item in myList {
        doubled.append(item * 2)
    }
    print("Doubled list: " + str(doubled))
    
    # ========== DICTIONARY OPERATIONS ==========
    var myDict = {"key1": "value1", "key2": 42, "key3": true}
    print("Original dict: " + str(myDict))
    
    # Dictionary access
    var val1 = myDict["key1"]
    print("Dict access: myDict['key1']=" + str(val1))
    
    # Dictionary modification
    myDict["key4"] = [1, 2, 3]
    print("After adding key4: " + str(myDict))
    
    # Dictionary deletion
    del myDict["key2"]
    print("After del key2: " + str(myDict))
    
    # ========== SET OPERATIONS ==========
    var set1 = {1, 2, 3}
    var set2 = {2, 3, 4}
    print("Sets: set1=" + str(set1) + ", set2=" + str(set2))
    
    # ========== MEMBERSHIP OPERATIONS ==========
    var in_list = 3 in myList
    var not_in_list = 999 in myList
    var in_dict = "key1" in myDict
    var in_set = 2 in set1
    print("Membership: 3 in list=" + str(in_list) + ", 999 in list=" + str(not_in_list))
    print("More membership: 'key1' in dict=" + str(in_dict) + ", 2 in set=" + str(in_set))
    
    # ========== CONTROL FLOW - IF/ELSE ==========
    if x > y {
        print("IF: x > y is true")
    } else {
        print("IF: x > y is false")
    }
    
    # Nested if
    if x > 0 {
        if y > 0 {
            print("NESTED IF: Both x and y are positive")
            if z > 0 {
                print("TRIPLE NESTED: x, y, and z are all positive")
            }
        }
    }
    
    # elif chains
    if x < 10 {
        print("ELIF: x < 10")
    } elif x < 50 {
        print("ELIF: x < 50")
    } elif x < 100 {
        print("ELIF: x < 100")
    } else {
        print("ELIF: x >= 100")
    }
    
    # ========== LOOPS - WHILE ==========
    var i = 0
    while i < 3 {
        print("WHILE loop: i=" + str(i))
        i = i + 1
    }
    
    # Nested while
    var j = 0
    while j < 2 {
        var k = 0
        while k < 2 {
            print("NESTED WHILE: j=" + str(j) + ", k=" + str(k))
            k = k + 1
        }
        j = j + 1
    }
    
    # ========== LOOPS - FOR ==========
    for n in range(3) {
        print("FOR range: n=" + str(n))
    }
    
    for item in myList {
        print("FOR list: item=" + str(item))
    }
    
    for key in myDict {
        print("FOR dict key: " + key + "=" + str(myDict[key]))
    }
    
    # Nested for
    for m in range(2) {
        for p in range(2) {
            print("NESTED FOR: m=" + str(m) + ", p=" + str(p))
        }
    }
    
    # ========== BREAK STATEMENT ==========
    var count = 0
    while true {
        if count == 3 {
            print("BREAK: Breaking at count=" + str(count))
            break
        }
        count = count + 1
    }
    
    # Break in nested loop
    for outer in range(3) {
        for inner in range(3) {
            if outer == 1 and inner == 1 {
                print("BREAK: Breaking inner loop at outer=" + str(outer) + ", inner=" + str(inner))
                break
            }
            print("NESTED BREAK: outer=" + str(outer) + ", inner=" + str(inner))
        }
    }
    
    # ========== CONTINUE STATEMENT ==========
    for q in range(5) {
        if q == 2 {
            print("CONTINUE: Skipping q=" + str(q))
            continue
        }
        print("CONTINUE loop: q=" + str(q))
    }
    
    # ========== PASS STATEMENT ==========
    if true {
        pass  # Do nothing
    }
    
    for r in range(1) {
        pass  # Empty loop body
    }
    
    # ========== ASSERT STATEMENT ==========
    assert x == 42
    assert y > 0
    assert len(myList) > 0
    print("ASSERT: All assertions passed")
    
    # ========== TRY-EXCEPT-FINALLY ==========
    try {
        print("TRY: Attempting division")
        var result = safe_divide(10, 2)
        print("TRY: Division result = " + str(result))
    } except {
        print("EXCEPT: Division failed")
    } finally {
        print("FINALLY: Cleanup after division")
    }
    
    # Try with actual exception
    try {
        print("TRY: Attempting risky operation")
        var result = safe_divide(10, 0)  # Will throw
        print("TRY: This should not print")
    } except {
        print("EXCEPT: Caught division by zero")
    } finally {
        print("FINALLY: Cleanup after risky operation")
    }
    
    # Nested try
    try {
        print("OUTER TRY: Starting")
        try {
            print("INNER TRY: Starting")
            throw_if_negative(-1)
        } except {
            print("INNER EXCEPT: Caught inner exception")
        }
        print("OUTER TRY: Continuing after inner")
    } except {
        print("OUTER EXCEPT: Should not reach here")
    } finally {
        print("OUTER FINALLY: Done with nested try")
    }
    
    # ========== FUNCTION CALLS ==========
    # Simple function call
    var add_result = add(10, 20)
    print("FUNCTION: add(10, 20) = " + str(add_result))
    
    # Function with multiple returns
    var div, rem = divmod_custom(17, 5)
    print("FUNCTION: divmod(17, 5) = (" + str(div) + ", " + str(rem) + ")")
    
    # Recursive function
    var fact = factorial(5)
    print("RECURSIVE: factorial(5) = " + str(fact))
    
    # Function with side effects
    greet("Alice", "Hello")
    
    # Nested function calls
    var nested_result = add(add(1, 2), add(3, 4))
    print("NESTED CALLS: add(add(1,2), add(3,4)) = " + str(nested_result))
    
    # Function returning function result
    var chain = process_chain(5)
    print("FUNCTION CHAIN: process_chain(5) = " + str(chain))
    
    # ========== EXCEPTION THROWING ==========
    # Test exception propagation
    try {
        print("Testing exception from function")
        test_exception_thrower(true)
        print("This line should not execute")
    } except {
        print("Caught exception from function")
    }
    
    # Test no exception case
    try {
        print("Testing no exception case")
        test_exception_thrower(false)
        print("No exception was thrown")
    } except {
        print("This should not execute")
    }
    
    # ========== COMPLEX EXPRESSIONS ==========
    # Conditional assignment (ternary not supported, use if-else)
    var ternary = 0
    if x > y {
        ternary = x
    } else {
        ternary = y
    }
    print("Conditional assignment: " + str(ternary))
    
    # Complex nested expressions
    var complex_expr = ((x + y) * z / (a + b)) ** 2 + sum - diff * prod
    print("Complex expression: " + str(complex_expr))
    
    # Boolean short-circuit evaluation
    var short1 = false and crash_function()  # Should not call crash_function
    var short2 = true or crash_function()    # Should not call crash_function
    print("Short-circuit evaluation completed successfully")
    
    # ========== EDGE CASES ==========
    # Empty collections
    var empty_list = []
    var empty_dict = {}
    print("Empty collections: list=" + str(empty_list) + ", dict=" + str(empty_dict))
    
    # Single element collections
    var single_list = [42]
    var single_dict = {"only": "one"}
    print("Single element: list=" + str(single_list) + ", dict=" + str(single_dict))
    
    # Large numbers
    var large = 999999999
    var very_large = large * large
    print("Large numbers: " + str(large) + " * " + str(large) + " = " + str(very_large))
    
    # Negative numbers
    var neg = -42
    var neg_result = neg * neg
    print("Negative: " + str(neg) + " * " + str(neg) + " = " + str(neg_result))
    
    # Float precision
    var float1 = 0.1
    var float2 = 0.2
    var float_sum = float1 + float2
    print("Float precision: 0.1 + 0.2 = " + str(float_sum))
    
    # ========== CLASS TESTING ==========
    var class_result = test_classes()
    print("Class test result (GPA): " + str(class_result))
    
    print("=== All Tests Completed Successfully ===")
    return
}

# ========== HELPER FUNCTIONS ==========

fn add(a, b) {
    print("  [add] Adding " + str(a) + " + " + str(b))
    return a + b
}

fn divmod_custom(dividend, divisor) {
    print("  [divmod] Dividing " + str(dividend) + " by " + str(divisor))
    var quotient = dividend // divisor
    var remainder = dividend % divisor
    return quotient, remainder
}

fn unpack_three() {
    print("  [unpack_three] Returning three values")
    return 100, 200, 300
}

fn greet(name, greeting) {
    print("  [greet] " + greeting + ", " + name + "!")
    return greeting + ", " + name
}

fn factorial(n) {
    print("  [factorial] Computing factorial(" + str(n) + ")")
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

fn sum_list(numbers) {
    print("  [sum_list] Summing list: " + str(numbers))
    var total = 0
    for val in numbers {
        total = total + val
    }
    return total
}

fn process_chain(val) {
    print("  [process_chain] Processing " + str(val))
    var step1 = val * 2
    var step2 = step1 + 10
    var step3 = step2 / 2
    return step3
}

fn safe_divide(a, b) {
    print("  [safe_divide] Dividing " + str(a) + " by " + str(b))
    if b == 0 {
        print("  [safe_divide] ERROR: Division by zero!")
        var error = "Division by zero"
        throw error
    }
    return a / b
}

fn throw_if_negative(val) {
    print("  [throw_if_negative] Checking " + str(val))
    if val < 0 {
        print("  [throw_if_negative] ERROR: Negative value!")
        var error = "Negative value not allowed"
        throw error
    }
    return val
}

fn test_exception_thrower(should_throw) {
    print("  [test_exception_thrower] should_throw=" + str(should_throw))
    if should_throw {
        print("  [test_exception_thrower] Throwing exception")
        var error = "Test exception from function"
        throw error
    }
    print("  [test_exception_thrower] Returning normally")
    return "success"
}

fn crash_function() {
    print("  ERROR: This function should never be called due to short-circuit!")
    var error = "Should not reach here"
    throw error
    return false
}

# ========== CLASS DEFINITIONS ==========

class Person {
    var name = ""
    var age = 0
    var email = ""
    
    fn __init__(self, name, age, email) {
        self.name = name
        self.age = age
        self.email = email
        print("  [Person.__init__] Created person: " + name)
    }
    
    fn greet(self) {
        print("  [Person.greet] Hello, I'm " + self.name + " and I'm " + str(self.age) + " years old")
        return "Greeting from " + self.name
    }
    
    fn get_info(self) {
        print("  [Person.get_info] Getting info for " + self.name)
        return {"name": self.name, "age": self.age, "email": self.email}
    }
    
    fn have_birthday(self) {
        print("  [Person.have_birthday] " + self.name + " is having a birthday!")
        self.age = self.age + 1
        return self.age
    }
}

class Student {
    var name = ""
    var age = 0
    var email = ""
    var student_id = ""
    var grades = []
    var courses = []
    
    fn __init__(self, name, age, email, student_id) {
        self.name = name
        self.age = age
        self.email = email
        self.student_id = student_id
        self.grades = []
        self.courses = []
        print("  [Student.__init__] Created student with ID: " + student_id)
    }
    
    fn enroll(self, course) {
        print("  [Student.enroll] " + self.name + " enrolling in " + course)
        self.courses.append(course)
        return len(self.courses)
    }
    
    fn add_grade(self, course, grade) {
        print("  [Student.add_grade] Adding grade " + str(grade) + " for " + course)
        self.grades.append({"course": course, "grade": grade})
        return self.calculate_gpa()
    }
    
    fn calculate_gpa(self) {
        print("  [Student.calculate_gpa] Calculating GPA")
        if len(self.grades) == 0 {
            return 0.0
        }
        var total = 0
        for grade_record in self.grades {
            total = total + grade_record["grade"]
        }
        var gpa = total / len(self.grades)
        return gpa
    }
    
    fn greet(self) {
        # Override parent method
        print("  [Student.greet] Hi! I'm student " + self.name + " (ID: " + self.student_id + ")")
        return "Student greeting from " + self.name
    }
}

class Calculator {
    var memory = 0
    var history = []
    
    fn __init__(self) {
        self.memory = 0
        self.history = []
        print("  [Calculator.__init__] Calculator initialized")
    }
    
    fn add(self, a, b) {
        print("  [Calculator.add] Adding " + str(a) + " + " + str(b))
        var result = a + b
        self.history.append("add(" + str(a) + ", " + str(b) + ") = " + str(result))
        return result
    }
    
    fn multiply(self, a, b) {
        print("  [Calculator.multiply] Multiplying " + str(a) + " * " + str(b))
        var result = a * b
        self.history.append("multiply(" + str(a) + ", " + str(b) + ") = " + str(result))
        return result
    }
    
    fn store(self, value) {
        print("  [Calculator.store] Storing " + str(value) + " in memory")
        self.memory = value
        return self.memory
    }
    
    fn recall(self) {
        print("  [Calculator.recall] Recalling from memory: " + str(self.memory))
        return self.memory
    }
    
    fn clear_history(self) {
        print("  [Calculator.clear_history] Clearing history")
        var old_count = len(self.history)
        self.history = []
        return old_count
    }
}

fn test_classes() {
    print("=== Testing Classes ===")
    
    # Test basic class instantiation
    var person1 = Person("Alice", 30, "alice@example.com")
    person1.greet()
    var info = person1.get_info()
    print("Person info: " + str(info))
    
    # Test method calls and property access
    var new_age = person1.have_birthday()
    print("New age after birthday: " + str(new_age))
    
    # Test inheritance
    var student1 = Student("Bob", 20, "bob@university.edu", "STU001")
    student1.greet()  # Should use overridden method
    
    # Test methods on inherited class
    student1.enroll("Computer Science")
    student1.enroll("Mathematics")
    student1.add_grade("Computer Science", 85)
    student1.add_grade("Mathematics", 90)
    var gpa = student1.calculate_gpa()
    print("Student GPA: " + str(gpa))
    
    # Test another class
    var calc = Calculator()
    var sum_result = calc.add(10, 20)
    var mult_result = calc.multiply(5, 6)
    calc.store(sum_result)
    var recalled = calc.recall()
    print("Calculator memory: " + str(recalled))
    print("Calculator history: " + str(calc.history))
    
    # Test multiple instances
    var person2 = Person("Charlie", 25, "charlie@example.com")
    var student2 = Student("Diana", 22, "diana@university.edu", "STU002")
    
    person2.greet()
    student2.greet()
    
    # Test instance variables are separate
    student2.enroll("Physics")
    print("Bob's courses: " + str(student1.courses))
    print("Diana's courses: " + str(student2.courses))
    
    print("=== Class Testing Complete ===")
    return gpa
}
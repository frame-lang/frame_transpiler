# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION



# Comprehensive debugging test - covers ALL statements and expressions (except classes/systems)
# Tests various permutations and edge cases for complete debugging coverage

fn main() {
    print("=== Starting Comprehensive Debug Test ===")
    
    # ========== VARIABLE OPERATIONS ==========
    # Basic variable declarations
    x = 101
    print("Variable declaration: x = " + str(x))
    
    y = 20
    z = 30.5
    flag = true
    name = "Frame"
    print("Multiple types: y=" + str(y) + ", z=" + str(z) + ", flag=" + str(flag) + ", name=" + name)
    
    # Multiple variable assignment
    a, b = divmod_custom(17, 5)
    print("Multiple assignment: a=" + str(a) + ", b=" + str(b))
    
    # Tuple unpacking
    t1, t2, t3 = unpack_three()
    print("Tuple unpacking: t1=" + str(t1) + ", t2=" + str(t2) + ", t3=" + str(t3))
    
    # Reassignment
    x = 42
    print("Reassignment: x now = " + str(x))
    
    # ========== ARITHMETIC EXPRESSIONS ==========
    # Basic arithmetic
    sum = x + y
    diff = x - y  
    prod = x * y
    quot = x / y
    mod = x % y
    power = x ** 2
    floor_div = x // y
    print("Arithmetic: sum=" + str(sum) + ", diff=" + str(diff) + ", prod=" + str(prod))
    print("More math: quot=" + str(quot) + ", mod=" + str(mod) + ", power=" + str(power) + ", floor=" + str(floor_div))
    
    # Compound expressions
    complex1 = (x + y) * (x - y)
    complex2 = x + y * z - power / 2
    complex3 = ((x + y) * z) / (a + b)
    print("Complex expressions: c1=" + str(complex1) + ", c2=" + str(complex2) + ", c3=" + str(complex3))
    
    # ========== COMPARISON OPERATIONS ==========
    gt = x > y
    lt = x < y
    gte = x >= y
    lte = x <= y
    eq = x == y
    neq = x != y
    print("Comparisons: x>y=" + str(gt) + ", x<y=" + str(lt) + ", x>=y=" + str(gte))
    print("More comparisons: x<=y=" + str(lte) + ", x==y=" + str(eq) + ", x!=y=" + str(neq))
    
    # Chained comparisons
    chained = 10 < x < 100
    print("Chained comparison 10 < x < 100: " + str(chained))
    
    # ========== LOGICAL OPERATIONS ==========
    and1 = true and true
    and2 = true and false
    or1 = false or true
    or2 = false or false
    not1 = not true
    not2 = not false
    print("Logic: and(T,T)=" + str(and1) + ", and(T,F)=" + str(and2) + ", or(F,T)=" + str(or1))
    print("More logic: or(F,F)=" + str(or2) + ", not(T)=" + str(not1) + ", not(F)=" + str(not2))
    
    # Complex logical expressions
    complex_logic = (x > y) and (not flag or (z < 100))
    print("Complex logic expression: " + str(complex_logic))
    
    # ========== BITWISE OPERATIONS ==========
    bit_and = 5 & 3  # 101 & 011 = 001
    bit_or = 5 | 3   # 101 | 011 = 111
    bit_xor = 5 ^ 3  # 101 ^ 011 = 110
    bit_not = ~5     # ~101 = ...11111010 (two's complement)
    bit_left = 5 << 1  # 101 << 1 = 1010
    bit_right = 5 >> 1 # 101 >> 1 = 10
    print("Bitwise: 5&3=" + str(bit_and) + ", 5|3=" + str(bit_or) + ", 5^3=" + str(bit_xor))
    print("More bitwise: ~5=" + str(bit_not) + ", 5<<1=" + str(bit_left) + ", 5>>1=" + str(bit_right))
    
    # ========== STRING OPERATIONS ==========
    str1 = "Hello"
    str2 = "World"
    concat = str1 + " " + str2
    repeated = str1 * 3
    print("String concat: " + concat)
    print("String repeat: " + repeated)
    
    # String formatting
    formatted = "x=" + str(x) + ", y=" + str(y)
    print("Formatted string: " + formatted)
    
    # ========== LIST OPERATIONS ==========
    myList = [1, 2, 3, 4, 5]
    print("Original list: " + str(myList))
    
    # List access
    first = myList[0]
    last = myList[-1]
    print("List access: first=" + str(first) + ", last=" + str(last))
    
    # List slicing
    slice1 = myList[1:3]
    slice2 = myList[::2]
    slice3 = myList[::-1]
    print("Slices: [1:3]=" + str(slice1) + ", [::2]=" + str(slice2) + ", [::-1]=" + str(slice3))
    
    # List modification
    myList.append(6)
    print("After append(6): " + str(myList))
    
    myList[0] = 100
    print("After myList[0]=100: " + str(myList))
    
    # List comprehension-like operations
    doubled = []
    for item in myList:
        doubled.append(item * 2)
    print("Doubled list: " + str(doubled))
    
    # ========== DICTIONARY OPERATIONS ==========
    myDict = {"key1": "value1", "key2": 42, "key3": true}
    print("Original dict: " + str(myDict))
    
    # Dictionary access
    val1 = myDict["key1"]
    print("Dict access: myDict['key1']=" + str(val1))
    
    # Dictionary modification
    myDict["key4"] = [1, 2, 3]
    print("After adding key4: " + str(myDict))
    
    # Dictionary deletion
    del myDict["key2"]
    print("After del key2: " + str(myDict))
    
    # ========== SET OPERATIONS ==========
    set1 = {1, 2, 3}
    set2 = {2, 3, 4}
    print("Sets: set1=" + str(set1) + ", set2=" + str(set2))
    
    # ========== MEMBERSHIP OPERATIONS ==========
    in_list = 3 in myList
    not_in_list = 999 in myList
    in_dict = "key1" in myDict
    in_set = 2 in set1
    print("Membership: 3 in list=" + str(in_list) + ", 999 in list=" + str(not_in_list))
    print("More membership: 'key1' in dict=" + str(in_dict) + ", 2 in set=" + str(in_set))
    
    # ========== CONTROL FLOW - IF/ELSE ==========
    if x > y:
        print("IF: x > y is true")
    else:
        print("IF: x > y is false")
    }
    
    # Nested if
    if x > 0:
        if y > 0:
            print("NESTED IF: Both x and y are positive")
            if z > 0:
                print("TRIPLE NESTED: x, y, and z are all positive")
    
    # elif chains
    if x < 10:
        print("ELIF: x < 10")
    elif x < 50:
        print("ELIF: x < 50")
    elif x < 100:
        print("ELIF: x < 100")
    else:
        print("ELIF: x >= 100")
    }
    
    # ========== LOOPS - WHILE ==========
    i = 0
    while i < 3:
        print("WHILE loop: i=" + str(i))
        i = i + 1
    
    # Nested while
    j = 0
    while j < 2:
        k = 0
        while k < 2:
            print("NESTED WHILE: j=" + str(j) + ", k=" + str(k))
            k = k + 1
        j = j + 1
    
    # ========== LOOPS - FOR ==========
    for n in range(3):
        print("FOR range: n=" + str(n))
    
    for item in myList:
        print("FOR list: item=" + str(item))
    
    for key in myDict:
        print("FOR dict key: " + key + "=" + str(myDict[key]))
    
    # Nested for
    for m in range(2):
        for p in range(2):
            print("NESTED FOR: m=" + str(m) + ", p=" + str(p))
    
    # ========== BREAK STATEMENT ==========
    count = 0
    while true:
        if count == 3:
            print("BREAK: Breaking at count=" + str(count))
            break
        count = count + 1
    
    # Break in nested loop
    for outer in range(3):
        for inner in range(3):
            if outer == 1 and inner == 1:
                print("BREAK: Breaking inner loop at outer=" + str(outer) + ", inner=" + str(inner))
                break
            print("NESTED BREAK: outer=" + str(outer) + ", inner=" + str(inner))
    
    # ========== CONTINUE STATEMENT ==========
    for q in range(5):
        if q == 2:
            print("CONTINUE: Skipping q=" + str(q))
            continue
        print("CONTINUE loop: q=" + str(q))
    
    # ========== PASS STATEMENT ==========
    if true:
        pass  # Do nothing
    
    for r in range(1):
        pass  # Empty loop body
    
    # ========== ASSERT STATEMENT ==========
    assert x == 42
    assert y > 0
    assert len(myList) > 0
    print("ASSERT: All assertions passed")
    
    # ========== TRY-EXCEPT-FINALLY ==========
    try:
        print("TRY: Attempting division")
        result1 = safe_divide(10, 2)
        print("TRY: Division result = " + str(result1))
    } except {
        print("EXCEPT: Division failed")
    } finally {
        print("FINALLY: Cleanup after division")
    
    # Try with actual exception
    try:
        print("TRY: Attempting risky operation")
        result2 = safe_divide(10, 0)  # Will throw
        print("TRY: This should not print")
    } except {
        print("EXCEPT: Caught division by zero")
    } finally {
        print("FINALLY: Cleanup after risky operation")
    
    # Nested try
    try:
        print("OUTER TRY: Starting")
        try:
            print("INNER TRY: Starting")
            throw_if_negative(-1)
        } except {
            print("INNER EXCEPT: Caught inner exception")
        print("OUTER TRY: Continuing after inner")
    } except {
        print("OUTER EXCEPT: Should not reach here")
    } finally {
        print("OUTER FINALLY: Done with nested try")
    
    # ========== FUNCTION CALLS ==========
    # Simple function call
    add_result = add(10, 20)
    print("FUNCTION: add(10, 20) = " + str(add_result))
    
    # Function with multiple returns
    div, rem = divmod_custom(17, 5)
    print("FUNCTION: divmod(17, 5) = (" + str(div) + ", " + str(rem) + ")")
    
    # Recursive function
    fact = factorial(5)
    print("RECURSIVE: factorial(5) = " + str(fact))
    
    # Function with side effects
    greet("Alice", "Hello")
    
    # Nested function calls
    nested_result = add(add(1, 2), add(3, 4))
    print("NESTED CALLS: add(add(1,2), add(3,4)) = " + str(nested_result))
    
    # Function returning function result
    chain = process_chain(5)
    print("FUNCTION CHAIN: process_chain(5) = " + str(chain))
    
    # ========== EXCEPTION THROWING ==========
    # Test exception propagation
    try:
        print("Testing exception from function")
        test_exception_thrower(true)
        print("This line should not execute")
    } except {
        print("Caught exception from function")
    
    # Test no exception case
    try:
        print("Testing no exception case")
        test_exception_thrower(false)
        print("No exception was thrown")
    } except {
        print("This should not execute")
    
    # ========== COMPLEX EXPRESSIONS ==========
    # Conditional assignment (ternary not supported, use if-else)
    ternary = 0
    if x > y:
        ternary = x
    else:
        ternary = y
    }
    print("Conditional assignment: " + str(ternary))
    
    # Complex nested expressions
    complex_expr = ((x + y) * z / (a + b)) ** 2 + sum - diff * prod
    print("Complex expression: " + str(complex_expr))
    
    # Boolean short-circuit evaluation
    short1 = false and crash_function()  # Should not call crash_function
    short2 = true or crash_function()    # Should not call crash_function
    print("Short-circuit evaluation completed successfully")
    
    # ========== EDGE CASES ==========
    # Empty collections
    empty_list = []
    empty_dict = {}
    print("Empty collections: list=" + str(empty_list) + ", dict=" + str(empty_dict))
    
    # Single element collections
    single_list = [42]
    single_dict = {"only": "one"}
    print("Single element: list=" + str(single_list) + ", dict=" + str(single_dict))
    
    # Large numbers
    large = 999999999
    very_large = large * large
    print("Large numbers: " + str(large) + " * " + str(large) + " = " + str(very_large))
    
    # Negative numbers
    neg = -42
    neg_result = neg * neg
    print("Negative: " + str(neg) + " * " + str(neg) + " = " + str(neg_result))
    
    # Float precision
    float1 = 0.1
    float2 = 0.2
    float_sum = float1 + float2
    print("Float precision: 0.1 + 0.2 = " + str(float_sum))
    
    # ========== CLASS TESTING ==========
    class_result = test_classes()
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
    quotient = dividend // divisor
    remainder = dividend % divisor
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
    if n <= 1:
        return 1
    return n * factorial(n - 1)
}

fn sum_list(numbers) {
    print("  [sum_list] Summing list: " + str(numbers))
    total = 0
    for val in numbers:
        total = total + val
    return total
}

fn process_chain(val) {
    print("  [process_chain] Processing " + str(val))
    step1 = val * 2
    step2 = step1 + 10
    step3 = step2 / 2
    return step3
}

fn safe_divide(a, b) {
    print("  [safe_divide] Dividing " + str(a) + " by " + str(b))
    if b == 0:
        print("  [safe_divide] ERROR: Division by zero!")
        error = "Division by zero"
        throw error
    return a / b
}

fn throw_if_negative(val) {
    print("  [throw_if_negative] Checking " + str(val))
    if val < 0:
        print("  [throw_if_negative] ERROR: Negative value!")
        error = "Negative value not allowed"
        throw error
    return val
}

fn test_exception_thrower(should_throw) {
    print("  [test_exception_thrower] should_throw=" + str(should_throw))
    if should_throw:
        print("  [test_exception_thrower] Throwing exception")
        error = "Test exception from function"
        throw error
    print("  [test_exception_thrower] Returning normally")
    return "success"
}

fn crash_function() {
    print("  ERROR: This function should never be called due to short-circuit!")
    error = "Should not reach here"
    throw error
    return false
}

# ========== CLASS DEFINITIONS ==========

class Person {
    name = ""
    age = 0
    email = ""
    
    fn init(name, age, email) {
        self.name = name
        self.age = age
        self.email = email
        print("  [Person.init] Created person: " + name)
    }
    
    fn greet() {
        print("  [Person.greet] Hello, I'm " + self.name + " and I'm " + str(self.age) + " years old")
        return "Greeting from " + self.name
    }
    
    fn get_info() {
        print("  [Person.get_info] Getting info for " + self.name)
        return {"name": self.name, "age": self.age, "email": self.email}
    }
    
    fn have_birthday() {
        print("  [Person.have_birthday] " + self.name + " is having a birthday!")
        self.age = self.age + 1
        return self.age
    }
}

class Student(Person) {
    student_id = ""
    grades = []
    courses = []
    
    fn init(name, age, email, student_id) {
        super.init(name, age, email)  # Call parent constructor
        self.student_id = student_id
        self.grades = []
        self.courses = []
        print("  [Student.init] Created student with ID: " + student_id)
    }
    
    fn enroll(course) {
        print("  [Student.enroll] " + self.name + " enrolling in " + course)
        self.courses.append(course)
        return len(self.courses)
    }
    
    fn add_grade(course, grade) {
        print("  [Student.add_grade] Adding grade " + str(grade) + " for " + course)
        self.grades.append({"course": course, "grade": grade})
        return self.calculate_gpa()
    }
    
    fn calculate_gpa() {
        print("  [Student.calculate_gpa] Calculating GPA")
        if len(self.grades) == 0:
            return 0.0
        total = 0
        for grade_record in self.grades:
            total = total + grade_record["grade"]
        gpa = total / len(self.grades)
        return gpa
    }
    
    fn greet() {
        # Override parent method
        print("  [Student.greet] Hi! I'm student " + self.name + " (ID: " + self.student_id + ")")
        return "Student greeting from " + self.name
    }
}

class Calculator {
    memory = 0
    history = []
    
    fn init() {
        self.memory = 0
        self.history = []
        print("  [Calculator.init] Calculator initialized")
    }
    
    fn add(a, b) {
        print("  [Calculator.add] Adding " + str(a) + " + " + str(b))
        add_result = a + b
        self.history.append("add(" + str(a) + ", " + str(b) + ") = " + str(add_result))
        return add_result
    }
    
    fn multiply(a, b) {
        print("  [Calculator.multiply] Multiplying " + str(a) + " * " + str(b))
        mult_result = a * b
        self.history.append("multiply(" + str(a) + ", " + str(b) + ") = " + str(mult_result))
        return mult_result
    }
    
    fn store(value) {
        print("  [Calculator.store] Storing " + str(value) + " in memory")
        self.memory = value
        return self.memory
    }
    
    fn recall() {
        print("  [Calculator.recall] Recalling from memory: " + str(self.memory))
        return self.memory
    }
    
    fn clear_history() {
        print("  [Calculator.clear_history] Clearing history")
        old_count = len(self.history)
        self.history = []
        return old_count
    }
}

fn test_classes() {
    print("=== Testing Classes ===")
    
    # Test basic class instantiation
    person1 = Person("Alice", 30, "alice@example.com")
    person1.greet()
    info = person1.get_info()
    print("Person info: " + str(info))
    
    # Test method calls and property access
    new_age = person1.have_birthday()
    print("New age after birthday: " + str(new_age))
    
    # Test inheritance
    student1 = Student("Bob", 20, "bob@university.edu", "STU001")
    student1.greet()  # Should use overridden method
    
    # Test methods on inherited class
    student1.enroll("Computer Science")
    student1.enroll("Mathematics")
    student1.add_grade("Computer Science", 85)
    student1.add_grade("Mathematics", 90)
    gpa = student1.calculate_gpa()
    print("Student GPA: " + str(gpa))
    
    # Test another class
    calc = Calculator()
    sum_result = calc.add(10, 20)
    mult_result = calc.multiply(5, 6)
    calc.store(sum_result)
    recalled = calc.recall()
    print("Calculator memory: " + str(recalled))
    print("Calculator history: " + str(calc.history))
    
    # Test multiple instances
    person2 = Person("Charlie", 25, "charlie@example.com")
    student2 = Student("Diana", 22, "diana@university.edu", "STU002")
    
    person2.greet()
    student2.greet()
    
    # Test instance variables are separate
    student2.enroll("Physics")
    print("Bob's courses: " + str(student1.courses))
    print("Diana's courses: " + str(student2.courses))
    
    print("=== Class Testing Complete ===")
    return gpa
}

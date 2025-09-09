# Test features that work without backticks
import math
import os

fn main() {
    print("=== Testing Working Features ===")
    
    # 1. Module member access
    var pi = math.pi
    print("1. math.pi = " + str(pi))
    
    # 2. Nested module access  
    var path = os.path.join("dir", "file.txt")
    print("2. os.path.join = " + path)
    
    # 3. Method chaining
    var text = "hello"
    var result = text.upper().replace("H", "J")
    print("3. Method chain = " + result)
    
    # 4. Nested indexing
    var matrix = [[1, 2], [3, 4]]
    var val = matrix[1][1]
    print("4. matrix[1][1] = " + str(val))
    
    # 5. List operations
    var list = [1, 2, 3]
    list.append(4)
    list[0] = 10
    print("5. List ops = " + str(list))
    
    print("\nConclusion: Most features work WITHOUT backticks!")
}
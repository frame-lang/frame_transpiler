# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test FSL list operations - Phase 2
# These should work WITHOUT backticks

fn main() {
    # Test list.append()
    var numbers = [1, 2, 3]
    numbers.append(4)
    numbers.append(5)
    print("After append:", numbers)  # [1, 2, 3, 4, 5]
    
    # Test list.pop()
    var items = [10, 20, 30, 40]
    var last = items.pop()
    print("Popped:", last)  # 40
    print("After pop:", items)  # [10, 20, 30]
    
    # Test list.clear()
    var temp = [1, 2, 3]
    temp.clear()
    print("After clear:", temp)  # []
    
    # Test list.length property
    var data = [5, 10, 15]
    var length = data.length
    print("Length:", length)  # 3
    
    # Test chaining operations
    var chain_test = []
    chain_test.append(1)
    chain_test.append(2)
    chain_test.append(3)
    var popped = chain_test.pop()
    print("Chain test after operations:", chain_test)  # [1, 2]
    print("Popped from chain:", popped)  # 3
    
    return
}
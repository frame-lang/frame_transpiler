# Test negative list indexing
# Python-style negative indices should work

fn main() {
    var numbers = [10, 20, 30, 40, 50]
    
    # Test negative indexing
    var last = numbers[-1]
    print("Last element [-1]:", last)  # 50
    
    var second_last = numbers[-2]
    print("Second to last [-2]:", second_last)  # 40
    
    var third_last = numbers[-3]
    print("Third from last [-3]:", third_last)  # 30
    
    # Test assignment with negative index
    numbers[-1] = 99
    print("After setting [-1] to 99:", numbers)  # [10, 20, 30, 40, 99]
    
    numbers[-3] = 77
    print("After setting [-3] to 77:", numbers)  # [10, 20, 77, 40, 99]
    
    # Test with expressions
    var index = -2
    var value = numbers[index]
    print("numbers[index] where index=-2:", value)  # 40
    
    # Test edge cases
    var small_list = [1, 2]
    var first_via_neg = small_list[-2]
    print("First element via [-2]:", first_via_neg)  # 1
    
    var last_via_neg = small_list[-1]
    print("Last element via [-1]:", last_via_neg)  # 2
    
    return
}
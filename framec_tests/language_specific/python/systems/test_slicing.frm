@target python

# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

fn main() {
    text = "Hello, World!"
    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    # Test basic slicing
    first_5 = text[:5]
    print("First 5 chars: " + first_5)
    
    last_6 = text[7:]
    print("Last 6 chars: " + last_6)
    
    middle = text[2:8]
    print("Middle chars: " + middle)
    
    # Test list slicing
    first_half = numbers[:5]
    print("First half: " + str(first_half))
    
    second_half = numbers[5:]
    print("Second half: " + str(second_half))
    
    middle_section = numbers[3:7]
    print("Middle section: " + str(middle_section))
    
    # Test step parameter
    every_other = numbers[::2]
    print("Every other: " + str(every_other))
    
    reverse = numbers[::-1]
    print("Reversed: " + str(reverse))
    
    skip_two = numbers[1:8:2]
    print("Skip two from 1 to 8: " + str(skip_two))
}

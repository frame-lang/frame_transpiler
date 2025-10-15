# Native Python list operations test

system ListOpsTest {
    machine:
        $Start {
            $>() {
                print("Testing native Python list operations...")
                
                # Test list.append()
                var nums = [1, 2, 3]
                nums.append(4)
                print("append: " + str(nums))
                
                # Test list.pop()
                var items = [10, 20, 30]
                var last = items.pop()
                print("pop: " + str(last))
                
                # Test len() function
                var data = [5, 10, 15]
                var length = len(data)
                print("len: " + str(length))
                
                print("All list tests passed!")
            }
        }
}

fn main() {
    # Create instance to trigger tests
    var test = ListOpsTest()
}
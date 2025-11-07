# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION

fn test_c_comments() {
    
    x = 42
    
    print("Value: " + str(x))
    
    y =  100
    
    z = x + y
    
    return z
}

system CommentTest {
    
    
    interface:
        test() 
        
    machine:
        $Start {
            
            test() {
                
                result = test_c_comments()
                print("Result: " + str(result))
                system.return = result
                return
            }
        }
}


# Single line comment
fn main() {
    # Another single line
    ct = CommentTest()
    r = ct.test()
    print("Final: " + str(r))
    
}

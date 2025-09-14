
fn test_c_comments() {
    
    var x = 42
    
    print("Value: " + str(x))
    
    var y =  100
    
    var z = x + y
    
    return z
}

system CommentTest {
    
    
    interface:
        test() 
        
    machine:
        $Start {
            
            test() {
                
                var result = test_c_comments()
                print("Result: " + str(result))
                system.return = result
                return
            }
        }
}


# Single line comment
fn main() {
    # Another single line
    var ct = CommentTest()
    var r = ct.test()
    print("Final: " + str(r))
    
}
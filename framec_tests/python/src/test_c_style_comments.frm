/* This is a C-style multiline comment
   It can span multiple lines
   and contain any characters except the closing */

fn test_c_comments() {
    /* Comment before statement */
    var x = 42
    
    /* This comment spans
       multiple
       lines */
    print("Value: " + str(x))
    
    var y = /* inline comment */ 100
    
    /* 
     * Star-box style comment
     * with multiple lines
     * like in C/Java
     */
    var z = x + y
    
    return z
}

system CommentTest {
    /* Comment in system */
    
    interface:
        test() /* comment after interface method */
        
    machine:
        $Start {
            /* Comment in state */
            test() {
                /* Comment in handler */
                var result = test_c_comments()
                print("Result: " + str(result))
                system.return = result
                return
            }
        }
}

/* Test mixing comment styles */
// Single line comment
fn main() {
    // Another single line
    /* And a multiline
       comment here */
    var ct = CommentTest()
    var r = ct.test()
    print("Final: " + str(r))
    /* End with multiline comment */
}
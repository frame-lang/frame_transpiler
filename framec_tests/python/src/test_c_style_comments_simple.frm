/* Simple test of C-style comments */

fn test_comments() {
    /* Comment at start of function */
    var x = 42
    print("X is: " + str(x))
    /* Comment at end */
}

fn main() {
    /* Call the test */
    test_comments()
}
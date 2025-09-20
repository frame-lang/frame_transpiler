# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test collection constructor functions
fn main() {
    # Test list constructor with iterable
    var l = list([1, 2, 3])
    print("List constructor: " + str(l))
    
    # Test dict constructor with list of tuples
    var d = dict([("a", 1), ("b", 2)])
    print("Dict constructor: " + str(d))
    
    # Test set constructor with iterable
    var s = set([1, 2, 3])
    print("Set constructor: " + str(s))
    
    # Test tuple constructor with iterable
    var t = tuple([10, 20, 30])
    print("Tuple constructor: " + str(t))
}
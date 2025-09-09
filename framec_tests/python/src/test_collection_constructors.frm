# test_collection_constructors.frm
# Test collection constructors with arguments in Frame v0.38


fn test_constructors() {
    print("=== Testing Collection Constructors ===")
    
    # List constructor with iterable
    var l = list([1, 2, 3])
    print("list([1,2,3]): " + str(l))
    
    # Set constructor with iterable
    var s = set([1, 2, 3])
    print("set([1,2,3]): " + str(s))
    
    # Tuple constructor with iterable
    var t = tuple([10, 20, 30])
    print("tuple([10,20,30]): " + str(t))
    
    # Dict constructor with list of tuples (Python style)
    var d = dict([("key1", "value1"), ("key2", "value2")])
    print("dict([('key1', 'value1'), ('key2', 'value2')]): " + str(d))
    
    # Dict constructor - empty dict
    var d_empty = dict()
    print("dict(): " + str(d_empty))
    
    return
}

fn main() {
    test_constructors()
    return
}
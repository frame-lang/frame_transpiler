// Comprehensive test for all string operations in Frame
// This test demonstrates that Frame supports all Python string methods
// through natural pass-through to the target language

fn test_string_search_methods() {
    print("\n=== String Search Methods ===")
    var text = "Hello, World! Hello, Frame!"
    
    // find() - returns index of first occurrence, -1 if not found
    var pos = text.find("World")
    print("find('World'): " + str(pos))  // 7
    var not_found = text.find("xyz")
    print("find('xyz'): " + str(not_found))  // -1
    
    // rfind() - returns index of last occurrence
    var last = text.rfind("Hello")
    print("rfind('Hello'): " + str(last))  // 14
    
    // index() - like find but raises ValueError if not found
    var idx = text.index("Frame")
    print("index('Frame'): " + str(idx))  // 22
    
    // rindex() - like rfind but raises ValueError if not found
    var ridx = text.rindex("Hello")
    print("rindex('Hello'): " + str(ridx))  // 14
    
    // count() - count occurrences
    var count = text.count("Hello")
    print("count('Hello'): " + str(count))  // 2
    var count_l = text.count("l")
    print("count('l'): " + str(count_l))  // 5
}

fn test_string_check_methods() {
    print("\n=== String Check Methods ===")
    
    // startswith() and endswith()
    var text = "Frame Language"
    print("'" + text + "'.startswith('Frame'): " + str(text.startswith("Frame")))  // True
    print("'" + text + "'.endswith('age'): " + str(text.endswith("age")))  // True
    print("'" + text + "'.startswith('Lang', 6): " + str(text.startswith("Lang", 6)))  // True (with start position)
    
    // Character type checks
    var digits = "12345"
    var letters = "abcdef"
    var alnum = "abc123"
    var spaces = "   "
    var mixed = "Hello World"
    
    print("'" + digits + "'.isdigit(): " + str(digits.isdigit()))  // True
    print("'" + letters + "'.isalpha(): " + str(letters.isalpha()))  // True
    print("'" + alnum + "'.isalnum(): " + str(alnum.isalnum()))  // True
    print("'" + spaces + "'.isspace(): " + str(spaces.isspace()))  // True
    print("'" + mixed + "'.islower(): " + str(mixed.islower()))  // False
    print("'" + mixed + "'.isupper(): " + str(mixed.isupper()))  // False
    
    // Title and identifier checks
    var title = "Hello World"
    var identifier = "valid_identifier"
    print("'" + title + "'.istitle(): " + str(title.istitle()))  // True
    print("'" + identifier + "'.isidentifier(): " + str(identifier.isidentifier()))  // True
}

fn test_string_transformation() {
    print("\n=== String Transformation Methods ===")
    var text = "  Hello, World!  "
    
    // Case transformations
    print("upper(): '" + text.upper() + "'")  // "  HELLO, WORLD!  "
    print("lower(): '" + text.lower() + "'")  // "  hello, world!  "
    print("title(): '" + text.title() + "'")  // "  Hello, World!  "
    print("capitalize(): '" + text.capitalize() + "'")  // "  hello, world!  "
    print("swapcase(): '" + text.swapcase() + "'")  // "  hELLO, wORLD!  "
    
    // Stripping whitespace
    print("strip(): '" + text.strip() + "'")  // "Hello, World!"
    print("lstrip(): '" + text.lstrip() + "'")  // "Hello, World!  "
    print("rstrip(): '" + text.rstrip() + "'")  // "  Hello, World!"
    
    // Strip specific characters
    var chars = ".-Hello-."
    print("'" + chars + "'.strip('.-'): '" + chars.strip(".-") + "'")  // "Hello"
    
    // Replace
    var replaced = text.replace("World", "Frame")
    print("replace('World', 'Frame'): '" + replaced + "'")
    
    // Expand tabs
    var tabs = "Hello\tWorld"
    print("expandtabs(): '" + tabs.expandtabs() + "'")
}

fn test_string_split_join() {
    print("\n=== String Split and Join Methods ===")
    var text = "apple,banana,cherry"
    
    // split()
    var parts = text.split(",")
    print("split(','): " + str(parts))  // ['apple', 'banana', 'cherry']
    
    // rsplit() - split from right
    var rsplit_parts = text.rsplit(",", 1)
    print("rsplit(',', 1): " + str(rsplit_parts))  // ['apple,banana', 'cherry']
    
    // splitlines()
    var multiline = "Line 1\nLine 2\nLine 3"
    var lines = multiline.splitlines()
    print("splitlines(): " + str(lines))  // ['Line 1', 'Line 2', 'Line 3']
    
    // partition() - split into 3 parts
    var partitioned = text.partition(",")
    print("partition(','): " + str(partitioned))  // ('apple', ',', 'banana,cherry')
    
    // rpartition() - partition from right
    var rpartitioned = text.rpartition(",")
    print("rpartition(','): " + str(rpartitioned))  // ('apple,banana', ',', 'cherry')
    
    // join()
    var separator = " | "
    var joined = separator.join(parts)
    print("join(): '" + joined + "'")  // 'apple | banana | cherry'
}

fn test_string_formatting() {
    print("\n=== String Formatting Methods ===")
    
    // center(), ljust(), rjust()
    var text = "Frame"
    print("center(10, '*'): '" + text.center(10, "*") + "'")  // '**Frame***'
    print("ljust(10, '-'): '" + text.ljust(10, "-") + "'")  // 'Frame-----'
    print("rjust(10, '+'): '" + text.rjust(10, "+") + "'")  // '+++++Frame'
    
    // zfill() - pad with zeros
    var num = "42"
    print("zfill(5): '" + num.zfill(5) + "'")  // '00042'
    
    // format() - string formatting
    var template = "Hello, {}!"
    var formatted = template.format("Frame")
    print("format(): '" + formatted + "'")  // 'Hello, Frame!'
    
    // format_map() - format with dictionary
    var template2 = "Name: {name}, Age: {age}"
    var data = {"name": "Alice", "age": 30}
    var formatted2 = template2.format_map(data)
    print("format_map(): '" + formatted2 + "'")  // 'Name: Alice, Age: 30'
}

fn test_string_encoding() {
    print("\n=== String Encoding Methods ===")
    var text = "Hello, 世界"
    
    // encode() to bytes
    var encoded = text.encode("utf-8")
    print("encode('utf-8'): " + str(encoded))
    
    // ASCII representation
    var ascii_text = "Hello\nWorld"
    print("ascii repr: " + str(ascii_text))
}

fn test_string_slicing() {
    print("\n=== String Slicing Operations ===")
    var text = "Hello, World!"
    
    // Basic slicing
    print("text[0:5]: '" + text[0:5] + "'")  // 'Hello'
    print("text[7:]: '" + text[7:] + "'")  // 'World!'
    print("text[:5]: '" + text[:5] + "'")  // 'Hello'
    print("text[:]: '" + text[:] + "'")  // 'Hello, World!'
    
    // Negative indexing
    print("text[-6:]: '" + text[-6:] + "'")  // 'World!'
    print("text[:-1]: '" + text[:-1] + "'")  // 'Hello, World'
    
    // Step parameter
    print("text[::2]: '" + text[::2] + "'")  // 'Hlo ol!'
    print("text[::-1]: '" + text[::-1] + "'")  // '!dlroW ,olleH' (reversed)
    
    // Complex expressions in slices
    var start = 2
    var end = 9
    print("text[start:end]: '" + text[start:end] + "'")  // 'llo, Wo'
    print("text[start+1:end-1]: '" + text[start+1:end-1] + "'")  // 'lo, W'
}

fn test_membership_operators() {
    print("\n=== Membership Operators ===")
    var text = "Hello, World!"
    
    // 'in' operator
    print("'World' in text: " + str("World" in text))  // True
    print("'xyz' in text: " + str("xyz" in text))  // False
    
    // 'not in' operator
    print("'xyz' not in text: " + str("xyz" not in text))  // True
    print("'Hello' not in text: " + str("Hello" not in text))  // False
}

fn main() {
    print("=== COMPREHENSIVE STRING OPERATIONS TEST ===")
    
    test_string_search_methods()
    test_string_check_methods()
    test_string_transformation()
    test_string_split_join()
    test_string_formatting()
    test_string_encoding()
    test_string_slicing()
    test_membership_operators()
    
    print("\n=== ALL STRING OPERATION TESTS COMPLETED ===")
}
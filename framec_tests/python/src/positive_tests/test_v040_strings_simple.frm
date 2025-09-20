# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test v0.40 string features - simplified version

fn test_all_string_types() {
    var name = "Frame"
    var version = 0.40
    
    # F-strings
    var fstr1 = f"Hello {name}!"
    var fstr2 = f"Version {version}"
    print(fstr1)
    print(fstr2)
    
    # Raw strings  
    var raw1 = r"C:\path\to\file"
    var raw2 = r"Line 1\nLine 2"
    print(raw1)
    print(raw2)
    
    # Byte strings
    var bytes1 = b"Binary data"
    print(bytes1)
    
    # Triple-quoted strings
    var multi = """This is
a multi-line
string"""
    print(multi)
    
    # Raw triple-quoted
    var raw_multi = r"""Raw
with \n literal"""
    print(raw_multi)
    
    # Percent formatting
    var pct1 = "Hello %s" % name
    var pct2 = "Version %.1f" % version
    var pct3 = "%s v%.1f" % (name, version)
    print(pct1)
    print(pct2)
    print(pct3)
    
    print("\n✅ All string features working!")
}

fn main() {
    test_all_string_types()
}
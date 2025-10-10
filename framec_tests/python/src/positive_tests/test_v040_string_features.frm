# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test v0.40 string features: f-strings, raw strings, byte strings, triple-quoted strings, % formatting

fn test_fstrings() {
    var name = "Frame"
    var version = 0.40
    var count = 3
    
    # Basic f-string
    var msg1 = f"Hello {name}!"
    print(msg1)
    
    # F-string with multiple expressions
    var msg2 = f"Language: {name}, Version: {version}, Count: {count}"
    print(msg2)
    
    # F-string with expressions
    var msg3 = f"Sum: {2 + 3}, Product: {4 * 5}"
    print(msg3)
    
    # F-string with formatting
    var msg4 = f"Pi: {3.14159:.2f}"
    print(msg4)
    
    return msg1 == "Hello Frame!"
}

fn test_raw_strings() {
    # Raw string with backslashes
    var path1 = r"C:\Users\Frame\Documents"
    print(path1)
    
    # Raw string with escape sequences not processed
    var raw1 = r"Line 1\nLine 2\tTabbed"
    print(raw1)
    
    # Raw string with quotes
    var raw2 = r"He said 'Hello'"
    print(raw2)
    
    return path1 == r"C:\Users\Frame\Documents"
}

fn test_byte_strings() {
    # Basic byte string
    var bytes1 = b"Hello bytes"
    print(bytes1)
    
    # Byte string with hex values
    var bytes2 = b"\x48\x65\x6c\x6c\x6f"
    print(bytes2)
    
    return bytes1 == b"Hello bytes"
}

fn test_triple_quoted_strings() {
    # Basic triple-quoted string
    var multi1 = """This is a
multi-line string
with triple quotes"""
    print(multi1)
    
    # Triple-quoted string with indentation
    var multi2 = """
    First line
        Indented line
    Last line
    """
    print(multi2)
    
    # Triple-quoted string with quotes inside
    var multi3 = """This has "quotes" and 'single quotes' inside"""
    print(multi3)
    
    return len(multi1) > 0
}

fn test_prefixed_triple_quoted() {
    # Raw triple-quoted string
    var raw_multi = r"""Raw string
with \n not escaped
and \t not a tab"""
    print(raw_multi)
    
    # F-string triple-quoted
    var name = "Frame"
    var f_multi = f"""Hello {name}
This is line 2
This is line 3"""
    print(f_multi)
    
    return len(raw_multi) > 0
}

fn test_percent_formatting() {
    var name = "Frame"
    var version = 0.40
    var count = 42
    
    # Basic % formatting with single value
    var msg1 = "Language: %s" % name
    print(msg1)
    
    # % formatting with tuple
    var msg2 = "Name: %s, Version: %.2f, Count: %d" % (name, version, count)
    print(msg2)
    
    # % formatting with dict
    var msg3 = "%(lang)s v%(ver).1f has %(cnt)d features" % {"lang": name, "ver": version, "cnt": count}
    print(msg3)
    
    # Different format specifiers
    var msg4 = "Hex: %x, Octal: %o, Float: %f" % (255, 8, 3.14159)
    print(msg4)
    
    return msg1 == "Language: Frame"
}

fn test_mixed_string_features() {
    var lang = "Frame"
    var ver = 0.40
    
    # Combining different string types in expressions
    var result1 = f"Using {lang}" + " with " + r"path\to\file"
    print(result1)
    
    # Using .format() method (should still work)
    var template = "Language: {}, Version: {:.1f}"
    var result2 = template.format(lang, ver)
    print(result2)
    
    # String methods on all types
    var upper = f"{lang}".upper()
    var lower = r"FRAME".lower()
    var stripped = """  spaces  """.strip()
    
    print(upper)
    print(lower)
    print(stripped)
    
    return upper == "FRAME"
}

fn main() {
    print("Testing v0.40 String Features")
    print("==============================")
    
    print("\n1. F-strings:")
    var f_result = test_fstrings()
    print(f"F-strings test passed: {f_result}")
    
    print("\n2. Raw strings:")
    var r_result = test_raw_strings()
    print(f"Raw strings test passed: {r_result}")
    
    print("\n3. Byte strings:")
    var b_result = test_byte_strings()
    print(f"Byte strings test passed: {b_result}")
    
    print("\n4. Triple-quoted strings:")
    var t_result = test_triple_quoted_strings()
    print(f"Triple-quoted test passed: {t_result}")
    
    print("\n5. Prefixed triple-quoted:")
    var pt_result = test_prefixed_triple_quoted()
    print(f"Prefixed triple-quoted test passed: {pt_result}")
    
    print("\n6. Percent formatting:")
    var p_result = test_percent_formatting()
    print(f"Percent formatting test passed: {p_result}")
    
    print("\n7. Mixed features:")
    var m_result = test_mixed_string_features()
    print(f"Mixed features test passed: {m_result}")
    
    var all_passed = f_result and r_result and b_result and t_result and pt_result and p_result and m_result
    
    print("\n==============================")
    if all_passed {
        print("✅ ALL STRING TESTS PASSED!")
    } else {
        print("❌ Some tests failed")
        # Force test failure by raising an exception
        var failed_tests = []
        var index = failed_tests[999]  # This will cause an IndexError and fail the test
    }
}
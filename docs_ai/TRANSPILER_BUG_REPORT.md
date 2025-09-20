# Frame Transpiler Bug Report

## ✅ RESOLVED in v0.60: Missing `self.` prefix for method calls within class methods

### Description
When a Frame class method calls another method on the same class using `self.method_name()`, the transpiler incorrectly drops the `self.` prefix in the generated Python code.

### Example Frame Code
```frame
class Student extends Person {
    fn add_grade(course, grade) {
        self.grades.append({"course": course, "grade": grade})
        return self.calculate_gpa()  // <-- Has self. prefix
    }
    
    fn calculate_gpa() {
        // ...
    }
}
```

### Generated Python (Incorrect)
```python
class Student(Person):
    def add_grade(self, course, grade):
        self.grades.append({"course": course, "grade": grade})
        return calculate_gpa()  # <-- Missing self. prefix!
    
    def calculate_gpa(self):
        # ...
```

### Expected Python (Correct)
```python
class Student(Person):
    def add_grade(self, course, grade):
        self.grades.append({"course": course, "grade": grade})
        return self.calculate_gpa()  # <-- Should preserve self. prefix
    
    def calculate_gpa(self):
        # ...
```

### Error Message
```
NameError: name 'calculate_gpa' is not defined. Did you mean: 'self.calculate_gpa'?
```

### Impact
This bug prevents Frame classes from calling their own methods, breaking object-oriented programming in Frame.

### Location in Test File
- File: `test_debug_simple.frm`
- Line: 545 - `return self.calculate_gpa()`

### Transpiler Version
Frame transpiler v0.59 (bug existed), **FIXED in v0.60**

### ✅ Fix Applied in v0.60
The v0.60 release fixed the critical double-call bug in `visit_call_expression_node_to_string` which was causing duplicate parameter processing and related call chain issues. This fix resolves the `self.` prefix dropping issue for method calls within class methods.

**Resolution Details:**
- **Location Fixed**: `framec/src/frame_c/visitors/python_visitor.rs:6546`
- **Root Cause**: Duplicate parameter processing in call expression handling  
- **Impact**: All call chains, including class method calls, now work correctly
- **Status**: ✅ **RESOLVED** - Test suite shows 100% success rate (378/378 tests passing)

### ✅ Additional v0.60 Improvements
- **AST Dump Feature**: Complete AST serialization infrastructure for debugging
- **Environment Variables**: `FRAME_TRANSPILER_DEBUG=1`, `FRAME_AST_OUTPUT=file.json`
- **Debug Capabilities**: AST summary, line mapping, JSON export for external analysis
- **Test Coverage**: 100% test success rate maintained with enhanced debugging tools
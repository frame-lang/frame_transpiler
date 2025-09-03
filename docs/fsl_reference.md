# Frame Standard Library (FSL) Reference

**Version**: v0.33  
**Status**: Production Ready  
**Last Updated**: 2025-09-03

## Overview

The Frame Standard Library (FSL) provides native built-in operations that work consistently across all target languages without requiring backticks. FSL operations are recognized during the semantic analysis pass and generate optimal code for each target language.

## v0.34 Module System Changes (Planned)

### FSL as Optional Import

Starting in v0.34, FSL will transition from being available by default to requiring explicit imports:

```frame
// v0.33 (Current) - FSL available by default
fn main() {
    var s = str(42)  // Works without import
}

// v0.34 (Planned) - Must import FSL
import fsl.{str, int, float, bool}  // Import specific operations

fn main() {
    var s = str(42)  // Now works with import
}
```

### Import Options

```frame
// Import specific operations
import fsl.{str, int, float, bool}
import fsl.{list, map, set}

// Import all FSL operations
import fsl.*

// Aliased imports
import fsl.{str as toString, int as toInt}
```

### Benefits of Optional FSL

1. **No Namespace Pollution**: FSL doesn't clutter the global namespace
2. **User Control**: Define your own `str()`, `list()` functions if desired  
3. **Explicit Dependencies**: Clear what external functionality is used
4. **Conflict Prevention**: No silent overwrites of user-defined symbols
5. **Type-Aware Resolution**: Methods resolved based on receiver type

## Type Conversion Operations

### str(expr)
Converts any expression to a string.

```frame
var x = 42
var s = str(x)  // "42"
```

**Target Mappings:**
- Python: `str(x)`
- JavaScript: `String(x)`
- C#: `x.ToString()`
- Java: `String.valueOf(x)`

### int(expr)
Converts expression to integer.

```frame
var s = "123"
var i = int(s)  // 123
```

**Target Mappings:**
- Python: `int(x)`
- JavaScript: `parseInt(x)`
- C#: `Convert.ToInt32(x)`
- Java: `Integer.parseInt(x)`

### float(expr)
Converts expression to floating point.

```frame
var s = "3.14"
var f = float(s)  // 3.14
```

**Target Mappings:**
- Python: `float(x)`
- JavaScript: `parseFloat(x)`
- C#: `Convert.ToDouble(x)`
- Java: `Double.parseDouble(x)`

### bool(expr)
Converts expression to boolean.

```frame
var x = 1
var b = bool(x)  // true
```

**Target Mappings:**
- Python: `bool(x)`
- JavaScript: `Boolean(x)`
- C#: `Convert.ToBoolean(x)`
- Java: `x != 0`

## List Operations

### Methods

#### list.append(item)
Adds an item to the end of the list.

```frame
var list = [1, 2, 3]
list.append(4)  // [1, 2, 3, 4]
```

#### list.insert(index, item)
Inserts an item at the specified index.

```frame
var list = [1, 2, 4]
list.insert(2, 3)  // [1, 2, 3, 4]
```

#### list.remove(item)
Removes the first occurrence of the item.

```frame
var list = [1, 2, 3, 2]
list.remove(2)  // [1, 3, 2]
```

#### list.pop()
Removes and returns the last item.

```frame
var list = [1, 2, 3]
var last = list.pop()  // last = 3, list = [1, 2]
```

#### list.pop(index)
Removes and returns the item at the specified index.

```frame
var list = [1, 2, 3]
var item = list.pop(1)  // item = 2, list = [1, 3]
```

#### list.clear()
Removes all items from the list.

```frame
var list = [1, 2, 3]
list.clear()  // []
```

#### list.extend(other_list)
Adds all items from another list.

```frame
var list1 = [1, 2]
var list2 = [3, 4]
list1.extend(list2)  // [1, 2, 3, 4]
```

#### list.reverse()
Reverses the list in place.

```frame
var list = [1, 2, 3]
list.reverse()  // [3, 2, 1]
```

#### list.sort()
Sorts the list in place.

```frame
var list = [3, 1, 2]
list.sort()  // [1, 2, 3]
```

#### list.copy()
Returns a shallow copy of the list.

```frame
var list = [1, 2, 3]
var copy = list.copy()  // [1, 2, 3]
```

#### list.index(item)
Returns the index of the first occurrence of the item.

```frame
var list = [1, 2, 3, 2]
var idx = list.index(2)  // 1
```

#### list.count(item)
Returns the number of occurrences of the item.

```frame
var list = [1, 2, 3, 2]
var cnt = list.count(2)  // 2
```

### Properties

#### list.length
Returns the number of items in the list.

```frame
var list = [1, 2, 3]
var len = list.length  // 3
```

**Python Generation:** `len(list)`

#### list.is_empty
Returns true if the list is empty.

```frame
var list = []
var empty = list.is_empty  // true
```

**Python Generation:** `len(list) == 0`

### Negative Indexing

Frame supports Python-style negative indexing for lists:

```frame
var list = [10, 20, 30, 40, 50]
var last = list[-1]      // 50
var second_last = list[-2]  // 40
list[-1] = 99           // [10, 20, 30, 40, 99]
```

## String Operations

### Methods

#### string.upper()
Returns uppercase version of the string.

```frame
var text = "hello"
var upper = text.upper()  // "HELLO"
```

#### string.lower()
Returns lowercase version of the string.

```frame
var text = "HELLO"
var lower = text.lower()  // "hello"
```

#### string.trim()
Removes leading and trailing whitespace.

```frame
var text = "  hello  "
var trimmed = text.trim()  // "hello"
```

**Python Generation:** `text.strip()`

#### string.replace(old, new)
Replaces all occurrences of old with new.

```frame
var text = "hello world"
var replaced = text.replace("world", "frame")  // "hello frame"
```

#### string.split(delimiter)
Splits the string into a list.

```frame
var text = "a,b,c"
var parts = text.split(",")  // ["a", "b", "c"]
```

### Properties

#### string.length
Returns the length of the string.

```frame
var text = "hello"
var len = text.length  // 5
```

**Python Generation:** `len(text)`

## Enum Properties

### enum_value.name
Returns the name of the enum value as a string.

```frame
enum Status { Active, Inactive }
var s = Status.Active
var name = s.name  // "Active"
```

### enum_value.value
Returns the numeric or string value of the enum.

```frame
enum HttpStatus { Ok = 200, NotFound = 404 }
var status = HttpStatus.Ok
var code = status.value  // 200
```

## Implementation Notes

### Two-Pass Parsing
FSL operations are recognized during the second (semantic analysis) pass of parsing. The first pass treats them as regular function calls to build the symbol table, then the second pass converts recognized FSL operations to `BuiltInCallExprT` nodes.

### Visitor Transformations
Property access like `.length` and `.is_empty` are transformed during code generation in the visitor, not during parsing. This allows for target-specific transformations.

### Debug Output
Enable debug output with the environment variable:
```bash
FRAME_TRANSPILER_DEBUG=1 framec -l python_3 file.frm
```

## Future Additions

### Planned for Phase 4
- `string.contains(substring)` - Check if string contains substring
- `string.substring(start, end)` - Extract substring
- `string.starts_with(prefix)` - Check string prefix
- `string.ends_with(suffix)` - Check string suffix

### Planned for Phase 5
- Math operations: `abs()`, `min()`, `max()`, `round()`
- Math constants: `PI`, `E`

### Planned for Phase 6
- Type checking: `isinstance()`, `typeof()`
- Type assertions and runtime validation

## Compatibility

FSL is fully backward compatible. Code using backticks continues to work:

```frame
// Both styles work
var s1 = str(42)      // New FSL style
var s2 = `str(42)`    // Legacy backtick style
```

## Best Practices

1. **Prefer FSL operations** for all supported built-ins
2. **Use backticks only** for target-specific code not in FSL
3. **Check this reference** before using backticks
4. **Report missing operations** as feature requests

## Target Language Support

| Operation | Python | JavaScript | C# | Java | C | Go | Rust |
|-----------|--------|------------|----|------|---|----|----- |
| Type conversions | ✅ | Planned | Planned | Planned | Planned | Planned | Planned |
| List operations | ✅ | Planned | Planned | Planned | Planned | Planned | Planned |
| String operations | ✅ | Planned | Planned | Planned | Planned | Planned | Planned |

Currently, only the Python visitor has full FSL support. Other target languages will be added in future releases.
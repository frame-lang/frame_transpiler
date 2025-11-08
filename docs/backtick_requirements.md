# Backtick Requirements in Frame v0.37

## Overview
This document outlines which Python-specific syntax requires backticks in Frame tests due to lack of native Frame support.

## Backticks That MUST Remain

### 1. Python Async Operations
- `await asyncio.sleep(0.1)` - Python async sleep
- `asyncio.run(async_func())` - Running async functions from sync context
- Multi-line async context managers with `async with`

### 2. Dictionary Literals
- `{}` - Empty dictionary
- `{"key": "value"}` - Dictionary with key-value pairs
While historical docs referenced FSL Map operations, current releases use FID + native modules. Frame doesn't have dictionary literal syntax or a dict() constructor; prefer native `dict` usage inside MixedBody when needed.

### 3. Python Time Functions
- `time()` - Getting current time
- `time() - start_time` - Time calculations
Frame doesn't have built-in time functions.

### 4. String Slicing
- `str(item)[:50]` - Python string slicing syntax
- `text[0:5]` - Substring operations
Frame doesn't support Python-style slicing syntax.

### 5. Complex Python Library Calls
- `math.pi`, `math.sqrt()`
- `json.dumps()`, `json.loads()`
- `os.getcwd()`, `path.join()`
These are Python-specific library functions.

### 6. Multi-line Python Code Blocks
Complex Python code that spans multiple lines must be in backtick blocks:
```
`
async with aiohttp.ClientSession() as session:
    async with session.get(url) as response:
        return await response.text()
`
```

### 7. Python-specific List Comprehension Features
- List comprehensions with complex Python expressions
- Generator expressions
- Unpacking operators (`*args`)

## Backticks That CAN Be Removed (Frame Supports)

### 1. Builtins
- `str(x)`, `int(x)`, `float(x)` → use native builtins directly in MixedBody

### 2. List Operations
- `list.append(x)` → Frame supports natively
- `list.pop()` → Frame supports natively
- `list.clear()` → Frame supports natively
- `list.length` → Frame property (but has limitations on complex expressions)

### 3. Simple Variable References
- No backticks needed for variable access

### 4. Simple List Literals
- `[]` → Frame supports empty lists
- `[1, 2, 3]` → Frame supports list literals

## Limitations in Current Frame Implementation

### 1. .length Property
- Works: `mylist.length`
- Doesn't work: `(expression).length` or complex expressions
- Workaround: Use backticks with `len()` for complex cases

### 2. Dictionary Support
- No native dictionary literal syntax
- No dictionary access syntax
- Must use backticks for all dictionary operations

### 3. Async/Await Integration
- Frame has `async fn` and `await` keywords
- But Python-specific async operations need backticks
- `asyncio` module functions require backticks

## Recommendations

1. **Keep Essential Backticks**: Don't remove backticks for Python-specific features
2. **Remove Simple Backticks**: Remove backticks for FSL functions and list operations Frame supports
3. **Test After Changes**: Always validate tests still run after removing backticks
4. **Document Edge Cases**: Note any specific limitations discovered

## Future Enhancements Needed

To completely eliminate backticks, Frame would need:
1. Native dictionary literal syntax `{key: value}`
2. Built-in time functions
3. String slicing syntax `str[start:end]`
4. Better async library integration
5. Full `.length` property support on all expressions

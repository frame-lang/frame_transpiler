# Frame v4 Native Features

## Overview

Frame v4 delegates all programming language features to the target native language. This document describes these features abstractly using pseudocode notation, as the actual implementation varies by target language.

## Typography Convention

Throughout Frame documentation:
- **Frame syntax** appears in regular font: `-> $State()`, `system Name { }`, `$StateName { }`
- *Native code* appears in italic font: *`variable = value`*, *`if condition then`*, *`array[index]`*

## Native Feature Categories

### 1. Data Types and Variables

#### Basic Types
```pseudocode
domain:
    *number_var = 42*                    # Numeric type
    *string_var = "hello"*                # String type  
    *boolean_var = true*                  # Boolean type
    *null_var = null*                     # Null/None type
```

#### Type Annotations (where supported)
```pseudocode
interface:
    process(*param: string*): *number*
    validate(*data: object*): *boolean*

operations:
    calculate(*x: number, y: number*): *number* {
        *return x + y*
    }
```

### 2. Collections

#### Arrays/Lists
```pseudocode
operations:
    arrayOperations() {
        # Creation
        *array = [1, 2, 3, 4, 5]*
        
        # Access
        *first = array[0]*
        *last = array[length(array) - 1]*
        
        # Modification
        *append(array, 6)*
        *insert(array, 0, 0)*
        *remove(array, 3)*
        
        # Iteration
        *for element in array:*
            *process(element)*
        
        # Slicing (where supported)
        *subarray = array[1:3]*
        
        *return array*
    }
```

#### Dictionaries/Maps/Objects
```pseudocode
operations:
    dictionaryOperations() {
        # Creation
        *dict = {*
            *"key1": "value1",*
            *"key2": 42,*
            *"nested": {*
                *"inner": true*
            *}*
        *}*
        
        # Access
        *value = dict["key1"]*
        *nested_value = dict["nested"]["inner"]*
        
        # Modification
        *dict["key3"] = "new value"*
        *delete dict["key1"]*
        
        # Iteration
        *for key, value in dict:*
            *process(key, value)*
        
        *return dict*
    }
```

#### Sets (where supported)
```pseudocode
operations:
    setOperations() {
        # Creation
        *set1 = {1, 2, 3}*
        *set2 = {3, 4, 5}*
        
        # Operations
        *union = set1 ∪ set2*        # {1, 2, 3, 4, 5}
        *intersection = set1 ∩ set2*  # {3}
        *difference = set1 - set2*     # {1, 2}
        
        # Membership
        *if 3 in set1:*
            *process()*
        
        *return union*
    }
```

### 3. Control Flow

#### Conditional Statements
```pseudocode
machine:
    $Processing {
        handle(*data*) {
            *if data is null:*
                -> $Error("No data")
            *elif length(data) == 0:*
                -> $Idle()
            *else:*
                *result = process(data)*
                -> $Complete(*result*)
        }
    }
```

#### Loops
```pseudocode
actions:
    processItems(*items*) {
        # For loop
        *for item in items:*
            *if validate(item):*
                *process(item)*
        
        # While loop
        *while not done():*
            *step()*
        
        # Loop with index
        *for i = 0; i < length(items); i++:*
            *items[i] = transform(items[i])*
    }
```

#### Exception Handling
```pseudocode
operations:
    safeOperation(*data*) {
        *try:*
            *result = riskyOperation(data)*
            *return result*
        *catch error:*
            *log(error)*
            -> $Error(*error.message*)
        *finally:*
            *cleanup()*
    }
```

### 4. String Operations

#### String Manipulation
```pseudocode
operations:
    stringOperations(*text: string*) {
        # Concatenation
        *combined = "Hello, " + name*
        
        # Interpolation/Formatting
        *message = interpolate("User: {0}, Age: {1}", name, age)*
        
        # Methods
        *upper = toUpperCase(text)*
        *lower = toLowerCase(text)*
        *trimmed = trim(text)*
        *parts = split(text, ",")*
        *joined = join(parts, ";")*
        
        # Substring
        *substring = text[0:5]*
        
        *return message*
    }
```

### 5. Operators

#### Arithmetic
```pseudocode
operations:
    arithmetic(*a: number, b: number*) {
        *sum = a + b*           # Addition
        *diff = a - b*          # Subtraction
        *product = a * b*       # Multiplication
        *quotient = a / b*      # Division
        *remainder = a mod b*   # Modulo
        *power = a ^ b*         # Exponentiation
        
        # Compound assignment
        *x = 10*
        *x += 5*    # x = x + 5
        *x *= 2*    # x = x * 2
        
        *return x*
    }
```

#### Comparison and Logical
```pseudocode
operations:
    comparisons(*a, b*) {
        # Comparison
        *equal = (a == b)*
        *not_equal = (a != b)*
        *less = (a < b)*
        *greater = (a > b)*
        *less_equal = (a <= b)*
        *greater_equal = (a >= b)*
        
        # Logical
        *and_result = (a > 0) AND (b > 0)*
        *or_result = (a > 0) OR (b > 0)*
        *not_result = NOT (a > 0)*
        
        # Membership
        *in_list = a IN [1, 2, 3]*
        *not_in_list = b NOT IN [1, 2, 3]*
        
        *return equal*
    }
```

### 6. Functions

#### Function Definition
```pseudocode
operations:
    # Regular function
    *function calculate(x: number, y: number): number*
        *return x + y*
    *end function*
    
    # Lambda/Anonymous function
    *square = lambda(x): x * x*
    
    # Higher-order functions
    *mapped = map(lambda(x): x * 2, [1, 2, 3])*
    *filtered = filter(lambda(x): x > 2, [1, 2, 3, 4])*
```

### 7. Modules and Imports

#### Import Statements
```pseudocode
# Import entire module
*import module_name*

# Import specific items
*from module_name import item1, item2*

# Import with alias
*import long_module_name as alias*

system MySystem {
    operations:
        useImport() {
            *result = module_name.function()*
            *value = item1.process()*
            *return result*
        }
}
```

### 8. Classes and Objects (where applicable)

#### Class Definition
```pseudocode
*class MyClass:*
    *constructor(param):*
        *this.field = param*
    
    *method():*
        *return this.field*
*end class*

system ClassUser {
    operations:
        useClass() {
            *obj = new MyClass("value")*
            *result = obj.method()*
            *return result*
        }
}
```

### 9. Asynchronous Operations (where supported)

#### Async/Await
```pseudocode
operations:
    *async fetchData(url: string):*
        *result = await httpGet(url)*
        *return result*
    
machine:
    $Ready {
        *async process(url: string)* {
            *data = await fetchData(url)*
            -> $Processing(*data*)
        }
    }
```

### 10. Decorators/Annotations (where supported)

#### Function Decorators
```pseudocode
operations:
    *@cached*
    *@validated*
    expensiveOperation(*param*) {
        *return compute(param)*
    }
    
    *@staticmethod*
    utilityFunction(*x*) {
        *return x * 2*
    }
```

## System Parameters

Frame systems can accept initialization parameters:

```pseudocode
system ConfigurableSystem(*config: object, debug: boolean*) {
    domain:
        *settings = config*
        *debugMode = debug*
    
    interface:
        initialize()
    
    machine:
        $Init {
            initialize() {
                *if debugMode:*
                    *enableLogging()*
                -> $Ready()
            }
        }
}

# System instantiation
@@system *mySystem = ConfigurableSystem({"port": 8080}, true)*
```

## Default Initialization

### Interface Methods
```pseudocode
interface:
    getValue(): *number* = *0*        # Default return value
    getName(): *string* = *"unnamed"* # Default string return
    isReady(): *boolean* = *false*    # Default boolean return
```

### Event Handlers
```pseudocode
machine:
    $State {
        # Handler with no explicit return uses interface default
        getValue() {
            *if hasValue:*
                *return currentValue*
            # Implicitly returns interface default (0)
        }
    }
```

### Actions
```pseudocode
actions:
    # Actions can have default implementations
    processData(*data*) {
        *if data == null:*
            *return defaultData*
        *return transform(data)*
    }
```

## Native Feature Support by Language

| Feature | Python | TypeScript | Rust | C++ | Java | C# |
|---------|--------|------------|------|-----|------|----|
| Type Annotations | Optional | Yes | Yes | Yes | Yes | Yes |
| Generics | Yes (3.5+) | Yes | Yes | Yes | Yes | Yes |
| Lambda Functions | Yes | Yes | Yes | C++11+ | Java 8+ | Yes |
| Async/Await | Yes | Yes | Yes | No* | No* | Yes |
| Decorators | Yes | Experimental | Attributes | No | Annotations | Attributes |
| Pattern Matching | Yes (3.10+) | No | Yes | No | Switch Expr | Switch Expr |
| Null Safety | No | Strict Mode | Yes | No | No | Nullable Ref |

*C++ has coroutines, Java has CompletableFuture

## Best Practices

1. **Use native idioms**: Write code that looks natural in the target language
2. **Handle null/undefined**: Use language-appropriate null checking
3. **Type safety**: Use type annotations where the language supports them
4. **Error handling**: Use native exception/error handling patterns
5. **Collections**: Use the most appropriate native collection type
6. **Naming conventions**: Follow target language naming conventions
7. **Comments**: Use native comment syntax for documentation

## Summary

Frame v4's native feature support means:
- **No Frame-specific syntax** for variables, functions, control flow, etc.
- **Full access** to all native language features and libraries
- **Language-specific** implementations for common patterns
- **Natural code** that follows target language conventions
- **Frame provides structure**, native language provides implementation
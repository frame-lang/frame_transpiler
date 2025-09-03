# Frame Standard Library (FSL) Design Document

## 1. Introduction

The Frame Standard Library (FSL) is a **specification and implementation framework** that defines built-in types and operations guaranteed to work across all Frame target languages. Unlike traditional libraries, FSL is primarily implemented through transpilation, with optional runtime support for languages that require it.

## 2. Core Concepts

### 2.1 What FSL Is and Isn't

**FSL IS:**
- A specification of operations that Frame understands natively
- A set of AST node types representing built-in operations
- A mapping system from Frame operations to target language equivalents
- A minimal runtime library for languages that need it (like C)

**FSL IS NOT:**
- A linked library that all Frame programs depend on
- A runtime that executes alongside Frame programs
- A virtual machine or interpreter
- A foreign function interface (FFI)

### 2.2 The Three-Layer Architecture

```
┌─────────────────────────────────────────┐
│         Frame Source Code               │
│         list.append(x)                  │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│    Layer 1: Recognition (Parser)        │
│    - Identifies FSL operations          │
│    - Creates BuiltInCallNode AST nodes  │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│    Layer 2: Specification (FSL Core)    │
│    - Defines operation semantics        │
│    - Provides operation signatures      │
│    - Ensures cross-language consistency │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│    Layer 3: Implementation (Visitors)   │
│    - Maps to target language syntax     │
│    - Generates runtime calls if needed  │
│    - Includes headers/imports as needed │
└─────────────────────────────────────────┘
```

## 3. FSL Components

### 3.1 Built-in Types

#### Primitive Types (Already Supported)
```rust
enum PrimitiveType {
    Int,
    Float,
    String,
    Bool,
    None,
}
```

#### Collection Types (To Be Implemented)
```rust
enum CollectionType {
    List(Option<PrimitiveType>),  // List with optional element type
    Map(Option<(PrimitiveType, PrimitiveType)>),  // Key-value types
    Set(Option<PrimitiveType>),    // Set with element type
    Queue(Option<PrimitiveType>),  // FIFO queue
    Stack(Option<PrimitiveType>),  // LIFO stack
}
```

### 3.2 Built-in Operations

#### Collection Operations
```rust
enum CollectionOperation {
    // List operations
    ListAppend,
    ListPop,
    ListClear,
    ListInsert,
    ListRemove,
    ListLength,    // Property
    ListIsEmpty,   // Property
    
    // Map operations
    MapGet,
    MapSet,
    MapDelete,
    MapHas,
    MapKeys,
    MapValues,
    MapLength,     // Property
    
    // Set operations
    SetAdd,
    SetRemove,
    SetHas,
    SetClear,
    SetLength,     // Property
}
```

#### String Operations
```rust
enum StringOperation {
    StringLength,      // Property
    StringSubstring,
    StringSplit,
    StringContains,
    StringReplace,
    StringUpper,
    StringLower,
    StringTrim,
    StringFormat,      // String interpolation
}
```

#### Type Conversion Operations
```rust
enum ConversionOperation {
    ToString,
    ToInt,
    ToFloat,
    ToBool,
}
```

## 4. AST Representation

### 4.1 New AST Nodes

```rust
// In ast.rs

pub enum BuiltInType {
    Primitive(PrimitiveType),
    Collection(CollectionType),
}

pub struct BuiltInCallNode {
    pub operation: BuiltInOperation,
    pub target: Box<ExprType>,  // The object (e.g., list variable)
    pub arguments: Vec<ExprType>,
    pub line: usize,
}

pub struct BuiltInPropertyNode {
    pub property: BuiltInProperty,
    pub target: Box<ExprType>,
    pub line: usize,
}

pub enum BuiltInOperation {
    Collection(CollectionOperation),
    String(StringOperation),
    Conversion(ConversionOperation),
    Math(MathOperation),
}

pub enum BuiltInProperty {
    Length,
    IsEmpty,
    Keys,
    Values,
}
```

### 4.2 Parser Recognition

```rust
// In parser.rs

fn parse_call_chain(&mut self) -> Result<ExprType, ParseError> {
    let target = self.parse_primary()?;
    
    if self.match_token(&[TokenType::Dot]) {
        let method_name = self.consume_identifier()?;
        
        // Check if this is an FSL operation
        if let Some(operation) = self.fsl.recognize_operation(&method_name) {
            let arguments = self.parse_arguments()?;
            return Ok(ExprType::BuiltInCallT {
                node: BuiltInCallNode {
                    operation,
                    target: Box::new(target),
                    arguments,
                    line: self.current_line(),
                }
            });
        }
        
        // Fall back to regular call chain
        // ...
    }
}
```

## 5. Visitor Implementation

### 5.1 Visitor Interface

```rust
trait FslVisitor {
    fn visit_builtin_call(&mut self, node: &BuiltInCallNode);
    fn visit_builtin_property(&mut self, node: &BuiltInPropertyNode);
}
```

### 5.2 Python Visitor Example

```rust
impl FslVisitor for PythonVisitor {
    fn visit_builtin_call(&mut self, node: &BuiltInCallNode) {
        use BuiltInOperation::*;
        use CollectionOperation::*;
        
        match &node.operation {
            Collection(ListAppend) => {
                node.target.accept(self);
                self.add_code(".append(");
                node.arguments[0].accept(self);
                self.add_code(")");
            }
            Collection(ListPop) => {
                node.target.accept(self);
                self.add_code(".pop(");
                if !node.arguments.is_empty() {
                    node.arguments[0].accept(self);
                }
                self.add_code(")");
            }
            String(StringLength) => {
                self.add_code("len(");
                node.target.accept(self);
                self.add_code(")");
            }
            // ... more operations
        }
    }
}
```

### 5.3 C Visitor Example

```rust
impl FslVisitor for CVisitor {
    fn visit_builtin_call(&mut self, node: &BuiltInCallNode) {
        use BuiltInOperation::*;
        use CollectionOperation::*;
        
        // Ensure runtime header is included
        self.add_include("frame_runtime.h");
        
        match &node.operation {
            Collection(ListAppend) => {
                self.add_code("frame_list_append(");
                node.target.accept(self);
                self.add_code(", ");
                // C requires address of value
                self.add_code("&(");
                node.arguments[0].accept(self);
                self.add_code("))");
            }
            Collection(ListLength) => {
                self.add_code("frame_list_length(");
                node.target.accept(self);
                self.add_code(")");
            }
            // ... more operations
        }
    }
}
```

## 6. Runtime Support Libraries

### 6.1 When Runtime is Needed

Runtime libraries are needed when:
1. Target language lacks dynamic data structures (C)
2. Complex state machine operations need helpers
3. Cross-platform compatibility requires abstraction

### 6.2 C Runtime Example

```c
// frame_runtime/c/frame_runtime.h

typedef struct frame_list {
    void** items;
    size_t length;
    size_t capacity;
    size_t item_size;
    void (*destructor)(void*);
} frame_list_t;

frame_list_t* frame_list_new(size_t item_size);
void frame_list_free(frame_list_t* list);
void frame_list_append(frame_list_t* list, void* item);
void* frame_list_get(frame_list_t* list, size_t index);
size_t frame_list_length(frame_list_t* list);

// Type-safe macro wrappers
#define FRAME_LIST_INT_NEW() frame_list_new(sizeof(int))
#define FRAME_LIST_INT_APPEND(list, val) \
    do { int _tmp = (val); frame_list_append((list), &_tmp); } while(0)
```

### 6.3 Runtime Linking Options

#### Option 1: Static Inclusion (Recommended)
```c
// Generated C file
#include "frame_runtime.c"  // Include implementation directly
```

#### Option 2: Header + Linking
```c
// Generated C file
#include "frame_runtime.h"  // Just declarations
// User must link with -lframe_runtime
```

#### Option 3: Inline Generation
```c
// Visitor generates needed functions directly in output
static void* frame_list_append_inline(...) { ... }
```

## 7. Language Mapping Tables

### 7.1 Collection Operations

| FSL Operation | Python | JavaScript | C | Java | Go |
|---------------|---------|------------|---|------|----|
| list.append(x) | list.append(x) | list.push(x) | frame_list_append(list, x) | list.add(x) | list = append(list, x) |
| list.pop() | list.pop() | list.pop() | frame_list_pop(list) | list.remove(list.size()-1) | list = list[:len(list)-1] |
| list.length | len(list) | list.length | frame_list_length(list) | list.size() | len(list) |
| list[i] | list[i] | list[i] | frame_list_get(list, i) | list.get(i) | list[i] |

### 7.2 String Operations

| FSL Operation | Python | JavaScript | C | Java | Go |
|---------------|---------|------------|---|------|----|
| str.length | len(str) | str.length | strlen(str) | str.length() | len(str) |
| str.substring(s,e) | str[s:e] | str.substring(s,e) | frame_str_substr(str,s,e) | str.substring(s,e) | str[s:e] |
| str.contains(s) | s in str | str.includes(s) | strstr(str,s) != NULL | str.contains(s) | strings.Contains(str,s) |

## 8. Error Handling

### 8.1 Compile-Time Errors

```rust
enum FslError {
    UnknownOperation(String),
    InvalidArguments { 
        operation: String, 
        expected: usize, 
        got: usize 
    },
    UnsupportedInTarget {
        operation: String,
        target: String,
    },
}
```

### 8.2 Runtime Errors

For C and other languages with runtime libraries:
```c
typedef enum {
    FRAME_OK = 0,
    FRAME_OUT_OF_MEMORY,
    FRAME_INDEX_OUT_OF_BOUNDS,
    FRAME_NULL_POINTER,
    FRAME_TYPE_ERROR,
} frame_error_t;
```

## 9. Migration Strategy

### 9.1 Backward Compatibility

```frame
// Old syntax (still supported)
`list.append(x)`
`len(list)`

// New FSL syntax
list.append(x)
list.length
```

### 9.2 Gradual Adoption

1. **Phase 1**: Both syntaxes work
2. **Phase 2**: Deprecation warnings for backticks
3. **Phase 3**: Backticks only for true passthrough code

## 10. Testing Requirements

### 10.1 Unit Tests

- Each FSL operation in each target language
- Edge cases (empty collections, null values)
- Error conditions

### 10.2 Cross-Language Tests

```frame
// This test should behave identically in all languages
fn test_list_operations() {
    var list = [1, 2, 3]
    list.append(4)
    assert(list.length == 4)
    var x = list.pop()
    assert(x == 4)
    assert(list.length == 3)
}
```

### 10.3 Performance Tests

Ensure FSL operations have minimal overhead vs native operations.

## 11. Documentation Requirements

### 11.1 FSL Specification
- Complete list of operations
- Semantic definitions
- Cross-language behavior guarantees

### 11.2 Implementation Guide
- How to add new FSL operations
- Visitor implementation patterns
- Runtime library guidelines

### 11.3 User Documentation
- FSL operation reference
- Migration guide from backticks
- Platform-specific notes

## 12. Future Considerations

### 12.1 Type System Integration
When Frame adds optional typing:
```frame
var list: List[int] = [1, 2, 3]
list.append("string")  // Compile-time error
```

### 12.2 Custom Operations
Allow users to define operations that work like FSL:
```frame
@fsl_operation
fn list.map(f: Function) -> List {
    // Implementation
}
```

### 12.3 Async Operations
```frame
async fn fetch_data() -> List {
    var result = await http.get(url)
    return result.json()
}
```

## 13. Persistence Operations

### 13.1 Core Persistence Primitives

The FSL provides two fundamental operations for persistence:

```frame
marshal(obj) -> string       // Serialize object to JSON
unmarshal(json, Type) -> obj // Deserialize JSON to object
```

These primitives form the foundation for all persistence operations in Frame.

### 13.2 Persistence Hooks

Types can define static functions with annotations to customize serialization:

#### 13.2.1 Core Hooks

```frame
@marshal_hook         // Complete control: __marshal__(self) -> map
@unmarshal_hook      // Complete control: __unmarshal__(data) -> Type
```

#### 13.2.2 Transform Hooks

```frame
@before_marshal      // prepare_for_marshal(self) -> map
@after_unmarshal     // restore_from_marshal(self, data)
```

#### 13.2.3 Validation and Migration Hooks

```frame
@unmarshal_validator // validate_unmarshal(data) -> bool
@unmarshal_migrator  // migrate_data(data) -> map
```

#### 13.2.4 Field Control Hooks

```frame
@marshal_filter      // should_marshal(field_name) -> bool
@marshal_rename      // marshal_field_name(field_name) -> string
```

#### 13.2.5 Field-Level Annotations

```frame
@no_persist         // Exclude field from serialization
@persist_encrypted  // Encrypt field before serialization
@persist_as(name)   // Rename field in serialized form
```

### 13.3 System Serialization Format

When marshaling a Frame system, the following structure is generated:

```json
{
    "_frame_type": "SystemName",
    "_frame_version": "1.0",
    "_current_state": "StateName",
    "_state_variables": {
        "StateName": {
            "var_name": "value"
        }
    },
    "_domain_variables": {
        "var_name": "value"
    },
    "_state_stack": [],
    "_event_queue": []
}
```

### 13.4 Hook Integration Flow

```frame
// Marshal flow
1. Check for @marshal_hook -> use __marshal__()
2. Else check @before_marshal -> use prepare_for_marshal()
3. Else use default extraction
4. Apply @marshal_filter if present
5. Apply @marshal_rename if present
6. Add Frame metadata
7. Convert to JSON

// Unmarshal flow
1. Parse JSON
2. Apply @unmarshal_validator if present
3. Apply @unmarshal_migrator if present
4. Check for @unmarshal_hook -> use __unmarshal__()
5. Else create instance and apply @after_unmarshal if present
6. Restore Frame-specific state
```

### 13.5 Example: Complete Persistence Implementation

```frame
@persistable
system ShoppingCart {
    @marshal_hook
    static fn __marshal__(self) -> map {
        return {
            "items": self.items,
            "total": self.calculate_total(),
            "saved_at": now()
        }
    }
    
    @unmarshal_hook
    static fn __unmarshal__(data: map) -> ShoppingCart {
        var cart = ShoppingCart()
        cart.items = data["items"]
        cart.last_loaded = data["saved_at"]
        return cart
    }
    
    interface:
        save(filename: string) {
            var json = marshal(self)
            file.write(filename, json)
            return
        }
        
        load(filename: string) {
            var json = file.read(filename)
            var temp = unmarshal(json, ShoppingCart)
            self._copy_from(temp)
            return
        }
        
    domain:
        var items: map
        @no_persist
        var cache: map  // Excluded from serialization
        var last_loaded: string
}
```

### 13.6 Target Language Mappings

| Operation | Python | JavaScript | C | Java |
|-----------|---------|------------|---|------|
| marshal(obj) | json.dumps(obj.__dict__) | JSON.stringify(obj) | frame_marshal(obj) | gson.toJson(obj) |
| unmarshal(json, T) | json.loads(json) | JSON.parse(json) | frame_unmarshal(json) | gson.fromJson(json, T.class) |

## 14. Conclusion

The Frame Standard Library provides a clean abstraction over common operations while maintaining the efficiency of transpiled code. By carefully choosing what belongs in FSL and implementing it primarily through code generation rather than runtime libraries, Frame can provide a consistent, powerful programming model without sacrificing performance or target language compatibility.

The persistence system, built on marshal/unmarshal primitives with customizable hooks, provides a flexible and powerful way to serialize Frame systems while maintaining encapsulation and allowing for optimization.
# Frame Enum Iteration Proposal

## Overview

Enum iteration is a fundamental capability in most modern languages, allowing code to process all values of an enumeration. Frame currently lacks this feature, limiting use cases like generating UI options, validating input against all possible values, or implementing state machine traversals.

## Current State: No Iteration Support

### What Frame Can't Do Today
```frame
// This is NOT currently possible in Frame
enum Color {
    Red
    Green
    Blue
}

// Cannot iterate
for color in Color {  // ❌ Not supported
    print(color)
}

// Cannot get all values
var all_colors = Color.values()  // ❌ Not supported

// Cannot get count
var count = Color.len()  // ❌ Not supported
```

### Python's Enum Iteration Capabilities
```python
from enum import Enum

class Color(Enum):
    Red = 1
    Green = 2
    Blue = 3

# Direct iteration
for color in Color:
    print(color)  # Color.Red, Color.Green, Color.Blue

# Get all values
all_colors = list(Color)  # [<Color.Red: 1>, <Color.Green: 2>, <Color.Blue: 3>]

# Get count
count = len(Color)  # 3

# Access by name
color = Color['Red']  # Color.Red

# Access by value  
color = Color(1)  # Color.Red

# Get names and values
Color.Red.name   # 'Red'
Color.Red.value  # 1
```

## Proposed Frame Enum Iteration Design

### Option 1: Built-in Iterator Support (Recommended)

#### Syntax Design
```frame
enum Status {
    Pending
    Active
    Complete
    Cancelled
}

system TaskManager {
    interface:
        printAllStatuses()
        validateStatus(s: Status): bool
        
    machine:
        $Ready {
            printAllStatuses() {
                // Direct iteration over enum type
                for status in Status {
                    print("Status: " + status.name + " = " + status.value)
                }
                return
            }
            
            validateStatus(s: Status): bool {
                // Check if value exists in enum
                for valid in Status {
                    if s == valid {
                        return true
                    }
                }
                return false
            }
        }
}
```

#### Generated Python Code
```python
class TaskManager_Status(Enum):
    Pending = 0
    Active = 1
    Complete = 2
    Cancelled = 3

class TaskManager:
    def printAllStatuses(self):
        # Frame's 'for status in Status' becomes Python iteration
        for status in TaskManager_Status:
            print(f"Status: {status.name} = {status.value}")
        return
```

### Option 2: Enum Methods Approach

#### Syntax Design
```frame
enum Priority {
    Low = 1
    Medium = 5
    High = 10
}

system Scheduler {
    actions:
        processAllPriorities() {
            // Method call to get all values
            var priorities = Priority.values()
            for p in priorities {
                processPriority(p)
            }
            
            // Get count
            var count = Priority.count()
            print("Total priorities: " + count)
            
            // Get names
            var names = Priority.names()  // ["Low", "Medium", "High"]
            for name in names {
                print("Priority: " + name)
            }
        }
        
        processPriority(p: Priority) {
            print("Processing priority: " + p.name)
        }
}
```

### Option 3: Enum as Collection Type

#### Syntax Design
```frame
enum Department : string {
    Engineering = "ENG"
    Sales = "SALES"
    Marketing = "MKT"
    Support = "SUPPORT"
}

system EmployeeDirectory {
    actions:
        generateDepartmentReport() {
            // Treat enum as collection
            print("Departments: " + Department.len())
            
            // Index access
            var first = Department[0]  // Engineering
            var last = Department[-1]   // Support
            
            // Range iteration
            for i in range(0, Department.len()) {
                var dept = Department[i]
                print(i + ": " + dept.name + " (" + dept.value + ")")
            }
            
            // Filter/map operations (future)
            var eng_depts = Department.filter(|d| d.value.startsWith("ENG"))
        }
}
```

## Implementation Requirements

### 1. Parser Changes

#### For Direct Iteration (Option 1)
```rust
// In parser.rs - extend for_statement parsing
fn for_statement(&mut self) -> Result<ForStmtNode, ParseError> {
    // ... existing code ...
    
    // Check if iterating over enum type
    if let Some(symbol) = self.arcanum.lookup(&collection_name) {
        match symbol {
            SymbolType::EnumDeclSymbolT { .. } => {
                // Mark this as enum iteration for code generation
                for_stmt_node.is_enum_iteration = true;
                for_stmt_node.enum_type = Some(collection_name);
            }
            _ => { /* regular collection */ }
        }
    }
}
```

### 2. AST Extensions

```rust
// In ast.rs
pub struct ForStmtNode {
    pub var_ident: String,
    pub collection_expr: ExprType,
    pub statements: Vec<DeclOrStmtType>,
    pub is_enum_iteration: bool,  // New field
    pub enum_type: Option<String>, // New field
}

// New enum property access node
pub struct EnumPropertyNode {
    pub enum_expr: EnumeratorExprNode,
    pub property: EnumProperty,
}

pub enum EnumProperty {
    Name,
    Value,
    Ordinal,  // Position in enum
}
```

### 3. Code Generation (Python Visitor)

```rust
// In python_visitor.rs
fn visit_for_stmt_node(&mut self, for_stmt_node: &ForStmtNode) {
    if for_stmt_node.is_enum_iteration {
        // Generate enum iteration
        let enum_name = format!("{}_{}", 
            self.system_name, 
            for_stmt_node.enum_type.as_ref().unwrap());
        
        self.newline();
        self.add_code(&format!("for {} in {}:", 
            for_stmt_node.var_ident,
            enum_name));
        self.indent();
        
        // Process loop body
        for statement in &for_stmt_node.statements {
            statement.accept(self);
        }
        self.outdent();
    } else {
        // Regular collection iteration
        // ... existing code ...
    }
}
```

## Use Cases and Examples

### 1. Menu Generation
```frame
enum MenuOption {
    NewFile
    OpenFile
    SaveFile
    SaveAs
    Exit
}

system Application {
    actions:
        displayMenu() {
            print("=== File Menu ===")
            var index = 1
            for option in MenuOption {
                print(index + ". " + option.name)
                index = index + 1
            }
        }
        
        handleSelection(index: int): MenuOption {
            var current = 1
            for option in MenuOption {
                if current == index {
                    return option
                }
                current = current + 1
            }
            return MenuOption.Exit  // Default
        }
}
```

### 2. State Machine Visualization
```frame
enum State {
    Idle
    Loading
    Processing
    Complete
    Error
}

system StateMachine {
    actions:
        generateDotGraph() {
            var dot = "digraph StateMachine {\n"
            
            // Add all states as nodes
            for state in State {
                dot = dot + "  " + state.name + ";\n"
            }
            
            // Add transitions (simplified)
            dot = dot + "  Idle -> Loading;\n"
            dot = dot + "  Loading -> Processing;\n"
            dot = dot + "  Processing -> Complete;\n"
            dot = dot + "}\n"
            
            return dot
        }
}
```

### 3. Validation and Conversion
```frame
enum LogLevel : string {
    Debug = "DEBUG"
    Info = "INFO"
    Warning = "WARNING"
    Error = "ERROR"
}

system Logger {
    actions:
        parseLogLevel(input: string): LogLevel {
            // Iterate to find matching value
            for level in LogLevel {
                if level.value == input {
                    return level
                }
            }
            
            // Try case-insensitive match on name
            var upper_input = input.toUpper()
            for level in LogLevel {
                if level.name.toUpper() == upper_input {
                    return level
                }
            }
            
            // Default
            return LogLevel.Info
        }
        
        getAllLevelNames(): list<string> {
            var names = []
            for level in LogLevel {
                names.append(level.name)
            }
            return names
        }
}
```

### 4. Testing All Enum Values
```frame
enum HttpMethod {
    GET
    POST
    PUT
    DELETE
    PATCH
}

system HttpTester {
    actions:
        testAllMethods(url: string) {
            for method in HttpMethod {
                var response = sendRequest(url, method)
                print(method.name + ": " + response.status)
            }
        }
        
        generateTestCases(): list<TestCase> {
            var cases = []
            for method in HttpMethod {
                cases.append(TestCase(method, "/" + method.name.toLower()))
            }
            return cases
        }
}
```

## Comparison with Other Languages

| Feature | Python | Rust | TypeScript | Java | Frame (Proposed) |
|---------|--------|------|------------|------|------------------|
| Direct iteration | ✅ `for e in Enum` | ✅ `Enum::iter()` | ❌ | ✅ `Enum.values()` | ✅ `for e in Enum` |
| Get all values | ✅ `list(Enum)` | ✅ `Enum::variants()` | ✅ `Object.values()` | ✅ `.values()` | ✅ `Enum.values()` |
| Get count | ✅ `len(Enum)` | ✅ `Enum::COUNT` | ✅ `.length` | ✅ `.values().length` | ✅ `Enum.count()` |
| Access by index | ❌ | ✅ | ✅ | ✅ | ⚠️ (proposed) |
| Get names | ✅ `.name` | ✅ | ✅ `Object.keys()` | ✅ `.name()` | ✅ `.name` |
| Get values | ✅ `.value` | ✅ | ✅ `Object.values()` | ✅ `.ordinal()` | ✅ `.value` |

## Design Decisions

### 1. Iteration Syntax

**Recommendation**: Support direct iteration (`for e in EnumType`)
- Most intuitive and readable
- Consistent with Python target
- Natural for Frame's domain

### 2. Property Access

**Recommendation**: Support `.name` and `.value` properties
- Essential for practical use
- Matches Python's model
- Clear and explicit

### 3. Collection Methods

**Recommendation**: Start with basics, expand later
- Phase 1: `for...in` iteration
- Phase 2: `.values()`, `.count()`, `.names()`
- Phase 3: Index access, filtering (if needed)

### 4. Type Safety

**Recommendation**: Maintain strong typing
```frame
// Should be type-safe
for color in Color {  // color is type Color
    processColor(color)  // Type-checked
}

// Values should return typed list
var colors: list<Color> = Color.values()
```

## Implementation Priority

### Phase 1: Basic Iteration (Critical)
- `for item in EnumType` syntax
- `.name` and `.value` properties
- Enables 80% of use cases

### Phase 2: Collection Methods (Important)
- `EnumType.values()` → list of enum values
- `EnumType.count()` → integer count
- `EnumType.names()` → list of strings

### Phase 3: Advanced Features (Nice to Have)
- Index access: `EnumType[0]`
- Lookup by name: `EnumType.fromName("Red")`
- Lookup by value: `EnumType.fromValue(1)`

## Benefits of Enum Iteration

1. **Dynamic UIs**: Generate menus, dropdowns, radio buttons from enums
2. **Validation**: Check if values are valid enum members
3. **Testing**: Automatically test all enum cases
4. **Documentation**: Generate docs from enum definitions
5. **Serialization**: Convert enums to/from JSON, databases
6. **State Machines**: Traverse all possible states
7. **Configuration**: Process all configuration options

## Challenges and Solutions

### Challenge 1: Performance
**Issue**: Iteration creates overhead
**Solution**: Generate static lists at compile time for Python

### Challenge 2: Order Guarantee
**Issue**: Should iteration order be guaranteed?
**Solution**: Yes - declaration order (matches Python)

### Challenge 3: Modified During Iteration
**Issue**: What if enum is conceptually modified?
**Solution**: Enums are immutable by design

### Challenge 4: Empty Enums
**Issue**: How to handle empty enum iteration?
**Solution**: Empty iteration (no loop body execution)

## Conclusion

Enum iteration is a critical missing feature in Frame that would significantly enhance its practical utility. The proposed implementation:

1. **Aligns with Python**: Natural transpilation to Python's enum iteration
2. **Maintains Simplicity**: Clean, intuitive syntax
3. **Enables Common Patterns**: UI generation, validation, testing
4. **Type-Safe**: Preserves Frame's type safety guarantees

The recommended approach is to implement basic iteration (`for...in`) with property access (`.name`, `.value`) as the highest priority, followed by collection methods in a later phase. This would bring Frame's enum capabilities closer to modern language standards while maintaining its elegant simplicity.
# Frame Enum Enhancement Proposal: Custom Values and String Support

## Executive Summary

This proposal outlines how Frame could be enhanced to support custom enum values and string enums, aligning more closely with Python's enum capabilities while maintaining Frame's simplicity and domain-oriented design philosophy.

## Current Implementation Analysis

### Parser Implementation (parser.rs:2983-3080)
```rust
fn enum_decl(&mut self) -> Result<Rc<RefCell<EnumDeclNode>>, ParseError> {
    // ...
    let mut enums = Vec::new();
    let mut enum_value = 0;  // Auto-incrementing counter
    
    // Parse enum members
    while self.match_token(&[TokenType::Identifier]) {
        let enum_name = self.previous().lexeme.clone();
        enums.push(Rc::new(EnumeratorDeclNode::new(enum_name, enum_value)));
        enum_value += 1;  // Simple increment
        // ...
    }
}
```

### AST Structure (ast.rs:954-962)
```rust
pub struct EnumeratorDeclNode {
    pub name: String,
    pub value: i32,  // Currently only supports i32
}
```

## Proposed Enhancement 1: Custom Integer Values

### Syntax Design

```frame
// Current (auto-increment only)
enum Priority {
    Low      // 0
    Medium   // 1
    High     // 2
}

// Proposed: Explicit values
enum Priority {
    Low = 1
    Medium = 5
    High = 10
}

// Proposed: Mixed (auto continues from last explicit)
enum Status {
    Unknown = -1
    Idle = 0
    Running     // 1 (auto)
    Paused      // 2 (auto)
    Complete = 100
    Error       // 101 (auto continues from 100)
}
```

### Implementation Changes Required

#### 1. Parser Modifications
```rust
fn enum_decl(&mut self) -> Result<Rc<RefCell<EnumDeclNode>>, ParseError> {
    let mut enums = Vec::new();
    let mut enum_value = 0;
    
    while self.match_token(&[TokenType::Identifier]) {
        let enum_name = self.previous().lexeme.clone();
        
        // Check for explicit value assignment
        if self.match_token(&[TokenType::Equals]) {
            // Parse expression (initially just integer literals)
            if self.match_token(&[TokenType::Number]) {
                enum_value = self.previous().lexeme.parse::<i32>()
                    .unwrap_or_else(|_| {
                        self.error_at_current("Invalid enum value");
                        0
                    });
            } else {
                self.error_at_current("Expected integer value after '='");
            }
        }
        
        enums.push(Rc::new(EnumeratorDeclNode::new(enum_name, enum_value)));
        enum_value += 1;  // Auto-increment for next member
        
        // Handle comma or newline
        if !self.match_token(&[TokenType::Comma, TokenType::Newline]) {
            break;
        }
    }
}
```

#### 2. Benefits
- **Compatibility**: Can represent existing Python/C enums accurately
- **Flexibility**: Supports sparse values for flags, error codes
- **Backward Compatible**: Existing Frame enums work unchanged

#### 3. Challenges
- **Duplicate Values**: Need validation to prevent/allow duplicates
- **Negative Values**: Parser must handle negative numbers
- **Expression Evaluation**: Could support `1 << 2` for flags

## Proposed Enhancement 2: String Enums

### Syntax Design

```frame
// Proposed: String enum with explicit type
enum Color : string {
    Red = "red"
    Green = "green"  
    Blue = "blue"
}

// Proposed: Auto string values (use member name)
enum LogLevel : string {
    Debug    // "Debug"
    Info     // "Info"
    Warning  // "Warning"
    Error    // "Error"
}

// Mixed types NOT allowed (Frame principle: consistency)
enum Invalid : string {
    First = "one"
    Second = 2  // ERROR: Type mismatch
}
```

### Implementation Changes Required

#### 1. AST Modifications
```rust
// Current
pub struct EnumeratorDeclNode {
    pub name: String,
    pub value: i32,
}

// Proposed
pub enum EnumValue {
    Integer(i32),
    String(String),
    Auto,  // Compiler determines based on enum type
}

pub struct EnumeratorDeclNode {
    pub name: String,
    pub value: EnumValue,
}

pub struct EnumDeclNode {
    pub name: String,
    pub enum_type: EnumType,  // New field
    pub enums: Vec<Rc<EnumeratorDeclNode>>,
}

pub enum EnumType {
    Integer,  // Default
    String,
}
```

#### 2. Parser Modifications
```rust
fn enum_decl(&mut self) -> Result<Rc<RefCell<EnumDeclNode>>, ParseError> {
    let identifier = self.consume_identifier("enum name")?;
    
    // Check for type annotation
    let enum_type = if self.match_token(&[TokenType::Colon]) {
        if self.match_token(&[TokenType::Identifier]) {
            match self.previous().lexeme.as_str() {
                "string" => EnumType::String,
                "int" => EnumType::Integer,
                _ => {
                    self.error_at_current("Invalid enum type (use 'string' or 'int')");
                    EnumType::Integer
                }
            }
        } else {
            self.error_at_current("Expected type after ':'");
            EnumType::Integer
        }
    } else {
        EnumType::Integer  // Default
    };
    
    // Parse members with type-appropriate values
    // ...
}
```

#### 3. Python Code Generation
```python
# Integer enum (current)
class System_Priority(Enum):
    Low = 1
    Medium = 5
    High = 10

# String enum (proposed)
class System_Color(Enum):
    Red = "red"
    Green = "green"
    Blue = "blue"

# Auto string enum
class System_LogLevel(Enum):
    Debug = "Debug"
    Info = "Info"
    Warning = "Warning"
    Error = "Error"
```

## Design Decisions and Trade-offs

### 1. Type Safety vs Flexibility

**Option A: Strict Type Declaration** (Recommended)
```frame
enum Status : int {     // Explicit type
    Active = 1
    Inactive = 0
}

enum Color : string {   // Must use strings
    Red = "red"
    Blue = "blue"
}
```

**Option B: Inferred Types**
```frame
enum Mixed {
    One = 1        // Inferred as int enum
    Two = "two"    // ERROR: Inconsistent types
}
```

**Recommendation**: Option A - Explicit types prevent confusion and errors

### 2. Auto Values for String Enums

**Option A: Use Member Name** (Recommended)
```frame
enum LogLevel : string {
    Debug    // "Debug"
    Info     // "Info"
}
```

**Option B: Require Explicit Values**
```frame
enum LogLevel : string {
    Debug = "DEBUG"    // Must specify
    Info = "INFO"
}
```

**Recommendation**: Option A with Option B available - Maximum flexibility

### 3. Complex Value Expressions

**Option A: Literals Only** (Recommended for Phase 1)
```frame
enum Flags {
    Read = 4
    Write = 2
    Execute = 1
}
```

**Option B: Allow Expressions** (Future Enhancement)
```frame
enum Flags {
    Read = 1 << 2     // 4
    Write = 1 << 1    // 2
    Execute = 1 << 0  // 1
    All = Read | Write | Execute  // 7
}
```

**Recommendation**: Start with Option A, consider B for future

## Migration Path

### Phase 1: Custom Integer Values (v0.32)
- Add `=` value assignment to parser
- Support negative integers
- Maintain auto-increment behavior
- 100% backward compatible

### Phase 2: String Enums (v0.33)
- Add `: type` syntax to parser
- Extend AST with EnumType
- Update all visitors
- Add validation for type consistency

### Phase 3: Advanced Features (Future)
- Expression evaluation for values
- Enum methods (if needed)
- Flag enum support with operators

## Code Examples

### Before Enhancement
```frame
system ConfigManager {
    domain:
        // Limited to sequential integers from 0
        enum LogLevel {
            Debug     // 0
            Info      // 1
            Warning   // 2
            Error     // 3
        }
        
        // Can't represent actual HTTP codes
        enum HttpStatus {
            Ok           // 0 (want 200)
            NotFound     // 1 (want 404)
            ServerError  // 2 (want 500)
        }
}
```

### After Enhancement
```frame
system ConfigManager {
    domain:
        // String enum for clarity
        enum LogLevel : string {
            Debug = "DEBUG"
            Info = "INFO"
            Warning = "WARNING"
            Error = "ERROR"
            Critical = "CRITICAL"
        }
        
        // Accurate HTTP status codes
        enum HttpStatus : int {
            Ok = 200
            Created = 201
            BadRequest = 400
            NotFound = 404
            ServerError = 500
        }
        
        // Database states with meaningful strings
        enum DbState : string {
            Disconnected = "disconnected"
            Connecting = "connecting"
            Connected = "connected"
            Error = "error"
        }
}
```

### Usage Examples
```frame
system WebServer {
    interface:
        handleRequest(path: string): HttpStatus
        
    machine:
        $Serving {
            handleRequest(path: string): HttpStatus {
                if path == "/health" {
                    log(LogLevel.Info, "Health check")
                    return HttpStatus.Ok
                }
                
                if !exists(path) {
                    log(LogLevel.Warning, "Path not found: " + path)
                    return HttpStatus.NotFound
                }
                
                return HttpStatus.Ok
            }
        }
        
    actions:
        log(level: LogLevel, message: string) {
            // String enums can be used directly
            print(level + ": " + message)
        }
}
```

## Impact Analysis

### Positive Impacts
1. **Better Interoperability**: Can accurately represent external enums
2. **Clearer Intent**: String enums for human-readable values
3. **Reduced Errors**: No need to remember magic numbers
4. **Type Safety**: Explicit types prevent mixing
5. **Real-world Alignment**: Matches actual use cases

### Potential Concerns
1. **Complexity**: More syntax to learn
2. **Type Checking**: Need robust validation
3. **Documentation**: Must explain type system
4. **Testing**: More edge cases to handle

## Comparison with Competitors

| Feature | Frame (Current) | Frame (Proposed) | Python | Rust | TypeScript |
|---------|----------------|------------------|--------|------|------------|
| Auto Values | ✅ | ✅ | ✅ | ✅ | ✅ |
| Custom Int | ❌ | ✅ | ✅ | ✅ | ✅ |
| String Values | ❌ | ✅ | ✅ | ✅ | ✅ |
| Mixed Types | ❌ | ❌ | ✅ | ❌ | ✅ |
| Type Safety | ✅ | ✅ | ⚠️ | ✅ | ✅ |

## Recommendation

Implement both enhancements in phases:

1. **Immediate (v0.32)**: Custom integer values
   - Low risk, high value
   - Backward compatible
   - Solves common pain points

2. **Near-term (v0.33)**: String enums with explicit types
   - Medium complexity
   - High user value
   - Maintains Frame's type safety

3. **Future**: Consider expression evaluation and advanced features based on user feedback

This approach balances Frame's simplicity principle with practical needs while maintaining type safety and clarity.
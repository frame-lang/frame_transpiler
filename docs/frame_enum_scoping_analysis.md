# Frame Enum Scoping Analysis: Alignment with Python

## Executive Summary

Python **fully supports class-scoped enums**, which aligns perfectly with Frame's current design of system-scoped enums. Frame's approach of generating enums as `SystemName_EnumName` could be evolved to use Python's native class-scoped enums for better encapsulation and more Pythonic code.

## Current Frame Approach

### Frame Source
```frame
system OrderManager {
    domain:
        enum Status {
            Pending
            Processing
            Shipped
            Delivered
        }
}

system PaymentProcessor {
    domain:
        enum Status {
            Pending
            Authorized
            Captured
            Refunded
        }
}
```

### Current Generated Python (Module-Level with Prefixing)
```python
# Frame currently generates at module level with prefixes
class OrderManager_Status(Enum):
    Pending = 0
    Processing = 1
    Shipped = 2
    Delivered = 3

class PaymentProcessor_Status(Enum):
    Pending = 0
    Authorized = 1
    Captured = 2
    Refunded = 3

class OrderManager:
    def check_status(self):
        if self.status == OrderManager_Status.Pending:
            # ...
```

## Proposed Evolution: Native Class-Scoped Enums

### Proposed Generated Python (Class-Scoped)
```python
class OrderManager:
    # Enum nested inside the system class
    class Status(Enum):
        Pending = 0
        Processing = 1
        Shipped = 2
        Delivered = 3
    
    def check_status(self):
        if self.status == self.Status.Pending:  # More natural
            # ...

class PaymentProcessor:
    # Same enum name, no collision due to class scoping
    class Status(Enum):
        Pending = 0
        Authorized = 1
        Captured = 2
        Refunded = 3
    
    def process(self):
        if self.status == self.Status.Authorized:
            # ...
```

## Benefits of Class-Scoped Enums

### 1. Natural Namespacing
- **Current**: `OrderManager_Status.Pending`
- **Proposed**: `OrderManager.Status.Pending`
- More Pythonic and clearer relationship

### 2. No Name Collisions
```python
# Different Status enums in different classes - no problem!
OrderManager.Status.Pending      # Value: 0
PaymentProcessor.Status.Pending  # Value: 0 (same name, different enum)
```

### 3. Better Encapsulation
```python
class System:
    class InternalState(Enum):  # Clearly belongs to System
        INIT = 0
        READY = 1
        ERROR = 2
    
    def __init__(self):
        self._state = self.InternalState.INIT
```

### 4. Cleaner Internal Access
```python
class OrderManager:
    class Status(Enum):
        Pending = 0
        Shipped = 1
    
    def update_status(self):
        # Inside the class, can use self.Status
        self.current = self.Status.Shipped  # Clean!
        
        # Or just Status (Python finds it in class scope)
        for status in self.Status:  # Iteration works!
            print(status)
```

## Implementation Comparison

### Access Patterns

| Context | Current Frame | Proposed Class-Scoped |
|---------|--------------|----------------------|
| Inside system | `SystemName_EnumName.Member` | `self.EnumName.Member` or `EnumName.Member` |
| Outside system | `SystemName_EnumName.Member` | `SystemName.EnumName.Member` |
| From another system | `OtherSystem_EnumName.Member` | `OtherSystem.EnumName.Member` |
| Type annotation | `: SystemName_EnumName` | `: SystemName.EnumName` |

### Code Generation Changes

#### Current Visitor Pattern
```rust
// python_visitor.rs - Current implementation
fn visit_enum_decl_node(&mut self, enum_decl_node: &EnumDeclNode) {
    // Generates at module level with prefix
    self.add_code(&format!(
        "class {}_{}(Enum):",
        self.system_name, enum_decl_node.name
    ));
    // ...
}
```

#### Proposed Visitor Pattern
```rust
// python_visitor.rs - Proposed implementation
fn visit_enum_decl_node(&mut self, enum_decl_node: &EnumDeclNode) {
    // Generate inside the system class
    self.indent();  // Already inside system class
    self.newline();
    self.add_code(&format!(
        "class {}(Enum):",
        enum_decl_node.name  // No prefix needed!
    ));
    self.indent();
    // ... generate members ...
    self.outdent();
}
```

## Migration Path

### Option 1: Backward Compatible (Recommended)
Generate both styles with a flag:
```python
# Generated with --enum-style=compatible
class OrderManager_Status(Enum):  # Old style for compatibility
    Pending = 0

class OrderManager:
    Status = OrderManager_Status  # Alias for new style
    
    def check(self):
        # Both work:
        self.Status.Pending          # New style
        OrderManager_Status.Pending  # Old style
```

### Option 2: Clean Break
Generate only class-scoped (requires migration):
```python
# Generated with --enum-style=class-scoped
class OrderManager:
    class Status(Enum):
        Pending = 0
    # No module-level enum
```

## Test Case Validation

### Python Test Results
```python
✅ Python FULLY supports class-scoped enums
✅ Each class can have its own enum with the same name (no collision)
✅ Class enums are accessible as ClassName.EnumName.Member
✅ Instances can access via self.EnumName.Member
✅ Full iteration support for class-scoped enums
✅ Type safety maintained (different class enums are different types)
```

### Frame Usage Examples

#### Current Syntax (No Change Needed!)
```frame
system OrderManager {
    domain:
        enum Status {
            Pending
            Shipped
        }
    
    machine:
        $Active {
            checkOrder() {
                // Frame syntax stays the same!
                if self.order_status == Status.Pending {
                    updateStatus(Status.Shipped)
                }
                
                // Iteration (proposed)
                for s in Status {
                    print(s.name)
                }
            }
        }
}
```

#### Cross-System Access
```frame
system Monitor {
    actions:
        checkOrderSystem(order: OrderManager) {
            // Cross-system enum access
            if order.getStatus() == OrderManager.Status.Pending {
                alert("Order pending")
            }
        }
}
```

## Advantages of Class-Scoped Approach

1. **More Pythonic**: Follows Python conventions and idioms
2. **Better IDE Support**: IDEs understand the relationship better
3. **Cleaner Imports**: No need to import each enum separately
4. **Logical Grouping**: Enums clearly belong to their systems
5. **Simplified Naming**: No artificial prefixes needed
6. **Natural Hierarchy**: Reflects Frame's system.enum structure

## Challenges and Solutions

### Challenge 1: External Access
**Issue**: How to access from outside the system?
**Solution**: Use qualified name: `SystemName.EnumName.Member`

### Challenge 2: Type Annotations
**Issue**: Function parameters need full type path
**Solution**: Generate proper type hints:
```python
def process(status: OrderManager.Status) -> PaymentProcessor.Status:
    # ...
```

### Challenge 3: Import Statements
**Issue**: Can't import nested class directly in Python
**Solution**: Import the system class:
```python
from order_module import OrderManager
# Use: OrderManager.Status.Pending
```

## Recommendation

### Phase 1: Add Class-Scoped as Option
- Add `--enum-scoping=class` flag to framec
- Default to current behavior for compatibility
- Allow projects to opt-in to class-scoped

### Phase 2: Transition Period
- Generate both with aliases
- Deprecation warnings for old style
- Documentation and migration guides

### Phase 3: Class-Scoped as Default
- Make class-scoped the default
- Keep flag for legacy compatibility
- Update all documentation

## Conclusion

Python's full support for class-scoped enums validates Frame's system-scoped enum design. Moving to class-scoped enum generation would:

1. **Eliminate** the need for `SystemName_` prefixing
2. **Improve** code readability and Pythonic style
3. **Maintain** complete type safety and isolation
4. **Enable** more natural access patterns (`self.Status` inside systems)

This evolution would make Frame's generated Python code more idiomatic while preserving all current functionality and improving the developer experience.
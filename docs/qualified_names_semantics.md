# Frame Qualified Names - Semantic Specification

## Purpose
This document defines the semantic requirements for module-qualified names in Frame, ensuring compatibility with all first-class target languages (Python, Rust, Go, Java, C#, JavaScript).

## Semantic Requirements

### 1. Module-Qualified Function Calls

#### Frame Syntax
```frame
module utils {
    fn helper() { return 42 }
    
    module math {
        fn add(a, b) { return a + b }
    }
}

fn main() {
    var x = utils.helper()          // Module.function
    var y = utils.math.add(2, 3)    // Module.submodule.function
}
```

#### Target Language Mappings

**Python** (Our implementation):
```python
# Module as namespace (class or actual module)
class utils:
    @staticmethod
    def helper():
        return 42
    
    class math:
        @staticmethod
        def add(a, b):
            return a + b

def main():
    x = utils.helper()
    y = utils.math.add(2, 3)
```

**Rust** (Must support):
```rust
mod utils {
    pub fn helper() -> i32 { 42 }
    
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 { a + b }
    }
}

fn main() {
    let x = utils::helper();
    let y = utils::math::add(2, 3);
}
```

**Go** (Must support):
```go
package main

// utils package functions
func utilsHelper() int { return 42 }
func utilsMathAdd(a, b int) int { return a + b }

func main() {
    x := utilsHelper()        // Flattened namespace
    y := utilsMathAdd(2, 3)   // Go doesn't have nested packages
}
```

**Java/C#** (Must support):
```java
class Utils {
    public static int helper() { return 42; }
    
    public static class Math {
        public static int add(int a, int b) { return a + b; }
    }
}

class Main {
    public static void main(String[] args) {
        int x = Utils.helper();
        int y = Utils.Math.add(2, 3);
    }
}
```

**JavaScript** (Must support):
```javascript
const utils = {
    helper() { return 42; },
    
    math: {
        add(a, b) { return a + b; }
    }
};

function main() {
    const x = utils.helper();
    const y = utils.math.add(2, 3);
}
```

### 2. Module-Qualified Type References

#### Frame Syntax
```frame
module models {
    system User {
        // System definition
    }
}

fn main() {
    var user = models.User()  // Module.Type
}
```

#### Target Mappings
- **Python**: `user = models.User()`
- **Rust**: `let user = models::User::new();`
- **Go**: `user := models.NewUser()`
- **Java/C#**: `var user = new Models.User();`
- **JavaScript**: `const user = new models.User();`

### 3. Module-Qualified Enum References

#### Frame Syntax
```frame
module constants {
    enum Status {
        Active,
        Inactive
    }
}

fn main() {
    var s = constants.Status.Active  // Module.Enum.Member
}
```

#### Target Mappings
- **Python**: `s = constants.Status.Active`
- **Rust**: `let s = constants::Status::Active;`
- **Go**: `s := constants.StatusActive`
- **Java/C#**: `var s = Constants.Status.ACTIVE;`
- **JavaScript**: `const s = constants.Status.Active;`

## Implementation Strategy

### Phase 1: Parser Enhancement (Current)
1. Recognize `module.identifier` patterns
2. Build qualified name AST nodes
3. Track module namespace in symbol table

### Phase 2: Symbol Resolution
1. Look up symbols across module boundaries
2. Validate qualified names against module contents
3. Error on unresolved qualified names

### Phase 3: Code Generation
1. Generate appropriate syntax for Python
2. Document patterns for LLM translation
3. Ensure semantic equivalence

## Validation Rules

1. **Module must exist**: `utils.helper()` requires module `utils` to be defined
2. **Symbol must exist in module**: `helper` must be defined in `utils`
3. **Accessibility**: All module members are public by default
4. **No circular dependencies**: Modules cannot import each other circularly

## Error Messages

```frame
// Error: Module 'utils' not found
var x = utils.helper()

// Error: Function 'helper' not found in module 'utils'
module utils {}
var x = utils.helper()

// Error: Cannot access 'helper' - not a module member
fn helper() {}
var x = main.helper()  // 'main' is not a module
```

## LLM Translation Guidelines

When translating Frame with qualified names to target languages:

1. **Preserve namespace hierarchy**: Module structure must map to target language namespaces
2. **Static vs Instance**: Module functions are always static/class methods
3. **Visibility**: All module members are public unless target language requires explicit marking
4. **Naming conventions**: Follow target language conventions (camelCase, snake_case, etc.)
5. **Import statements**: Generate appropriate import/using/require statements

## Test Cases

### Basic Module Function
```frame
module utils {
    fn double(x) { return x * 2 }
}

fn main() {
    assert(utils.double(5) == 10)
}
```

### Nested Modules
```frame
module app {
    module services {
        fn process() { return "processed" }
    }
}

fn main() {
    assert(app.services.process() == "processed")
}
```

### Mixed Qualified and Unqualified
```frame
module helpers {
    fn help() { return "helping" }
}

fn local() { return "local" }

fn main() {
    var a = helpers.help()  // Qualified
    var b = local()         // Unqualified
}
```

## Future Considerations

1. **Private modules**: `private module internal { ... }`
2. **Module aliases**: `import module utils as u`
3. **Re-exports**: `export module.function as myFunction`
4. **Module interfaces**: Define contracts for modules
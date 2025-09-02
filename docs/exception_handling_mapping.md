# Frame Exception Handling - Cross-Language Mapping

## Overview
This document describes how Frame's exception handling (based on Python's model) maps to various target languages.

## Frame Syntax
```frame
try {
    // code that may throw
}
except ExceptionType as e {
    // handle specific exception
}
except (Type1, Type2) as err {
    // handle multiple exception types
}
else {
    // runs if no exception
}
finally {
    // cleanup code
}

// Raise exception
raise ExceptionType("message")
raise NewException("message") from original_exception
```

## Language Mappings

### 1. Python (Reference Implementation) ✅
Direct 1:1 mapping as Frame's model is based on Python:
```python
try:
    # code that may throw
except ExceptionType as e:
    # handle specific exception
except (Type1, Type2) as err:
    # handle multiple exception types
else:
    # runs if no exception
finally:
    # cleanup code

raise ExceptionType("message")
raise NewException("message") from original_exception
```

### 2. JavaScript/TypeScript
JavaScript has try-catch-finally but no else clause or multiple exception types:

```javascript
try {
    // code that may throw
} catch (e) {
    if (e instanceof ExceptionType) {
        // handle specific exception
    } else if (e instanceof Type1 || e instanceof Type2) {
        let err = e;
        // handle multiple exception types
    } else {
        throw e; // re-throw unhandled
    }
} finally {
    // cleanup code
}

// For else clause, use a flag:
let __succeeded = false;
try {
    // code
    __succeeded = true;
} catch (e) {
    // handle
} finally {
    if (__succeeded) {
        // else block code
    }
    // finally code
}

// Raise
throw new ExceptionType("message");
// No native "from" support - could use Error.cause in modern JS
throw new NewException("message", { cause: original_exception });
```

### 3. Java/C#
Similar structure but different syntax:

#### Java:
```java
try {
    // code that may throw
} catch (ExceptionType e) {
    // handle specific exception
} catch (Type1 | Type2 err) {  // Java 7+ multi-catch
    // handle multiple exception types
} finally {
    // cleanup code
}

// For else clause, use a flag:
boolean succeeded = false;
try {
    // code
    succeeded = true;
} catch (Exception e) {
    // handle
} finally {
    if (succeeded) {
        // else block code
    }
    // finally code
}

// Raise
throw new ExceptionType("message");
// Chain exceptions
throw new NewException("message", originalException);
```

#### C#:
```csharp
try {
    // code that may throw
} catch (ExceptionType e) {
    // handle specific exception
} catch (Exception err) when (err is Type1 || err is Type2) {
    // handle multiple exception types
} finally {
    // cleanup code
}

// Else clause using flag (same as Java)

// Raise
throw new ExceptionType("message");
// Chain exceptions
throw new NewException("message", originalException);
```

### 4. Go
Go uses error returns instead of exceptions, requiring significant transformation:

```go
// Simple error handling
err := someFunctionThatMayError()
if err != nil {
    // handle error
}

// For try-catch simulation, use defer/recover:
func tryBlock() (err error) {
    defer func() {
        if r := recover(); r != nil {
            // This is like catch block
            switch e := r.(type) {
            case ExceptionType:
                // handle specific exception
                err = e
            case Type1, Type2:
                // handle multiple types
                err = e.(error)
            default:
                panic(r) // re-throw unhandled
            }
        }
        // This is like finally block
        cleanupCode()
    }()
    
    // Try block code
    codeThatMayPanic()
    
    // Else block (runs if no panic)
    elseBlockCode()
    
    return nil
}

// Raise
panic(errors.New("message"))
// Chain errors (Go 1.13+)
panic(fmt.Errorf("message: %w", originalError))
```

### 5. Rust
Rust uses Result<T, E> for error handling:

```rust
// Try-catch pattern using Result
let result = (|| -> Result<(), Box<dyn Error>> {
    // try block code
    code_that_may_error()?;
    Ok(())
})();

match result {
    Ok(_) => {
        // else block - runs if no error
    }
    Err(e) => {
        // catch block
        if e.is::<ExceptionType>() {
            // handle specific exception
        } else if e.is::<Type1>() || e.is::<Type2>() {
            // handle multiple types
        } else {
            return Err(e); // re-throw
        }
    }
}

// Finally block using drop guard or defer-like pattern
struct Finally<F: FnMut()>(Option<F>);
impl<F: FnMut()> Drop for Finally<F> {
    fn drop(&mut self) {
        if let Some(mut f) = self.0.take() {
            f();
        }
    }
}

let _finally = Finally(Some(|| {
    // cleanup code
}));

// Raise
return Err(Box::new(ExceptionType::new("message")));
// Chain errors
return Err(Box::new(NewException::new("message")).context(original_error));
```

## Implementation Strategy

### Phase 1: Python (COMPLETED) ✅
- Direct mapping of all features
- Full support for try/except/else/finally
- Raise with from clause

### Phase 2: JavaScript (NEXT)
- Implement type checking in catch blocks
- Simulate else clause with success flag
- Map raise to throw
- Use Error.cause for exception chaining

### Phase 3: Java/C#
- Use multi-catch for multiple exception types
- Simulate else clause with success flag
- Map exception chaining to constructor patterns

### Phase 4: Go
- Transform to defer/recover pattern
- Or use error return pattern based on context
- Map raise to panic (with caution)

### Phase 5: Rust
- Transform to Result-based error handling
- Use match for exception type dispatch
- Implement cleanup with Drop guards

## Design Decisions

1. **Else Clause**: Since most languages don't have Python's else clause, we'll use a success flag pattern
2. **Multiple Exception Types**: Use language-specific features (Java multi-catch, C# when clause, JS instanceof chain)
3. **Exception Chaining**: Map to language-specific patterns (Java/C# constructors, JS Error.cause, Go error wrapping)
4. **Finally**: All major languages support this directly except Go/Rust which need special patterns

## Frame Transpiler Configuration

Future enhancement: Add configuration option to choose error handling style:
```frame
@error_handling("exceptions")  // Use exceptions (default)
@error_handling("result")      // Use Result/Option pattern (Rust/Go)
@error_handling("error_return") // Use error return values (Go)
```

## Test Coverage Requirements

Each language implementation should handle:
1. Basic try-catch
2. Specific exception types
3. Multiple exception types in one handler
4. Variable binding for caught exceptions
5. Else clause (success case)
6. Finally clause
7. Raise/throw statements
8. Exception chaining (from clause)
9. Re-raising (bare raise)
10. Nested try-catch blocks
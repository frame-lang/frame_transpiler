> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 Language Features

## Type System

### Type Annotations

Frame supports type annotations for parameters and return values:

```python
@@target python

system TypedSystem {
    interface:
        process(data: str): dict
        calculate(x: int, y: float): float
    
    operations:
        validateInput(text: str): bool {
            return len(text) > 0
        }
    
    domain:
        count: int = 0
        name: str = "default"
        items: list = []
        config: dict = {}
}
```

### TypeScript Type Support

```typescript
@@target typescript

system TypedSystem {
    interface:
        process(data: string): object
        calculate(x: number, y: number): number
    
    operations:
        validateInput(text: string): boolean {
            return text.length > 0
        }
    
    domain:
        let count: number = 0
        let name: string = "default"
        let items: string[] = []
        let config: Record<string, any> = {}
}
```

## Native Language Integration

### Import Statements

Frame v4 uses native import syntax:

```python
@@target python

# Python imports
import json
import asyncio
from datetime import datetime
from typing import List, Dict, Optional

system DataProcessor {
    operations:
        parseJson(text: str) {
            # Use imported module
            return json.loads(text)
        }
        
        getCurrentTime() {
            return datetime.now()
        }
}
```

```typescript
@@target typescript

// TypeScript imports
import { readFile } from 'fs/promises'
import axios from 'axios'
import type { UserData } from './types'

system DataFetcher {
    operations:
        async fetchData(url: string): Promise<any> {
            const response = await axios.get(url)
            return response.data
        }
}
```

## Control Flow

### Conditional Statements

Use native language if/else syntax:

```python
@@target python

machine:
    $Processing {
        handle(data) {
            if data is None:
                print("No data")
                -> $Error()
            elif len(data) == 0:
                print("Empty data")
                -> $Idle()
            else:
                result = self.process(data)
                if result['status'] == 'success':
                    -> $Complete(result)
                else:
                    -> $Retry(data)
        }
    }
```

### Loops

Use native loop constructs:

```python
@@target python

actions:
    processItems(items: list) {
        results = []
        for item in items:
            if self.validate(item):
                processed = self.transform(item)
                results.append(processed)
            else:
                print(f"Invalid item: {item}")
        
        # While loop
        retries = 0
        while retries < 3 and not self.verified:
            self.verify()
            retries += 1
        
        return results
    }
```

### Exception Handling

Use native try/catch/finally:

```typescript
@@target typescript

actions:
    safeProcess(data: any): any {
        try {
            const result = this.riskyOperation(data)
            return result
        } catch (error) {
            console.error("Processing failed:", error)
            system.return = { error: error.message }
            -> $Error()
        } finally {
            this.cleanup()
        }
    }
```

## Collections and Data Structures

### Lists/Arrays

```python
@@target python

operations:
    listOperations() {
        # List creation
        numbers = [1, 2, 3, 4, 5]
        
        # List comprehension
        squares = [x**2 for x in numbers]
        
        # Filtering
        evens = [x for x in numbers if x % 2 == 0]
        
        # Slicing
        first_three = numbers[:3]
        last_two = numbers[-2:]
        
        return squares
    }
```

### Dictionaries/Objects

```python
@@target python

operations:
    dictOperations() {
        # Dictionary creation
        person = {
            "name": "Alice",
            "age": 30,
            "active": True
        }
        
        # Dictionary comprehension
        squared = {x: x**2 for x in range(5)}
        
        # Nested structures
        config = {
            "database": {
                "host": "localhost",
                "port": 5432
            },
            "cache": {
                "enabled": True,
                "ttl": 3600
            }
        }
        
        return config
    }
```

### Sets

```python
@@target python

operations:
    setOperations() {
        # Set creation
        unique = {1, 2, 3, 3, 2, 1}  # {1, 2, 3}
        
        # Set comprehension
        squares = {x**2 for x in range(10)}
        
        # Set operations
        a = {1, 2, 3}
        b = {3, 4, 5}
        union = a | b           # {1, 2, 3, 4, 5}
        intersection = a & b    # {3}
        difference = a - b      # {1, 2}
        
        return unique
    }
```

## String Operations

### String Formatting

```python
@@target python

operations:
    formatStrings(name: str, age: int) {
        # f-strings (Python)
        message = f"Hello, {name}! You are {age} years old."
        
        # Format method
        template = "User: {}, Status: {}".format(name, "active")
        
        # Percent formatting
        old_style = "Name: %s, Age: %d" % (name, age)
        
        return message
    }
```

```typescript
@@target typescript

operations:
    formatStrings(name: string, age: number): string {
        // Template literals (TypeScript)
        const message = `Hello, ${name}! You are ${age} years old.`
        
        // String concatenation
        const combined = "User: " + name + ", Age: " + age
        
        return message
    }
```

## Operators

### Arithmetic Operators

```python
@@target python

operations:
    arithmetic(a: int, b: int) {
        addition = a + b
        subtraction = a - b
        multiplication = a * b
        division = a / b          # Float division
        floor_division = a // b   # Integer division
        modulo = a % b
        power = a ** b
        
        # Compound assignments
        x = 10
        x += 5    # x = 15
        x *= 2    # x = 30
        x //= 3   # x = 10
        
        return x
    }
```

### Comparison and Logical Operators

```python
@@target python

operations:
    comparisons(a: int, b: int) {
        # Comparison
        equal = a == b
        not_equal = a != b
        less_than = a < b
        greater_than = a > b
        less_equal = a <= b
        greater_equal = a >= b
        
        # Logical
        and_result = (a > 0) and (b > 0)
        or_result = (a > 0) or (b > 0)
        not_result = not (a > 0)
        
        # Membership
        items = [1, 2, 3, 4, 5]
        is_member = a in items
        not_member = b not in items
        
        return equal
    }
```

### Bitwise Operators

```python
@@target python

operations:
    bitwise(a: int, b: int) {
        bit_and = a & b
        bit_or = a | b
        bit_xor = a ^ b
        bit_not = ~a
        left_shift = a << 2
        right_shift = a >> 2
        
        return bit_and
    }
```

## Advanced Features

### Lambda Functions

```python
@@target python

operations:
    useLambdas() {
        # Lambda functions
        square = lambda x: x ** 2
        add = lambda x, y: x + y
        
        # Using with higher-order functions
        numbers = [1, 2, 3, 4, 5]
        squared = list(map(lambda x: x**2, numbers))
        filtered = list(filter(lambda x: x > 2, numbers))
        
        return squared
    }
```

### Destructuring/Unpacking

```python
@@target python

operations:
    unpacking() {
        # Tuple unpacking
        point = (3, 4)
        x, y = point
        
        # Multiple assignment
        a, b, c = 1, 2, 3
        
        # Swapping
        x, y = y, x
        
        # Extended unpacking
        first, *middle, last = [1, 2, 3, 4, 5]
        # first = 1, middle = [2, 3, 4], last = 5
        
        return (x, y)
    }
```

### Walrus Operator (Python 3.8+)

```python
@@target python

operations:
    walrusOperator(items: list) {
        # Assignment expression
        if (n := len(items)) > 10:
            print(f"List has {n} items")
        
        # In while loops
        while (line := self.read_line()) != "":
            self.process(line)
        
        return n
    }
```

## Async/Await

```python
@@target python

system AsyncSystem {
    operations:
        async fetchData(url: str) {
            import aiohttp
            async with aiohttp.ClientSession() as session:
                async with session.get(url) as response:
                    return await response.json()
        }
    
    machine:
        $Ready {
            async process(url: str) {
                data = await self.fetchData(url)
                -> $Processing(data)
            }
        }
}
```

## Module-Level Code

Frame files can contain module-level functions:

```python
@@target python

# Module-level function
fn main() {
    # Entry point
    sys = MySystem()
    sys.start()
}

fn helper(x: int): int {
    return x * 2
}

system MySystem {
    interface:
        start()
    
    machine:
        $Init {
            start() {
                # Can call module-level functions
                value = helper(5)
                print(f"Helper returned: {value}")
            }
        }
}
```

## Attributes and Decorators

### Python Decorators

```python
@@target python

@dataclass
class Config:
    host: str
    port: int

system DecoratedSystem {
    operations:
        @staticmethod
        @lru_cache(maxsize=128)
        expensiveComputation(n: int) {
            # Cached static method
            return sum(range(n))
        }
}
```

### TypeScript Decorators

```typescript
@@target typescript

@injectable()
system ServiceSystem {
    operations:
        @log()
        @validate()
        processRequest(data: any): void {
            // Method with decorators
            this.handle(data)
        }
}
```

## Comments

Frame supports native language comment syntax:

```python
@@target python

system CommentExample {
    # Python single-line comment
    
    """
    Python multi-line
    docstring comment
    """
    
    machine:
        $State {
            handler() {
                # Inline comment
                value = 42  # End-of-line comment
            }
        }
}
```

```typescript
@@target typescript

system CommentExample {
    // TypeScript single-line comment
    
    /*
     * TypeScript multi-line
     * block comment
     */
    
    machine:
        $State {
            handler() {
                // Inline comment
                const value = 42  // End-of-line comment
            }
        }
}
```

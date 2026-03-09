# CodeBuilder Migration Guide

## Overview

The CodeBuilder architecture (introduced in v0.75) provides a robust, automatic solution for code generation with perfect source mappings. This guide explains how to migrate from the fragile manual line tracking to the new architecture.

## The Problem with Manual Tracking

The current PythonVisitor uses manual line tracking which is fragile and error-prone:

```rust
// Old approach - fragile and buggy
fn generate_code(&mut self) {
    self.newline();                    // Increments current_line
    self.add_code("def foo():");       // Doesn't track newlines!
    self.add_source_mapping(10);       // Hopes current_line is correct
    
    // Problems:
    // 1. add_code() doesn't count newlines in the string
    // 2. Mapping happens at wrong time
    // 3. Need manual +1/-1 offsets to "fix" mappings
    // 4. Any change breaks all mappings
}
```

## The CodeBuilder Solution

CodeBuilder tracks every character automatically:

```rust
// New approach - automatic and robust
fn generate_code(&mut self) {
    self.builder
        .newline()                           // Tracked automatically
        .map_next(10)                        // Deferred mapping
        .write("def foo():");                // Every char tracked
    
    // Benefits:
    // 1. Every character position tracked
    // 2. Newlines in strings detected
    // 3. Mappings always correct
    // 4. No manual offsets needed
}
```

## Migration Steps

### Step 1: Replace Core Generation Methods

Replace the visitor's string building with CodeBuilder:

```rust
// Before
pub struct PythonVisitor {
    code: String,
    current_line: usize,
    // ...
}

// After
pub struct PythonVisitorV2 {
    builder: CodeBuilder,
    // ...
}
```

### Step 2: Convert Code Generation

```rust
// Before - Manual tracking
self.newline();
self.add_code(&format!("def {}():", name));
self.add_source_mapping(line);
self.indent();

// After - Automatic tracking
self.builder.write_function(name, "", false, line);
```

### Step 3: Handle Complex Generation

```rust
// Before - Error prone
self.add_code("class ");
self.add_code(&name);
self.add_code(":");
self.newline();  // Did we count all newlines?

// After - Bulletproof
self.builder
    .map_next(line)
    .write(&format!("class {}:", name))
    .newline();
```

## Key CodeBuilder Features

### 1. Automatic Line Tracking

```rust
builder.write("line 1\nline 2\nline 3");
// Automatically at line 3, column 6
```

### 2. Deferred Mapping

```rust
builder
    .map_next(frame_line)    // Set mapping for next write
    .write("code");           // Mapping applied here
```

### 3. Helper Methods

```rust
// Functions
builder.write_function("name", "params", is_async, frame_line);
builder.end_function();

// Classes
builder.write_class("Name", Some("Base"), Some(frame_line));
builder.end_class();

// Blocks with auto indent/dedent
builder.write_block(|b| {
    b.writeln("indented code");
});
```

### 4. Composition Support

```rust
// Generate in child builder
let child = builder.child();
child.writeln("some code");

// Merge back with adjusted mappings
builder.merge(child, Some(frame_line));
```

## Testing the Migration

The CodeBuilder includes comprehensive unit tests:

```bash
cargo test code_builder
```

Example test showing perfect mapping:

```rust
#[test]
fn test_mapping_accuracy() {
    let mut builder = CodeBuilder::new("    ");
    
    builder.writeln_mapped("def foo():", 10);
    builder.indent();
    builder.writeln_mapped("return 42", 11);
    
    let (code, mappings) = builder.build();
    
    assert_eq!(mappings[0].frame_line, 10);
    assert_eq!(mappings[0].python_line, 1);  // Exactly right!
    assert_eq!(mappings[1].frame_line, 11);
    assert_eq!(mappings[1].python_line, 2);  // No offset needed!
}
```

## Benefits Summary

1. **Correctness**: Mappings are mathematically guaranteed to be correct
2. **Robustness**: Can't get out of sync - every character is tracked
3. **Simplicity**: No manual offset calculations
4. **Composability**: Support for non-linear code generation
5. **Debugging**: Know exactly where every piece of code came from
6. **Maintenance**: Changes don't break mappings

## Next Steps

1. Complete PythonVisitorV2 implementation using CodeBuilder
2. Run side-by-side tests comparing old vs new mappings
3. Deprecate manual tracking in old visitor
4. Switch to V2 as default
5. Remove old visitor code

## Example: Complete Function Generation

```rust
impl PythonVisitorV2 {
    fn generate_event_handler(&mut self, handler: &EventHandler) {
        // Perfect mapping with no manual offsets!
        self.builder
            .newline()  // Visual spacing
            .write_function(
                &handler.name,
                "self, event",
                handler.is_async,
                handler.line  // Maps to exact function def line
            )
            .writeln_mapped("# Handler body", handler.body_line)
            .writeln("return")
            .end_function();
    }
}
```

The future is automatic, robust, and maintenance-free source mapping!
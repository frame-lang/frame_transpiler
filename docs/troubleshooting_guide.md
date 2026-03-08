# Frame Transpiler Troubleshooting Guide

## Common Issues and Solutions

### 1. Variable Scoping Issues

#### Problem: "KeyError: 'variable_name'" at runtime
**Cause**: Variable incorrectly categorized as state variable instead of local/parameter
**Solution**:
1. Check `visit_variable_node_to_string()` in python_visitor_v2.rs
2. Verify variable is tracked in `current_handler_locals` or `current_handler_params`
3. Check scope with `var_node.id_node.scope`

#### Problem: Module variables initialized as `None`
**Cause**: Using wrong value accessor or visitor method not in trait impl
**Solution**:
1. Use `get_initializer_value_rc()` not `value_rc`
2. Ensure visitor method is in `impl AstVisitor` block
3. Generate module variables after systems/classes

### 2. Visitor Trait Issues

#### Problem: Visitor method not being called
**Symptoms**: No code generated for certain AST nodes
**Solution**:
1. Check if method is inside `impl AstVisitor for PythonVisitorV2` block
2. Verify method signature matches trait definition in `visitors/mod.rs`
3. Look for duplicate method definitions outside trait impl

#### Example Structure:
```rust
impl AstVisitor for PythonVisitorV2 {
    // These methods get called by AST nodes
    fn visit_function_node(&mut self, node: &FunctionNode) { ... }
    fn visit_variable_decl_node(&mut self, node: &VariableDeclNode) { ... }
}

impl PythonVisitorV2 {
    // Helper methods go here
    fn helper_method(&mut self) { ... }
}
```

### 3. State Parameter Access

#### Problem: "NameError: name 'data' is not defined" in enter handlers
**Cause**: State parameters not accessible in scope
**Areas to Check**:
- State parameter passing in transitions
- Enter event argument handling
- Compartment state_args access

### 4. Expression Type Handling

#### Problem: "TODO: expr type" in generated code
**Cause**: Missing handler in `visit_expr_node_to_string()`
**Solution**:
Add case for the missing expression type:
```rust
ExprType::DefaultLiteralValueForTypeExprT => {
    // Handle default literal values
}
```

### 5. Build and Test Issues

#### Problem: Cargo build shows "unexpected argument '2'"
**Cause**: Shell redirection issues with complex commands
**Solution**: Use simpler commands without complex pipes:
```bash
cargo build --release
# Instead of: cargo build --release 2>&1 | grep error
```

#### Problem: Test runner fails with argument errors
**Cause**: Incorrect use of stderr redirection with test runner
**Solution**: Run tests without complex redirections:
```bash
python3 runner/frame_test_runner.py --all --framec ../target/release/framec
```

## Debugging Techniques

### 1. Enable Debug Output
```bash
FRAME_TRANSPILER_DEBUG=1 ./target/release/framec -l python_3 test.frm
```

### 2. Check AST Structure
```bash
FRAME_AST_OUTPUT=/tmp/ast.json ./target/release/framec -l python_3 test.frm
```

### 3. Compare with V1 Visitor
```bash
# Generate with V1
USE_PYTHON_V1=1 ./target/release/framec -l python_3 test.frm > v1.py

# Generate with V2 (default)
./target/release/framec -l python_3 test.frm > v2.py

# Compare outputs
diff v1.py v2.py
```

### 4. Add Debug Prints
```rust
eprintln!("DEBUG: Variable '{}' scope: {:?}", var_name, var_node.id_node.scope);
```
Note: Debug output goes to stderr, not stdout

## Key Files to Understand

### Core Visitor Implementation
- `framec/src/frame_c/visitors/python_visitor_v2.rs` - Main V2 visitor
- `framec/src/frame_c/visitors/python_visitor.rs` - V1 visitor (reference)
- `framec/src/frame_c/visitors/mod.rs` - Visitor trait definition

### AST and Parser
- `framec/src/frame_c/ast.rs` - AST node definitions
- `framec/src/frame_c/parser.rs` - Two-pass parser
- `framec/src/frame_c/scanner.rs` - Lexical scanner

### Code Generation
- `framec/src/frame_c/code_builder.rs` - Line-aware code builder
- `framec/src/frame_c/source_map.rs` - Source mapping

## Variable Scoping in Frame

### Scope Types (IdentifierDeclScope)
1. **ModuleScope** - Module-level variables (global)
2. **DomainBlockScope** - Domain block variables (instance)
3. **StateVarScope** - State variables
4. **EventHandlerVarScope** - Local to event handler
5. **LoopVarScope** - Loop iteration variables
6. **BlockVarScope** - Block-scoped variables

### Variable Tracking in V2
- `global_vars`: HashSet of module-level variables
- `current_handler_params`: Parameters of current handler
- `current_handler_locals`: Local variables in current handler
- `domain_vars`: Domain block variables

## Test Failure Analysis

### Running Specific Failed Tests
```bash
# Generate Python code for failed test
../target/release/framec -l python_3 test_comprehensive_v0_20_features.frm > test.py

# Run to see error
python3 test.py

# Check specific line mentioned in error
sed -n '98p' test.py
```

### Common Runtime Errors
1. **NameError**: Variable not in scope
2. **KeyError**: Accessing non-existent compartment state variable
3. **TypeError**: Async/await mismatch
4. **AttributeError**: Method/property doesn't exist

## Progress Tracking

### Current Status (v0.76.1)
- Test Success: 89.2% (338/379)
- Recent Fixes: Local variable scoping, module variable init
- Main Issues: State parameters, async/await, class decorators

### How to Check Progress
```bash
# Run all tests and get summary
python3 runner/frame_test_runner.py --all --framec ../target/release/framec | tail -10

# Get detailed JSON report
python3 runner/frame_test_runner.py --all --json --framec ../target/release/framec
# Results in: reports/test_results_v0.31.json
```

## Quick Reference

### Build Commands
```bash
cd /Users/marktruluck/projects/frame_transpiler
cargo build --release
```

### Test Commands
```bash
cd framec_tests
# All tests
python3 runner/frame_test_runner.py --all --framec ../target/release/framec

# With details
python3 runner/frame_test_runner.py --all --json --matrix --verbose --framec ../target/release/framec
```

### Debug a Specific Test
```bash
# Generate
../target/release/framec -l python_3 path/to/test.frm > test.py

# Run
python3 test.py

# Debug with environment variable
FRAME_TRANSPILER_DEBUG=1 ../target/release/framec -l python_3 test.frm
```
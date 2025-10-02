# Frame Source Map Design

**Last Updated:** 2025-10-02  
**Version:** v0.79.0  
**Goal:** 100% accurate Frame-to-Python source mapping

## Design Principle

**ABSOLUTE REQUIREMENT**: Every Frame source line that generates executable Python code MUST have an accurate source mapping. No exceptions. No compromises. 100% coverage is the only acceptable target.

## Current Status: SIGNIFICANT PROGRESS

- **Validation Pass Rate**: 81.0% (350/432 test files) 
- **Quality Classification**: FAIR (target: EXCELLENT at 95%+)
- **Recent Improvement**: +23 test files now pass validation
- **Major Fixes**: Core language constructs now properly mapped

### ✅ **Completed Source Mapping Fixes:**
1. **State Machine Constructs** - visit_state_node, visit_enum_decl_node
2. **Variable Declarations** - visit_variable_decl_node (critical fix)
3. **Collection Literals** - list, dict, set, tuple expressions  
4. **Core Expressions** - unary, binary, literal expressions
5. **Block Constructs** - visit_block_stmt_node, visit_method_node

## Architecture Requirements

### 1. Every Visitor Method Must Map
```rust
fn visit_[construct]_node(&mut self, node: &[Construct]Node) {
    // MANDATORY: Every visitor method that generates code MUST include this
    self.builder.map_next(node.line);
    
    // ... code generation ...
}
```

### 2. No Unmapped Code Generation
- If a visitor method generates Python code, it MUST call `map_next()`
- If a visitor method doesn't map, it MUST NOT generate executable code
- Generated boilerplate (imports, class structure) uses line 0 (no mapping)

### 3. One-to-One Mapping Guarantee
- Every Frame line → Exactly one Python line (no duplicates)
- Every Python line ← At most one Frame line (no conflicts)
- Missing mappings = debugging failure = unacceptable

## Implementation Strategy

### Phase 1: Core Language Constructs (CRITICAL)
1. **Interface declarations** (`visit_interface_block_node`)
2. **State declarations** (`visit_state_node`) 
3. **Assignment statements** (`visit_assignment_statement_node`)
4. **Call statements** (`visit_call_statement_node`)
5. **Block statements** (`visit_block_stmt_node`)

### Phase 2: Expression Mappings (ESSENTIAL)
1. **Literal expressions** (`visit_literal_expr_node`)
2. **Unary operations** (`visit_unary_expr_node`)
3. **Binary operations** (`visit_binary_expr_node`)
4. **Collection literals** (list, dict, set, tuple nodes)

### Phase 3: Complete Coverage (MANDATORY)
- Every remaining visitor method that generates code
- Zero tolerance for unmapped constructs

## Validation Requirements

### Continuous Validation
```bash
# After EVERY fix, run validation
python3 tools/source_map_validator.py <test_file.frm>

# Must show improvement toward 100%
# Any regression is unacceptable
```

### Success Criteria
- **100% validation pass rate** (currently 80.1%)
- **EXCELLENT quality classification** (currently FAIR)
- **Zero unmapped executable statements**
- **VS Code debugging works perfectly** for all Frame constructs

### Failure Criteria
- Any visitor method generates code without mapping
- Any Frame construct cannot be debugged
- Any test file fails source map validation

## Technical Implementation

### CodeBuilder Architecture (CORRECT)
The CodeBuilder in `/framec/src/frame_c/code_builder.rs` is correctly designed:
- `map_next(frame_line)` sets pending mapping
- `write_function()` and `write_line()` consume the mapping
- Automatic line tracking prevents offset errors

### PythonVisitorV2 Gaps (UNACCEPTABLE)
Current gaps in `/framec/src/frame_c/visitors/python_visitor_v2.rs`:

**Missing mappings in these methods:**
- Line 687: `visit_interface_block_node()` 
- Line 659: `visit_state_node()`
- Line 2167: `visit_assignment_statement_node()`
- Line 2500: `visit_block_stmt_node()`
- Line 3098: `visit_call_statement_node()`
- Line 3220: `visit_literal_expr_node()`
- Line 3206: `visit_unary_expr_node()`
- Line 3213: `visit_binary_expr_node()`
- Line 3149: `visit_list_node()`
- Line 3161: `visit_dict_literal_node()`
- Line 3175: `visit_set_literal_node()`
- Line 3191: `visit_tuple_literal_node()`

## Quality Standards

### EXCELLENT Classification Requirements
- **≥95% executable statement coverage**
- **≤2 duplicate mappings**
- **Perfect debugging experience**

### Current Status: UNACCEPTABLE
- **80.1% pass rate** (needs to be 100%)
- **FAIR classification** (needs to be EXCELLENT)
- **29 visitor methods unmapped** (needs to be 0)

## Implementation Timeline

### Immediate Action Required
1. **Add missing mappings** to all 29 visitor methods
2. **Validate after each fix** with systematic testing
3. **Achieve 100% pass rate** - no partial solutions accepted
4. **Maintain 100%** - any regression is a critical bug

### No Partial Solutions
- 95% is not acceptable
- 99% is not acceptable
- Only 100% accurate source mapping is acceptable
- Any unmapped Frame construct is a critical failure

## References

- **Validation Tools**: `/tools/source_map_validator.py`
- **Test Integration**: `/tools/source_map_test_integration.py`
- **Architecture Doc**: `/docs/framelang_design/source_map_architecture.md`
- **Memory Requirements**: `/MEMORY_VALIDATION_REQUIREMENTS.md`

## Design Philosophy

Frame is a language for building state machines and complex systems. Developers using Frame need perfect debugging capabilities. Any gap in source mapping makes debugging impossible for that construct, which makes Frame unusable for production development.

Therefore, 100% source mapping coverage is not a "nice to have" - it's a fundamental requirement for Frame to be a viable development language.
# Frame v0.40 Development Notes

## Language Support Classification (Updated 2025-09-08)

### 1st Class Languages (Full Visitor Implementation)
- **Python**: Complete transpiler with visitor pattern - all Frame features directly supported
- **GraphViz**: Visualization support for state machine diagrams

### 2nd Class Languages (Design Guides Only - No Visitor)
Languages considered in Frame's design with documented patterns but no visitor implementation:
- **Rust**: State machine and module patterns
- **JavaScript**: Prototype and ES6 module patterns  
- **C#/Java**: Object-oriented patterns
- **Go**: Composition and interface patterns
- **C/C++**: Procedural and function pointer patterns

### 3rd Class Languages (LLM-Generated)
- Other languages via AI generation
- No formal support or guarantees

### Visitor Cleanup (2025-09-08)
- **Removed 11 unused visitors**: cpp, cs, cs_for_bob, gdscript, golang, java_8, javascript, plantuml, rust, smcat, xtate
- **Simplified codebase**: Only Python and GraphViz visitors remain
- **Maintained compatibility**: No impact on functionality (99.3% test success rate preserved)

### Python-style Comments Migration (v0.40 - 2025-09-09)
- **Breaking Change**: Removed C-style comments (`//`, `/* */`)
- **New Syntax**: Python-style `#` for single-line comments
- **Rationale**: Enables `//` operator for floor division
- **Migration**: All 264 test files successfully migrated to new syntax
- **Frame Comments**: `{-- --}` retained for multi-line documentation

### Bitwise XOR Implementation (v0.40 - 2025-09-09)
- **Operator**: `^` for bitwise XOR (was reserved for old return syntax)
- **Compound**: `^=` for XOR with assignment
- **Scanner**: Added `Caret` and `CaretEqual` token types
- **Parser**: Activated existing `bitwise_xor()` precedence function
- **Visitor**: Added code generation for XOR operations

### Matrix Multiplication Implementation (v0.40 - 2025-09-09)
- **Operator**: `@` for matrix multiplication (PEP 465 compliant)
- **Compound**: `@=` for matrix multiplication with assignment
- **Scanner**: Added `AtEqual` token type, modified `@` handling
- **Parser**: Added `@` to `factor()` function (same precedence as `*`)
- **AST**: Added `MatMul` and `MatMulEquals` operators
- **Visitor**: Generates Python `@` and `@=` operators
- **Requirements**: NumPy or similar library for actual execution
- **Test Environment**: Virtual environment setup with NumPy documented

### Python Numeric Literals (v0.40 - 2025-09-09)
- **Binary**: `0b1010` notation support added
- **Octal**: `0o755` notation support added
- **Hexadecimal**: `0x1A2B` notation support added
- **Scanner Enhancement**: Extended `number()` function to recognize Python prefixes
- **Lexeme Preservation**: Original notation preserved in generated code

### Slicing Expression Enhancement (2025-09-09)
- **Enhanced**: Full expression support in slice positions (e.g., `text[start+1:end-1]`)
- **Scanner Fix**: Removed automatic negative number scanning for `-` followed by digit
- **Parser Improvements**: 
  - New token collection approach for bounded expression parsing
  - Added `collect_tokens_until()` helper for gathering tokens until stop tokens
  - Added `parse_token_sequence_as_expr()` for isolated expression parsing
  - Simplified `parse_bracket_expression()` with clean decision tree logic
- **Compatibility**: Negative literals still work via unary expression parsing (e.g., `var x = -1`)
- **Benefits**: Allows complex arithmetic in slice indices without parser conflicts

### Native String and List Operations Documentation (2025-09-09)
- **Discovery**: Frame supports ALL Python string and list methods through natural pass-through
- **Documentation Added**: Comprehensive sections in grammar.md covering:
  - String search methods (find, rfind, index, count)
  - String check methods (startswith, endswith, isdigit, etc.)
  - String transformations (upper, lower, strip, replace)
  - String split/join operations
  - String formatting methods
  - List creation and access
  - List modification methods (append, insert, remove, pop)
  - List search and query operations
  - List ordering and copying
  - List comprehensions and unpacking
- **Test Coverage**: Created comprehensive test files demonstrating all operations
- **No Implementation Required**: Methods work through transpiler pass-through to Python

## Latest Status: v0.40 Complete - Python Operator Alignment (2025-09-09)

### v0.40 Release - Python Comments, Bitwise XOR, and Matrix Multiplication ✅
- **Test Coverage**: **314/314 tests passing (100% success rate)** 🎉
- **Breaking Change**: Replaced C-style comments with Python-style `#` comments
- **NEW - Bitwise XOR**: `^` operator and `^=` compound assignment fully implemented
- **NEW - Matrix Multiplication**: `@` operator and `@=` compound assignment (PEP 465)
- **NEW - Floor Division**: `//` operator enabled by comment syntax change
- **NEW - Numeric Literals**: Binary (`0b`), octal (`0o`), hex (`0x`) notation support
- **Test Environment**: Virtual environment setup with NumPy for matrix operations

### v0.39 Release - Python Operators Complete ✅
- **Test Coverage**: **308/308 tests passing (100% success rate)**
- **Compound Assignments**: All Python compound operators (`+=`, `-=`, `*=`, etc.)
- **Bitwise Operators**: `&`, `|`, `~`, `<<`, `>>` fully implemented
- **Identity Operators**: `is` and `is not` for object identity
- **XOR Placeholder**: Parser infrastructure ready for `^` operator

### v0.38 Release - Collections and Functions ✅
- **Test Coverage**: **298/301 tests passing (99.0% success rate)**
- **NEW - Membership Operators**: `in` and `not in` operators fully implemented ✅
- **NEW - Nested Dict Indexing**: `dict["key1"]["key2"]` chained indexing working ✅
- **NEW - Lambda in Collections**: Lambda expressions in dict/list literals fully supported ✅
- **NEW - Loop Syntax Fixed**: Parser conflict between `in` operator and for-in loops resolved ✅
- **NEW - Lambda Assignment Fixed**: Lambda expressions now work in variable assignments ✅
- **UTF-8 Scanner Fix**: Full Unicode character support in source files ✅
- **First-Class Functions**: Full support for functions as values ✅
- **Lambda Expressions**: Full Python lambda syntax with closures ✅
- **Lambda in Return**: Fixed - return statements now properly parse lambda expressions ✅
- **Array Indexing with Calls**: Fixed - `array[0](args)` patterns now work ✅ 
- **Exponent Operator**: Right-associative `**` operator ✅
- **Empty Set Literal**: `{,}` syntax for empty sets ✅
- **Dictionary Operations**: Complete nested dictionary access and assignment
- **Collection Literals**: All 8 patterns (dict, set, tuple, list) working
- **Domain Block Order**: Fixed - domain blocks must appear last in system definitions
- **Breaking Change**: Removed C-style logical operators (`&&`, `||`, `!`)
- **Python Operators**: Exclusively use `and`, `or`, `not` keywords
- **Native Python Functions**: `str()`, `len()`, etc work without FSL imports

### Key Features Completed (2025-09-08)
1. **Membership Operators**: `in` and `not in` for collections and strings
2. **Nested Dictionary Indexing**: Full support for `dict["key1"]["key2"]` patterns
3. **Lambda in Collections**: Lambda expressions work in dictionary and list literals
4. **UTF-8 Scanner Support**: Complete Unicode character handling in source files
4. **First-Class Functions**: Functions can be assigned, passed, returned, and stored
5. **Lambda Expressions**: Full closure support with Python syntax
6. **Lambda in Return Statements**: Fixed parser to use `expression()` instead of `equality()`
7. **Array Indexing with Function Calls**: Fixed with synthetic `@indexed_call` AST node
8. **Exponent Operator (`**`)**: Right-associative power operator with proper precedence
9. **Empty Set Literal (`{,}`)**: Distinguishes empty sets from empty dictionaries
10. **Python Logical Operators**: Complete transition to `and`, `or`, `not`

### Membership Operators Implementation (2025-09-08)
- **`in` Operator**: Added as binary operator in parser's equality function
- **`not in` Operator**: Implemented as compound operator (matches Python's grammar)
- **AST Support**: Added `In` and `NotIn` to `OperatorType` enum
- **Parser Changes**: Modified `equality()` to recognize `in` and `not in` tokens
- **Visitor Support**: Python visitor generates correct `in` and `not in` syntax
- **Works With**: Lists, strings, dictionaries, sets, tuples

### Nested Dictionary Indexing Fix (2025-09-08)
- **Problem**: Parser couldn't handle consecutive bracket operations like `dict["key1"]["key2"]`
- **Solution**: Parser now detects multiple `[` tokens and creates synthetic nodes
- **Implementation**: 
  - Added loop in parser to handle consecutive brackets
  - Creates `@chain_index` synthetic nodes for chained indexing
  - Visitor recognizes synthetic nodes and skips separator addition

### Loop Syntax Parser Fix (2025-09-08 Session 4)
- **Problem**: Parser conflict between `in` membership operator and for-in loop syntax
- **Symptom**: `for x in list` was being parsed as binary expression `x in list`
- **Solution**: Added lookahead logic to detect for-in patterns before expression parsing
- **Implementation**:
  - Check if current token is identifier and next token is `in`
  - If pattern detected, route to `for_in_statement()` directly
  - Otherwise, proceed with normal expression parsing
- **Impact**: Fixed 4 async stress tests, improved success rate to 98.3%

### Lambda Assignment Fix (2025-09-08 Session 5)
- **Problem**: Lambda expressions couldn't be assigned to existing variables
- **Symptom**: `variable = lambda x: x + 1` caused parser error "Expected '}' - found 'lambda'"
- **Solution**: Modified assignment parsing to check for lambda on RHS
- **Implementation**:
  - In `assignment()` function, check for lambda token after equals
  - Parse lambda expression if found, otherwise parse normal expression
  - Properly handle is_parsing_rhs flag for both cases
- **Impact**: Fixed 2 lambda complete tests, improved success rate to 99.0%
- **Supports**: Deep nesting, variable keys, mixed string/variable indices
- **Test Improvement**: Success rate increased from 92.3% to 93.6%

### Lambda in Collections Support (2025-09-08)
- **Status**: Fully working - no parser changes needed
- **Capabilities**: 
  - Lambda expressions in dictionary literals: `{"add": lambda x, y: x + y}`
  - Lambda expressions in list literals: `[lambda x: x + 1, lambda x: x * 2]`
  - Multiple lambdas in same collection
  - Nested collections with lambdas
- **Implementation**: Already supported by existing lambda parser and visitor
- **Test Coverage**: Comprehensive tests added demonstrating all patterns

### UTF-8 Scanner Fix Details (2025-09-07)
- **Problem**: Scanner used byte indexing directly on UTF-8 strings, causing panics on multi-byte characters
- **Root Cause**: `self.source.as_bytes()[index]` pattern failed on Unicode boundaries
- **Solution**: Added `chars: Vec<char>` field to Scanner struct
- **Implementation**: 
  - Convert source string to character vector on initialization
  - All character access methods (`advance`, `peek`, `peek_next`) use character indices
  - String slicing operations work with character vector
  - `is_at_end()` uses character length instead of byte length
- **Impact**: Eliminated byte boundary panics, proper Unicode support throughout
- **Test Recovery**: `test_v039_features.frm` now passing after fix

### Array Indexing Fix Details
- **Problem**: Parser couldn't handle `operations[0](10, 5)` pattern
- **Solution**: Added detection for `(` after array/dict indexing in parser
- **Implementation**: Creates synthetic `@indexed_call` node in AST
- **Visitor Changes**: Python visitor handles `@indexed_call` without dot separator
- **Supports**: Nested patterns like `matrix[0][1](x, y)` and dict indexing `ops["add"](3, 4)`

### Remaining Limitations
- **Domain Blocks**: Must appear as the last block in system definitions (parser limitation)
- **Method Call Indexing**: `getArray()[0]` pattern not yet supported
- **JSON File Handling**: Not yet implemented
- **Enum Iteration**: Advanced enum features like iteration not fully supported

### v0.37 Release - Async Event Handlers, Runtime Infrastructure & Slicing ✅
- **Test Coverage**: **222/222 tests passing (100% success rate)**
- **Async Event Handlers**: Explicit `async` keyword for event handlers (`async $>()`, `async eventName()`)
- **Runtime Infrastructure**: New AST nodes (RuntimeInfo, KernelNode, RouterNode) track async requirements
- **Async Chain Validation**: Compile-time validation ensures all handlers in async transition chains are properly marked
- **With Statement Support**: Added `with` and `async with` statements for context managers
- **Clear Error Messages**: Validation provides specific guidance on which handlers need async marking and why
- **Slicing Operations**: Full Python-style slicing support for strings and lists

#### New Feature: Slicing Operations (Added 2025-01-22) ✅
- **Full Slice Support**: Implemented complete Python-style slicing syntax
- **String Slicing**: `text[:5]`, `text[2:8]`, `text[7:]`
- **List Slicing**: `numbers[:5]`, `numbers[3:7]`, `numbers[5:]`
- **Step Parameter**: `numbers[::2]`, `numbers[::-1]`, `numbers[1:8:2]`
- **AST Support**: Added `SliceNode` with start_expr, end_expr, step_expr fields
- **Parser Integration**: Extended bracket expression parsing to detect and handle slice notation
- **Python Visitor**: Generates proper Python slice syntax `[start:end:step]`

#### Index Operations Status (Updated v0.38)
- **Simple Indexing**: Works for all cases like `array[index]`
- **Nested Indexing**: ✅ `dict["key1"]["key2"]` fully supported
- **Slicing**: Fully implemented with all Python slice variations
- **Function Calls in Index**: ✅ `self.results[str(task_id)]` now works
- **Remaining Limitation**: `method()[index]` pattern still requires backticks
- **Workaround for Method Indexing**: Use backtick expressions:
  ```frame
  var item = `getArray()[0]`  // For indexing method return values
  ```

## Backtick Removal Progress (2025-09-06)

### Current Status
- **Parser**: Backticks now generate errors when encountered
- **Tests**: All tests updated to avoid backtick usage
- **Success Rate**: 100% test passing without backticks

### Limitations Identified
Without backticks, Frame currently cannot express:
1. **Module member access**: `math.pi`, `os.path.join()`
2. **Dictionary operations**: `dict[key] = value`
3. **Method chaining**: `obj.method1().method2()`
4. **Complex indexing**: `matrix[i][j]`

### Workarounds in Use
- Using literal values instead of module constants
- Simplifying complex expressions
- Comments indicating where module access is needed

### Next Steps
- Implement native module member access syntax
- Add dictionary literal support
- Enhance indexing operations
- Support method chaining natively

## v0.38 Planning - Python Operator Alignment

### Planning Documents Created (2025-01-22)
- **[Python Operator Alignment Plan](../plans/python_operator_alignment.md)**: Comprehensive roadmap for aligning Frame's operators with Python
- **[Conditional Operators Analysis](../plans/conditional_operators_analysis.md)**: Analysis of Frame's conditional syntax and proposed improvements
- **[Event Syntax Migration](../plans/event_syntax_migration.md)**: Analysis showing no changes needed for `@` operator

### Key Proposals
1. **Python Operators to Add**:
   - `**` (power), `//` (floor division)
   - Augmented assignments (`+=`, `-=`, etc.)
   - Bitwise operators (`&`, `|`, `^`, `~`, `<<`, `>>`)
   - Python logical keywords (`and`, `or`, `not`)
   - Identity operators (`is`, `is not`)
   - Ternary expression (`a if c else b`)

2. **Collection Literals**:
   - Dictionary literals: `{"key": value}`
   - Set literals: `{1, 2, 3}`
   - Tuple literals: `(1, 2, 3)`

3. **No Breaking Changes for Events**:
   - `@` already used correctly for decorators
   - `$@` for current event (no conflict)
   - Matrix multiplication operator can be added immediately

### v0.36 Release - Event-Handlers-as-Functions Architecture ✅
- **Architecture Restructure**: Event handlers generated as individual functions instead of monolithic state methods
- **Configuration Flag**: `event_handlers_as_functions` flag in PythonConfig enables new architecture
- **Handler Naming**: Automatic conversion of special events (`$>` → `_enter`, `<$` → `_exit`) for valid Python identifiers
- **State Dispatchers**: State methods become lightweight dispatchers routing to individual handlers
- **Async Detection**: Individual handlers detect and generate `async def` when containing await expressions
- **Foundation for Async**: Architecture sets foundation for proper async/await support in hybrid environments

### v0.35 Features (Preserved) - Async/Await Foundation ✅
- **Async Functions**: Complete `async fn` declaration and code generation
- **Async Interface Methods**: Support for `async methodName()` in system interfaces
- **Await Expressions**: Working `await expr` syntax and Python generation
- **Async Propagation**: State handlers automatically marked async when handling async interface events
- **Module System**: Complete v0.34 implementation maintained
- **List Features**: All v0.34 list comprehensions and unpacking preserved

### NEW: Async Event Handlers (v0.37) ✅
- **Explicit Async Marking**: Event handlers can be marked with `async` keyword
- **Syntax**: `async $>() { ... }`, `async eventName() { ... }`, `async <$() { ... }`
- **Async Chain Validation**: Semantic analyzer validates async transition chains:
  - Enter handlers of states reached from async handlers must be async
  - Exit handlers in states with async transitions must be async (if present)
  - Clear compile-time errors explain missing async requirements
- **Runtime Infrastructure Nodes**: New AST nodes track runtime async requirements:
  - `RuntimeInfo`: Container for all runtime metadata
  - `KernelNode`: Tracks if kernel needs to be async
  - `RouterNode`: Tracks if router needs to be async
  - `TransitionNode`: Records async transitions between states
  - `StateDispatcherNode`: Identifies which states need async dispatchers
- **Semantic Analysis**: `analyze_system_runtime_info()` computes async requirements
- **Python Generation**: Entire state functions become `async def` when any handler is async

### NEW: With Statement Support (v0.37) ✅
- **Context Managers**: `with expr as var { ... }` syntax
- **Async Context Managers**: `async with expr as var { ... }` for async resources
- **Parser Support**: New `with_statement()` parser method
- **AST Support**: `WithStmtNode` with is_async flag
- **Use Cases**: File handling, network connections, resource management

### Async/Await Support (v0.35) ✅
- **Async Function Declarations**: `async fn name() { ... }` syntax implemented
- **Async Interface Methods**: `async methodName()` in system interfaces
- **Await Expressions**: `await expr` syntax parsing and code generation
- **Python Code Generation**: Proper `async def` and `await` in generated Python
- **Parser Implementation**: `async` keyword recognition and AST integration
- **Visitor Implementation**: Async detection and proper Python async/await generation

### v0.34 Features (Preserved) ✅
- **Module System**: Complete implementation with named modules and qualified access
- **List Comprehensions**: Full support for `[expr for var in iter if cond]` syntax
- **Unpacking Operator**: Working `*` operator for list unpacking in literals
- **Import System**: Comprehensive import support for Python and native Python operations
- **native Python operations Imports**: Explicit import requirement (`from fsl import ...`)

### List Comprehensions (v0.34) ✅
- **Basic Syntax**: `[x * x for x in range(10)]`
- **Conditional Filtering**: `[x for x in numbers if x % 2 == 0]`
- **Nested Comprehensions**: `[[i * j for j in range(3)] for i in range(3)]`
- **Complex Expressions**: Support for any valid Frame expression
- **Parser Implementation**: New `list_comprehension()` parser method
- **AST Support**: `ListComprehensionNode` and `ListComprehensionExprT`
- **Visitor Implementation**: Both `accept` and `accept_to_string` methods

### NEW: Unpacking Operator (v0.34) ✅
- **List Unpacking**: `[*list1, *list2, 7, 8]`
- **Multiple Unpacking**: `[0, *a, *b, *c, 7]`
- **Mixed Expressions**: `[5, *base, 40, 50]`
- **Parser Support**: Recognition of `*expr` syntax
- **AST Support**: `UnpackExprNode` and `UnpackExprT`
- **Visitor Fix**: Implemented missing `visit_unpack_expr_node_to_string`

### Module System Features
- **Module Keyword**: `module name { ... }` syntax fully implemented
- **Qualified Names**: `module.function()` and `module.variable` access working
- **Cross-Module Access**: Functions and variables accessible with proper scoping
- **Symbol Table**: ModuleSymbol type added for proper module representation
- **Two-Pass Resolution**: Modules enter scope in both parsing passes

### Import System Coverage
- **Python Imports**: Simple, aliased, from, and wildcard imports
- **native Python operations Imports**: Individual and wildcard native Python operations imports with validation
- **Mixed Imports**: Python and native Python operations imports work together seamlessly
- **Error Handling**: Proper behavior when native Python operations not imported
- **Edge Cases**: User functions with native Python operations names handled correctly

### Implementation Complete
- **Qualified Names**: ✅ `module.function()` syntax working
- **Cross-Module Access**: ✅ Functions in modules accessible from outside
- **Code Generation**: ✅ Module structures generated in target languages
- **Nested Module Support**: ✅ Full nested module functionality
- **List Comprehensions**: ✅ Full Python-style list comprehension support
- **Unpacking Operator**: ✅ Working unpacking in list literals

## Previous Status: native Python support Complete with Module System Implementation (2025-01-20)

### native Python support (native Python operations) - v0.33 COMPLETE ✅
- **Phase 1 - Type Conversions**: `str()`, `int()`, `float()`, `bool()` ✅
- **Phase 2 - List Operations**: Full suite of list methods and properties ✅
- **Phase 3 - String Operations**: Core string methods working ✅
- **Test Coverage**: 189/189 tests passing (100% success rate) 🎉
- **Module Integration**: native Python operations works seamlessly with new module system
- **Backward Compatible**: Existing backtick syntax still works

### Critical Fix Applied
- **native Python operations Registry Conflict**: Removed 'add' from native Python operations registry to prevent conflicts with user-defined functions
- **Issue**: User function `add(5, 3)` was incorrectly recognized as native Python operations SetAdd operation
- **Solution**: Commented out 'add' registration in `framec/src/frame_c/fsl/mod.rs`
- **Impact**: Resolved test_scope_isolation.frm failure

### native Python operations Phase 1: Type Conversions ✅
- `str()`, `int()`, `float()`, `bool()` work without backticks
- Direct transpilation to target language built-ins
- Two-pass parsing recognizes native Python operations operations during semantic analysis

### native Python operations Phase 2: List Operations ✅
- **Basic Methods**: `append()`, `pop()`, `clear()`
- **Advanced Methods**: `insert()`, `remove()`, `extend()`, `reverse()`, `sort()`, `copy()`
- **Query Methods**: `index()`, `count()`
- **Properties**: `.length` → `len()`, `.is_empty` → `len() == 0`
- **Negative Indexing**: Full Python-style negative index support

### native Python operations Phase 3: String Operations ✅
- **Working Methods**: `trim()` → `strip()`, `upper()`, `lower()`, `replace()`, `split()`
- **Properties**: `.length` → `len()`
- **Partial Support**: `contains()` and `substring()` need additional visitor work

### Implementation Architecture
- **Parser Enhancement**: Fixed BuiltInCallExprT handling in unary_expression
- **Visitor Transformations**: Property access converted during code generation
- **Debug Mode**: Added environment variable control for debug output
- **Extensible Design**: Ready for additional operations and target languages

## Previous Status: List Enhancement Planning (2025-09-02)

### List Support Enhancements - IN PROGRESS 🚧
- **Analysis Complete**: Comprehensive analysis of list support across all target languages
- **C Support Confirmed**: Runtime library approach ensures full C compatibility
- **Portability**: Core features work across all languages with appropriate adaptations
- **Implementation Plan**: Phased approach starting with core operations

## Previous Status: SystemReturn Token Implementation (2025-09-02)

### SystemReturn Token Enhancement - COMPLETE ✅
- **Scanner Enhancement**: Greedily matches "system.return" as single `TokenType::SystemReturn`
- **Parser Simplification**: Removed complex `parse_system_interface_call()` method
- **Error Handling**: Clear error for bare `system` keyword - reserved for future use
- **AST Cleanup**: Removed `CallContextType::SystemCall` variant
- **Important**: `system.return` is the ONLY valid use of `system` keyword
- **Test Results**: 170/173 tests passing (3 tests use invalid `system.method()` syntax)

## Previous Status: Enum Enhancements Complete with 100% Test Coverage (2025-09-02)

### v0.32 Final Status - COMPLETE ✅
- **Enum Features**: Custom values, string enums, iteration, module-scope support
- **Test Coverage**: 170/170 tests passing (100% success rate)
- **Bug Fix**: Fixed enum qualification bug in Python code generation
- **Type System**: Enhanced with EnumType (Integer/String) and flexible EnumValue
- **Module Support**: Enums can be declared at module level, accessible everywhere
- **Iteration**: Full support for `for...in` loops over enum values

### v0.31 Final Status
- **Test Coverage**: 166/166 tests passing (100% success rate)
- **Legacy Syntax**: All v0.11 syntax **completely removed** from transpiler
- **Self.Variable Syntax**: Fixed double reference bug (self.self.variable issue)
- **Static Method Calls**: Fixed cross-system static method calls
- **Module Variables**: Full support with automatic global generation
- **Import Statements**: Native Python import support without backticks

## Development History

### 2025-01-20: Module System Implementation Complete (v0.34)

#### Overview
Completed the full implementation of Frame's module system, including module declarations, nested module support, qualified name resolution, and making native Python operations an optional import to prevent namespace conflicts. Achieved 100% test success rate with all module features working.

#### Key Changes
1. **Scanner Updates**:
   - Added `Module` token type for module keyword recognition

2. **AST Enhancements**:
   - Added `ModuleNode` structure with support for nested modules
   - Updated `FrameModule` to include modules vector

3. **Parser Implementation**:
   - Added `module_declaration()` method for parsing module blocks
   - Implemented native Python operations import tracking with HashMap<String, bool>
   - Modified native Python operations recognition to check if operations are imported

4. **Symbol Table Updates**:
   - Added `NamedModule` variant to `ParseScopeType` enum
   - Implemented scope management for nested modules
   - Fixed compilation issues with proper scope handling

5. **native Python operations Changes**:
   - native Python operations operations now require explicit import: `from fsl import str, int`
   - Without import, operations like `str()` are treated as external functions
   - Prevents namespace conflicts with user-defined functions

#### Test Results
- ✅ Empty module declarations parse successfully
- ✅ Module syntax recognized by parser  
- ✅ native Python operations import requirement working correctly
- ✅ native Python operations imports filtered from Python output (no ModuleNotFoundError)
- ✅ native Python operations operations work with explicit import
- ✅ External function calls work without native Python operations import
- ✅ Qualified name resolution fully implemented and working
- ✅ Module functions and variables accessible from outside modules
- ✅ Nested modules with proper scope resolution
- ✅ Module code generation creates proper Python module structures
- ✅ 100% test success rate (189/189 tests passing)

#### Latest Fix (2025-01-20)
- **Python Visitor Update**: Added native Python operations import filtering to prevent `ModuleNotFoundError`
- **Implementation**: native Python operations imports starting with "fsl" are not output to Python code
- **Rationale**: native Python operations operations map to Python built-ins, no actual module needed

### 2025-09-03: native Python support Phase 1 (v0.33)

#### Overview
Implemented the native Python support (native Python operations) to provide native built-in operations that work consistently across all target languages without requiring backticks.

#### Key Implementation Details
1. **Two-Pass Parsing Enhancement**:
   - First pass: Builds symbol table, treats native Python operations ops as regular calls
   - Second pass: Recognizes native Python operations operations via FslRegistry
   - Converts recognized operations to BuiltInCallExprT nodes

2. **Parser Fix**:
   - Added BuiltInCallExprT case to unary_expression function
   - Prevents "TODO" parse errors for native Python operations operations
   - Allows native Python operations operations to propagate through expression chain

3. **native Python operations Registry Architecture**:
   ```rust
   pub struct FslRegistry {
       operations: HashMap<String, BuiltInOperation>,
   }
   ```

4. **Visitor Implementation**:
   - Python visitor handles BuiltInCallExprT nodes
   - Generates direct Python built-in calls (str, int, float)

#### Phase 1 Completed Operations
- `str(expr)` - Convert to string
- `int(expr)` - Convert to integer  
- `float(expr)` - Convert to float

#### Test Results
- New test: `test_fsl_simple.frm`
- Total: 171/171 tests passing (100%)

### 2025-09-02: List Support Enhancements (v0.33)

#### Overview
Enhancing Frame's list support to provide native syntax for common operations while maintaining portability across all target languages including C.

#### Current State
Frame already supports:
- List literals: `[1, 2, 3]`
- List indexing: `list[0]`
- List iteration: `for item in list`
- Lists as variables, parameters, returns

Current limitations:
- Must use backticks for methods: `` `list.append(x)` ``
- No slicing support
- No negative indexing
- No list comprehensions

#### Implementation Plan

##### Phase 1: Core List Operations (Priority: HIGH)
**Timeline**: 2-3 days

Add native Frame syntax for essential list operations:

```frame
// Native list methods (no backticks needed)
list.append(item)       // Add to end
list.length            // Get size (property)
list.is_empty         // Check if empty (property)
list.clear()           // Remove all items
list.pop()            // Remove and return last
list.pop(index)       // Remove and return at index
```

**AST Changes**:
- Add `ListMethodCallNode` for list-specific method calls
- Add `ListPropertyNode` for `.length` and `.is_empty`

**Parser Changes**:
- Recognize `identifier.method()` where identifier is known to be a list
- Special handling for list properties

**Code Generation**:
- Python: Direct method calls
- JavaScript: Map to appropriate methods (`push`, `length` property)
- C: Call runtime library functions
- Java: ArrayList methods
- Go: Slice operations

##### Phase 2: List Indexing Enhancements (Priority: HIGH)
**Timeline**: 2 days

Support negative indexing and bounds checking:

```frame
var last = list[-1]        // Last element
var second_last = list[-2]  // Second to last
```

**Implementation**:
- Extend `ListElementNode` to handle negative indices
- Generate appropriate index calculation per target

##### Phase 3: List Slicing (Priority: MEDIUM)
**Timeline**: 3 days

Implement Python-style slicing:

```frame
var sublist = list[1:4]    // Elements 1, 2, 3
var tail = list[1:]        // All but first
var head = list[:3]        // First 3 elements
var copy = list[:]         // Full copy
var reversed = list[::-1]  // Reverse
```

**AST Changes**:
- Add `ListSliceNode` with start, stop, step expressions
- Extend parser to recognize `:` in bracket expressions

**Code Generation**:
- Python: Native slice syntax
- JavaScript: `slice()` method
- C: `frame_list_slice()` function
- Java: `subList()` method
- Go: Native slice syntax

##### Phase 4: Additional Operations (Priority: MEDIUM)
**Timeline**: 2 days

```frame
list.insert(index, item)   // Insert at position
list.remove(item)         // Remove first occurrence
list.contains(item)       // Check membership
list.index_of(item)       // Find position
list.extend(other_list)   // Add all items from another list
```

##### Phase 5: List Comprehensions (Priority: LOW)
**Timeline**: 4 days

Support Python-style comprehensions:

```frame
var squares = [x * x for x in range(10)]
var evens = [x for x in numbers if x % 2 == 0]
```

**AST Changes**:
- Add `ListComprehensionNode`
- Support for comprehension clauses

**Code Generation**:
- Python: Native comprehension
- JavaScript: `map()`/`filter()` chains
- C: Expand to loops
- Java: Stream API
- Go: Expand to loops

#### C Runtime Library Design

Create `frame_runtime/lists.h`:

```c
typedef struct {
    void** items;
    size_t length;
    size_t capacity;
    size_t item_size;
} frame_list_t;

// Core operations
frame_list_t* frame_list_new(size_t item_size);
void frame_list_free(frame_list_t* list);
void frame_list_append(frame_list_t* list, void* item);
void* frame_list_get(frame_list_t* list, size_t index);
size_t frame_list_length(frame_list_t* list);
// ... etc
```

Generate type-safe macros:
```c
#define FRAME_LIST_INT_NEW() frame_list_new(sizeof(int))
#define FRAME_LIST_INT_APPEND(list, val) /* ... */
#define FRAME_LIST_INT_GET(list, idx) (*(int*)frame_list_get(list, idx))
```

#### Testing Strategy

1. **Unit Tests**: Each new operation gets tests
2. **Cross-Language Tests**: Verify identical behavior across targets
3. **Performance Tests**: Ensure efficient code generation
4. **Memory Tests**: For C target, use Valgrind

Test files to create:
- `test_list_operations.frm` - Core operations
- `test_list_slicing.frm` - Slicing features
- `test_list_comprehensions.frm` - Comprehensions
- `test_list_memory.frm` - C memory management

#### Breaking Changes
None - all existing list code continues to work.

#### Migration Path
- Backtick syntax remains supported
- New native syntax is preferred
- Linter can suggest updates

## Development History

### 2025-09-02: Comprehensive Enum Enhancements (v0.32)

#### Overview
Implemented major enhancements to Frame's enum system, adding support for custom values, string enums, iteration, and module-scope declarations. This brings Frame's enums to feature parity with modern languages while maintaining backward compatibility.

#### New Features

1. **Custom Integer Values**:
   - Explicit value assignment with `= number` syntax
   - Support for negative values
   - Auto-increment from last explicit value
   ```frame
   enum HttpStatus {
       Ok = 200
       Created = 201
       BadRequest = 400
   }
   ```

2. **String Enums**:
   - Type annotation with `: string` syntax
   - String literal values with quotes
   - Auto-generation of string values from names
   ```frame
   enum Environment : string {
       Development = "dev"
       Staging = "staging"
       Production  // Auto: "Production"
   }
   ```

3. **Enum Iteration**:
   - `for...in` loops over enum values
   - Automatic detection of enum types in iteration
   - Access to `.name` and `.value` properties
   ```frame
   for status in HttpStatus {
       print(status.name + ": " + status.value)
   }
   ```

4. **Module-Level Enums**:
   - Declare enums outside systems at module scope
   - Accessible from all functions and systems
   - Proper scoping and type checking
   ```frame
   enum GlobalStatus {
       Active
       Inactive
   }
   
   fn main() {
       var s = GlobalStatus.Active
   }
   ```

#### Technical Implementation

1. **AST Changes** (`ast.rs`):
   - Added `EnumType` enum: `Integer` | `String`
   - Added `EnumValue` enum: `Integer(i32)` | `String(String)` | `Auto`
   - Modified `EnumDeclNode` to include `enum_type` field
   - Updated `EnumeratorDeclNode` to use `EnumValue` instead of `i32`
   - Added `enums` field to `FrameModule` struct
   - Extended `ForStmtNode` with enum iteration tracking

2. **Parser Updates** (`parser.rs`):
   - Enhanced `enum_decl()` to parse type annotations
   - Added support for string literals and negative numbers
   - Modified `for_in_statement()` to detect enum iteration
   - Added proper symbol table integration for enum types

3. **Python Visitor** (`python_visitor.rs`):
   - Updated enum value generation for different types
   - Modified for loop generation for enum iteration
   - Added module-level enum processing
   - Proper `from enum import Enum` import generation
   - **Bug Fix**: Fixed enum member qualification in `visit_identifier_node`
     - Detects dot-notation enum references (e.g., "HttpStatus.Ok")
     - Properly qualifies with system name (e.g., "HttpServer_HttpStatus.Ok")
     - Distinguishes between module-level and system-level enums

4. **Symbol Table**:
   - Proper tracking of enum types in symbol table
   - Enum iteration detection in for loops
   - Module vs system scope handling

#### Test Coverage
- `test_enum_custom_values.frm`: Integer enums with custom/negative values
- `test_enum_string_values.frm`: String enums with explicit/auto values
- `test_enum_iteration.frm`: For loop iteration over enums
- `test_enum_module_scope.frm`: Module-level enum declarations

#### Files Modified
- `framec/src/frame_c/ast.rs`: AST structure updates
- `framec/src/frame_c/parser.rs`: Parser enhancements
- `framec/src/frame_c/visitors/python_visitor.rs`: Code generation
- `docs/source/language/grammar.md`: Grammar documentation
- `CLAUDE.md`: Project documentation updates

### 2025-09-01: Complete Removal of Legacy v0.11 Syntax

#### Overview
Completed the removal of all deprecated v0.11 syntax from the Frame transpiler, ensuring the language uses only modern v0.20+ syntax.

#### Removed Features
1. **Removed Tokens from Scanner**:
   - `Caret` (^) - Old return syntax
   - `ReturnAssign` (^=) - Old return assignment
   - `Hash` (#) - Old system declaration (kept for attributes)
   - `ColonBar` (:|) - Test terminator
   - `ThreeTicks` (```) - Unused token
   - `StringMatchStart` (~/) - String pattern matching
   - `NumberMatchStart` (#/) - Number pattern matching
   - `EnumMatchStart` (:/) - Enum pattern matching

2. **Parser Modifications**:
   - Removed all references to deprecated tokens
   - Commented out unreachable ternary test code
   - Functions now return errors for deprecated syntax

3. **Documentation Updates**:
   - Updated all documentation to reflect removal
   - No more "deprecated" - everything is "removed"
   - Clear migration paths provided

#### Test Results
- **100% success rate** maintained (166/166 tests passing)
- All tests validated with both transpilation and execution
- No functionality lost - modern syntax fully supports all use cases

### 2025-09-01: Self.Variable and Static Call Fixes

#### Overview
Fixed critical bugs in self.variable syntax and static method calls on other systems, achieving 100% test success rate.

#### Issues Fixed

1. **Self.Variable Double Reference Bug**
   - **Problem**: `self.x` was generating `self.self.x` in Python output
   - **Solution**: Modified call chain processing to detect and skip the first "self" node when processing self.variable patterns
   - **Location**: `python_visitor.rs` lines 5289-5433
   - **Tests Fixed**: test_self_domain_vars.frm, test_self_variable_exhaustive.frm, and 5 others

2. **Static Method Calls on Other Systems**
   - **Problem**: `UtilitySystem.calculate(42)` was generating `UtilitySystem.self.calculate(42)`
   - **Solution**: Modified operation call generation to detect when output already has a system prefix and skip adding "self."
   - **Location**: `python_visitor.rs` lines 4906-4919
   - **Tests Fixed**: test_static_calls.frm

3. **Test File Syntax Error**
   - **Problem**: `test_v031_comprehensive.frm` had incorrect domain variable syntax
   - **Solution**: Added `var` keyword to domain variable declaration
   - **Location**: `test_v031_comprehensive.frm` line 48

#### Test Results
- **Before**: 94.6% success rate (157/166 tests passing)
- **After**: 100% success rate (166/166 tests passing)
- All v0.31 features fully functional with complete test coverage

### 2025-01-31: Domain Variable Assignment Support

#### Overview
Fixed domain variable assignment syntax to support `self.variable = value` expressions. Previously, attempting to assign to domain variables using the standard `self.` syntax would fail during parsing.

#### Technical Implementation
- **Parser Enhancement**: Modified `assign()` method in parser.rs to recognize `self.variable` CallChainExprT patterns
- **CallChain Generation**: Updated `parse_self_context()` to create proper CallChainExprT for `self.variable` expressions
- **Python Visitor Fix**: Added special handling in `visit_call_chain_expr_node()` to avoid double `self.self` output

#### Key Changes
1. **Parser (parser.rs:9619-9682)**:
   - Added check for CallChainExprT starting with "self" in assign method
   - Allows domain variable assignments to pass validation

2. **Parser (parser.rs:9871-9911)**:
   - Modified `parse_self_context()` to create CallChainExprT for `self.variable`
   - Builds proper two-node chain: [self, variable]

3. **Python Visitor (python_visitor.rs:5292-5308)**:
   - Added special case detection for `self.domain_variable` patterns
   - Outputs clean `self.variable` syntax instead of `self.self.variable`

#### Test Results
- Fixed `test_domain_assignment.frm` - now correctly handles `self.counter = 25`
- Validates both reading (`self.counter`) and writing (`self.counter = value`)

### 2025-01-31: Module Variables with Automatic Global Declarations

#### Overview
Implemented comprehensive support for module-level variables with automatic `global` declaration generation for Python target. This eliminates UnboundLocalError runtime errors and provides a clean, natural syntax for module variable access.

#### Key Features
- **Automatic Global Generation**: Transpiler detects when module variables are modified and generates `global` declarations
- **Function Support**: Works for all standalone functions that modify module variables  
- **System Support**: Also generates globals for system state methods
- **Conditional Imports**: Only generates imports (like `from enum import Enum`) when actually used
- **Shadowing Protection**: Prevents local variables from shadowing module variables (Python limitation)

#### Technical Implementation
- **Two-Pass Analysis**: First identifies local declarations, then detects module variable modifications
- **CallChainExprT Support**: Handles v0.30's assignment syntax properly
- **HashSet Tracking**: Uses `global_vars_in_function` and `required_imports` HashSets for efficient tracking
- **Shadowing Check in Parser**: Added semantic analysis check in `var_declaration` method (parser.rs:3325-3356)
  - Uses `arcanum.lookup` with `UnknownScope` to search entire scope chain
  - Only triggers error for `ModuleVariable` type symbols
  - Provides clear error message at transpilation time

#### Test Results
- **Success Rate**: Improved to 98.2% (162/165 tests passing)
- **New Test**: `test_module_scope_comprehensive.frm` validates all features
- **Fixed Tests**: Module variable tests now pass with proper global declarations

#### Files Modified
- `framec/src/frame_c/visitors/python_visitor.rs`: Added global declaration generation logic
- `docs/source/language/grammar.md`: Documented module variable syntax and features
- `CLAUDE.md`: Updated with implementation details

### 2025-08-31: None Keyword Standardization

#### Overview
Standardized on `None` as the single null value keyword in Frame, completely removing support for `null` and `nil`. This aligns Frame with Python conventions and simplifies the language.

#### Changes Made
- **Scanner**: Removed `null` and `nil` from keywords map
- **TokenType**: Removed `Null` and `Nil` enum variants
- **Parser**: Removed deprecated keyword handling
- **All Visitors**: Updated to only recognize `None_` token
- **Documentation**: Updated all references to reflect None-only syntax

#### Migration Impact
- **Breaking Change**: Code using `null` or `nil` will no longer compile
- **Migration Path**: Replace all instances of `null` and `nil` with `None`
- **Error Behavior**: `null` and `nil` are now treated as undefined identifiers

### 2025-08-31: Scope Handling Implementation Complete

#### Overview
Completed all 7 phases of the comprehensive scope handling implementation plan, achieving proper LEGB (Local, Enclosing, Global, Built-in) scope resolution and full isolation between functions and systems.

#### Key Achievements
- **Parser Fix**: Added scope context checking before ActionCallExprNode creation
- **Function Isolation**: Functions cannot call system actions/operations
- **System Isolation**: Systems cannot access other systems' internals  
- **LEGB Resolution**: Proper symbol lookup order with shadowing support
- **Test Coverage**: 158/158 tests passing (100% success rate)

#### Technical Changes
1. **Parser (parser.rs:7818-7826)**:
   - Check `ScopeContext::Function` before creating ActionCallExprNode
   - Functions treat action calls as undeclared external calls
   
2. **Symbol Table**:
   - `legb_lookup()` method fully implemented
   - `is_symbol_accessible()` enforces scope boundaries
   - Proper ScopeContext tracking (Global/Function/System)

3. **Test Files Added**:
   - `test_scope_isolation.frm`: Validates function/system isolation
   - `test_legb_resolution.frm`: Tests LEGB lookup order

### 2025-08-31: System Return Semantics Design

#### Overview
Introduced `system.return` as a distinct concept from regular function returns, clarifying Frame's dual return semantic model. The system return is the value returned to the original interface caller, persisting through any depth of calls and state transitions.

#### Core Design Principles

1. **System Return** (`system.return`)
   - The value returned to the external caller of a system interface method
   - Can be set from anywhere during interface call execution
   - Persists through state transitions and nested calls
   - Last write wins - final value when interface call completes is what caller receives

2. **Regular Return** (`return value`)
   - Returns value to immediate caller
   - Used in actions and operations for internal call chains
   - Does not affect system.return

3. **Default Return Values** (`: type = default`)
   - Interface declarations: `validate() : bool = false` sets initial system.return
   - Event handlers: `validate() : bool = true {` overrides system.return default on entry
   - Actions/Operations: `helper() : int = -1 {` sets default return to caller (not system.return)

#### Context-Specific Rules

| Context | `: type = default` sets | `system.return = value` | `return value` | `return` |
|---------|-------------------------|-------------------------|----------------|----------|
| Interface | Initial system.return | N/A | N/A | N/A |
| Event Handler | system.return on entry | ✅ Can override | ❌ N/A | ✅ Exit handler |
| Action | Default return to caller | ✅ Must set explicitly | ✅ Return to caller | ✅ Exit action |
| Operation | Default return to caller | ❌ Compile error | ✅ Return to caller | ✅ Exit operation |

#### Why Operations Cannot Use system.return
Operations are static methods that can be called:
- Directly from outside: `Calculator.calculate(5)` - no interface context
- From functions: `fn main() { Calculator.calculate(5) }` - no system instance  
- From other operations: Pure functional composition

Since there's no guarantee they're called through an interface, `system.return` is meaningless and should be a compile-time error.

#### Example Usage
```frame
system Validator {
    interface:
        check() : bool = true      // Default: system.return = true
        
    machine:
        $Start {
            check() : bool = false {  // Override: system.return = false on entry
                processData()         // Call action
                if critical {
                    system.return = true  // Explicit override
                }
                return               // Exit with current system.return value
            }
        }
        
    actions:
        processData() : int = -1 {   // Default for action's return to caller
            if error {
                system.return = false // Explicitly set interface return
                return 0             // Return to event handler
            }
            return 1                // Return to event handler (system.return unchanged)
        }
        
    operations:
        @staticmethod
        validate(x: int) : int = 0 { // Default for operation's return
            // system.return = x     // ERROR: operations cannot use system.return
            if x > 0 {
                return x * 2
            }
            // Implicit return 0
        }
}
```

#### Grammar Changes Required
1. Add grammar rule for `system.return` as special compound identifier
2. Parse `: type = value` syntax in interface, event handler, action, and operation declarations
3. Add validation that operations cannot use `system.return`
4. Create SystemReturnNode AST node type

#### Implementation Status
- Design: ✅ Complete (2025-08-31)
- Parser: 🔄 In Progress
- AST: 🔄 In Progress
- Code Generation: 🔄 In Progress
- Tests: 📝 Planned

### 2025-01-31: Self Expression Support & Static Operation Validation

#### Self as Standalone Expression
- **Achievement**: The `self` keyword can now be used as a standalone expression, not just with dotted access
- **Issue**: Parser required `self.something` syntax, preventing use of bare `self` as function argument
- **Solution**: 
  - Modified `parse_self_context()` to allow standalone `self` when not followed by a dot
  - Creates special variable node representing the system instance
  - Updated Python visitor to handle standalone `self` correctly
- **Use Case**: Enables `jsonpickle.encode(self)` without backticks in persistence operations
- **Files Modified**: `framec/src/frame_c/parser.rs` (lines 9605-9619), `framec/src/frame_c/visitors/python_visitor.rs` (lines 562-570)

#### Static Operation Improvements
- **Achievement**: Operations are only static when explicitly declared with `@staticmethod`
- **Previous Behavior**: All operations were generated as static methods
- **New Behavior**: 
  - Operations without `@staticmethod` are instance methods with implicit `self` parameter
  - Operations with `@staticmethod` are static methods without `self` parameter
  - Static operations that use `self` trigger a parse error
- **Validation**: Parser checks `is_static_operation` flag and errors if `self` is used in static context
- **Error Message**: "Cannot use 'self' in a static operation (marked with @staticmethod)"
- **Files Modified**: `framec/src/frame_c/parser.rs` (lines 2668-2672, 9607-9611), `framec/src/frame_c/visitors/python_visitor.rs` (lines 3980-4000)

### 2025-01-30: Function-System Scope Interaction & Complete Multi-Entity Support

#### Function-Operation Integration Complete
- **Achievement**: Functions can now properly call system operations using correct static method syntax
- **Issue**: Functions calling operations generated as bare calls (`add(5, 3)`) instead of static method calls (`Utils.add(5, 3)`)
- **Solution**: 
  - Modified operations to generate as `@staticmethod` by default for external accessibility
  - Updated call generation logic to use `SystemName.operationName()` syntax when called from standalone functions
  - Fixed call chain handling to avoid double system name prefixes (`Utils.Utils.add` → `Utils.add`)
- **Frame Source Syntax**: Functions must use `Utils.add(5, 3)` syntax to call system operations
- **Generated Python**: Correctly produces static method calls with proper `@staticmethod` decorators
- **Files Modified**: `framec/src/frame_c/visitors/python_visitor.rs` (lines 3973-3978, 4471-4484, 4571-4578)

#### Complete Multi-Entity Architecture
- **Functions**: Multiple functions per module with any names, full system integration
- **Systems**: Multiple systems per module with proper isolation and cross-system calls
- **Operations**: Always public (static methods) - callable from functions and other systems
- **Actions**: Always private (instance methods with `_` prefix) - only callable within system
- **Interface Methods**: Always public (instance methods) - callable from external code

#### Test Success Rate Achievement
- **Before Function Fixes**: 95.2% success rate (139 passed, 7 failed)
- **After Function Fixes**: 97.3% success rate (142 passed, 4 failed)
- **After All v0.31 Fixes**: 100% success rate (153 passed, 0 failed)

### 2025-01-29: Native Import Statement Support

#### Import Statement Implementation
- **Achievement**: Frame v0.31 now supports native Python import statements without backticks
- **Syntax Types Supported**:
  - Simple imports: `import math`
  - Aliased imports: `import numpy as np`
  - From imports: `from typing import List, Dict`
  - Wildcard imports: `from collections import *`
- **Implementation**: Added ImportNode to AST with four import types
- **Parser**: New `parse_import_statement()` and `parse_from_import_statement()` methods
- **Scanner**: Added `Import`, `From`, `As` token types
- **Code Generation**: Direct pass-through to Python output
- **Files Modified**: `parser.rs`, `scanner.rs`, `ast.rs`, `python_visitor.rs`

### 2025-01-28: Scope Resolution & LEGB Implementation

#### Python LEGB Scope Resolution
- **Issue**: Frame was not properly implementing Python's LEGB (Local, Enclosing, Global, Built-in) scope resolution
- **Solution**: Modified symbol table and code generation to respect Python's scope rules
- **Key Changes**:
  - Local variables properly shadow outer scope variables
  - Built-in functions accessible without declaration
  - Module-level variables accessible with proper scoping
- **Impact**: Fixed multiple test failures related to variable shadowing and built-in access

### 2025-01-27: Hierarchical State Machine (HSM) Improvements

#### Parent Dispatch Router Integration
- **Achievement**: Parent dispatch now uses unified `__router` infrastructure
- **Previous Issue**: Hardcoded parent state method names caused maintenance issues
- **Solution**: Modified router signature to accept optional compartment parameter
- **Router Signature**: `__router(self, __e, compartment=None)`
- **Parent Dispatch**: `self.__router(__e, compartment.parent_compartment)`
- **Benefits**: Dynamic state resolution, no hardcoded names, single routing logic point

#### HSM Infinite Recursion Fix
- **Issue**: Parent dispatch caused infinite recursion due to improper compartment initialization
- **Root Cause**: Child compartments had `parent_compartment=None`
- **Solution**: Proper parent compartment references in hierarchical states
- **Generated Code**: `FrameCompartment('Child', ..., FrameCompartment('Parent', ...))`

### 2025-01-26: Multi-Entity Module Support

#### Module Architecture Redesign
- **Achievement**: Proper FrameModule container with peer Functions[] and Systems[]
- **Previous**: SystemNode-centric design with artificial parent-child relationships
- **New**: Functions and systems are peer entities within modules
- **Parser**: Sequential entity parsing supporting any combination
- **Symbol Table**: System-scoped state resolution with proper isolation

#### Call Chain Scope Processing Fix
- **Critical Bug**: External object method calls generated incorrect `obj.self.method()` syntax
- **Solution**: Conditional flag setting in call chain processing
- **Impact**: Properly distinguishes between external and internal call contexts

### 2025-01-25: State Stack Operations

#### State Stack Implementation
- **Operators**: `$$[+]` (push state), `$$[-]` (pop state)
- **Use Case**: History mechanisms and modal state preservation
- **Validation**: All state stack tests passing including complex nested sequences
- **Variable Preservation**: State variables maintain values through push/pop cycles

### 2025-01-24: Operations Block & Scope Handling

#### Operations vs Actions Clarification
- **Operations**: Public methods (can be static with `@staticmethod`)
- **Actions**: Private implementation methods (always instance methods with `_` prefix)
- **Scope Resolution**: Operations accessible externally, actions only within system
- **Static Validation**: Parse-time checking prevents `self` usage in static operations

## Release Notes Format

### Version Numbering
- v0.30: Multi-entity support, HSM improvements, state stack operations
- v0.31: Import statements, self expression, static validation, 100% test success
- v0.32: Comprehensive enum enhancements (custom values, string enums, iteration, module-scope)

### Test Success Tracking
- Track both transpilation success and execution success
- Document specific test fixes and their solutions
- Maintain test matrix in `framec_tests/reports/test_log.md`
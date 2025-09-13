# Frame Transpiler Architecture v0.57 (Target)

## Overview

The Frame transpiler v0.57 extends the single-file compiler to support multi-file programs with module imports, dependency resolution, and incremental compilation capabilities. The system maintains backward compatibility while enabling large-scale program development.

**Target Version**: v0.57  
**Key Addition**: Multi-file module support  
**Compilation Model**: Dependency-aware with caching

## Compilation Pipeline

```
Multiple Frame Sources (.frm)
    ↓
Import Resolution → framec/src/frame_c/modules/resolver.rs
    ↓
Dependency Graph → framec/src/frame_c/modules/graph.rs
    ↓
For each module (in dependency order):
    ↓
    Scanner → framec/src/frame_c/scanner.rs
    ↓  
    Parser → framec/src/frame_c/parser/mod.rs (modularized)
    ↓
    AST → framec/src/frame_c/ast.rs
    ↓
    Symbol Table → framec/src/frame_c/symbol_table.rs
    ↓
    Module Cache → .frame/cache/module.frmc
    ↓
Linker → framec/src/frame_c/modules/linker.rs
    ↓
Visitor → framec/src/frame_c/visitors/python_visitor.rs
    ↓
Target Code (Python)
```

## Core Components

### Module Resolver (`modules/resolver.rs`) - NEW
**Purpose**: Resolves import statements to actual file paths.

**Key Features**:
- Path resolution with search directories
- Circular dependency detection
- Module cache management
- Project module resolution

**Resolution Order**:
1. Relative to current file (`./`, `../`)
2. Project source directories (from frame.toml)

### Dependency Graph (`modules/graph.rs`) - NEW
**Purpose**: Builds and validates module dependency relationships.

**Key Features**:
- Topological sorting for compilation order
- Cycle detection with clear reporting
- Incremental update support
- Dependency tracking for cache invalidation

### Module Cache (`modules/cache.rs`) - NEW
**Purpose**: Caches parsed modules for incremental compilation.

**Cache Structure** (JSON format):
```json
{
  "version": "1.0",
  "frame_version": "0.57",
  "module_path": "Utils::Math",
  "source_hash": "abc123...",
  "exports": {
    "functions": [...],
    "systems": [...],
    "classes": [...],
    "type_aliases": [...],
    "variables": [...]
  },
  "imports": [...],
  "metadata": {
    "source_file": "src/utils/math.frm",
    "last_modified": "2025-01-25T10:00:00Z",
    "dependencies": [...]
  }
}
```

**Cache Location**: `.frame/cache/`

### Scanner (Enhanced)
**New Token Types**:
- `TokenType::DoubleColon` - Module path separator (`::`)

**No other changes** - Scanner remains largely unchanged.

### Parser (Modularized)
**Structure**:
```
framec/src/frame_c/parser/
├── mod.rs              # Public API
├── parser.rs           # Core parser struct
├── expressions.rs      # Expression parsing
├── statements.rs       # Statement parsing
├── types.rs           # Type parsing
├── imports.rs         # Import parsing (NEW)
└── errors.rs          # Error handling
```

**New Parsing Features**:
- Import statement parsing with module paths
- Module path resolution in qualified names
- Cross-file symbol references

### AST (Extended)
**Enhanced FrameModule**:
```rust
pub struct FrameModule {
    // Existing fields...
    pub source_file: Option<PathBuf>,  // NEW: Source location
    pub module_path: ModulePath,       // NEW: Module identifier
}
```

**New Import Node Types**:
```rust
pub struct ImportNode {
    pub module_path: String,        // "Utils::Math" or "./utils.frm"
    pub source_type: ImportSource,  // File, StandardLib, Package
    pub imported_items: Vec<ImportItem>,
    pub module_alias: Option<String>,
}

pub enum ImportSource {
    File(PathBuf),      // Local .frm file
    Package(String),     // Future: external packages
}
```

### Symbol Table (Extended)
**Enhanced ModuleSymbol**:
```rust
pub struct ModuleSymbol {
    pub name: String,
    pub symtab_rcref: Rc<RefCell<SymbolTable>>,
    pub imports: Vec<ImportDef>,        // NEW: Import tracking
    pub exports: HashSet<String>,       // NEW: Export tracking
    pub source_file: Option<PathBuf>,   // NEW: Source location
}
```

**Cross-Module Resolution**:
- Extended LEGB to handle imported symbols
- Qualified name resolution across module boundaries
- Export visibility enforcement

### Linker (`modules/linker.rs`) - NEW
**Purpose**: Combines multiple modules into final output.

**Linking Strategies**:
1. **Concatenation** (v0.57): Simple module concatenation in dependency order
2. **Smart Linking** (Future): Dead code elimination, optimization

### Python Visitor (Enhanced)
**Multi-Module Generation**:
```python
# Generated structure for multi-file Frame program

# Module: Utils::Math
class Utils_Math:
    @staticmethod
    def add(a, b):
        return a + b

# Module: App::Main
class App_Main:
    def __init__(self):
        # Uses Utils::Math
        self.result = Utils_Math.add(3, 4)
```

## Language Features (Multi-File)

### Import Statements
```frame
# Import entire module
import Utils::Math from "./utils/math.frm"

# Import specific items
from "./models.frm" import User, Product

# Import with alias
import VeryLongModuleName as VLMN from "./long.frm"

# Import all (discouraged)
from "./helpers.frm" import *
```

### Module Path Separator
- `::` for module hierarchy: `Math::Trig::sin`
- `.` for member access: `instance.method()`
- Clear distinction between module paths and object access

### Qualified Names
```frame
# Module function call
var result = Math::Trig.sin(angle)

# Module variable access
var pi = Math::Constants.PI

# Class instantiation from module
var user = Models::User("Alice")
```

## Configuration System (Simplified)

### New Structure (`frame.toml`)
```toml
[project]
name = "my-app"
version = "0.1.0"
entry = "src/main.frm"

[build]
output_dir = "dist"
source_dirs = ["src", "lib"]
optimize = false
debug = true

[python]
event_handlers_as_functions = true
runtime = "asyncio"
```

### Config Loading
1. Check for `frame.toml` (new format)
2. Fall back to `config.yaml` with deprecation warning
3. Use defaults if no config found

## Build System

### Build Commands
```bash
# Build multi-file project
framec build                    # Uses frame.toml
framec build --entry main.frm   # Specify entry point
framec build --watch            # Watch mode

# Single-file (backward compatible)
framec -l python_3 file.frm

# With experimental flag (initial release)
framec --experimental-modules build
```

### Build Process
1. Load configuration
2. Resolve all imports from entry point
3. Build dependency graph
4. Check cache for each module
5. Parse uncached/changed modules
6. Generate symbol tables
7. Link modules
8. Generate target code

## Test Infrastructure (Enhanced)

### Test Organization
```
framec_tests/
├── python/
│   ├── src/           # Single-file tests (existing)
│   └── projects/      # Multi-file test projects (NEW)
│       ├── basic_import/
│       │   ├── main.frm
│       │   ├── utils.frm
│       │   └── expected.py
│       └── circular_deps/
│           ├── a.frm
│           ├── b.frm
│           └── expected_error.txt
└── runner/
    ├── frame_test_runner.py      # Enhanced for multi-file
    └── multi_file_runner.py      # Multi-file specific tests
```

## Error Handling

### New Error Types
```rust
pub enum ModuleError {
    NotFound { path: String, searched: Vec<PathBuf> },
    CircularDependency { chain: Vec<String> },
    SymbolConflict { symbol: String, modules: Vec<String> },
    InvalidPath { path: String, reason: String },
}
```

### Error Messages
```
Error: Circular dependency detected
  main.frm → utils.frm → helpers.frm → main.frm
                                          ↑
  Break the cycle by restructuring your modules

Error: Module not found: './utils.frm'
  in main.frm:3:1
  Searched: [src/, lib/]
  Did you mean: './util.frm'?
```

## Performance Characteristics

### Compilation Speed
- First build: O(n) with n modules
- Incremental: O(m) with m changed modules
- Cache lookup: O(1) per module
- Target: <2x slower than equivalent single-file

### Memory Usage
- Module cache reduces redundant parsing
- Symbol tables loaded on demand
- AST pruning for unused exports (future)

### Cache Performance
- JSON format: ~10ms parse per module
- Binary format (future): ~1ms parse per module
- Cache invalidation: Hash-based, O(1)

## Migration Path

### From Single-File to Multi-File
1. Existing single-file programs work unchanged
2. Optional migration tool splits large files
3. Gradual refactoring supported
4. No breaking changes to existing code

### Compatibility
- All v0.56 features supported
- Single-file compilation preserved
- Config migration automated
- Test infrastructure backward compatible

## Future Enhancements (Post-v0.57)

### v0.58: Project System
- Package dependencies
- Build profiles (debug/release)
- Test integration

### v0.59: Optimization
- Binary cache format
- Parallel compilation
- Dead code elimination

### v0.60: Package Ecosystem
- Package registry
- Version management
- Distribution format

---

## Key Architectural Changes from v0.56

### Added Components
1. **Module Resolver** - Import path resolution
2. **Dependency Graph** - Build order determination
3. **Module Cache** - Incremental compilation support
4. **Linker** - Multi-module combination

### Modified Components
1. **Parser** - Modularized into separate files
2. **Config** - Simplified to ~200 lines from 794
3. **Symbol Table** - Extended for cross-file resolution
4. **Test Runner** - Enhanced for multi-file projects

### Unchanged Components
1. **Scanner** - Minimal changes (added `::` token)
2. **AST** - Extended but backward compatible
3. **Visitor** - Enhanced but same architecture
4. **Runtime** - No changes to generated code structure
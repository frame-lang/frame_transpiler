# Frame Transpiler v0.86.72 Release Notes

## Released: 2025-12-13

### 🐛 Bug Fixes

#### Rust Transpiler
- **Fixed duplicate method generation** for interface methods
  - Resolved compilation error `E0592: duplicate definitions with name` 
  - Methods declared in both interface and as event handlers no longer generate duplicate implementations
  - Rust operator tests now pass 100% (5/5) in Docker containers

### 🏗️ Infrastructure Improvements

#### Docker Test Infrastructure
- **Pure Rust implementation** of Docker test runner (`frame-docker-runner`)
  - Replaced Python-based test harness with pure Rust implementation
  - Complete architectural separation between transpiler and test infrastructure
  - All Docker-related test infrastructure now resides in shared environment

#### V3 File Extension Support  
- Updated test infrastructure to support V3 language-specific extensions:
  - Python: `.fpy`
  - TypeScript: `.frts`
  - Rust: `.frs`
  - C: `.fc`
  - C++: `.fcpp`
  - C#: `.frcs`
  - Java: `.fjava`
- Docker runner now correctly recognizes and processes all V3 extensions

### ✅ Test Results

All PRT (Python, Rust, TypeScript) languages now achieve **100% test success** in Docker containers:

- **Python**: 100% success rate
- **TypeScript**: 100% success rate  
- **Rust**: 100% success rate
  - v3_data_types: 5/5 tests ✅
  - v3_systems: 2/2 tests ✅
  - v3_operators: 5/5 tests ✅
  - v3_persistence: 2/2 tests ✅

### 📦 Dependencies

- Removed `serde_json` dependency from Rust transpiler output
- Replaced with custom `FrameValue` enum for better encapsulation
- Generated Rust code now has zero external dependencies for test execution

### 🔧 Technical Details

- Fixed issue where interface methods and event handlers with same name generated duplicate `pub fn` implementations
- Added check in Rust code generation to skip simple event handler wrapper if method exists in interface
- Interface methods now generate proper wrappers with return value handling via `_system_return_stack`

### 🚀 Next Steps

- CI cutover to Docker-based shared environment
- Stage 15: Complete Persistence & Snapshots for TypeScript and Rust

---

For questions or issues, please file a bug report at the [Frame Transpiler repository](https://github.com/frame-lang/frame_transpiler).
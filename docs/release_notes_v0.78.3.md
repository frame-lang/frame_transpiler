# Frame v0.78.3 Release Notes

## 📚 Documentation Corrections Release

This release corrects documentation throughout the codebase to accurately reflect the required block ordering in Frame systems.

## 🔧 Documentation Fixes

### Corrected Block Order Documentation
The parser enforces the following block order in systems:
1. **operations** (if present)
2. **interface** (if present)  
3. **machine** (if present)
4. **actions** (if present)
5. **domain** (if present, must be last)

Previously, documentation incorrectly stated that `interface` came before `operations`.

### Files Updated
- `docs/ai/frame_comprehensive_ai_guide.md` - Fixed parser limitations and system structure documentation
- `docs/ai/frame_quick_reference.md` - Corrected "Block Order (Must Follow)" section
- `docs/ai/frame_pattern_library.md` - Updated generator pattern examples
- `docs/framepiler_design/architecture_v0.56.md` - Fixed system scope descriptions
- `docs/v0.66_achievements.md` - Reordered example code to follow correct syntax

## 🐛 Bug Fixes

### Source Map Improvements (from v0.78.2)
- Added line tracking to `OperationNode` in AST
- Operations now properly map to their source lines in debug output
- Improved debugging experience in VS Code

## 📝 Parser Implementation Reference
The actual parsing order is defined in `framec/src/frame_c/parser.rs`:
```rust
// Lines 612-627 in system() method:
operations_block_node_opt = self.parse_operations_block()?;
interface_block_node_opt = self.parse_interface_block()?;
machine_block_node_opt = self.parse_machine_block()?;
actions_block_node_opt = self.parse_actions_block()?;
domain_block_node_opt = self.parse_domain_block()?;
```

## ⚠️ Important Notes
- No code changes in this release, only documentation corrections
- Existing Frame code continues to work as before
- Systems without operations blocks are unaffected

## Version
- Frame Transpiler: v0.78.3
- Build Tool: v0.78.3  
- Runtime: v0.78.3
# Frame Transpiler Source Map Architecture

**Version:** v0.79.0  
**Date:** 2025-10-01  
**Status:** Production with Validation Infrastructure

## Table of Contents
1. [Overview](#overview)
2. [Architecture Components](#architecture-components)
3. [Source Map Validation Infrastructure](#source-map-validation-infrastructure)
4. [VS Code Extension Integration](#vs-code-extension-integration)
5. [Source Map Flow](#source-map-flow)
6. [Line Tracking System](#line-tracking-system)
7. [Mapping Generation](#mapping-generation)
8. [Quality Standards and Bug Detection](#quality-standards-and-bug-detection)
9. [Debugging Methodology](#debugging-methodology)
10. [Implementation Details](#implementation-details)

## Overview

The Frame transpiler generates source maps to enable debugging of Frame code by mapping Frame source lines to generated Python lines. As of v0.79.0, the system includes comprehensive validation infrastructure that serves as the **source of truth** for both the transpiler team and VS Code extension.

**Key Features (v0.79.0):**
- ✅ **Validation Infrastructure**: Standardized tools for quality assessment
- ✅ **VS Code Extension Integration**: Source of truth protocol for consistent debugging
- ✅ **Bug Detection**: Systematic detection of duplicate mappings and coverage gaps
- ✅ **Quality Classification**: EXCELLENT/GOOD/FAIR/POOR quality ratings
- ✅ **Executable Coverage**: 100% coverage of executable statements achieved

## Architecture Components

### 1. SourceMapBuilder (`source_map.rs`)
- **Purpose**: Collects and manages Frame-to-Python line mappings
- **Key Methods**:
  - `add_simple_mapping(frame_line, python_line)`: Adds a direct mapping
  - `add_mapping_with_marker(marker_id, python_line)`: Resolves marker-based mappings
  - `to_json()`: Exports mappings in JSON format (0-based for debuggers)

### 2. PythonVisitor (`python_visitor.rs`)
- **Purpose**: Generates Python code while tracking line numbers
- **Key Fields**:
  - `current_line`: Tracks the current Python line being generated
  - `source_map_builder`: Reference to the SourceMapBuilder
  - `pending_markers`: HashMap of marker IDs to Frame lines

### 3. Mapping Systems

#### Direct Mapping
```rust
// Directly map current Frame line to current Python line
self.add_source_mapping(node.line);
```

#### Marker-Based Mapping
```rust
// Place marker for later resolution
let marker_id = self.generate_marker_id();
self.pending_markers.insert(marker_id, frame_line);
self.add_code(&format!("# __MARKER_{}__", marker_id));
// Later, when marker is encountered:
self.source_map_builder.add_mapping_with_marker(marker_id, self.current_line);
```

## Source Map Validation Infrastructure

**Version:** v0.79.0+  
**Purpose:** Establish source of truth for source map quality assessment shared between transpiler team and VS Code extension

### Validation Tools Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   FRAME TRANSPILER                          │
│  ┌─────────────────────────────────────────────────────┐   │
│  │            Source Map Generation                    │   │
│  │         (python_visitor_v2.rs)                     │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│               SOURCE OF TRUTH VALIDATION                    │
│  ┌───────────────────┐  ┌─────────────────────────────┐   │
│  │ source_map_       │  │ source_map_test_           │   │
│  │ validator.py      │  │ integration.py              │   │
│  │ (Core Analysis)   │  │ (Test Framework)            │   │
│  └───────────────────┘  └─────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ test_framework_integration.py                       │   │
│  │ (VS Code Extension Interface)                       │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┼─────────────┐
                ▼             ▼             ▼
    ┌─────────────────┐ ┌─────────────┐ ┌─────────────┐
    │   VS Code       │ │ Test Suite  │ │ CI/CD       │
    │   Extension     │ │ Runner      │ │ Pipeline    │
    └─────────────────┘ └─────────────┘ └─────────────┘
```

### Core Validation Tool: `source_map_validator.py`

**Location:** `/tools/source_map_validator.py`  
**Purpose:** Fundamental source map analysis engine for single files

```bash
# Basic validation
python3 tools/source_map_validator.py test_file.frm

# Verbose analysis
python3 tools/source_map_validator.py test_file.frm --verbose

# Example output:
# === SOURCE MAP ANALYSIS REPORT ===
# Transpiler Version: framec 0.79.0
# Total Mappings: 46
# Main Function Coverage: 64.7% (22/34 lines mapped)
# Executable Statement Coverage: 100.0%
# Assessment: ✅ PERFECT: 100% executable statement coverage
```

**Key Features:**
- **Executable vs. Total Coverage**: Distinguishes between comments/braces and executable code
- **Duplicate Detection**: Identifies Bug #27 patterns (`() {` and `-> $` duplicates)
- **Quality Classification**: EXCELLENT/GOOD/FAIR/POOR ratings
- **Gap Analysis**: Shows unmapped executable statements
- **Version Tracking**: Reports transpiler version for consistency

### Test Suite Integration: `source_map_test_integration.py`

**Location:** `/tools/source_map_test_integration.py`  
**Purpose:** Batch validation for entire test suites with VS Code extension reporting

```bash
# Validate test suite
python3 tools/source_map_test_integration.py --test-dir framec_tests/python/src

# Generate VS Code extension report
python3 tools/source_map_test_integration.py --report /tmp/quality_report.json

# Example quality report (JSON):
{
  "transpiler_version": "framec 0.79.0",
  "total_files": 371,
  "passed_files": 285,
  "pass_rate": 76.8,
  "quality_classification": "GOOD",
  "total_duplicates": 683,
  "average_coverage": 84.2
}
```

### Framework Integration: `test_framework_integration.py`

**Location:** `/tools/test_framework_integration.py`  
**Purpose:** Integration layer for existing Frame test framework and VS Code extension interface

```bash
# Console output for developers
python3 tools/test_framework_integration.py --mode console

# JSON output for VS Code extension
python3 tools/test_framework_integration.py --mode json

# VS Code extension status query
python3 tools/test_framework_integration.py --mode vscode-status
```

## VS Code Extension Integration

### Source of Truth Protocol

**Critical Design Principle:** The VS Code extension should **NOT** implement its own source map validation logic. Instead, it **relies on the transpiler team's validation tools** as the authoritative source.

### Extension Integration Commands

```typescript
// VS Code Extension Integration Examples

// Get current validation status
const status = await exec('python3 tools/test_framework_integration.py --mode vscode-status');

// Validate specific file during debugging  
const fileValidation = await exec('python3 tools/test_framework_integration.py --file', frameFile);

// Check if source maps meet quality standards
const qualityReport = JSON.parse(status.stdout);
const isExcellent = qualityReport.quality.classification === 'EXCELLENT';
```

### Expected VS Code Extension Workflow

```typescript
class FrameSourceMapValidator {
    async validateCurrentFile(frameFile: string): Promise<ValidationResult> {
        // Call transpiler team's source of truth validation
        const result = await this.callTranspilerValidation(frameFile);
        
        // Use standardized quality assessment
        return {
            isValid: result.passed,
            quality: result.analysis.quality_classification,
            coverage: result.analysis.executable_coverage,
            duplicates: result.analysis.duplicates.length,
            recommendations: result.analysis.recommendations
        };
    }
    
    async getOverallQuality(): Promise<QualityStatus> {
        // Get source of truth status from transpiler team
        const status = await this.getTranspilerQualityStatus();
        
        // Display quality in VS Code UI
        return {
            status: status.quality.classification,
            passRate: status.quality.pass_rate,
            lastValidation: status.last_validation,
            sourceOfTruth: true
        };
    }
}
```

### Quality Standards Configuration

**Location:** `/tools/source_map_config.json`

```json
{
  "quality_standards": {
    "minimum_executable_coverage": 95,
    "maximum_acceptable_duplicates": 5,
    "critical_duplicate_threshold": 10
  },
  "vscode_extension_interface": {
    "quality_thresholds": {
      "excellent": 95,
      "good": 90, 
      "fair": 80,
      "poor": 50
    }
  }
}
```

### Extension AI Integration

**Can the VS Code Extension AI run validation?** ✅ **YES - Recommended**

The extension AI should:

1. **Use Validation Tools**: Call the transpiler validation tools directly
2. **Interpret Results**: Analyze quality reports to make debugging recommendations  
3. **Provide Context**: Explain source map issues to users in plain language
4. **Suggest Fixes**: Recommend when to update transpiler or report bugs

**Example Extension AI Integration:**

```typescript
class FrameExtensionAI {
    async assessDebuggingExperience(frameFile: string): Promise<AIAssessment> {
        // Run transpiler validation
        const validation = await this.runTranspilerValidation(frameFile);
        
        // AI interprets the results
        if (validation.quality === 'EXCELLENT') {
            return {
                recommendation: "Perfect debugging experience expected",
                confidence: "high",
                actions: ["Set breakpoints anywhere", "Step-through will be accurate"]
            };
        } else if (validation.duplicates.length > 5) {
            return {
                recommendation: "Some duplicate mappings detected - stepping may be choppy",
                confidence: "medium", 
                actions: ["Focus breakpoints on main logic", "Avoid step-into for complex expressions"],
                bug_report: "Consider reporting Bug #27 patterns to transpiler team"
            };
        }
        
        // AI provides human-readable explanation
        return this.generateUserFriendlyAssessment(validation);
    }
    
    async suggestTranspilerUpdate(): Promise<AIRecommendation> {
        const currentQuality = await this.getSourceMapQuality();
        
        if (currentQuality.classification === 'POOR') {
            return {
                action: "update_transpiler",
                reason: "Source map quality below debugging standards",
                expected_improvement: "Better step-through and breakpoint accuracy"
            };
        }
        
        return { action: "no_update_needed" };
    }
}
```

### Benefits of Extension AI Integration

1. **Proactive Quality Assessment**: AI can warn users about debugging issues before they encounter them
2. **Contextual Recommendations**: AI explains technical validation results in user-friendly terms
3. **Automated Bug Detection**: AI can identify patterns and suggest when to report issues
4. **Version Guidance**: AI can recommend transpiler updates based on source map quality
5. **Learning System**: AI can learn from validation patterns to improve debugging guidance

## Source Map Flow

```
Frame AST Node
    ↓
Visitor Method (e.g., visit_state_node)
    ↓
Generate Python Code
    ↓
Track Line Numbers (current_line)
    ↓
Add Source Mapping
    ↓
SourceMapBuilder
    ↓
JSON Export (0-based)
    ↓
Debugger/VS Code Extension
```

## Line Tracking System

### Line Counter Management

1. **Initialization**: `current_line` starts at 1
2. **Newline Method**: 
   ```rust
   fn newline(&mut self) {
       self.code.push_str("\n");
       self.code.push_str(&self.dent());  // Add indentation
       self.current_line += 1;  // Increment after newline
   }
   ```
3. **Add Code Method**: Does NOT increment line counter
   ```rust
   fn add_code(&mut self, s: &str) {
       self.code.push_str(s);
       // No line increment - code goes on current line
   }
   ```

### Critical Timing Issues

The order of operations is critical for correct mappings:

```rust
// WRONG - maps to line after the code
self.newline();  // current_line becomes N+1
self.add_code("def foo():");  // Code placed on line N+1
self.add_source_mapping(frame_line);  // Maps to N+1 (correct)

// WRONG - maps to blank line before the code
self.newline();  // current_line becomes N+1
self.add_source_mapping(frame_line);  // Maps to N+1
self.add_code("def foo():");  // Code placed on line N+1
// Result: Mapping points to blank line, not the function def

// CORRECT - maps to the actual code line
self.newline();  // current_line becomes N+1
self.add_code("def foo():");  // Code placed on line N+1
self.add_source_mapping(frame_line);  // Maps to N+1 (where code is)
```

## Mapping Generation

### What Gets Mapped

#### Should Generate Mappings:
- **Event Handlers**: `$>() {` → `def __handle_state_enter(...)`
- **Statements**: `print(x)` → `print(x)`
- **Transitions**: `-> $State` → `_ChangeState(...)`
- **Function Definitions**: `fn foo() {` → `def foo():`
- **Action Calls**: `self.action()` → `self._action()`

#### Should NOT Generate Mappings:
- **State Declarations**: `$Running {` (structural, not executable)
- **Block Markers**: `interface:`, `machine:`, etc.
- **Comments**: Both Frame and generated Python comments
- **Blank Lines**: Visual spacing in generated code

### Common Patterns

#### Event Handler Generation
```rust
fn generate_event_handler_function(&mut self, evt_handler_node: &EventHandlerNode) {
    // Add visual spacing
    self.newline();
    
    // Generate function definition
    self.add_code(&format!("def {}(self, __e, compartment):", handler_name));
    
    // Map AFTER code is placed
    self.add_source_mapping(evt_handler_node.line);
    
    // Continue with function body
    self.indent();
    // ...
}
```

#### Statement Generation
```rust
fn visit_print_stmt_node(&mut self, print_stmt_node: &PrintStmtNode) {
    // Add mapping for the statement line
    self.add_source_mapping(print_stmt_node.line);
    
    // Generate the Python code
    self.add_code("print(");
    print_stmt_node.expr_t.accept(self);
    self.add_code(")");
}
```

## Quality Standards and Bug Detection

### Quality Classification System

**Introduced:** v0.79.0  
**Purpose:** Standardized quality assessment for consistent debugging experience

| Classification | Executable Coverage | Max Duplicates | Debugging Experience |
|----------------|-------------------|----------------|---------------------|
| **EXCELLENT**  | ≥95%              | ≤2             | Perfect breakpoints and step-through |
| **GOOD**       | ≥90%              | ≤5             | Reliable debugging with minor issues |
| **FAIR**       | ≥80%              | ≤10            | Acceptable for most debugging tasks |
| **POOR**       | <80%              | >10            | Significant debugging limitations |

### Bug #27: Duplicate Source Mappings (Active)

**Status:** Active (Minor - Functional but Suboptimal)  
**Pattern Detection:** Event handlers (`() {`) and state transitions (`-> $`)  
**Impact:** Debugger may stop multiple times on same Frame line

#### Detection Command:
```bash
python3 tools/source_map_validator.py test_file.frm
# Automatically detects Bug #27 patterns
```

#### Example Output:
```
=== DUPLICATE MAPPINGS ===
Frame 16 → 2 Python lines: [80, 84]
  ⚠️ Bug #27 pattern: start() {
Frame 18 → 2 Python lines: [82, 83]  
  ⚠️ Bug #27 pattern: -> $Running
📋 Bug #27 Impact: 2 event handler/transition duplicates detected
```

### Resolved Issues (v0.78.0+)

#### Bug #26: Missing Source Maps for Generated Code (RESOLVED ✅)
- **Status**: RESOLVED in v0.78.24
- **Solution**: Interface methods and system constructors now properly mapped
- **Coverage Improvement**: 55% → 100% executable statement coverage
- **Validation**: Confirmed via automated validation tools

#### Bug #25: Incorrect Control Flow Source Mapping (RESOLVED ✅)  
- **Status**: RESOLVED in v0.78.23
- **Solution**: Fixed while loop control flow mappings
- **Impact**: Debugger now correctly follows Frame line sequence

### Historical Issues (Pre-v0.78.0)
- Missing mappings for some statements

#### Root Cause:
The v0.73.0 fix introduced a regression where:
1. State dispatchers are generating blank lines without proper tracking
2. Event handler mappings are added BEFORE the function definition
3. Line counter gets out of sync with actual code placement

#### Example of the Problem:
```python
# Line 64
# Line 65 (blank)  ← Both Frame 27 and 28 incorrectly map here
def __handle_running_enter(self, __e, compartment):  # Line 66
    print("FirstSystem running")  # Line 67
    return  # Line 68
```

#### The Fix (v0.74.0):
```rust
// In generate_event_handler_function
fn generate_event_handler_function(&mut self, evt_handler_node: &EventHandlerNode) {
    // Add blank line for spacing
    self.newline();
    
    // Generate the function definition
    if handler_needs_async {
        self.add_code(&format!("async def {}(self, __e, compartment):", handler_name));
    } else {
        self.add_code(&format!("def {}(self, __e, compartment):", handler_name));
    }
    
    // Map AFTER the function def is placed on current_line
    self.add_source_mapping(evt_handler_node.line);
    
    // Continue with body
    self.indent();
    // ...
}
```

### Historical Issues

#### Bug #5: State Declaration Mapping (Fixed in v0.72.0)
- **Issue**: State declarations mapped to Python functions
- **Fix**: Removed `add_source_mapping(state_node.line)` from `visit_state_node()`

#### Bug #4: Off-by-One in Event Handlers (Fixed in v0.71.0)
- **Issue**: Event handlers mapped to line before function def
- **Fix**: Corrected timing of mapping addition relative to newline

#### Bug #1: Missing Event Handler Mapping (Fixed in v0.67.0)
- **Issue**: No mapping for event handler declarations
- **Fix**: Added `add_source_mapping(evt_handler_node.line)`

## Debugging Methodology

### 1. Generate Debug Output
```bash
framec -l python_3 --debug-output test.frm > debug.json
```

### 2. Extract Specific Mappings
```python
#!/usr/bin/env python3
import json, sys
data = json.load(sys.stdin)
for m in data['sourceMap']['mappings']:
    if 27 <= m['frameLine'] <= 31:
        print(f"Frame {m['frameLine']}: Python {m['pythonLine']}")
```

### 3. View Generated Python with Line Numbers
```bash
framec -l python_3 test.frm | awk 'NR==65,NR==70 {print NR ": " $0}'
```

### 4. Cross-Reference Analysis
Create a table comparing:
- Frame line number and content
- Mapped Python line according to source map
- Actual Python content at that line
- Expected Python line

### 5. Common Problem Indicators
- **Duplicate mappings**: Multiple Frame lines → same Python line
- **Blank line mappings**: Frame code → Python blank lines
- **Off-by-one**: Frame statements → wrong Python statements
- **Missing mappings**: No mapping for executable Frame code

## Implementation Details

### Critical Code Sections

#### 1. Event Handler Generation (`python_visitor.rs:490-520`)
```rust
fn generate_event_handler_function(&mut self, evt_handler_node: &EventHandlerNode, ...) {
    // Determine if async
    let handler_needs_async = evt_handler_node.is_async || self.system_has_async_runtime;
    
    // Add blank line for visual spacing
    self.newline();
    
    // Generate function definition
    if handler_needs_async {
        self.add_code(&format!("async def {}(self, __e, compartment):", handler_name));
    } else {
        self.add_code(&format!("def {}(self, __e, compartment):", handler_name));
    }
    
    // Add source mapping AFTER function is placed
    self.add_source_mapping(evt_handler_node.line);
    
    // Generate function body
    self.indent();
    // ...
}
```

#### 2. State Node Visitor (`python_visitor.rs:6380-6410`)
```rust
fn visit_state_node(&mut self, state_node: &StateNode) {
    // Generate comments if needed
    if self.generate_comment(state_node.line) {
        self.newline();
    }
    
    // v0.73: State declarations do NOT generate mappings
    // They are structural, not executable code
    
    // Set current state context
    self.current_state_name_opt = Some(state_node.name.clone());
    
    // Process state content...
}
```

#### 3. Source Map Builder (`source_map.rs:190-210`)
```rust
pub fn to_json(&self) -> String {
    // Convert 1-based line numbers to 0-based for debuggers
    let zero_based_mappings: Vec<SourceMapping> = self.mappings.iter().map(|m| {
        SourceMapping {
            frame_line: m.frame_line.saturating_sub(1),
            python_line: m.python_line.saturating_sub(1),
        }
    }).collect();
    
    // Generate JSON...
}
```

### Line Number Conversion

The transpiler works with 1-based line numbers internally (matching text editor display), but exports 0-based line numbers in the source map for debugger compatibility:

| Internal (1-based) | Source Map (0-based) | Display |
|-------------------|---------------------|---------|
| Line 1 | Line 0 | "Line 1" in editor |
| Line 28 | Line 27 | "Line 28" in editor |
| Line 66 | Line 65 | "Line 66" in editor |

## Testing Source Maps

### Test File Structure
```frame
# Lines 1-26: Padding/comments to push content to specific lines
system TestSystem {
    machine:
        $Running {                    # Line 28 - No mapping
            $>() {                    # Line 29 - Should map to function def
                print("Running")      # Line 30 - Should map to print
                return                # Line 31 - Should map to return
            }
        }
}
```

### Expected Mappings
```json
{
  "mappings": [
    { "frameLine": 28, "pythonLine": null },  // State declaration
    { "frameLine": 29, "pythonLine": 66 },    // Event handler → def
    { "frameLine": 30, "pythonLine": 67 },    // print → print
    { "frameLine": 31, "pythonLine": 68 }     // return → return
  ]
}
```

### Validation Script
```python
def validate_mappings(debug_json, expected):
    data = json.load(debug_json)
    mappings = {m['frameLine']: m['pythonLine'] 
                for m in data['sourceMap']['mappings']}
    
    for frame_line, expected_python in expected.items():
        actual = mappings.get(frame_line)
        if actual != expected_python:
            print(f"ERROR: Line {frame_line}: "
                  f"expected {expected_python}, got {actual}")
```

## Future Improvements

1. **Automatic Validation**: Add source map validation to test suite
2. **Visual Debugging**: Tool to visualize Frame→Python mappings
3. **Incremental Updates**: Track only changed mappings for faster compilation
4. **Multi-Target Support**: Extend to other target languages beyond Python
5. **Column Mapping**: Add column-level precision for more accurate debugging
6. **Source Map Spec**: Full compliance with Source Map v3 specification

## References

- [Source Map v3 Specification](https://sourcemaps.info/spec.html)
- [VS Code Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol/)
- [Frame Language Documentation](../frame_language.md)
- [Python Visitor Implementation](../../framec/src/frame_c/visitors/python_visitor.rs)
- [Source Map Builder](../../framec/src/frame_c/source_map.rs)
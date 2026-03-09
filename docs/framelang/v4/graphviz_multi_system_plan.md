# GraphViz Multi-System Generation Plan

## Overview
This document outlines the evolution plan for GraphViz generation in Frame, specifically for handling multi-system files and integration with the VSCode extension.

## Current State (v0.58)
- **Single System**: Generates standard GraphViz DOT → SVG perfectly
- **Multi-System**: Only generates the first system (prevents crashes but incomplete)
- **Issue**: `system_hierarchy` is built for one system at a time by semantic parser

## Phase 1: Basic Multi-System Support (Immediate)
**Goal**: Generate all systems without crashes

### Implementation
```rust
pub fn run_v2(&mut self, frame_module: &FrameModule) -> Vec<(String, String)> {
    let mut results = Vec::new();
    
    for system in &frame_module.systems {
        // Reset all visitor state between systems
        self.reset();
        
        // Generate DOT for this system
        system.accept(self);
        
        // Collect result
        results.push((system.name.clone(), self.code.clone()));
    }
    
    results  // Returns [(system_name, dot_code), ...]
}

fn reset(&mut self) {
    self.code.clear();
    self.states.clear();
    self.transitions.clear();
    self.current_state_name_opt = None;
    self.first_state_name.clear();
    self.system_name.clear();
    // Reset other fields...
}
```

### CLI Output
```bash
# For backward compatibility, concatenate with comments
framec -l graphviz multi_system.frm

# Output with system count header (per extension feedback):
// Frame Module: 2 systems
// System: TrafficLight
digraph TrafficLight { ... }

// System: Elevator  
digraph Elevator { ... }
```

## Phase 2: Structured SVG Output (Next)
**Goal**: Generate SVG that mirrors Frame code structure

### Single System (unchanged)
```xml
<!-- Standard GraphViz SVG output -->
<svg width="800" height="600">
    <g id="graph0">
        <!-- State machine diagram -->
    </g>
</svg>
```

### Multiple Systems (new structure)
```xml
<svg id="module-traffic-control" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <metadata>
            <frame:module name="traffic_control">
                <frame:systems>TrafficLight,Elevator,PedestrianCrossing</frame:systems>
            </frame:module>
        </metadata>
    </defs>
    
    <!-- Each system as nested SVG -->
    <svg id="system-TrafficLight" viewBox="0 0 800 600">
        <!-- Complete TrafficLight state machine -->
    </svg>
    
    <svg id="system-Elevator" viewBox="0 0 800 600">
        <!-- Complete Elevator state machine -->
    </svg>
    
    <svg id="system-PedestrianCrossing" viewBox="0 0 600 400">
        <!-- Complete PedestrianCrossing state machine -->
    </svg>
</svg>
```

### Benefits
1. **Structural alignment** with Frame code
2. **Self-contained** - all systems in one file
3. **Extractable** - VSCode can pull out individual systems
4. **Metadata** - systems list for navigation

## Phase 3: VSCode Extension Integration
**Goal**: Enable rich navigation in VSCode

### Extension Can Extract Systems
```javascript
// VSCode extension code
const svgDoc = parseSVG(frameSvg);

// Get list of systems from metadata
const systems = svgDoc.querySelector('frame\\:systems').textContent.split(',');

// Extract specific system for display
function showSystem(systemName) {
    const systemSvg = svgDoc.querySelector(`#system-${systemName}`);
    return systemSvg.outerHTML;
}

// Create navigation UI
function createSystemTabs(systems) {
    return systems.map(name => ({
        label: name,
        content: showSystem(name)
    }));
}
```

### Display Options for Extension

#### Option A: Tabs
```
[TrafficLight] [Elevator] [PedestrianCrossing]
+------------------------------------------------+
|                                                |
|         <Selected System Diagram>              |
|                                                |
+------------------------------------------------+
```

#### Option B: Dropdown
```
System: [TrafficLight ▼]
+------------------------------------------------+
|                                                |
|         <Selected System Diagram>              |
|                                                |
+------------------------------------------------+
```

#### Option C: Split View
```
+------------------------+------------------------+
|     TrafficLight       |       Elevator         |
|     <diagram>          |       <diagram>        |
+------------------------+------------------------+
```

## Phase 4: Multi-Module Support (Future)
**Goal**: Handle Frame projects with multiple modules

### Program-Level SVG
```xml
<svg id="program-main" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <metadata>
            <frame:program entry="main.frm">
                <frame:imports>
                    <frame:import module="traffic_control" href="./traffic_control.svg"/>
                    <frame:import module="sensors" href="./sensors.svg"/>
                </frame:imports>
            </frame:program>
        </metadata>
    </defs>
    
    <!-- Reference external module SVGs -->
    <image href="./traffic_control.svg" id="import-traffic-control"/>
    <image href="./sensors.svg" id="import-sensors"/>
    
    <!-- Local systems -->
    <svg id="system-MainController" viewBox="0 0 800 600">
        <!-- Main controller state machine -->
    </svg>
</svg>
```

## Implementation Priority

### Immediate (Phase 1)
1. Add `reset()` method to GraphVizVisitor
2. Fix `run_v2()` to generate all systems
3. Return array of (name, dot) tuples
4. Update CLI to handle multiple outputs

### Next Sprint (Phase 2)
1. Implement DOT to SVG conversion in visitor
2. Generate nested SVG structure for multi-system
3. Add metadata with system information
4. Maintain backward compatibility for single system

### Extension Team (Phase 3)
1. Parse structured SVG
2. Extract individual systems
3. Implement navigation UI (tabs/dropdown)
4. Handle both old and new formats

## API Contract

### GraphViz Visitor Output
```rust
pub enum GraphVizOutput {
    Single(String),           // Single system: standard SVG
    Multiple(Vec<SystemSvg>), // Multi-system: structured SVG
}

pub struct SystemSvg {
    pub name: String,
    pub svg: String,          // Complete SVG for this system
    pub viewBox: String,      // ViewBox dimensions
}
```

### CLI Flags
```bash
# Default: backward compatible
framec -l graphviz file.frm              # Concatenated DOT output

# New: structured output
framec -l graphviz --svg file.frm        # Generate SVG directly
framec -l graphviz --json file.frm       # JSON with system array

# VSCode mode
framec -l graphviz --vscode file.frm     # Optimized for extension
```

## Testing Strategy

### Test Cases
1. **Empty file** - No systems
2. **Single system** - Current behavior preserved
3. **Two systems** - Both generated correctly
4. **Many systems** (10+) - Performance acceptable
5. **Hierarchical states** - Parent-child relationships work
6. **Complex transitions** - All transition types handled

### Validation
- DOT syntax valid for each system
- SVG structure well-formed
- Metadata accurate
- IDs unique and valid
- ViewBox dimensions appropriate

## Migration Path

### For Users
1. **No breaking changes** - Single system files work identically
2. **Gradual adoption** - Multi-system features opt-in via flags
3. **Clear documentation** - Examples for each output format

### For Extension
1. **Feature detection** - Check SVG structure to determine format
2. **Fallback support** - Handle old format gracefully
3. **Progressive enhancement** - Add features as available

## Success Metrics
- All existing GraphViz tests pass
- Multi-system files generate without errors
- VSCode extension can display all systems
- Performance acceptable (< 1s for 10 systems)
- No regression in single-system generation

## Notes for VSCode Extension Team

### What You Get
- **Single System**: Standard GraphViz SVG (no changes needed)
- **Multi-System**: Structured SVG with nested systems
- **Metadata**: System list for navigation
- **Unique IDs**: Each system has `id="system-{name}"`

### What You Build
- Navigation UI (tabs, dropdown, etc.)
- System extraction logic
- Display panel management
- User preferences (default view, layout, etc.)

### We Handle
- GraphViz layout algorithms
- DOT generation
- SVG structure
- System metadata

### You Handle  
- UI/UX decisions
- Navigation controls
- Display preferences
- Multi-panel layouts

This separation keeps the transpiler focused on generation while giving the extension maximum flexibility for presentation.

response from vscode extension ai:

review /Users/marktruluck/vscode_editor/CLAUDE.md

⏺ Read(CLAUDE.md)
⎿  Read 94 lines (ctrl+r to expand)

⏺ The CLAUDE.md file is well-structured and comprehensive. Given
the new transpiler integration scripts I just created, here's
an updated version with the new automation features:

⏺ Update(CLAUDE.md)
⎿  (No changes)

> review /Users/marktruluck/projects/frame_transpiler/docs/graph
viz_multi_system_plan.md - i meant this one

⏺ Read(../projects/frame_transpiler/docs/graphviz_multi_system_p
lan.md)
⎿  Read 297 lines (ctrl+r to expand)

⏺ This is an excellent architectural plan for evolving GraphViz
generation to support multi-system Frame files. Here's my
review with key observations and recommendations:

🟢 Strengths

1. Phased Approach - Smart incremental evolution from basic
   support to full multi-module
2. Backward Compatibility - Preserves single-system behavior
   throughout
3. Clear API Contracts - Well-defined interfaces between
   transpiler and extension
4. Structured SVG Format - Clever use of nested SVGs with
   metadata for navigation

🔍 Key Insights for VS Code Extension

Immediate Actions Needed:

1. Update webview handler (handlers.ts:82-94) to detect
   multi-system SVG structure
2. Implement system navigation UI - The dropdown option
   (Option B) seems most space-efficient
3. Modify SVG rendering to handle nested structure

Suggested Implementation for Extension:

// In handlers.ts, detect multi-system SVG
function detectSvgStructure(svgContent: string): 'single' |
'multi' {
return svgContent.includes('frame:module') ? 'multi' :
'single';
}

// Extract systems from structured SVG
function extractSystems(svgContent: string): SystemInfo[] {
const parser = new DOMParser();
const doc = parser.parseFromString(svgContent,
'image/svg+xml');
const systemsMetadata =
doc.querySelector('frame\\:systems')?.textContent;

    if (!systemsMetadata) return [];

    return systemsMetadata.split(',').map(name => ({
      name: name.trim(),
      svg:
doc.querySelector(`#system-${name.trim()}`)?.outerHTML || ''
}));
}

📋 Recommendations

For Phase 1 (Immediate):

- ✅ The reset() approach is solid
- Consider adding a --format dot|svg|json flag instead of
  multiple flags
- Add system count to CLI output header for clarity

For Phase 2 (Structured SVG):

- Consider adding zoom level hints in metadata for optimal
  initial display
- Include state count per system for UI sizing decisions
- Add frame:version attribute for format versioning

For Extension Integration:

1. Progressive Enhancement: Start with simple concatenated
   display, add navigation later
2. User Preferences: Store last selected system per file
3. Performance: Consider lazy loading for files with many
   systems
4. Export Options: Allow exporting individual system SVGs

🚀 Quick Win Opportunities

1. Today: Update extension to handle concatenated DOT output
   (Phase 1)
2. This Week: Add basic system selector dropdown
3. Next Sprint: Implement full structured SVG support

⚠️ Potential Challenges

1. SVG Parsing: Browser DOM may struggle with custom
   namespaces (frame:*)
2. Performance: Large multi-system files might need
   virtualization
3. Layout: ViewBox coordination for consistent sizing across
   systems

💡 Enhancement Ideas

1. Inter-System Visualization: Show connections between
   systems in overview mode
2. Search: Find states/transitions across all systems
3. Diff View: Compare two systems side-by-side
4. Export Formats: PNG per system, combined PDF, Mermaid
   conversion

The plan is well-thought-out and provides a clear path
forward. The separation of concerns between transpiler
(generation) and extension (presentation) is particularly
good. The phased approach minimizes risk while delivering
value incrementally.
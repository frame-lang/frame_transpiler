> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 as a Native Preprocessing Tool

## Core Concept

Frame v4 becomes a **pure preprocessing tool** that integrates into native build toolchains. Instead of creating its own ecosystem, Frame simply:

1. **Reads** Frame source files (language-specific extensions: `.fpy`, `.frts`, `.frs`, etc.)
2. **Generates** native source files (`.py`, `.ts`, `.rs`, `.java`, etc.)
3. **Exits** - letting native toolchains handle everything else

## Integration per Language

### TypeScript/JavaScript Ecosystem

```json
// package.json
{
  "scripts": {
    "prebuild": "frame compile src/**/*.frts --out src/generated",
    "build": "tsc"
  },
  "devDependencies": {
    "@frame-lang/compiler": "^4.0.0",
    "typescript": "^5.0.0"
  }
}
```

**Options**:
- NPM package with CLI: `npx frame compile`
- Webpack loader: `frame-loader` 
- Vite plugin: `vite-plugin-frame`
- TypeScript transformer plugin
- SWC/Babel plugin for compilation

**Usage**:
```typescript
// traffic_light.frts compiles to traffic_light.ts
import { TrafficLight } from './generated/traffic_light'

const light = new TrafficLight()
light.start()
```

### Python Ecosystem

```toml
# pyproject.toml (PEP 517/518)
[build-system]
requires = ["setuptools", "frame-compiler"]
build-backend = "setuptools.build_meta"

[tool.frame]
source = "src/**/*.fpy"
output = "src/generated"
```

**Options**:
- PyPI package: `pip install frame-compiler`
- Setup.py integration: Custom build step
- Pre-commit hook for development
- Poetry plugin

**Usage**:
```python
# traffic_light.fpy compiles to traffic_light.py
from generated.traffic_light import TrafficLight

light = TrafficLight()
light.start()
```

### Rust Ecosystem

```toml
# Cargo.toml
[build-dependencies]
frame-compiler = "4.0"

[dependencies]
# Generated code just uses standard Rust
```

```rust
// build.rs
use frame_compiler::compile_frame_files;

fn main() {
    compile_frame_files("src/**/*.frs", "src/generated");
}
```

**Options**:
- Cargo subcommand: `cargo frame build`
- Build.rs integration (shown above)
- Procedural macro: `#[frame::system("traffic_light.frs")]`

**Usage**:
```rust
// src/main.rs
mod generated;
use generated::traffic_light::TrafficLight;

fn main() {
    let light = TrafficLight::new();
    light.start();
}
```

### Java Ecosystem

```xml
<!-- pom.xml (Maven) -->
<plugin>
    <groupId>org.frame-lang</groupId>
    <artifactId>frame-maven-plugin</artifactId>
    <version>4.0.0</version>
    <executions>
        <execution>
            <phase>generate-sources</phase>
            <goals><goal>compile</goal></goals>
        </execution>
    </executions>
</plugin>
```

```gradle
// build.gradle
plugins {
    id 'org.frame-lang.compiler' version '4.0.0'
}

frame {
    sourceDir = 'src/main/frame'
    outputDir = 'build/generated/sources/frame'
}
```

### C# Ecosystem

```xml
<!-- .csproj -->
<ItemGroup>
    <PackageReference Include="Frame.Compiler" Version="4.0.0" />
    <FrameFiles Include="**/*.frcs" />
</ItemGroup>

<Target Name="CompileFrame" BeforeTargets="CoreCompile">
    <Exec Command="frame compile @(FrameFiles) --out Generated" />
</Target>
```

### Go Ecosystem

```go
//go:generate frame compile traffic_light.fgo -o traffic_light.go
package main

import "./generated"

func main() {
    light := generated.NewTrafficLight()
    light.Start()
}
```

### C/C++ Ecosystem

```cmake
# CMakeLists.txt
find_program(FRAME frame)

# Custom command to generate C++ from Frame
add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/traffic_light.cpp
    COMMAND ${FRAME} compile ${CMAKE_CURRENT_SOURCE_DIR}/traffic_light.fcpp
    DEPENDS traffic_light.fcpp
)

add_executable(app main.cpp ${CMAKE_CURRENT_BINARY_DIR}/traffic_light.cpp)
```

## System Resolution Strategy

### Native Import Generation

When Frame systems reference other Frame systems, the compiler:

1. **Tracks** Frame system declarations and their output paths
2. **Resolves** Frame system references to native import paths
3. **Generates** appropriate native import statements

**Example**:
```frame
// controller.frts
@@target typescript

import TrafficLight from './traffic_light.frts'  // Frame import
import { Logger } from 'winston'                // Native import

@@system Controller {
    machine:
        $Active {
            init() {
                const light = new TrafficLight()
                light.start()
            }
        }
}
```

**Generates**:
```typescript
// generated/controller.ts
import { TrafficLight } from './traffic_light'  // Resolved Frame import
import { Logger } from 'winston'                // Passed through

export class Controller {
    // ... generated code
}
```

### Resolution Rules

1. **Frame imports** (language-specific extensions):
   - Resolve to generated file location
   - Convert to native import syntax
   - Use language-appropriate path separators

2. **Native imports** (no Frame extension):
   - Pass through unchanged
   - Already in native syntax

3. **Mixed projects**:
   - Frame files can import native modules
   - Native files can import generated Frame modules
   - Everything uses native module resolution

## Key Advantages

### 1. Zero Runtime Overhead
- No Frame runtime needed
- Generated code is pure native code
- Native optimizers work normally

### 2. Full Ecosystem Access
- Use any native package/library
- Publish Frame systems as native packages
- Standard debugging tools work

### 3. Simple Mental Model
- Frame is just a code generator
- Like TypeScript → JavaScript
- Like Sass → CSS

### 4. Standard Tooling
- IDEs already understand generated code
- Debuggers work out of the box
- Profilers, linters, formatters all work

### 5. Gradual Adoption
- Add Frame to existing projects easily
- Mix Frame and native code freely
- Migrate incrementally

## Implementation Simplifications

### What Frame NO LONGER Needs

1. **Module resolution system** - Use native
2. **Package format** - Use native packages
3. **Build system** - Use native build tools
4. **Project configuration** - Use native configs
5. **Runtime libraries** - Minimal or none
6. **Complex CLI** - Just compile command

### What Frame STILL Needs

1. **Parser** for Frame syntax
2. **Code generator** per target language
3. **System resolver** for Frame→Frame imports
4. **Source maps** for debugging (optional)

## CLI Design

Super simple CLI:

```bash
# Basic compilation
frame compile <input> --out <output> [--target <lang>]

# Watch mode
frame compile <input> --out <output> --watch

# Version
frame --version

# Help
frame --help
```

That's it! No complex project commands, no build system, no package management.

## File Extensions

Frame v4 uses language-specific file extensions for clarity:

- **Python**: `.fpy` → `.py`
- **TypeScript**: `.frts` → `.ts`
- **Rust**: `.frs` → `.rs`
- **Java**: `.fjava` → `.java`
- **C#**: `.frcs` → `.cs`
- **C**: `.fc` → `.c`
- **C++**: `.fcpp` → `.cpp`
- **Go**: `.fgo` → `.go`

Each extension clearly indicates the target language while maintaining the `@@target` pragma for explicit compilation intent.

## Example Workflow

### TypeScript Project

1. **Write** Frame systems:
```frame
// src/systems/traffic_light.frts
@@target typescript

@@system TrafficLight {
    machine:
        $Red {
            tick() { -> $Green() }
        }
        $Green {
            tick() { -> $Yellow() }
        }
        $Yellow {
            tick() { -> $Red() }
        }
}
```

2. **Add** build script:
```json
{
  "scripts": {
    "build:frame": "frame compile src/**/*.frts --out src/generated",
    "build": "npm run build:frame && tsc"
  }
}
```

3. **Import** and use:
```typescript
// src/app.ts
import { TrafficLight } from './generated/systems/traffic_light'

const light = new TrafficLight()
light.tick() // Red → Green
```

4. **Build** normally:
```bash
npm run build
```

## Migration Path from v3

1. **Phase 1**: Keep `framec` as is, add preprocessing mode
2. **Phase 2**: Deprecate complex features (project layer, etc.)
3. **Phase 3**: Slim down to pure preprocessor
4. **Phase 4**: Integrate with native tooling

## Open Questions

1. **Source maps**: Should Frame generate source maps for debugging?
2. **Watch mode**: Built-in or rely on native watchers?
3. **Incremental compilation**: Needed or compile all each time?
4. **Generated code style**: Match language conventions perfectly?
5. **Comments preservation**: Keep Frame comments in output?

## Conclusion

By becoming a pure preprocessing tool, Frame:
- **Simplifies** dramatically
- **Integrates** seamlessly with existing ecosystems
- **Leverages** native tooling and packages
- **Reduces** maintenance burden
- **Increases** adoption potential

Frame becomes the "TypeScript for state machines" - a familiar model that developers already understand.

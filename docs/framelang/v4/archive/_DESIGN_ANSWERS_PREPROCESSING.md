> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 Design Answers - Preprocessing Architecture

## Core Philosophy
**Frame is a preprocessing tool that generates native code and integrates into native toolchains.**

This fundamental decision simplifies many design questions by delegating to native ecosystems wherever possible.

## Answers Based on Preprocessing Philosophy

### 1. Project Architecture

#### Project Configuration
- **No frame.toml** - use native project files (package.json, Cargo.toml, etc.)
- Frame is just another build tool in the native toolchain
- Configuration goes in native build configs

#### Module System  
- **Frame imports**: Use `.frm` extension to indicate Frame imports
- **Resolution**: Simple relative paths, converted to native imports
- **Publishing**: Frame systems compile to native code, published as regular packages

#### Multiple Systems
- **Yes, supported** - each system generates a separate class/module in output
- **Export/access**: Use native export mechanisms in generated code

### 2. Language Features

#### Control Flow
- **Native only** - no Frame-specific `assert`, `loop`, `continue`
- Use native control flow everywhere
- Simpler parser, less to learn

#### State Machine Features
- **Transitions only** (`->`) - no separate change state operator
- **No history states initially** - can add later if needed
- **No guards initially** - use if statements in handlers
- **Start state**: First state in machine block

#### Constants and Enums
- **Native only** - declare in domain blocks using native syntax
- No Frame-level constructs needed

### 3. System Capabilities

#### Inheritance/Composition
- **No system inheritance** - only state inheritance
- Systems can instantiate other systems (composition over inheritance)

#### Generics
- **Not initially** - would complicate preprocessing significantly
- Could use native generics in generated code eventually

#### Access Modifiers
- **Simple model**: interface = public, actions = private
- No method-level modifiers (keeps it simple)

#### Async
- **Yes for actions/operations** - native async syntax
- **No for event handlers initially** - keeps state machine semantics simple

### 4. Runtime and Code Generation

#### Event Router
- **Minimal generation** - simple switch/match on state and event
- No complex runtime, just basic dispatch
- Predictable, debuggable generated code

#### Runtime Library
- **Minimal or none** - just generate self-contained native code
- Maybe thin helpers for state stack, but prefer inline generation

#### Source Maps
- **Optional** - nice to have but not required for MVP
- Native debugging of generated code is acceptable

#### Error Handling
- **Native exceptions** - handlers can throw
- Transitions abort on error (no partial state changes)
- No built-in error recovery (use native try/catch)

### 5. State Machine Semantics

#### Transitions
- **Best effort atomic** - but no transactions
- Exit handler runs, then transition, then enter handler
- Errors abort the transition

#### Events
- **Synchronous dispatch** - no event queue
- Events during transitions are programmer error
- Keep it simple

#### State Parameters
- **Live until next transition** - stored in state compartment
- Replaced on next transition to same state

#### Compartments
- **Created on entry, destroyed on exit**
- Simple lifetime model

### 6. Migration and Compatibility

#### Version Detection
- **@@target required** - explicit is better
- No auto-detection needed
- Clear error if missing

#### Backwards Compatibility
- **Clean break** - v4 is new product
- v3 continues to exist separately
- Migration guide but no compatibility mode

### 7. Implementation Strategy

#### Language Priority
- **PRT first** (Python, Rust, TypeScript)
- Others added based on demand
- Each language can have slightly different features

#### Rollout
- **New command**: `frame` (not `framec`)
- No feature flags - it's v4 or v3, not mixed
- Beta period with clear communication

### 8. Critical Design Decisions

#### Frame vs Native Balance
→ **Minimal Frame**: Only state machine structure, everything else native

#### Module System
→ **Native modules**: Frame just resolves .frm imports to generated paths

#### Runtime Generation  
→ **Minimal runtime**: Generate simple, readable, debuggable code

#### Project Structure
→ **Native structure**: Frame is just a preprocessor in the build pipeline

## Simplified CLI

Based on preprocessing architecture:

```bash
# Single command
frame compile <pattern> --out <dir> [--watch]

# Examples
frame compile src/**/*.frm --out src/generated
frame compile systems/*.frm --out build/
frame compile traffic_light.frm --out .

# That's it!
```

## What This Means for Implementation

### Dramatically Simplified

**Remove/Skip**:
- Project layer (Stage 13)
- Module resolution system
- Package management
- Complex CLI commands
- Runtime libraries (mostly)
- Version detection
- Compatibility layers

**Focus On**:
1. Clean Frame parser
2. Simple Frame→Frame import resolver
3. Quality code generators for PRT
4. Basic preprocessing CLI
5. Integration examples for each ecosystem

### Implementation Priority

1. **Core preprocessor** (parser + minimal CLI)
2. **Python generator** (simplest)
3. **TypeScript generator** (most common)
4. **Rust generator** (most complex)
5. **Documentation** (integration guides)
6. **Ecosystem integration** (npm package, pip package, etc.)

## Benefits of This Approach

1. **Vastly simpler** implementation and maintenance
2. **Familiar model** (like TypeScript, Sass, etc.)
3. **Zero learning curve** for tooling (use what you know)
4. **Full ecosystem access** immediately
5. **Easy adoption** in existing projects
6. **Better debugging** (it's just native code)
7. **No runtime overhead** or dependencies

## Next Steps

1. Validate this approach with key use cases
2. Prototype the preprocessor for one language
3. Test integration with native toolchain
4. Refine based on real usage
5. Build out other languages

This preprocessing approach transforms Frame from a complex language platform into a focused, valuable tool that fits naturally into existing development workflows.

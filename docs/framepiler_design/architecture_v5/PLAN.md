# Frame V5 Architecture Plan: Native Compiler Integration

**Status:** Future (Post V4 Completion)
**Prerequisite:** V4 stable as preprocessor

---

## Executive Summary

V5 extends Frame from a preprocessor to an **integrated validator** that understands both Frame and native code. By leveraging open source language parsers, V5 can cross-reference symbols between Frame declarations and native code usage, catching errors that V4 (and target compilers alone) cannot.

---

## Motivation

### The Gap V4 Leaves

V4 treats native code as opaque bytes. This means:

```python
@@system Counter {
    domain:
        var counter = 0

    machine:
        $Running {
            increment() {
                # V4 cannot validate these:
                couter += 1        # Typo - 'couter' not 'counter'
                self.doThing()     # No such action exists
                -> $Runing         # V4 catches this (Frame pattern)
            }
        }
}
```

- `couter` typo → Caught by Python at runtime (NameError)
- `self.doThing()` → Caught by Python at runtime (AttributeError)
- `-> $Runing` → Caught by V4 (Frame validates state names)

### What V5 Adds

V5 extracts symbols from native code and cross-references against Arcanum:

```
couter += 1
   ↓
Extract identifier 'couter'
   ↓
Check Arcanum: domain has 'counter', not 'couter'
   ↓
Warning: Unknown identifier 'couter' (did you mean 'counter'?)
```

---

## Architecture

### V4 Architecture (Preprocessor)

```
Source → Frame Parser → Arcanum
              ↓
       Native code (opaque bytes)
              ↓
       Pattern scan for ->, >>, $$
              ↓
       Splice + Emit
```

### V5 Architecture (Integrated Validator)

```
Source
   │
   ├──────────────────────────────────────┐
   ↓                                      ↓
Frame Parser                        Native Parser
   ↓                                      ↓
Arcanum                             Native Symbols
(Frame symbols)                     (extracted identifiers,
                                     calls, assignments)
   │                                      │
   └──────────────┬───────────────────────┘
                  ↓
         Cross-Environment Resolver
                  ↓
         ┌───────┴───────┐
         ↓               ↓
      Errors          Warnings
   (definite)      (likely typos)
                  ↓
            Codegen (same as V4)
```

---

## Native Parser Integration

### PRT Language Parsers

| Language | Parser | Integration Method | Maturity |
|----------|--------|-------------------|----------|
| Python | `ast` (stdlib) | Python subprocess or `rustpython-parser` | High |
| TypeScript | TypeScript Compiler API | Node subprocess or wasm | High |
| Rust | `syn` crate | Native Rust, direct integration | High |

### Integration Approaches

#### Option A: Subprocess Invocation

```rust
// Invoke Python to parse and extract symbols
fn extract_python_symbols(code: &str) -> Vec<Symbol> {
    let output = Command::new("python3")
        .args(["-c", EXTRACTOR_SCRIPT])
        .stdin(code)
        .output()?;
    parse_json_symbols(&output.stdout)
}
```

**Pros:** Simple, uses actual language parser
**Cons:** Requires language runtime installed, slower

#### Option B: Rust-Native Parsers

```rust
// Use Rust crates that parse target languages
fn extract_python_symbols(code: &str) -> Vec<Symbol> {
    let ast = rustpython_parser::parse(code)?;
    extract_identifiers(&ast)
}
```

**Pros:** Fast, no subprocess, single binary
**Cons:** May lag behind language versions, more dependencies

#### Option C: Hybrid (Recommended)

- **Default:** Rust-native parsers for speed
- **Fallback:** Subprocess for edge cases or newer syntax
- **Flag:** `--native-parser=subprocess` to force real parser

---

## Symbol Extraction

### What We Extract from Native Code

| Symbol Type | Example | Cross-Reference Against |
|-------------|---------|------------------------|
| Identifiers | `counter`, `x` | Arcanum domain vars |
| Method calls | `self.doThing()` | Arcanum actions/operations |
| Attribute access | `self.status` | Arcanum domain vars |
| String literals | `"Running"` in certain contexts | Arcanum state names |

### Extraction API

```rust
pub struct NativeSymbols {
    /// Identifiers used (variable references)
    pub identifiers: Vec<IdentifierRef>,

    /// Method calls on self
    pub self_calls: Vec<MethodCallRef>,

    /// Attribute access on self
    pub self_attributes: Vec<AttributeRef>,

    /// String literals (for state name detection)
    pub string_literals: Vec<StringLiteralRef>,
}

pub struct IdentifierRef {
    pub name: String,
    pub span: Span,
    pub context: IdentifierContext,  // Read, Write, Call
}

pub trait NativeExtractor {
    fn extract(&self, code: &str) -> Result<NativeSymbols, ExtractError>;
    fn language(&self) -> TargetLanguage;
}
```

---

## Cross-Environment Resolution

### Resolution Rules

```rust
pub struct CrossResolver {
    arcanum: Arcanum,
    native_symbols: NativeSymbols,
}

impl CrossResolver {
    pub fn resolve(&self) -> Vec<ResolutionIssue> {
        let mut issues = vec![];

        // Check identifiers against domain vars
        for ident in &self.native_symbols.identifiers {
            if !self.arcanum.has_domain_var(&ident.name)
               && !self.is_builtin(&ident.name)
               && !self.is_local(&ident) {
                issues.push(self.unknown_identifier(ident));
            }
        }

        // Check self.method() calls against actions/operations
        for call in &self.native_symbols.self_calls {
            if !self.arcanum.has_action(&call.method)
               && !self.arcanum.has_operation(&call.method)
               && !self.is_generated_method(&call.method) {
                issues.push(self.unknown_method(call));
            }
        }

        issues
    }

    fn unknown_identifier(&self, ident: &IdentifierRef) -> ResolutionIssue {
        // Fuzzy match for typo suggestions
        let suggestion = self.arcanum.fuzzy_match_domain(&ident.name);
        ResolutionIssue {
            code: "W501",
            severity: Warning,
            message: format!("Unknown identifier '{}'", ident.name),
            suggestion: suggestion.map(|s| format!("Did you mean '{}'?", s)),
            span: ident.span,
        }
    }
}
```

### Issue Severity

| Situation | Severity | Rationale |
|-----------|----------|-----------|
| Identifier not in domain, no close match | Warning | Could be local var we missed |
| Identifier not in domain, close match exists | Warning | Likely typo |
| Method call not in actions/operations | Warning | Could be inherited method |
| State name in string doesn't match | Info | Could be intentional |

We use warnings, not errors, because native code may have legitimate symbols we don't track (imports, locals, builtins).

---

## CLI Integration

```bash
# V4 behavior (default)
framec input.frm -l python_3 -o output.py

# V5 cross-validation (opt-in initially, default later)
framec input.frm -l python_3 -o output.py --cross-validate

# Deep validation with real native parser
framec input.frm -l python_3 -o output.py --cross-validate --native-parser=subprocess

# Suppress specific cross-validation warnings
framec input.frm -l python_3 --cross-validate --suppress=W501,W502
```

---

## IDE Integration (LSP)

V5's cross-resolution enables rich IDE features:

### Diagnostics

```
counter.py:15:17: warning[W501]: Unknown identifier 'couter'
   |
15 |     couter += 1
   |     ^^^^^^ did you mean 'counter'?
   |
   = note: 'counter' is declared in domain (line 5)
```

### Go to Definition

- Click `counter` in native code → Jump to `domain: var counter`
- Click `doThing()` in native code → Jump to `actions: doThing()`

### Autocomplete

- Type `self.` in handler → Suggest actions, operations, domain vars
- Type `-> $` → Suggest state names

### Hover Information

- Hover `counter` → "Domain variable, type: int, declared at line 5"

---

## Implementation Phases

### Phase 1: Python Extraction

1. Implement `PythonExtractor` using `rustpython-parser`
2. Extract identifiers, self calls, self attributes
3. Basic cross-reference against Arcanum
4. CLI `--cross-validate` flag

### Phase 2: Fuzzy Matching & Suggestions

1. Add Levenshtein distance for typo detection
2. Implement "did you mean?" suggestions
3. Tune warning thresholds

### Phase 3: TypeScript Extraction

1. Implement `TypeScriptExtractor` (likely subprocess)
2. Handle TS-specific patterns (this vs self)
3. Test with TS codebases

### Phase 4: Rust Extraction

1. Implement `RustExtractor` using `syn`
2. Handle Rust-specific patterns (self, &mut self)
3. Test with Rust codebases

### Phase 5: LSP Integration

1. Implement Frame Language Server
2. Diagnostics from cross-resolver
3. Go to definition across Frame/native boundary
4. Autocomplete in handlers

---

## Open Questions

### Architecture Questions

1. **Should cross-validation be opt-in or default?**
   - Opt-in initially for safety
   - Default once proven reliable

2. **How to handle native parse errors?**
   - Skip cross-validation for that handler?
   - Warn but continue?
   - Fail fast?

3. **Should we track local variables in native code?**
   - Needed to avoid false positives on local vars
   - Requires scope analysis in native parser

4. **What about imports in native code?**
   - `import math` then `math.sqrt()` is valid
   - Need to track import symbols too?

### Scope Questions

5. **Include type checking?**
   - Domain says `count: int`, native assigns `count = "hello"`
   - Requires type inference on native side
   - Probably V6 scope

6. **Include control flow analysis?**
   - Dead code detection in native handlers
   - Unreachable code after transition
   - Probably V6 scope

7. **Multi-file support?**
   - Native imports from other Frame-generated files
   - Cross-file symbol resolution
   - Probably V6 scope

### Priority Questions

8. **Which validations are highest value?**
   - Domain var typos?
   - Missing action calls?
   - State name in strings?

9. **What's acceptable false positive rate?**
   - 1%? 5%?
   - Users will disable if too noisy

---

## Dependencies

### Rust Crates (Potential)

| Crate | Purpose | Notes |
|-------|---------|-------|
| `rustpython-parser` | Parse Python | Active, good coverage |
| `swc_ecma_parser` | Parse TypeScript/JS | Fast, used by Next.js |
| `syn` | Parse Rust | Standard, mature |
| `strsim` | Fuzzy string matching | For typo detection |
| `tower-lsp` | LSP server framework | For IDE integration |

### External Tools (Subprocess Option)

| Tool | Purpose |
|------|---------|
| `python3` | Invoke Python ast module |
| `node` | Invoke TypeScript compiler API |
| (none for Rust) | syn is native |

---

## Success Criteria

1. **Catch real bugs:** At least 3 typo categories caught that V4 misses
2. **Low noise:** <5% false positive rate on real codebases
3. **Fast enough:** <2x slowdown vs V4 for typical files
4. **Opt-in safe:** No behavior change unless flag specified
5. **IDE ready:** LSP provides useful diagnostics

---

## Relationship to V4

| Aspect | V4 | V5 |
|--------|----|----|
| Native code | Opaque bytes | Parsed + extracted |
| Validation | Frame-only | Frame + cross-env |
| Dependencies | None | Native parsers |
| Speed | Fast | Moderate |
| Error coverage | Frame errors | Frame + native reference errors |
| Default | Yes (current) | Opt-in (future default) |

V5 is a superset of V4. All V4 tests must pass with `--cross-validate` disabled.

---

## Timeline Estimate

| Phase | Duration | Dependency |
|-------|----------|------------|
| V4 Complete | Current work | - |
| Phase 1: Python extraction | 2-3 weeks | V4 stable |
| Phase 2: Fuzzy matching | 1 week | Phase 1 |
| Phase 3: TypeScript | 2 weeks | Phase 2 |
| Phase 4: Rust | 1-2 weeks | Phase 3 |
| Phase 5: LSP | 3-4 weeks | Phase 4 |

**Total V5:** ~10-12 weeks after V4 completion

---

*Document created: February 2026*
*Status: Planning*

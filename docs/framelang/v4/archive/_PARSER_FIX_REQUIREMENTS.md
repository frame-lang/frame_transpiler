> **⚠️ DEPRECATED - DO NOT READ UNLESS INSTRUCTED ⚠️**
>
> This document is archived and may contain outdated or incorrect information about Frame syntax.
> For current Frame V4 syntax, see `frame_v4_lang_reference.md` in the parent directory.

---

# Frame v4 Parser Fix Requirements

## Problem Analysis

The v4 pure parser is failing because of fundamental tokenization and parsing issues. Here's what needs to be fixed:

## 1. Scanner Issues

### Problem: @@system Token Ambiguity
**Current Behavior**: 
- Scanner treats `@@system` as a single `FrameAnnotation` token
- Parser expects the keyword `system` as a `System` token

**Root Cause**:
- Frame v4 design has `@@system` serving dual purposes:
  1. As a Frame annotation (like `@@persist`)
  2. As the system declaration keyword

**Solution Options**:
1. **Option A**: Special-case `@@system` in scanner
   ```rust
   fn scan_annotation() {
       if self.peek() == Some('@') {
           self.advance();
           // Check if this is @@system
           let text = self.scan_identifier_text();
           if text == "system" {
               self.add_token(TokenType::System);
           } else {
               self.add_token(TokenType::FrameAnnotation);
           }
       }
   }
   ```

2. **Option B**: Make parser handle both forms
   ```rust
   fn parse() {
       // Accept either 'system' or '@@system' as FrameAnnotation
       if self.check(TokenType::System) || 
          (self.check(TokenType::FrameAnnotation) && 
           self.current_lexeme() == "@@system") {
           // Parse system
       }
   }
   ```

3. **Option C**: Redesign syntax (RECOMMENDED)
   - Use `@@system` only as annotation for persistence/metadata
   - Use plain `system` for declarations
   - This aligns with v3 compatibility

## 2. Parser Structure Issues

### Problem: Expecting Wrong Token Sequence
**Current Error**: "Expected keyword 'system', got Some("}") at 13:1"

**Root Cause**:
- Parser tries to parse native code after system block
- Doesn't recognize end of system properly

**Fix Required**:
```rust
fn parse() {
    // Parse system
    let system = self.parse_system()?;
    
    // Parse trailing native code if present
    let trailing_code = if !self.is_at_end() {
        self.parse_native_code_block()
    } else {
        None
    };
}
```

## 3. Native Code Handling

### Problem: Native Code in Handlers Not Captured
**Current Issue**: Handler bodies are scanned but MIR assembly fails

**Root Cause**:
- `parse_handler_body()` collects tokens but doesn't preserve formatting
- Native scanner expects properly formatted code

**Fix Required**:
```rust
fn parse_handler_body() -> MirBlock {
    // Capture the EXACT source text, not reconstructed from tokens
    let start_pos = self.current_source_position();
    let depth = 1;
    
    // Skip to matching brace, tracking source position
    while depth > 0 && !self.is_at_end() {
        if self.check(LeftBrace) { depth += 1; }
        if self.check(RightBrace) { depth -= 1; }
        self.advance();
    }
    
    let end_pos = self.current_source_position();
    let native_code = &self.source[start_pos..end_pos];
    
    // Now scan for MIR
    let scanner = get_native_scanner(self.target);
    scanner.scan_native_block(native_code)
}
```

## 4. State Recognition

### Problem: States Not Properly Identified
**Issue**: `$StateName` tokens not correctly identified in contexts

**Fix Required**:
- Scanner needs context-aware state recognition
- Can't just look for `$` followed by identifier
- Need to distinguish from `$>` (enter) and `$<` (exit)

```rust
fn scan_frame_construct() {
    if self.peek() == Some('>') {
        self.advance();
        self.add_token(TokenType::Enter);
    } else if self.peek() == Some('<') {
        self.advance();
        self.add_token(TokenType::Exit);
    } else if self.peek() == Some('$') {
        // Handle $$[+] or $$[-]
        self.scan_stack_op();
    } else {
        // State reference
        self.scan_state_name();
    }
}
```

## 5. Complete Implementation Path

### Phase 1: Fix Scanner (High Priority)
1. [ ] Fix `@@system` tokenization
2. [ ] Preserve source positions for all tokens
3. [ ] Handle state references correctly
4. [ ] Improve native code capture

### Phase 2: Fix Parser (High Priority)
1. [ ] Handle both `system` and `@@system` forms
2. [ ] Fix handler body parsing to preserve source
3. [ ] Parse trailing native code properly
4. [ ] Better error recovery

### Phase 3: Fix MIR Assembly (Medium Priority)
1. [ ] Improve native scanner for each language
2. [ ] Better Frame construct detection
3. [ ] Preserve indentation and formatting
4. [ ] Handle nested constructs

### Phase 4: Integration (Low Priority)
1. [ ] Connect parser AST to code generator
2. [ ] Implement proper symbol resolution
3. [ ] Add source map generation
4. [ ] Complete error reporting

## Recommended Approach

Given the complexity and the fact that v3 backend provides 80% functionality:

1. **Short Term**: Continue using v3 backend (current approach)
   - Production ready for Python/TypeScript
   - Proven and tested
   - Minimal work required

2. **Medium Term**: Fix critical v4 parser issues
   - Focus on scanner tokenization fixes
   - Get basic system parsing working
   - Incremental improvements

3. **Long Term**: Complete v4 pure implementation
   - Full MIR support
   - Remove runtime dependencies
   - Native-first code generation

## Effort Estimate

- **Scanner fixes**: 2-3 days
- **Parser fixes**: 3-5 days  
- **MIR assembly**: 5-7 days
- **Full integration**: 10-15 days

**Total**: ~20-30 days for complete v4 pure implementation

## Alternative: Hybrid Approach

Instead of fixing the pure v4 implementation, enhance the v3 adapter:

1. Use v3's proven scanner/parser
2. Transform AST to v4 format
3. Apply v4-specific transformations
4. Generate cleaner code without runtime

This would give us v4 benefits with less risk and effort.

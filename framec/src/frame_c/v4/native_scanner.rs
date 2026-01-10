// Frame v4 Native Scanner - Scans native code for Frame constructs
// Language-aware scanning that respects native syntax while finding Frame statements

use super::mir::{MirBlock, MirItem, Region, RegionSpan, SymbolTable};
use super::TargetLanguage;

/// Scanner for finding Frame constructs in native code
pub trait NativeScanner {
    /// Scan native code block for Frame constructs and symbols
    fn scan_native_block(&self, native_code: &str) -> Result<MirBlock, String>;
}

/// Get the appropriate scanner for a target language
pub fn get_scanner(target: TargetLanguage) -> Box<dyn NativeScanner> {
    match target {
        TargetLanguage::Python => Box::new(PythonScanner),
        TargetLanguage::TypeScript => Box::new(TypeScriptScanner),
        TargetLanguage::Rust => Box::new(RustScanner),
        _ => Box::new(GenericScanner), // Fallback scanner
    }
}

/// Python-specific scanner
struct PythonScanner;

impl NativeScanner for PythonScanner {
    fn scan_native_block(&self, native_code: &str) -> Result<MirBlock, String> {
        let mut block = MirBlock::new();
        let bytes = native_code.as_bytes();
        let mut i = 0;
        let end = bytes.len();
        let mut segment_start = 0;
        let mut at_line_start = true;
        let mut indent = 0;
        
        while i < end {
            let b = bytes[i];
            
            // Track line starts and indentation
            if at_line_start {
                if b == b' ' || b == b'\t' {
                    indent += 1;
                    i += 1;
                    continue;
                }
                
                // Check for Frame constructs at start of line
                
                // Transition: -> $State
                if b == b'-' && i + 3 < end && bytes[i + 1] == b'>' && bytes[i + 2] == b' ' && bytes[i + 3] == b'$' {
                    // Save native text before this Frame construct
                    if segment_start < i - indent {
                        let content = std::str::from_utf8(&bytes[segment_start..i - indent])
                            .map_err(|e| format!("UTF-8 error: {}", e))?
                            .to_string();
                        block.regions.push(Region::NativeText {
                            content,
                            span: RegionSpan { start: segment_start, end: i - indent },
                        });
                    }
                    
                    // Parse the transition
                    let trans_start = i;
                    i += 4; // Skip "-> $"
                    
                    // Get state name
                    let mut state_name = String::new();
                    while i < end && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                        state_name.push(bytes[i] as char);
                        i += 1;
                    }
                    
                    // Check for arguments
                    let mut state_args = Vec::new();
                    if i < end && bytes[i] == b'(' {
                        i += 1; // Skip '('
                        let mut arg = String::new();
                        while i < end && bytes[i] != b')' {
                            if bytes[i] == b',' {
                                if !arg.trim().is_empty() {
                                    state_args.push(arg.trim().to_string());
                                }
                                arg.clear();
                            } else {
                                arg.push(bytes[i] as char);
                            }
                            i += 1;
                        }
                        if !arg.trim().is_empty() {
                            state_args.push(arg.trim().to_string());
                        }
                        if i < end && bytes[i] == b')' {
                            i += 1;
                        }
                    }
                    
                    // Skip to end of line
                    while i < end && bytes[i] != b'\n' {
                        i += 1;
                    }
                    
                    block.regions.push(Region::FrameSegment {
                        item: MirItem::Transition {
                            target: state_name,
                            exit_args: Vec::new(),
                            enter_args: Vec::new(),
                            state_args,
                            span: RegionSpan { start: trans_start, end: i },
                        },
                        span: RegionSpan { start: trans_start, end: i },
                        indent,
                    });
                    
                    segment_start = i;
                    at_line_start = false;
                    indent = 0;
                    continue;
                }
                
                // Forward: => $^
                if b == b'=' && i + 3 < end && bytes[i + 1] == b'>' && bytes[i + 2] == b' ' && bytes[i + 3] == b'$' {
                    if i + 4 < end && bytes[i + 4] == b'^' {
                        // Save native text before
                        if segment_start < i - indent {
                            let content = std::str::from_utf8(&bytes[segment_start..i - indent])
                                .map_err(|e| format!("UTF-8 error: {}", e))?
                                .to_string();
                            block.regions.push(Region::NativeText {
                                content,
                                span: RegionSpan { start: segment_start, end: i - indent },
                            });
                        }
                        
                        let forward_start = i;
                        i += 5; // Skip "=> $^"
                        
                        // Skip to end of line
                        while i < end && bytes[i] != b'\n' {
                            i += 1;
                        }
                        
                        block.regions.push(Region::FrameSegment {
                            item: MirItem::Forward {
                                span: RegionSpan { start: forward_start, end: i },
                            },
                            span: RegionSpan { start: forward_start, end: i },
                            indent,
                        });
                        
                        segment_start = i;
                        at_line_start = false;
                        indent = 0;
                        continue;
                    }
                }
                
                // Stack operations: $$[+] or $$[-]
                if b == b'$' && i + 3 < end && bytes[i + 1] == b'$' && bytes[i + 2] == b'[' {
                    let is_push = i + 3 < end && bytes[i + 3] == b'+';
                    let is_pop = i + 3 < end && bytes[i + 3] == b'-';
                    
                    if (is_push || is_pop) && i + 4 < end && bytes[i + 4] == b']' {
                        // Save native text before
                        if segment_start < i - indent {
                            let content = std::str::from_utf8(&bytes[segment_start..i - indent])
                                .map_err(|e| format!("UTF-8 error: {}", e))?
                                .to_string();
                            block.regions.push(Region::NativeText {
                                content,
                                span: RegionSpan { start: segment_start, end: i - indent },
                            });
                        }
                        
                        let stack_start = i;
                        i += 5; // Skip "$$[+]" or "$$[-]"
                        
                        let item = if is_push {
                            MirItem::StackPush {
                                span: RegionSpan { start: stack_start, end: i },
                            }
                        } else {
                            MirItem::StackPop {
                                span: RegionSpan { start: stack_start, end: i },
                            }
                        };
                        
                        block.regions.push(Region::FrameSegment {
                            item,
                            span: RegionSpan { start: stack_start, end: i },
                            indent,
                        });
                        
                        segment_start = i;
                        at_line_start = false;
                        indent = 0;
                        continue;
                    }
                }
                
                at_line_start = false;
                indent = 0;
            }
            
            // Handle Python-specific syntax
            match b {
                b'\n' => {
                    at_line_start = true;
                    indent = 0;
                    i += 1;
                }
                b'#' => {
                    // Skip comment
                    while i < end && bytes[i] != b'\n' {
                        i += 1;
                    }
                }
                b'\'' | b'"' => {
                    // Skip string literal
                    let quote = b;
                    let mut is_triple = false;
                    if i + 2 < end && bytes[i + 1] == quote && bytes[i + 2] == quote {
                        is_triple = true;
                        i += 3;
                    } else {
                        i += 1;
                    }
                    
                    while i < end {
                        if is_triple {
                            if i + 2 < end && bytes[i] == quote && bytes[i + 1] == quote && bytes[i + 2] == quote {
                                i += 3;
                                break;
                            }
                        } else {
                            if bytes[i] == b'\\' && i + 1 < end {
                                i += 2;
                                continue;
                            }
                            if bytes[i] == quote {
                                i += 1;
                                break;
                            }
                        }
                        i += 1;
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        
        // Add any remaining native text
        if segment_start < end {
            let content = std::str::from_utf8(&bytes[segment_start..end])
                .map_err(|e| format!("UTF-8 error: {}", e))?
                .to_string();
            if !content.trim().is_empty() {
                block.regions.push(Region::NativeText {
                    content,
                    span: RegionSpan { start: segment_start, end },
                });
            }
        }
        
        // TODO: Scan for symbol definitions and usages
        // This would involve lightweight parsing to find variable assignments,
        // parameter references, self.domain_var accesses, etc.
        
        Ok(block)
    }
}

/// TypeScript scanner
struct TypeScriptScanner;

impl NativeScanner for TypeScriptScanner {
    fn scan_native_block(&self, native_code: &str) -> Result<MirBlock, String> {
        // TODO: Implement TypeScript-specific scanning
        // Handle: //, /* */, template literals, etc.
        GenericScanner.scan_native_block(native_code)
    }
}

/// Rust scanner
struct RustScanner;

impl NativeScanner for RustScanner {
    fn scan_native_block(&self, native_code: &str) -> Result<MirBlock, String> {
        // TODO: Implement Rust-specific scanning
        // Handle: //, /* */, raw strings, etc.
        GenericScanner.scan_native_block(native_code)
    }
}

/// Generic fallback scanner
struct GenericScanner;

impl NativeScanner for GenericScanner {
    fn scan_native_block(&self, native_code: &str) -> Result<MirBlock, String> {
        // Simple scanner that just looks for Frame patterns
        // May have false positives in strings/comments
        let mut block = MirBlock::new();
        
        // For now, treat entire block as native text
        // A real implementation would scan for Frame patterns
        block.regions.push(Region::NativeText {
            content: native_code.to_string(),
            span: RegionSpan { start: 0, end: native_code.len() },
        });
        
        Ok(block)
    }
}
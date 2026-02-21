use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::typescript::BodyCloserTs;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerTs;

/// TypeScript syntax skipper - handles //, /* */, strings, and template literals
struct TypeScriptSkipper;

impl SyntaxSkipper for TypeScriptSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserTs)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        if let Some(j) = skip_line_comment(bytes, i, end) {
            return Some(j);
        }
        skip_block_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Template literals
        if let Some(j) = skip_template_literal(bytes, i, end) {
            return Some(j);
        }
        // Simple strings
        skip_simple_string(bytes, i, end)
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        // TypeScript uses C-like line ending (semicolons, // comments)
        // But also needs to handle template literals
        let mut j = start;
        let mut in_string: Option<u8> = None;
        let mut in_template = false;
        let mut template_brace_depth = 0i32;

        while j < end {
            let b = bytes[j];

            if b == b'\n' {
                break;
            }

            // Inside template literal
            if in_template {
                if b == b'`' && template_brace_depth == 0 {
                    in_template = false;
                    j += 1;
                    continue;
                }
                if b == b'\\' {
                    j += 2;
                    continue;
                }
                if b == b'$' && j + 1 < end && bytes[j + 1] == b'{' {
                    template_brace_depth += 1;
                    j += 2;
                    continue;
                }
                if b == b'}' && template_brace_depth > 0 {
                    template_brace_depth -= 1;
                }
                j += 1;
                continue;
            }

            // Inside regular string
            if let Some(q) = in_string {
                if b == b'\\' {
                    j += 2;
                    continue;
                }
                if b == q {
                    in_string = None;
                }
                j += 1;
                continue;
            }

            // Statement terminators
            if b == b';' {
                break;
            }
            if b == b'/' && j + 1 < end && (bytes[j + 1] == b'/' || bytes[j + 1] == b'*') {
                break;
            }

            // String/template starts
            if b == b'\'' || b == b'"' {
                in_string = Some(b);
                j += 1;
                continue;
            }
            if b == b'`' {
                in_template = true;
                j += 1;
                continue;
            }

            j += 1;
        }
        j
    }

    fn balanced_paren_end(&self, bytes: &[u8], mut i: usize, end: usize) -> Option<usize> {
        if i >= end || bytes[i] != b'(' {
            return None;
        }

        let mut depth = 0i32;
        let mut in_string: Option<u8> = None;
        let mut in_template = false;

        while i < end {
            let b = bytes[i];

            // Inside template literal
            if in_template {
                if b == b'`' {
                    in_template = false;
                }
                if b == b'\\' {
                    i += 2;
                    continue;
                }
                i += 1;
                continue;
            }

            // Inside string
            if let Some(q) = in_string {
                if b == b'\\' {
                    i += 2;
                    continue;
                }
                if b == q {
                    in_string = None;
                }
                i += 1;
                continue;
            }

            match b {
                b'\'' | b'"' => {
                    in_string = Some(b);
                    i += 1;
                }
                b'`' => {
                    in_template = true;
                    i += 1;
                }
                b'(' => {
                    depth += 1;
                    i += 1;
                }
                b')' => {
                    depth -= 1;
                    i += 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        None
    }
}

impl NativeRegionScanner for NativeRegionScannerTs {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        scan_native_regions(&TypeScriptSkipper, bytes, open_brace_index)
    }
}

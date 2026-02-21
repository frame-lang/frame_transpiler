use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::rust::BodyCloserRust;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerRust;

/// Rust syntax skipper - handles //, /* */, strings, and raw strings
struct RustSkipper;

impl SyntaxSkipper for RustSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserRust)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        if let Some(j) = skip_line_comment(bytes, i, end) {
            return Some(j);
        }
        // Rust has nested block comments
        skip_rust_block_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Try raw string first
        if let Some(j) = skip_rust_raw_string(bytes, i, end) {
            return Some(j);
        }
        // Simple strings
        skip_simple_string(bytes, i, end)
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        find_line_end_rust(bytes, start, end)
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        balanced_paren_end_rust(bytes, i, end)
    }
}

/// Skip Rust nested block comment: /* ... /* ... */ ... */
fn skip_rust_block_comment(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if i + 1 >= end || bytes[i] != b'/' || bytes[i + 1] != b'*' {
        return None;
    }

    let mut j = i + 2;
    let mut depth = 1i32;

    while j + 1 < end && depth > 0 {
        if bytes[j] == b'/' && bytes[j + 1] == b'*' {
            depth += 1;
            j += 2;
            continue;
        }
        if bytes[j] == b'*' && bytes[j + 1] == b'/' {
            depth -= 1;
            j += 2;
            continue;
        }
        j += 1;
    }
    Some(j)
}

/// Find line end for Rust, handling raw strings
fn find_line_end_rust(bytes: &[u8], mut j: usize, end: usize) -> usize {
    let mut in_string: Option<u8> = None;
    let mut raw_hashes = 0usize;

    while j < end {
        let b = bytes[j];

        if b == b'\n' {
            break;
        }

        // Inside raw string
        if raw_hashes > 0 {
            if b == b'"' {
                let mut k = j + 1;
                let mut matched = 0usize;
                while k < end && matched < raw_hashes && bytes[k] == b'#' {
                    matched += 1;
                    k += 1;
                }
                if matched == raw_hashes {
                    j = k;
                    raw_hashes = 0;
                    continue;
                }
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

        // String starts
        if b == b'\'' || b == b'"' {
            in_string = Some(b);
            j += 1;
            continue;
        }

        // Raw string start
        if b == b'r' {
            let mut k = j + 1;
            let mut hashes = 0usize;
            while k < end && bytes[k] == b'#' {
                hashes += 1;
                k += 1;
            }
            if k < end && bytes[k] == b'"' {
                raw_hashes = hashes;
                j = k + 1;
                continue;
            }
        }

        j += 1;
    }
    j
}

/// Find balanced paren for Rust, handling raw strings
fn balanced_paren_end_rust(bytes: &[u8], mut i: usize, end: usize) -> Option<usize> {
    if i >= end || bytes[i] != b'(' {
        return None;
    }

    let mut depth = 0i32;
    let mut in_string: Option<u8> = None;

    while i < end {
        let b = bytes[i];

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

impl NativeRegionScanner for NativeRegionScannerRust {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        scan_native_regions(&RustSkipper, bytes, open_brace_index)
    }
}

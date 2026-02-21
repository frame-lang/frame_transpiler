use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::csharp::BodyCloserCs;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerCs;

/// C# syntax skipper - handles //, /* */, preprocessor #, strings, verbatim @"", interpolated $""
struct CSharpSkipper;

impl SyntaxSkipper for CSharpSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserCs)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Preprocessor directive
        if bytes[i] == b'#' {
            let mut j = i + 1;
            while j < end && bytes[j] != b'\n' {
                j += 1;
            }
            return Some(j);
        }
        if let Some(j) = skip_line_comment(bytes, i, end) {
            return Some(j);
        }
        skip_block_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        let b = bytes[i];

        // Verbatim string @"..." (doubled quotes for escape)
        if b == b'@' && i + 1 < end && bytes[i + 1] == b'"' {
            return skip_verbatim_string(bytes, i, end);
        }

        // Interpolated string $"..." or $@"..." or raw $"""..."""
        if b == b'$' {
            return skip_csharp_interpolated(bytes, i, end);
        }

        // Simple strings
        skip_simple_string(bytes, i, end)
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        find_line_end_c_like(bytes, start, end)
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        balanced_paren_end_c_like(bytes, i, end)
    }
}

/// Skip C# verbatim string @"..." where "" is escaped quote
fn skip_verbatim_string(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if i + 1 >= end || bytes[i] != b'@' || bytes[i + 1] != b'"' {
        return None;
    }

    let mut j = i + 2;
    while j < end {
        if bytes[j] == b'"' {
            if j + 1 < end && bytes[j + 1] == b'"' {
                j += 2; // Escaped quote
                continue;
            }
            return Some(j + 1);
        }
        j += 1;
    }
    Some(end)
}

/// Skip C# interpolated strings: $"...", $@"...", or raw $"""..."""
fn skip_csharp_interpolated(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if bytes[i] != b'$' {
        return None;
    }

    let mut j = i + 1;

    // Skip additional $ for raw strings
    while j < end && bytes[j] == b'$' {
        j += 1;
    }

    // Check for @
    if j < end && bytes[j] == b'@' {
        j += 1;
    }

    // Count opening quotes
    let mut quotes = 0;
    while j < end && bytes[j] == b'"' {
        quotes += 1;
        j += 1;
    }

    if quotes == 0 {
        return None;
    }

    // Raw string (3+ quotes)
    if quotes >= 3 {
        while j < end {
            if bytes[j] == b'"' {
                let mut q = 0;
                let mut p = j;
                while p < end && bytes[p] == b'"' {
                    q += 1;
                    p += 1;
                }
                if q >= quotes {
                    return Some(p);
                }
                j = p;
                continue;
            }
            j += 1;
        }
        return Some(end);
    }

    // Normal interpolated string
    while j < end {
        if bytes[j] == b'\\' {
            j += 2;
            continue;
        }
        if bytes[j] == b'"' {
            return Some(j + 1);
        }
        j += 1;
    }
    Some(end)
}

impl NativeRegionScanner for NativeRegionScannerCs {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        scan_native_regions(&CSharpSkipper, bytes, open_brace_index)
    }
}

use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::cpp::BodyCloserCpp;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerCpp;

/// C++ syntax skipper - handles //, /* */, strings, and raw strings R"delim(...)delim"
struct CppSkipper;

impl SyntaxSkipper for CppSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserCpp)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        if let Some(j) = skip_line_comment(bytes, i, end) {
            return Some(j);
        }
        skip_block_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Try C++ raw string R"delim(...)delim"
        if let Some(j) = skip_cpp_raw_string(bytes, i, end) {
            return Some(j);
        }
        skip_simple_string(bytes, i, end)
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        find_line_end_c_like(bytes, start, end)
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        balanced_paren_end_c_like(bytes, i, end)
    }
}

/// Skip C++ raw string: R"delim(...)delim"
fn skip_cpp_raw_string(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if i + 1 >= end || bytes[i] != b'R' || bytes[i + 1] != b'"' {
        return None;
    }

    // Parse delimiter
    let mut j = i + 2;
    let mut delim = Vec::new();
    while j < end && bytes[j] != b'(' {
        delim.push(bytes[j]);
        j += 1;
        if delim.len() > 32 {
            return None; // Invalid delimiter
        }
    }

    if j >= end || bytes[j] != b'(' {
        return None;
    }
    j += 1; // Skip '('

    // Find closing )delim"
    while j < end {
        if bytes[j] == b')' {
            let mut k = j + 1;
            let mut m = 0usize;
            while m < delim.len() && k < end && bytes[k] == delim[m] {
                k += 1;
                m += 1;
            }
            if m == delim.len() && k < end && bytes[k] == b'"' {
                return Some(k + 1);
            }
        }
        j += 1;
    }
    Some(end) // Unterminated
}

impl NativeRegionScanner for NativeRegionScannerCpp {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        scan_native_regions(&CppSkipper, bytes, open_brace_index)
    }
}

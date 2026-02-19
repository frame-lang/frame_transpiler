use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::java::BodyCloserJavaV3;
use crate::frame_c::v4::body_closer::BodyCloserV3;

pub struct NativeRegionScannerJavaV3;

/// Java syntax skipper - handles //, /* */, and strings (same as C)
struct JavaSkipper;

impl SyntaxSkipper for JavaSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloserV3> {
        Box::new(BodyCloserJavaV3)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        if let Some(j) = skip_line_comment(bytes, i, end) {
            return Some(j);
        }
        skip_block_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Java 15+ text blocks """..."""
        if let Some(j) = skip_java_text_block(bytes, i, end) {
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

/// Skip Java text block """..."""
fn skip_java_text_block(bytes: &[u8], i: usize, end: usize) -> Option<usize> {
    if i + 2 >= end || bytes[i] != b'"' || bytes[i + 1] != b'"' || bytes[i + 2] != b'"' {
        return None;
    }

    let mut j = i + 3;
    while j + 2 < end {
        if bytes[j] == b'"' && bytes[j + 1] == b'"' && bytes[j + 2] == b'"' {
            return Some(j + 3);
        }
        j += 1;
    }
    Some(end)
}

impl NativeRegionScannerV3 for NativeRegionScannerJavaV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        scan_native_regions(&JavaSkipper, bytes, open_brace_index)
    }
}

use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::python::BodyCloserPy;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerPy;

/// Python syntax skipper - handles # comments, strings, and triple-quoted strings
struct PythonSkipper;

impl SyntaxSkipper for PythonSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserPy)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        skip_hash_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        // Try triple-quoted first (must check before simple string)
        if let Some(j) = skip_triple_string(bytes, i, end) {
            return Some(j);
        }
        skip_simple_string(bytes, i, end)
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        find_line_end_python(bytes, start, end)
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        balanced_paren_end_c_like(bytes, i, end)
    }
}

impl NativeRegionScanner for NativeRegionScannerPy {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        scan_native_regions(&PythonSkipper, bytes, open_brace_index)
    }
}

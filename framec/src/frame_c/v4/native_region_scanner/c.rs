use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::c::BodyCloserCV3;
use crate::frame_c::v4::body_closer::BodyCloserV3;

pub struct NativeRegionScannerCV3;

/// C syntax skipper - handles //, /* */, and strings
struct CSkipper;

impl SyntaxSkipper for CSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloserV3> {
        Box::new(BodyCloserCV3)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        if let Some(j) = skip_line_comment(bytes, i, end) {
            return Some(j);
        }
        skip_block_comment(bytes, i, end)
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        skip_simple_string(bytes, i, end)
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        find_line_end_c_like(bytes, start, end)
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        balanced_paren_end_c_like(bytes, i, end)
    }
}

impl NativeRegionScannerV3 for NativeRegionScannerCV3 {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResultV3, ScanErrorV3> {
        scan_native_regions(&CSkipper, bytes, open_brace_index)
    }
}

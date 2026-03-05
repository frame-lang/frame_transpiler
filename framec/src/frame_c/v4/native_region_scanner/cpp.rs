// C++ syntax skipper — Frame-generated state machine.
//
// Source: cpp_skipper.frs (Frame specification)
// Generated: cpp_skipper.gen.rs (via framec compile -l rust)
// This file: glue module wiring generated FSM to SyntaxSkipper trait
//
// To regenerate:
//   ./target/release/framec compile -l rust -o /tmp framec/src/frame_c/v4/native_region_scanner/cpp_skipper.frs
//   cp /tmp/cpp_skipper.rs framec/src/frame_c/v4/native_region_scanner/cpp_skipper.gen.rs

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("cpp_skipper.gen.rs");

use super::*;
use super::unified::SyntaxSkipper;
use crate::frame_c::v4::body_closer::cpp::BodyCloserCpp;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerCpp;

/// C++ syntax skipper - handles //, /* */, strings, and raw strings R"delim(...)delim"
pub struct CppSkipper;

impl SyntaxSkipper for CppSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserCpp)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        let mut fsm = CppSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = i;
        fsm.end = end;
        fsm.do_skip_comment();
        if fsm.success != 0 { Some(fsm.result_pos) } else { None }
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        let mut fsm = CppSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = i;
        fsm.end = end;
        fsm.do_skip_string();
        if fsm.success != 0 { Some(fsm.result_pos) } else { None }
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        let mut fsm = CppSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = start;
        fsm.end = end;
        fsm.do_find_line_end();
        fsm.result_pos
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        let mut fsm = CppSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = i;
        fsm.end = end;
        fsm.do_balanced_paren_end();
        if fsm.success != 0 { Some(fsm.result_pos) } else { None }
    }
}

impl NativeRegionScanner for NativeRegionScannerCpp {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        super::unified::scan_native_regions(&CppSkipper, bytes, open_brace_index)
    }
}

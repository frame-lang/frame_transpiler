// Python syntax skipper — Frame-generated state machine.
//
// Source: python_skipper.frs (Frame specification)
// Generated: python_skipper.gen.rs (via framec compile -l rust)
// This file: glue module wiring generated FSM to SyntaxSkipper trait
//
// To regenerate:
//   ./target/release/framec compile -l rust -o /tmp framec/src/frame_c/v4/native_region_scanner/python_skipper.frs
//   cp /tmp/python_skipper.rs framec/src/frame_c/v4/native_region_scanner/python_skipper.gen.rs

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("python_skipper.gen.rs");

use super::*;
use super::unified::*;
use crate::frame_c::v4::body_closer::python::BodyCloserPy;
use crate::frame_c::v4::body_closer::BodyCloser;

pub struct NativeRegionScannerPy;

/// Python syntax skipper - handles # comments, triple-quoted strings, and simple strings
pub struct PythonSkipper;

impl SyntaxSkipper for PythonSkipper {
    fn body_closer(&self) -> Box<dyn BodyCloser> {
        Box::new(BodyCloserPy)
    }

    fn skip_comment(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        let mut fsm = PythonSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = i;
        fsm.end = end;
        fsm.do_skip_comment();
        if fsm.success != 0 { Some(fsm.result_pos) } else { None }
    }

    fn skip_string(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        let mut fsm = PythonSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = i;
        fsm.end = end;
        fsm.do_skip_string();
        if fsm.success != 0 { Some(fsm.result_pos) } else { None }
    }

    fn find_line_end(&self, bytes: &[u8], start: usize, end: usize) -> usize {
        let mut fsm = PythonSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = start;
        fsm.end = end;
        fsm.do_find_line_end();
        fsm.result_pos
    }

    fn balanced_paren_end(&self, bytes: &[u8], i: usize, end: usize) -> Option<usize> {
        let mut fsm = PythonSyntaxSkipperFsm::new();
        fsm.bytes = bytes[..end].to_vec();
        fsm.pos = i;
        fsm.end = end;
        fsm.do_balanced_paren_end();
        if fsm.success != 0 { Some(fsm.result_pos) } else { None }
    }
}

impl NativeRegionScanner for NativeRegionScannerPy {
    fn scan(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<ScanResult, ScanError> {
        super::unified::scan_native_regions(&PythonSkipper, bytes, open_brace_index)
    }
}

// C++ import scanner — Frame-generated state machine.
//
// Source: cpp_import.frs (Frame specification)
// Generated: cpp_import.gen.rs (via framec compile -l rust)
// This file: glue module wiring generated FSM to ImportScanner trait

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("cpp_import.gen.rs");

use super::*;

pub struct ImportScannerCpp;

impl ImportScanner for ImportScannerCpp {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResult {
        let mut fsm = CppImportScannerFsm::new();
        fsm.bytes = bytes.to_vec();
        fsm.start = start;
        fsm.do_scan();
        ImportScanResult { spans: fsm.spans, issues: fsm.issues }
    }
}

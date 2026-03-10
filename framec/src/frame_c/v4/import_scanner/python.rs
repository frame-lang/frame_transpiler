// Python import scanner — Frame-generated state machine.
//
// Source: python_import.frs (Frame specification)
// Generated: python_import.gen.rs (via framec compile -l rust)
// This file: glue module wiring generated FSM to ImportScanner trait

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("python_import.gen.rs");

use super::*;

pub struct ImportScannerPy;

impl ImportScanner for ImportScannerPy {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResult {
        let mut fsm = PythonImportScannerFsm::new();
        fsm.bytes = bytes.to_vec();
        fsm.start = start;
        fsm.do_scan();
        ImportScanResult { spans: fsm.spans, issues: fsm.issues }
    }
}

// C import scanner — Frame-generated state machine.
//
// Source: c_import.frs (Frame specification)
// Generated: c_import.gen.rs (via framec compile -l rust)
// This file: glue module wiring generated FSM to ImportScanner trait
//
// To regenerate:
//   ./target/release/framec compile -l rust -o /tmp framec/src/frame_c/v4/import_scanner/c_import.frs
//   cp /tmp/c_import.rs framec/src/frame_c/v4/import_scanner/c_import.gen.rs

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("c_import.gen.rs");

use super::*;

pub struct ImportScannerC;

impl ImportScanner for ImportScannerC {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResult {
        let mut fsm = CImportScannerFsm::new();
        fsm.bytes = bytes.to_vec();
        fsm.start = start;
        fsm.do_scan();
        ImportScanResult { spans: fsm.spans, issues: fsm.issues }
    }
}

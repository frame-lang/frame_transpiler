// TypeScript import scanner — Frame-generated state machine.
//
// Source: typescript_import.frs (Frame specification)
// Generated: typescript_import.gen.rs (via framec compile -l rust)
// This file: glue module wiring generated FSM to ImportScanner trait

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("typescript_import.gen.rs");

use super::*;

pub struct ImportScannerTs;

impl ImportScanner for ImportScannerTs {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResult {
        let mut fsm = TypeScriptImportScannerFsm::new();
        fsm.bytes = bytes.to_vec();
        fsm.start = start;
        fsm.do_scan();
        ImportScanResult { spans: fsm.spans, issues: fsm.issues }
    }
}

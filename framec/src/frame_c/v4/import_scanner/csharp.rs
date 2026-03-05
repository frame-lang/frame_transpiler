// C# import scanner — Frame-generated state machine.
//
// Source: csharp_import.frs (Frame specification)
// Generated: csharp_import.gen.rs (via framec compile -l rust)
// This file: glue module wiring generated FSM to ImportScanner trait

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("csharp_import.gen.rs");

use super::*;

pub struct ImportScannerCs;

impl ImportScanner for ImportScannerCs {
    fn scan(&self, bytes: &[u8], start: usize) -> ImportScanResult {
        let mut fsm = CSharpImportScannerFsm::new();
        fsm.bytes = bytes.to_vec();
        fsm.start = start;
        fsm.do_scan();
        ImportScanResult { spans: fsm.spans, issues: fsm.issues }
    }
}

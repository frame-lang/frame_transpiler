// Body closer for C++ language — Frame-generated state machine.
//
// Source: cpp.frs (Frame specification)
// Generated: cpp.gen.rs (via framec --target rust)
// This file: glue module wiring generated FSM to BodyCloser trait
//
// To regenerate:
//   ./target/release/framec framec/src/frame_c/v4/body_closer/cpp.frs -l rust > framec/src/frame_c/v4/body_closer/cpp.gen.rs

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("cpp.gen.rs");

use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserCpp;

impl BodyCloser for BodyCloserCpp {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut fsm = CppBodyCloserFsm::new();
        fsm.bytes = bytes.to_vec();
        fsm.pos = open_brace_index + 1;
        fsm.depth = 1;
        fsm.scan();
        match fsm.error_kind {
            0 => Ok(fsm.result_pos),
            1 => Err(CloseError { kind: CloseErrorKind::UnterminatedString, message: fsm.error_msg }),
            2 => Err(CloseError { kind: CloseErrorKind::UnterminatedComment, message: fsm.error_msg }),
            4 => Err(CloseError { kind: CloseErrorKind::UnterminatedRawString, message: fsm.error_msg }),
            _ => Err(CloseError { kind: CloseErrorKind::UnmatchedBraces, message: fsm.error_msg }),
        }
    }
}

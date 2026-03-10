// Body closer for C# language — Frame-generated state machine.
//
// Source: csharp.frs (Frame specification)
// Generated: csharp.gen.rs (via framec --target rust)
// This file: glue module wiring generated FSM to BodyCloser trait
//
// To regenerate:
//   ./target/release/framec framec/src/frame_c/v4/body_closer/csharp.frs -l rust > framec/src/frame_c/v4/body_closer/csharp.gen.rs

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("csharp.gen.rs");

use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserCs;

impl BodyCloser for BodyCloserCs {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut fsm = CsBodyCloserFsm::new();
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

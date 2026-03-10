// Body closer for TypeScript language — Frame-generated state machine.
//
// Source: typescript.frs (Frame specification)
// Generated: typescript.gen.rs (via framec --target rust)
// This file: glue module wiring generated FSM to BodyCloser trait
//
// To regenerate:
//   ./target/release/framec framec/src/frame_c/v4/body_closer/typescript.frs -l rust > framec/src/frame_c/v4/body_closer/typescript.gen.rs

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("typescript.gen.rs");

use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserTs;

impl BodyCloser for BodyCloserTs {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut fsm = TsBodyCloserFsm::new();
        fsm.bytes = bytes.to_vec();
        fsm.pos = open_brace_index + 1;
        fsm.depth = 1;
        fsm.scan();
        match fsm.error_kind {
            0 => Ok(fsm.result_pos),
            1 => Err(CloseError { kind: CloseErrorKind::UnterminatedString, message: fsm.error_msg }),
            2 => Err(CloseError { kind: CloseErrorKind::UnterminatedComment, message: fsm.error_msg }),
            _ => Err(CloseError { kind: CloseErrorKind::UnmatchedBraces, message: fsm.error_msg }),
        }
    }
}

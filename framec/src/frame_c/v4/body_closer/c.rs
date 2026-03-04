// Body closer for C language — Frame-generated state machine.
//
// Source: c.frs (Frame specification)
// Generated: c.gen.rs (via framec --target rust)
// This file: glue module wiring generated FSM to BodyCloser trait
//
// To regenerate:
//   ./target/release/framec framec/src/frame_c/v4/body_closer/c.frs -l rust > framec/src/frame_c/v4/body_closer/c.gen.rs
//   (then apply manual fixes for known Rust backend issues — see c.gen.rs comments)

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("c.gen.rs");

use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserC;

impl BodyCloser for BodyCloserC {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut fsm = CBodyCloserFsm::new();
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

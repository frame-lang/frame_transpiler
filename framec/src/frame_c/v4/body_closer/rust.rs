// Body closer for Rust language — Frame-generated state machine.
//
// Source: rust_lang.frs (Frame specification)
// Generated: rust_lang.gen.rs (via framec --target rust)
// This file: glue module wiring generated FSM to BodyCloser trait
//
// To regenerate:
//   ./target/release/framec framec/src/frame_c/v4/body_closer/rust_lang.frs -l rust > framec/src/frame_c/v4/body_closer/rust_lang.gen.rs

#![allow(unreachable_patterns)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

include!("rust_lang.gen.rs");

use super::{BodyCloser, CloseError, CloseErrorKind};

pub struct BodyCloserRust;

impl BodyCloser for BodyCloserRust {
    fn close_byte(&mut self, bytes: &[u8], open_brace_index: usize) -> Result<usize, CloseError> {
        let mut fsm = RustBodyCloserFsm::new();
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

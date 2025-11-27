// Glue module for self-hosted V3 machines.
//
// The IndentNormalizer machine is generated from
// `framec/src/frame_c/v3/machines/indent_normalizer.frs` into
// `indent_normalizer.gen.rs` via `tools/gen_v3_machines_rs.py` using the
// bootstrap compiler. This module wires the generated struct into a small
// helper function that the main V3 pipeline can call.

// Bring the generated IndentNormalizer into this module's scope.
include!("indent_normalizer.gen.rs");

use std::string::String;
use std::vec::Vec;

/// Run the IndentNormalizer machine over a handler body described by
/// `lines` + flags and return the normalized Python lines.
pub fn run_indent_normalizer(
    lines: &[String],
    flags_is_expansion: &[bool],
    flags_is_comment: &[bool],
    pad: &str,
) -> Vec<String> {
    let mut m = IndentNormalizer::new();
    m.lines = lines.to_vec();
    m.flags_is_expansion = flags_is_expansion.to_vec();
    m.flags_is_comment = flags_is_comment.to_vec();
    m.pad = pad.to_string();
    m.run();
    m.out_lines
}


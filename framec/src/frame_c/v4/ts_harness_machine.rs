// TypeScript harness builder machine glue.
//
// This module wires the generated `TsHarnessBuilder` machine (compiled from
// `framec/src/frame_c/v3/machines/ts_harness_builder.frs`) into a small
// helper function that mirrors the behavior of
// `_execute_ts_harness_from_spliced` in the Python test runner.
//
// Machine-generated code can trigger lints like `unreachable_patterns` and
// `dead_code` that are not actionable here, so relax them at the module level.
#![allow(unreachable_patterns)]
#![allow(dead_code)]

include!("machines/ts_harness_builder.gen.rs");

use std::string::String;
use std::vec::Vec;

/// Build a minimal TypeScript harness program from a spliced V3 output.
///
/// This helper:
///   - splits the spliced output into lines,
///   - forwards them to the `TsHarnessBuilder` Frame machine, and
///   - returns the assembled harness program string.
pub fn run_ts_harness_builder(spliced_output: &str) -> String {
    let lines: Vec<String> = spliced_output
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut m = TsHarnessBuilder::new();
    m.lines = lines;
    m.run();
    m.out_program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ts_harness_builder_matches_runner_shape() {
        let spliced = "__frame_transition(\"__S_state_A\", null)\n// comment\n__frame_forward()\n";
        let program = run_ts_harness_builder(spliced);
        let expected = concat!(
            "function __frame_transition(state: string, ...args: any[]) {}\n",
            "function __frame_forward() {}\n",
            "function __frame_stack_push() {}\n",
            "function __frame_stack_pop() {}\n",
            "function main() {\n",
            "__frame_transition(\"__S_state_A\", null);\n",
            "__frame_forward();\n",
            "}\n",
            "main();\n",
        );
        assert_eq!(program, expected);
    }
}

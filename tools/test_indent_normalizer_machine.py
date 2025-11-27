#!/usr/bin/env python3
"""
Stage 14 IndentNormalizer harness (Phase A).

This script exercises the `IndentNormalizer` Frame system defined in:
  framec_tests/language_specific/rust/v3_internal/indent_normalizer.frs

It compiles the .frs to Rust via the V3 pipeline, builds a tiny Rust
binary that calls `IndentNormalizer::new().run()`, captures stdout, and
asserts that the normalized lines match the expected output for the
hard‑coded sample handler body used in the machine's $ComputeBase state.

This is an internal harness and is not wired into the main test runner;
it can be invoked manually or from CI as needed.
"""

import os
import subprocess
import sys
import tempfile
from pathlib import Path


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    framec = os.environ.get("FRAMEC_BIN", str(root / "target" / "release" / "framec"))

    frs = root / "framec_tests" / "language_specific" / "rust" / "v3_internal" / "indent_normalizer.frs"
    if not frs.is_file():
        print(f"ERROR: IndentNormalizer FRM not found at {frs}", file=sys.stderr)
        return 1

    outdir = Path(tempfile.mkdtemp(prefix="indent_norm_stage14_"))

    # Compile the Frame machine to Rust.
    try:
        res = subprocess.run(
            [framec, "compile", "-l", "rust", str(frs), "-o", str(outdir)],
            capture_output=True,
            text=True,
            check=True,
        )
    except subprocess.CalledProcessError as e:
        print("ERROR: framec compile failed", file=sys.stderr)
        print(e.stdout, file=sys.stderr)
        print(e.stderr, file=sys.stderr)
        return 1

    rs_path = outdir / "indent_normalizer.rs"
    if not rs_path.is_file():
        print(f"ERROR: Expected generated Rust file at {rs_path}", file=sys.stderr)
        return 1

    # Build a small Rust harness that includes the generated module and runs it.
    main_rs = outdir / "main.rs"
    main_rs.write_text(
        'include!("indent_normalizer.rs");\n\n'
        "fn main() {\n"
        "    let mut s = IndentNormalizer::new();\n"
        "    s.run();\n"
        "}\n",
        encoding="utf-8",
    )

    bin_path = outdir / "indent_norm_bin"
    try:
        subprocess.run(
            ["rustc", "main.rs", "-O", "-o", str(bin_path)],
            cwd=str(outdir),
            capture_output=True,
            text=True,
            check=True,
        )
    except subprocess.CalledProcessError as e:
        print("ERROR: rustc failed for IndentNormalizer harness", file=sys.stderr)
        print(e.stdout, file=sys.stderr)
        print(e.stderr, file=sys.stderr)
        return 1

    # Run the harness and capture normalized lines.
    res = subprocess.run(
        [str(bin_path)],
        capture_output=True,
        text=True,
        check=True,
    )
    lines = res.stdout.splitlines()

    # Expected normalized output for the hard-coded sample handler body
    # in $ComputeBase.run() of indent_normalizer.frs.
    expected = [
        "        if self.stopOnEntry:",
        "            # Skip stop on entry if user continues",
        "            next_compartment = FrameCompartment(\"__S_state_Waiting\")",
        "            self._frame_transition(next_compartment)",
        "            return",
        "        ",
    ]

    if lines != expected:
        print("ERROR: IndentNormalizer output mismatch", file=sys.stderr)
        print("Expected:", repr(expected), file=sys.stderr)
        print("Actual  :", repr(lines), file=sys.stderr)
        return 1

    print("IndentNormalizer Stage 14 harness OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

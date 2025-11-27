#!/usr/bin/env python3
"""
Stage 14 machine precompile helper (V3).

This script regenerates Rust sources from self-hosted V3 machines defined
in Frame (`.frs`), using a pinned bootstrap compiler. It does NOT run as
part of `cargo build`; it is an explicit precompile step.

Current scope:
  - framec/src/frame_c/v3/machines/indent_normalizer.frs

Bootstrap compiler policy (see PLAN.md, Stage 14 Phase B):
  - The repo MUST contain a single bootstrap compiler at:
        boot/framec/framec
    or be provided via FRAMEC_BOOTSTRAP_BIN.
  - This script uses that binary exclusively to regenerate machine-
    generated Rust.
"""

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


def find_bootstrap() -> Path:
    """Resolve the bootstrap compiler path."""
    override = os.environ.get("FRAMEC_BOOTSTRAP_BIN")
    if override:
        return Path(override)
    root = Path(__file__).resolve().parents[1]
    return root / "boot" / "framec" / "framec"


def regen_indent_normalizer_rs(bootstrap: Path, repo_root: Path) -> None:
    """Regenerate Rust for the Stage 14 IndentNormalizer machine."""
    frs = (
        repo_root
        / "framec"
        / "src"
        / "frame_c"
        / "v3"
        / "machines"
        / "indent_normalizer.frs"
    )
    if not frs.is_file():
        raise SystemExit(f"ERROR: IndentNormalizer FRM not found at {frs}")

    out_rs = (
        repo_root
        / "framec"
        / "src"
        / "frame_c"
        / "v3"
        / "machines"
        / "indent_normalizer.gen.rs"
    )

    tmpdir = Path(tempfile.mkdtemp(prefix="v3_machines_indent_norm_"))
    try:
        # Compile the FRM machine to Rust using the bootstrap compiler.
        cmd = [str(bootstrap), "compile", "-l", "rust", str(frs), "-o", str(tmpdir)]
        res = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
        )
        if res.returncode != 0:
            sys.stderr.write("ERROR: framec bootstrap compile failed\n")
            sys.stderr.write(res.stdout or "")
            sys.stderr.write(res.stderr or "")
            raise SystemExit(res.returncode)

        generated = tmpdir / "indent_normalizer.rs"
        if not generated.is_file():
            raise SystemExit(
                f"ERROR: Expected generated Rust file at {generated}"
            )

        # Copy with a small header noting that this file is generated.
        header = (
            "// NOTE: This file is generated from "
            "framec/src/frame_c/v3/machines/indent_normalizer.frs\n"
            "// via tools/gen_v3_machines_rs.py using the bootstrap compiler.\n"
            "// Do not edit directly.\n\n"
        )
        text = generated.read_text(encoding="utf-8")
        out_rs.write_text(header + text, encoding="utf-8")
        print(f"Regenerated {out_rs} using {bootstrap}")
    finally:
        try:
            shutil.rmtree(tmpdir)
        except Exception:
            pass


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    bootstrap = find_bootstrap()
    if not bootstrap.is_file():
        sys.stderr.write(
            f"ERROR: bootstrap compiler not found at {bootstrap}\n"
            "Set FRAMEC_BOOTSTRAP_BIN or place a binary at boot/framec/framec.\n"
        )
        return 1

    regen_indent_normalizer_rs(bootstrap, repo_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())


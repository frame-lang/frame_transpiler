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


def _compile_machine(
    bootstrap: Path,
    repo_root: Path,
    frs_rel: Path,
    expected_rs_name: str,
    out_rel: Path,
    tmp_prefix: str,
) -> None:
    """Shared helper: compile a Frame machine to Rust using the bootstrap."""
    frs = repo_root / frs_rel
    if not frs.is_file():
        raise SystemExit(f"ERROR: FRM machine not found at {frs}")

    out_rs = repo_root / out_rel

    tmpdir = Path(tempfile.mkdtemp(prefix=tmp_prefix))
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

        generated = tmpdir / expected_rs_name
        if not generated.is_file():
            raise SystemExit(
                f"ERROR: Expected generated Rust file at {generated}"
            )

        # Copy with a small header noting that this file is generated.
        # Lints for machine-generated code are relaxed at the module level in
        # `framec/src/frame_c/v3/machines/mod.rs`.
        header = (
            f"// NOTE: This file is generated from {frs_rel.as_posix()}\n"
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


def regen_indent_normalizer_rs(bootstrap: Path, repo_root: Path) -> None:
    """Regenerate Rust for the Stage 14 IndentNormalizer machine."""
    _compile_machine(
        bootstrap=bootstrap,
        repo_root=repo_root,
        frs_rel=Path("framec/src/frame_c/v3/machines/indent_normalizer.frs"),
        expected_rs_name="indent_normalizer.rs",
        out_rel=Path("framec/src/frame_c/v3/machines/indent_normalizer.gen.rs"),
        tmp_prefix="v3_machines_indent_norm_",
    )


def regen_ts_harness_builder_rs(bootstrap: Path, repo_root: Path) -> None:
    """Regenerate Rust for the Stage 16 TypeScript harness builder machine."""
    _compile_machine(
        bootstrap=bootstrap,
        repo_root=repo_root,
        frs_rel=Path("framec/src/frame_c/v3/machines/ts_harness_builder.frs"),
        expected_rs_name="ts_harness_builder.rs",
        out_rel=Path("framec/src/frame_c/v3/machines/ts_harness_builder.gen.rs"),
        tmp_prefix="v3_machines_ts_harness_",
    )


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
    regen_ts_harness_builder_rs(bootstrap, repo_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

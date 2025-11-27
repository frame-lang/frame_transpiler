#!/usr/bin/env python3
"""
Publish a framec release binary to:
  - boot/framec/framec (bootstrap compiler used for FRM precompile), and
  - the shared test env reference path, if present:
        ../framepiler_test_env/bug/releases/frame_transpiler/v<version>/framec/framec

This script is intended to be run after:
  1) Bumping the workspace version (Cargo.toml + version.toml),
  2) Building a release framec:
       cargo build --release -p framec
"""

import os
import shutil
import sys
from pathlib import Path


def read_version(repo_root: Path) -> str:
    """Read the semantic version from version.toml (full = \"0.86.63\")."""
    version_file = repo_root / "version.toml"
    if not version_file.is_file():
        raise SystemExit(f"ERROR: version.toml not found at {version_file}")
    text = version_file.read_text(encoding="utf-8")
    for line in text.splitlines():
        line = line.strip()
        if line.startswith("full"):
            # full = "0.86.63"
            parts = line.split("=")
            if len(parts) == 2:
                val = parts[1].strip().strip('"')
                if val:
                    return val
    raise SystemExit("ERROR: could not find full version in version.toml")


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    version = read_version(repo_root)

    release_bin = repo_root / "target" / "release" / "framec"
    if not release_bin.is_file():
        sys.stderr.write(
            f"ERROR: release binary not found at {release_bin}\n"
            "Build it first with: cargo build --release -p framec\n"
        )
        return 1

    # 1) Update boot/framec/framec inside the main repo
    boot_dir = repo_root / "boot" / "framec"
    boot_dir.mkdir(parents=True, exist_ok=True)
    boot_bin = boot_dir / "framec"
    shutil.copy2(release_bin, boot_bin)
    print(f"Updated bootstrap compiler at {boot_bin} (version {version})")

    # 2) Update shared test env reference, if it exists as a sibling repo
    shared_root = repo_root.parent / "framepiler_test_env"
    releases_dir = shared_root / "bug" / "releases" / "frame_transpiler"
    if releases_dir.is_dir():
        vdir = releases_dir / f"v{version}" / "framec"
        vdir.mkdir(parents=True, exist_ok=True)
        shared_bin = vdir / "framec"
        shutil.copy2(release_bin, shared_bin)
        print(f"Updated shared env reference at {shared_bin}")
    else:
        print(
            f"Shared test env not found at {releases_dir}; "
            "skipping shared-env copy."
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())


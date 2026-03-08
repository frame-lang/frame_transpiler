#!/usr/bin/env python3
"""
Split a common .frm fixture into language-specific variants.
For Python variant, convert domain var declarations to native assignments.

Usage:
  python3 scripts/codemods/split_common_fixture.py <common_fixture.frm> [more files...]
"""
import sys
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def convert_python_domain_vars(lines):
    out = []
    in_domain = False
    for i, line in enumerate(lines):
        stripped = line.strip()
        # Detect entering domain section
        if stripped.startswith('domain:'):
            in_domain = True
            out.append(line)
            continue
        # Heuristic: domain section ends at a closing brace on its own line or another top-level section
        if in_domain and (stripped == '}' or re.match(r'^(interface|machine|actions|operations)\s*:', stripped)):
            in_domain = False
        if in_domain:
            m = re.match(r'^(\s*)var\s+(.*)', line)
            if m:
                indent, rest = m.groups()
                out.append(f"{indent}{rest}\n")
                continue
        out.append(line)
    return out

def split_fixture(common_path: Path):
    rel = common_path.relative_to(ROOT / 'framec_tests' / 'common' / 'tests')
    category = rel.parts[0]
    dest_py = ROOT / 'framec_tests' / 'language_specific' / 'python' / category / rel.name
    dest_ts = ROOT / 'framec_tests' / 'language_specific' / 'typescript' / category / rel.name

    src = common_path.read_text()
    lines = src.splitlines(keepends=True)

    # Write TS variant unmodified
    dest_ts.parent.mkdir(parents=True, exist_ok=True)
    dest_ts.write_text(src)

    # Python: convert domain var to native assignment
    py_lines = convert_python_domain_vars(lines)
    dest_py.parent.mkdir(parents=True, exist_ok=True)
    dest_py.write_text(''.join(py_lines))

def main():
    if len(sys.argv) < 2:
        print("Usage: split_common_fixture.py <path.frm> [more]")
        sys.exit(2)
    for arg in sys.argv[1:]:
        p = Path(arg)
        if not p.exists():
            print(f"Skip missing {p}")
            continue
        split_fixture(p)

if __name__ == '__main__':
    main()


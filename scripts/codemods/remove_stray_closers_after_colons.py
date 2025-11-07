#!/usr/bin/env python3
import sys
from pathlib import Path

def process(path: Path) -> bool:
    lines = path.read_text(encoding='utf-8').splitlines()
    out = []
    changed = False
    prev_meaningful = None  # store stripped prev non-empty line
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == '}' and prev_meaningful and prev_meaningful.rstrip().endswith(':'):
            # Likely a stray closer for a colon-based block; drop it
            changed = True
            continue
        out.append(line)
        if stripped != '' and not stripped.startswith('//') and not stripped.startswith('#'):
            prev_meaningful = stripped
    if changed:
        path.write_text("\n".join(out) + "\n", encoding='utf-8')
    return changed


def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('.')
    count = 0
    files = 0
    for p in root.rglob('framec_tests/language_specific/python/**/*.frm'):
        if process(p):
            files += 1
            print(f"stray-closer removed in {p}")
    print(f"done: files={files}")

if __name__ == '__main__':
    main()

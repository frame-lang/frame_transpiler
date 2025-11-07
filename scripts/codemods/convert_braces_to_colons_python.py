#!/usr/bin/env python3
import re
import sys
from pathlib import Path

OPEN_PATTERNS = [
    (re.compile(r"^(?P<indent>\s*)if\s+(.+?)\s*\{\s*$"), lambda m: f"{m.group('indent')}if {m.group(2)}:"),
    (re.compile(r"^(?P<indent>\s*)elif\s+(.+?)\s*\{\s*$"), lambda m: f"{m.group('indent')}elif {m.group(2)}:"),
    (re.compile(r"^(?P<indent>\s*)else\s*\{\s*$"), lambda m: f"{m.group('indent')}else:"),
    (re.compile(r"^(?P<indent>\s*)for\s+(.+?)\s*\{\s*$"), lambda m: f"{m.group('indent')}for {m.group(2)}:"),
    (re.compile(r"^(?P<indent>\s*)while\s+(.+?)\s*\{\s*$"), lambda m: f"{m.group('indent')}while {m.group(2)}:"),
    (re.compile(r"^(?P<indent>\s*)try\s*\{\s*$"), lambda m: f"{m.group('indent')}try:"),
    (re.compile(r"^(?P<indent>\s*)except\s+(.*)\{\s*$"), lambda m: f"{m.group('indent')}except {m.group(2)}:"),
    (re.compile(r"^(?P<indent>\s*)except\s*\{\s*$"), lambda m: f"{m.group('indent')}except:"),
    (re.compile(r"^(?P<indent>\s*)else\s*\{\s*$"), lambda m: f"{m.group('indent')}else:"),
    (re.compile(r"^(?P<indent>\s*)finally\s*\{\s*$"), lambda m: f"{m.group('indent')}finally:"),
]

# Patterns that combine a close + open, e.g., "} else {"
COMBINED = [
    (re.compile(r"^(?P<indent>\s*)\}\s*else\s*\{\s*$"), lambda m: ("pop", f"{m.group('indent')}else:")),
    (re.compile(r"^(?P<indent>\s*)\}\s*elif\s+(.+?)\s*\{\s*$"), lambda m: ("pop_push", f"{m.group('indent')}elif {m.group(2)}:")),
]

CLOSE_ONLY = re.compile(r"^\s*\}\s*$")

# Convert a file in-place; returns (changed: bool, modifications: int)

def convert_file(path: Path) -> tuple[bool,int]:
    text = path.read_text(encoding='utf-8')
    lines = text.splitlines()
    out = []
    stack = []  # track converted native blocks
    changed = False
    mods = 0

    for line in lines:
        # First handle combined cases
        handled = False
        for rx, repl in COMBINED:
            m = rx.match(line)
            if m:
                kind, newline = repl(m)
                if kind in ("pop", "pop_push"):
                    if stack:
                        stack.pop()
                    # push for elif/else as a new block
                    if kind == "pop_push":
                        stack.append('block')
                out.append(newline)
                changed = True
                mods += 1
                handled = True
                break
        if handled:
            continue

        # Openers
        opened = False
        for rx, repl in OPEN_PATTERNS:
            m = rx.match(line)
            if m:
                out.append(repl(m))
                stack.append('block')
                changed = True
                mods += 1
                opened = True
                break
        if opened:
            continue

        # Closers (only remove when we have an open native block)
        if CLOSE_ONLY.match(line):
            if stack:
                stack.pop()
                changed = True
                mods += 1
                # do not emit this '}'
            else:
                out.append(line)
            continue

        # Default: passthrough
        out.append(line)

    # Do not attempt to auto-fix unmatched stack; leave as-is
    if changed:
        path.write_text("\n".join(out) + "\n", encoding='utf-8')
    return changed, mods


def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('.')
    targets = list(root.rglob('framec_tests/language_specific/python/**/*.frm'))
    total = 0
    changed_files = 0
    for p in targets:
        ch, n = convert_file(p)
        if ch:
            changed_files += 1
            total += n
            print(f"converted {p} (+{n})")
    print(f"done: files={changed_files}, edits={total}")

if __name__ == '__main__':
    main()

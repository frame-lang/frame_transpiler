#!/usr/bin/env python3
"""
Migrate legacy Python Frame fixtures to V3 module form:
- Insert "@target python" prolog if missing
- Convert handler/state headers from colon form to brace form
  e.g., "$A:" -> "$A {" and "e():" -> "e() {"
- Insert closing '}' based on indentation when leaving header/state blocks

This codemod is SOL-anchored and indentation-aware; it does NOT alter native
Python control flow inside handler bodies.
"""
from __future__ import annotations
import sys
from pathlib import Path
import re

HEADER_FUNC = re.compile(r"^(?P<indent>\s*)(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\([^)]*\)\s*:\s*$")
HEADER_STATE = re.compile(r"^(?P<indent>\s*)\$(?P<name>[A-Za-z_][A-Za-z0-9_]*)(?:\s*\([^)]*\))?\s*:\s*$")
SECTION = re.compile(r"^\s*(interface|machine|actions|operations)\s*:\s*$")
BLANK_OR_COMMENT = re.compile(r"^\s*(#.*)?$")

def migrate_text(text: str) -> str:
    lines = text.splitlines(keepends=False)
    out: list[str] = []
    # Prolog injection
    has_target = any(l.strip().startswith('@target ') for l in lines[:5])
    if not has_target:
        out.append('@target python')
        out.append('')

    # Process with indentation-aware brace insertion
    stack: list[int] = []  # stores indentation levels for our inserted '{'

    def close_to(indent: int):
        while stack and stack[-1] >= 0 and indent < stack[-1]:
            close_indent = ' ' * stack[-1]
            out.append(f"{close_indent}}}")
            stack.pop()

    for raw in lines:
        line = raw.rstrip('\n')
        # Skip pure BOMs or keep as-is later
        if BLANK_OR_COMMENT.match(line):
            out.append(line)
            continue

        # Detect state header
        m = HEADER_STATE.match(line)
        if m:
            indent = len(m.group('indent').expandtabs(4))
            close_to(indent)
            # Replace ':' with '{'
            out.append(f"{m.group('indent')}${m.group('name')} {{")
            stack.append(indent)
            continue

        # Detect handler/function header (inside sections/states)
        m = HEADER_FUNC.match(line)
        if m:
            indent = len(m.group('indent').expandtabs(4))
            close_to(indent)
            name = m.group('name')
            # Replace only trailing ':' with '{'
            replaced = line[:-1] + '{'
            out.append(replaced)
            stack.append(indent)
            continue

        # Section lines remain unchanged
        if SECTION.match(line):
            out.append(line)
            continue

        # Non-header code line: handle dedent-triggered closures
        leading_spaces = len(line) - len(line.lstrip(' '))
        close_to(leading_spaces)
        out.append(line)

    # Close any remaining open blocks
    close_to(-1)
    return '\n'.join(out) + '\n'

def process_path(path: Path) -> bool:
    text = path.read_text(encoding='utf-8')
    new_text = migrate_text(text)
    if new_text != text:
        path.write_text(new_text, encoding='utf-8')
        return True
    return False

def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('framec_tests/language_specific/python')
    changed = 0
    for p in root.rglob('*.frm'):
        if process_path(p):
            changed += 1
    print(f"Migrated {changed} files under {root}")

if __name__ == '__main__':
    main()


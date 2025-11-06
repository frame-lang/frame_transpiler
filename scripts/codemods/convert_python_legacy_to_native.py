#!/usr/bin/env python3
import re
import sys
from pathlib import Path

PY_BLOCK_START = re.compile(r"^(\s*)(try|if|elif|else|for|while|with|async\s+with|async\s+for)\b(.*)\{\s*$")
PY_EXCEPT_START = re.compile(r"^(\s*)\}?(\s*)except\b(.*)\{\s*$")
PY_FINALLY_START = re.compile(r"^(\s*)\}?(\s*)finally\b(.*)\{\s*$")
INDENTED_BRACE = re.compile(r"^\s+\}\s*$")
VAR_DECL = re.compile(r"^(\s*)var\s+")

def transform_lines(lines):
    out = []
    for line in lines:
        # remove indented closing braces for legacy block endings inside Python bodies
        if INDENTED_BRACE.match(line):
            continue

        # convert 'var x = ...' to 'x = ...'
        m = VAR_DECL.match(line)
        if m:
            indent = m.group(1)
            rest = line[m.end():]
            out.append(f"{indent}{rest}")
            continue
        # convert block starters with '{' to ':'
        m = PY_BLOCK_START.match(line)
        if m:
            indent, kw, rest = m.groups()
            rest = rest.rstrip().rstrip('{').rstrip()
            if rest and not rest.strip().endswith((':',)):
                out.append(f"{indent}{kw}{rest}:\n")
            else:
                out.append(f"{indent}{kw}:\n")
            continue
        # convert 'except X {' to 'except X:'
        m = PY_EXCEPT_START.match(line)
        if m:
            indent = m.group(1) or ""
            rest = m.group(3) or ""
            rest = rest.rstrip().rstrip('{').rstrip()
            if rest:
                out.append(f"{indent}except{rest}:\n")
            else:
                out.append(f"{indent}except:\n")
            continue

        # convert 'finally {' to 'finally:'
        m = PY_FINALLY_START.match(line)
        if m:
            indent = m.group(1) or ""
            out.append(f"{indent}finally:\n")
            continue
        out.append(line)
    return out

def process_file(path: Path):
    text = path.read_text()
    lines = text.splitlines(keepends=True)
    new_lines = transform_lines(lines)
    if new_lines != lines:
        path.write_text(''.join(new_lines))
        return True
    return False

def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path('framec_tests/language_specific/python')
    changed = 0
    for p in root.rglob('*.frm'):
        if process_file(p):
            changed += 1
    print(f"Converted {changed} files under {root}")

if __name__ == '__main__':
    main()

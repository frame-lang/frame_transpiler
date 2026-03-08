#!/usr/bin/env python3
import os
import shutil
import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LS_ROOT = ROOT / 'framec_tests' / 'language_specific'
RET_ROOT = ROOT / 'framec_tests' / 'retired_legacy'

def find_legacy(langs=None):
    legacy = []
    for lang_dir in sorted(LS_ROOT.iterdir()):
        if not lang_dir.is_dir():
            continue
        lang = lang_dir.name
        if langs and lang not in langs:
            continue
        for cat_dir in sorted(lang_dir.iterdir()):
            if not cat_dir.is_dir():
                continue
            if cat_dir.name.startswith('v3_'):
                continue
            legacy.append((lang, cat_dir.name, cat_dir))
    return legacy

def retire(legacy, apply=False):
    moves = []
    for lang, cat, src in legacy:
        dest = RET_ROOT / lang / cat
        moves.append((src, dest))
        if apply:
            dest.parent.mkdir(parents=True, exist_ok=True)
            # Move directory wholesale
            shutil.move(str(src), str(dest))
    return moves

def ensure_readme():
    RET_ROOT.mkdir(parents=True, exist_ok=True)
    readme = RET_ROOT / 'README.md'
    if not readme.exists():
        readme.write_text(
            """# Retired Legacy Tests

This folder contains pre‑V3 test suites that have been retired.

Rationale:

- V3 architecture (SOL‑only, DPDA scanners, strict validator, curated exec) supersedes legacy semantics.
- Redundant fixtures are removed from active runs to keep CI hermetic and focused.
- Unique legacy cases, if any, are converted into V3 categories.

If needed, individual fixtures can be re‑introduced as V3 equivalents under `framec_tests/language_specific/<lang>/v3_*`.
"""
        )

def main():
    ap = argparse.ArgumentParser(description='Retire non‑V3 legacy tests into retired_legacy/')
    ap.add_argument('--apply', action='store_true', help='Perform the move (otherwise dry‑run)')
    ap.add_argument('--languages', nargs='*', help='Limit to specific languages (e.g., python typescript)')
    args = ap.parse_args()

    ensure_readme()
    legacy = find_legacy(set(args.languages) if args.languages else None)
    if not legacy:
        print('No legacy tests found')
        return
    moves = retire(legacy, apply=args.apply)
    print(('Applied' if args.apply else 'Dry‑run') + ' retirement for:')
    for src, dest in moves:
        print(f'  {src}  ->  {dest}')

if __name__ == '__main__':
    main()

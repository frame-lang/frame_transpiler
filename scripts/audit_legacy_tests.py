#!/usr/bin/env python3
import os, json, sys

ROOT = os.path.join(os.path.dirname(__file__), '..', 'framec_tests', 'language_specific')

def collect_legacy():
    legacy = {}
    for lang in sorted(os.listdir(ROOT)):
        ldir = os.path.join(ROOT, lang)
        if not os.path.isdir(ldir):
            continue
        entries = []
        for cat in sorted(os.listdir(ldir)):
            if cat.startswith('v3_'):
                continue
            path = os.path.join(ldir, cat)
            if not os.path.isdir(path):
                continue
            count = 0
            for dp, dn, fn in os.walk(path):
                count += sum(1 for f in fn if f.endswith('.frm'))
            entries.append((cat, count, os.path.relpath(path, os.path.join('..', 'framec_tests'))))
        if entries:
            legacy[lang] = entries
    return legacy

def main():
    legacy = collect_legacy()
    print(json.dumps(legacy, indent=2))

if __name__ == '__main__':
    main()


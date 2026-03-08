# Comments Policy (Native Everywhere)

Decision: A Frame source file uses the target language’s native comment syntax throughout the file, once the `@target <lang>` prolog is declared.

## Prolog Rule

- `@target <lang>` must appear as the first non‑whitespace token in the file. Comments are not permitted before the prolog because the target is not yet known.

## After the Prolog

- All comments in the file follow the target language:
  - TypeScript: `// line`, `/* block */`
  - Python: `# line` (triple quotes `''' … '''` / `""" … """` are strings/docstrings, not comments)
- Applies uniformly to Frame shell (systems, states, transitions, signatures) and target bodies (member code).

## Parsing & Emission Notes

- The scanner switches into target‑aware mode after the prolog and skips native comments during tokenization and target‑body segmentation.
- Target body parsers/segmenters are comment‑aware (and string/template‑literal aware) so Frame tokens are recognized only at the top level and never inside comments or strings.
- Target comments are re‑emitted verbatim as part of native bodies. Shell comments are not emitted into generated target code by default (configurable later).

## Frame Tokens & Comments

- Frame tokens (e.g., `-> $State`, `push$`, `pop$`, `=> $^`) inside comments are ignored and have no effect.
- Strings containing `->` or `=>` are also ignored by the interleaver.

## Rationale

- Keeps files idiomatic for the target language and reduces cognitive switching.
- Simplifies parsing because comment rules match the target once the prolog is known.

# Body Boundary Detectors (DPDA) and Failure Characterizers

Status: Authoritative; staged rollout per target
Last updated: 2025‑11‑06

## Overview

Frame uses per‑language body boundary detectors to find the matching closing `}` for target‑native bodies embedded in Frame. Each detector is a deterministic pushdown automaton (DPDA): a small finite‑state machine with a depth counter/stack for nested constructs (e.g., braces, template expressions), scanning source text in a single pass without backtracking.

To improve diagnostics and recovery, we complement the primary DPDA with optional secondary “characterizers” that run only when detection fails or is ambiguous and classify the error (e.g., unterminated string/template/comment).

## Why DPDA

- Nested structures: `{ … }`, nested `${ … }` (TS), or multi‑line strings need depth‑aware scanning.
- Determinism: no backtracking, stable O(n) passes on raw text.
- Independence: boundary detection is orthogonal to statement parsing and does not consume tokens.

## Primary Detector (per language)

Inputs:
- `source`: the file contents (read as lines)
- `body_start_line` (1‑based): line where the body’s opening `{` appears

Outputs:
- `DetectionResult::Ok { close_line, notes }` with the line containing the closing `}`
- `DetectionResult::Failure { kind, context }` if detection fails

Modes & Counters (examples)
- TypeScript: in_squote/in_dquote/in_template/in_block_comment/in_line_comment; `brace_depth`, `tpl_expr_depth` for `${ … }`
- Python: in_squote/in_dquote/in_tsquote/in_tdquote; `brace_depth`; skip rest of line after `#`

Guarantees
- Never consumes tokens; returns a close line for the parser to advance up to (just before `}`)
- Total and non‑panicking; on failure, returns a structured Failure kind

## Failure Characterizers

Run only when the primary DPDA returns Failure or an ambiguous result. Each characterizer is a tiny single‑purpose scan in a small window (e.g., next 256 lines) that either reports a specific failure or `None`.

Examples
- `UnterminatedString`: matches unterminated `'…'`/`"…"`/`'''…'''`/`"""…"""`
- `UnterminatedTemplate` (TS): detects backtick template with dangling `${ … }` depth
- `UnterminatedBlockComment` (C‑style): detects `/* …` without closing `*/`
- `PreprocessorBlock` (C/C++): flags lines that may interfere with brace counting

Result Shape
- `FailureKind::UnterminatedString { quote, start_line }`
- `FailureKind::UnterminatedTemplate { start_line, depth }`
- `FailureKind::UnterminatedComment { kind, start_line }`
- `FailureKind::EOFBeforeClose { last_line }`

## Parser Integration & Recovery

1) The parser calls the detector to get `close_line`.
2) If `Ok`, advance tokens up to just before the `}`; the parser then consumes `}` normally.
3) If `Failure`, run characterizers; if a specific Failure is identified, raise a targeted `ParseError` with frame line + snippet; else fall back to token‑depth guard.
4) If the guard also cannot find a close (EOF), error "Unterminated body" and enter panic mode; synchronize using existing tokens (e.g., `CloseBrace`, section headers: `actions:`, `operations:`, `machine:`, `domain:`).

Invariants
- Never advance past the closing `}`; the parser owns `{`/`}` consumption.
- MixedBody remains authoritative; the detector only bounds the slice, it does not parse statements.
- SOL detection + segmenters ignore strings/comments/templates; false positives are minimized.

## Per‑Language Bundles

TypeScript
- Primary: template‑aware detector (`scan_ts_closing_brace_line`)
- Characterizers: unterminated template literal (`\`` + unbalanced `${ … }`), unterminated string, unterminated `/* … */`

Python
- Primary: triple‑quote/f‑string‑aware detector (`scan_py_closing_brace_line`)
- Characterizers: unterminated triple‑quoted string, unterminated single/double quoted string

## Error Messages (suggested)
- TS: "Unterminated template literal (started at line N)"; "Unterminated block comment (started at line M)"
- Py: "Unterminated triple‑quoted string (started at line N)"; "Unterminated string literal (started at line M)"

## Roadmap Tasks
- Represent detector outputs as `DetectionResult` (Ok/Failure) in code
- Add characterizers per target and wire them to parser error surfaces
- Add negative fixtures for unterminated literals/templates/comments with expected failure classification
- Keep detector fallbacks and parser synchronization unchanged


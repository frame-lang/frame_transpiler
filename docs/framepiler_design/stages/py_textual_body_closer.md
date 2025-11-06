# Python Body Boundary Detection (Triple‑Quote/F‑String Aware)

Status: Ready; staged for guarded rollout
Last updated: 2025‑11‑06

## Purpose

Find the closing `}` for a Python target body without being confused by Python string constructs that may contain braces, such as triple‑quoted strings and f‑strings.

## What it tracks

- Single and double quotes with escapes: `'…'`, `"…"`.
- Triple‑quoted strings: `'''…'''` and `"""…"""` across multiple lines.
- f‑strings: treated as strings for boundary detection (braces inside the string are ignored). Escaped braces and nested dicts inside expressions are irrelevant at this stage because the entire f‑string is considered a single string region for boundary detection.
- Comments: `# …` (line remainder is ignored).

## Algorithm (high‑level)

- Inputs: `body_start_line` and `source` split by lines.
- State booleans: `in_squote`, `in_dquote`, `in_tsquote` (triple `'`), `in_tdquote` (triple `"`).
- For each line after the opening `{`:
  - If not in a string:
    - Detect start of triple quotes before single/double quotes.
    - Enter strings and triple‑quoted strings appropriately.
    - `#` starts a comment; ignore rest of line.
    - Count `{`/`}` as body delimiters.
  - If in a string:
    - For single/double quotes, handle backslash escapes and close on matching quote.
    - For triple quotes, close on matching triple sequence.
- Stop when body brace depth returns to 0; record `close_line`.

## Notes

- Handles CRLF (`\r\n`) and treats NBSP/tabs as ordinary characters.
- We do not parse f‑string expression contents; boundary detection only needs to avoid counting braces inside string literals.

## Usage plan

- Guarded rollout: use the textual detector for Python bodies only when triple quotes or obvious f‑string markers are present; otherwise keep the current token‑depth guard. Validate the full Python single‑file suite after each change.
- MixedBody/segmenter behavior is unchanged; this detector only provides a reliable body slice.


# Research RFCs (Authoritative Index)

This folder hosts design RFCs for Frame language and semantics.

## Conventions
- Location: docs/framelang_design/research/RFC-####-kebab-title.md
- Numbering: four digits, monotonically increasing (0001, 0002, …).
- Status lifecycle:
  - draft → proposed → accepted → implemented
  - Terminal: rejected | withdrawn | superseded

### Status Definitions
- `draft`: Authoring in progress; open to early feedback; no formal review yet.
- `proposed`: Submitted for design review; content frozen except for review‑driven edits; seeking explicit approval.
- `accepted`: Approved design; becomes source of truth for implementation and docs; semantic changes require a new RFC.
- `implemented`: Landed in code and docs (flagged or fully on); RFC remains as the historical record.
- `rejected`: Reviewed and explicitly not proceeding; kept for history.
- `withdrawn`: Pulled by author/owner before decision or no longer pursued.
- `superseded`: Replaced by a newer RFC; use `superseded_by` / `supersedes` links to connect them.

### Typical Transitions
- `draft` → `proposed` (author requests review)
- `proposed` → `accepted` | `rejected` (design decision)
- `accepted` → `implemented` (work lands)
- `draft`/`proposed`/`accepted` → `withdrawn` (author/owner choice)
- any → `superseded` (new RFC replaces an older one)
- Header (YAML front‑matter):
  - id, title, authors, status, created, updated,
    tracking (issue/PR), supersedes, superseded_by.
- Structure (sections):
  - Summary
  - Motivation / Problem
  - Goals / Non‑Goals
  - Design (syntax + semantics)
  - Backward Compatibility / Migration
  - Parsing & Codegen Impact
  - Source Maps & Validation
  - Alternatives Considered
  - Risks / Open Questions
  - Test Plan / Rollout
  - References
- Scope: RFCs are language/semantics focused. Implementation plans for compiler/runtime live in docs/framepiler_design and are referenced from RFCs.
- Style: concise, normative; examples in valid Frame.
- Links: reference authoritative design docs (core contract, grammar) where relevant.

## Index
- RFC‑0001 — Nested HSM Syntax (this repo): RFC-0001-nested-hsm-syntax.md

# Stage 06.5 — Structural Validation (Early)

Purpose
- Provide lightweight, hermetic structural checks before native parse facades (Stage 07) to de-risk and validate core segmentation/MIR across all languages.

Inputs
- Module outline (from `ModulePartitionerV3` + `OutlineScannerV3`)
- MixedBody MIR per handler (from `MirAssemblerV3`)

Rules (Early)
- Terminal‑last: Transition/Forward/Stack ops must be last MIR items in handler bodies.
- No Frame statements in actions/operations (Outline kinds authoritative).
- Machine state header must include '{' following `$State …` on the same logical line.

Diagnostics
- Human oriented messages; precise spans via MIR and outline header spans. Error taxonomy codes are introduced at Stage 09.

Complexity
- Linear in MIR item and outline count; no native parsing, no external toolchains.

Testing
- Python runner fixtures under `language_specific/<lang>/v3_validator/{positive,negative}` for all 7 languages.
- Positives: terminal‑last, clean actions/ops.
- Negatives: multiple Frame statements after terminal; Frame statements in actions/ops; missing '{' after state header.

Notes
- Stage 06.5 does not perform policy/semantic validation (e.g., Python native policy). Those live in Stage 09.


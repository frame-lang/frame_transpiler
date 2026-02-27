# Stage 06.5 — Structural Validation (Early)

Purpose
- Provide lightweight, hermetic structural checks before native parse facades (Stage 07) to de-risk and validate core segmentation/MIR across all languages.

Inputs
- Module outline (from `ModulePartitionerV3` + `OutlineScannerV3`)
- MixedBody MIR per handler (from `MirAssemblerV3`)

Rules (Early)
- Transition-in-block terminal: A Transition must be the last statement in its containing block. Forward and Stack operations are not mandated terminal and may be followed by additional native statements.
- No Frame statements in actions/operations (Outline kinds authoritative).
- Machine state header must include '{' following `$State …` on the same logical line.
- Parent forward availability (preview): For module demos (files with a `machine:` section), parent forward (=> $^) must only be used in states that participate in a declared parent relationship (e.g., `$Child => $Parent { … }`). If no parent is declared in the machine:, validation reports “Cannot forward to parent: no parent available.” Single‑body demo fixtures that do not have a `machine:` section are exempt at this stage.

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

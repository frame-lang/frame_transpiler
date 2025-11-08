# Archive (Project Hygiene)

This directory holds non-critical, legacy, or tooling artifacts that should not
live at the repository root. Moving items here keeps the root clean and reduces
noise for contributors and CI.

Policy
- Only move items that are not required by builds, tests, or packaging.
- Prefer to delete generated/transient artifacts instead of archiving them
  (e.g., node_modules, .DS_Store, IDE caches).
- Keep an index of archived items with a one-line rationale.
- Do not archive Cargo manifests, code, runtimes, tests, or authoritative docs.

Index (to be filled after approval)
- <file-or-folder>: <reason>

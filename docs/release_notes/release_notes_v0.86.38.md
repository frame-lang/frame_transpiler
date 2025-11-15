# Release Notes — v0.86.38 (2025-11-15)

Type: Bug-fix (metadata alignment)

Highlights
- Formalizes the release boundary for Bug #070.
  - Python interface handler/action emission fix (OutlineScanner allows bare IDENT headers under `interface:`) is now shipped in an explicit 0.86.38 release.
  - v3_cli fixture `interface_handlers_emitted.frm` validates presence of both handler and `_action_*` methods.

Notes
- Code changes were implemented prior to this tag; 0.86.38 is a version/metadata alignment release so downstream tooling can depend on a clean tag that includes the Bug #070 fix.

Version
- Workspace version bumped to 0.86.38; CLI `--version` reflects this.

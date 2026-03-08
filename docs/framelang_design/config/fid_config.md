# FID Configuration in frame.toml

Project-level settings for Frame Interface Definition (FID) generation and consumption live in `frame.toml`. The FID manifest (`fid_manifest.json`) stays source-only; these settings control behavior.

## Example

```toml
[fid]
manifest = "fid_manifest.json"   # where to read sources (optional; CLI can override)
lockfile = "fid.lock.json"       # where to write/read the lock
lock_policy = "strict"           # strict | update | off
cache_dir = ".frame/cache/fid" # write location for generated .fid cache
vendor_dirs = ["vendor/fid/{target}"] # read-first search paths for vendored caches
on_missing = "error"             # error | warn | ignore
auto_generate = "on-demand"      # on-build | on-demand | never

[fid.importers.typescript]
typedoc_bin = "npx typedoc"
# tsconfig = "tsconfig.json"    # optional override

[fid.importers.python]
python_bin = "python3"
introspect_site = true
```

## Semantics

- `lock_policy`
  - `strict`: require up-to-date `fid.lock.json`; fail if regeneration is needed.
  - `update`: regenerate on input change and update the lock.
  - `off`: ignore the lock (dev-only).
- `cache_dir`: base directory for `.fid` outputs; target subfolders are appended automatically (e.g., `…/typescript/`).
- `vendor_dirs`: prioritized read locations for hermetic builds. The compiler searches vendor paths before the cache. Use `{target}` to map per backend.
- `on_missing`: controls compiler behavior when a required declaration is missing.
- `auto_generate`
  - `on-build`: run `fid import` automatically during `framec build`.
  - `on-demand`: only when invoked explicitly.
  - `never`: disable automation.

Importer-specific tables (under `fid.importers.*`) provide tool-specific knobs (e.g., `typedoc_bin`).

## CLI Precedence

CLI flags > environment variables > `frame.toml` > built-in defaults.

## Related Docs

- `../frame_interface_definition/native_imports_and_fid.md` — manifest format and `.fid` grammar
- `frame_toml_guide.md` — broader project configuration

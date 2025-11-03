# Frame Project Configuration Guide (frame.toml)

Version: v0.58

## Overview

Frame uses a standardized project configuration file, `frame.toml`. This centralizes build settings, paths, and target‑specific toggles, and provides hooks for native import/FID generation.

## Quick Start

### Initialize a New Project

```bash
framec init
```
Creates:
- `frame.toml` (project configuration)
- `src/` directory
- `src/main.frm` (entry point)

### Build a Project

```bash
framec build
# Or specify a custom config file
framec build --config custom.toml
```

## Configuration File Format

The `frame.toml` file uses TOML and consists of several sections.

### Complete Example

```toml
# Project metadata
[project]
name = "my-frame-app"
version = "0.1.0"
entry = "src/main.frm"
authors = ["Jane Developer <jane@example.com>"]
description = "A Frame state machine application"

# Build settings
[build]
target = "python_3"
output_dir = "dist"
output_mode = "separate_files"  # or "concatenated"
source_dirs = ["src", "lib"]
optimize = false
debug = false
incremental = true

# Python-specific settings
[python]
event_handlers_as_functions = true
runtime = "Standard"  # or "AsyncIO"
min_version = "3.8"
public_state_info = false
public_compartment = false

# Module search paths and aliases
[paths]
modules = ["src", "lib", "vendor"]
imports = ["external"]

[paths.aliases]
"@utils" = "./src/utils"
"@components" = "./src/components"
"@models" = "./src/models"

# FID generation/consumption (see fid_config.md for full details)
[fid]
manifest = "fid_manifest.json"
lockfile = "fid.lock.json"
lock_policy = "strict"           # strict | update | off
cache_dir = ".frame/cache/fid"
vendor_dirs = ["vendor/fid/{target}"]
on_missing = "error"             # error | warn | ignore
auto_generate = "on-demand"      # on-build | on-demand | never

[fid.importers.typescript]
typedoc_bin = "npx typedoc"

[fid.importers.python]
python_bin = "python3"
introspect_site = true

# Custom scripts
[scripts]
build = "framec build"
test = "python -m pytest tests/"
clean = "rm -rf dist/"
dev = "framec build --watch"
lint = "ruff check src/"
format = "black src/"
```

## Configuration Sections

### [project] — Project Metadata

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `name` | string | Project name | Directory name |
| `version` | string | Project version | "0.1.0" |
| `entry` | path | Entry point file | "src/main.frm" |
| `authors` | array | List of authors | [] |
| `description` | string | Project description | None |

### [build] — Build Configuration

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `target` | string | Target language | "python_3" |
| `output_dir` | path | Output directory | "dist" |
| `output_mode` | enum | "concatenated" or "separate_files" | "concatenated" |
| `source_dirs` | array | Source directories | ["src"] |
| `optimize` | bool | Enable optimizations | false |
| `debug` | bool | Enable debug output | false |
| `incremental` | bool | Use incremental compilation | true |

#### Output Modes
- `concatenated`: All modules combined into a single output file
- `separate_files`: Each module generates its own file with proper imports

### [python] — Python‑Specific Settings

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `event_handlers_as_functions` | bool | Generate handlers as functions | true |
| `runtime` | enum | "Standard", "AsyncIO", "Trio" | "Standard" |
| `min_version` | string | Minimum Python version | None |
| `public_state_info` | bool | Make state info public | false |
| `public_compartment` | bool | Make compartment public | false |

### [paths] — Module Resolution

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `modules` | array | Module search directories | ["src"] |
| `imports` | array | Import search directories | [] |
| `aliases` | table | Path alias mappings | {} |

#### Using Path Aliases
```toml
[paths.aliases]
"@utils" = "./src/utils"
"@core" = "./src/core"
```

Then in Frame:
```frame
import Utils from "@utils/helpers.frm"
import Core from "@core/engine.frm"
```

### [fid] — FID Generation & Consumption
High‑level toggles for the native import pipeline. See `fid_config.md` for full semantics and importer‑specific options.

| Field | Type | Description |
|-------|------|-------------|
| `manifest` | path | Path to `fid_manifest.json` describing native sources |
| `lockfile` | path | Path to `fid.lock.json` written/read by the importer |
| `lock_policy` | enum | `strict` (require lock), `update` (refresh on change), `off` |
| `cache_dir` | path | Base directory for generated `.fid` cache |
| `vendor_dirs` | array | Read‑first search paths for vendored `.fid` caches |
| `on_missing` | enum | Compiler behavior on missing declaration: `error`/`warn`/`ignore` |
| `auto_generate` | enum | `on-build`/`on-demand`/`never` |

Importer‑specific tables live under `fid.importers.*`.

### [scripts] — Custom Commands
```toml
[scripts]
build = "framec build"
test = "python -m pytest"
clean = "rm -rf dist/"
dev = "framec build && python dist/main.py"
```

## File Discovery

Frame searches for configuration files in this order:
1. Path specified with `--config`
2. `frame.toml` in current directory
3. `.framerc.toml` in current directory
4. Parents up to filesystem root

## Environment Variables
- `FRAME_CONFIG`: Default config path
- `FRAME_TARGET`: Override target language
- `FRAME_OUTPUT`: Override output directory
- `FRAME_TRANSPILER_DEBUG`: Enable debug output (1 to enable)

## Examples

### Minimal frame.toml
```toml
[project]
name = "minimal"
entry = "main.frm"

[build]
target = "python_3"
```

### Library Project
```toml
[project]
name = "frame-utils"
version = "1.0.0"

description = "Utility functions for Frame"

[build]
output_mode = "separate_files"
source_dirs = ["src"]

[paths]
modules = ["src"]
```

### Application Project
```toml
[project]
name = "web-server"
version = "2.1.0"
entry = "src/server/main.frm"

[build]
target = "python_3"
output_dir = "build"
output_mode = "concatenated"
optimize = true

[python]
runtime = "AsyncIO"
min_version = "3.10"

[scripts]
start = "python build/main.py"
test = "pytest tests/"
deploy = "./scripts/deploy.sh"
```

## Conclusion
`frame.toml` provides a standardized, extensible way to manage Frame projects. It simplifies the build process, enables better tooling integration, and integrates cleanly with the native import/FID workflow.

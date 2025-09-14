# Frame Project Configuration Guide (frame.toml)

**Version**: v0.58  
**Last Updated**: September 14, 2025

## Overview

Frame v0.58 introduces a standardized project configuration system using `frame.toml` files. This system simplifies project management, standardizes build processes, and provides a foundation for future package management features.

## Quick Start

### Initialize a New Project

```bash
# Create a new Frame project in the current directory
framec init

# This creates:
# - frame.toml (project configuration)
# - src/ directory
# - src/main.frm (entry point)
```

### Build a Project

```bash
# Build using frame.toml configuration
framec build

# Or specify a custom config file
framec build --config custom.toml
```

## Configuration File Format

The `frame.toml` file uses the TOML format and consists of several sections:

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

### [project] - Project Metadata

Defines basic project information:

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `name` | string | Project name | Directory name |
| `version` | string | Project version | "0.1.0" |
| `entry` | path | Entry point file | "src/main.frm" |
| `authors` | array | List of authors | [] |
| `description` | string | Project description | None |

### [build] - Build Configuration

Controls the compilation process:

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `target` | string | Target language | "python_3" |
| `output_dir` | path | Output directory | "dist" |
| `output_mode` | enum | "concatenated" or "separate_files" | "concatenated" |
| `source_dirs` | array | Source directories to search | ["src"] |
| `optimize` | bool | Enable optimizations | false |
| `debug` | bool | Enable debug output | false |
| `incremental` | bool | Use incremental compilation | true |

#### Output Modes

- **concatenated**: All modules combined into a single output file
- **separate_files**: Each module generates its own file with proper imports

### [python] - Python-Specific Settings

Configure Python code generation:

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `event_handlers_as_functions` | bool | Generate handlers as functions (v0.36) | true |
| `runtime` | enum | "Standard", "AsyncIO", or "Trio" | "Standard" |
| `min_version` | string | Minimum Python version | None |
| `public_state_info` | bool | Make state info public | false |
| `public_compartment` | bool | Make compartment public | false |

### [paths] - Module Resolution

Configure import path resolution:

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `modules` | array | Module search directories | ["src"] |
| `imports` | array | Import search directories | [] |
| `aliases` | table | Path alias mappings | {} |

#### Using Path Aliases

Define shortcuts for common import paths:

```toml
[paths.aliases]
"@utils" = "./src/utils"
"@core" = "./src/core"
```

Then use in Frame code:
```frame
import Utils from "@utils/helpers.frm"
import Core from "@core/engine.frm"
```

### [scripts] - Custom Commands

Define project-specific commands:

```toml
[scripts]
build = "framec build"
test = "python -m pytest"
clean = "rm -rf dist/"
dev = "framec build && python dist/main.py"
```

Run with: `framec run <script-name>` (future feature)

## File Discovery

Frame searches for configuration files in this order:

1. Path specified with `--config` flag
2. `frame.toml` in current directory
3. `.framerc.toml` in current directory
4. Search parent directories up to filesystem root

## Project Structure Best Practices

### Simple Project
```
my-project/
├── frame.toml
├── src/
│   └── main.frm
└── dist/
    └── (generated files)
```

### Complex Project
```
my-project/
├── frame.toml
├── src/
│   ├── main.frm
│   ├── systems/
│   │   ├── server.frm
│   │   └── client.frm
│   ├── utils/
│   │   └── helpers.frm
│   └── models/
│       └── data.frm
├── lib/
│   └── external.frm
├── tests/
│   └── test_server.frm
├── dist/
│   └── (generated files)
└── README.md
```

## Output Examples

### Concatenated Mode

With `output_mode = "concatenated"`:

```bash
framec build > dist/app.py
# Generates single file containing all modules
```

### Separate Files Mode

With `output_mode = "separate_files"`:

```bash
framec build
# Generates:
# dist/
#   ├── __init__.py
#   ├── main.py
#   ├── server.py
#   └── utils.py
```

## Environment Variables

Frame respects these environment variables:

- `FRAME_CONFIG`: Default config file path
- `FRAME_TARGET`: Override target language
- `FRAME_OUTPUT`: Override output directory
- `FRAME_TRANSPILER_DEBUG`: Enable debug output (1 to enable)

## Migration from CLI Flags

### Before (v0.57)
```bash
framec -m src/main.frm -l python_3 -o dist/
```

### After (v0.58)
```bash
# One-time setup
framec init

# Then just:
framec build
```

## Troubleshooting

### Common Issues

**Issue**: "No frame.toml found"
- **Solution**: Run `framec init` or create frame.toml manually

**Issue**: "Invalid configuration"
- **Solution**: Check TOML syntax and required fields

**Issue**: Output not where expected
- **Solution**: Check `output_dir` and `output_mode` settings

### Validation

Frame validates configuration on load:
- Required fields must be present
- Paths must be valid
- Enums must have valid values

## Future Features

The configuration system is designed to support:

- Package dependencies
- Version management
- Profile-based builds (dev/release)
- Custom code generation options
- Plugin system configuration
- Watch mode settings
- Test runner integration

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

The frame.toml configuration system provides a standardized, extensible way to manage Frame projects. It simplifies the build process, enables better tooling integration, and prepares Frame for future package management capabilities.
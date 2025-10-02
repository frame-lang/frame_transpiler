# Frame Transpiler Scripts

This directory contains utility scripts for maintaining the Frame transpiler.

## Version Management

### Single Source of Truth: `version.toml`
The project version is maintained in `/version.toml` at the project root:

```toml
[version]
major = 0
minor = 79
patch = 0
full = "0.79.0"
```

### Automatic Synchronization

#### Pre-commit Hook (Automatic)
A Git pre-commit hook automatically syncs versions from `version.toml` to:
- `framec/Cargo.toml`
- `frame_build/Cargo.toml` 
- `framec/src/frame_c/compiler.rs`

**Location**: `.git/hooks/pre-commit`

The hook runs automatically before every commit and:
- ✅ Reads version from `version.toml`
- ✅ Updates all Cargo.toml files
- ✅ Updates compiler version string
- ✅ Stages updated files for commit
- ✅ Provides colored output showing changes

#### Manual Sync Script
For manual version synchronization without committing:

```bash
# Dry run (see what would change)
./scripts/sync-versions.sh --dry-run

# Apply changes
./scripts/sync-versions.sh
```

### Version Update Workflow

To update the project version:

1. **Edit `version.toml`** (single source of truth):
   ```toml
   full = "0.80.0"  # Update this line
   ```

2. **Commit** (automatic sync via pre-commit hook):
   ```bash
   git add version.toml
   git commit -m "chore: bump version to 0.80.0"
   ```

3. **Build** (updates Cargo.lock):
   ```bash
   cargo build --release
   ```

4. **Verify**:
   ```bash
   ./target/release/framec --version
   # Should show: framec 0.80.0
   ```

### Files Automatically Updated

| File | Content | Updated By |
|------|---------|------------|
| `version.toml` | Source of truth | Manual edit |
| `framec/Cargo.toml` | Package version | Pre-commit hook |
| `frame_build/Cargo.toml` | Package version | Pre-commit hook |
| `framec/src/frame_c/compiler.rs` | Version string | Pre-commit hook |
| `Cargo.lock` | Dependency versions | `cargo build` |

### CLI Version Output

The CLI version comes from the build script (`framec/build.rs`) which reads `version.toml`:

```bash
framec --version
# Output: framec 0.79.0
```

### Benefits

- ✅ **Single source of truth**: Only edit `version.toml`
- ✅ **Automatic sync**: No manual updates to multiple files
- ✅ **Impossible to forget**: Pre-commit hook ensures consistency
- ✅ **Team consistency**: Same process for all developers
- ✅ **Build-time version**: CLI always shows correct version

### Troubleshooting

**Hook not running?**
```bash
# Check if hook is executable
ls -la .git/hooks/pre-commit

# Make executable if needed
chmod +x .git/hooks/pre-commit
```

**Manual sync needed?**
```bash
# Run the manual sync script
./scripts/sync-versions.sh
```

**Skip hook temporarily?**
```bash
# Bypass the pre-commit hook (not recommended)
git commit --no-verify
```

**Test hook manually?**
```bash
# Run the hook directly
.git/hooks/pre-commit
```